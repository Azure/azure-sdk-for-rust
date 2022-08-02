use azure_core::date;
use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_security_keyvault::SecretClient;
use std::env;
use std::sync::Arc;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_version =
        env::var("SECRET_VERSION").expect("Missing SECRET_VERSION environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );
    let client = SecretClient::new(&keyvault_url, Arc::new(creds))?;

    // Disable secret.
    client
        .update(secret_name)
        .version(secret_version)
        .enabled(false)
        .recovery_level("Purgeable")
        .expiration(OffsetDateTime::now_utc() + date::duration_from_days(14))
        .into_future()
        .await?;

    Ok(())
}
