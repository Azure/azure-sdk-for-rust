# Migrating to azure_identity v0.22+ from azure_identity ≤0.21

This guide helps you migrate from the experimental-era `azure_identity` (v0.1–0.21, published 2022–2024) to the current GA-track `azure_identity` (v0.22+, 2025–present). If you are migrating from the community-era `azure_sdk_auth_aad` crate, replace it entirely with `azure_identity` and follow this guide.

## Table of Contents

- [Why Migrate](#why-migrate)
- [General Changes](#general-changes)
  - [Crate Name and Cargo.toml](#crate-name-and-cargotoml)
  - [use Statements and Module Layout](#use-statements-and-module-layout)
  - [Authentication](#authentication)
  - [Client Construction](#client-construction)
- [Common Scenarios](#common-scenarios)
  - [Authenticating with DeveloperToolsCredential](#authenticating-with-developertoolscredential)
  - [Authenticating with AzureCliCredential](#authenticating-with-azureclicredential)
  - [Authenticating with ClientSecretCredential](#authenticating-with-clientsecretcredential)
  - [Authenticating with ManagedIdentityCredential](#authenticating-with-managedidentitycredential)
  - [Authenticating with WorkloadIdentityCredential](#authenticating-with-workloadidentitycredential)
  - [Authenticating with AzurePipelinesCredential](#authenticating-with-azurepipelinescredential)
  - [Authenticating with ClientCertificateCredential](#authenticating-with-clientcertificatecredential)
  - [Using a Credential with a Service Client](#using-a-credential-with-a-service-client)
- [Error Handling](#error-handling)
- [Async Runtime and Concurrency](#async-runtime-and-concurrency)
- [Feature Flags](#feature-flags)
- [Additional Resources](#additional-resources)

## Why Migrate

- **Stable API design**: The ≤0.21 versions were experimental. The v0.22+ rewrite follows the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html) and targets GA stability.
- **Unified credential model**: All credentials now implement `azure_core::credentials::TokenCredential` with a consistent `get_token(&self, scopes, options)` signature.
- **New credentials**: `AzurePipelinesCredential`, `AzureDeveloperCliCredential`, `ClientAssertionCredential`, and `DeveloperToolsCredential` are new in the GA track.
- **Improved managed identity**: `ManagedIdentityCredential` now unifies App Service, IMDS, and VM managed identity behind a single type.
- **Token caching**: Built-in token cache in credentials like `ClientSecretCredential` reduces token requests.
- **Active maintenance**: The ≤0.21 versions are no longer maintained. Bug fixes and security patches only ship on the current track.

## General Changes

### Crate Name and Cargo.toml

The crate name remains `azure_identity`. Update the version:

```diff
 [dependencies]
- azure_identity = "0.21"
+ azure_identity = "0.35"
```

If you were using `azure_sdk_auth_aad` (community era):

```diff
- azure_sdk_auth_aad = "0.47"
+ azure_identity = "0.35"
```

### use Statements and Module Layout

The ≤0.21 `azure_identity` had a `DefaultAzureCredential` as the primary entry point. In v0.35, `DeveloperToolsCredential` replaces it for local development scenarios, and credentials are exported directly from the crate root.

```rust
// Before (≤0.21):
// use azure_identity::DefaultAzureCredential;
// use azure_identity::token_credentials::AzureCliCredential;
// let credential = DefaultAzureCredential::default();

// After (v0.35): credentials are at the crate root
use azure_identity::DeveloperToolsCredential;
use azure_identity::AzureCliCredential;
use azure_identity::ClientSecretCredential;
use azure_identity::ManagedIdentityCredential;
use azure_identity::WorkloadIdentityCredential;
use azure_identity::AzurePipelinesCredential;
use azure_identity::ClientAssertionCredential;
```

### Authentication

The `TokenCredential` trait changed significantly between ≤0.21 and v0.35:

```rust
// Before (≤0.21):
// #[async_trait]
// pub trait TokenCredential: Send + Sync {
//     async fn get_token(&self, resource: &str) -> Result<TokenResponse>;
// }
// let token = credential.get_token("https://management.azure.com/").await?;

// After (v0.35):
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};

// The trait now takes scopes (a slice) instead of a single resource string:
let token: AccessToken = credential
    .get_token(&["https://management.azure.com/.default"], None)
    .await?;

// Access the token value via the Secret wrapper:
println!("Token: {}", token.token.secret());
println!("Expires: {}", token.expires_on);
```

Key changes:
- `resource: &str` → `scopes: &[&str]` (append `/.default` to resource URIs)
- `TokenResponse` → `AccessToken { token: Secret, expires_on: OffsetDateTime }`
- Optional `TokenRequestOptions` parameter added
- `TokenCredential` now requires `Debug`

### Client Construction

All credentials in v0.35 return `Arc<Self>` from their constructors and take an optional options struct:

```rust
// Before (≤0.21): constructors returned Self
// let credential = AzureCliCredential::new();

// After (v0.35): constructors return Result<Arc<Self>>
use azure_identity::{AzureCliCredential, AzureCliCredentialOptions};
use std::sync::Arc;

let credential: Arc<AzureCliCredential> = AzureCliCredential::new(None)?;

// With options:
let credential = AzureCliCredential::new(Some(AzureCliCredentialOptions {
    tenant_id: Some("your-tenant-id".to_string()),
    subscription: Some("your-subscription-id".to_string()),
    ..Default::default()
}))?;
```

## Common Scenarios

### Authenticating with DeveloperToolsCredential

`DeveloperToolsCredential` is the recommended credential for local development. It tries `AzureCliCredential` and then `AzureDeveloperCliCredential`, caching whichever succeeds first.

```rust
// Before (≤0.21): DefaultAzureCredential
// use azure_identity::DefaultAzureCredential;
// let credential = DefaultAzureCredential::default();

// After (v0.35): DeveloperToolsCredential
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let token = credential
    .get_token(&["https://management.azure.com/.default"], None)
    .await?;
```

### Authenticating with AzureCliCredential

```rust
// Before (≤0.21):
// use azure_identity::AzureCliCredential;
// let credential = AzureCliCredential::new();
// let token = credential.get_token("https://management.azure.com/").await?;

// After (v0.35):
use azure_identity::{AzureCliCredential, AzureCliCredentialOptions};

let credential = AzureCliCredential::new(None)?;
let token = credential
    .get_token(&["https://management.azure.com/.default"], None)
    .await?;

// With a specific tenant:
let credential = AzureCliCredential::new(Some(AzureCliCredentialOptions {
    tenant_id: Some("your-tenant-id".to_string()),
    ..Default::default()
}))?;
```

### Authenticating with ClientSecretCredential

```rust
// Before (≤0.21):
// use azure_identity::ClientSecretCredential;
// let credential = ClientSecretCredential::new(
//     tenant_id.to_string(),
//     client_id.to_string(),
//     client_secret.to_string(),
// );

// After (v0.35):
use azure_identity::{ClientSecretCredential, ClientSecretCredentialOptions};
use azure_core::credentials::Secret;

let credential = ClientSecretCredential::new(
    "your-tenant-id",               // &str (not String)
    "your-client-id".to_string(),   // String
    Secret::new("your-secret"),     // Secret (not String)
    None,                            // Option<ClientSecretCredentialOptions>
)?;

let token = credential
    .get_token(&["https://vault.azure.net/.default"], None)
    .await?;
```

Key differences:
- `tenant_id` is now `&str` (not `String`)
- `secret` is now `azure_core::credentials::Secret` (not a plain `String`) for secure handling
- Constructor returns `Result<Arc<Self>>` (can fail on invalid input)
- Built-in token caching — repeat calls reuse cached tokens

### Authenticating with ManagedIdentityCredential

`ManagedIdentityCredential` now unifies App Service managed identity and IMDS (VM) managed identity behind a single constructor.

```rust
// Before (≤0.21):
// use azure_identity::ManagedIdentityCredential;
// let credential = ManagedIdentityCredential::default();
// // or for user-assigned:
// let credential = ManagedIdentityCredential::default()
//     .with_client_id("your-client-id");

// After (v0.35): system-assigned
use azure_identity::ManagedIdentityCredential;

let credential = ManagedIdentityCredential::new(None)?;

// User-assigned by client ID:
use azure_identity::{ManagedIdentityCredential, ManagedIdentityCredentialOptions, UserAssignedId};

let credential = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
    user_assigned_id: Some(UserAssignedId::ClientId("your-client-id".to_string())),
    ..Default::default()
}))?;

// User-assigned by object ID:
let credential = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
    user_assigned_id: Some(UserAssignedId::ObjectId("your-object-id".to_string())),
    ..Default::default()
}))?;

// User-assigned by resource ID (not supported on App Service):
let credential = ManagedIdentityCredential::new(Some(ManagedIdentityCredentialOptions {
    user_assigned_id: Some(UserAssignedId::ResourceId("your-resource-id".to_string())),
    ..Default::default()
}))?;
```

### Authenticating with WorkloadIdentityCredential

`WorkloadIdentityCredential` is new in the GA track. It authenticates [Kubernetes workload identities](https://learn.microsoft.com/azure/aks/workload-identity-overview).

```rust
// No ≤0.21 equivalent — this credential is new.

// After (v0.35):
use azure_identity::{WorkloadIdentityCredential, WorkloadIdentityCredentialOptions};

// Reads AZURE_CLIENT_ID, AZURE_TENANT_ID, and AZURE_FEDERATED_TOKEN_FILE
// from environment variables (set by AKS workload identity webhook):
let credential = WorkloadIdentityCredential::new(None)?;

// Or with explicit options:
let credential = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
    client_id: Some("your-client-id".to_string()),
    tenant_id: Some("your-tenant-id".to_string()),
    token_file_path: Some("/var/run/secrets/tokens/azure-token".into()),
    ..Default::default()
}))?;
```

### Authenticating with AzurePipelinesCredential

`AzurePipelinesCredential` is new in the GA track. It authenticates Azure Pipelines [service connections](https://learn.microsoft.com/azure/devops/pipelines/library/service-endpoints).

```rust
// No ≤0.21 equivalent — this credential is new.

// After (v0.35):
use azure_identity::AzurePipelinesCredential;

let credential = AzurePipelinesCredential::new(
    "your-tenant-id".to_string(),
    "your-client-id".to_string(),
    "service-connection-id",
    system_access_token, // &str or Secret
    None,
)?;
```

### Authenticating with ClientCertificateCredential

`ClientCertificateCredential` requires the `client_certificate` feature flag.

```rust
// After (v0.35):
// In Cargo.toml: azure_identity = { version = "0.35", features = ["client_certificate"] }

use azure_identity::ClientCertificateCredential;
use azure_core::credentials::SecretBytes;

let cert_data = std::fs::read("path/to/cert.pfx")?;
let credential = ClientCertificateCredential::new(
    "your-tenant-id".to_string(),
    "your-client-id".to_string(),
    SecretBytes::new(cert_data),
    None,
)?;
```

### Using a Credential with a Service Client

All Azure SDK service clients accept any `Arc<dyn TokenCredential>`:

```rust
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::SecretClient;

let credential = DeveloperToolsCredential::new(None)?;
let client = SecretClient::new(
    "https://my-vault.vault.azure.net/",
    credential.clone(),
    None,
)?;
```

## Error Handling

Credential errors use `azure_core::error::ErrorKind::Credential`. Each credential wraps its errors with a descriptive message and troubleshooting link:

```rust
use azure_core::error::ErrorKind;

match credential.get_token(&["https://management.azure.com/.default"], None).await {
    Ok(token) => println!("Token acquired"),
    Err(err) => {
        match err.kind() {
            ErrorKind::Credential => {
                // Authentication-specific error
                eprintln!("Auth failed: {err}");
                // Error messages include troubleshooting links, e.g.:
                // "AzureCliCredential authentication failed. Please run 'az login'...
                //  To troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#azure-cli"
            }
            ErrorKind::HttpResponse { status, error_code, .. } => {
                eprintln!("HTTP error {status}: {error_code:?}");
            }
            _ => eprintln!("Other error: {err}"),
        }

        // Unwrap the inner error for more detail:
        if let Some(inner) = err.downcast_ref::<azure_core::Error>() {
            eprintln!("Inner: {inner}");
        }
    }
}
```

The error chain for credential failures:
1. **Outer error**: `ErrorKind::Credential` with formatted message including credential name and troubleshooting link
2. **Inner error**: The underlying cause (e.g., `ErrorKind::HttpResponse` for Entra ID errors, `ErrorKind::Io` for CLI process failures)

## Async Runtime and Concurrency

`azure_identity` requires the [tokio](https://tokio.rs) runtime (inherited from `azure_core`). All credential methods are `async`.

```rust
// Cargo.toml:
// [dependencies]
// azure_identity = "0.35"
// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = azure_identity::DeveloperToolsCredential::new(None)?;
    let token = credential
        .get_token(&["https://management.azure.com/.default"], None)
        .await?;
    println!("Token expires at: {}", token.expires_on);
    Ok(())
}
```

Credentials are `Send + Sync` and wrapped in `Arc`, so they can be shared across tasks safely:

```rust
use std::sync::Arc;
use azure_core::credentials::TokenCredential;

let credential: Arc<dyn TokenCredential> = azure_identity::DeveloperToolsCredential::new(None)?;

let cred_clone = credential.clone();
let handle = tokio::spawn(async move {
    cred_clone
        .get_token(&["https://storage.azure.com/.default"], None)
        .await
});
```

## Feature Flags

| Feature | Default | Description |
|---|---|---|
| `default` | ✅ | Inherits `azure_core/default` (reqwest + rustls + tokio) |
| `tokio` | ❌ | Enable tokio integration for process execution (enabled via `azure_core/tokio` in default) |
| `client_certificate` | ❌ | Enable `ClientCertificateCredential` (requires OpenSSL) |

To use `ClientCertificateCredential`:

```toml
azure_identity = { version = "0.35", features = ["client_certificate"] }
```

## Additional Resources

- [azure_identity README](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/README.md)
- [Troubleshooting guide](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/TROUBLESHOOTING.md)
- [Examples directory](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/examples)
- [API documentation on docs.rs](https://docs.rs/azure_identity/latest/)
- [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/CHANGELOG.md)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
