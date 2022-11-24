use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::prelude::*;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client = SecretClient::new(&keyvault_url, creds)?;

    get_secret(&client).await?;

    Ok(())
}

async fn get_secret(client: &SecretClient) -> Result<(), Box<dyn std::error::Error>> {
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secrets = client.get(secret_name).await?;
    dbg!(&secrets);

    Ok(())
}
