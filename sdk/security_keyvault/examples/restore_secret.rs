use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::SecretClient;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let backup_blob = env::var("BACKUP_BLOB").expect("Missing BACKUP_BLOB environment variable.");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client = SecretClient::new(&keyvault_url, creds)?;

    client.restore_secret(&backup_blob).into_future().await?;

    Ok(())
}
