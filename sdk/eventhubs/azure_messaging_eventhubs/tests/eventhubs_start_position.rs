// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Live tests for the consumer start positions.
//!
//! Each test seeds a partition with labeled events, reads the partition tail as
//! a boundary, sends more labeled events, then opens a receiver at a start
//! position derived from that boundary. Every event carries a per-run marker,
//! so a test asserts on its own events and ignores the foreign traffic that
//! shares the partition.

use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    models::{AmqpSimpleValue, EventData, EventHubPartitionProperties, ReceivedEventData},
    ConsumerClient, EventDataBatchOptions, OpenReceiverOptions, ProducerClient, StartLocation,
    StartPosition,
};
use futures::stream::{Stream, StreamExt};
use std::{
    env,
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tracing::info;

const RUN_MARKER_KEY: &str = "start-position-run";
const EVENT_LABEL_KEY: &str = "start-position-label";
const SEED_COUNT: usize = 3;
const SOURCE_COUNT: usize = 3;
const TIME_GAP: Duration = Duration::from_secs(2);
const READ_DEADLINE: Duration = Duration::from_secs(60);
const ATTACH_POKE: Duration = Duration::from_secs(2);
const LATEST_DEADLINE: Duration = Duration::from_secs(60);
const SEND_INTERVAL: Duration = Duration::from_secs(1);
const SEED_DEADLINE: Duration = Duration::from_secs(30);
const SEED_POLL_INTERVAL: Duration = Duration::from_millis(500);
const LATEST_SEND_ROUNDS: usize = 30;

// Several tests seed the same partition. Each one reads the partition tail as
// its start boundary, so a test that seeds in parallel pushes the boundary past
// this test's own events and the test never sees them. Run the tests in turn.
static SERIAL: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// A seeded partition, plus the clients and the boundary that a test reads from.
struct Fixture {
    producer: Arc<ProducerClient>,
    consumer: ConsumerClient,
    run_marker: String,
    properties: EventHubPartitionProperties,
}

/// Sends `SEED_COUNT` labeled events to `partition`, waits until the service
/// reports them, and returns the partition properties as the start boundary.
async fn seed_partition(
    ctx: &TestContext,
    test_name: &str,
    partition: &str,
) -> Result<Fixture, Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = recording.credential();

    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(test_name.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    let run_marker = format!("{test_name}-{}", azure_core::Uuid::new_v4());
    let before = producer
        .get_partition_properties(partition)
        .await?
        .last_enqueued_sequence_number;

    let seed_labels: Vec<String> = (0..SEED_COUNT).map(|i| format!("seed-{i}")).collect();
    send_labeled_batch(&producer, partition, &run_marker, &seed_labels).await?;

    // Partition properties are eventually consistent, so poll for the seed
    // events instead of sleeping a fixed time.
    let target = before + SEED_COUNT as i64;
    let started = tokio::time::Instant::now();
    let properties = loop {
        let properties = producer.get_partition_properties(partition).await?;
        if properties.last_enqueued_sequence_number >= target {
            break properties;
        }
        if started.elapsed() >= SEED_DEADLINE {
            return Err(format!(
                "run {run_marker}: partition {partition} did not report the {SEED_COUNT} seed \
                 events within {SEED_DEADLINE:?}. The sequence number stopped at {} and the test \
                 wanted {target}.",
                properties.last_enqueued_sequence_number
            )
            .into());
        }
        tokio::time::sleep(SEED_POLL_INTERVAL).await;
    };

    assert!(
        !properties.is_empty,
        "run {run_marker}: partition {partition} reports empty after {SEED_COUNT} seed events \
         within {SEED_DEADLINE:?}"
    );
    if properties.last_enqueued_time_utc.is_none() {
        return Err(format!(
            "run {run_marker}: partition {partition} has no last_enqueued_time_utc after the seed \
             within {SEED_DEADLINE:?}, so the enqueued-time boundary is not available"
        )
        .into());
    }

    let consumer = ConsumerClient::builder()
        .with_application_id(test_name.to_string())
        .open(host.as_str(), eventhub, credential)
        .await?;

    Ok(Fixture {
        producer,
        consumer,
        run_marker,
        properties,
    })
}

/// Sends one batch to `partition`. Each event carries the run marker and its
/// own label.
async fn send_labeled_batch(
    producer: &ProducerClient,
    partition: &str,
    run_marker: &str,
    labels: &[String],
) -> azure_messaging_eventhubs::Result<()> {
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(partition.to_string()),
            ..Default::default()
        }))
        .await?;
    for label in labels {
        assert!(
            batch.try_add_event_data(
                EventData::builder()
                    .with_body(label.clone())
                    .add_property(RUN_MARKER_KEY.to_string(), run_marker.to_string())
                    .add_property(EVENT_LABEL_KEY.to_string(), label.clone())
                    .build(),
                None,
            )?,
            "run {run_marker}: event {label} did not fit in the batch for partition {partition}"
        );
    }
    producer.send_batch(batch, None).await
}

