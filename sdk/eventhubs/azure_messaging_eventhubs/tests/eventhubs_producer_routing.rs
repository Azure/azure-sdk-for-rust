// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Live tests for producer routing and for the payload edges of an event.
//!
//! Each test captures the tail sequence number of a partition before it sends,
//! then reads strictly after that boundary. Every event carries a per-run
//! marker, so a test asserts on its own events and ignores the foreign traffic
//! that shares the partition. No assertion depends on a partition tail staying
//! still, so these tests need no lock against each other.

use azure_core_amqp::{message::AmqpMessageBody, AmqpList};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind,
    models::{AmqpMessage, AmqpSimpleValue, AmqpValue, EventData, ReceivedEventData},
    ConsumerClient, EventDataBatchOptions, EventReceiver, OpenReceiverOptions, ProducerClient,
    SendEventOptions, StartLocation, StartPosition,
};
use futures::stream::StreamExt;
use std::{collections::HashMap, env, error::Error, time::Duration};
use tracing::info;

const RUN_MARKER_KEY: &str = "producer-routing-run";
const EVENT_LABEL_KEY: &str = "producer-routing-label";
const ROUTED_EVENT_COUNT: usize = 5;

// Event Hubs caps a single event by tier: 256 KB on Basic, 1 MB on Standard,
// 1 MB on Premium, and 20 MB on Dedicated.
// https://learn.microsoft.com/en-us/azure/event-hubs/event-hubs-quotas
// 100_000 bytes is safe on every tier. The .NET test
// ProducerCanSendSingleLargeEventInASet uses new byte[100000] with the comment
// "Actual limit is 1046520 for a single event".
const LARGE_BODY_LEN: usize = 100_000;
const SEQUENCE_INT_VALUE: i32 = 1_234_567_890;
const BINARY_PROPERTY_KEY: &str = "producer-routing-binary";
const BINARY_PROPERTY_VALUE: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

// A sweep pays this deadline once for each partition that holds no marked
// event. A namespace with 32 partitions can cost about 10 minutes in the worst
// ordering.
const PARTITION_SWEEP_DEADLINE: Duration = Duration::from_secs(20);
const READ_DEADLINE: Duration = Duration::from_secs(60);

/// Returns the label of an event when the event belongs to this run.
///
/// The function takes the property map and not the event, so a plain unit test
/// can build the input.
fn tagged_label(
    properties: Option<&HashMap<String, AmqpSimpleValue>>,
    run_marker: &str,
) -> Option<String> {
    let properties = properties?;
    match properties.get(RUN_MARKER_KEY) {
        Some(AmqpSimpleValue::String(marker)) if marker == run_marker => {}
        _ => return None,
    }
    match properties.get(EVENT_LABEL_KEY) {
        Some(AmqpSimpleValue::String(label)) => Some(label.clone()),
        _ => None,
    }
}

/// Builds a body of `len` bytes that holds a repeating pattern.
///
/// 251 is a prime number, so the pattern never aligns with a 256 byte block or
/// with any other power of two block.
fn large_body(len: usize) -> Vec<u8> {
    (0..len).map(|index| (index % 251) as u8).collect()
}

/// Opens a producer for `test_name`.
async fn open_producer(
    ctx: &TestContext,
    test_name: &str,
) -> Result<ProducerClient, Box<dyn Error>> {
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = ctx.recording().credential();
    Ok(ProducerClient::builder()
        .with_application_id(test_name.to_string())
        .open(host.as_str(), eventhub.as_str(), credential)
        .await?)
}

/// Opens a consumer for `test_name`.
async fn open_consumer(
    ctx: &TestContext,
    test_name: &str,
) -> Result<ConsumerClient, Box<dyn Error>> {
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = ctx.recording().credential();
    Ok(ConsumerClient::builder()
        .with_application_id(test_name.to_string())
        .open(host.as_str(), eventhub, credential)
        .await?)
}

