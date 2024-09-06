use lambda_runtime::LambdaEvent;
use serde_json::json;
use gameserver::lambda_handler::main_handler;

#[tokio::test]
async fn test_main_handler_happy_path() {
    let event = json!({
        "message": "ping!"
    });
    let context: lambda_runtime::Context = lambda_runtime::Context::default();
    let event: LambdaEvent<serde_json::Value> = LambdaEvent::new(event, context);
    let result: serde_json::Value = main_handler(event).await.unwrap();
    println!("result: {:?}", result);
    assert_eq!(result, json!({"message":"ping!", "response":"responding!"}));
}


#[tokio::test]
async fn test_main_handler_sad_path() {
    let event: serde_json::Value = json!({
        "message": "ping!"
    });
    let context: lambda_runtime::Context = lambda_runtime::Context::default();
    let event: LambdaEvent<serde_json::Value> = LambdaEvent::new(event, context);
    let result: serde_json::Value = main_handler(event).await.unwrap();
    println!("result: {:?}", result);
    assert_ne!(result, json!({ "message": "failed!" }));
}