/// Returns the label of `event` when the event belongs to this run.
fn tagged_label(event: &ReceivedEventData, run_marker: &str) -> Option<String> {
    let properties = event.event_data().properties()?;
    match properties.get(RUN_MARKER_KEY) {
        Some(AmqpSimpleValue::String(marker)) if marker == run_marker => {}
        _ => return None,
    }
    match properties.get(EVENT_LABEL_KEY) {
        Some(AmqpSimpleValue::String(label)) => Some(label.clone()),
        _ => None,
    }
}

/// Reads until `want_tagged` events of this run arrive or `deadline` expires.
/// Each entry is `Some(label)` for an event of this run and `None` for a
/// foreign event.
async fn read_events<S>(
    stream: &mut S,
    run_marker: &str,
    want_tagged: usize,
    deadline: Duration,
) -> Vec<Option<String>>
where
    S: Stream<Item = azure_messaging_eventhubs::Result<ReceivedEventData>> + Unpin,
{
    let mut seen: Vec<Option<String>> = Vec::new();
    let mut tagged = 0usize;

    // The stream does not end on its own, so the deadline is the only exit.
    let _ = tokio::time::timeout(deadline, async {
        while let Some(event) = stream.next().await {
            match event {
                Ok(event) => {
                    let label = tagged_label(&event, run_marker);
                    if label.is_some() {
                        tagged += 1;
                    }
                    seen.push(label);
                    if tagged >= want_tagged {
                        break;
                    }
                }
                Err(err) => {
                    info!("run {run_marker}: the stream failed, stop reading. {err:?}");
                    break;
                }
            }
        }
    })
    .await;

    seen
}

/// An exclusive offset start position must skip the event at that offset.
#[recorded::test(live)]
async fn start_position_offset_exclusive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "start_position_offset_exclusive";
    const PARTITION: &str = "2";
    let _serial = SERIAL.lock().await;

    let Fixture {
        producer,
        consumer,
        run_marker,
        properties,
    } = seed_partition(&ctx, TEST_NAME, PARTITION).await?;

    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Offset(properties.last_enqueued_offset.clone()),
                    inclusive: false,
                }),
                ..Default::default()
            }),
        )
        .await?;

    let source_labels: Vec<String> = (0..SOURCE_COUNT).map(|i| format!("source-{i}")).collect();
    send_labeled_batch(&producer, PARTITION, &run_marker, &source_labels).await?;

    let tagged: Vec<String> = {
        let mut stream = receiver.stream_events();
        read_events(&mut stream, &run_marker, SOURCE_COUNT, READ_DEADLINE)
            .await
            .into_iter()
            .flatten()
            .collect()
    };

    receiver.close().await?;
    consumer.close().await?;
    Arc::try_unwrap(producer)
        .map_err(|_| "A task still holds the producer.")?
        .close()
        .await?;

    assert!(
        !tagged.is_empty(),
        "run {run_marker}: partition {PARTITION} gave no event of this run within \
         {READ_DEADLINE:?} from the exclusive offset {}",
        properties.last_enqueued_offset
    );
    assert_eq!(
        tagged[0], "source-0",
        "run {run_marker}: partition {PARTITION} started at the wrong event within \
         {READ_DEADLINE:?} from the exclusive offset {}. The events of this run were {tagged:?}.",
        properties.last_enqueued_offset
    );
    assert!(
        !tagged.iter().any(|label| label == "seed-2"),
        "run {run_marker}: partition {PARTITION} gave the boundary event seed-2 within \
         {READ_DEADLINE:?}, but the offset {} is exclusive. The events of this run were {tagged:?}.",
        properties.last_enqueued_offset
    );
    Ok(())
}

