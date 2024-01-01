use azure_iot_deviceupdate::DeviceUpdateClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID")
        .expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");
    let provider = env::var("DEVICE_UPDATE_PROVIDER")
        .expect("Missing DEVICE_UPDATE_PROVIDER environment variable.");

    let credential = azure_identity::new_credential();
    let client = DeviceUpdateClient::new(&device_update_url, credential)?;

    let list_names_response = client.list_names(&instance_id, &provider).await?;
    dbg!(&list_names_response);

    Ok(())
}