/// Reads until `want` events of this run arrive or `deadline` expires.
async fn read_run_events(
    receiver: &EventReceiver,
    run_marker: &str,
    want: usize,
    deadline: Duration,
) -> Vec<ReceivedEventData> {
    let mut found: Vec<ReceivedEventData> = Vec::new();
    let mut stream = receiver.stream_events();

    // The stream does not end on its own, so the deadline is the only exit.
    let _ = tokio::time::timeout(deadline, async {
        while let Some(event) = stream.next().await {
            match event {
                Ok(event) => {
                    if tagged_label(event.event_data().properties(), run_marker).is_some() {
                        found.push(event);
                    }
                    if found.len() >= want {
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

    found
}

/// Reads the events of this run from one partition, starting after
/// `from_sequence`. Exactly one receiver is open at any moment.
async fn read_run_events_on_partition(
    consumer: &ConsumerClient,
    partition: &str,
    from_sequence: i64,
    run_marker: &str,
    want: usize,
    deadline: Duration,
) -> Result<Vec<ReceivedEventData>, Box<dyn Error>> {
    let receiver = consumer
        .open_receiver_on_partition(
            partition.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(from_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;
    let events = read_run_events(&receiver, run_marker, want, deadline).await;
    receiver.close().await?;
    Ok(events)
}

/// Reads the last sequence number of every partition. A test reads strictly
/// after these numbers, so a foreign event that came before is never read.
async fn capture_tails(
    producer: &ProducerClient,
    partitions: &[String],
) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let mut tails = HashMap::new();
    for partition in partitions {
        let properties = producer.get_partition_properties(partition).await?;
        tails.insert(partition.clone(), properties.last_enqueued_sequence_number);
    }
    Ok(tails)
}

/// Builds an event that carries the run marker and its own label.
fn marked_event(run_marker: &str, label: &str, body: impl Into<Vec<u8>>) -> EventData {
    EventData::builder()
        .with_body(body)
        .add_property(RUN_MARKER_KEY.to_string(), run_marker.to_string())
        .add_property(EVENT_LABEL_KEY.to_string(), label.to_string())
        .build()
}

/// Sends one batch with `key` as the partition key. The batch names no
/// partition id, so the service picks the partition from the key.
async fn send_keyed_batch(
    producer: &ProducerClient,
    key: &str,
    run_marker: &str,
    labels: &[String],
) -> Result<(), Box<dyn Error>> {
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_key: Some(key.to_string()),
            partition_id: None,
            ..Default::default()
        }))
        .await?;
    for label in labels {
        assert!(
            batch.try_add_event_data(marked_event(run_marker, label, label.clone()), None)?,
            "run {run_marker}: event {label} did not fit in the batch for key {key}"
        );
    }
    producer.send_batch(batch, None).await?;
    Ok(())
}

/// Returns the first partition in `order` that holds every marked event of one
/// batch of this run.
async fn sweep_for_batch(
    consumer: &ConsumerClient,
    order: &[String],
    tails: &HashMap<String, i64>,
    run_marker: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    for partition in order {
        let events = read_run_events_on_partition(
            consumer,
            partition,
            tails[partition],
            run_marker,
            ROUTED_EVENT_COUNT,
            PARTITION_SWEEP_DEADLINE,
        )
        .await?;
        if events.len() >= ROUTED_EVENT_COUNT {
            return Ok(Some(partition.clone()));
        }
    }
    Ok(None)
}

/// A batch that carries a partition key must land on exactly one partition, and
/// every event of that batch must arrive with the key.
///
/// The test never asserts a specific partition id, and it never asserts that a
/// different key lands on a different partition. The hash is stable only for a
/// fixed partition count, and a hash over 4 partitions can put two keys on the
/// same partition.
#[recorded::test(live)]
async fn partition_key_routes_batch_to_one_partition(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "partition_key_routes_batch_to_one_partition";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;

    let key = format!("routing-{}", azure_core::Uuid::new_v4());
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let partitions = producer.get_eventhub_properties().await?.partition_ids;
    assert!(
        partitions.len() > 1,
        "run {run_marker}: the Event Hub reports {} partition(s), so this test cannot tell one \
         partition from another",
        partitions.len()
    );

    let tails = capture_tails(&producer, &partitions).await?;
    let labels: Vec<String> = (0..ROUTED_EVENT_COUNT)
        .map(|index| format!("routed-{index}"))
        .collect();
    send_keyed_batch(&producer, &key, &run_marker, &labels).await?;

    let mut carriers: Vec<(String, usize)> = Vec::new();
    let mut received_keys: Vec<Option<String>> = Vec::new();
    for partition in &partitions {
        let events = read_run_events_on_partition(
            &consumer,
            partition,
            tails[partition],
            &run_marker,
            ROUTED_EVENT_COUNT,
            PARTITION_SWEEP_DEADLINE,
        )
        .await?;
        if events.is_empty() {
            continue;
        }
        for event in &events {
            received_keys.push(event.partition_key().clone());
        }
        let count = events.len();
        carriers.push((partition.clone(), count));
        // The test sent exactly ROUTED_EVENT_COUNT events, so once one partition
        // holds all of them no other partition can hold one. Stop the sweep and
        // save the deadline that each remaining partition would cost.
        if count >= ROUTED_EVENT_COUNT {
            break;
        }
    }

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        carriers.len(),
        1,
        "run {run_marker}: key {key} put events on {} partition(s) of {}, and a partition key must \
         pick exactly one. Each partition read had a {PARTITION_SWEEP_DEADLINE:?} deadline. The \
         partitions that carried events were {carriers:?}.",
        carriers.len(),
        partitions.len()
    );
    let (carrier, count) = &carriers[0];
    assert_eq!(
        *count, ROUTED_EVENT_COUNT,
        "run {run_marker}: partition {carrier} gave {count} of the {ROUTED_EVENT_COUNT} events of \
         key {key} within {PARTITION_SWEEP_DEADLINE:?}"
    );
    for partition_key in &received_keys {
        assert_eq!(
            partition_key.as_deref(),
            Some(key.as_str()),
            "run {run_marker}: an event on partition {carrier} arrived with the partition key \
             {partition_key:?}, and every event of this batch must carry {key}"
        );
    }
    Ok(())
}

/// Two batches that carry the same partition key must land on the same
/// partition.
#[recorded::test(live)]
async fn same_partition_key_routes_to_same_partition(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "same_partition_key_routes_to_same_partition";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;

    let key = format!("routing-{}", azure_core::Uuid::new_v4());
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let partitions = producer.get_eventhub_properties().await?.partition_ids;
    assert!(
        partitions.len() > 1,
        "run {run_marker}: the Event Hub reports {} partition(s), so this test cannot tell one \
         partition from another",
        partitions.len()
    );

    let tails_a = capture_tails(&producer, &partitions).await?;
    let labels_a: Vec<String> = (0..ROUTED_EVENT_COUNT)
        .map(|index| format!("a-{index}"))
        .collect();
    send_keyed_batch(&producer, &key, &run_marker, &labels_a).await?;
    let partition_a = sweep_for_batch(&consumer, &partitions, &tails_a, &run_marker)
        .await?
        .ok_or_else(|| {
            format!(
                "run {run_marker}: no partition of the {} gave all {ROUTED_EVENT_COUNT} events of \
                 batch a for key {key}. Each partition read had a {PARTITION_SWEEP_DEADLINE:?} \
                 deadline.",
                partitions.len()
            )
        })?;

    // The second capture is the boundary for batch b, so every marked event that
    // the second sweep reads is a batch b event.
    let tails_b = capture_tails(&producer, &partitions).await?;
    let labels_b: Vec<String> = (0..ROUTED_EVENT_COUNT)
        .map(|index| format!("b-{index}"))
        .collect();
    send_keyed_batch(&producer, &key, &run_marker, &labels_b).await?;

    // Read partition_a first. The passing case then costs one fast read, and the
    // failing case still names where batch b landed.
    let mut order: Vec<String> = vec![partition_a.clone()];
    order.extend(
        partitions
            .iter()
            .filter(|partition| **partition != partition_a)
            .cloned(),
    );
    let partition_b = sweep_for_batch(&consumer, &order, &tails_b, &run_marker).await?;

    consumer.close().await?;
    producer.close().await?;

    let partition_b = partition_b.ok_or_else(|| {
        format!(
            "run {run_marker}: no partition of the {} gave all {ROUTED_EVENT_COUNT} events of \
             batch b for key {key}, and batch a landed on partition {partition_a}. Each partition \
             read had a {PARTITION_SWEEP_DEADLINE:?} deadline.",
            partitions.len()
        )
    })?;
    assert_eq!(
        partition_b, partition_a,
        "run {run_marker}: key {key} sent batch a to partition {partition_a} and batch b to \
         partition {partition_b}. One key must always pick the same partition. Each partition read \
         had a {PARTITION_SWEEP_DEADLINE:?} deadline."
    );
    Ok(())
}

/// A zero length body must arrive as an empty slice.
#[recorded::test(live)]
async fn zero_length_body_round_trips_as_empty_slice(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "zero_length_body_round_trips_as_empty_slice";
    const PARTITION: &str = "0";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let tail = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    // The explicit empty Vec makes a zero length Data section, which matches the
    // .NET Array.Empty<byte>() case. A builder with no with_body call makes an
    // absent body section, a different wire shape, so do not omit the call.
    producer
        .send_event(
            marked_event(&run_marker, "empty-body", Vec::<u8>::new()),
            Some(SendEventOptions {
                partition_id: Some(PARTITION.to_string()),
            }),
        )
        .await?;

    let events =
        read_run_events_on_partition(&consumer, PARTITION, tail, &run_marker, 1, READ_DEADLINE)
            .await?;

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        events.len(),
        1,
        "run {run_marker}: partition {PARTITION} gave {} marked event(s) within {READ_DEADLINE:?} \
         after sequence number {tail}, and the test wanted 1",
        events.len()
    );
    assert_eq!(
        events[0].event_data().body(),
        Some(&[][..]),
        "run {run_marker}: a zero length body must arrive as an empty slice and not as an absent \
         body. Partition {PARTITION}, after sequence number {tail}, deadline {READ_DEADLINE:?}."
    );
    Ok(())
}

/// A large body must arrive byte for byte.
#[recorded::test(live)]
async fn large_event_body_round_trips_intact(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "large_event_body_round_trips_intact";
    const PARTITION: &str = "0";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());
    let body = large_body(LARGE_BODY_LEN);

    let tail = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    // The batch names no max_size_in_bytes, so the sender link maximum applies
    // and the batch accepts an event of the size the service allows.
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(PARTITION.to_string()),
            ..Default::default()
        }))
        .await?;
    assert!(
        batch.try_add_event_data(marked_event(&run_marker, "large-body", body.clone()), None)?,
        "run {run_marker}: a {LARGE_BODY_LEN} byte event must fit under the sender link maximum; a \
         smaller Event Hubs tier fails here. Partition {PARTITION}."
    );
    producer.send_batch(batch, None).await?;

    let events =
        read_run_events_on_partition(&consumer, PARTITION, tail, &run_marker, 1, READ_DEADLINE)
            .await?;

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        events.len(),
        1,
        "run {run_marker}: partition {PARTITION} gave {} marked event(s) within {READ_DEADLINE:?} \
         after sequence number {tail}, and the test wanted 1",
        events.len()
    );
    let received = events[0].event_data().body().unwrap_or_else(|| {
        panic!(
            "run {run_marker}: the large event arrived with no body from partition {PARTITION} \
             within {READ_DEADLINE:?} after sequence number {tail}"
        )
    });

    // Compare the length first and then the first difference. A direct
    // assert_eq! of two 100 KB slices prints an unreadable panic.
    assert_eq!(
        received.len(),
        LARGE_BODY_LEN,
        "run {run_marker}: partition {PARTITION} gave a body of {} bytes within {READ_DEADLINE:?}, \
         and the test sent {LARGE_BODY_LEN} bytes",
        received.len()
    );
    let first_diff = received
        .iter()
        .zip(body.iter())
        .position(|(left, right)| left != right);
    assert!(
        first_diff.is_none(),
        "run {run_marker}: the received body differs from the sent body at index {first_diff:?}. \
         Partition {PARTITION}, deadline {READ_DEADLINE:?}."
    );
    Ok(())
}

