// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use azure_core::time::Duration;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::models::StartPositions;
use azure_messaging_eventhubs::{
    ConsumerClient, EventProcessor, InMemoryCheckpointStore, ProcessorStrategy, ProducerClient,
    Result, SendEventOptions, StartLocation, StartPosition,
};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

#[recorded::test(live)]
async fn start_processor(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let consumer_client = ConsumerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None),
            recording.credential().clone(),
        )
        .await?;

    let event_processor = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(Duration::seconds(5))
        .with_partition_expiration_duration(Duration::seconds(10))
        .with_prefetch(300)
        .build(consumer_client, Arc::new(InMemoryCheckpointStore::new()))
        .await?;

    {
        const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);

        info!("Started event processor");
        info!("Waiting for event processor to finish");
        info!("Timeout set to {:?}", TIMEOUT);

        tokio::select! {
            result = event_processor.run() => {
                info!("Event processor finished: {:?}", result);
                if let Err(e) = result {
                    info!("Event processor failed: {:?}", e);
                } else {
                    info!("Event processor finished successfully");
                }
            }
            _ = tokio::time::sleep(TIMEOUT) => {
                info!("Timeout reached.");
            }
        }
    }

    info!("Dereferencing the processor.");
    let processor = Arc::into_inner(event_processor);
    if let Some(processor) = processor {
        info!("Closing event processor");
        let result = processor.close().await;
        if let Err(e) = result {
            error!("Failed to close event processor: {:?}", e);
        } else {
            info!("Event processor closed successfully");
        }
    } else {
        info!("Event processor still running..");
    }

    Ok(())
}

async fn create_consumer_client(ctx: &TestContext) -> Result<ConsumerClient> {
    let recording = ctx.recording();

    let c = ConsumerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None),
            recording.credential().clone(),
        )
        .await?;
    Ok(c)
}

async fn create_producer_client(ctx: &TestContext) -> Result<ProducerClient> {
    let recording = ctx.recording();

    let p = ProducerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None).as_str(),
            recording.credential().clone(),
        )
        .await?;
    Ok(p)
}

async fn create_processor(
    consumer_client: ConsumerClient,
    update_interval: Duration,
    start_positions: Option<StartPositions>,
) -> Result<Arc<EventProcessor>> {
    let mut builder = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(update_interval)
        .with_partition_expiration_duration(Duration::seconds(120))
        .with_prefetch(300);
    if let Some(start_positions) = start_positions {
        builder = builder.with_start_positions(start_positions);
    }
    let p = builder
        .build(consumer_client, Arc::new(InMemoryCheckpointStore::new()))
        .await?;
    Ok(p)
}

async fn start_processor_running(
    event_processor: &Arc<EventProcessor>,
) -> JoinHandle<azure_messaging_eventhubs::Result<()>> {
    let event_processor = Arc::clone(event_processor);
    tokio::spawn(async move { event_processor.run().await })
}

#[recorded::test(live)]
async fn get_next_partition_client(ctx: TestContext) -> Result<()> {
    let consumer_client = create_consumer_client(&ctx).await?;
    let processor = create_processor(consumer_client, Duration::seconds(20), None).await?;

    let running_processor = start_processor_running(&processor).await;

    info!("Getting the first partition client.");
    let partition_client = processor
        .next_partition_client()
        .await
        .expect("Failed to get next partition client");
    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    running_processor.abort();
    info!("Processor task aborted");
    let _ = running_processor.await;
    info!("Processor task joined");

    Ok(())
}

#[recorded::test(live)]
async fn get_all_partition_clients(ctx: TestContext) -> Result<()> {
    use std::collections::HashSet;

    use azure_messaging_eventhubs::EventHubsError;

    let consumer_client = create_consumer_client(&ctx).await?;

    let eh_properties = consumer_client.get_eventhub_properties().await?;

    // The processor only adds one client as needed up to the max, so we block waiting
    // on all the clients to become available.
    let processor = create_processor(consumer_client, Duration::seconds(3), None).await?;

    let running_processor = start_processor_running(&processor).await;

    let mut found_clients = HashSet::new();
    let mut partition_clients = Vec::new();
    for partition in 0..eh_properties.partition_ids.len() {
        info!("Partition ID: {}", partition);

        let next_client = processor.next_partition_client().await?;
        if found_clients.contains(next_client.get_partition_id()) {
            panic!(
                "Duplicate partition client found: {}",
                next_client.get_partition_id()
            );
        }
        info!(
            "Received partition client for partition {}",
            next_client.get_partition_id()
        );
        found_clients.insert(next_client.get_partition_id().to_string());
        partition_clients.push(next_client);
    }

    info!("Received {} partition clients", partition_clients.len());

    for client in partition_clients.iter() {
        info!(
            "Received partition client for partition {}",
            client.get_partition_id()
        );
    }

    {
        info!("Retrieving one more processor client than possible.");
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) =>
                info!("Timeout reached - event processor has no more partitions."),

            _ = processor.next_partition_client() =>
                 panic!("Received next partition client, this should not happen."),
        }
    }
    // Now drop one of the partition clients.
    let partition_client = partition_clients.pop().unwrap();
    info!(
        "Dropping partition client for partition {}",
        partition_client.get_partition_id()
    );

    if let Some(partition_client) = Arc::into_inner(partition_client) {
        info!("All references to partition client dropped");
        partition_client.close().await?;
        info!("Partition client closed");
    } else {
        panic!("Partition client not dropped: Arc has multiple strong references (this should not happen).");
    }

    info!("Partition client dropped, getting another partition client.");

    // Wait for the processor to notice the partition client is dropped.
    let partition_client = tokio::select! {
        result = processor.next_partition_client() => {
            info!("Received next partition client");
            result?
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(15)) => {
            info!("Timeout reached - event processor has no more partitions.");
            return Err(EventHubsError::from(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "Timeout waiting for next partition client"
            )));
        }
    };

    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    running_processor.abort();
    info!("Processor task aborted");
    let _ = running_processor.await;
    info!("Processor task joined");

    Ok(())
}

