// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Live tests for Event Hubs owner level (epoch) arbitration.
//!
//! The broker gives the partition to the reader with the highest owner level
//! and detaches the readers below it with `amqp:link:stolen`.
//!
//! THE TRAP: the two competing readers must come from two separate
//! [`ConsumerClient`] instances. `RecoverableConnection` caches its receivers
//! by the source URL `{endpoint}/Partitions/{id}`, and the endpoint already
//! carries the consumer group. A second `open_receiver_on_partition` call on
//! the same client and the same partition gets the cached link back, so it
//! never attaches again and the broker never sees a rival. That failure is
//! silent: the test hangs to its timeout instead of failing loudly.

mod common;

use azure_core_amqp::AmqpErrorCondition;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind, models::ReceivedEventData, ConsumerClient, EventDataBatchOptions,
    EventHubsError, OpenReceiverOptions, ProducerClient, Result, StartLocation, StartPosition,
};
use futures::{Stream, StreamExt};
use std::{sync::LazyLock, time::Duration};
use tokio::time::timeout;

/// The consumer groups that `sdk/eventhubs/test-resources.bicep` declares.
const DEFAULT_GROUP: &str = "defaultGroup";
const OWNER_LEVEL_GROUP: &str = "ownerLevelGroup";

/// How long a reader gets to attach and deliver its first event.
const FIRST_EVENT_TIMEOUT: Duration = Duration::from_secs(30);
/// How long the broker gets to detach the losing reader.
const DISPLACEMENT_TIMEOUT: Duration = Duration::from_secs(60);
/// How long a reader that must survive gets to prove that it is still alive.
const SURVIVOR_TIMEOUT: Duration = Duration::from_secs(30);

const EVENT_COUNT: usize = 50;
const WAVE_COUNT: usize = 10;
/// A small prefetch keeps the loser from buffering the whole partition, so the
/// detach surfaces after a few events instead of after the full batch.
const PREFETCH: u32 = 5;

/// The six scenarios need seven partition slots on a four-partition hub, and
/// cargo runs the tests in one file on parallel threads. Every test takes this
/// lock and holds it for the whole body, so no two scenarios share a partition.
static PARTITION_LOCK: LazyLock<async_lock::Mutex<()>> =
    LazyLock::new(|| async_lock::Mutex::new(()));

/// Reads the partition tail, then sends `count` events in one batch.
///
/// The returned sequence number is the tail before the send, so a reader that
/// starts there reads exactly the events of this test. `StartLocation::Earliest`
/// would not do: the shared hub keeps 24 hours of unrelated traffic.
async fn publish(
    producer: &ProducerClient,
    partition_id: &str,
    marker: &str,
    count: usize,
) -> Result<i64> {
    let start_sequence = producer
        .get_partition_properties(partition_id)
        .await?
        .last_enqueued_sequence_number;

    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some(partition_id.to_string()),
            ..Default::default()
        }))
        .await?;
    for i in 0..count {
        assert!(
            batch.try_add_event_data(format!("{marker}-{i}"), None)?,
            "{marker}: the batch rejected event {i}"
        );
    }
    producer.send_batch(batch, None).await?;

    Ok(start_sequence)
}

/// The one place that enforces the strict displacement assertion.
///
/// `ConsumerDisconnected(None)` is not accepted here. The local backstop in
/// `EventReceiver::request_close` produces that form with no broker involved,
/// so a bare `ConsumerDisconnected(_)` match cannot tell a real steal from a
/// local close.
fn assert_link_stolen(err: &EventHubsError, context: &str) {
    let ErrorKind::ConsumerDisconnected(Some(described)) = &err.kind else {
        panic!(
            "{context}: expected ConsumerDisconnected(Some(_)), got {:?}",
            err.kind
        )
    };
    assert_eq!(
        described.condition,
        AmqpErrorCondition::LinkStolen,
        "{context}: expected LinkStolen, got {:?}",
        described.condition
    );
}

