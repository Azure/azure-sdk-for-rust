use azure_security_keyvault::prelude::*;
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");

    let credential = azure_identity::new_credential();

    let client = SecretClient::new(&keyvault_url, credential)?;

    let mut versions = client.get_versions(&secret_name).into_stream();
    while let Some(version) = versions.next().await {
        println!("{:?}", version?);
    }

    let secret = client.get(secret_name).await?;
    dbg!(secret.value);

    Ok(())
}
