use lambda_runtime::{LambdaEvent, Error};
use serde_json::{json, Value};
use crate::rds_client::create_rds_client;


pub async fn main_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // Your game server logic here
    let (_event, _context) = event.into_parts();
    // read in the json data and print it out
    // println!("Received event: {}", serde_json::to_string_pretty(&_event).unwrap());
    // Create a HashMap to store the event data
    let mut data_map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();

    // Check if the event is an object
    if let Some(obj) = _event.as_object() {
        // Iterate over the key-value pairs and insert them into the HashMap
        for (key, value) in obj {
            data_map.insert(key.to_string(), value.clone());
        }
    }

    // write the data in the map to the return json
    let mut result = json!({ "response": "responding!" });
    for (key, value) in &data_map {
        result[key] = value.clone();
    }


    // Print the HashMap for verification
    println!("Event data in HashMap: {:?}", data_map);

    let client: aws_sdk_rds::Client = create_rds_client().await;

    // Get data from the db with the json data as the parameter
    let db_query_result = match client.describe_db_instances().send().await {
        Ok(output) => {
            let instances: &[aws_sdk_rds::types::DbInstance] = output.db_instances();
            let instance_info: Vec<String> = instances
                .iter()
                .map(|instance: &aws_sdk_rds::types::DbInstance| format!("Instance ID: {}", instance.db_instance_identifier().unwrap_or("N/A")))
                .collect();
            json!({ "db_instances": instance_info })
        },
        Err(err) => {
            eprintln!("Error querying RDS: {:?}", err);
            json!({ "error": "Failed to query RDS" })
        }
    };

    // Merge the db_query_result into the result
    if let Some(db_result_obj) = db_query_result.as_object() {
        for (key, value) in db_result_obj {
            result[key] = value.clone();
        }
    }



    Ok(result)
}