# Migrating to azure_identity v1.0+ from azure_identity ≤0.21

This guide helps you migrate from the experimental `azure_identity` (v0.1–0.21, published 2022–2024) to `azure_identity` v1.0.0 or newer. The significant API changes first arrived in the supported releases before `v1.0.0`, and that surface is what `v1.0.0` stabilizes. If you are migrating from the community `azure_sdk_auth_aad` crate, replace it entirely with `azure_identity` and follow this guide.

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

- **Stable API design**: The ≤0.21 versions were experimental. `azure_identity` v1.0.0 follows the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html) and stabilizes the supported API shape.
- **Unified credential model**: All credentials now implement `azure_core::credentials::TokenCredential` with a consistent `get_token(&self, scopes, options)` signature.
- **Expanded credential surface**: The v1.0.0 API includes newer credentials such as `AzurePipelinesCredential`, `AzureDeveloperCliCredential`, and `DeveloperToolsCredential`, while continuing to support credentials such as `ClientAssertionCredential`.
- **Improved managed identity**: `ManagedIdentityCredential` now unifies App Service, IMDS, and VM managed identity behind a single type.
- **Token caching**: Built-in token cache in credentials like `ClientSecretCredential` reduces token requests.
- **Active maintenance**: The ≤0.21 versions are no longer maintained. Bug fixes and security patches only ship on the current track.

## General Changes

### Crate Name and Cargo.toml

The crate name remains `azure_identity`. Update the version:

```diff
 [dependencies]
- azure_identity = "0.21"
+ azure_identity = "1.0.0"
```

If you were using `azure_sdk_auth_aad` (community era):

```diff
- azure_sdk_auth_aad = "0.47"
+ azure_identity = "1.0.0"
```

### use Statements and Module Layout

The ≤0.21 `azure_identity` had `DefaultAzureCredential` as the primary entry point. `DefaultAzureCredential` and `ChainedTokenCredential` were intentionally removed before 1.0 because automatically trying developer and deployed credentials can be surprising and unsafe. Prefer the specific credential you intend to use in production; for local development, use `DeveloperToolsCredential`.

```rust ignore
use azure_core::credentials::TokenCredential;
use azure_identity::{DeveloperToolsCredential, ManagedIdentityCredential};
use std::sync::Arc;

let credential: Arc<dyn TokenCredential> = match std::env::var("ENV").as_deref() {
    Ok("production") => ManagedIdentityCredential::new(None)?,
    _ => DeveloperToolsCredential::new(None)?,
};
```

The v1.0.0 credentials are exported directly from the crate root:

```rust ignore use_statements
// Before (≤0.21):
// use azure_identity::DefaultAzureCredential;
// use azure_identity::AzureCliCredential;
// Very early releases also exposed some credentials from `azure_identity::token_credentials`.
// let credential = DefaultAzureCredential::default();

// After (v1.0.0): credentials are at the crate root
use azure_identity::AzureCliCredential;
use azure_identity::AzureDeveloperCliCredential;
use azure_identity::AzurePipelinesCredential;
use azure_identity::ClientAssertionCredential;
#[cfg(feature = "client_certificate")]
use azure_identity::ClientCertificateCredential;
use azure_identity::ClientSecretCredential;
use azure_identity::DeveloperToolsCredential;
use azure_identity::ManagedIdentityCredential;
use azure_identity::WorkloadIdentityCredential;
```

### Authentication

The `TokenCredential` trait changed significantly between ≤0.21 and v1.0.0:

```rust ignore authentication
// Before (≤0.21):
// #[async_trait]
// pub trait TokenCredential: Send + Sync {
//     async fn get_token(&self, resource: &str) -> Result<TokenResponse>;
// }
// let token = credential.get_token("https://management.azure.com/").await?;

// After (v1.0.0):
use azure_core::credentials::{AccessToken, TokenCredential};

let credential = azure_identity::DeveloperToolsCredential::new(None)?;
// The trait now takes scopes (a slice) instead of a single resource string:
let token: AccessToken = credential
    .get_token(&["https://management.azure.com/.default"], None)
    .await?;

println!("Expires: {}", token.expires_on);
// AccessToken::token is wrapped in Secret. Only unwrap it for non-production debugging.
let raw_token = token.token.secret();
println!("Debug-only token length: {}", raw_token.len());
```

Key changes:

- `resource: &str` → `scopes: &[&str]` (append `/.default` to resource URIs)
- `TokenResponse` → `AccessToken { token: Secret, expires_on: OffsetDateTime }`
- Optional `TokenRequestOptions` parameter added
- `TokenCredential` now requires `Debug`

### Client Construction

Credential constructors in v1.0.0 return `Result<Arc<Self>>`. When a credential has configurable behavior, that configuration is passed through an options struct, typically as `Option<...>`:

