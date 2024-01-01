use azure_iot_deviceupdate::DeviceUpdateClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID")
        .expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");

    let credential = azure_identity::new_credential();
    let client = DeviceUpdateClient::new(&device_update_url, credential)?;

    let s_filter = env::var("DEVICE_UPDATE_FILTER").unwrap_or_default();
    let mut filter: Option<&str> = None;
    if !s_filter.is_empty() {
        filter = Some(&s_filter);
    }

    let s_search = env::var("DEVICE_UPDATE_SEARCH").unwrap_or_default();
    let mut search: Option<&str> = None;
    if !s_search.is_empty() {
        search = Some(&s_search);
    }

    let list_names_response = client.list_updates(&instance_id, filter, search).await?;
    dbg!(&list_names_response);

    Ok(())
}
