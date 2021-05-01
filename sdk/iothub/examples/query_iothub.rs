use std::sync::Arc;

use azure_core::HttpClient;
use iothub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let query = "SELECT * FROM devices";
    println!("Invoking query '{}' on the IoT Hub", query);

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let service_client =
        ServiceClient::from_connection_string(http_client, iothub_connection_string, 3600)?;

    let response = service_client
        .query()
        .max_item_count(1)
        .execute(query)
        .await?;

    println!(
        "Response of first result: {}",
        serde_json::to_string(&response.result)?
    );

    let token = match response.continuation_token {
        Some(val) => val,
        None => return Ok(()),
    };

    let response = service_client
        .query()
        .max_item_count(1)
        .continuation(token.as_str())
        .execute(query)
        .await?;

    println!(
        "Response of second result: {}",
        serde_json::to_string(&response.result)?
    );

    Ok(())
}