/// A batch refuses an event above the negotiated link maximum.
///
/// This mirrors the .NET `ProducerCannotSendSetLargerThanMaximumSize`, which
/// also asserts on a set rather than on one event. The maximum is read from the
/// `InvalidBatchSize` error rather than hardcoded, so the test holds on every
/// tier: Basic caps a publication at 256 KB, Standard and Premium at 1 MB, and
/// Dedicated at 20 MB.
/// <https://learn.microsoft.com/en-us/azure/event-hubs/event-hubs-quotas>
///
/// `ProducerClient::send_event` does NOT enforce this maximum. A live run
/// against a Standard namespace whose link reported `max_allowed: 1048576`
/// accepted a 2 MiB single event, returned `Ok(())`, and moved the partition
/// tail. That divergence between the batch path and the single event path
/// needs its own issue and its own fix, so no test here pins it.
#[recorded::test(live)]
async fn batch_refuses_event_above_the_link_maximum(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "batch_refuses_event_above_the_link_maximum";
    const PARTITION: &str = "0";
    const ABSURD_BATCH_SIZE: u64 = 512 * 1024 * 1024;

    let producer = open_producer(&ctx, TEST_NAME).await?;

    // Ask for a batch far above any link maximum. The error carries the real
    // maximum, which is the only public way to read it.
    let error = producer
        .create_batch(Some(EventDataBatchOptions {
            max_size_in_bytes: Some(ABSURD_BATCH_SIZE),
            partition_id: Some(PARTITION.to_string()),
            ..Default::default()
        }))
        .await
        .err()
        .expect("a batch above the link maximum must be refused");

    let max_allowed = match error.kind {
        ErrorKind::InvalidBatchSize { max_allowed, .. } => max_allowed,
        other => panic!("expected InvalidBatchSize, got {other:?}"),
    };
    info!("{TEST_NAME}: the link reports max_allowed {max_allowed} bytes.");
    assert!(
        max_allowed > 0,
        "the link maximum must be positive, got {max_allowed}"
    );

    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(PARTITION.to_string()),
            ..Default::default()
        }))
        .await?;

    let oversized = usize::try_from(max_allowed).expect("link maximum must fit in a usize") + 1;
    let added = batch.try_add_event_data(
        EventData::builder()
            .with_body(large_body(oversized))
            .build(),
        None,
    )?;
    assert!(
        !added,
        "a {oversized} byte event must not fit a batch whose maximum is {max_allowed}"
    );

    producer.close().await?;
    Ok(())
}

