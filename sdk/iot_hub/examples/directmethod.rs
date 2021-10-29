use iot_hub::service::ServiceClient;
use serde_json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let module_id = std::env::args()
        .nth(2)
        .expect("Please pass the module id as the second parameter");

    let method_name = std::env::args()
        .nth(3)
        .expect("Please pass the method name as the third parameter");

    let payload = std::env::args()
        .nth(4)
        .expect("Please pass the payload as the fourth parameter");

    let http_client = azure_core::new_http_client();
    let service_client =
        ServiceClient::from_connection_string(http_client, iot_hub_connection_string, 3600)?;
    println!(
        "Sending direct method {} to {}:{} on: {}",
        method_name, device_id, module_id, service_client.iot_hub_name
    );

    let direct_method =
        service_client.create_module_method(device_id, module_id, method_name, 30, 30);

    let response = direct_method
        .execute(serde_json::from_str(&payload)?)
        .await?;

    println!(
        "Received a response from the direct method with status code {} and payload {:?}",
        response.status, response.payload
    );

    Ok(())
}
