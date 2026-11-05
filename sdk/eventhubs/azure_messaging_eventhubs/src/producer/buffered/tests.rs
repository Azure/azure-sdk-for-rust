// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore backpressure retryable

use super::{
    send_client::mock::{MockSendClient, SendScript},
    *,
};
use crate::models::EventData;
use azure_core::time::Duration;
use futures::{pin_mut, poll, StreamExt};
use std::collections::HashSet;

/// One delivery report that a test collected from a handler.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Report {
    Succeeded {
        partition_id: String,
        bodies: Vec<String>,
    },
    Failed {
        partition_id: String,
        bodies: Vec<String>,
        error: String,
    },
}

impl Report {
    fn partition_id(&self) -> &str {
        match self {
            Report::Succeeded { partition_id, .. } | Report::Failed { partition_id, .. } => {
                partition_id
            }
        }
    }

    fn bodies(&self) -> &[String] {
        match self {
            Report::Succeeded { bodies, .. } | Report::Failed { bodies, .. } => bodies,
        }
    }

    fn is_success(&self) -> bool {
        matches!(self, Report::Succeeded { .. })
    }
}

fn bodies_of(events: &[EventData]) -> Vec<String> {
    events
        .iter()
        .map(|event| String::from_utf8_lossy(event.body().unwrap_or_default()).into_owned())
        .collect()
}

/// The settings that a test needs from the client.
struct Config {
    max_wait_time: Duration,
    max_buffered: usize,
    max_message_size: u64,
    with_success_handler: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // Long enough that only an explicit trigger sends a batch. A test
            // that wants the timer sets a short time itself.
            max_wait_time: Duration::seconds(30),
            max_buffered: 64,
            max_message_size: 1024 * 1024,
            with_success_handler: true,
        }
    }
}

struct Harness {
    client: Arc<BufferedProducerClient>,
    mock: Arc<MockSendClient>,
    reports: mpsc::UnboundedReceiver<Report>,
    started: mpsc::UnboundedReceiver<String>,
}

impl Harness {
    /// Waits for the next delivery report.
    async fn next_report(&mut self) -> Report {
        self.reports
            .next()
            .await
            .expect("a delivery report was expected")
    }

    /// Waits for the next `count` delivery reports.
    async fn next_reports(&mut self, count: usize) -> Vec<Report> {
        let mut reports = Vec::with_capacity(count);
        for _ in 0..count {
            reports.push(self.next_report().await);
        }
        reports
    }

    /// Waits until the mock starts a send on any partition.
    async fn next_started(&mut self) -> String {
        self.started.next().await.expect("a send was expected")
    }
}

async fn harness(partitions: &[&str], config: Config) -> Harness {
    let (mock, started) =
        MockSendClient::with_max_message_size(partitions, config.max_message_size);
    let (report_tx, reports) = mpsc::unbounded();

    let mut builder = BufferedProducerClient::builder()
        .with_max_wait_time(config.max_wait_time)
        .with_max_buffered_event_count_per_partition(config.max_buffered);

    if config.with_success_handler {
        let tx = report_tx.clone();
        builder = builder.with_on_send_succeeded(move |context| {
            let tx = tx.clone();
            async move {
                let _ = tx.unbounded_send(Report::Succeeded {
                    partition_id: context.partition_id,
                    bodies: bodies_of(&context.events),
                });
            }
        });
    }

    let tx = report_tx;
    let client = builder
        .with_on_send_failed(move |context| {
            let tx = tx.clone();
            async move {
                let _ = tx.unbounded_send(Report::Failed {
                    partition_id: context.partition_id,
                    bodies: bodies_of(&context.events),
                    error: context.error.to_string(),
                });
            }
        })
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    Harness {
        client: Arc::new(client),
        mock,
        reports,
        started,
    }
}

fn to_partition(partition_id: &str) -> Option<EnqueueEventOptions> {
    Some(EnqueueEventOptions {
        partition_id: Some(partition_id.to_string()),
        ..Default::default()
    })
}

// 1. A full batch sends immediately.
#[tokio::test]
async fn full_batch_sends_without_waiting_for_the_timer() {
    // The wait time is 30 seconds, so only a full batch can trigger this send.
    let mut h = harness(
        &["0"],
        Config {
            max_buffered: 2,
            ..Default::default()
        },
    )
    .await;

    h.client
        .enqueue_events(vec!["a", "b"], to_partition("0"))
        .await
        .unwrap();

    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), ["a", "b"]);
    h.client.close().await.unwrap();
}

// 1b. A batch also sends when the next event does not fit the maximum message size.
#[tokio::test]
async fn batch_sends_when_the_next_event_does_not_fit() {
    // One 1000 byte event fits. Two do not.
    let mut h = harness(
        &["0"],
        Config {
            max_message_size: 2500,
            ..Default::default()
        },
    )
    .await;

    let first = "a".repeat(1000);
    let second = "b".repeat(1000);
    h.client
        .enqueue_event(first.clone(), to_partition("0"))
        .await
        .unwrap();
    h.client
        .enqueue_event(second.clone(), to_partition("0"))
        .await
        .unwrap();

    // The second event does not fit, so the client sends the first on its own.
    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), [first]);

    h.client.flush().await.unwrap();
    let report = h.next_report().await;
    assert_eq!(report.bodies(), [second]);

    assert_eq!(h.mock.sends().len(), 2);
    h.client.close().await.unwrap();
}

// 2. A partial batch sends after the maximum wait time.
#[tokio::test]
async fn partial_batch_sends_after_the_maximum_wait_time() {
    let mut h = harness(
        &["0"],
        Config {
            max_wait_time: Duration::milliseconds(50),
            ..Default::default()
        },
    )
    .await;

    h.client
        .enqueue_event("only", to_partition("0"))
        .await
        .unwrap();

    // No flush and no close. Only the timer can send this batch.
    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), ["only"]);
    h.client.close().await.unwrap();
}

