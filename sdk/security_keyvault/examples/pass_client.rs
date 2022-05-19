use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_security_keyvault::KeyClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );
    let mut client = KeyClient::new(&keyvault_url, &creds)?;

    get_secret(&mut client).await?;

    Ok(())
}

async fn get_secret(
    client: &mut KeyClient<'_, ClientSecretCredential>,
) -> Result<(), Box<dyn std::error::Error>> {
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secrets = client.get_secret(&secret_name).await?;
    dbg!(&secrets);

    Ok(())
}
