use azure_identity::DefaultAzureCredential;
use azure_security_keyvault::SecretClient;
use futures::stream::StreamExt;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let creds = Arc::new(DefaultAzureCredential::default());

    let client = SecretClient::new(&keyvault_url, creds)?;

    let mut stream = client.list_secrets().into_stream();
    while let Some(response) = stream.next().await {
        println!("{:#?}", response?);
    }

    Ok(())
}
