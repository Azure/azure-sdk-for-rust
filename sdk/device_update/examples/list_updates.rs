use azure_identity::token_credentials::{ClientSecretCredential, TokenCredentialOptions};
use azure_device_update::DeviceUpdateClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID").expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );
    let mut client = DeviceUpdateClient::new(&device_update_url, &creds)?;

    let s_filter= match env::var("DEVICE_UPDATE_FILTER") {
        Err(_e) => { "".to_owned() },
        Ok(s) => { s }
    };
    let mut filter: Option<&str> = None;
    if  s_filter.len() != 0 {
        filter=Some(&s_filter);
    }

    let s_search = match env::var("DEVICE_UPDATE_SEARCH") {
        Err(_e) => { "".to_owned() },
        Ok(s) => { s }
    };
    let mut search: Option<&str> = None;
    if s_search.len() != 0 {
        search=Some(&s_search);
    }

    let list_names_response = client.list_updates(&instance_id, filter, search).await?;
    dbg!(&list_names_response);

    Ok(())
}
