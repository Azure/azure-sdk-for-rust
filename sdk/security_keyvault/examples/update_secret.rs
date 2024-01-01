use azure_core::date;
use azure_security_keyvault::prelude::*;
use std::env;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");
    let secret_version =
        env::var("SECRET_VERSION").expect("Missing SECRET_VERSION environment variable.");

    let credential = azure_identity::new_credential();

    let client = SecretClient::new(&keyvault_url, credential)?;

    // Disable secret.
    client
        .update(secret_name)
        .version(secret_version)
        .enabled(false)
        .recovery_level("Purgeable")
        .expiration(OffsetDateTime::now_utc() + date::duration_from_days(14))
        .await?;

    Ok(())
}
