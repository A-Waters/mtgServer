use aws_sdk_rds::Client;

pub async fn create_rds_client() -> Client {
    let config = aws_config::from_env().region("us-east-1").load().await;
    Client::new(&config)
}