/// An AMQP value body must arrive as a value body, and EventData must report no
/// body for it.
#[recorded::test(live)]
async fn amqp_value_body_round_trips_without_event_data_body(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "amqp_value_body_round_trips_without_event_data_body";
    const PARTITION: &str = "0";
    const VALUE_BODY: &str = "producer-routing-value-body";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let tail = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(PARTITION.to_string()),
            ..Default::default()
        }))
        .await?;
    assert!(
        batch.try_add_amqp_message(
            AmqpMessage::builder()
                .with_body(AmqpValue::from(VALUE_BODY))
                .add_application_property(RUN_MARKER_KEY.to_string(), run_marker.clone())
                .add_application_property(EVENT_LABEL_KEY.to_string(), "value-body".to_string())
                .build(),
            None,
        )?,
        "run {run_marker}: the value body message did not fit in the batch for partition \
         {PARTITION}"
    );
    producer.send_batch(batch, None).await?;

    let events =
        read_run_events_on_partition(&consumer, PARTITION, tail, &run_marker, 1, READ_DEADLINE)
            .await?;

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        events.len(),
        1,
        "run {run_marker}: partition {PARTITION} gave {} marked event(s) within {READ_DEADLINE:?} \
         after sequence number {tail}, and the test wanted 1",
        events.len()
    );
    let event = &events[0];
    match &event.raw_amqp_message().body {
        AmqpMessageBody::Value(AmqpValue::String(text)) => assert_eq!(
            text.as_str(),
            VALUE_BODY,
            "run {run_marker}: the value body arrived with the wrong text from partition \
             {PARTITION} within {READ_DEADLINE:?}"
        ),
        other => panic!(
            "run {run_marker}: the value body must arrive as AmqpMessageBody::Value(String) from \
             partition {PARTITION} within {READ_DEADLINE:?}, got {other:?}"
        ),
    }

    // EventData::from_message copies a body only for an AmqpMessageBody::Binary
    // that holds exactly one element, so a value body gives None by design.
    assert!(
        event.event_data().body().is_none(),
        "run {run_marker}: a value body must give no EventData body, because only a single element \
         binary body is copied. Partition {PARTITION}, deadline {READ_DEADLINE:?}."
    );
    Ok(())
}

