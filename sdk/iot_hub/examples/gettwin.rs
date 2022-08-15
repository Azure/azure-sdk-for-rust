use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    println!("Getting device twin for device: {}", device_id);

    let service_client = ServiceClient::new_connection_string(iot_hub_connection_string, 3600)?;
    let twin = service_client
        .get_device_twin(device_id)
        .into_future()
        .await?;

    println!("Received device twin: {:?}", twin);

    Ok(())
}
