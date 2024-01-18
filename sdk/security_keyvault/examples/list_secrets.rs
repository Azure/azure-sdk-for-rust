use azure_security_keyvault::SecretClient;
use futures::stream::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let credential = azure_identity::create_credential()?;

    let client = SecretClient::new(&keyvault_url, credential)?;

    let mut stream = client.list_secrets().into_stream();
    while let Some(response) = stream.next().await {
        println!("{:#?}", response?);
    }

    Ok(())
}
