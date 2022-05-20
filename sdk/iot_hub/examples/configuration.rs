use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let configuration_id = std::env::args()
        .nth(1)
        .expect("Please pass the configuration id as the first parameter");

    println!("Getting configuration: {}", configuration_id);

    let http_client = azure_core::new_http_client();
    let service_client =
        ServiceClient::from_connection_string(http_client, iot_hub_connection_string, 3600)?;
    let configuration = service_client.get_configuration(configuration_id).await?;

    println!(
        "Successfully retrieved the new configuration '{:?}'",
        configuration
    );

    Ok(())
}
