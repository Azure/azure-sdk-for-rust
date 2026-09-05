// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

//! Live tests for the event processor on the blob checkpoint store.
//!
//! This is a separate test binary, so a file mutex cannot reach the tests in
//! `eventhubs_processor.rs`. Isolation comes from a separate consumer group
//! instead.

use azure_core::http::{StatusCode, Url};
use azure_core::time::Duration;
use azure_core_test::{recorded, Recording, TestContext};
use azure_messaging_eventhubs::error::ErrorKind;
use azure_messaging_eventhubs::models::{
    AmqpSimpleValue, Checkpoint, EventData, ReceivedEventData, StartPositions,
};
use azure_messaging_eventhubs::{
    CheckpointStore, ConsumerClient, EventProcessor, ProcessorStrategy, ProducerClient, Result,
    RetryOptions, SendEventOptions, StartLocation, StartPosition,
};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use futures::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::info;

/// The test resources bicep declares this consumer group and emits no output
/// for it, and `Recording::var` panics on an unset key, so the name is fixed
/// here.
const BLOB_CONSUMER_GROUP: &str = "defaultGroup";

const BLOB_SEED_EVENT_COUNT: i32 = 5;
const UPDATE_INTERVAL_FAST: Duration = Duration::seconds(5);
const PARTITION_EXPIRATION: Duration = Duration::seconds(30);
const FIRST_CLIENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);
const EVENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const SHUTDOWN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const BUILD_ERROR_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const RUN_ERROR_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);
const MARKER_PROPERTY: &str = "test_marker";
const INDEX_PROPERTY: &str = "test_index";

fn create_container_client(
    recording: &Recording,
    container_name: &str,
) -> azure_core::Result<BlobContainerClient> {
    let mut options = BlobContainerClientOptions::default();
    recording.instrument(&mut options.client_options);
    let endpoint = recording.var("AZURE_STORAGE_BLOB_ENDPOINT", None);
    let mut container_url = Url::parse(&endpoint)?;
    container_url
        .path_segments_mut()
        .expect("the blob endpoint must be a valid base URL")
        .push(container_name);
    BlobContainerClient::new(container_url, Some(recording.credential()), Some(options))
}

async fn create_producer_client(ctx: &TestContext) -> Result<ProducerClient> {
    let recording = ctx.recording();
    ProducerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None).as_str(),
            recording.credential().clone(),
        )
        .await
}

/// Reads the per-run marker off a received event.
fn marker_of(event: &ReceivedEventData) -> Option<String> {
    match event.event_data().properties()?.get(MARKER_PROPERTY)? {
        AmqpSimpleValue::String(value) => Some(value.clone()),
        _ => None,
    }
}

/// Reads the send order index off a received event.
fn index_of(event: &ReceivedEventData) -> Option<i32> {
    match event.event_data().properties()?.get(INDEX_PROPERTY)? {
        AmqpSimpleValue::Int(value) => Some(*value),
        _ => None,
    }
}

/// Sends `count` events to one partition. Each event carries the per-run
/// marker and its send order index, so a test can tell its own events from
/// the events of another run on the same shared hub.
async fn send_tagged_events(
    ctx: &TestContext,
    partition_id: &str,
    marker: &str,
    count: i32,
) -> Result<()> {
    let producer_client = create_producer_client(ctx).await?;
    for i in 0..count {
        let event = EventData::builder()
            .with_body(format!("{marker}-{i}"))
            .add_property(MARKER_PROPERTY.to_string(), marker)
            .add_property(INDEX_PROPERTY.to_string(), i)
            .build();
        producer_client
            .send_event(
                event,
                Some(SendEventOptions {
                    partition_id: Some(partition_id.to_string()),
                }),
            )
            .await?;
    }
    producer_client.close().await?;
    info!("Sent {count} tagged events to partition {partition_id}");
    Ok(())
}

/// Reads the stream until it yields an event that carries `marker`, or until
/// `total_budget` runs out. The budget covers the whole call, so a partition
/// full of other runs' events cannot make this wait forever.
async fn next_tagged_event<S>(
    stream: &mut S,
    marker: &str,
    total_budget: std::time::Duration,
) -> Option<ReceivedEventData>
where
    S: futures::Stream<Item = Result<ReceivedEventData>> + Unpin,
{
    let deadline = std::time::Instant::now() + total_budget;
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return None;
        }
        match tokio::time::timeout(remaining, stream.next()).await {
            Err(_) => return None,
            Ok(None) => return None,
            Ok(Some(Err(e))) => panic!("the partition stream returned an error: {e:?}"),
            Ok(Some(Ok(event))) => {
                if marker_of(&event).as_deref() == Some(marker) {
                    return Some(event);
                }
            }
        }
    }
}

