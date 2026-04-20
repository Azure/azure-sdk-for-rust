# Migrating to azure_core v0.22+ from azure_core ≤0.21

This guide helps you migrate from the experimental-era `azure_core` (v0.1–0.21, published 2022–2024) to the current GA-track `azure_core` (v0.22+, 2025–present). If you are migrating from the community-era `azure_sdk_core` crate, start by replacing `azure_sdk_core` with `azure_core` in your `Cargo.toml` and then follow this guide.

## Table of Contents

- [Why Migrate](#why-migrate)
- [General Changes](#general-changes)
  - [Crate Name and Cargo.toml](#crate-name-and-cargotoml)
  - [use Statements and Module Layout](#use-statements-and-module-layout)
  - [Authentication](#authentication)
  - [Client Construction](#client-construction)
- [Common Scenarios](#common-scenarios)
  - [Making HTTP Requests via the Pipeline](#making-http-requests-via-the-pipeline)
  - [Pagination with Pager](#pagination-with-pager)
  - [Long-Running Operations with Poller](#long-running-operations-with-poller)
  - [Custom Policies](#custom-policies)
- [Error Handling](#error-handling)
- [Async Runtime and Concurrency](#async-runtime-and-concurrency)
- [Feature Flags](#feature-flags)
- [Additional Resources](#additional-resources)

## Why Migrate

- **Stable API design**: The ≤0.21 versions were experimental and never reached a stable API. The v0.22+ rewrite follows the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html) and targets GA stability.
- **Unified cross-service experience**: All official Azure SDK for Rust crates (identity, Key Vault, Storage, Event Hubs, Cosmos DB) are built on the same `azure_core` foundation, ensuring consistent error handling, retry, telemetry, and authentication.
- **Active maintenance**: The ≤0.21 versions are no longer maintained. Bug fixes, security patches, and new features only ship on the current track.
- **New capabilities**: Redesigned `Pager`/`Poller`, pluggable async runtime, `CloudConfiguration` for sovereign clouds, OpenTelemetry tracing, HTTP request/response logging with sanitization, and `SecretBytes` for sensitive data.

## General Changes

### Crate Name and Cargo.toml

The crate name remains `azure_core`. Update the version in your `Cargo.toml`:

```diff
 [dependencies]
-- azure_core = "0.21"
-+ azure_core = "0.35"
+- azure_core = "0.21"
++ azure_core = "0.35"
```

The default features have changed significantly. In ≤0.21, you typically needed to opt in to an HTTP backend. In v0.35, the defaults include `reqwest`, `reqwest_rustls`, and `tokio`:

```toml
# Current default features (you get these automatically):
# reqwest, reqwest_deflate, reqwest_gzip, reqwest_rustls, tokio
azure_core = "0.35"

# To opt out of defaults and pick your own:
azure_core = { version = "0.35", default-features = false, features = ["reqwest", "tokio"] }
```

If you were using the community-era `azure_sdk_core`, replace it entirely:

```diff
-- azure_sdk_core = "0.43"
-+ azure_core = "0.35"
+- azure_sdk_core = "0.43"
++ azure_core = "0.35"
```

### use Statements and Module Layout

The module structure changed substantially at v0.22. Key re-exports are now at the crate root, and HTTP types live under `azure_core::http`.

```rust
// Before (≤0.21): modules were organized differently
// use azure_core::prelude::*;
// use azure_core::HttpClient;
// use azure_core::policies::{Policy, PolicyResult};

// After (v0.35): clear module hierarchy
use azure_core::http::{
    ClientOptions, Method, Pipeline, Request, Response, StatusCode, Url,
};
use azure_core::http::headers;
use azure_core::http::policies;
use azure_core::credentials::TokenCredential;
use azure_core::{Error, Result};
```

Major module changes:

| ≤0.21 Location | v0.35 Location |
|---|---|
| `azure_core::HttpClient` | `azure_core::http::HttpClient` |
| `azure_core::prelude::*` | Removed — import types explicitly |
| `azure_core::Policy` | `typespec_client_core::http::policies::Policy` (re-exported) |
| `azure_core::Pageable` / `Pageable<T>` | `azure_core::http::Pager` |
| `azure_core::error::Error` | `azure_core::Error` (re-exported from `typespec_client_core`) |
| `azure_core::Context` | `azure_core::http::Context` |

### Authentication

Authentication in `azure_core` is defined through the `TokenCredential` trait. The trait signature changed:

```rust
// Before (≤0.21):
// #[async_trait]
// pub trait TokenCredential: Send + Sync {
//     async fn get_token(&self, resource: &str) -> Result<TokenResponse>;
// }

// After (v0.35):
use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};

#[async_trait::async_trait]
pub trait TokenCredential: Send + Sync + std::fmt::Debug {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken>;
}
```

Key differences:
- The method now takes `scopes: &[&str]` (a slice of scope strings) instead of a single `resource: &str`.
- An optional `TokenRequestOptions` parameter is available for advanced scenarios.
- The return type is `AccessToken` (with `.token: Secret` and `.expires_on: OffsetDateTime`) instead of `TokenResponse`.
- The trait now requires `Debug`.

### Client Construction

Service clients in the new SDK use `ClientOptions` for configuration:

```rust
// Before (≤0.21): varied per client, often ad-hoc configuration
// let client = SomeClient::new(endpoint, credential, options);

// After (v0.35): uniform ClientOptions pattern
use azure_core::http::{ClientOptions, ExponentialRetryOptions, RetryOptions};
use std::sync::Arc;

let options = ClientOptions {
    retry: RetryOptions::exponential(ExponentialRetryOptions {
        max_retries: 5,
        ..Default::default()
    }),
    ..Default::default()
};
```

The `ClientOptions` struct provides:

| Field | Description |
|---|---|
| `retry` | `RetryOptions` — exponential or fixed backoff configuration |
| `transport` | Optional custom HTTP transport (`Transport`) |
| `per_call_policies` | Custom policies executed once per API call |
| `per_try_policies` | Custom policies executed on each retry attempt |
| `user_agent` | `UserAgentOptions` for telemetry |
| `instrumentation` | `InstrumentationOptions` for distributed tracing |
| `logging` | `LoggingOptions` for header/query parameter sanitization |
| `cloud` | Optional `CloudConfiguration` for sovereign clouds |

## Common Scenarios

### Making HTTP Requests via the Pipeline

The pipeline model has been redesigned. The `Pipeline` struct is the core abstraction for sending HTTP requests through a chain of policies.

```rust
// Before (≤0.21): HttpClient trait used directly
// let client = azure_core::new_http_client();
// let response = client.execute_request(&request).await?;

// After (v0.35): Pipeline wraps policies and transport
use azure_core::http::{new_http_client, Pipeline, ClientOptions, Request, Method, Url, Context};

// Most users don't create Pipeline directly — service clients handle this.
// If you need raw HTTP access:
let client = new_http_client(None);
let request = Request::new(Url::parse("https://example.com")?, Method::Get);
let response = client.execute_request(&request).await?;
let status = response.status();
let body = response.into_body().collect_string().await?;
```

### Pagination with Pager

The pagination model changed from a `Stream`-based `Pageable<T>` to the new `Pager` type.

```rust
// Before (≤0.21): Pageable returned a Stream of pages
// use azure_core::Pageable;
// use futures::StreamExt;
// let mut stream = client.list().into_stream();
// while let Some(page) = stream.next().await {
//     let page = page?;
//     for item in page.items {
//         println!("{:?}", item);
//     }
// }

// After (v0.35): Pager provides PageIterator and ItemIterator
use azure_core::http::pager::{Pager, PageIterator, ItemIterator};

// Service clients return a Pager. Iterate pages:
let pager: Pager<MyPageResponse> = client.list_items(None).await?;
let mut pages = pager.into_pages();
while let Some(page) = pages.next().await {
    let page = page?;
    // Process each page
}

// Or iterate items directly:
let pager: Pager<MyPageResponse> = client.list_items(None).await?;
let mut items = pager.into_items();
while let Some(item) = items.next().await {
    let item = item?;
    // Process each item
}
```

### Long-Running Operations with Poller

Long-running operations now use the `Poller` type instead of ad-hoc polling patterns.

```rust
// After (v0.35): Poller for long-running operations
use azure_core::http::Poller;

// Service clients return a Poller for LROs:
let poller: Poller<MyOperationResult> = client.begin_operation(params, None).await?;
let result = poller.wait().await?;
```

### Custom Policies

The policy trait moved to `typespec_client_core`, but is re-exported for use:

```rust
// Before (≤0.21):
// use azure_core::policies::{Policy, PolicyResult};
// use azure_core::{Context, Request};

// After (v0.35):
use azure_core::http::{ClientOptions, Context, Request, Response};
use std::sync::Arc;
use typespec_client_core::http::policies::{Policy, PolicyResult};

// Implement a custom policy
#[derive(Debug)]
struct MyPolicy;

#[async_trait::async_trait]
impl Policy for MyPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // Add a custom header before sending
        request.insert_header("x-custom-header", "value");
        next[0].send(ctx, request, &next[1..]).await
    }
}

// Use it in ClientOptions:
let options = ClientOptions {
    per_call_policies: vec![Arc::new(MyPolicy)],
    ..Default::default()
};
```

## Error Handling

Error handling changed significantly. The old `azure_core::error::Error` was replaced with a unified `Error` type re-exported from `typespec_client_core`.

```rust
// Before (≤0.21): various error patterns
// use azure_core::error::Error;
// match result {
//     Err(e) => eprintln!("Error: {}", e),
// }

// After (v0.35): azure_core::Error with ErrorKind for matching
use azure_core::{Error, Result};
use azure_core::error::ErrorKind;

fn handle_error(err: Error) {
    match err.kind() {
        ErrorKind::HttpResponse { status, error_code, .. } => {
            eprintln!("HTTP {status}: code={error_code:?}");
        }
        ErrorKind::Credential => {
            eprintln!("Authentication failed: {err}");
        }
        ErrorKind::Io => {
            eprintln!("I/O error: {err}");
        }
        ErrorKind::DataConversion => {
            eprintln!("Serialization error: {err}");
        }
        ErrorKind::Other => {
            eprintln!("Other error: {err}");
        }
        _ => {
            eprintln!("Unexpected error: {err}");
        }
    }
}
```

The `ErrorKind` enum includes:

| Variant | Description |
|---|---|
| `HttpResponse { status, error_code, raw_response }` | HTTP error with status code and optional error code |
| `Credential` | Authentication/authorization failure |
| `Io` | I/O error |
| `DataConversion` | Serialization/deserialization error |
| `Other` | Catch-all for other errors |

Use `Error::with_message` and `Error::with_error` to create errors:

```rust
use azure_core::{Error, error::ErrorKind};

// Create from a message
let err = Error::with_message(ErrorKind::Other, "something went wrong");

// Wrap another error
let io_err = std::io::Error::other("disk full");
let err = Error::with_error(ErrorKind::Io, io_err, "failed to write cache");
```

## Async Runtime and Concurrency

The v0.22+ SDK requires the [tokio](https://tokio.rs) async runtime by default. This is a change from ≤0.21, which had a more flexible (but less reliable) runtime story.

```rust
// The tokio feature is enabled by default in azure_core.
// Your binary crate must include tokio as a dependency:

// Cargo.toml:
// [dependencies]
// azure_core = "0.35"
// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // All Azure SDK operations are async
    let credential = azure_identity::DeveloperToolsCredential::new(None)?;
    // ... use credential with service clients
    Ok(())
}
```

Cancellation in Rust is handled by dropping futures — there is no explicit cancellation token pattern like in .NET or Java. Simply drop the future or use `tokio::select!` for timeouts:

```rust
use std::time::Duration;

let result = tokio::time::timeout(
    Duration::from_secs(30),
    client.get_item("key", None),
).await;

match result {
    Ok(Ok(item)) => println!("Got item: {:?}", item),
    Ok(Err(e)) => eprintln!("SDK error: {e}"),
    Err(_) => eprintln!("Request timed out"),
}
```

## Feature Flags

| Feature | Default | Description |
|---|---|---|
| `reqwest` | ✅ | Enable the [reqwest](https://crates.io/crates/reqwest) HTTP client |
| `reqwest_deflate` | ✅ | Enable deflate decompression for reqwest |
| `reqwest_gzip` | ✅ | Enable gzip decompression for reqwest |
| `reqwest_rustls` | ✅ | Use [rustls](https://crates.io/crates/rustls) TLS backend (instead of OpenSSL) |
| `tokio` | ✅ | Enable [tokio](https://tokio.rs) async runtime support |
| `xml` | ❌ | Enable XML serialization/deserialization support |
| `hmac_rust` | ❌ | Pure-Rust HMAC-SHA256 implementation (via `sha2` + `hmac` crates) |
| `hmac_openssl` | ❌ | OpenSSL-backed HMAC-SHA256 implementation |
| `decimal` | ❌ | Decimal number support |
| `debug` | ❌ | Additional debug utilities |
| `test` | ❌ | Test utilities (for SDK development) |

To switch from the default `rustls` TLS backend to OpenSSL:

```toml
azure_core = { version = "0.35", default-features = false, features = [
    "reqwest",
    "reqwest_deflate",
    "reqwest_gzip",
    "tokio",
] }
```

For services that require HMAC authentication (e.g., Storage with shared key):

```toml
azure_core = { version = "0.35", features = ["hmac_rust"] }
# Or for OpenSSL-backed HMAC:
# azure_core = { version = "0.35", features = ["hmac_openssl"] }
```

## Additional Resources

- [azure_core README](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core/README.md)
- [Examples directory](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core/examples)
- [API documentation on docs.rs](https://docs.rs/azure_core/latest/)
- [CHANGELOG.md](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/core/azure_core/CHANGELOG.md)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
