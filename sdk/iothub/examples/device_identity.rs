use iothub::service::identity::{DesiredCapability, Status};
use iothub::service::ServiceClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let iothub_connection_string = std::env::var("IOTHUB_CONNECTION_STRING")
        .expect("Set env variable IOTHUB_CONNECTION_STRING first!");

    let device_id = std::env::args()
        .nth(1)
        .expect("Please pass the device id as the first parameter");

    println!("Getting device twin for device '{}'", device_id);

    let service_client = ServiceClient::from_connection_string(iothub_connection_string, 3600)?;
    let device = service_client
        .create_device_identity()
        .device_id(device_id)
        .authentication_using_sas(
            "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
            "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk=",
        )
        .status(Status::Enabled)
        .execute()
        .await?;

    println!("Successfully created a new device '{}'", device.device_id);

    println!(
        "Setting status to disabled and set IoT Edge capability of device '{}'",
        device.device_id
    );
    let device = service_client
        .update_device_identity(device.etag)
        .device_id(device.device_id)
        .device_capability(DesiredCapability::IotEdge)
        .authentication_using_sas(
            "QhgevIUBSWe37q1MP+M/vtktjOcrE74BVbpcxlLQw58=",
            "6YS6w5wqkpdfkEW7iOP1NvituehFlFRfPko2n7KY4Gk=",
        )
        .status(Status::Disabled)
        .execute()
        .await?;

    println!("Getting device identity of '{}'", device.device_id);
    let device = service_client.get_device_identity(device.device_id).await?;
    println!("Identity is: {:?}", device);

    println!("Deleting device '{}'", device.device_id);
    service_client
        .delete_device_identity(device.device_id, None)
        .await?;

    Ok(())
}
