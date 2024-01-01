use azure_iot_deviceupdate::DeviceUpdateClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID")
        .expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");
    let operation_id = env::var("DEVICE_UPDATE_OPERATION_ID")
        .expect("Missing DEVICE_UPDATE_OPERATION_ID environment variable.");

    let credential = azure_identity::new_credential();
    let client = DeviceUpdateClient::new(&device_update_url, credential)?;

    let get_operation_response = client.get_operation(&instance_id, &operation_id).await?;
    dbg!(&get_operation_response);

    Ok(())
}
