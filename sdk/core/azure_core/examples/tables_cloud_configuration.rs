// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example showing how a Tables service client would use cloud configuration.
//! This demonstrates the pattern that Tables and other services would follow.

use azure_core::cloud::configurations;
use azure_core::credentials::Secret;
use azure_core::http::{ClientOptions, policies::BearerTokenCredentialPolicy};
use azure_identity::ClientSecretCredential;
use std::env;
use std::sync::Arc;

/// Example Tables client that demonstrates cloud configuration usage.
#[derive(Debug)]
pub struct ExampleTablesClient {
    #[allow(dead_code)]
    endpoint: String,
    #[allow(dead_code)]
    pipeline: Vec<Arc<dyn azure_core::http::policies::Policy>>,
}

impl ExampleTablesClient {
    /// Create a new Tables client for Azure Public Cloud.
    pub fn new_for_public_cloud(
        account_name: &str,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> Self {
        let options = ClientOptions::default()
            .with_cloud_config(configurations::azure_public_cloud())
            .with_audience("https://storage.azure.com");
        
        Self::new_with_options(account_name, credential, options)
    }

    /// Create a new Tables client for Azure China Cloud.
    pub fn new_for_china_cloud(
        account_name: &str,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> Self {
        let options = ClientOptions::default()
            .with_cloud_config(configurations::azure_china_cloud())
            .with_audience("https://storage.azure.com");
        
        Self::new_with_options(account_name, credential, options)
    }

    /// Create a new Tables client for Azure US Government Cloud.
    pub fn new_for_us_government_cloud(
        account_name: &str,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
    ) -> Self {
        let options = ClientOptions::default()
            .with_cloud_config(configurations::azure_us_government_cloud())
            .with_audience("https://storage.azure.com");
        
        Self::new_with_options(account_name, credential, options)
    }

    /// Create a new Tables client with custom options.
    pub fn new_with_options(
        account_name: &str,
        credential: Arc<dyn azure_core::credentials::TokenCredential>,
        options: ClientOptions,
    ) -> Self {
        // Get the appropriate scope for the service
        let scope = options.get_auth_scope(Some("tables"))
            .unwrap_or_else(|| "https://storage.azure.com/.default".to_string());

        // Create authentication policy with the derived scope
        let auth_policy = BearerTokenCredentialPolicy::new(credential, [scope]);

        // Build the endpoint URL based on cloud configuration
        let cloud_config = options.cloud_config.unwrap_or_else(|| configurations::azure_public_cloud());
        let storage_suffix = get_storage_suffix_for_cloud(cloud_config);
        let endpoint = format!("https://{}.table.{}", account_name, storage_suffix);

        println!("Tables client created:");
        println!("  Endpoint: {}", endpoint);
        println!("  Cloud: {}", cloud_config.authority_host);

        Self {
            endpoint,
            pipeline: vec![Arc::new(auth_policy)],
        }
    }
}

/// Helper function to get the storage suffix for different clouds.
fn get_storage_suffix_for_cloud(cloud_config: &azure_core::cloud::CloudConfiguration) -> &'static str {
    // In a real implementation, this would be part of the cloud configuration
    match cloud_config.authority_host.as_str() {
        "https://login.microsoftonline.com/" => "core.windows.net",
        "https://login.chinacloudapi.cn/" => "core.chinacloudapi.cn", 
        "https://login.microsoftonline.de/" => "core.cloudapi.de",
        "https://login.microsoftonline.us/" => "core.usgovcloudapi.net",
        _ => "core.windows.net", // Default to public cloud
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Example Tables Service with Cloud Configuration ===\n");

    // Get credentials from environment variables
    let tenant_id = env::var("AZURE_TENANT_ID").unwrap_or_else(|_| "tenant-id".to_string());
    let client_id = env::var("AZURE_CLIENT_ID").unwrap_or_else(|_| "client-id".to_string());
    let client_secret = env::var("AZURE_CLIENT_SECRET").unwrap_or_else(|_| "client-secret".to_string());
    let account_name = "exampleaccount";

    // Example 1: Tables client for Public Cloud
    println!("1. Creating Tables client for Public Cloud:");
    let public_credential = ClientSecretCredential::new_for_public_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    let _public_tables = ExampleTablesClient::new_for_public_cloud(account_name, public_credential);

    // Example 2: Tables client for China Cloud
    println!("\n2. Creating Tables client for China Cloud:");
    let china_credential = ClientSecretCredential::new_for_china_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    let _china_tables = ExampleTablesClient::new_for_china_cloud(account_name, china_credential);

    // Example 3: Tables client for US Government Cloud
    println!("\n3. Creating Tables client for US Government Cloud:");
    let us_gov_credential = ClientSecretCredential::new_for_us_government_cloud(
        &tenant_id,
        client_id.clone(),
        Secret::new(client_secret.clone()),
    )?;
    let _us_gov_tables = ExampleTablesClient::new_for_us_government_cloud(account_name, us_gov_credential);

    // Example 4: Custom configuration
    println!("\n4. Creating Tables client with custom configuration:");
    let custom_credential = ClientSecretCredential::new_for_public_cloud(
        &tenant_id,
        client_id,
        Secret::new(client_secret),
    )?;
    
    let custom_options = ClientOptions::default()
        .with_cloud_config(configurations::azure_germany_cloud())
        .with_audience("https://storage.azure.com");
    
    let _custom_tables = ExampleTablesClient::new_with_options(account_name, custom_credential, custom_options);

    println!("\n✅ All Tables clients created successfully!");
    println!("\nKey benefits of cloud configuration:");
    println!("• Automatic endpoint selection based on cloud");
    println!("• Correct authentication scopes for each cloud");
    println!("• Easy switching between sovereign clouds");
    println!("• Consistent API across all cloud environments");

    Ok(())
}