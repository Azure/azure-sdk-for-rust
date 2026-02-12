// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get Key Vault URL from command line argument or environment variable
    let vault_url = env::args()
        .nth(1)
        .or_else(|| env::var("AZURE_KEYVAULT_URL").ok())
        .ok_or("Key Vault URL must be provided as an argument or in AZURE_KEYVAULT_URL environment variable")?;

    // Create a new secret client
    let credential = DeveloperToolsCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    // List all secrets and collect their names
    let mut secret_names = Vec::new();
    let mut pager = client.list_secret_properties(None)?;
    while let Some(secret) = pager.try_next().await? {
        let name = secret.resource_id()?.name;
        secret_names.push(name);
    }

    // Sort the secret names
    secret_names.sort();

    // Print each secret name on its own line
    for name in secret_names {
        println!("{}", name);
    }

    Ok(())
}