```rust ignore client_construction
// Before (≤0.21): constructors returned Self
// let credential = AzureCliCredential::new();

// After (v1.0.0): constructors return Result<Arc<Self>>
use azure_identity::{AzureCliCredential, AzureCliCredentialOptions};

let credential = AzureCliCredential::new(None)?;

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

```rust ignore developer_tools
// Before (≤0.21): DefaultAzureCredential
// use azure_identity::DefaultAzureCredential;
// let credential = DefaultAzureCredential::default();

// After (v1.0.0): DeveloperToolsCredential for local development
use azure_core::credentials::TokenCredential;
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let token = credential
    .get_token(&["https://management.azure.com/.default"], None)
    .await?;
```

### Authenticating with AzureCliCredential

```rust ignore azure_cli
// Before (≤0.21):
// use azure_identity::AzureCliCredential;
// let credential = AzureCliCredential::new();
// let token = credential.get_token("https://management.azure.com/").await?;

// After (v1.0.0):
use azure_core::credentials::TokenCredential;
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

```rust ignore client_secret
// Before (≤0.21):
// use azure_identity::ClientSecretCredential;
// let credential = ClientSecretCredential::new(
//     tenant_id.to_string(),
//     client_id.to_string(),
//     client_secret.to_string(),
// );

// After (v1.0.0):
use azure_core::credentials::{Secret, TokenCredential};
use azure_identity::ClientSecretCredential;

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

```rust ignore managed_identity
// Before (≤0.21):
// use azure_identity::ManagedIdentityCredential;
// let credential = ManagedIdentityCredential::default();
// // or for user-assigned:
// let credential = ManagedIdentityCredential::default()
//     .with_client_id("your-client-id");

// After (v1.0.0):
use azure_identity::{ManagedIdentityCredential, ManagedIdentityCredentialOptions, UserAssignedId};

let credential = ManagedIdentityCredential::new(None)?;

// User-assigned by client ID:
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

`WorkloadIdentityCredential` is part of the v1.0.0 surface. It authenticates [Kubernetes workload identities](https://learn.microsoft.com/azure/aks/workload-identity-overview).

```rust ignore workload_identity
// No ≤0.21 equivalent — this credential is new.

// After (v1.0.0):
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

`AzurePipelinesCredential` is part of the v1.0.0 surface. It authenticates Azure Pipelines [service connections](https://learn.microsoft.com/azure/devops/pipelines/library/service-endpoints).

```rust ignore azure_pipelines
// No ≤0.21 equivalent — this credential is new.

// After (v1.0.0):
use azure_identity::AzurePipelinesCredential;

let system_access_token = "system-access-token";
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

```rust ignore client_certificate
// After (v1.0.0):
// In Cargo.toml: azure_identity = { version = "1.0.0", features = ["client_certificate"] }

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

```rust ignore service_client
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

Credential `get_token()` errors are typically wrapped as `azure_core::error::ErrorKind::Credential`. The wrapped inner error can still carry details such as an HTTP response:

```rust ignore error_handling
use azure_core::{credentials::TokenCredential, error::ErrorKind};
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;

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
                if let Some(inner) = err.downcast_ref::<azure_core::Error>() {
                    if let ErrorKind::HttpResponse {
                        status,
                        error_code,
                        ..
                    } = inner.kind()
                    {
                        eprintln!("Inner HTTP error {status}: {error_code:?}");
                    }
                }
            }
            _ => eprintln!("Other error: {err}"),
        }
    }
}
```

The error chain for credential failures:

1. **Outer error**: Usually `ErrorKind::Credential`, with a formatted message including the credential name and troubleshooting link
2. **Inner error**: The underlying cause (for example `ErrorKind::HttpResponse` for Entra ID errors or `ErrorKind::Io` for CLI process failures)

## Async Runtime and Concurrency

`azure_identity` requires the [tokio](https://tokio.rs) runtime (inherited from `azure_core`). All credential methods are `async`.

```rust ignore async_runtime
// Cargo.toml:
//
// [dependencies]
// azure_identity = "1.0.0"
// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

use azure_core::credentials::TokenCredential;

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

```rust ignore concurrency
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

| Feature              | Default | Description                                                                                      |
|----------------------|---------|--------------------------------------------------------------------------------------------------|
| `default`            | ✅      | Enables `azure_core/default`.                                                                    |
| `tokio`              | ❌      | Enables tokio-based process execution (`tokio::process`) for CLI and developer-tool credentials. |
| `client_certificate` | ❌      | Enables `ClientCertificateCredential` (requires OpenSSL).                                        |

To use `ClientCertificateCredential`:

```toml
azure_identity = { version = "1.0.0", features = ["client_certificate"] }
```

## Additional Resources

- [azure_identity README](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/README.md)
- [Troubleshooting guide](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/TROUBLESHOOTING.md)
- [Examples directory](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/examples)
- [API documentation on docs.rs](https://docs.rs/azure_identity/latest/)
- [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity/CHANGELOG.md)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
