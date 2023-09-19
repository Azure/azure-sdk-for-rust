use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::prelude::*;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_value =
        env::var("SECRET_VALUE").expect("Missing SECRET_VALUE environment variable.");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client = SecretClient::new(&keyvault_url, creds)?;

    client.set(&secret_name, &secret_value).await?;

    let secret = client.get(secret_name).await?;
    assert_eq!(secret.value, secret_value);

    Ok(())
}
