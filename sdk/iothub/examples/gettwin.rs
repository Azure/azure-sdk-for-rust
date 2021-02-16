use std::sync::Arc;

use azure_core::HttpClient;
use iothub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    println!("Getting device twin for device: {}", device_id);

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let service_client =
        ServiceClient::from_connection_string(http_client, iothub_connection_string, 3600)?;
    let twin = service_client.get_device_twin(device_id).await?;

    println!("Received device twin: {:?}", twin);

    Ok(())
}
