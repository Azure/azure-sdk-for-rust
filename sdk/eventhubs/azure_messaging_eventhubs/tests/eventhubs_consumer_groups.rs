// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Live tests for Event Hubs consumer groups.
//!
//! Every event carries a marker for the run in its application properties. A
//! reader keeps only the events that have the marker of its own run, so
//! foreign traffic on the shared namespace does not change a result.
//!
//! `open_receiver_on_partition` does no network I/O. The broker attaches the
//! link on the first poll of `stream_events()`. An attach error therefore
//! arrives as the first item of the stream.

use azure_core_amqp::{error::AmqpErrorKind, AmqpDescribedError, AmqpError, AmqpErrorCondition};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind,
    models::{AmqpSimpleValue, EventData, ReceivedEventData},
    ConsumerClient, EventDataBatchOptions, EventHubsError, OpenReceiverOptions, ProducerClient,
    Result, StartLocation, StartPosition,
};
use futures::{Stream, StreamExt};
use std::{sync::LazyLock, time::Duration};
use tokio::time::timeout;

const DEFAULT_GROUP: &str = "$Default";
const SECOND_GROUP: &str = "defaultGroup";
const MISSING_GROUP: &str = "ThisIsFake";
const MISSING_PARTITION: &str = "-1";
const PARTITION: &str = "0";
const EVENT_COUNT: usize = 5;
const RUN_MARKER_KEY: &str = "consumer-group-run";
const EVENT_LABEL_KEY: &str = "consumer-group-label";

/// The time each consumer group gets to deliver every event of the run.
const READ_DEADLINE: Duration = Duration::from_secs(60);

/// The time the broker gets to reject one attach. The attach is not retried.
const ATTACH_ERROR_DEADLINE: Duration = Duration::from_secs(30);

// The three tests share partition 0, and cargo runs them on parallel threads.
static PARTITION_LOCK: LazyLock<async_lock::Mutex<()>> =
    LazyLock::new(|| async_lock::Mutex::new(()));

/// Finds the AMQP described error that the broker sent, either in this error
/// or in its source chain.
fn described_error(amqp: &AmqpError) -> Option<&AmqpDescribedError> {
    if let AmqpErrorKind::AmqpDescribedError(described) = amqp.kind() {
        return Some(described);
    }
    let mut source = std::error::Error::source(amqp);
    for _ in 0..16 {
        let current = source?;
        let inner = current
            .downcast_ref::<AmqpError>()
            .or_else(|| current.downcast_ref::<Box<AmqpError>>().map(|e| e.as_ref()));
        if let Some(inner) = inner {
            if let AmqpErrorKind::AmqpDescribedError(described) = inner.kind() {
                return Some(described);
            }
        }
        source = current.source();
    }
    None
}

/// Takes exactly one item from the stream and returns the error that it holds.
async fn first_stream_error<S>(stream: &mut S, limit: Duration, context: &str) -> EventHubsError
where
    S: Stream<Item = Result<ReceivedEventData>> + Unpin,
{
    match timeout(limit, stream.next()).await {
        Err(_) => panic!(
            "{context}: no stream item arrived within {limit:?}. The attach must fail, not hang."
        ),
        Ok(None) => panic!("{context}: the stream ended with no item."),
        Ok(Some(Ok(event))) => panic!(
            "{context}: expected an attach error, got an event at sequence number {:?}",
            event.sequence_number()
        ),
        Ok(Some(Err(err))) => err,
    }
}

/// Makes sure that the attach failed with one of the expected conditions.
///
/// The expected sets come from the .NET mapping in
/// `Azure.Messaging.EventHubs/src/Amqp/AmqpError.cs`. They are not verified
/// against this broker. `authorizer.authorize_path` runs at CBS put-token time
/// before the attach, and that path carries the consumer group segment, so a
/// rejection there can carry a different condition. Step 4 names the observed
/// condition, so the first live run can correct the set.
fn assert_attach_condition(err: &EventHubsError, expected: &[AmqpErrorCondition], context: &str) {
    assert!(
        !matches!(err.kind, ErrorKind::ConsumerDisconnected(_)),
        "{context}: the error says that the broker stole the link. Expected one of {expected:?}."
    );
    let ErrorKind::AmqpError(amqp) = &err.kind else {
        panic!(
            "{context}: expected ErrorKind::AmqpError, got {:?}",
            err.kind
        );
    };
    let Some(described) = described_error(amqp) else {
        panic!("{context}: no AMQP described error in the source chain: {amqp:?}");
    };
    assert!(
        expected.contains(&described.condition),
        "{context}: observed condition {:?} with description {:?}. Expected one of {expected:?}.",
        described.condition,
        described.description
    );
}

/// Sends one batch to `partition`, with one event for each label.
async fn send_labeled_batch(
    producer: &ProducerClient,
    partition: &str,
    run_marker: &str,
    labels: &[String],
) -> Result<()> {
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(partition.to_string()),
            ..Default::default()
        }))
        .await?;
    for label in labels {
        let event = EventData::builder()
            .with_body(label.clone())
            .add_property(RUN_MARKER_KEY.to_string(), run_marker.to_string())
            .add_property(EVENT_LABEL_KEY.to_string(), label.clone())
            .build();
        assert!(
            batch.try_add_event_data(event, None)?,
            "run {run_marker}: the batch had no room for {label}"
        );
    }
    producer.send_batch(batch, None).await
}

