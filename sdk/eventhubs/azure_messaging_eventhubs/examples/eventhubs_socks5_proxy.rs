// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

//! This sample demonstrates how to connect to an Event Hub through a SOCKS5 proxy using the `ProducerClient`.
//!
//! # SOCKS5 Proxy Setup
//!
//! To run this example, you need:
//!
//! 1. **A running SOCKS5 proxy server**:
//!    ```bash
//!    # Example using SSH tunnel
//!    ssh -D 8080 user@proxy-server.example.com
//!
//!    # Or using a dedicated SOCKS5 proxy like Dante
//!    sockd -D -p 8080
//!    ```
//!
//! 2. **Environment variables**:
//!    ```bash
//!    export EVENTHUBS_HOST="your-eventhub.servicebus.windows.net"
//!    export EVENTHUB_NAME="your-eventhub-name"
//!    export SOCKS5_PROXY_URL="socks5h://my-proxy-domain:12345"  # Optional, defaults to socks5h://my-proxy-domain:12345
//!    export SOCKS5_PROXY_URL="socks5://user:pass@my-proxy-domain:12345"  # With authentication
//!    ```
//!
//! # Protocol Support
//!
//! - **socks5://** - Standard SOCKS5 with local DNS resolution
//! - **socks5h://** - SOCKS5 with proxy-side DNS resolution (recommended for corporate environments)
//!
//! # Usage
//!
//! ```bash
//! cargo run --example eventhubs_socks5_proxy
//! ```

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Get EventHub connection details
    let eventhub_namespace =
        env::var("EVENTHUBS_HOST").expect("EVENTHUBS_HOST environment variable must be set");
    let eventhub_name =
        env::var("EVENTHUB_NAME").expect("EVENTHUB_NAME environment variable must be set");

    // Get SOCKS5 proxy URL (with fallback default)
    let proxy_url = env::var("SOCKS5_PROXY_URL").unwrap_or_else(|_| {
        warn!("SOCKS5_PROXY_URL not set, using default: socks5h://my-proxy-domain:12345");
        "socks5h://my-proxy-domain:12345".to_string()
    });

    info!(
        eventhub_host = %eventhub_namespace,
        eventhub_name = %eventhub_name,
        proxy_url = %mask_proxy_credentials(&proxy_url),
        "Connecting to EventHub through SOCKS5 proxy"
    );

    // Create credential
    let credential = DeveloperToolsCredential::new(None)?;

    // Create producer client with SOCKS5 proxy
    let client = ProducerClient::builder()
        .with_application_id("socks5_proxy_example".to_string())
        .with_custom_endpoint(proxy_url)
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential,
        )
        .await?;

    info!("Successfully created producer client through SOCKS5 proxy");

    // Send a test message through the proxy
    let test_message = format!(
        "Hello from SOCKS5 proxy at {:?}",
        std::time::SystemTime::now()
    );
    client.send_event(test_message.as_str(), None).await?;

    info!(
        message = %test_message,
        "Successfully sent message through SOCKS5 proxy"
    );

    // Send a more complex event with properties
    use azure_messaging_eventhubs::models::EventData;
    let event = EventData::builder()
        .with_content_type("application/json".to_string())
        .with_body(r#"{"event_type": "socks5_test", "timestamp": "2024-01-01T00:00:00Z"}"#)
        .add_property("proxy_type".to_string(), "socks5")
        .add_property(
            "test_run".to_string(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
        .build();

    client.send_event(event, None).await?;

    info!("Successfully sent structured event through SOCKS5 proxy");

    // Clean up
    client.close().await?;
    info!("Closed producer client connection");

    Ok(())
}

/// Mask credentials in proxy URL for secure logging
///
/// This function replaces username and password in the proxy URL with asterisks
/// to prevent credential exposure in log files.
fn mask_proxy_credentials(url: &str) -> String {
    use azure_core::http::Url;

    if let Ok(parsed_url) = Url::parse(url) {
        let mut masked = parsed_url.clone();
        if !parsed_url.username().is_empty() {
            let _ = masked.set_username("***");
        }
        if parsed_url.password().is_some() {
            let _ = masked.set_password(Some("***"));
        }
        masked.to_string()
    } else {
        // If URL parsing fails, return a safe placeholder
        "invalid_proxy_url".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_proxy_credentials() {
        // Test masking credentials
        let url_with_auth = "socks5://user:password@proxy.example.com:1080";
        let masked = mask_proxy_credentials(url_with_auth);
        assert_eq!(masked, "socks5://***:***@proxy.example.com:1080");

        // Test URL without credentials
        let url_no_auth = "socks5h://proxy.example.com:1080";
        let unmasked = mask_proxy_credentials(url_no_auth);
        assert_eq!(unmasked, "socks5h://proxy.example.com:1080");

        // Test invalid URL
        let invalid_url = "not-a-url";
        let result = mask_proxy_credentials(invalid_url);
        assert_eq!(result, "invalid_proxy_url");
    }
}
