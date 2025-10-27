// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell::ignore Uncategorized

use azure_core::time::Duration;
use azure_core_amqp::AmqpErrorKind;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind, ConsumerClient, OpenReceiverOptions, StartPosition,
};
use futures::StreamExt;
use std::{env, error::Error};
use tokio::time::timeout;
use tracing::{info, trace};

#[recorded::test(live)]
async fn consumer_new(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let _client = ConsumerClient::builder()
        .with_application_id("test_new".to_string())
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    Ok(())
}

#[recorded::test(live)]
async fn consumer_new_with_error(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    trace!("test_new_with_error");
    let eventhub = env::var("EVENTHUB_NAME")?;
    let result = ConsumerClient::builder()
        .with_application_id("test_new".to_string())
        .open("invalid_host", eventhub, recording.credential())
        .await;
    assert!(result.is_err());
    match result {
        Err(err) => {
            info!("EH Error: {:?}", err);
            assert!(matches!(err.kind, ErrorKind::AmqpError(_)));
            let ErrorKind::AmqpError(err) = err.kind else {
                panic!("Error is not an AMQP  error.");
            };
            info!("AMQP Error: {:?}", err);

            assert!(matches!(*err.kind(), AmqpErrorKind::AzureCore(_)));

            let AmqpErrorKind::AzureCore(ref err) = *err.kind() else {
                panic!("Error is not an azure core error.");
            };
            info!("Azure Error: {:?}", err);
            if matches!(err.kind(), azure_core::error::ErrorKind::Io) {
                // Dig into the I/O error to determine the type.
                let e = err
                    .source()
                    .unwrap()
                    .downcast_ref::<std::io::Error>()
                    .unwrap();
                info!("IO Error: {}", e);
                info!("IO Error Kind: {:?}", e.kind());
                let e = e.source();
                info!("IO Error Source: {:?}", e);
            } else {
                panic!("Expected IO Error");
            }
        }
        _ => panic!("Expected Error (cannot hit due to previous assert)"),
    }

    Ok(())
}

#[recorded::test(live)]
async fn consumer_open(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let _client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, recording.credential())
        .await?;

    Ok(())
}
#[recorded::test(live)]
async fn consumer_close(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, recording.credential())
        .await?;
    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn consumer_get_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;
    info!("Properties: {:?}", properties);
    assert_eq!(properties.name, eventhub);

    Ok(())
}

#[recorded::test(live)]
async fn consumer_get_partition_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;

    for partition_id in properties.partition_ids {
        let partition_properties = client.get_partition_properties(&partition_id).await?;
        info!("Partition properties: {:?}", partition_properties);
        assert_eq!(partition_properties.id, partition_id);
    }

    Ok(())
}

#[recorded::test(live)]
async fn receive_events_on_all_partitions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    info!("Establishing credentials.");

    let recording = ctx.recording();
    let credential = recording.credential();

    info!("Creating client.");
    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, credential.clone())
        .await?;

    let eh_properties = client.get_eventhub_properties().await?;
    info!("EventHub properties: {:?}", eh_properties);

    let mut receivers = Vec::new();
    for partition_id in eh_properties.partition_ids {
        info!("Creating receiver for partition: {partition_id}");
        let receiver = client
            .open_receiver_on_partition(
                partition_id,
                Some(OpenReceiverOptions {
                    start_position: Some(StartPosition {
                        location: azure_messaging_eventhubs::StartLocation::Earliest,
                        ..Default::default()
                    }),
                    // Timeout for individual receive operations.
                    receive_timeout: Some(Duration::seconds(5)),
                    ..Default::default()
                }),
            )
            .await?;

        receivers.push(receiver);
    }
    info!("Created {} receivers.", receivers.len());

    for receiver in receivers {
        info!(
            "Creating event receive stream on receiver for: {:?}",
            receiver.partition_id()
        );
        {
            let mut event_stream = receiver.stream_events();

            let mut count = 0;

            const TEST_DURATION: Duration = Duration::seconds(10);
            info!("Receiving events for {:?}.", TEST_DURATION);

            // Read events from the stream for a bit of time.

            let result = timeout(TEST_DURATION.try_into().unwrap(), async {
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

            info!("Received {count} messages in {TEST_DURATION:?}. Timeout: {result:?}");
        }
        receiver.close().await?;
    }

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn multiple_receivers_on_one_partition(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    info!("Establishing credentials.");

    let recording = ctx.recording();
    let credential = recording.credential();

    info!("Creating client.");
    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, credential.clone())
        .await?;

    let eh_properties = client.get_eventhub_properties().await?;
    info!("EventHub properties: {:?}", eh_properties);

    let mut receivers = Vec::new();

    info!(
        "Creating receiver for partition {}",
        eh_properties.partition_ids[0]
    );

    {
        let receiver = client
            .open_receiver_on_partition(
                eh_properties.partition_ids[0].clone(),
                Some(OpenReceiverOptions {
                    start_position: Some(StartPosition {
                        location: azure_messaging_eventhubs::StartLocation::Earliest,
                        ..Default::default()
                    }),
                    // Timeout for individual receive operations.
                    receive_timeout: Some(Duration::seconds(5)),
                    ..Default::default()
                }),
            )
            .await?;

        receivers.push(receiver);
    }

    info!(
        "Creating receiver 2 for partition {}",
        eh_properties.partition_ids[0]
    );
    {
        let receiver = client
            .open_receiver_on_partition(
                eh_properties.partition_ids[0].clone(),
                Some(OpenReceiverOptions {
                    start_position: Some(StartPosition {
                        location: azure_messaging_eventhubs::StartLocation::Earliest,
                        ..Default::default()
                    }),
                    // Timeout for individual receive operations.
                    receive_timeout: Some(Duration::seconds(5)),
                    ..Default::default()
                }),
            )
            .await?;

        receivers.push(receiver);
    }
    info!("Created {} receivers.", receivers.len());

    for receiver in receivers.iter() {
        info!(
            "Creating event receive stream on receiver for: {:?}",
            receiver.partition_id()
        );
        let mut event_stream = receiver.stream_events();

        let mut count = 0;

        const TEST_DURATION: Duration = Duration::seconds(10);
        info!("Receiving events for {:?}.", TEST_DURATION);

        // Read events from the stream for a bit of time.

        let result = timeout(TEST_DURATION.try_into().unwrap(), async {
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

        info!("Received {count} messages in {TEST_DURATION:?}. Timeout: {result:?}");
    }

    Ok(())
}

#[recorded::test(live)]
async fn receive_lots_of_events(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    info!("Establishing credentials.");

    let credential = recording.credential();

    info!("Creating client.");
    let client = ConsumerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub, credential.clone())
        .await?;

    info!("Client open, create receiver.");
    let receiver = client
        .open_receiver_on_partition(
            "0".to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: azure_messaging_eventhubs::StartLocation::Earliest,
                    ..Default::default()
                }),
                // Timeout for individual receive operations.
                receive_timeout: Some(Duration::seconds(5)),
                ..Default::default()
            }),
        )
        .await?;

    info!("Creating event receive stream.");
    let mut event_stream = receiver.stream_events();

    let mut count = 0;

    const TEST_DURATION: Duration = Duration::seconds(10);
    info!("Receiving events for {:?}.", TEST_DURATION);

    // Read events from the stream for a bit of time.
    let result = timeout(TEST_DURATION.try_into().unwrap(), async {
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

    info!("Received {count} messages in {TEST_DURATION:?}. Timeout: {result:?}");

    Ok(())
}
