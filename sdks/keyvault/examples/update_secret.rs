use azure_keyvault::{KeyVaultClient, RecoveryLevel};
use chrono::prelude::*;
use chrono::Duration;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_name =
        env::var("KEYVAULT_NAME").expect("Missing KEYVAULT_NAME environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_version =
        env::var("SECRET_VERSION").expect("Missing SECRET_VERSION environment variable.");

    let mut client = KeyVaultClient::new(&client_id, &client_secret, &tenant_id, &keyvault_name);

    // Disable secret.
    client
        .update_secret_enabled(&secret_name, &secret_version, false)
        .await?;

    // Update secret recovery level to `Purgeable`.
    client
        .update_secret_recovery_level(&secret_name, &secret_version, RecoveryLevel::Purgeable)
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
