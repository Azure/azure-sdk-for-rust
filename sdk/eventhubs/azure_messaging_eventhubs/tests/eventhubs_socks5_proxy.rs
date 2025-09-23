// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! SOCKS5 Proxy Integration Tests for EventHubs
//!
//! These tests verify EventHub connections work through SOCKS5 proxies.
//! All tests are marked as `#[recorded::test(live)]` and require a running
//! SOCKS5 proxy server for live testing.
//!
//! # Live Testing Setup
//!
//! To run these tests against a live SOCKS5 proxy:
//!
//! 1. **Set up a SOCKS5 proxy server** (e.g., using SSH tunnel or dedicated proxy)
//!    ```bash
//!    # Example SSH SOCKS5 tunnel
//!    ssh -D 12345 user@proxy-server.example.com
//!    ```
//!
//! 2. **Configure environment variables**:
//!    ```bash
//!    export EVENTHUBS_HOST="your-eventhub.servicebus.windows.net"
//!    export EVENTHUB_NAME="your-eventhub-name"
//!    export SOCKS5_PROXY_URL="socks5h://my-proxy-domain:12345"
//!    export SOCKS5H_PROXY_URL="socks5h://my-proxy-domain:12345"
//!    export SOCKS5_PROXY_URL_WITH_AUTH="socks5://user:pass@my-proxy-domain:12345"
//!    ```
//!
//! 3. **Run live tests**:
//!    ```bash
//!    AZURE_TEST_MODE=live cargo test --test eventhubs_socks5_proxy
//!    ```
//!
//! # Default Configuration
//!
//! If environment variables are not set, tests will use default values:
//! - SOCKS5_PROXY_URL: `socks5h://my-proxy-domain:12345`
//! - SOCKS5H_PROXY_URL: `socks5h://my-proxy-domain:12345`
//! - SOCKS5_PROXY_URL_WITH_AUTH: `socks5://testuser:testpass@my-proxy-domain:12345`
//!
//! # Test Coverage
//!
//! - **socks5_proxy_connection**: Basic SOCKS5 connection test
//! - **socks5h_proxy_connection**: SOCKS5h (proxy-side DNS) connection test
//! - **socks5_proxy_with_auth**: Authenticated SOCKS5 connection test
//! - **direct_connection_still_works**: Verify backward compatibility

use azure_core::{http::Url};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::ConsumerClient;
use std::{env, error::Error};
use tracing::{info, trace};

mod common;

#[recorded::test(live)]
async fn socks5_proxy_connection(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup(); // Initialize logging per best practices

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let proxy_url = env::var("SOCKS5_PROXY_URL").unwrap_or("socks5h://my-proxy-domain:12345".to_string());

    info!(
        host = %host,
        eventhub = %eventhub,
        proxy_url = %proxy_url,
        "Testing EventHub connection through SOCKS5 proxy"
    );

    let _client = ConsumerClient::builder()
        .with_application_id("socks5_test".to_string())
        .with_custom_endpoint(proxy_url)
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    trace!("SOCKS5 connection test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn socks5h_proxy_connection(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup();

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let proxy_url = env::var("SOCKS5H_PROXY_URL").unwrap_or("socks5h://my-proxy-domain:12345".to_string());

    info!(
        host = %host,
        eventhub = %eventhub,
        proxy_url = %proxy_url,
        dns_resolution = "proxy-side",
        "Testing EventHub connection through SOCKS5h proxy with proxy-side DNS"
    );

    let _client = ConsumerClient::builder()
        .with_application_id("socks5h_test".to_string())
        .with_custom_endpoint(proxy_url)
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    trace!("SOCKS5h connection test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn socks5_proxy_with_auth(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup();

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    // Test with authentication if credentials are provided
    let proxy_url = env::var("SOCKS5_PROXY_URL_WITH_AUTH")
        .unwrap_or("socks5://testuser:testpass@my-proxy-domain:12345".to_string());

    info!(
        host = %host,
        eventhub = %eventhub,
        proxy_url = %mask_proxy_credentials(&proxy_url),
        "Testing EventHub connection through SOCKS5 proxy with authentication"
    );

    let _client = ConsumerClient::builder()
        .with_application_id("socks5_auth_test".to_string())
        .with_custom_endpoint(proxy_url)
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    trace!("SOCKS5 authenticated connection test completed successfully");
    Ok(())
}

#[recorded::test(live)]
async fn direct_connection_still_works(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup();

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    info!(
        host = %host,
        eventhub = %eventhub,
        connection_type = "direct",
        "Testing that direct EventHub connections still work after SOCKS5 implementation"
    );

    let _client = ConsumerClient::builder()
        .with_application_id("direct_test".to_string())
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    trace!("Direct connection test completed successfully");
    Ok(())
}

/// Mask credentials in proxy URL for logging
fn mask_proxy_credentials(url: &str) -> String {
    if let Ok(parsed_url) = Url::parse(url) {
        let mut masked = parsed_url.clone();
        if masked.username() != "" {
            let _ = masked.set_username("***");
        }
        if masked.password().is_some() {
            let _ = masked.set_password(Some("***"));
        }
        masked.to_string()
    } else {
        "invalid_url".to_string()
    }
}