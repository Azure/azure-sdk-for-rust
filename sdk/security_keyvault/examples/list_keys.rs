use azure_identity::DefaultAzureCredentialBuilder;
use azure_security_keyvault::prelude::*;
use futures::stream::StreamExt;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let creds = Arc::new(
        DefaultAzureCredentialBuilder::new()
            .exclude_managed_identity_credential()
            .build(),
    );

    let client = SecretClient::new(&keyvault_url, creds)?;

    let mut stream = client.list_secrets().into_stream();
    while let Some(response) = stream.next().await {
        dbg!(&response?);
    }

    Ok(())
}
