use iothub::service::ServiceClient;
use serde_json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let iothub_name = std::env::var("IOTHUB_NAME").expect("Set env variable IOTHUB_NAME first!");
    let iothub_private_key =
        std::env::var("IOTHUB_PRIVATE_KEY").expect("Set env variable IOTHUB_PRIVATE_KEY first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let module_id = std::env::args()
        .nth(2)
        .expect("Please pass the device id as the second parameter");

    let method_name = std::env::args()
        .nth(3)
        .expect("Please pass the method name as the third parameter");

    let payload = std::env::args()
        .nth(4)
        .expect("Please pass the payload as the fourth parameter");

    let service_client = ServiceClient::from_private_key(iothub_name, iothub_private_key, 3600)?;
    println!(
        "Sending direct method {} to {}:{} on: {}",
        method_name, device_id, module_id, service_client.iothub_name
    );

    let direct_method =
        service_client.create_module_method(device_id, module_id, method_name, 30, 30);
    let response = direct_method
        .invoke::<serde_json::Value>(serde_json::from_str(&payload)?)
        .await?;
    println!(
        "Received a response from the direct method with status code {} and payload {}",
        response.status, response.payload
    );

    Ok(())
}
