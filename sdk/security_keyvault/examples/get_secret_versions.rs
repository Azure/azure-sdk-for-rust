use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::prelude::*;
use futures::StreamExt;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client = SecretClient::new(&keyvault_url, creds)?;

    let mut secrets = client.get_versions(secret_name).into_stream();
    while let Some(secret) = secrets.next().await {
        println!("{:?}", secret?);
    }

    dbg!(&secrets);

    Ok(())
}