/// An inclusive offset start position must give the event at that offset.
#[recorded::test(live)]
async fn start_position_offset_inclusive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "start_position_offset_inclusive";
    const PARTITION: &str = "2";
    let _serial = SERIAL.lock().await;

    let Fixture {
        producer,
        consumer,
        run_marker,
        properties,
    } = seed_partition(&ctx, TEST_NAME, PARTITION).await?;

    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Offset(properties.last_enqueued_offset.clone()),
                    inclusive: true,
                }),
                ..Default::default()
            }),
        )
        .await?;

    let source_labels: Vec<String> = (0..SOURCE_COUNT).map(|i| format!("source-{i}")).collect();
    send_labeled_batch(&producer, PARTITION, &run_marker, &source_labels).await?;

    let tagged: Vec<String> = {
        let mut stream = receiver.stream_events();
        read_events(&mut stream, &run_marker, SOURCE_COUNT + 1, READ_DEADLINE)
            .await
            .into_iter()
            .flatten()
            .collect()
    };

    receiver.close().await?;
    consumer.close().await?;
    Arc::try_unwrap(producer)
        .map_err(|_| "A task still holds the producer.")?
        .close()
        .await?;

    assert!(
        !tagged.is_empty(),
        "run {run_marker}: partition {PARTITION} gave no event of this run within \
         {READ_DEADLINE:?} from the inclusive offset {}",
        properties.last_enqueued_offset
    );
    assert_eq!(
        tagged[0], "seed-2",
        "run {run_marker}: partition {PARTITION} did not start at the boundary event within \
         {READ_DEADLINE:?}, but the offset {} is inclusive. The events of this run were {tagged:?}.",
        properties.last_enqueued_offset
    );
    Ok(())
}

/// An enqueued-time start position must give the events that come after that
/// time.
#[recorded::test(live)]
async fn start_position_enqueued_time(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "start_position_enqueued_time";
    const PARTITION: &str = "3";
    let _serial = SERIAL.lock().await;

    let Fixture {
        producer,
        consumer,
        run_marker,
        properties,
    } = seed_partition(&ctx, TEST_NAME, PARTITION).await?;

    let boundary_time = properties
        .last_enqueued_time_utc
        .ok_or_else(|| format!("run {run_marker}: partition {PARTITION} has no enqueued time"))?;

    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::EnqueuedTime(boundary_time),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // Put a clear gap between the boundary time and the source events.
    tokio::time::sleep(TIME_GAP).await;
    let source_labels: Vec<String> = (0..SOURCE_COUNT).map(|i| format!("source-{i}")).collect();
    send_labeled_batch(&producer, PARTITION, &run_marker, &source_labels).await?;

    let tagged: Vec<String> = {
        let mut stream = receiver.stream_events();
        read_events(
            &mut stream,
            &run_marker,
            SEED_COUNT + SOURCE_COUNT,
            READ_DEADLINE,
        )
        .await
        .into_iter()
        .flatten()
        .collect()
    };

    receiver.close().await?;
    consumer.close().await?;
    Arc::try_unwrap(producer)
        .map_err(|_| "A task still holds the producer.")?
        .close()
        .await?;

    // The service truncates the boundary time to milliseconds, so a seed event
    // from the same millisecond may arrive as well. Assert a lower bound only.
    let sources = tagged
        .iter()
        .filter(|label| label.starts_with("source-"))
        .count();
    assert!(
        sources >= SOURCE_COUNT,
        "run {run_marker}: partition {PARTITION} gave {sources} of {SOURCE_COUNT} source events \
         within {READ_DEADLINE:?} from the enqueued time {boundary_time:?}. The events of this run \
         were {tagged:?}."
    );
    Ok(())
}

