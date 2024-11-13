use azure_security_keyvault::{prelude::KeyVaultKey, KeyClient};
use std::env;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let key_name = env::var("KEYVAULT_KEY").expect("Missing KEYVAULT_KEY environment variable.");

    let credential = azure_identity::create_credential()?;

    let client = KeyClient::new(&keyvault_url, credential)?;

    let key: KeyVaultKey = client.get(key_name).await?;
    println!("{key:#?}");

    Ok(())
}
