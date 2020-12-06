use iothub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    println!("Getting device twin for device: {}", device_id);

    let service_client = ServiceClient::from_connection_string(iothub_connection_string, 3600)?;
    let device = service_client.get_device_identity(device_id).await?;

    println!("Received device twin: {:?}", device);

    Ok(())
}