/// Consumes exactly one event.
///
/// This is the only proof that a link attached. `open_receiver_on_partition`
/// returns without touching the wire, and the attach happens on the first poll
/// of `stream_events()`.
async fn consume_one<S>(stream: &mut S, limit: Duration, context: &str)
where
    S: Stream<Item = Result<ReceivedEventData>> + Unpin,
{
    let item = timeout(limit, stream.next())
        .await
        .unwrap_or_else(|_| panic!("{context}: no event arrived within {limit:?}"))
        .unwrap_or_else(|| panic!("{context}: the stream ended before an event arrived"));
    if let Err(err) = item {
        panic!("{context}: expected an event, got an error: {err:?}");
    }
}

/// Polls a stream and returns its first error.
///
/// The `Ok` items are discarded on purpose: with a prefetch of five the loser
/// holds up to five delivered events, and it must chew through them before the
/// detach surfaces.
async fn drain_until_error<S>(stream: &mut S, limit: Duration, context: &str) -> EventHubsError
where
    S: Stream<Item = Result<ReceivedEventData>> + Unpin,
{
    let drained = timeout(limit, async {
        while let Some(item) = stream.next().await {
            if let Err(err) = item {
                return Some(err);
            }
        }
        None
    })
    .await;

    match drained {
        Ok(Some(err)) => err,
        Ok(None) => panic!("{context}: the stream ended with no error"),
        Err(_) => panic!("{context}: no error arrived within {limit:?}"),
    }
}

/// A reader at owner level 1 displaces a reader that asked for no owner level.
///
/// The thief takes owner level 1 rather than 0. Whether owner level 0 displaces
/// a reader that sent no epoch at all is unsettled, and this test does not
/// depend on the answer.
#[recorded::test(live)]
async fn exclusive_receiver_displaces_non_exclusive_receiver(ctx: TestContext) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "exclusive_receiver_displaces_non_exclusive_receiver";
    const PARTITION: &str = "0";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let start_sequence = publish(&producer, PARTITION, CONTEXT, EVENT_COUNT).await?;

    let loser_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let loser = loser_client
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: None,
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the thief. See THE TRAP in the module docs.
    let thief_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let stolen = {
        let mut loser_stream = loser.stream_events();
        consume_one(&mut loser_stream, FIRST_EVENT_TIMEOUT, "loser attach").await;

        let thief = thief_client
            .open_receiver_on_partition(
                PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(1),
                    prefetch: Some(PREFETCH),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(start_sequence),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut thief_stream = thief.stream_events();
            consume_one(&mut thief_stream, FIRST_EVENT_TIMEOUT, "thief attach").await;
            drain_until_error(&mut loser_stream, DISPLACEMENT_TIMEOUT, CONTEXT).await
        };
        thief.close().await?;
        observed
    };

    loser.close().await?;
    loser_client.close().await?;
    thief_client.close().await?;
    producer.close().await?;

    assert_link_stolen(&stolen, CONTEXT);
    Ok(())
}

/// A reader at owner level 2 displaces a reader at owner level 1.
#[recorded::test(live)]
async fn higher_owner_level_displaces_lower(ctx: TestContext) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "higher_owner_level_displaces_lower";
    const PARTITION: &str = "1";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let start_sequence = publish(&producer, PARTITION, CONTEXT, EVENT_COUNT).await?;

    let loser_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let loser = loser_client
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: Some(1),
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the thief. See THE TRAP in the module docs.
    let thief_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let stolen = {
        let mut loser_stream = loser.stream_events();
        consume_one(&mut loser_stream, FIRST_EVENT_TIMEOUT, "loser attach").await;

        let thief = thief_client
            .open_receiver_on_partition(
                PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(2),
                    prefetch: Some(PREFETCH),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(start_sequence),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut thief_stream = thief.stream_events();
            consume_one(&mut thief_stream, FIRST_EVENT_TIMEOUT, "thief attach").await;
            drain_until_error(&mut loser_stream, DISPLACEMENT_TIMEOUT, CONTEXT).await
        };
        thief.close().await?;
        observed
    };

    loser.close().await?;
    loser_client.close().await?;
    thief_client.close().await?;
    producer.close().await?;

    assert_link_stolen(&stolen, CONTEXT);
    Ok(())
}