/// An AMQP sequence body must arrive with its elements in order and with the
/// type of each element intact.
#[recorded::test(live)]
async fn amqp_sequence_body_round_trips_in_order(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "amqp_sequence_body_round_trips_in_order";
    const PARTITION: &str = "0";
    const SEQUENCE_TEXT: &str = "sequence-text";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let tail = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(PARTITION.to_string()),
            ..Default::default()
        }))
        .await?;
    // A small integer can go on the wire in one byte and come back in a narrower
    // variant, which would make the assertion below flake on the encoding width.
    // A value above 2^16 forces a 4 byte integer.
    assert!(
        batch.try_add_amqp_message(
            AmqpMessage::builder()
                .with_body(vec![
                    AmqpValue::from(SEQUENCE_TEXT),
                    AmqpValue::from(SEQUENCE_INT_VALUE),
                ])
                .add_application_property(RUN_MARKER_KEY.to_string(), run_marker.clone())
                .add_application_property(EVENT_LABEL_KEY.to_string(), "sequence-body".to_string())
                .build(),
            None,
        )?,
        "run {run_marker}: the sequence body message did not fit in the batch for partition \
         {PARTITION}"
    );
    producer.send_batch(batch, None).await?;

    let events =
        read_run_events_on_partition(&consumer, PARTITION, tail, &run_marker, 1, READ_DEADLINE)
            .await?;

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        events.len(),
        1,
        "run {run_marker}: partition {PARTITION} gave {} marked event(s) within {READ_DEADLINE:?} \
         after sequence number {tail}, and the test wanted 1",
        events.len()
    );
    match &events[0].raw_amqp_message().body {
        // From<Vec<AmqpValue>> for AmqpMessageBody wraps the values in one
        // AmqpList, so the sequence holds exactly one list.
        AmqpMessageBody::Sequence(lists) => {
            assert_eq!(
                lists.len(),
                1,
                "run {run_marker}: the sequence body must hold exactly one list from partition \
                 {PARTITION} within {READ_DEADLINE:?}, got {} list(s)",
                lists.len()
            );
            let list: &AmqpList = &lists[0];
            let values = &list.0;
            assert_eq!(
                values.len(),
                2,
                "run {run_marker}: the sequence list must hold the 2 values that the test sent to \
                 partition {PARTITION}, got {} within {READ_DEADLINE:?}",
                values.len()
            );
            assert_eq!(
                values[0],
                AmqpValue::String(SEQUENCE_TEXT.to_string()),
                "run {run_marker}: the first sequence element must arrive as the text that the \
                 test sent to partition {PARTITION} within {READ_DEADLINE:?}"
            );
            match &values[1] {
                AmqpValue::Int(value) => assert_eq!(
                    *value, SEQUENCE_INT_VALUE,
                    "run {run_marker}: the second sequence element arrived with the wrong integer \
                     from partition {PARTITION} within {READ_DEADLINE:?}"
                ),
                other => panic!(
                    "run {run_marker}: the second sequence element must arrive as an integer from \
                     partition {PARTITION} within {READ_DEADLINE:?}, got {other:?}"
                ),
            }
        }
        other => panic!(
            "run {run_marker}: the sequence body must arrive as AmqpMessageBody::Sequence from \
             partition {PARTITION} within {READ_DEADLINE:?}, got {other:?}"
        ),
    }
    Ok(())
}

