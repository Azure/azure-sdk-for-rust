// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates how to use Azure credentials with different cloud configurations.
//! This is particularly useful when working with sovereign clouds like Azure China, Azure Germany,
//! or Azure US Government.

use azure_core::credentials::Secret;
use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using convenience methods for specific clouds
    println!("Example 1: Using convenience methods for specific clouds");

    // Get credentials from environment variables
    let tenant_id = env::var("AZURE_TENANT_ID").unwrap_or_else(|_| "tenant-id".to_string());
    let client_id = env::var("AZURE_CLIENT_ID").unwrap_or_else(|_| "client-id".to_string());
    let client_secret = env::var("AZURE_CLIENT_SECRET").unwrap_or_else(|_| "client-secret".to_string());

    // Create credentials for different clouds using convenience methods
    let _public_cloud_credential = ClientSecretCredential::new_for_public_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    println!("✓ Created credential for Azure Public Cloud");

    let _china_cloud_credential = ClientSecretCredential::new_for_china_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    println!("✓ Created credential for Azure China Cloud");

    let _us_gov_credential = ClientSecretCredential::new_for_us_government_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    println!("✓ Created credential for Azure US Government Cloud");

    // Example 2: Using cloud configuration with TokenCredentialOptions
    println!("\nExample 2: Using cloud configuration with TokenCredentialOptions");

    use azure_core::cloud::configurations;
    use azure_identity::ClientSecretCredentialOptions;

    // Create options for China Cloud
    let mut china_options = TokenCredentialOptions::default();
    china_options.set_cloud_config(configurations::azure_china_cloud());

    let _china_credential_with_options = ClientSecretCredential::new(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
        Some(ClientSecretCredentialOptions {
            credential_options: china_options,
        }),
    )?;
    println!("✓ Created China Cloud credential using explicit cloud configuration");

    // Example 3: Using cloud configuration with ClientOptions for service clients
    println!("\nExample 3: Using cloud configuration for service clients");

    use azure_core::http::ClientOptions;

    // For a Tables service client (example), you would configure cloud and audience
    let _tables_options = ClientOptions::default()
        .with_cloud_config(configurations::azure_china_cloud())
        .with_audience("https://storage.azure.com"); // Tables uses storage audience

    println!("✓ Created ClientOptions for Tables service in China Cloud");

    // Example 4: Deriving scopes from audience
    println!("\nExample 4: Deriving OAuth scopes from audience");

    use azure_core::cloud::CloudConfiguration;

    let storage_audience = "https://storage.azure.com";
    let storage_scope = CloudConfiguration::audience_to_scope(storage_audience);
    println!("Audience: {}", storage_audience);
    println!("Derived scope: {}", storage_scope);

    let keyvault_audience = "https://vault.azure.net";
    let keyvault_scope = CloudConfiguration::audience_to_scope(keyvault_audience);
    println!("Audience: {}", keyvault_audience);
    println!("Derived scope: {}", keyvault_scope);

    // Example 5: Accessing cloud-specific service audiences
    println!("\nExample 5: Accessing cloud-specific service audiences");

    let public_cloud = configurations::azure_public_cloud();
    let china_cloud = configurations::azure_china_cloud();

    println!("Public Cloud Storage Audience: {:?}", public_cloud.get_service_audience("storage"));
    println!("China Cloud Storage Audience: {:?}", china_cloud.get_service_audience("storage"));

    println!("Public Cloud KeyVault Audience: {:?}", public_cloud.get_service_audience("keyvault"));
    println!("China Cloud KeyVault Audience: {:?}", china_cloud.get_service_audience("keyvault"));

    println!("\n✅ All examples completed successfully!");

    Ok(())
}