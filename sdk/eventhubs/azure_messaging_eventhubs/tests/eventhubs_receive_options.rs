// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Live tests for the consumer receive options and the client identifiers.
//!
//! Each test reads the tail of its partition first, then sends events that
//! carry its own run marker. The assertions look only at events with that
//! marker. Traffic that another run writes to the same partition arrives after
//! the boundary and the marker filter drops it, so it never enters a result.

use azure_core_amqp::AmqpErrorKind;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind,
    models::{AmqpSimpleValue, EventData, ReceivedEventData},
    ConsumerClient, EventDataBatchOptions, EventHubsError, OpenReceiverOptions, ProducerClient,
    StartLocation, StartPosition,
};
use futures::{Stream, StreamExt};
use std::{env, error::Error, time::Duration};

/// Application property that names the run which sent an event.
const RUN_MARKER_KEY: &str = "receive-options-run";
/// Application property that names the position of an event inside its run.
const EVENT_LABEL_KEY: &str = "receive-options-label";

const EVENT_COUNT: usize = 10;
const READ_DEADLINE: Duration = Duration::from_secs(90);
const ATTACH_POKE: Duration = Duration::from_secs(2);

const IDLE_TIMEOUT: azure_core::time::Duration = azure_core::time::Duration::seconds(5);
const MIN_IDLE_ELAPSED: Duration = Duration::from_secs(4);
const MAX_IDLE_ELAPSED: Duration = Duration::from_secs(15);
const IDLE_DEADLINE: Duration = Duration::from_secs(120);
const END_POLL: Duration = Duration::from_secs(10);

const SLOW_TIMEOUT: azure_core::time::Duration = azure_core::time::Duration::seconds(10);
const SLOW_COUNT: usize = 6;
const SEND_INTERVAL: Duration = Duration::from_secs(3);
const SLOW_DEADLINE: Duration = Duration::from_secs(120);

/// The two clients a test needs, plus the marker that identifies its events.
struct Clients {
    producer: ProducerClient,
    consumer: ConsumerClient,
    run_marker: String,
}

/// Opens a producer and a consumer against the live Event Hub.
///
/// `test_name` becomes the application identifier on both clients. When
/// `instance_id` is set, the consumer also carries that instance identifier.
async fn open_clients(
    ctx: &TestContext,
    test_name: &str,
    instance_id: Option<&str>,
) -> Result<Clients, Box<dyn Error>> {
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = ctx.recording().credential();

    let producer = ProducerClient::builder()
        .with_application_id(test_name.to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let mut builder = ConsumerClient::builder().with_application_id(test_name.to_string());
    if let Some(instance_id) = instance_id {
        builder = builder.with_instance_id(instance_id.to_string());
    }
    let consumer = builder.open(host.as_str(), eventhub, credential).await?;

    Ok(Clients {
        producer,
        consumer,
        run_marker: format!("{test_name}-{}", azure_core::Uuid::new_v4()),
    })
}

/// Returns the sequence number of the last event in the partition.
///
/// The read goes through the consumer on purpose. It opens the consumer
/// connection, so the later link attach is short.
async fn boundary_sequence(
    consumer: &ConsumerClient,
    partition: &str,
) -> Result<i64, Box<dyn Error>> {
    Ok(consumer
        .get_partition_properties(partition)
        .await?
        .last_enqueued_sequence_number)
}

/// Sends one batch of labeled events to a partition.
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
            "the batch refused label {label} of run marker {run_marker} for partition {partition}"
        );
    }

    producer.send_batch(batch, None).await
}

/// Returns the label of an event, but only when the event belongs to this run.
fn tagged_label(event: &ReceivedEventData, run_marker: &str) -> Option<String> {
    let properties = event.event_data().properties()?;
    let AmqpSimpleValue::String(marker) = properties.get(RUN_MARKER_KEY)? else {
        return None;
    };
    if marker != run_marker {
        return None;
    }
    let AmqpSimpleValue::String(label) = properties.get(EVENT_LABEL_KEY)? else {
        return None;
    };
    Some(label.clone())
}