/// A binary application property must arrive with the exact bytes.
#[recorded::test(live)]
async fn binary_application_property_round_trips(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const TEST_NAME: &str = "binary_application_property_round_trips";
    const PARTITION: &str = "0";

    let producer = open_producer(&ctx, TEST_NAME).await?;
    let consumer = open_consumer(&ctx, TEST_NAME).await?;
    let run_marker = format!("{TEST_NAME}-{}", azure_core::Uuid::new_v4());

    let tail = producer
        .get_partition_properties(PARTITION)
        .await?
        .last_enqueued_sequence_number;

    producer
        .send_event(
            EventData::builder()
                .with_body("binary-property")
                .add_property(RUN_MARKER_KEY.to_string(), run_marker.clone())
                .add_property(EVENT_LABEL_KEY.to_string(), "binary-property".to_string())
                .add_property(
                    BINARY_PROPERTY_KEY.to_string(),
                    AmqpSimpleValue::Binary(BINARY_PROPERTY_VALUE.to_vec()),
                )
                .build(),
            Some(SendEventOptions {
                partition_id: Some(PARTITION.to_string()),
            }),
        )
        .await?;

    let events =
        read_run_events_on_partition(&consumer, PARTITION, tail, &run_marker, 1, READ_DEADLINE)
            .await?;

    consumer.close().await?;
    producer.close().await?;

    assert_eq!(
        events.len(),
        1,
        "run {run_marker}: partition {PARTITION} gave {} marked event(s) within {READ_DEADLINE:?} \
         after sequence number {tail}, and the test wanted 1",
        events.len()
    );
    let properties = events[0].event_data().properties().unwrap_or_else(|| {
        panic!(
            "run {run_marker}: the event arrived with no application properties from partition \
             {PARTITION} within {READ_DEADLINE:?}"
        )
    });
    assert_eq!(
        properties.get(BINARY_PROPERTY_KEY),
        Some(&AmqpSimpleValue::Binary(BINARY_PROPERTY_VALUE.to_vec())),
        "run {run_marker}: the binary application property must arrive with the exact bytes from \
         partition {PARTITION} within {READ_DEADLINE:?}"
    );
    Ok(())
}

