use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use base64::{engine::general_purpose, Engine as _};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use std::error::Error as StdError;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        lambda_runtime::run(service_fn(func)).await?;
    } else {
        // Mock Lambda input
        let mock_event = LambdaEvent::new(
            json!({"name": "Black Lotus"}),
            lambda_runtime::Context::default()
        );
        let result = func(mock_event).await?;
        println!("Mock Lambda Result: {:?}", result);
    }
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {

    // check if on lambda
    let csv_reader: Vec<u8> = if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {

        println!("on lambda");
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        let bucket_name = "card-server-data-bucket-619648504467";
        let key = "magic_cards.csv";

        println!("getting object from S3");
        let resp = client.get_object()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                eprintln!("Failed to get object from S3: {:?}", e);

                // Log the source of the error if available
                if let Some(source) = e.source() {
                    eprintln!("Source error: {:?}", source);
                }

                // Return a custom error with more detailed context
                Error::from(format!("Failed to get object from S3: {:?}", e))
            })?;

        let mut buffer = Vec::new();
        resp.body.into_async_read().read_to_end(&mut buffer).await?;
        buffer
    } else {
        // Read the local file asynchronously
        tokio::fs::read("magic_cards.csv").await.map_err(|e| {
            eprintln!("Failed to open CSV file: {}", e);
            Error::from(format!("Failed to open CSV file: {}", e))
        })?
    };

    println!("reading csv");
    let mut csv_reader = csv::Reader::from_reader(csv_reader.as_slice());

    // Skip the header row
    let _headers = csv_reader.headers()
        .map_err(|e| {
        eprintln!("Failed to read CSV headers: {}", e);
        Error::from(format!("Failed to read CSV headers: {}", e))
    })?;

    println!("creating cards map");
    let mut cards_map = std::collections::HashMap::new();

    let mut cards = Vec::new();
    for (index, result) in csv_reader.records().enumerate() {
        if index % 1000 == 0 {
            println!("Processed {} records", index);
        }
        let record = result.map_err(|e| {
            eprintln!("Failed to read CSV record: {}", e);
            Error::from(format!("Failed to read CSV record: {}", e))
        })?;

        let card = json!({
            "name": decode_field(&record,0),
            "manaCost": decode_field(&record,1),
            "cmc": decode_field(&record,2),
            "colors": decode_field(&record,3),
            "colorIdentity": decode_field(&record,4),
            "type": decode_field(&record,5),
            "types": decode_field(&record,6),
            "subtypes": decode_field(&record,7),
            "rarity": decode_field(&record,8),
            "set": decode_field(&record,9),
            "setName": decode_field(&record,10),
            "text": decode_field(&record,11),
            "artist": decode_field(&record,12),
            "number": decode_field(&record,13),
            "power": decode_field(&record,14),
            "toughness": decode_field(&record,15),
            "layout": decode_field(&record,16),
            "multiverseid": decode_field(&record,17),
            "imageUrl": decode_field(&record,18),
            "originalType": decode_field(&record,19),
            "legalities": decode_field(&record,20),
            "id": decode_field(&record,21),
            "flavor": decode_field(&record,22),
            "rulings": decode_field(&record,23),
            "supertypes": decode_field(&record,24),
            "loyalty": decode_field(&record,25),
            "watermark": decode_field(&record,26),
            "hand": decode_field(&record,27),
            "life": decode_field(&record,28)
        });
        cards.push(card);
    }
    println!("Finished processing all {} records", cards.len());

    println!("inserting cards into map");
    for card in cards {
        if let Some(name) = card["name"].as_str() {
            let name_lower = name.to_lowercase();
            cards_map.insert(name_lower, card.clone());
        }
    }

    println!("searching for card");
    // get the card from the event
    let card_name = event.payload["name"].as_str().unwrap_or("Name Not Supplied");
    let card_name_lower = card_name.to_lowercase();
    println!("searching for card: -{}-", card_name_lower);

    let found_card: Option<&Value> = cards_map.get(&card_name_lower);

    if found_card.is_none() {
        return Ok(json!({
            "statusCode": 404,
            "body": format!("Card '{}' not found", card_name)
        }));
    }

    let response = json!({
        "statusCode": 200,
        "body": found_card
    });

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    println!("found card: {:?}", found_card);
    Ok(response)
}

fn decode_field(record: &csv::StringRecord, index: usize) -> Value {
    let raw_value = record.get(index).unwrap_or("");
    
    // Check if the value is a number
    if let Ok(num) = raw_value.parse::<f64>() {
        return Value::Number(serde_json::Number::from_f64(num).unwrap());
    }

    // If it's not a number, proceed with base64 decoding
    let decoded = general_purpose::STANDARD.decode(raw_value)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_default();
    
    if decoded.is_empty() {
        Value::Null
    } else {
        Value::String(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;

    #[tokio::test]
    async fn test_func_existing_card() {
        // Mock event with an existing card name
        let event = LambdaEvent::new(
            json!({"name": "Ancestor's Chosen"}),
            Context::default()
        );

        let result = func(event).await.unwrap();

        assert_eq!(result["statusCode"], 200);
        assert!(result["body"].is_object());
        assert_eq!(result["body"]["name"], "Ancestor's Chosen");
        assert_eq!(result["body"]["cmc"], "7.0");
    }

    #[tokio::test]
    async fn test_func_nonexistent_card() {
        // Mock event with a non-existent card name
        let event = LambdaEvent::new(
            json!({"name": "Nonexistent Card"}),
            Context::default()
        );

        let result = func(event).await.unwrap();

        assert_eq!(result["statusCode"], 404);
        assert_eq!(result["body"], "Card 'Nonexistent Card' not found");
    }

    #[tokio::test]
    async fn test_func_case_insensitive() {
        // Mock event with mixed case card name
        let event = LambdaEvent::new(
            json!({"name": "bLaCk LoTuS"}),
            Context::default()
        );

        let result = func(event).await.unwrap();

        assert_eq!(result["statusCode"], 200);
        assert!(result["body"].is_object());
        assert_eq!(result["body"]["name"], "Black Lotus");
    }
}