// 3. An explicit partition ID routes the event to that partition.
#[tokio::test]
async fn explicit_partition_id_routes_the_event() {
    let mut h = harness(&["0", "1", "2"], Config::default()).await;

    h.client
        .enqueue_event("routed", to_partition("2"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    assert_eq!(report.partition_id(), "2");
    assert_eq!(h.mock.sends()[0].partition_id, "2");
    h.client.close().await.unwrap();
}

// 3b. An unknown partition ID is an error.
#[tokio::test]
async fn unknown_partition_id_is_rejected() {
    let h = harness(&["0", "1"], Config::default()).await;

    let error = h
        .client
        .enqueue_event("nowhere", to_partition("9"))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("no partition"));
    h.client.close().await.unwrap();
}

// 4. A partition key routes every event with that key to one partition.
#[tokio::test]
async fn partition_key_routes_every_event_to_one_partition() {
    let mut h = harness(&["0", "1", "2", "3"], Config::default()).await;

    let options = Some(EnqueueEventOptions {
        partition_key: Some("customer-17".to_string()),
        ..Default::default()
    });
    h.client
        .enqueue_events(vec!["a", "b", "c"], options)
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    assert_eq!(report.bodies(), ["a", "b", "c"]);

    // The resolver decides the partition, and every event went to that one.
    let expected = h.client.resolver.assign_for_key("customer-17");
    assert_eq!(report.partition_id(), expected);
    h.client.close().await.unwrap();
}

// 5. Automatic assignment spreads events over the partitions in round-robin order.
#[tokio::test]
async fn automatic_assignment_uses_round_robin() {
    let h = harness(
        &["0", "1", "2", "3"],
        Config {
            // One event for each batch, so each event is its own send.
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    for index in 0..8 {
        h.client
            .enqueue_event(format!("e{index}"), None)
            .await
            .unwrap();
    }
    h.client.close().await.unwrap();

    let sends = h.mock.sends();
    assert_eq!(sends.len(), 8);
    for partition_id in ["0", "1", "2", "3"] {
        let count = sends
            .iter()
            .filter(|s| s.partition_id == partition_id)
            .count();
        assert_eq!(count, 2, "partition {partition_id} did not get two events");
    }
}

// 6. A request that sets a partition ID and a partition key is rejected.
#[tokio::test]
async fn conflicting_routing_options_are_rejected() {
    let h = harness(&["0", "1"], Config::default()).await;

    let options = Some(EnqueueEventOptions {
        partition_id: Some("0".to_string()),
        partition_key: Some("a-key".to_string()),
    });

    let error = h
        .client
        .enqueue_event("conflict", options.clone())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("not both"));

    let error = h
        .client
        .enqueue_events(vec!["conflict"], options)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("not both"));

    assert_eq!(h.client.total_buffered_event_count(), 0);
    h.client.close().await.unwrap();
}

// 7. Events keep the enqueue order inside one partition.
#[tokio::test]
async fn events_keep_their_order_inside_a_partition() {
    let mut h = harness(&["0"], Config::default()).await;

    for index in 0..10 {
        h.client
            .enqueue_event(format!("e{index}"), to_partition("0"))
            .await
            .unwrap();
    }
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    let expected: Vec<String> = (0..10).map(|index| format!("e{index}")).collect();
    assert_eq!(report.bodies(), expected.as_slice());
    h.client.close().await.unwrap();
}

// 8. Two partitions publish at the same time.
#[tokio::test]
async fn different_partitions_publish_concurrently() {
    let mut h = harness(
        &["0", "1"],
        Config {
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    // Hold the first send on each partition.
    let release_zero = h.mock.gate("0");
    let release_one = h.mock.gate("1");

    h.client
        .enqueue_event("a", to_partition("0"))
        .await
        .unwrap();
    h.client
        .enqueue_event("b", to_partition("1"))
        .await
        .unwrap();

    // Both sends start while both are held, so one partition does not block the
    // other.
    let first = h.next_started().await;
    let second = h.next_started().await;
    let started: HashSet<String> = [first, second].into_iter().collect();
    assert_eq!(
        started.len(),
        2,
        "both partitions should have a send in flight"
    );

    let _ = release_zero.send(());
    let _ = release_one.send(());

    let reports = h.next_reports(2).await;
    assert!(reports.iter().all(|r| r.is_success()));
    h.client.close().await.unwrap();
}

// 9. A full buffer makes an enqueue wait.
#[tokio::test]
async fn a_full_buffer_applies_backpressure() {
    let mut h = harness(
        &["0"],
        Config {
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    // Hold the worker inside its first send, so it stops draining the queue.
    let _release = h.mock.gate("0");
    h.client
        .enqueue_event("first", to_partition("0"))
        .await
        .unwrap();
    let _ = h.next_started().await;

    // The queue now fills, and the enqueue cannot finish.
    let pending = h
        .client
        .enqueue_events(vec!["a", "b", "c", "d", "e", "f"], to_partition("0"));
    pin_mut!(pending);
    assert!(
        poll!(pending.as_mut()).is_pending(),
        "the enqueue should wait for space in a full buffer"
    );

    // A close must not wait for the enqueue that is still parked.
    h.client.abort().await.unwrap();
}

// 10. A waiting enqueue continues once the buffer has space.
#[tokio::test]
async fn a_waiting_enqueue_resumes_when_space_appears() {
    let mut h = harness(
        &["0"],
        Config {
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    let release = h.mock.gate("0");
    h.client
        .enqueue_event("first", to_partition("0"))
        .await
        .unwrap();
    let _ = h.next_started().await;

    let pending = h
        .client
        .enqueue_events(vec!["a", "b", "c", "d", "e", "f"], to_partition("0"));
    pin_mut!(pending);
    assert!(poll!(pending.as_mut()).is_pending());

    // Letting the send finish drains the queue, so the enqueue continues.
    let _ = release.send(());
    pending.await.unwrap();

    h.client.close().await.unwrap();
    assert_eq!(h.mock.total_events(), 7);
    assert_eq!(h.client.total_buffered_event_count(), 0);
}

// 11. A waiting enqueue fails once the client closes.
#[tokio::test]
async fn a_waiting_enqueue_fails_when_the_client_closes() {
    let mut h = harness(
        &["0"],
        Config {
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    let _release = h.mock.gate("0");
    h.client
        .enqueue_event("first", to_partition("0"))
        .await
        .unwrap();
    let _ = h.next_started().await;

    let client = h.client.clone();
    let pending = client.enqueue_events(vec!["a", "b", "c", "d", "e", "f"], to_partition("0"));
    pin_mut!(pending);
    assert!(poll!(pending.as_mut()).is_pending());

    // The close signal must wake the waiting enqueue with an error.
    let closing = h.client.abort();
    pin_mut!(closing);
    let _ = poll!(closing.as_mut());

    let error = pending.await.unwrap_err();
    assert!(error.to_string().contains("closed"));

    closing.await.unwrap();
}

// 12. One event that is larger than an empty batch fails on its own.
#[tokio::test]
async fn an_oversized_event_produces_one_failure() {
    let mut h = harness(
        &["0"],
        Config {
            max_message_size: 200,
            ..Default::default()
        },
    )
    .await;

    let big = "x".repeat(4096);
    h.client
        .enqueue_event(big, to_partition("0"))
        .await
        .unwrap();

    let report = h.next_report().await;
    assert!(!report.is_success());
    assert_eq!(report.bodies().len(), 1);
    assert!(report.bodies()[0].starts_with("xxxx"));

    // The worker stays healthy, so a later event still goes out.
    h.client
        .enqueue_event("small", to_partition("0"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();
    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), ["small"]);

    assert_eq!(h.client.total_buffered_event_count(), 0);
    h.client.close().await.unwrap();
}

// 13. A retryable failure reaches the client only after the retry policy runs.
//
// The retry policy sits below the send seam, inside `RecoverableSender`. The
// tests for the policy itself are in `common/retry.rs`, and the live
// forced-error test covers recovery from end to end. This test states the
// contract that the worker depends on: one error from the seam is already
// terminal, so the worker reports one failure and does not try again.
#[tokio::test]
async fn a_terminal_error_is_not_retried_by_the_worker() {
    let mut h = harness(&["0"], Config::default()).await;

    h.mock.push_outcome("0", SendScript::Error("server busy"));

    h.client
        .enqueue_event("a", to_partition("0"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    assert!(!report.is_success());

    // Exactly one send attempt reached the seam.
    assert_eq!(h.mock.sends().len(), 1);
    h.client.close().await.unwrap();
}

// 14. An exhausted retry produces exactly one failure result.
#[tokio::test]
async fn retry_exhaustion_produces_one_failure_result() {
    let mut h = harness(&["0"], Config::default()).await;

    h.mock
        .push_outcome("0", SendScript::Error("retries exhausted"));

    h.client
        .enqueue_events(vec!["a", "b", "c"], to_partition("0"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    match report {
        Report::Failed { bodies, error, .. } => {
            assert_eq!(bodies, ["a", "b", "c"]);
            assert!(error.contains("retries exhausted"));
        }
        other => panic!("expected a failure report, got {other:?}"),
    }

    // The client reports the batch one time only.
    h.client.close().await.unwrap();
    assert!(h.reports.next().await.is_none());
}

// 15. A batch that the service accepts produces one success result.
#[tokio::test]
async fn a_successful_batch_produces_one_success_result() {
    let mut h = harness(&["0"], Config::default()).await;

    h.client
        .enqueue_events(vec!["a", "b"], to_partition("0"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), ["a", "b"]);

    h.client.close().await.unwrap();
    assert!(h.reports.next().await.is_none());
}

// 16. Modified, Released, and Rejected are never a success.
#[tokio::test]
async fn modified_released_and_rejected_are_not_reported_as_success() {
    for outcome in [
        SendScript::Modified,
        SendScript::Released,
        SendScript::Rejected,
    ] {
        let mut h = harness(&["0"], Config::default()).await;
        h.mock.push_outcome("0", outcome.clone());

        h.client
            .enqueue_event("a", to_partition("0"))
            .await
            .unwrap();
        h.client.flush().await.unwrap();

        let report = h.next_report().await;
        assert!(
            !report.is_success(),
            "outcome {outcome:?} must not be a success"
        );
        assert_eq!(report.bodies(), ["a"]);

        h.client.close().await.unwrap();
    }
}

// 16b. A Modified or Released outcome carries the SendNotAccepted error kind.
#[tokio::test]
async fn a_not_accepted_outcome_uses_the_send_not_accepted_error() {
    let mut h = harness(&["0"], Config::default()).await;
    h.mock.push_outcome("0", SendScript::Modified);

    h.client
        .enqueue_event("a", to_partition("0"))
        .await
        .unwrap();
    h.client.flush().await.unwrap();

    match h.next_report().await {
        Report::Failed { error, .. } => {
            assert!(error.contains("not durably accepted"), "got {error}");
        }
        other => panic!("expected a failure report, got {other:?}"),
    }
    h.client.close().await.unwrap();
}

// 17. A flush waits for the events that the client accepted before the barrier.
#[tokio::test]
async fn flush_waits_for_events_accepted_before_the_barrier() {
    let mut h = harness(&["0"], Config::default()).await;

    let release = h.mock.gate("0");
    h.client
        .enqueue_event("before", to_partition("0"))
        .await
        .unwrap();

    let client = h.client.clone();
    let flush = client.flush();
    pin_mut!(flush);
    assert!(
        poll!(flush.as_mut()).is_pending(),
        "the flush must wait for the held send"
    );

    let _ = release.send(());
    flush.await.unwrap();

    let report = h.next_report().await;
    assert!(report.is_success());
    assert_eq!(report.bodies(), ["before"]);
    h.client.close().await.unwrap();
}

// 18. An event that arrives after the barrier does not delay that flush.
#[tokio::test]
async fn events_after_the_barrier_do_not_delay_the_flush() {
    let mut h = harness(&["0"], Config::default()).await;

    h.client
        .enqueue_event("before", to_partition("0"))
        .await
        .unwrap();

    let client = h.client.clone();
    let flush = client.flush();
    pin_mut!(flush);
    // The first poll puts the barrier into the queue.
    let _ = poll!(flush.as_mut());

    // This event sits behind the barrier.
    h.client
        .enqueue_event("after", to_partition("0"))
        .await
        .unwrap();

    flush.await.unwrap();

    // The flush covered the first event only. The wait time is 30 seconds, so
    // the second event is still in the buffer.
    let report = h.next_report().await;
    assert_eq!(report.bodies(), ["before"]);
    assert_eq!(h.client.total_buffered_event_count(), 1);

    h.client.close().await.unwrap();
}

// 19. Two flush calls at the same time both complete.
#[tokio::test]
async fn concurrent_flush_calls_both_complete() {
    let mut h = harness(&["0", "1"], Config::default()).await;

    h.client
        .enqueue_event("a", to_partition("0"))
        .await
        .unwrap();
    h.client
        .enqueue_event("b", to_partition("1"))
        .await
        .unwrap();

    let (first, second) = futures::join!(h.client.flush(), h.client.flush());
    first.unwrap();
    second.unwrap();

    assert_eq!(h.client.total_buffered_event_count(), 0);
    let reports = h.next_reports(2).await;
    assert!(reports.iter().all(|r| r.is_success()));
    h.client.close().await.unwrap();
}

// 20. A graceful close sends the buffered events.
#[tokio::test]
async fn graceful_close_sends_buffered_events() {
    let mut h = harness(&["0", "1"], Config::default()).await;

    h.client
        .enqueue_events(vec!["a", "b", "c"], to_partition("0"))
        .await
        .unwrap();
    h.client
        .enqueue_event("d", to_partition("1"))
        .await
        .unwrap();

    // No flush. The close must send them.
    h.client.close().await.unwrap();

    assert_eq!(h.mock.total_events(), 4);
    assert_eq!(h.client.total_buffered_event_count(), 0);

    let mut seen = Vec::new();
    while let Some(report) = h.reports.next().await {
        assert!(report.is_success());
        seen.extend(report.bodies().to_vec());
    }
    seen.sort();
    assert_eq!(seen, ["a", "b", "c", "d"]);
}

// 21. An immediate close abandons the buffered events and clears the counts.
#[tokio::test]
async fn immediate_close_abandons_buffered_events() {
    let h = harness(&["0"], Config::default()).await;

    h.client
        .enqueue_events(vec!["a", "b", "c"], to_partition("0"))
        .await
        .unwrap();
    assert_eq!(h.client.total_buffered_event_count(), 3);

    h.client.abort().await.unwrap();

    // The events never reached the service, and the counts agree.
    assert_eq!(h.mock.total_events(), 0);
    assert_eq!(h.client.total_buffered_event_count(), 0);
    assert_eq!(h.client.buffered_event_count("0"), 0);

    // A close is idempotent, in either form and in either order. A second call
    // does nothing and returns Ok.
    h.client.abort().await.unwrap();
    h.client.close().await.unwrap();
    assert_eq!(h.mock.total_events(), 0);
    assert_eq!(h.client.total_buffered_event_count(), 0);
    assert_eq!(h.client.buffered_event_count("0"), 0);
}

// 22. A close on an idle client completes.
#[tokio::test]
async fn closing_an_idle_client_completes() {
    let h = harness(&["0", "1", "2"], Config::default()).await;

    h.client.close().await.unwrap();

    // A second close does nothing and still succeeds.
    h.client.close().await.unwrap();
    assert_eq!(h.client.total_buffered_event_count(), 0);
}

// 22b. An enqueue after a close is rejected.
#[tokio::test]
async fn enqueue_after_close_is_rejected() {
    let h = harness(&["0"], Config::default()).await;
    h.client.close().await.unwrap();

    let error = h.client.enqueue_event("late", None).await.unwrap_err();
    assert!(error.to_string().contains("closed"));
}

// 23. A worker shutdown releases every reference that the worker held.
#[tokio::test]
async fn worker_shutdown_releases_references() {
    let h = harness(&["0", "1", "2"], Config::default()).await;

    // The workers each hold a reference to the send client.
    assert!(Arc::strong_count(&h.mock) > 1);

    h.client.close().await.unwrap();

    assert_eq!(
        Arc::strong_count(&h.mock),
        1,
        "the workers still hold a reference to the send client"
    );
}

// 24. A terminal failure does not stop the worker, and no event is lost or sent
// two times.
//
// A real connection recovery happens below the send seam, inside
// `RecoverableSender`. The live forced-error test covers that path. This test
// covers the part that the buffered producer owns: the worker survives a
// terminal failure and keeps serving its queue.
#[tokio::test]
async fn a_worker_keeps_serving_after_a_terminal_failure() {
    let mut h = harness(
        &["0"],
        Config {
            max_buffered: 1,
            ..Default::default()
        },
    )
    .await;

    h.mock.push_outcome("0", SendScript::Error("link detached"));

    h.client
        .enqueue_event("lost", to_partition("0"))
        .await
        .unwrap();
    let first = h.next_report().await;
    assert!(!first.is_success());
    assert_eq!(first.bodies(), ["lost"]);

    for index in 0..3 {
        h.client
            .enqueue_event(format!("after{index}"), to_partition("0"))
            .await
            .unwrap();
    }
    h.client.close().await.unwrap();

    let mut delivered = Vec::new();
    while let Some(report) = h.reports.next().await {
        if report.is_success() {
            delivered.extend(report.bodies().to_vec());
        }
    }
    delivered.sort();
    assert_eq!(delivered, ["after0", "after1", "after2"]);

    // Four sends in total: the one that failed, and the three that followed.
    assert_eq!(h.mock.sends().len(), 4);
}

// 25. Many enqueues at the same time neither lose nor repeat an event.
#[tokio::test]
async fn concurrent_enqueues_do_not_lose_or_repeat_events() {
    let mut h = harness(&["0", "1", "2", "3"], Config::default()).await;

    let mut tasks = Vec::new();
    for task_index in 0..4 {
        let client = h.client.clone();
        tasks.push(tokio::spawn(async move {
            for event_index in 0..25 {
                client
                    .enqueue_event(format!("t{task_index}-e{event_index}"), None)
                    .await
                    .unwrap();
            }
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }

    h.client.close().await.unwrap();

    assert_eq!(h.mock.total_events(), 100);

    let mut delivered = Vec::new();
    while let Some(report) = h.reports.next().await {
        assert!(report.is_success());
        delivered.extend(report.bodies().to_vec());
    }

    let unique: HashSet<&String> = delivered.iter().collect();
    assert_eq!(delivered.len(), 100, "an event was lost or repeated");
    assert_eq!(unique.len(), 100, "an event was repeated");
    assert_eq!(h.client.total_buffered_event_count(), 0);
}

// 26. Dropping the client stops the background work.
#[tokio::test]
async fn dropping_the_client_cancels_background_work() {
    let mut h = harness(&["0", "1"], Config::default()).await;

    h.client
        .enqueue_event("a", to_partition("0"))
        .await
        .unwrap();

    drop(h.client);

    // Every worker ends, so it releases the send client and the handlers. The
    // report stream ends once the last handler reference drops. The test holds
    // the only other reference to the send client.
    while h.reports.next().await.is_some() {}
    assert_eq!(
        Arc::strong_count(&h.mock),
        1,
        "a worker outlived the client that was dropped"
    );
}

// A client with no failure handler cannot open.
#[tokio::test]
async fn a_missing_failure_handler_is_an_error() {
    let (mock, _started) = MockSendClient::new(&["0"]);
    let error = BufferedProducerClient::builder()
        .open_with_send_client(mock as Arc<dyn BufferedSendClient>)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("with_on_send_failed"));
}

// The builder rejects settings that cannot work.
#[tokio::test]
async fn the_builder_rejects_invalid_settings() {
    let (mock, _started) = MockSendClient::new(&["0"]);
    let error = BufferedProducerClient::builder()
        .with_max_buffered_event_count_per_partition(0)
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("at least 1"));

    let error = BufferedProducerClient::builder()
        .with_max_wait_time(Duration::ZERO)
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock as Arc<dyn BufferedSendClient>)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("longer than zero"));
}

// The per-partition count follows the total count.
#[tokio::test]
async fn buffered_counts_track_each_partition() {
    let h = harness(&["0", "1"], Config::default()).await;

    h.client
        .enqueue_events(vec!["a", "b"], to_partition("0"))
        .await
        .unwrap();
    h.client
        .enqueue_event("c", to_partition("1"))
        .await
        .unwrap();

    assert_eq!(h.client.total_buffered_event_count(), 3);
    assert_eq!(h.client.buffered_event_count("0"), 2);
    assert_eq!(h.client.buffered_event_count("1"), 1);
    assert_eq!(h.client.buffered_event_count("unknown"), 0);

    h.client.close().await.unwrap();
    assert_eq!(h.client.total_buffered_event_count(), 0);
}

// 27. A worker that nothing cancels still abandons its active batch.
//
// `AbortableTask::abort` cancels a task on a runtime that supports it. On the
// standard thread runtime it only detaches the thread, so the worker keeps
// running and reaches the end of its queue, which is the same path as a
// graceful close. This test drives the worker with no cancellation at all, so
// only the abandon flag can stop the batch from going to the service.
#[tokio::test]
async fn an_abandoning_worker_that_nothing_cancels_does_not_publish() {
    let (mock, _started) = MockSendClient::with_max_message_size(&["0"], 1024 * 1024);

    let (sender, receiver) = mpsc::unbounded();
    let buffered = Arc::new(AtomicUsize::new(0));
    let total_buffered = Arc::new(AtomicUsize::new(0));
    let abandon = Arc::new(AtomicBool::new(false));
    let (stopped_tx, stopped_rx) = oneshot::channel();

    let reported = Arc::new(AtomicUsize::new(0));
    let counter = reported.clone();
    let handlers = DeliveryHandlers {
        succeeded: None,
        failed: Arc::new(move |_context| {
            let counter = counter.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::AcqRel);
            })
        }),
    };

    let worker = PartitionWorker::new(
        "0".to_string(),
        receiver,
        mock.clone() as Arc<dyn BufferedSendClient>,
        // Long enough that the timer cannot send the batch during the test.
        Duration::seconds(30),
        64,
        handlers,
        buffered.clone(),
        total_buffered.clone(),
        abandon.clone(),
        stopped_tx,
    );

    let capacity = Arc::new(Semaphore::new(4));
    let permit = capacity
        .try_acquire_arc()
        .expect("a new semaphore has capacity");
    let event = EventData::from("abandoned");
    buffered.fetch_add(1, Ordering::AcqRel);
    total_buffered.fetch_add(1, Ordering::AcqRel);
    sender
        .unbounded_send(Command::Event {
            message: Box::new(AmqpMessage::from(event.clone())),
            event,
            permit,
        })
        .expect("the worker holds the receiver");

    let run = worker.run();
    pin_mut!(run);

    // One poll takes the event into the active batch. The batch is not full and
    // the wait time is long, so the worker then waits for the next command.
    assert!(poll!(&mut run).is_pending());

    // Abandon the events and end the queue, exactly as an immediate close does.
    abandon.store(true, Ordering::Release);
    drop(sender);

    // Nothing cancels this future, so the worker runs its close path in full.
    run.await;

    assert_eq!(
        mock.total_events(),
        0,
        "an immediate close promised to drop these events, so none may reach the service"
    );
    assert_eq!(
        reported.load(Ordering::Acquire),
        0,
        "an abandoned event has no delivery outcome to report"
    );
    assert!(
        stopped_rx.await.is_ok(),
        "the worker must tell the client that it stopped"
    );
}

// 28. The buffered counts stay sane when every event reaches a terminal outcome
// as soon as the worker sees it.
//
// An oversized event fails inside the worker without a send. The client counts
// the event before it publishes the command, so that failure can never
// decrement a count that is still zero and wrap it to `usize::MAX`. The
// interleaving that this guards against is a race, so this test does not
// reproduce it on demand; it states the invariant and exercises the path with
// many events on more than one thread.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_fast_terminal_outcome_does_not_wrap_the_buffered_counts() {
    const EVENT_COUNT: usize = 200;

    let mut h = harness(
        &["0"],
        Config {
            max_message_size: 200,
            with_success_handler: false,
            ..Default::default()
        },
    )
    .await;

    let big = "x".repeat(4096);
    for _ in 0..EVENT_COUNT {
        h.client
            .enqueue_event(big.clone(), to_partition("0"))
            .await
            .unwrap();
        // A wrapped count is astronomically large, and the real count can never
        // pass the number of events that the test enqueued.
        assert!(
            h.client.total_buffered_event_count() <= EVENT_COUNT,
            "the buffered count wrapped: {}",
            h.client.total_buffered_event_count()
        );
    }

    for _ in 0..EVENT_COUNT {
        assert!(!h.next_report().await.is_success());
    }

    h.client.close().await.unwrap();
    assert_eq!(h.client.total_buffered_event_count(), 0);
    assert_eq!(h.client.buffered_event_count("0"), 0);
}

// 28b. An immediate close cannot reset counts while an enqueue can still roll
// back its accounting.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn abort_during_enqueue_keeps_buffered_counts_at_zero() {
    use std::sync::{mpsc as std_mpsc, Arc};

    let h = harness(
        &["0"],
        Config {
            with_success_handler: false,
            ..Default::default()
        },
    )
    .await;

    let accounted = Arc::new(AtomicBool::new(false));
    let accounted_for_hook = accounted.clone();
    let (entered_tx, entered_rx) = std_mpsc::channel();
    let (release_tx, release_rx) = std_mpsc::channel();
    let client_for_hook = h.client.clone();
    h.client.set_enqueue_hook(Box::new(move || {
        // The fixed implementation still owns this lock here. The old
        // implementation released it before accounting.
        accounted_for_hook.store(
            client_for_hook
                .partitions
                .get("0")
                .expect("the test client has partition 0")
                .sender
                .try_lock()
                .is_err(),
            Ordering::Release,
        );
        entered_tx
            .send(())
            .expect("the test must observe the enqueue");
        release_rx
            .recv()
            .expect("the test must release the enqueue");
    }));

    let client_for_enqueue = h.client.clone();
    let enqueue = tokio::spawn(async move {
        client_for_enqueue
            .enqueue_event("race", to_partition("0"))
            .await
    });

    while entered_rx.try_recv().is_err() {
        tokio::task::yield_now().await;
    }

    // Start the abort while enqueue is paused after accounting.
    let client_for_abort = h.client.clone();
    let abort = tokio::spawn(async move { client_for_abort.abort().await });
    while !h.client.closed.load(Ordering::Acquire) {
        tokio::task::yield_now().await;
    }

    if accounted.load(Ordering::Acquire) {
        // The fixed implementation keeps the sender lock, so abort cannot
        // finish until enqueue sends and releases the lock.
        release_tx.send(()).expect("the enqueue hook is waiting");
        let _ = enqueue.await.unwrap();
        abort.await.unwrap().unwrap();
    } else {
        // The old implementation released the sender lock before accounting.
        // Let abort reset both counters before enqueue rolls back its failed
        // send. That is the wrapping interleaving this test guards against.
        abort.await.unwrap().unwrap();
        release_tx.send(()).expect("the enqueue hook is waiting");
        let _ = enqueue.await.unwrap();
    }

    assert!(accounted.load(Ordering::Acquire));
    assert_eq!(h.client.total_buffered_event_count(), 0);
    assert_eq!(h.client.buffered_event_count("0"), 0);
}

// 29. A delivery handler can enqueue again without deadlocking the worker.
//
// The handler runs on the worker task, and the worker is the only thing that
// returns capacity permits. While the worker held the permits of the batch it
// was reporting, a handler that enqueued to the same partition waited for a
// permit that only the worker could return, and the worker waited for the
// handler. The event is already at a terminal outcome when the handler runs, so
// the permit goes back first.
#[tokio::test]
async fn a_failure_handler_can_enqueue_again() {
    use std::sync::{OnceLock, Weak};

    // One permit for the partition, so the retry can only proceed if the
    // failing event already gave its permit back.
    const BUFFER: usize = 1;

    let (mock, _started) = MockSendClient::with_max_message_size(&["0"], 200);

    let slot: Arc<OnceLock<Weak<BufferedProducerClient>>> = Arc::new(OnceLock::new());
    let retried = Arc::new(AtomicUsize::new(0));

    let for_handler = slot.clone();
    let counter = retried.clone();
    let client = BufferedProducerClient::builder()
        .with_max_wait_time(Duration::seconds(30))
        .with_max_buffered_event_count_per_partition(BUFFER)
        .with_on_send_failed(move |_context| {
            let slot = for_handler.clone();
            let counter = counter.clone();
            async move {
                // Retry once. A second retry would recurse without an end.
                if counter.fetch_add(1, Ordering::AcqRel) > 0 {
                    return;
                }
                let client = slot
                    .get()
                    .expect("the test sets the client before it enqueues")
                    .upgrade()
                    .expect("the client is alive while the handler runs");
                client
                    .enqueue_event("small", to_partition("0"))
                    .await
                    .expect("the retry must not be rejected");
            }
        })
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    let client = Arc::new(client);
    slot.set(Arc::downgrade(&client)).expect("set once");

    // Too large for the link, so the worker fails it without a send. That is
    // the path that calls the handler while it still holds the permit.
    let oversized = "x".repeat(4096);
    client
        .enqueue_event(oversized, to_partition("0"))
        .await
        .unwrap();

    // The deadlock shows up as a hang, so bound it.
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        while retried.load(Ordering::Acquire) == 0 {
            tokio::task::yield_now().await;
        }
        client.flush().await
    })
    .await
    .expect("the failure handler deadlocked with the worker")
    .expect("the flush completed");

    assert_eq!(
        mock.total_events(),
        1,
        "the event that the handler enqueued must reach the service"
    );

    client.close().await.unwrap();
}

// 30. One partition has one send in flight at a time.
//
// The worker awaits each send, so the batch behind it cannot start before the
// batch in front of it settles. That is what keeps the events of a partition in
// the order that the caller enqueued them, and what makes the delivery reports
// of a partition arrive in that same order.
#[tokio::test]
async fn only_one_send_is_in_flight_for_a_partition() {
    // One 1000 byte event fits the link and two do not, so each event is a
    // batch of its own. The buffer holds 64 events, so no enqueue waits.
    let mut h = harness(
        &["0"],
        Config {
            max_message_size: 2500,
            ..Default::default()
        },
    )
    .await;

    let release = h.mock.gate("0");

    let first = "a".repeat(1000);
    let second = "b".repeat(1000);
    let third = "c".repeat(1000);
    for body in [&first, &second, &third] {
        h.client
            .enqueue_event(body.clone(), to_partition("0"))
            .await
            .unwrap();
    }

    // The gate holds the first send, so that send is in flight.
    let started = h.next_started().await;
    assert_eq!(started, "0");

    {
        // The next two events are each a full batch, and both are in the queue.
        // Neither may start while the first send is in flight.
        let next_start = h.started.next();
        pin_mut!(next_start);
        assert!(
            poll!(next_start.as_mut()).is_pending(),
            "a second send started while the first was in flight"
        );
    }

    let _ = release.send(());

    // The reports arrive in the enqueue order, because the sends did.
    let reports = h.next_reports(2).await;
    assert!(reports.iter().all(|r| r.is_success()));
    assert_eq!(reports[0].bodies(), [first]);
    assert_eq!(reports[1].bodies(), [second]);

    h.client.close().await.unwrap();
    assert_eq!(h.mock.sends().len(), 3);
}

// 31. The builder defaults match the other Azure SDKs.
//
// A default that drifts changes the throughput and the memory of every
// application that does not set these values.
#[tokio::test]
async fn the_builder_defaults_match_the_other_azure_sdks() {
    assert_eq!(DEFAULT_MAX_WAIT_TIME_SECONDS, 1);
    assert_eq!(DEFAULT_MAX_BUFFERED_EVENT_COUNT_PER_PARTITION, 1500);

    let (mock, _started) = MockSendClient::new(&["0"]);
    let client = BufferedProducerClient::builder()
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    // Hold the first send. No event then reaches a terminal outcome, so no
    // capacity permit returns while the test fills the buffer.
    let _release = mock.gate("0");

    // The default buffer holds 1500 events for one partition.
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        for index in 0..1500 {
            client
                .enqueue_event(format!("e{index}"), to_partition("0"))
                .await
                .expect("the default buffer holds 1500 events for one partition");
        }
    })
    .await
    .expect("an enqueue waited for space before the buffer held 1500 events");

    // The next event finds no permit, so the enqueue waits for space.
    let pending = client.enqueue_event("overflow", to_partition("0"));
    pin_mut!(pending);
    assert!(
        poll!(pending.as_mut()).is_pending(),
        "the default buffer accepted more than 1500 events for one partition"
    );

    client.abort().await.unwrap();
}

// 32. A success handler can enqueue again once a send settles.
//
// Test 29 covers the path that fails an event before any send. This test covers
// the send path. The batch is at a terminal outcome as soon as the send settles,
// so the worker gives the capacity back before it calls the handlers. A handler
// that enqueues to the same partition would otherwise wait for a permit that
// only the worker can return, and the worker would wait for the handler.
#[tokio::test]
async fn a_success_handler_can_enqueue_again_after_a_send_settles() {
    use std::sync::{OnceLock, Weak};

    // One permit for the partition, so the handler can only proceed if the
    // batch that it reports already gave its permit back.
    const BUFFER: usize = 1;

    let (mock, _started) = MockSendClient::new(&["0"]);

    let slot: Arc<OnceLock<Weak<BufferedProducerClient>>> = Arc::new(OnceLock::new());
    let handled = Arc::new(AtomicUsize::new(0));

    let for_handler = slot.clone();
    let counter = handled.clone();
    let client = BufferedProducerClient::builder()
        .with_max_wait_time(Duration::seconds(30))
        .with_max_buffered_event_count_per_partition(BUFFER)
        .with_on_send_succeeded(move |_context| {
            let slot = for_handler.clone();
            let counter = counter.clone();
            async move {
                // Enqueue one time only. A second one would recurse without an
                // end.
                if counter.fetch_add(1, Ordering::AcqRel) > 0 {
                    return;
                }
                let client = slot
                    .get()
                    .expect("the test sets the client before it enqueues")
                    .upgrade()
                    .expect("the client is alive while the handler runs");
                client
                    .enqueue_event("second", to_partition("0"))
                    .await
                    .expect("the handler must not be rejected");
            }
        })
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    let client = Arc::new(client);
    slot.set(Arc::downgrade(&client)).expect("set once");

    // The batch holds one event, so the worker sends this one at once and the
    // mock accepts it.
    client
        .enqueue_event("first", to_partition("0"))
        .await
        .unwrap();

    // The deadlock shows up as a hang, so bound it. The counter reaches 2 once
    // the handler ran for the event that the handler itself enqueued.
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        while handled.load(Ordering::Acquire) < 2 {
            tokio::task::yield_now().await;
        }
        client.flush().await
    })
    .await
    .expect("the success handler deadlocked with the worker")
    .expect("the flush completed");

    assert_eq!(
        mock.total_events(),
        2,
        "the event that the handler enqueued must reach the service"
    );

    client.close().await.unwrap();
}

// 33. A slow delivery handler delays only its own partition.
//
// Each partition has its own worker, and a worker calls the handlers itself.
// The builder documents that a slow handler slows only its own partition, so
// the client must not report the outcomes of every partition through one task.
#[tokio::test]
async fn a_slow_handler_delays_only_its_own_partition() {
    let (mock, _started) = MockSendClient::new(&["0", "1"]);
    let (report_tx, mut reports) = mpsc::unbounded();

    // The handler of partition 0 waits for this channel. The test holds the
    // sender, so only the test releases that handler.
    let (release, parked) = oneshot::channel::<()>();
    let parked = Arc::new(Mutex::new(Some(parked)));

    let tx = report_tx.clone();
    let for_handler = parked.clone();
    let client = BufferedProducerClient::builder()
        .with_max_wait_time(Duration::seconds(30))
        // One event for each batch, so each event is its own send.
        .with_max_buffered_event_count_per_partition(1)
        .with_on_send_succeeded(move |context: SendBatchSucceededContext| {
            let tx = tx.clone();
            let parked = for_handler.clone();
            async move {
                if context.partition_id == "0" {
                    let waiter = parked.lock().unwrap().take();
                    if let Some(waiter) = waiter {
                        let _ = waiter.await;
                    }
                }
                let _ = tx.unbounded_send(Report::Succeeded {
                    partition_id: context.partition_id,
                    bodies: bodies_of(&context.events),
                });
            }
        })
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    client.enqueue_event("a", to_partition("0")).await.unwrap();
    client.enqueue_event("b", to_partition("1")).await.unwrap();

    // The handler of partition 0 is parked, and the report of partition 1 still
    // arrives.
    let report = tokio::time::timeout(std::time::Duration::from_secs(10), reports.next())
        .await
        .expect("the parked handler of partition 0 also held partition 1")
        .expect("a delivery report was expected");
    assert_eq!(report.partition_id(), "1");
    assert_eq!(report.bodies(), ["b"]);

    // The permit of "a" already returned, so partition 0 accepts another event.
    // Its worker is inside the parked handler, so that event has no report yet.
    client.enqueue_event("c", to_partition("0")).await.unwrap();
    {
        let next_report = reports.next();
        pin_mut!(next_report);
        assert!(
            poll!(next_report.as_mut()).is_pending(),
            "partition 0 reported an outcome while its handler was held"
        );
    }

    let _ = release.send(());

    let first = reports.next().await.expect("a report was expected");
    assert_eq!(first.partition_id(), "0");
    assert_eq!(first.bodies(), ["a"]);
    let second = reports.next().await.expect("a report was expected");
    assert_eq!(second.partition_id(), "0");
    assert_eq!(second.bodies(), ["c"]);

    client.close().await.unwrap();
    assert_eq!(mock.total_events(), 3);
}

// 34. A delivery handler that panics does not hang a close.
//
// The panicking handler kills the worker of its partition. The client must
// still serve the other partitions, and a close must still complete: it waits
// for an acknowledgement that a dead worker can never send, so it has to accept
// the cancelled channel and the join error of that task.
//
// The panic message in the test log is expected.
#[tokio::test]
async fn close_completes_after_a_handler_panics() {
    let (mock, _started) = MockSendClient::new(&["0", "1"]);
    let (report_tx, mut reports) = mpsc::unbounded();

    let client = BufferedProducerClient::builder()
        .with_max_wait_time(Duration::seconds(30))
        // One event for each batch, so each event is its own send.
        .with_max_buffered_event_count_per_partition(1)
        .with_on_send_succeeded(move |context: SendBatchSucceededContext| {
            let tx = report_tx.clone();
            async move {
                if context.partition_id == "0" {
                    panic!("the handler of partition 0 panics on purpose");
                }
                let _ = tx.unbounded_send(Report::Succeeded {
                    partition_id: context.partition_id,
                    bodies: bodies_of(&context.events),
                });
            }
        })
        .with_on_send_failed(|_| async {})
        .open_with_send_client(mock.clone() as Arc<dyn BufferedSendClient>)
        .await
        .expect("the client opened");

    // The send settles, the handler panics, and the worker of partition 0 dies.
    client.enqueue_event("a", to_partition("0")).await.unwrap();

    // The client still serves the other partition.
    client.enqueue_event("b", to_partition("1")).await.unwrap();
    let report = tokio::time::timeout(std::time::Duration::from_secs(10), reports.next())
        .await
        .expect("the client stopped serving partition 1 after the panic")
        .expect("a delivery report was expected");
    assert_eq!(report.partition_id(), "1");
    assert_eq!(report.bodies(), ["b"]);

    tokio::time::timeout(std::time::Duration::from_secs(10), client.close())
        .await
        .expect("close hung after a handler panicked")
        .expect("close failed after a handler panicked");
}

/// Live tests for the buffered producer.
///
/// These tests need a real Event Hub. They live in the crate because
/// `ProducerClient::force_error` is only available to crate tests.
#[cfg(test)]
mod live {
    use crate::{
        common::tests::force_errors, BufferedProducerClient, EnqueueEventOptions,
        SendBatchFailedContext, SendBatchSucceededContext,
    };
    use azure_core::time::Duration;
    use azure_core_amqp::{error::AmqpErrorKind, AmqpError};
    use azure_core_test::{recorded, TestContext};
    use std::{
        collections::HashSet,
        sync::{Arc, Mutex},
    };

    /// Records which event bodies reached a terminal outcome.
    #[derive(Default)]
    struct Outcomes {
        succeeded: Mutex<Vec<String>>,
        failed: Mutex<Vec<String>>,
    }

    fn bodies(events: &[crate::models::EventData]) -> Vec<String> {
        events
            .iter()
            .map(|event| String::from_utf8_lossy(event.body().unwrap_or_default()).into_owned())
            .collect()
    }

    /// A connection recovery must not lose an event, and it must not report an
    /// event two times.
    #[recorded::test(live)]
    async fn buffered_recovery_keeps_every_event(ctx: TestContext) -> crate::Result<()> {
        const TEST_NAME: &str = "buffered_recovery_keeps_every_event";
        const PARTITION: &str = "1";

        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();

        let outcomes = Arc::new(Outcomes::default());
        let for_success = outcomes.clone();
        let for_failure = outcomes.clone();

        let producer = Arc::new(
            BufferedProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .with_max_wait_time(Duration::milliseconds(200))
                .with_on_send_succeeded(move |context: SendBatchSucceededContext| {
                    let outcomes = for_success.clone();
                    async move {
                        outcomes
                            .succeeded
                            .lock()
                            .unwrap()
                            .extend(bodies(&context.events));
                    }
                })
                .with_on_send_failed(move |context: SendBatchFailedContext| {
                    let outcomes = for_failure.clone();
                    async move {
                        outcomes
                            .failed
                            .lock()
                            .unwrap()
                            .extend(bodies(&context.events));
                    }
                })
                .open(host.as_str(), eventhub.as_str(), credential.clone())
                .await?,
        );

        let enqueued = Arc::new(Mutex::new(Vec::<String>::new()));
        let for_test = enqueued.clone();

        force_errors(
            producer.clone(),
            move |producer: Arc<BufferedProducerClient>| {
                let enqueued = for_test.clone();
                async move {
                    let mut index = 0usize;
                    loop {
                        let body = format!("recovery-{index}");
                        // An enqueue can fail only when the client closes.
                        if producer
                            .enqueue_event(
                                body.clone(),
                                Some(EnqueueEventOptions {
                                    partition_id: Some(PARTITION.to_string()),
                                    ..Default::default()
                                }),
                            )
                            .await
                            .is_err()
                        {
                            break;
                        }
                        enqueued.lock().unwrap().push(body);
                        index += 1;
                    }
                }
            },
            |producer: Arc<BufferedProducerClient>| {
                // Break the link under the worker. The recoverable sender must
                // rebuild it without the buffered producer losing an event.
                producer
                    .inner_producer()
                    .expect("a live client has a producer")
                    .force_error(AmqpError::from(AmqpErrorKind::LinkClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )))
                    .unwrap();
            },
            Duration::seconds(5),
            Duration::seconds(20),
        )
        .await?;

        producer.close().await?;

        let enqueued = enqueued.lock().unwrap().clone();
        let succeeded = outcomes.succeeded.lock().unwrap().clone();
        let failed = outcomes.failed.lock().unwrap().clone();

        // Every event that the client accepted reached exactly one terminal
        // outcome.
        let mut reported: Vec<String> = succeeded.iter().chain(failed.iter()).cloned().collect();
        reported.sort();
        let unique: HashSet<&String> = reported.iter().collect();
        assert_eq!(
            reported.len(),
            unique.len(),
            "an event reached a terminal outcome two times"
        );
        assert_eq!(
            reported.len(),
            enqueued.len(),
            "the client did not report every accepted event"
        );
        assert_eq!(producer.total_buffered_event_count(), 0);

        Ok(())
    }

    /// A transient sender attach failure must not fail an event that the
    /// buffered producer already accepted.
    #[recorded::test(live)]
    async fn buffered_sender_attach_retries_before_reporting_failure(
        ctx: TestContext,
    ) -> crate::Result<()> {
        const TEST_NAME: &str = "buffered_sender_attach_retries_before_reporting_failure";
        const PARTITION: &str = "1";

        let recording = ctx.recording();
        let host = recording.var("EVENTHUBS_HOST", None);
        let eventhub = recording.var("EVENTHUB_NAME", None);
        let credential = recording.credential();

        let outcomes = Arc::new(Outcomes::default());
        let for_success = outcomes.clone();
        let for_failure = outcomes.clone();
        let producer = Arc::new(
            BufferedProducerClient::builder()
                .with_application_id(TEST_NAME.to_string())
                .with_max_wait_time(Duration::seconds(30))
                .with_on_send_succeeded(move |context: SendBatchSucceededContext| {
                    let outcomes = for_success.clone();
                    async move {
                        outcomes
                            .succeeded
                            .lock()
                            .unwrap()
                            .extend(bodies(&context.events));
                    }
                })
                .with_on_send_failed(move |context: SendBatchFailedContext| {
                    let outcomes = for_failure.clone();
                    async move {
                        outcomes
                            .failed
                            .lock()
                            .unwrap()
                            .extend(bodies(&context.events));
                    }
                })
                .open(host.as_str(), eventhub.as_str(), credential)
                .await?,
        );

        // The first sender attach fails before it reaches the broker. The
        // recoverable sender must re-run the attach and size lookup, then the
        // worker can build and send the batch.
        producer
            .inner_producer()
            .expect("a live client has a producer")
            .force_attach_error(AmqpError::from(AmqpErrorKind::LinkClosedByRemote(
                Box::new(azure_core::error::Error::new(
                    azure_core::error::ErrorKind::Other,
                    "Forced attach error",
                )),
            )))
            .unwrap();

        producer
            .enqueue_event(
                "attach-retry",
                Some(EnqueueEventOptions {
                    partition_id: Some(PARTITION.to_string()),
                    ..Default::default()
                }),
            )
            .await?;
        producer.flush().await?;
        producer.close().await?;

        assert_eq!(
            outcomes.succeeded.lock().unwrap().as_slice(),
            ["attach-retry"]
        );
        assert!(outcomes.failed.lock().unwrap().is_empty());
        Ok(())
    }
}
