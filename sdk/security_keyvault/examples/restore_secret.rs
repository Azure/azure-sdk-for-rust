use azure_security_keyvault::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let backup_blob = env::var("BACKUP_BLOB").expect("Missing BACKUP_BLOB environment variable.");

    let credential = azure_identity::new_credential();

    let client = SecretClient::new(&keyvault_url, credential)?;

    client.restore_secret(&backup_blob).await?;

    Ok(())
}
