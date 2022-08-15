use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_iot_deviceupdate::DeviceUpdateClient;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
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

    let creds = Arc::new(ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    ));
    let client = DeviceUpdateClient::new(&device_update_url, creds)?;

    let get_update_response = client
        .get_update(&instance_id, &provider, &name, &version)
        .await?;
    dbg!(&get_update_response);

    Ok(())
}