/// The Latest start position must give only the events that arrive after the
/// receiver attaches.
#[recorded::test(live)]
async fn start_position_latest(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "start_position_latest";
    const PARTITION: &str = "3";
    const WANT_TAGGED: usize = 3;
    let _serial = SERIAL.lock().await;

    let Fixture {
        producer,
        consumer,
        run_marker,
        ..
    } = seed_partition(&ctx, TEST_NAME, PARTITION).await?;

    // Warm the connection, so the receiver attach is not the first round trip.
    consumer.get_partition_properties(PARTITION).await?;

    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Latest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let stop = Arc::new(AtomicBool::new(false));
    let tagged: Vec<String>;
    let sender;
    {
        let mut stream = receiver.stream_events();

        // Force the AMQP attach, so Latest is anchored before the sender starts.
        // The read expires because no event is available yet.
        let _ = tokio::time::timeout(ATTACH_POKE, stream.next()).await;

        sender = tokio::spawn({
            let producer = producer.clone();
            let run_marker = run_marker.clone();
            let stop = stop.clone();
            async move {
                for round in 0..LATEST_SEND_ROUNDS {
                    if stop.load(Ordering::SeqCst) {
                        break;
                    }
                    let labels: Vec<String> = (0..3).map(|i| format!("post-{round}-{i}")).collect();
                    if let Err(err) =
                        send_labeled_batch(&producer, PARTITION, &run_marker, &labels).await
                    {
                        info!("run {run_marker}: the background sender stopped. {err:?}");
                        break;
                    }
                    tokio::time::sleep(SEND_INTERVAL).await;
                }
            }
        });

        tagged = read_events(&mut stream, &run_marker, WANT_TAGGED, LATEST_DEADLINE)
            .await
            .into_iter()
            .flatten()
            .collect();
    }

    stop.store(true, Ordering::SeqCst);
    sender.await?;

    receiver.close().await?;
    consumer.close().await?;
    Arc::try_unwrap(producer)
        .map_err(|_| "A task still holds the producer.")?
        .close()
        .await?;

    // The sender repeats the labels each round, so a reader that attaches in
    // the middle of a round sees a rotated set. Assert the group, not the order.
    assert!(
        tagged.iter().any(|label| label.starts_with("post-")),
        "run {run_marker}: partition {PARTITION} gave no event that the sender wrote after the \
         attach within {LATEST_DEADLINE:?}. The events of this run were {tagged:?}."
    );
    assert!(
        !tagged.iter().any(|label| label.starts_with("seed-")),
        "run {run_marker}: partition {PARTITION} gave a seed event within {LATEST_DEADLINE:?}, \
         but Latest must skip the events that come before the attach. The events of this run were \
         {tagged:?}."
    );
    Ok(())
}

/// An inclusive sequence-number start position must give the event at that
/// sequence number.
#[recorded::test(live)]
async fn start_position_sequence_number_inclusive(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "start_position_sequence_number_inclusive";
    const PARTITION: &str = "2";
    let _serial = SERIAL.lock().await;

    let Fixture {
        producer,
        consumer,
        run_marker,
        properties,
    } = seed_partition(&ctx, TEST_NAME, PARTITION).await?;

    let boundary_sequence = properties.last_enqueued_sequence_number;
    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary_sequence),
                    inclusive: true,
                }),
                ..Default::default()
            }),
        )
        .await?;

    let source_labels: Vec<String> = (0..SOURCE_COUNT).map(|i| format!("source-{i}")).collect();
    send_labeled_batch(&producer, PARTITION, &run_marker, &source_labels).await?;

    let tagged: Vec<String> = {
        let mut stream = receiver.stream_events();
        read_events(&mut stream, &run_marker, SOURCE_COUNT + 1, READ_DEADLINE)
            .await
            .into_iter()
            .flatten()
            .collect()
    };

    receiver.close().await?;
    consumer.close().await?;
    Arc::try_unwrap(producer)
        .map_err(|_| "A task still holds the producer.")?
        .close()
        .await?;

    assert!(
        !tagged.is_empty(),
        "run {run_marker}: partition {PARTITION} gave no event of this run within \
         {READ_DEADLINE:?} from the inclusive sequence number {boundary_sequence}"
    );
    assert_eq!(
        tagged[0], "seed-2",
        "run {run_marker}: partition {PARTITION} did not start at the boundary event within \
         {READ_DEADLINE:?}, but the sequence number {boundary_sequence} is inclusive. The events \
         of this run were {tagged:?}."
    );
    Ok(())
}
