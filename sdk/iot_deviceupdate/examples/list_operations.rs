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

    let creds = Arc::new(ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    ));
    let client = DeviceUpdateClient::new(&device_update_url, creds)?;

    let s_filter = env::var("DEVICE_UPDATE_FILTER").unwrap_or_default();

    let mut filter: Option<&str> = None;
    if !s_filter.is_empty() {
        filter = Some(&s_filter);
    }

    let s_top = env::var("DEVICE_UPDATE_TOP").unwrap_or_default();

    let mut top: Option<&str> = None;
    if !s_top.is_empty() {
        top = Some(&s_top);
    }

    let list_names_response = client.list_operations(&instance_id, filter, top).await?;
    dbg!(&list_names_response);

    Ok(())
}
