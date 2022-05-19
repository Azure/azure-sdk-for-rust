use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_security_keyvault::KeyClient;
use chrono::prelude::*;
use chrono::Duration;
use std::env;

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
    let mut client = KeyClient::new(&keyvault_url, &creds)?;

    // Disable secret.
    client
        .update_secret_enabled(&secret_name, &secret_version, false)
        .await?;

    // Update secret recovery level to `Purgeable`.
    client
        .update_secret_recovery_level(&secret_name, &secret_version, "Purgeable".into())
        .await?;

    // Update secret to expire in two weeks.
    client
        .update_secret_expiration_time(
            &secret_name,
            &secret_version,
            Utc::now() + Duration::days(14),
        )
        .await?;

    Ok(())
}
