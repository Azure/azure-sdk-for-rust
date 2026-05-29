// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

use azure_core::time::Duration;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::models::StartPositions;
use azure_messaging_eventhubs::{
    error::ErrorKind, ConsumerClient, EventProcessor, InMemoryCheckpointStore, ProcessorStrategy,
    ProducerClient, Result, SendEventOptions, StartLocation, StartPosition,
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

fn processor_builder(
    strategy: ProcessorStrategy,
    update_interval: Duration,
    expiration: Duration,
) -> azure_messaging_eventhubs::builders::EventProcessorBuilder {
    EventProcessor::builder()
        .with_load_balancing_strategy(strategy)
        .with_update_interval(update_interval)
        .with_partition_expiration_duration(expiration)
        .with_prefetch(300)
}

async fn create_processor(
    consumer_client: ConsumerClient,
    update_interval: Duration,
    start_positions: Option<StartPositions>,
) -> Result<Arc<EventProcessor>> {
    let mut builder = processor_builder(
        ProcessorStrategy::Balanced,
        update_interval,
        Duration::seconds(120),
    );
    if let Some(start_positions) = start_positions {
        builder = builder.with_start_positions(start_positions);
    }
    let p = builder
        .build(consumer_client, Arc::new(InMemoryCheckpointStore::new()))
        .await?;
    Ok(p)
}

/// Drains `next_partition_client()` until it has been idle for `idle_timeout`.
/// Used when a test needs every partition the processor will claim, not just
/// the first.
async fn drain_partition_clients(
    processor: &EventProcessor,
    idle_timeout: std::time::Duration,
) -> Vec<Arc<azure_messaging_eventhubs::processor::PartitionClient>> {
    let mut clients = Vec::new();
    loop {
        match tokio::time::timeout(idle_timeout, processor.next_partition_client()).await {
            Ok(Ok(client)) => {
                info!("Claimed partition {}", client.get_partition_id());
                clients.push(client);
            }
            Ok(Err(e)) => {
                warn!("next_partition_client returned error during drain: {:?}", e);
                break;
            }
            Err(_) => break,
        }
    }
    clients
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

/// When a second `EventProcessor` instance starts against the same Event Hub
/// and consumer group, the broker disconnects at least one of the first
/// instance's partition receivers because both attach with
/// `owner_level = Some(0)`. The displaced instance must observe this as
/// `ErrorKind::ConsumerDisconnected` on `stream_events()` rather than
/// silently re-attaching.
///
/// This is the end-to-end guard against the auto-recovery regression: the
/// receive-path retry decider in `should_retry_receive_error` excludes
/// `AmqpErrorCondition::LinkStolen` and `event_receiver::translate_receive_error`
/// translates it to the typed variant. If either is broken, this test
/// fails.
///
/// The test watches every partition client A claims (not just the first),
/// because B will only steal a fair share of partitions and that share is
/// not deterministic.
#[recorded::test(live)]
async fn second_processor_displaces_first_with_consumer_disconnected(
    ctx: TestContext,
) -> Result<()> {
    // Use a short update interval so load balancing converges quickly; the
    // validation rule on the builder requires expiration > interval.
    const UPDATE_INTERVAL: Duration = Duration::seconds(5);
    const EXPIRATION: Duration = Duration::seconds(30);
    // Give the broker + load balancer up to this long to displace at least
    // one of the first processor's receivers and propagate the AMQP detach.
    const STEAL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(90);
    // How long to collect A's initial claims before starting B. With a 5s
    // interval, two cycles is enough for A to grab whatever it is going to
    // grab under a Greedy strategy.
    const COLLECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);

    let consumer_a = create_consumer_client(&ctx).await?;
    let processor_a = processor_builder(ProcessorStrategy::Greedy, UPDATE_INTERVAL, EXPIRATION)
        .build(consumer_a, Arc::new(InMemoryCheckpointStore::new()))
        .await?;
    let running_a = start_processor_running(&processor_a).await;

    info!("Collecting all partition clients claimed by processor A.");
    let partition_clients_a = drain_partition_clients(&processor_a, COLLECT_TIMEOUT).await;
    assert!(
        !partition_clients_a.is_empty(),
        "processor A did not claim any partitions within the collect window"
    );
    info!(
        "Processor A holds {} partition clients before B starts.",
        partition_clients_a.len()
    );

    // Start the second processor against the same hub + consumer group;
    // it will open epoch-0 receivers and the broker will disconnect at
    // least one of A's receivers within the load-balancing window.
    let consumer_b = create_consumer_client(&ctx).await?;
    let processor_b = processor_builder(ProcessorStrategy::Greedy, UPDATE_INTERVAL, EXPIRATION)
        .build(consumer_b, Arc::new(InMemoryCheckpointStore::new()))
        .await?;
    let running_b = start_processor_running(&processor_b).await;
    info!("Second processor started; watching every partition A holds for a steal.");

    // Merge every partition's stream into one and watch for the first
    // error. `select_all` does not require `Send`, which matters because
    // `PartitionClient::stream_events` returns a non-`Send` boxed stream.
    use futures::stream::select_all;
    let tagged_streams = partition_clients_a.iter().map(|client| {
        let partition_id = client.get_partition_id().to_string();
        client
            .stream_events()
            .map(move |result| (partition_id.clone(), result))
            .boxed_local()
    });
    let mut merged = select_all(tagged_streams);

    let race = async {
        loop {
            match merged.next().await {
                Some((_partition_id, Ok(_event))) => continue,
                Some((partition_id, Err(err))) => return (partition_id, Some(err)),
                // All streams ended without an error: the bug we guard against.
                None => return (String::new(), None),
            }
        }
    };
    let observed = tokio::time::timeout(STEAL_TIMEOUT, race).await;

    // Stop both before asserting so a panic does not leak background tasks.
    running_a.abort();
    running_b.abort();
    let _ = running_a.await;
    let _ = running_b.await;

    let (partition_id, maybe_err) = observed.unwrap_or_else(|_| {
        panic!(
            "no partition client streamed an error within {:?}; expected ConsumerDisconnected on at least one of {} partitions",
            STEAL_TIMEOUT,
            partition_clients_a.len(),
        )
    });
    let err = maybe_err.unwrap_or_else(|| {
        panic!(
            "partition {} stream ended without an error; expected ConsumerDisconnected",
            partition_id
        )
    });
    assert!(
        matches!(err.kind, ErrorKind::ConsumerDisconnected(_)),
        "expected ConsumerDisconnected on displaced partition {}, got {:?}",
        partition_id,
        err.kind,
    );

    Ok(())
}