/// Collects the labels of this run from the stream, in arrival order.
///
/// The read stops at `want` labels or at `deadline`. A short read returns what
/// it has, and the caller asserts on the count.
async fn read_tagged<S>(
    stream: &mut S,
    run_marker: &str,
    want: usize,
    deadline: Duration,
) -> Vec<String>
where
    S: Stream<Item = azure_messaging_eventhubs::Result<ReceivedEventData>> + Unpin,
{
    let mut seen: Vec<String> = Vec::with_capacity(want);
    let _ = tokio::time::timeout(deadline, async {
        while seen.len() < want {
            match stream.next().await {
                Some(Ok(event)) => {
                    if let Some(label) = tagged_label(&event, run_marker) {
                        seen.push(label);
                    }
                }
                Some(Err(err)) => panic!(
                    "the stream failed for run marker {run_marker} after it read {seen:?}: {err:?}"
                ),
                None => break,
            }
        }
    })
    .await;
    seen
}

#[recorded::test(live)]
async fn prefetch_custom_delivers_every_event(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "1";
    const PREFETCH: u32 = 3;

    let clients = open_clients(&ctx, "prefetch_custom_delivers_every_event", None).await?;
    let boundary = boundary_sequence(&clients.consumer, PARTITION).await?;

    let labels: Vec<String> = (0..EVENT_COUNT)
        .map(|index| format!("event-{index}"))
        .collect();
    // Send before the attach, so the receiver drains a real backlog and a
    // prefetch smaller than the backlog must replenish its link credit.
    send_labeled_batch(&clients.producer, PARTITION, &clients.run_marker, &labels).await?;

    let receiver = clients
        .consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary),
                    inclusive: false,
                }),
                // A receive timeout ends the stream when it fires, which would
                // truncate this read, so leave it unset.
                ..Default::default()
            }),
        )
        .await?;

    let seen = {
        let mut stream = receiver.stream_events();
        read_tagged(&mut stream, &clients.run_marker, EVENT_COUNT, READ_DEADLINE).await
    };

    assert_eq!(
        seen.len(),
        EVENT_COUNT,
        "run marker {} on partition {PARTITION} with prefetch {PREFETCH} read {seen:?} in {READ_DEADLINE:?}",
        clients.run_marker
    );
    assert_eq!(
        seen, labels,
        "partition {PARTITION} delivered run marker {} out of order",
        clients.run_marker
    );

    receiver.close().await?;
    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn prefetch_one_delivers_every_event(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "1";
    // One is the smallest usable prefetch. Zero gives the link no credit and the
    // reader then waits forever.
    const PREFETCH: u32 = 1;

    let clients = open_clients(&ctx, "prefetch_one_delivers_every_event", None).await?;
    let boundary = boundary_sequence(&clients.consumer, PARTITION).await?;

    let labels: Vec<String> = (0..EVENT_COUNT)
        .map(|index| format!("event-{index}"))
        .collect();
    send_labeled_batch(&clients.producer, PARTITION, &clients.run_marker, &labels).await?;

    let receiver = clients
        .consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary),
                    inclusive: false,
                }),
                ..Default::default()
            }),
        )
        .await?;

    let seen = {
        let mut stream = receiver.stream_events();
        read_tagged(&mut stream, &clients.run_marker, EVENT_COUNT, READ_DEADLINE).await
    };

    assert_eq!(
        seen.len(),
        EVENT_COUNT,
        "run marker {} on partition {PARTITION} with prefetch {PREFETCH} read {seen:?} in {READ_DEADLINE:?}",
        clients.run_marker
    );
    assert_eq!(
        seen, labels,
        "partition {PARTITION} delivered run marker {} out of order",
        clients.run_marker
    );

    receiver.close().await?;
    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn receive_timeout_ends_stream_when_idle(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "3";

    let clients = open_clients(&ctx, "receive_timeout_ends_stream_when_idle", None).await?;
    let boundary = boundary_sequence(&clients.consumer, PARTITION).await?;

    // This test sends nothing, so the receiver starts at the tail and stays idle.
    let receiver = clients
        .consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary),
                    inclusive: false,
                }),
                receive_timeout: Some(IDLE_TIMEOUT),
                ..Default::default()
            }),
        )
        .await?;

    let (elapsed, err, ended) = {
        let mut stream = receiver.stream_events();

        // The receive timer starts only after the link attach, so start the
        // clock here and poke the stream to make the attach happen. A cancelled
        // poke does not cancel the receive, so the timer keeps running and the
        // measured window covers the attach as well.
        let mut mark = std::time::Instant::now();
        let _ = tokio::time::timeout(ATTACH_POKE, stream.next()).await;

        let measured = tokio::time::timeout(IDLE_DEADLINE, async {
            loop {
                match stream.next().await {
                    // A delivery restarts the receive timer, so restart the clock.
                    Some(Ok(_)) => mark = std::time::Instant::now(),
                    Some(Err(err)) => break (mark.elapsed(), err),
                    None => {
                        panic!("the stream ended with no error before the receive timeout fired")
                    }
                }
            }
        })
        .await;

        let (elapsed, err) = match measured {
            Ok(outcome) => outcome,
            Err(_) => panic!(
                "run marker {} saw no receive timeout on partition {PARTITION} in {IDLE_DEADLINE:?} with a receive timeout of {IDLE_TIMEOUT:?}, so the partition carried continuous foreign traffic",
                clients.run_marker
            ),
        };

        let ended = matches!(
            tokio::time::timeout(END_POLL, stream.next()).await,
            Ok(None)
        );
        (elapsed, err, ended)
    };

    let ErrorKind::AmqpError(amqp) = &err.kind else {
        panic!("the receive timeout gave {err:?}, which is not an AMQP error");
    };
    let AmqpErrorKind::AzureCore(core) = amqp.kind() else {
        panic!("the receive timeout gave {amqp:?}, which is not an Azure core error");
    };
    assert!(
        matches!(core.kind(), azure_core::error::ErrorKind::Io),
        "the receive timeout gave {core:?}, which is not an I/O error"
    );
    // The receiver wraps the cause in a redundant `Box::new` before it reaches
    // `azure_core::Error::new`, which boxes again, so the stored concrete type
    // is `Box<std::io::Error>` and not `std::io::Error`. Accept either, so this
    // test keeps passing once that extra box goes away.
    let io = core
        .downcast_ref::<std::io::Error>()
        .or_else(|| core.downcast_ref::<Box<std::io::Error>>().map(|b| &**b))
        .unwrap_or_else(|| panic!("the receive timeout error {core:?} has no I/O source"));
    assert_eq!(
        io.kind(),
        std::io::ErrorKind::TimedOut,
        "the receive timeout gave the I/O error kind {:?}",
        io.kind()
    );
    assert!(
        elapsed >= MIN_IDLE_ELAPSED,
        "the receive timeout of {IDLE_TIMEOUT:?} fired after only {elapsed:?} on partition {PARTITION}"
    );
    assert!(
        elapsed <= MAX_IDLE_ELAPSED,
        "the receive timeout of {IDLE_TIMEOUT:?} fired late, after {elapsed:?}, on partition {PARTITION}"
    );
    assert!(
        ended,
        "the stream did not end in {END_POLL:?} after it reported the receive timeout on partition {PARTITION}"
    );

    receiver.close().await?;
    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn receive_timeout_does_not_truncate_a_slow_stream(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "0";

    let clients = open_clients(
        &ctx,
        "receive_timeout_does_not_truncate_a_slow_stream",
        None,
    )
    .await?;
    let boundary = boundary_sequence(&clients.consumer, PARTITION).await?;

    let receiver = clients
        .consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary),
                    inclusive: false,
                }),
                receive_timeout: Some(SLOW_TIMEOUT),
                ..Default::default()
            }),
        )
        .await?;

    let labels: Vec<String> = (0..SLOW_COUNT)
        .map(|index| format!("slow-{index}"))
        .collect();

    // The whole send takes longer than the receive timeout, but no single gap
    // comes near it. The stream must therefore carry every event.
    let (send_result, seen) = {
        let mut stream = receiver.stream_events();
        let _ = tokio::time::timeout(ATTACH_POKE, stream.next()).await;

        let sender = async {
            for label in &labels {
                send_labeled_batch(
                    &clients.producer,
                    PARTITION,
                    &clients.run_marker,
                    std::slice::from_ref(label),
                )
                .await?;
                tokio::time::sleep(SEND_INTERVAL).await;
            }
            Ok::<(), EventHubsError>(())
        };
        let reader = read_tagged(&mut stream, &clients.run_marker, SLOW_COUNT, SLOW_DEADLINE);

        tokio::join!(sender, reader)
    };

    send_result?;
    assert_eq!(
        seen.len(),
        SLOW_COUNT,
        "run marker {} sent one event every {SEND_INTERVAL:?} on partition {PARTITION} and read {seen:?}; the receive timeout of {SLOW_TIMEOUT:?} must apply to each delivery, not to the whole read",
        clients.run_marker
    );
    assert_eq!(
        seen, labels,
        "partition {PARTITION} delivered run marker {} out of order",
        clients.run_marker
    );

    receiver.close().await?;
    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn consumer_instance_id_receives_events(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "2";

    // The instance identifier becomes the AMQP link name, so the unique suffix
    // keeps parallel runs from claiming the same link.
    let instance_id = format!("receive-options-instance-{}", azure_core::Uuid::new_v4());
    let clients = open_clients(
        &ctx,
        "consumer_instance_id_receives_events",
        Some(&instance_id),
    )
    .await?;
    let boundary = boundary_sequence(&clients.consumer, PARTITION).await?;

    let labels: Vec<String> = (0..EVENT_COUNT)
        .map(|index| format!("event-{index}"))
        .collect();
    send_labeled_batch(&clients.producer, PARTITION, &clients.run_marker, &labels).await?;

    let receiver = clients
        .consumer
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(boundary),
                    inclusive: false,
                }),
                ..Default::default()
            }),
        )
        .await?;

    let seen = {
        let mut stream = receiver.stream_events();
        read_tagged(&mut stream, &clients.run_marker, EVENT_COUNT, READ_DEADLINE).await
    };

    assert_eq!(
        seen.len(),
        EVENT_COUNT,
        "the consumer with instance id {instance_id} read {seen:?} on partition {PARTITION} in {READ_DEADLINE:?}"
    );
    assert_eq!(
        seen, labels,
        "the consumer with instance id {instance_id} read partition {PARTITION} out of order"
    );

    receiver.close().await?;
    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn producer_application_id_sends_events(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const PARTITION: &str = "2";
    const SEND_COUNT: usize = 3;

    // This test overlaps the existing producer coverage on purpose, so the
    // application identifier requirement maps to a named test. The identifier
    // goes into the AMQP connection properties and the client cannot read it
    // back, so a successful send is the only outcome a test can observe.
    let application_id = format!("receive-options-producer-{}", azure_core::Uuid::new_v4());
    let clients = open_clients(&ctx, &application_id, None).await?;

    let labels: Vec<String> = (0..SEND_COUNT)
        .map(|index| format!("produced-{index}"))
        .collect();
    let result =
        send_labeled_batch(&clients.producer, PARTITION, &clients.run_marker, &labels).await;

    assert!(
        result.is_ok(),
        "the producer with application id {application_id} failed to send {SEND_COUNT} events to partition {PARTITION}: {result:?}"
    );

    clients.consumer.close().await?;
    clients.producer.close().await?;

    Ok(())
}
