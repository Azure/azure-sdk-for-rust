use azure_security_keyvault::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");

    let credential = azure_identity::new_credential();

    let client = SecretClient::new(&keyvault_url, credential)?;
    client.delete(secret_name).await?;

    Ok(())
}
