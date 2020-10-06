use azure_sdk_keyvault::KeyVaultClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_name = env::var("KEYVAULT_NAME").expect("Missing KEYVAULT_NAME environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_value = env::var("SECRET_VALUE").expect("Missing SECRET_VALUE environment variable.");

    let mut client = KeyVaultClient::new(&client_id, &client_secret, &tenant_id, &keyvault_name);

    client.set_secret(&secret_name, &secret_value).await?;

    let secret = client.get_secret(&secret_name).await?;
    assert_eq!(secret.value(), "whatup");

    Ok(())
}