/// A reader at owner level 1 cannot take a partition that a reader at owner
/// level 2 holds. The broker refuses the newcomer, and the incumbent keeps the
/// partition.
///
/// This is the only test whose error arrives on the attach path rather than on
/// the receive path.
#[recorded::test(live)]
async fn lower_owner_level_cannot_read_while_higher_is_active(ctx: TestContext) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "lower_owner_level_cannot_read_while_higher_is_active";
    const PARTITION: &str = "2";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let start_sequence = publish(&producer, PARTITION, CONTEXT, EVENT_COUNT).await?;

    let incumbent_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let incumbent = incumbent_client
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: Some(2),
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the newcomer. See THE TRAP in the module docs.
    let newcomer_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let wave_2_marker = format!("{CONTEXT}-wave-2-{start_sequence}");
    let wave_2_body = format!("{wave_2_marker}-0");

    let (refusal, incumbent_read_marker, incumbent_error) = {
        let mut incumbent_stream = incumbent.stream_events();
        consume_one(
            &mut incumbent_stream,
            FIRST_EVENT_TIMEOUT,
            "incumbent attach",
        )
        .await;

        let newcomer = newcomer_client
            .open_receiver_on_partition(
                PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(1),
                    prefetch: Some(PREFETCH),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(start_sequence),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut newcomer_stream = newcomer.stream_events();
            let first = timeout(DISPLACEMENT_TIMEOUT, newcomer_stream.next())
                .await
                .unwrap_or_else(|_| {
                    panic!("{CONTEXT}: the refused reader yielded nothing within {DISPLACEMENT_TIMEOUT:?}")
                })
                .unwrap_or_else(|| {
                    panic!("{CONTEXT}: the refused reader's stream ended with no item")
                });
            match first {
                Ok(_) => {
                    panic!("expected the refused reader's first item to be an error, got an event")
                }
                Err(err) => err,
            }
        };
        newcomer.close().await?;

        // Wave 2 goes out only after the refusal. The incumbent still holds up
        // to `PREFETCH` events in its local buffer, and those drain from memory
        // with no live link, so one more `Ok` would prove nothing. An event
        // published after the refusal cannot have been prefetched before it.
        publish(&producer, PARTITION, &wave_2_marker, 1).await?;

        let mut error = None;
        let mut read_marker = false;
        let _ = timeout(SURVIVOR_TIMEOUT, async {
            while let Some(item) = incumbent_stream.next().await {
                match item {
                    Ok(event) => {
                        if event.event_data().body() == Some(wave_2_body.as_bytes()) {
                            read_marker = true;
                            return;
                        }
                    }
                    Err(err) => {
                        error = Some(err);
                        return;
                    }
                }
            }
        })
        .await;
        (observed, read_marker, error)
    };

    incumbent.close().await?;
    incumbent_client.close().await?;
    newcomer_client.close().await?;
    producer.close().await?;

    assert_link_stolen(&refusal, CONTEXT);
    assert!(
        incumbent_read_marker,
        "{CONTEXT}: the incumbent did not read the wave 2 marker within {SURVIVOR_TIMEOUT:?} after the refusal, so its link did not survive; error: {incumbent_error:?}"
    );
    Ok(())
}

