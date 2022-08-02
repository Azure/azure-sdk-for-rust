use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let query = "SELECT * FROM devices";
    println!("Invoking query '{}' on the IoT Hub", query);

    let service_client = ServiceClient::new_connection_string(iot_hub_connection_string, 3600)?;

    let response = service_client
        .query(query)
        .max_item_count(1)
        .into_future()
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
        .query(query)
        .max_item_count(1)
        .continuation(token)
        .into_future()
        .await?;

    println!(
        "Response of second result: {}",
        serde_json::to_string(&response.result)?
    );

    Ok(())
}
