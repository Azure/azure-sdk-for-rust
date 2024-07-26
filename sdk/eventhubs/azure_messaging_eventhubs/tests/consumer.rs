// Copyright (c) Microsoft Corp. All Rights Reserved.

//#![cfg(all(test, feature = "test_e2e"))] // to run this, do: `cargo test --features test_e2e`
//cspell: words eventhubs eventhub eventdata

use async_std::future::timeout;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_messaging_eventhubs::{
    consumer::{ConsumerClient, ConsumerClientOptions, ReceiveOptions},
    models::StartPosition,
};
use futures::{pin_mut, StreamExt};
use std::{env, time::Duration};
use tracing::{info, trace};

mod common;

#[tokio::test]
async fn test_new() {
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();
    let _client = ConsumerClient::new(
        host,
        eventhub,
        None,
        DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_new")
                .build(),
        ),
    );
}

#[tokio::test]
async fn test_new_with_error() {
    common::setup();
    trace!("test_new_with_error");
    let eventhub = env::var("EVENTHUB_NAME").unwrap();
    let consumer = ConsumerClient::new(
        "invalid_host",
        eventhub,
        None,
        DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_new")
                .build(),
        ),
    );
    let result = consumer.open().await;
    assert!(result.is_err());
    info!("Error: {:?}", result);
}

#[tokio::test]
async fn test_open() {
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();
    let client = ConsumerClient::new(
        host,
        eventhub,
        None,
        azure_identity::DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_open")
                .build(),
        ),
    );
    client.open().await.unwrap();
}
#[tokio::test]
async fn test_close() {
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();
    let client = ConsumerClient::new(
        host,
        eventhub,
        None,
        azure_identity::DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap(),
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_close")
                .build(),
        ),
    );
    client.open().await.unwrap();
    client.close().await.unwrap();
}

#[tokio::test]
async fn test_get_properties() {
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap();

    let client = ConsumerClient::new(
        host,
        eventhub.clone(),
        None,
        credential,
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_get_properties")
                .build(),
        ),
    );
    client.open().await.unwrap();
    let properties = client.get_eventhub_properties().await.unwrap();
    info!("Properties: {:?}", properties);
    assert_eq!(properties.name, eventhub);
}

#[tokio::test]
async fn test_get_partition_properties() {
    common::setup();
    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap();

    let client = ConsumerClient::new(
        host,
        eventhub,
        None,
        credential,
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("test_get_properties")
                .build(),
        ),
    );
    client.open().await.unwrap();
    let properties = client.get_eventhub_properties().await.unwrap();

    for partition_id in properties.partition_ids {
        let partition_properties = client
            .get_partition_properties(partition_id.clone())
            .await
            .unwrap();
        info!("Partition properties: {:?}", partition_properties);
        assert_eq!(partition_properties.id, partition_id);
    }
}

#[tokio::test]
async fn receive_lots_of_events() {
    common::setup();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::create(TokenCredentialOptions::default()).unwrap();

    let client = ConsumerClient::new(
        host,
        eventhub,
        None,
        credential,
        Some(
            ConsumerClientOptions::builder()
                .with_application_id("receive_lots_of_events")
                .build(),
        ),
    );
    client.open().await.unwrap();

    let event_stream = client
        .receive_events_on_partition(
            "0",
            Some(
                ReceiveOptions::builder()
                    .with_start_position(StartPosition::builder().with_earliest_location().build())
                    .build(),
            ),
        )
        .await;

    pin_mut!(event_stream); // Needed for iteration.

    let mut count = 0;
    // Read events from the stream for 10 seconds.
    let result = timeout(Duration::from_secs(10), async {
        while let Some(event) = event_stream.next().await {
            match event {
                Ok(_event) => {
                    //                    info!("Received the following message:: {:?}", event);
                    count += 1;
                }
                Err(err) => {
                    info!("Error while receiving message: {:?}", err);
                }
            }
        }
    })
    .await;

    assert!(result.is_err());
    info!("Received {count} messages.");

    // let futures = vec![f(), f(), f(), f(), f(), f(), f(), f(), f(), f()];

    // join_all(futures).await;
}