#[recorded::test(live)]
async fn receive_events_from_processor(ctx: TestContext) -> Result<()> {
    let consumer_client = create_consumer_client(&ctx).await?;

    let eh_info = consumer_client.get_eventhub_properties().await?;

    // Determine the current start position for each partition, and configure
    // the processor to start from that position.
    //
    // This is useful for testing the processor with existing partitions.
    let mut start_positions = HashMap::new();
    for partition_id in eh_info.partition_ids.into_iter() {
        info!("Partition ID: {}", partition_id);
        let partition_info = consumer_client
            .get_partition_properties(&partition_id)
            .await?;

        info!(
            "[{partition_id}]: Last enqueued sequence number: {}",
            partition_info.last_enqueued_sequence_number
        );
        start_positions.insert(
            partition_id,
            StartPosition {
                location: StartLocation::SequenceNumber(
                    partition_info.last_enqueued_sequence_number,
                ),
                inclusive: false,
            },
        );
    }

    let processor = create_processor(
        consumer_client,
        Duration::seconds(20),
        Some(StartPositions {
            per_partition: start_positions,
            ..Default::default()
        }),
    )
    .await?;

    let running_processor = start_processor_running(&processor).await;

    info!("Getting the first partition client.");
    let partition_client = processor
        .next_partition_client()
        .await
        .expect("Failed to get next partition client");
    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    // Ensure there are some events in the partition to receive.
    {
        info!("Creating producer client");
        let producer_client = create_producer_client(&ctx).await?;

        info!(
            "Sending events to partition {}",
            partition_client.get_partition_id()
        );

        for i in 0..10 {
            let event_data = format!("Hello world {}", i);
            let send_event_options = SendEventOptions {
                partition_id: Some(partition_client.get_partition_id().to_string()),
            };
            producer_client
                .send_event(event_data, Some(send_event_options))
                .await
                .expect("Failed to send event data");
        }

        producer_client.close().await?;
        info!("Producer client closed");
    }

    // Receive events from the partition client.
    let event_stream = partition_client.stream_events();

    let messages = event_stream.take(10).collect::<Vec<_>>().await;

    info!("Received {} messages from the stream", messages.len());

    // Pull the first 10 messages from the stream. None of them should have failed.
    for message in messages {
        match message {
            Ok(event_data) => {
                info!("Received event: {:?}", event_data);
                // Process the received event data here
                partition_client
                    .update_checkpoint(&event_data)
                    .await
                    .expect("Failed to update checkpoint");
                info!("Checkpoint updated for event.");
                info!(
                    "Received event data body as text: {}",
                    String::from_utf8(
                        event_data
                            .event_data()
                            .body()
                            .unwrap_or_else(|| panic!("Event body is not present."))
                            .to_vec()
                    )
                    .unwrap_or_else(|_| panic!("Failed to convert event data to string"))
                );
            }
            Err(e) => {
                panic!("Error receiving event: {:?}", e);
            }
        }
    }

    if let Ok(partition_client) = Arc::try_unwrap(partition_client) {
        info!("All references to partition client dropped");
        partition_client.close().await?;
        info!("Partition client closed");
    } else {
        warn!("Partition client not dropped: Arc has multiple strong references (this should not happen).");
    }

    running_processor.abort();
    info!("Processor task aborted");
    let _ = running_processor.await;
    info!("Processor task joined");

    // Close the processor.
    info!("Closing processor");
    if let Ok(processor) = Arc::try_unwrap(processor) {
        processor.close().await?;
        info!("Processor closed");
    } else {
        info!("Processor still has references, not closing.");
    }

    Ok(())
}
