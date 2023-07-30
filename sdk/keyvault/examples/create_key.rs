use std::sync::Arc;

use azure_core::{new_http_client, Result};
use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_svc_keyvault::models::{key_create_parameters::Kty, KeyCreateParameters};

#[tokio::main]
async fn main() -> Result<()> {
    let tenant_id = std::env::var("TENANT_ID").expect("Missing TENANT_ID env var");
    let client_id = std::env::var("CLIENT_ID").expect("Missing CLIENT_ID env var");
    let client_secret = std::env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET env var");
    let vault_name = std::env::var("VAULT_NAME").expect("Missing VAULT_NAME env var");
    let key_name = std::env::args().nth(1).expect("No key name provided");

    let credentials = ClientSecretCredential::new(
        new_http_client(),
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );

    let keyvault_client =
        azure_keyvault::keyvault_client::KeyvaultClient::new(Arc::new(credentials), vault_name);

    let _key = keyvault_client
        .create_key(key_name, KeyCreateParameters::new(Kty::Rsa))
        .await?;

    Ok(())
}
