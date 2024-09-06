use lambda_runtime::{service_fn, Error};
use gameserver::lambda_handler::main_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // check if the environment variable is set and throw an error if it is not
    let env = std::env::var("ENV").unwrap_or("dev".to_string());
    println!("ENV: {}", env);
    // Mock the lambda runtime for local development
    if env == "dev" {
        println!("Running in dev mode, mocking lambda runtime");
        let event = serde_json::json!({
            "message": "local test"
        });
        let context = lambda_runtime::Context::default();
        let lambda_event = lambda_runtime::LambdaEvent::new(event, context);
        let result = main_handler(lambda_event).await?;
        println!("Local result: {:?}", result);
        return Ok(());
    }

    lambda_runtime::run(service_fn(main_handler)).await?;
    Ok(())
}




