# Cloud Configuration in Azure SDK for Rust

The Azure SDK for Rust now supports cloud configuration, enabling services to work across different Azure cloud environments including Azure Public Cloud, Azure China Cloud, Azure Germany Cloud, and Azure US Government Cloud.

## Overview

Cloud configuration provides:
- **Automatic endpoint selection** based on the target cloud environment
- **Correct authentication scopes** for each cloud and service
- **Easy switching** between sovereign clouds
- **Consistent API** across all cloud environments

## Quick Start

### Creating Credentials for Specific Clouds

```rust
use azure_core::credentials::Secret;
use azure_identity::ClientSecretCredential;

// Public Cloud (default)
let public_credential = ClientSecretCredential::new_for_public_cloud(
    "tenant-id",
    "client-id".to_string(),
    Secret::new("client-secret".to_string()),
)?;

// China Cloud
let china_credential = ClientSecretCredential::new_for_china_cloud(
    "tenant-id", 
    "client-id".to_string(),
    Secret::new("client-secret".to_string()),
)?;

// US Government Cloud
let us_gov_credential = ClientSecretCredential::new_for_us_government_cloud(
    "tenant-id",
    "client-id".to_string(), 
    Secret::new("client-secret".to_string()),
)?;
```

### Configuring Service Clients

```rust
use azure_core::cloud::configurations;
use azure_core::http::ClientOptions;

// Configure for China Cloud with Tables service
let options = ClientOptions::default()
    .with_cloud_config(configurations::azure_china_cloud())
    .with_audience("https://storage.azure.com");

// Automatically derive the correct OAuth scope
let scope = options.get_auth_scope(Some("tables"));
// Returns: "https://storage.azure.com/.default"
```

## Available Cloud Configurations

| Cloud | Configuration |
|-------|---------------|
| Azure Public Cloud | `configurations::azure_public_cloud()` |
| Azure China Cloud | `configurations::azure_china_cloud()` |
| Azure Germany Cloud | `configurations::azure_germany_cloud()` |
| Azure US Government Cloud | `configurations::azure_us_government_cloud()` |

## Service Audiences

Each cloud configuration includes service-specific audience URIs:

### Public Cloud
- **Storage/Tables**: `https://storage.azure.com`
- **Key Vault**: `https://vault.azure.net`
- **Resource Manager**: `https://management.azure.com`

### China Cloud  
- **Storage/Tables**: `https://storage.azure.com`
- **Key Vault**: `https://vault.azure.cn`
- **Resource Manager**: `https://management.chinacloudapi.cn`

### US Government Cloud
- **Storage/Tables**: `https://storage.azure.com`
- **Key Vault**: `https://vault.usgovcloudapi.net`
- **Resource Manager**: `https://management.usgovcloudapi.net`

## Complete Example: Tables Service

```rust
use azure_core::cloud::configurations;
use azure_core::credentials::Secret;
use azure_core::http::{ClientOptions, policies::BearerTokenCredentialPolicy};
use azure_identity::ClientSecretCredential;

// Create credential for China Cloud
let credential = ClientSecretCredential::new_for_china_cloud(
    "tenant-id",
    "client-id".to_string(),
    Secret::new("client-secret".to_string()),
)?;

// Configure client options for Tables service in China Cloud
let options = ClientOptions::default()
    .with_cloud_config(configurations::azure_china_cloud())
    .with_audience("https://storage.azure.com");

// Get the OAuth scope for authentication
let scope = options.get_auth_scope(Some("tables")).unwrap();
// scope = "https://storage.azure.com/.default"

// Create authentication policy with the derived scope
let auth_policy = BearerTokenCredentialPolicy::new(credential, [scope]);

// Build service endpoint (China Cloud Tables endpoint)
let endpoint = format!("https://{}.table.core.chinacloudapi.cn", account_name);
```

## Migration Guide

### From Deprecated Constants

The old `authority_hosts` and `resource_manager_endpoint` modules are deprecated. Migrate to cloud configurations:

```rust
// OLD (deprecated)
use azure_core::authority_hosts;
let authority = authority_hosts::AZURE_CHINA_CLOUD;

// NEW
use azure_core::cloud::configurations;
let cloud_config = configurations::azure_china_cloud();
let authority = &cloud_config.authority_host;
```

### Updating Service Clients

If you have existing service clients, update them to use cloud configuration:

```rust
// Before
let client = ServiceClient::new(endpoint, credential);

// After - with cloud configuration
let options = ClientOptions::default()
    .with_cloud_config(configurations::azure_china_cloud())
    .with_audience("https://service-audience.com");

let client = ServiceClient::new_with_options(endpoint, credential, options);
```

## Custom Cloud Configuration

For custom or private cloud environments:

```rust
use azure_core::cloud::CloudConfiguration;
use azure_core::http::Url;

let custom_cloud = CloudConfiguration::new(
    Url::parse("https://login.custom-cloud.com")?,
    Url::parse("https://management.custom-cloud.com")?,
    "https://management.custom-cloud.com".to_string(),
)
.with_service_audience("storage", "https://storage.custom-cloud.com")
.with_service_audience("keyvault", "https://vault.custom-cloud.com");

let options = ClientOptions::default()
    .with_cloud_config(&custom_cloud)
    .with_audience("https://storage.custom-cloud.com");
```

## Best Practices

1. **Use convenience methods** for well-known clouds: `new_for_china_cloud()`, etc.
2. **Let the SDK derive scopes** using `get_auth_scope()` instead of hardcoding
3. **Configure cloud at the client level** using `ClientOptions`
4. **Test with different clouds** to ensure your service works across environments

## Examples

See the examples in the repository:
- [`identity/examples/cloud_configuration.rs`](../sdk/identity/azure_identity/examples/cloud_configuration.rs) - Identity credential usage
- [`core/examples/tables_cloud_configuration.rs`](../sdk/core/azure_core/examples/tables_cloud_configuration.rs) - Service client integration

## Reference

- [Azure Cloud Configuration API](../sdk/core/azure_core/src/cloud.rs)
- [Identity Options API](../sdk/identity/azure_identity/src/options.rs)
- [Client Options API](../sdk/core/azure_core/src/http/options/mod.rs)