/// Returns the label of an event that carries the marker of this run.
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

/// Reads the events of this run, and drops the events of every other run.
async fn read_run_events<S>(
    stream: &mut S,
    run_marker: &str,
    want: usize,
    deadline: Duration,
) -> Vec<(String, i64)>
where
    S: Stream<Item = Result<ReceivedEventData>> + Unpin,
{
    let mut found: Vec<(String, i64)> = Vec::with_capacity(want);
    let _ = timeout(deadline, async {
        while let Some(item) = stream.next().await {
            let event = match item {
                Ok(event) => event,
                Err(err) => panic!("run {run_marker}: the stream failed while reading: {err:?}"),
            };
            let Some(label) = tagged_label(&event, run_marker) else {
                continue;
            };
            let sequence = event.sequence_number().unwrap_or_else(|| {
                panic!("run {run_marker}: the event for {label} has no sequence number")
            });
            found.push((label, sequence));
            if found.len() == want {
                break;
            }
        }
    })
    .await;
    found
}

#[recorded::test(live)]
async fn two_consumer_groups_read_the_same_events(ctx: TestContext) -> Result<()> {
    let _guard = PARTITION_LOCK.lock().await;

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let run_marker = format!("consumer-groups-{}", azure_core::Uuid::new_v4());
    let start_sequence = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    let expected: Vec<String> = (0..EVENT_COUNT).map(|i| format!("event-{i}")).collect();
    send_labeled_batch(&producer, PARTITION, &run_marker, &expected).await?;

    let default_consumer = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let second_consumer = ConsumerClient::builder()
        .with_consumer_group(SECOND_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    // Do not set owner_level. An epoch consumer displaces the other reader.
    let start_position = StartPosition {
        location: StartLocation::SequenceNumber(start_sequence),
        inclusive: false,
    };
    let default_receiver = default_consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(start_position.clone()),
                ..Default::default()
            }),
        )
        .await?;
    let second_receiver = second_consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(start_position),
                ..Default::default()
            }),
        )
        .await?;

    let default_read = {
        let mut stream = default_receiver.stream_events();
        read_run_events(&mut stream, &run_marker, EVENT_COUNT, READ_DEADLINE).await
    };
    let second_read = {
        let mut stream = second_receiver.stream_events();
        read_run_events(&mut stream, &run_marker, EVENT_COUNT, READ_DEADLINE).await
    };

    default_receiver.close().await?;
    second_receiver.close().await?;
    default_consumer.close().await?;
    second_consumer.close().await?;
    producer.close().await?;

    let (labels_default, sequences_default): (Vec<String>, Vec<i64>) =
        default_read.into_iter().unzip();
    let (labels_second, sequences_second): (Vec<String>, Vec<i64>) =
        second_read.into_iter().unzip();

    assert_eq!(
        labels_default, expected,
        "run {run_marker}: consumer group {DEFAULT_GROUP} on partition {PARTITION} did not read every event of this run within {READ_DEADLINE:?}"
    );
    assert_eq!(
        labels_second, expected,
        "run {run_marker}: consumer group {SECOND_GROUP} on partition {PARTITION} did not read every event of this run within {READ_DEADLINE:?}"
    );
    assert_eq!(
        sequences_default, sequences_second,
        "run {run_marker}: a sequence number is a property of the partition, so the two consumer groups must report the same sequence numbers"
    );

    Ok(())
}

#[recorded::test(live)]
async fn receiver_on_unknown_consumer_group_fails(ctx: TestContext) -> Result<()> {
    const CONTEXT: &str = "receiver on an unknown consumer group";
    let _guard = PARTITION_LOCK.lock().await;

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);

    let consumer = ConsumerClient::builder()
        .with_consumer_group(MISSING_GROUP.to_string())
        .open(host.as_str(), eventhub, recording.credential())
        .await?;
    // Earliest makes the test fail fast. If the consumer group segment stopped
    // reaching the wire, the receiver attaches on $Default and gets an event.
    let receiver = consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let observed = {
        let mut stream = receiver.stream_events();
        first_stream_error(&mut stream, ATTACH_ERROR_DEADLINE, CONTEXT).await
    };

    let _ = receiver.close().await;
    let _ = consumer.close().await;

    assert_attach_condition(&observed, &[AmqpErrorCondition::NotFound], CONTEXT);

    Ok(())
}

#[recorded::test(live)]
async fn receiver_on_unknown_partition_fails(ctx: TestContext) -> Result<()> {
    const CONTEXT: &str = "receiver on an unknown partition";
    let _guard = PARTITION_LOCK.lock().await;

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);

    let consumer = ConsumerClient::builder()
        .open(host.as_str(), eventhub, recording.credential())
        .await?;
    let receiver = consumer
        .open_receiver_on_partition(
            MISSING_PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let observed = {
        let mut stream = receiver.stream_events();
        first_stream_error(&mut stream, ATTACH_ERROR_DEADLINE, CONTEXT).await
    };

    let _ = receiver.close().await;
    let _ = consumer.close().await;

    assert_attach_condition(
        &observed,
        &[
            AmqpErrorCondition::ArgumentOutOfRangeError,
            AmqpErrorCondition::NotFound,
        ],
        CONTEXT,
    );

    Ok(())
}
