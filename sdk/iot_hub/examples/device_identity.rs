use azure_iot_hub::service::resources::{AuthenticationMechanism, DesiredCapability, Status};
use azure_iot_hub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let iot_hub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    println!("Getting device twin for device '{}'", device_id);
    let http_client = azure_core::new_http_client();
    let service_client =
        ServiceClient::from_connection_string(http_client, iot_hub_connection_string, 3600)?;
    let device = service_client
        .create_device_identity()
        .execute(
            &device_id,
            Status::Enabled,
            AuthenticationMechanism::new_using_symmetric_key(
                "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
                "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk",
            ),
        )
        .await?;

    println!("Successfully created a new device '{}'", device.device_id);

    println!(
        "Setting status to disabled and set IoT Edge capability of device '{}'",
        device.device_id
    );
    let device = service_client
        .update_device_identity(device.etag)
        .device_capability(DesiredCapability::IotEdge)
        .execute(
            &device_id,
            Status::Enabled,
            AuthenticationMechanism::new_using_symmetric_key(
                "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
                "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk",
            ),
        )
        .await?;

    println!("Getting device identity of '{}'", device.device_id);
    let device = service_client.get_device_identity(device.device_id).await?;
    println!("Identity is: {:?}", device);

    println!("Deleting device '{}'", device.device_id);
    service_client
        .delete_device_identity(device.device_id, device.etag)
        .execute()
        .await?;

    Ok(())
}
