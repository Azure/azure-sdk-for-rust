use azure_core::HttpClient;
use iothub::service::ServiceClient;
use serde_json;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let payload = std::env::args()
        .nth(2)
        .expect("Please pass the payload as the second parameter");

    println!("Updating device twin for device: {}", device_id);
    let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());

    let service_client =
        ServiceClient::from_connection_string(http_client, iothub_connection_string, 3600)?;
    let updated_twin = service_client
        .update_device_twin(device_id)
        .properties(serde_json::from_str(&payload)?)
        .execute()
        .await?;

    println!("Received device twin: {:?}", updated_twin);

    Ok(())
}