/// `tagged_label` must give a label only for an event of this run.
#[test]
fn tagged_label_matches_only_the_run_marker() {
    assert!(
        tagged_label(None, "run-1").is_none(),
        "an event with no application properties cannot belong to a run"
    );

    let mut matching = HashMap::new();
    matching.insert(
        RUN_MARKER_KEY.to_string(),
        AmqpSimpleValue::String("run-1".to_string()),
    );
    matching.insert(
        EVENT_LABEL_KEY.to_string(),
        AmqpSimpleValue::String("x".to_string()),
    );
    assert_eq!(
        tagged_label(Some(&matching), "run-1"),
        Some("x".to_string()),
        "the marker of this run with a string label must give the label"
    );

    let mut other_run = HashMap::new();
    other_run.insert(
        RUN_MARKER_KEY.to_string(),
        AmqpSimpleValue::String("run-2".to_string()),
    );
    other_run.insert(
        EVENT_LABEL_KEY.to_string(),
        AmqpSimpleValue::String("x".to_string()),
    );
    assert!(
        tagged_label(Some(&other_run), "run-1").is_none(),
        "the marker of another run must give no label"
    );

    let mut integer_marker = HashMap::new();
    integer_marker.insert(RUN_MARKER_KEY.to_string(), AmqpSimpleValue::Int(1));
    integer_marker.insert(
        EVENT_LABEL_KEY.to_string(),
        AmqpSimpleValue::String("x".to_string()),
    );
    assert!(
        tagged_label(Some(&integer_marker), "run-1").is_none(),
        "a marker that is not a string must give no label"
    );

    let mut no_label = HashMap::new();
    no_label.insert(
        RUN_MARKER_KEY.to_string(),
        AmqpSimpleValue::String("run-1".to_string()),
    );
    assert!(
        tagged_label(Some(&no_label), "run-1").is_none(),
        "the marker of this run with no label key must give no label"
    );

    let mut integer_label = HashMap::new();
    integer_label.insert(
        RUN_MARKER_KEY.to_string(),
        AmqpSimpleValue::String("run-1".to_string()),
    );
    integer_label.insert(EVENT_LABEL_KEY.to_string(), AmqpSimpleValue::Int(7));
    assert!(
        tagged_label(Some(&integer_label), "run-1").is_none(),
        "a label that is not a string must give no label"
    );
}

/// `large_body` must give the length that the caller asked for, and the fill
/// must vary.
#[test]
fn large_body_fills_a_non_constant_pattern() {
    assert_eq!(
        large_body(LARGE_BODY_LEN).len(),
        LARGE_BODY_LEN,
        "the helper must give the length that the caller asked for"
    );
    assert!(
        large_body(0).is_empty(),
        "a zero length request must give an empty body"
    );

    let body = large_body(512);
    assert_ne!(
        body[0], body[1],
        "the fill must vary so a zero filled buffer cannot pass by accident"
    );
    assert!(
        body.iter()
            .enumerate()
            .all(|(index, byte)| *byte == (index % 251) as u8),
        "every byte must follow the 251 byte pattern"
    );
    assert_ne!(
        body[0], body[256],
        "the fill period must not align with a power of two block"
    );
}