/// Two readers at the same owner level arbitrate by arrival: the last one in
/// wins, and the incumbent is displaced.
///
/// This test goes beyond the issue's four scenarios. `EventProcessor` opens
/// every partition receiver at `owner_level: Some(0)`, so equal-level
/// arbitration is the rule its steal detection depends on; drop this test only
/// if you also accept that that rule is untested.
#[recorded::test(live)]
async fn equal_owner_level_displaces_incumbent(ctx: TestContext) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "equal_owner_level_displaces_incumbent";
    const PARTITION: &str = "3";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let start_sequence = publish(&producer, PARTITION, CONTEXT, EVENT_COUNT).await?;

    let loser_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let loser = loser_client
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: Some(2),
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the thief. See THE TRAP in the module docs.
    let thief_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let stolen = {
        let mut loser_stream = loser.stream_events();
        consume_one(&mut loser_stream, FIRST_EVENT_TIMEOUT, "loser attach").await;

        let thief = thief_client
            .open_receiver_on_partition(
                PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(2),
                    prefetch: Some(PREFETCH),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(start_sequence),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut thief_stream = thief.stream_events();
            consume_one(&mut thief_stream, FIRST_EVENT_TIMEOUT, "thief attach").await;
            drain_until_error(&mut loser_stream, DISPLACEMENT_TIMEOUT, CONTEXT).await
        };
        thief.close().await?;
        observed
    };

    loser.close().await?;
    loser_client.close().await?;
    thief_client.close().await?;
    producer.close().await?;

    assert_link_stolen(&stolen, CONTEXT);
    Ok(())
}

/// An owner level applies to one partition only. A thief on partition 0 leaves
/// a reader on partition 1 alone.
#[recorded::test(live)]
async fn exclusive_receiver_does_not_displace_reader_on_another_partition(
    ctx: TestContext,
) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "exclusive_receiver_does_not_displace_reader_on_another_partition";
    const SURVIVOR_PARTITION: &str = "1";
    const THIEF_PARTITION: &str = "0";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    // Wave 1 goes to both partitions before any reader opens.
    let survivor_start = publish(&producer, SURVIVOR_PARTITION, CONTEXT, WAVE_COUNT).await?;
    let thief_start = publish(&producer, THIEF_PARTITION, CONTEXT, WAVE_COUNT).await?;

    let survivor_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let survivor = survivor_client
        .open_receiver_on_partition(
            SURVIVOR_PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: None,
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(survivor_start),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the thief. See THE TRAP in the module docs.
    let thief_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let wave_2_marker = format!("{CONTEXT}-wave-2-{survivor_start}");
    let wave_2_body = format!("{wave_2_marker}-0");

    let (observed_error, found_marker) = {
        let mut survivor_stream = survivor.stream_events();
        consume_one(&mut survivor_stream, FIRST_EVENT_TIMEOUT, "survivor attach").await;

        let thief = thief_client
            .open_receiver_on_partition(
                THIEF_PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(1),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(thief_start),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut thief_stream = thief.stream_events();
            consume_one(&mut thief_stream, FIRST_EVENT_TIMEOUT, "thief attach").await;

            // Wave 2 goes out only after the thief holds its partition. An event
            // that the survivor reads after that moment proves its link is alive,
            // where silence alone proves nothing.
            publish(&producer, SURVIVOR_PARTITION, &wave_2_marker, 1).await?;

            let mut error = None;
            let mut found = false;
            let _ = timeout(SURVIVOR_TIMEOUT, async {
                while let Some(item) = survivor_stream.next().await {
                    match item {
                        Ok(event) => {
                            if event.event_data().body() == Some(wave_2_body.as_bytes()) {
                                found = true;
                                return;
                            }
                        }
                        Err(err) => {
                            error = Some(err);
                            return;
                        }
                    }
                }
            })
            .await;
            (error, found)
        };
        thief.close().await?;
        observed
    };

    survivor.close().await?;
    survivor_client.close().await?;
    thief_client.close().await?;
    producer.close().await?;

    // The bare `_` is deliberate here. The survivor must not be disconnected for
    // any reason at all, so the wider match is the stronger claim in this
    // direction. The displacement tests above use the strict form instead.
    if let Some(err) = &observed_error {
        assert!(
            !matches!(err.kind, ErrorKind::ConsumerDisconnected(_)),
            "{CONTEXT}: the survivor on partition {SURVIVOR_PARTITION} was disconnected: {err:?}"
        );
    }
    assert!(
        found_marker,
        "{CONTEXT}: the survivor did not read the wave 2 marker within {SURVIVOR_TIMEOUT:?}, error: {observed_error:?}"
    );
    Ok(())
}