/// Retry options for a negative test. The real default is 8 retries over 60
/// seconds, which makes a test that expects a failure wait far too long.
fn short_retry_options() -> RetryOptions {
    RetryOptions {
        initial_delay: Duration::milliseconds(100),
        max_delay: Duration::seconds(1),
        max_retries: 1,
        max_total_elapsed: Duration::seconds(10),
    }
}

fn start_processor_running(processor: &Arc<EventProcessor>) -> JoinHandle<Result<()>> {
    let processor = Arc::clone(processor);
    tokio::spawn(async move { processor.run().await })
}

/// Closes the processor and panics if another reference holds it. A test that
/// silently skips the close leaves the connection open for the next test.
async fn close_processor_strict(processor: Arc<EventProcessor>) -> Result<()> {
    let processor = Arc::try_unwrap(processor)
        .unwrap_or_else(|_| panic!("the processor still has references; the test cannot close it"));
    processor.close().await
}

/// Stops a running processor and makes sure `run()` resolved cleanly.
async fn stop_processor(
    processor: &Arc<EventProcessor>,
    handle: JoinHandle<Result<()>>,
    budget: std::time::Duration,
) {
    processor
        .shutdown()
        .await
        .unwrap_or_else(|e| panic!("shutdown() failed: {e:?}"));
    match tokio::time::timeout(budget, handle).await {
        Err(_) => panic!("run() did not resolve within {budget:?} after shutdown()"),
        Ok(Err(e)) => panic!("the processor task did not join cleanly: {e:?}"),
        Ok(Ok(Err(e))) => panic!("run() returned an error after shutdown(): {e:?}"),
        Ok(Ok(Ok(()))) => {}
    }
}

/// A missing blob container must fail at `run()`, and the failure must carry
/// the storage status. `build()` never touches the checkpoint store, so a
/// caller learns about the container only once the load balancer reads it.
#[recorded::test(live)]
async fn processor_run_fails_on_missing_blob_container(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let hub = recording.var("EVENTHUB_NAME", None);
    let marker = recording.random_string::<20>(Some("r10"));
    let missing_container = format!("missing-{}", azure_core::Uuid::new_v4().simple());

    let container_client = create_container_client(recording, &missing_container)?;
    let store = BlobCheckpointStore::new(container_client);

    let consumer_client = ConsumerClient::builder()
        .with_consumer_group(BLOB_CONSUMER_GROUP.to_string())
        .with_application_id(format!("{marker}-owner"))
        .with_retry_options(short_retry_options())
        .open(host.as_str(), hub.clone(), recording.credential().clone())
        .await?;

    let built = tokio::time::timeout(
        BUILD_ERROR_TIMEOUT,
        EventProcessor::builder()
            .with_load_balancing_strategy(ProcessorStrategy::Balanced)
            .with_update_interval(UPDATE_INTERVAL_FAST)
            .with_partition_expiration_duration(PARTITION_EXPIRATION)
            .with_prefetch(300)
            .with_start_positions(StartPositions {
                per_partition: HashMap::new(),
                default: StartPosition {
                    location: StartLocation::Latest,
                    inclusive: false,
                },
            })
            .build(consumer_client, store),
    )
    .await
    .unwrap_or_else(|_| {
        panic!("build() with missing blob container {missing_container} did not return within {BUILD_ERROR_TIMEOUT:?}")
    });
    let processor = built.unwrap_or_else(|e| {
        panic!("expected build() to succeed with a missing blob container and the error to surface at run(); build() failed instead: {e:?}")
    });

    let run_result = tokio::time::timeout(RUN_ERROR_TIMEOUT, processor.run())
        .await
        .unwrap_or_else(|_| {
            panic!("run() with missing blob container {missing_container} did not return within {RUN_ERROR_TIMEOUT:?}")
        });
    let err = match run_result {
        Ok(()) => panic!("run() returned Ok with missing blob container {missing_container}"),
        Err(e) => e,
    };
    info!("run() failed with missing blob container {missing_container}: {err:?}");

    let ErrorKind::AzureCore(inner) = &err.kind else {
        panic!(
            "expected ErrorKind::AzureCore from the blob checkpoint store, got {:?}",
            err.kind
        )
    };
    match inner.kind() {
        azure_core::error::ErrorKind::HttpResponse {
            status, error_code, ..
        } => {
            assert_eq!(
                *status,
                StatusCode::NotFound,
                "expected 404 from blob storage for missing container {missing_container}"
            );
            assert_eq!(
                error_code.as_deref(),
                Some("ContainerNotFound"),
                "expected the ContainerNotFound error code from blob storage"
            );
        }
        other => panic!("expected an HttpResponse error from blob storage, got {other:?}"),
    }

    close_processor_strict(processor).await?;
    Ok(())
}

