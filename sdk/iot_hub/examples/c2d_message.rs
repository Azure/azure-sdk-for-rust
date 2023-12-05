use azure_iot_hub::service::ServiceClient;
use std::collections::HashMap;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    let message_body =
        std::env::args()
            .nth(2)
            .and_then(|s| if s.trim().is_empty() { None } else { Some(s) });

    // Properties should be passed as a comma separated list of key=value pairs.
    // For example: key1=value1,key2=value2,key3=value3
    let properties = std::env::args().nth(3).map(|s| {
        s.split(',')
            .map(|s| {
                let mut split = s.split('=');
                (
                    split.next().expect("use the format key=value").to_string(),
                    split.next().expect("use the format key=value").to_string(),
                )
            })
            .collect::<HashMap<String, String>>()
    });

    let service_client = ServiceClient::new_connection_string(iot_hub_connection_string, 3600)?;

    println!(
        "Sending cloud-to-device message to {device_id} on: {}\
        \n\tbody: {message_body:?}\n\tproperties: {properties:?}",
        service_client.iot_hub_name
    );

    let mut c2d_message_builder = service_client.send_c2d_message(&device_id);

    if let Some(message_body) = message_body {
        let body: serde_json::Value = serde_json::from_str(&message_body)?;
        c2d_message_builder = c2d_message_builder.message_body(body);
    }

    if let Some(properties) = properties {
        c2d_message_builder = c2d_message_builder.properties(properties);
    }

    c2d_message_builder.await?;

    println!("Cloud to device message sent successfully");

    Ok(())
}