/// An owner level applies to one consumer group only. A thief in
/// `ownerLevelGroup` leaves a reader on the same partition in `defaultGroup`
/// alone.
#[recorded::test(live)]
async fn exclusive_receiver_does_not_displace_reader_in_another_consumer_group(
    ctx: TestContext,
) -> Result<()> {
    common::setup();
    let _partition_guard = PARTITION_LOCK.lock().await;

    const CONTEXT: &str = "exclusive_receiver_does_not_displace_reader_in_another_consumer_group";
    const PARTITION: &str = "2";

    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    let producer = ProducerClient::builder()
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    // Wave 1 goes out before any reader opens. Both consumer groups see it.
    let start_sequence = publish(&producer, PARTITION, CONTEXT, WAVE_COUNT).await?;

    let survivor_client = ConsumerClient::builder()
        .with_consumer_group(DEFAULT_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;
    let survivor = survivor_client
        .open_receiver_on_partition(
            PARTITION.to_string(),
            Some(OpenReceiverOptions {
                owner_level: None,
                prefetch: Some(PREFETCH),
                start_position: Some(StartPosition {
                    location: StartLocation::SequenceNumber(start_sequence),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // A separate client for the thief, in the other consumer group.
    let thief_client = ConsumerClient::builder()
        .with_consumer_group(OWNER_LEVEL_GROUP.to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await?;

    let wave_2_marker = format!("{CONTEXT}-wave-2-{start_sequence}");
    let wave_2_body = format!("{wave_2_marker}-0");

    let (observed_error, found_marker) = {
        let mut survivor_stream = survivor.stream_events();
        consume_one(&mut survivor_stream, FIRST_EVENT_TIMEOUT, "survivor attach").await;

        let thief = thief_client
            .open_receiver_on_partition(
                PARTITION.to_string(),
                Some(OpenReceiverOptions {
                    owner_level: Some(1),
                    start_position: Some(StartPosition {
                        location: StartLocation::SequenceNumber(start_sequence),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            )
            .await?;

        let observed = {
            let mut thief_stream = thief.stream_events();
            consume_one(&mut thief_stream, FIRST_EVENT_TIMEOUT, "thief attach").await;

            // Wave 2 goes out only after the thief holds its group's partition.
            publish(&producer, PARTITION, &wave_2_marker, 1).await?;

            let mut error = None;
            let mut found = false;
            let _ = timeout(SURVIVOR_TIMEOUT, async {
                while let Some(item) = survivor_stream.next().await {
                    match item {
                        Ok(event) => {
                            if event.event_data().body() == Some(wave_2_body.as_bytes()) {
                                found = true;
                                return;
                            }
                        }
                        Err(err) => {
                            error = Some(err);
                            return;
                        }
                    }
                }
            })
            .await;
            (error, found)
        };
        thief.close().await?;
        observed
    };

    survivor.close().await?;
    survivor_client.close().await?;
    thief_client.close().await?;
    producer.close().await?;

    // The bare `_` is deliberate here, for the same reason as the test above.
    if let Some(err) = &observed_error {
        assert!(
            !matches!(err.kind, ErrorKind::ConsumerDisconnected(_)),
            "{CONTEXT}: the survivor in {DEFAULT_GROUP} was disconnected: {err:?}"
        );
    }
    assert!(
        found_marker,
        "{CONTEXT}: the survivor did not read the wave 2 marker within {SURVIVOR_TIMEOUT:?}, error: {observed_error:?}"
    );
    Ok(())
}
