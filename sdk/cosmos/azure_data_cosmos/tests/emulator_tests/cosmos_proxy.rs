// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests verifying proxy configuration behavior.
//! These tests run against the Cosmos DB emulator.

use super::framework;

use framework::{TestClient, TestOptions, CONNECTION_STRING_ENV_VAR, EMULATOR_CONNECTION_STRING};
use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::net::TcpListener;

/// Verifies that a client built with default settings does not route
/// requests through an HTTP proxy, even when `HTTPS_PROXY` is set.
#[tokio::test]
pub async fn proxy_disabled_by_default_ignores_env() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let connect_count = Arc::new(AtomicU32::new(0));
    let counter = Arc::clone(&connect_count);

    let accept_handle = tokio::spawn(async move {
        while let Ok((_stream, _addr)) = listener.accept().await {
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });

    let proxy_key = "HTTPS_PROXY";
    let prev = std::env::var(proxy_key).ok();
    std::env::set_var(proxy_key, format!("http://127.0.0.1:{port}"));

    // Run a real emulator test with proxy disabled (default).
    // TestClient::run uses the default CosmosClientBuilder which has no_proxy().
    let result = TestClient::run_with_options(
        async |run_context| {
            let client = run_context.client();
            let _ = client.database_client("nonexistent").read(None).await;
            Ok(())
        },
        TestOptions::for_emulator(),
    )
    .await;

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    match prev {
        Some(v) => std::env::set_var(proxy_key, v),
        None => std::env::remove_var(proxy_key),
    }

    accept_handle.abort();

    result?;
    assert_eq!(
        connect_count.load(Ordering::SeqCst),
        0,
        "Default builder should not route through proxy"
    );

    Ok(())
}

/// Verifies that a client built with `with_proxy_allowed(true)`
/// routes requests through the proxy specified by `HTTPS_PROXY`.
#[tokio::test]
pub async fn proxy_enabled_routes_through_proxy() -> Result<(), Box<dyn Error>> {
    // Skip on the vnext (Linux) emulator pipeline: the vnext gateway does
    // not honor an outbound proxy in the same way the legacy emulator does
    // and the test consistently fails there. Keep enabled for the legacy
    // emulator and for any non-emulator backend.
    if std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() == Ok("vnext") {
        eprintln!("Skipping proxy_enabled test on vnext emulator.");
        return Ok(());
    }
    // Skip when test mode is "skipped" or no connection string is available.
    let test_mode = std::env::var("AZURE_COSMOS_TEST_MODE").unwrap_or_default();
    let conn_string_available = std::env::var(CONNECTION_STRING_ENV_VAR).is_ok();
    if test_mode == "skipped" || (!conn_string_available && test_mode != "required") {
        eprintln!("Skipping proxy_enabled test: no emulator connection available.");
        return Ok(());
    }

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let connected = Arc::new(tokio::sync::Notify::new());
    let connected_signal = Arc::clone(&connected);

    let accept_handle = tokio::spawn(async move {
        if let Ok((_stream, _addr)) = listener.accept().await {
            connected_signal.notify_one();
        }
    });

    let proxy_key = "HTTPS_PROXY";
    let prev = std::env::var(proxy_key).ok();
    std::env::set_var(proxy_key, format!("http://127.0.0.1:{port}"));

    // Build a client manually with proxy enabled.
    let env_val = std::env::var(CONNECTION_STRING_ENV_VAR)
        .unwrap_or_else(|_| EMULATOR_CONNECTION_STRING.to_string());
    let conn_str = if env_val == "emulator" {
        EMULATOR_CONNECTION_STRING.to_string()
    } else {
        env_val
    };
    let parsed: azure_data_cosmos_driver::models::ConnectionString = conn_str.parse()?;

    let pool_builder = azure_data_cosmos::options::ConnectionPoolOptions::builder()
        .with_proxy_allowed(true)
        .with_server_certificate_validation(
            azure_data_cosmos::options::ServerCertificateValidation::RequiredUnlessEmulator,
        );

    let runtime = azure_data_cosmos::CosmosRuntime::builder()
        .with_connection_pool(pool_builder.build()?)
        .build()
        .await?;

    let builder = azure_data_cosmos::CosmosClient::builder().with_runtime(runtime);

    let endpoint: azure_data_cosmos::AccountEndpoint = parsed.account_endpoint().parse()?;

    // Spawn the build + request so we can wait on the proxy signal instead.
    // The driver probes the endpoint during build(), which will go through the
    // proxy. The probe will fail (our fake proxy doesn't implement CONNECT),
    // but the connection attempt itself proves proxy routing works.
    let request_handle = tokio::spawn(async move {
        let client = builder
            .build(
                azure_data_cosmos::AccountReference::with_authentication_key(
                    endpoint,
                    parsed.account_key().clone(),
                ),
                azure_data_cosmos::RoutingStrategy::ProximityTo(
                    azure_data_cosmos::options::Region::EAST_US,
                ),
            )
            .await;
        // Ignore the result — the driver's init probe will fail through the fake proxy,
        // but we only care that the proxy was contacted.
        if let Ok(client) = client {
            let _ = client.database_client("nonexistent").read(None).await;
        }
    });

    // Wait for the proxy listener to accept a connection, with a timeout fallback.
    let proxy_hit = tokio::time::timeout(std::time::Duration::from_secs(5), connected.notified())
        .await
        .is_ok();

    match prev {
        Some(v) => std::env::set_var(proxy_key, v),
        None => std::env::remove_var(proxy_key),
    }

    request_handle.abort();
    accept_handle.abort();

    assert!(
        proxy_hit,
        "Proxy-enabled builder should route through proxy, but no connection was received"
    );

    Ok(())
}
