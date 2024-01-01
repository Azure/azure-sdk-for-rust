use azure_iot_deviceupdate::DeviceUpdateClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID")
        .expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");
    let name =
        env::var("DEVICE_UPDATE_NAME").expect("Missing DEVICE_UPDATE_NAME environment variable.");
    let provider = env::var("DEVICE_UPDATE_PROVIDER")
        .expect("Missing DEVICE_UPDATE_PROVIDER environment variable.");
    let version = env::var("DEVICE_UPDATE_VERSION")
        .expect("Missing DEVICE_UPDATE_VERSION environment variable.");
    let file_id = env::var("DEVICE_UPDATE_FILE_ID")
        .expect("Missing DEVICE_UPDATE_FILE_ID environment variable.");

    let credential = azure_identity::new_credential();
    let client = DeviceUpdateClient::new(&device_update_url, credential)?;

    let get_file_response = client
        .get_file(&instance_id, &provider, &name, &version, &file_id)
        .await?;
    dbg!(&get_file_response);

    Ok(())
}