/// `update_checkpoint` must reach the blob store. An in-memory store cannot
/// catch a serialization or blob naming defect in the blob store path.
#[recorded::test(live)]
async fn processor_checkpoints_to_blob_store(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let hub = recording.var("EVENTHUB_NAME", None);
    let marker = recording.random_string::<20>(Some("r12"));

    let container_name = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let container_client = create_container_client(recording, &container_name)?;
    let store = BlobCheckpointStore::new(container_client);

    let consumer_client = ConsumerClient::builder()
        .with_consumer_group(BLOB_CONSUMER_GROUP.to_string())
        .with_application_id(format!("{marker}-owner"))
        .open(host.as_str(), hub.clone(), recording.credential().clone())
        .await?;
    let partition_ids = consumer_client
        .get_eventhub_properties()
        .await?
        .partition_ids;
    let target = partition_ids[0].clone();
    let tail = consumer_client
        .get_partition_properties(&target)
        .await?
        .last_enqueued_sequence_number;

    // Seed the store at the tail. Leftover state from an earlier run would
    // otherwise replay hours of retained events, and the final assertion
    // needs a known prior value to compare against.
    store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: host.clone(),
            event_hub_name: hub.clone(),
            consumer_group: BLOB_CONSUMER_GROUP.to_string(),
            partition_id: target.clone(),
            offset: None,
            sequence_number: Some(tail),
        })
        .await?;

    send_tagged_events(&ctx, &target, &marker, BLOB_SEED_EVENT_COUNT).await?;

    let processor = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(UPDATE_INTERVAL_FAST)
        .with_partition_expiration_duration(PARTITION_EXPIRATION)
        .with_prefetch(300)
        .with_max_partition_count(1)
        .with_start_positions(StartPositions {
            per_partition: HashMap::new(),
            default: StartPosition {
                location: StartLocation::SequenceNumber(tail),
                inclusive: false,
            },
        })
        .build(consumer_client, store.clone())
        .await?;
    let running = start_processor_running(&processor);

    let partition_client =
        tokio::time::timeout(FIRST_CLIENT_TIMEOUT, processor.next_partition_client())
            .await
            .unwrap_or_else(|_| {
                panic!("the processor issued no partition client within {FIRST_CLIENT_TIMEOUT:?}")
            })?;
    assert_eq!(partition_client.get_partition_id(), target);

    let last = {
        let mut stream = partition_client.stream_events().boxed_local();
        let mut last = None;
        for expected in 0..BLOB_SEED_EVENT_COUNT {
            let event = next_tagged_event(&mut stream, &marker, EVENT_TIMEOUT)
                .await
                .unwrap_or_else(|| {
                    panic!("the processor streamed no tagged event with index {expected} within {EVENT_TIMEOUT:?}")
                });
            assert_eq!(
                index_of(&event),
                Some(expected),
                "the processor received tagged events out of order"
            );
            last = Some(event);
        }
        last.expect("the read loop must run at least once")
    };
    partition_client.update_checkpoint(&last).await?;

    let cps = store
        .list_checkpoints(&host, &hub, BLOB_CONSUMER_GROUP)
        .await?;
    let cp = cps
        .iter()
        .find(|c| c.partition_id == target)
        .unwrap_or_else(|| {
            panic!("the blob store holds no checkpoint for partition {target}; it holds {cps:?}")
        });
    assert_ne!(
        cp.sequence_number,
        Some(tail),
        "the blob checkpoint still holds the seeded sequence number; update_checkpoint wrote nothing"
    );
    assert_eq!(
        cp.sequence_number,
        last.sequence_number(),
        "the blob checkpoint sequence number does not match the event the test passed to update_checkpoint"
    );

    stop_processor(&processor, running, SHUTDOWN_TIMEOUT).await;
    let partition_client = Arc::try_unwrap(partition_client)
        .unwrap_or_else(|_| panic!("the test cannot close the partition client"));
    partition_client.close().await?;
    close_processor_strict(processor).await?;
    Ok(())
}
