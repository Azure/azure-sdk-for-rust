use azure_security_keyvault::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_value =
        env::var("SECRET_VALUE").expect("Missing SECRET_VALUE environment variable.");

    let credential = azure_identity::new_credential();

    let client = SecretClient::new(&keyvault_url, credential)?;

    client.set(&secret_name, &secret_value).await?;

    let secret = client.get(secret_name).await?;
    assert_eq!(secret.value, secret_value);

    Ok(())
}
