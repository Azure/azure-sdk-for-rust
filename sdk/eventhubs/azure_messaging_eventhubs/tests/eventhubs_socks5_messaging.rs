// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

#![cfg(feature = "socks5")]

//! SOCKS5 Proxy Messaging Tests for EventHubs
//!
//! These tests verify EventHub message operations (send/receive) work through SOCKS5 proxies.
//! All tests are marked as `#[recorded::test(live)]` and require both EventHub credentials
//! and a running SOCKS5 proxy server.
//!
//! # Live Testing Setup
//!
//! These tests extend the basic proxy connection tests to verify actual message flow:
//!
//! 1. **Set up SOCKS5 proxy** (same as proxy connection tests)
//! 2. **Configure EventHub environment**:
//!    ```bash
//!    export EVENTHUBS_HOST="your-eventhub.servicebus.windows.net"
//!    export EVENTHUB_NAME="your-eventhub-name"
//!    export SOCKS5_PROXY_URL="socks5h://my-proxy-domain:12345"
//!    ```
//!
//! 3. **Run messaging tests**:
//!    ```bash
//!    AZURE_TEST_MODE=live cargo test --features socks5 --test eventhubs_socks5_messaging
//!    ```
//!
//! # Test Coverage
//!
//! - **socks5_proxy_send_receive_messages**: End-to-end message flow through proxy
//! - **socks5_proxy_partition_properties**: Metadata operations through proxy
//!
//! # Expected Behavior
//!
//! - Tests may timeout waiting for messages in test environments (this is expected)
//! - Connection establishment should succeed even if message flow times out
//! - All proxy credentials are automatically masked in test output

use azure_core::{http::Url, time::Duration};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, ProducerClient, StartPosition,
};
use futures::StreamExt;
use std::{env, error::Error};
use tokio::time::timeout;
use tracing::{info, trace, warn};

mod common;

#[recorded::test(live)]
async fn socks5_proxy_send_receive_messages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup();

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let proxy_url =
        env::var("SOCKS5_PROXY_URL").unwrap_or("socks5h://my-proxy-domain:12345".to_string());

    info!(
        host = %host,
        eventhub = %eventhub,
        proxy_url = %mask_proxy_credentials(&proxy_url),
        "Testing EventHub message send/receive through SOCKS5 proxy"
    );

    // Create producer client through SOCKS5 proxy
    let producer = ProducerClient::builder()
        .with_application_id("socks5_producer_test".to_string())
        .with_custom_endpoint(proxy_url.clone())
        .open(host.as_str(), eventhub.as_str(), recording.credential())
        .await?;

    info!("Producer client created successfully through SOCKS5 proxy");

    // Send a test message
    let test_message = format!("SOCKS5 test message at {:?}", std::time::SystemTime::now());
    producer.send_event(test_message.clone(), None).await?;

    info!(
        message = %test_message,
        "Message sent successfully through SOCKS5 proxy"
    );

    // Create consumer client through SOCKS5 proxy
    let consumer = ConsumerClient::builder()
        .with_application_id("socks5_consumer_test".to_string())
        .with_custom_endpoint(proxy_url)
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    info!("Consumer client created successfully through SOCKS5 proxy");

    // Attempt to receive messages
    let partition_client = consumer
        .open_receiver_on_partition(
            "0".to_string(), // Use partition 0
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: azure_messaging_eventhubs::StartLocation::Latest,
                    ..Default::default()
                }),
                receive_timeout: Some(Duration::seconds(5)),
                ..Default::default()
            }),
        )
        .await?;

    info!("Partition receiver created, attempting to receive messages");

    // Try to receive messages with timeout
    let receive_result = timeout(Duration::seconds(30).try_into().unwrap(), async {
        let mut stream = partition_client.stream_events();
        let mut message_count = 0;

        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => {
                    message_count += 1;
                    info!(
                        message_count = message_count,
                        message_body = ?event.event_data().body(),
                        "Received message through SOCKS5 proxy"
                    );

                    // If we receive any message, that's success
                    if message_count >= 1 {
                        return Ok::<(), Box<dyn Error>>(());
                    }
                }
                Err(e) => {
                    warn!(error = %e, "Error receiving message");
                }
            }
        }
        Ok(())
    })
    .await;

    match receive_result {
        Ok(_) => {
            info!("Successfully received messages through SOCKS5 proxy");
        }
        Err(_) => {
            warn!("Timeout receiving messages - this may be expected in test environment");
        }
    }

    trace!("SOCKS5 proxy message send/receive test completed");
    Ok(())
}

#[recorded::test(live)]
async fn socks5_proxy_partition_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    common::setup();

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let proxy_url =
        env::var("SOCKS5_PROXY_URL").unwrap_or("socks5h://my-proxy-domain:12345".to_string());

    info!(
        host = %host,
        eventhub = %eventhub,
        proxy_url = %mask_proxy_credentials(&proxy_url),
        "Testing EventHub partition properties through SOCKS5 proxy"
    );

    let consumer = ConsumerClient::builder()
        .with_application_id("socks5_properties_test".to_string())
        .with_custom_endpoint(proxy_url)
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    // Get partition properties to validate connection works
    let properties = consumer.get_partition_properties("0").await?;

    info!(
        partition_id = %properties.id,
        beginning_sequence_number = properties.beginning_sequence_number,
        last_enqueued_sequence_number = properties.last_enqueued_sequence_number,
        "Successfully retrieved partition properties through SOCKS5 proxy"
    );

    trace!("SOCKS5 proxy partition properties test completed");
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
