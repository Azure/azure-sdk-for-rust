// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuring and using `CosmosRuntime`.
//!
//! A [`CosmosRuntime`] is the shared background context that one or more
//! [`CosmosClient`] instances are built on top of. It owns the HTTP transport,
//! connection pool, CPU/memory sampler, and runtime-level defaults.
//!
//! This example demonstrates three patterns:
//!
//! 1. **Default (global) runtime** — omit `with_runtime` entirely and the
//!    client uses a process-wide shared runtime. This is the simplest path and
//!    is correct for most applications.
//! 2. **Custom runtime for the emulator** — build a runtime with relaxed TLS
//!    certificate validation so tests can target the local emulator.
//! 3. **Shared custom runtime** — build a runtime once and pass it to multiple
//!    clients that connect to different accounts but share the same transport.
//!
//! ## Running
//!
//! This example is illustrative — it compiles and runs but does not issue any
//! service calls. It demonstrates the configuration API surface.
//!
//! ```text
//! cargo run --example cosmos_runtime --features key_auth
//! ```

use azure_core::credentials::Secret;
use azure_data_cosmos::options::Region;
use azure_data_cosmos::options::{
    ConnectionPoolOptions, ServerCertificateValidation, UserAgentSuffix,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use std::error::Error;

/// The well-known emulator account key.
const EMULATOR_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // =========================================================================
    // 1. Default (global) runtime — the simplest and most common setup.
    // =========================================================================
    //
    // When you don't call `with_runtime`, the builder falls back to a
    // process-wide global runtime that is lazily initialized on the first
    // client build. This is the recommended path for most production apps.
    let _client_with_default_runtime = build_client_default_runtime().await?;
    println!("✓ built client with default (global) runtime");

    // =========================================================================
    // 2. Custom runtime for the emulator.
    // =========================================================================
    //
    // The Cosmos DB emulator uses a self-signed TLS certificate. In production
    // you should always validate server certificates, but for local development
    // you can relax validation for emulator endpoints only.
    let _emulator_client = build_client_for_emulator().await?;
    println!("✓ built client configured for the local emulator");

    // =========================================================================
    // 3. Shared runtime across multiple clients.
    // =========================================================================
    //
    // If your application requires custom connection pooling configuration
    // or other runtime settings, you can build a single `CosmosRuntime` and
    // pass it to multiple clients. This allows them to share the same
    // connection pool, HTTP/2 multiplexing, and background CPU sampler.
    //
    // This is only needed if you need to _change_ the runtime configuration.
    // By default, all clients share a global runtime with appropriate defaults.
    let (_client_a, _client_b) = build_clients_shared_runtime().await?;
    println!("✓ built two clients sharing the same custom runtime");

    Ok(())
}

/// Builds a client using the implicit global runtime (no `with_runtime` call).
#[cfg(feature = "key_auth")]
async fn build_client_default_runtime() -> Result<CosmosClient, Box<dyn Error>> {
    let endpoint: AccountEndpoint = "https://my-account.documents.azure.com:443/".parse()?;
    let account =
        AccountReference::with_authentication_key(endpoint, Secret::from(EMULATOR_KEY.to_owned()));

    // No `with_runtime` — the builder creates or reuses the global runtime.
    let client = CosmosClient::builder()
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    Ok(client)
}

#[cfg(not(feature = "key_auth"))]
async fn build_client_default_runtime() -> Result<CosmosClient, Box<dyn Error>> {
    Err("This example requires the `key_auth` feature. Rebuild with `--features key_auth`.".into())
}

/// Builds a client that targets the local emulator, using a custom runtime
/// with relaxed TLS certificate validation.
#[cfg(feature = "key_auth")]
async fn build_client_for_emulator() -> Result<CosmosClient, Box<dyn Error>> {
    // Build connection pool options that skip certificate validation when the
    // endpoint is a known emulator address (localhost / 127.0.0.1).
    let pool = ConnectionPoolOptions::builder()
        .with_server_certificate_validation(ServerCertificateValidation::RequiredUnlessEmulator)
        .build()?;

    // Attach the pool to a custom runtime.
    let runtime = CosmosRuntime::builder()
        .with_connection_pool(pool)
        .build()
        .await?;

    // Point at the local emulator endpoint.
    let endpoint: AccountEndpoint = "https://127.0.0.1:8081/".parse()?;
    let account =
        AccountReference::with_authentication_key(endpoint, Secret::from(EMULATOR_KEY.to_owned()));

    // Pass the custom runtime to the client builder.
    let client = CosmosClient::builder()
        .with_runtime(runtime)
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    Ok(client)
}

#[cfg(not(feature = "key_auth"))]
async fn build_client_for_emulator() -> Result<CosmosClient, Box<dyn Error>> {
    Err("This example requires the `key_auth` feature. Rebuild with `--features key_auth`.".into())
}

/// Builds two clients that talk to different accounts but share a single runtime.
///
/// Sharing a runtime means both clients reuse the same connection pool,
/// HTTP/2 multiplexing, and background CPU sampler — reducing overhead when
/// your application connects to multiple accounts.
#[cfg(feature = "key_auth")]
async fn build_clients_shared_runtime() -> Result<(CosmosClient, CosmosClient), Box<dyn Error>> {
    // Build a runtime once with any shared configuration you need.
    let runtime = CosmosRuntime::builder()
        .with_user_agent_suffix(UserAgentSuffix::new("multi-account-app"))
        .build()
        .await?;

    // First account.
    let endpoint_a: AccountEndpoint = "https://account-a.documents.azure.com:443/".parse()?;
    let account_a = AccountReference::with_authentication_key(
        endpoint_a,
        Secret::from(EMULATOR_KEY.to_owned()),
    );

    // Second account.
    let endpoint_b: AccountEndpoint = "https://account-b.documents.azure.com:443/".parse()?;
    let account_b = AccountReference::with_authentication_key(
        endpoint_b,
        Secret::from(EMULATOR_KEY.to_owned()),
    );

    // Both clients share the same runtime (clone is cheap — it's Arc-backed).
    let client_a = CosmosClient::builder()
        .with_runtime(runtime.clone())
        .build(account_a, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;

    let client_b = CosmosClient::builder()
        .with_runtime(runtime)
        .build(account_b, RoutingStrategy::ProximityTo(Region::WEST_US_2))
        .await?;

    Ok((client_a, client_b))
}

#[cfg(not(feature = "key_auth"))]
async fn build_clients_shared_runtime() -> Result<(CosmosClient, CosmosClient), Box<dyn Error>> {
    Err("This example requires the `key_auth` feature. Rebuild with `--features key_auth`.".into())
}
