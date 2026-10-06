// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod review_regressions;

use super::{
    decode_items,
    tests::{setup, setup_with_driver},
    BatchState, ChangeFeedMode, ChangeFeedPage, ChangeFeedReadOptions, CheckpointError,
    CheckpointStore, LeaseControl, LeaseRunOptions, LeaseRunOutcome, LeaseRunPhase, OwnedLease,
    RawChangeFeedPage,
};
use azure_core::{http::StatusCode, Bytes};
use azure_cosmos_change_feed_processor_engine::{
    run_lease, LeaseRunError, LeaseRunReport, RawBatchSource,
};
use azure_data_cosmos_driver::{
    models::{
        ChangeFeedStartFrom, ContinuationToken, CosmosOperation, FeedRange, MaxItemCountHint,
    },
    options::OperationOptions,
    CosmosError, ResponseBody,
};
use futures::future::BoxFuture;
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::VecDeque,
    error::Error,
    num::{NonZeroU32, NonZeroU64},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
    time::timeout,
};

#[derive(Debug, Deserialize)]
struct Document {
    id: String,
}

fn failure(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

async fn pages() -> Result<(OwnedLease, Vec<RawChangeFeedPage>), Box<dyn Error>> {
    let processor = setup(1).await?;
    let range = FeedRange::full();
    let mut reader = processor
        .engine
        .open_reader(
            ChangeFeedReadOptions::new(range.clone(), ChangeFeedStartFrom::Beginning)
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap())),
        )
        .await?;
    let lease = OwnedLease::new(
        "lease",
        "owner",
        NonZeroU64::new(1).unwrap(),
        "initial",
        range,
        reader.to_continuation_token()?,
    )?;
    let mut result = Vec::new();
    for _ in 0..4 {
        result.push(reader.read_page().await?);
    }
    assert_eq!(result[3].state(), BatchState::Idle);
    Ok((lease, result))
}

struct Source {
    pages: VecDeque<RawChangeFeedPage>,
    reads: Arc<AtomicU32>,
    events: mpsc::UnboundedSender<&'static str>,
}
impl RawBatchSource for Source {
    fn read_batch(&mut self) -> BoxFuture<'_, azure_data_cosmos_driver::Result<RawChangeFeedPage>> {
        Box::pin(async {
            self.reads.fetch_add(1, Ordering::SeqCst);
            self.events.send("read").unwrap();
            self.pages
                .pop_front()
                .ok_or_else(|| failure("scripted source exhausted"))
        })
    }
}

enum Write {
    Confirm,
    Gate(oneshot::Receiver<()>),
    Retryable,
    Ambiguous,
    Reject,
    InvalidReceipt,
}
struct Store {
    actions: VecDeque<Write>,
    writes: Arc<AtomicU32>,
    durable: Arc<Mutex<OwnedLease>>,
    events: mpsc::UnboundedSender<&'static str>,
}
impl CheckpointStore for Store {
    fn persist<'a>(
        &'a mut self,
        lease: &'a OwnedLease,
        candidate: &'a ContinuationToken,
    ) -> BoxFuture<'a, Result<OwnedLease, CheckpointError>> {
        Box::pin(async move {
            let attempt = self.writes.fetch_add(1, Ordering::SeqCst) + 1;
            self.events.send("checkpoint").unwrap();
            let action = self
                .actions
                .pop_front()
                .expect("every persistence attempt is scripted");
            match action {
                Write::Gate(receiver) => receiver
                    .await
                    .map_err(|_| CheckpointError::Ambiguous(failure("persistence interrupted")))?,
                Write::Retryable => {
                    return Err(CheckpointError::Retryable(failure(
                        "definitely not persisted",
                    )))
                }
                Write::Ambiguous => {
                    return Err(CheckpointError::Ambiguous(failure("store outcome unknown")))
                }
                Write::Reject => {
                    return Err(CheckpointError::Rejected(failure(
                        "conditional revision rejected",
                    )))
                }
                Write::InvalidReceipt => return Ok(lease.clone()),
                Write::Confirm => {}
            }
            let mut durable = self.durable.lock().unwrap();
            if durable.revision() != lease.revision()
                || durable.owner() != lease.owner()
                || durable.epoch() != lease.epoch()
            {
                return Err(CheckpointError::Rejected(failure(
                    "ownership/revision mismatch",
                )));
            }
            let confirmed = OwnedLease::new(
                lease.id(),
                lease.owner(),
                lease.epoch(),
                format!("revision-{attempt}"),
                lease.range().clone(),
                candidate.clone(),
            )
            .unwrap();
            *durable = confirmed.clone();
            Ok(confirmed)
        })
    }
}

enum Handle {
    Complete,
    Gate(oneshot::Receiver<()>),
    Fail,
    PanicOnCall,
    PanicOnPoll,
}
struct Harness {
    reads: Arc<AtomicU32>,
    writes: Arc<AtomicU32>,
    handled: Arc<AtomicU32>,
    durable: Arc<Mutex<OwnedLease>>,
    control: LeaseControl,
    events: mpsc::UnboundedReceiver<&'static str>,
    task: JoinHandle<Result<LeaseRunReport, LeaseRunError>>,
}

fn launch(
    lease: OwnedLease,
    pages: Vec<RawChangeFeedPage>,
    writes: Vec<Write>,
    handlers: Vec<Handle>,
    options: LeaseRunOptions,
) -> Harness {
    let reads = Arc::new(AtomicU32::new(0));
    let write_count = Arc::new(AtomicU32::new(0));
    let handled = Arc::new(AtomicU32::new(0));
    let durable = Arc::new(Mutex::new(lease.clone()));
    let control = LeaseControl::default();
    let (tx, events) = mpsc::unbounded_channel();
    let mut source = Source {
        pages: pages.into(),
        reads: reads.clone(),
        events: tx.clone(),
    };
    let mut store = Store {
        actions: writes.into(),
        writes: write_count.clone(),
        durable: durable.clone(),
        events: tx.clone(),
    };
    let run_control = control.clone();
    let handler_count = handled.clone();
    let mut handlers: VecDeque<_> = handlers.into();
    let task = tokio::spawn(async move {
        run_lease(
            &mut source,
            &mut store,
            lease,
            &run_control,
            &options,
            |raw| {
                let count = handler_count.clone();
                let tx = tx.clone();
                let action = handlers
                    .pop_front()
                    .expect("every delivered batch is scripted");
                if matches!(action, Handle::PanicOnCall) {
                    panic!("application callback construction panic");
                }
                async move {
                    let page: ChangeFeedPage<Document> = raw.try_into()?;
                    assert!(page
                        .items()
                        .iter()
                        .all(|item| item.current().is_some_and(|doc| !doc.id.is_empty())));
                    count.fetch_add(1, Ordering::SeqCst);
                    tx.send("handler").unwrap();
                    match action {
                        Handle::Complete => Ok(()),
                        Handle::Gate(receiver) => {
                            receiver
                                .await
                                .map_err(|_| failure("handler completion channel closed"))?;
                            Ok(())
                        }
                        Handle::Fail => Err(failure("application rejected batch")),
                        Handle::PanicOnCall => unreachable!("construction panic already occurred"),
                        Handle::PanicOnPoll => {
                            tokio::task::yield_now().await;
                            panic!("application callback poll panic")
                        }
                    }
                }
            },
        )
        .await
    });
    Harness {
        reads,
        writes: write_count,
        handled,
        durable,
        control,
        events,
        task,
    }
}

fn options(count: u32) -> LeaseRunOptions {
    LeaseRunOptions::new(NonZeroU32::new(count).unwrap())
        .with_retry_delay(Duration::ZERO)
        .with_idle_delay(Duration::ZERO)
}

async fn event(harness: &mut Harness, expected: &str) {
    assert_eq!(
        timeout(Duration::from_secs(5), harness.events.recv())
            .await
            .unwrap(),
        Some(expected)
    );
}

#[tokio::test]
async fn handling_and_checkpointing_each_block_the_next_read() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (ack, handling) = oneshot::channel();
    let (commit, persistence) = oneshot::channel();
    let mut h = launch(
        lease.clone(),
        pages,
        vec![Write::Gate(persistence), Write::Confirm],
        vec![Handle::Gate(handling), Handle::Complete],
        options(2),
    );
    event(&mut h, "read").await;
    event(&mut h, "handler").await;
    tokio::task::yield_now().await;
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    assert_eq!(h.durable.lock().unwrap().checkpoint(), lease.checkpoint());
    ack.send(()).unwrap();
    event(&mut h, "checkpoint").await;
    tokio::task::yield_now().await;
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    assert_eq!(h.durable.lock().unwrap().checkpoint(), lease.checkpoint());
    commit.send(()).unwrap();
    let report = timeout(Duration::from_secs(5), h.task).await???;
    assert_eq!(report.outcome(), LeaseRunOutcome::BatchLimitReached);
    assert_eq!(report.confirmed_batches(), 2);
    assert!(report.candidate().is_none());
    assert_eq!(h.reads.load(Ordering::SeqCst), 2);
    assert_eq!(h.handled.load(Ordering::SeqCst), 2);
    assert_eq!(h.writes.load(Ordering::SeqCst), 2);
    assert_eq!(
        report.lease().checkpoint(),
        h.durable.lock().unwrap().checkpoint()
    );
    Ok(())
}

#[tokio::test]
async fn handler_failure_preserves_durable_position_and_diagnostics() -> Result<(), Box<dyn Error>>
{
    let (lease, pages) = pages().await?;
    let candidate = pages[0].continuation().clone();
    let h = launch(lease.clone(), pages, vec![], vec![Handle::Fail], options(2));
    let error = h.task.await?.unwrap_err();
    assert_eq!(error.report().outcome(), LeaseRunOutcome::Failed);
    assert_eq!(error.report().phase(), Some(LeaseRunPhase::Handling));
    assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
    assert_eq!(error.report().candidate(), Some(&candidate));
    assert!(error.error().diagnostics().is_some());
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test]
async fn checkpoint_retry_does_not_repeat_the_handler() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (commit, persistence) = oneshot::channel();
    let mut h = launch(
        lease,
        pages,
        vec![Write::Retryable, Write::Gate(persistence)],
        vec![Handle::Complete],
        options(1),
    );
    for expected in ["read", "handler", "checkpoint", "checkpoint"] {
        event(&mut h, expected).await;
    }
    assert_eq!(h.handled.load(Ordering::SeqCst), 1);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    commit.send(()).unwrap();
    let report = h.task.await??;
    assert_eq!(report.confirmed_batches(), 1);
    assert_eq!(h.writes.load(Ordering::SeqCst), 2);
    Ok(())
}

#[tokio::test]
async fn ambiguous_rejected_and_invalid_receipts_never_report_progress(
) -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    for action in [Write::Ambiguous, Write::Reject, Write::InvalidReceipt] {
        let h = launch(
            lease.clone(),
            pages.clone(),
            vec![action],
            vec![Handle::Complete],
            options(2),
        );
        let error = h.task.await?.unwrap_err();
        assert_eq!(error.report().phase(), Some(LeaseRunPhase::Checkpointing));
        assert_eq!(error.report().confirmed_batches(), 0);
        assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
        assert_eq!(error.report().candidate(), Some(pages[0].continuation()));
        assert!(error.error().diagnostics().is_some());
        assert_eq!(h.reads.load(Ordering::SeqCst), 1);
        assert_eq!(h.writes.load(Ordering::SeqCst), 1);
    }
    Ok(())
}

#[tokio::test]
async fn loss_of_ownership_rejects_late_handler_completion() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (late_ack, handling) = oneshot::channel();
    let mut h = launch(
        lease.clone(),
        pages,
        vec![],
        vec![Handle::Gate(handling)],
        options(2),
    );
    event(&mut h, "read").await;
    event(&mut h, "handler").await;
    h.control.lose_ownership();
    let report = h.task.await??;
    assert_eq!(report.outcome(), LeaseRunOutcome::OwnershipLost);
    assert!(
        late_ack.send(()).is_err(),
        "revoked completion receiver must be dropped"
    );
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    assert_eq!(report.lease().checkpoint(), lease.checkpoint());
    assert!(report.candidate().is_some());
    Ok(())
}

#[tokio::test]
async fn stop_during_handling_or_persistence_drains_without_new_delivery(
) -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    for during_handler in [true, false] {
        let (release, gate) = oneshot::channel();
        let (writes, handlers) = if during_handler {
            (vec![Write::Confirm], vec![Handle::Gate(gate)])
        } else {
            (vec![Write::Gate(gate)], vec![Handle::Complete])
        };
        let mut h = launch(lease.clone(), pages.clone(), writes, handlers, options(2));
        event(&mut h, "read").await;
        event(&mut h, "handler").await;
        if !during_handler {
            event(&mut h, "checkpoint").await;
        }
        h.control.stop();
        tokio::task::yield_now().await;
        assert_eq!(h.reads.load(Ordering::SeqCst), 1);
        release.send(()).unwrap();
        let report = h.task.await??;
        assert_eq!(report.outcome(), LeaseRunOutcome::StoppedDrained);
        assert_eq!(report.confirmed_batches(), 1);
        assert!(report.candidate().is_none());
        assert_eq!(h.handled.load(Ordering::SeqCst), 1);
        assert_eq!(h.writes.load(Ordering::SeqCst), 1);
    }
    Ok(())
}

#[tokio::test]
async fn stop_deadline_reports_unconfirmed_handling_or_persistence() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    for during_handler in [true, false] {
        let (late, gate) = oneshot::channel();
        let (writes, handlers) = if during_handler {
            (vec![], vec![Handle::Gate(gate)])
        } else {
            (vec![Write::Gate(gate)], vec![Handle::Complete])
        };
        let mut h = launch(
            lease.clone(),
            pages.clone(),
            writes,
            handlers,
            options(2).with_drain_timeout(Duration::ZERO),
        );
        event(&mut h, "read").await;
        event(&mut h, "handler").await;
        if !during_handler {
            event(&mut h, "checkpoint").await;
        }
        h.control.stop();
        let report = h.task.await??;
        assert_eq!(report.outcome(), LeaseRunOutcome::DrainTimedOut);
        assert_eq!(report.confirmed_batches(), 0);
        assert_eq!(report.lease().checkpoint(), lease.checkpoint());
        assert_eq!(
            report.phase(),
            Some(if during_handler {
                LeaseRunPhase::Handling
            } else {
                LeaseRunPhase::Checkpointing
            })
        );
        assert!(report.candidate().is_some());
        assert!(late.send(()).is_err());
        assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    }
    Ok(())
}

#[tokio::test]
async fn idle_page_checkpoints_without_handler_and_is_not_termination() -> Result<(), Box<dyn Error>>
{
    let (lease, pages) = pages().await?;
    let h = launch(
        lease,
        vec![pages[3].clone(), pages[3].clone()],
        vec![Write::Confirm, Write::Confirm],
        vec![],
        options(2),
    );
    let report = h.task.await??;
    assert_eq!(report.confirmed_batches(), 2);
    assert_eq!(h.reads.load(Ordering::SeqCst), 2);
    assert_eq!(h.handled.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn public_owned_lease_path_decodes_and_confirms_a_real_driver_page(
) -> Result<(), Box<dyn Error>> {
    let processor = setup(1).await?;
    let read = ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning);
    let reader = processor.engine.open_reader(read.clone()).await?;
    let lease = OwnedLease::new(
        "lease",
        "owner",
        NonZeroU64::new(1).unwrap(),
        "initial",
        FeedRange::full(),
        reader.to_continuation_token()?,
    )?;
    let (tx, _events) = mpsc::unbounded_channel();
    let durable = Arc::new(Mutex::new(lease.clone()));
    let mut store = Store {
        actions: vec![Write::Confirm].into(),
        writes: Arc::new(AtomicU32::new(0)),
        durable: durable.clone(),
        events: tx,
    };
    let report = processor
        .run_owned_lease(
            lease,
            read.with_continuation(ContinuationToken::from_string("c1.invalid".into())),
            &LeaseControl::default(),
            &options(1),
            &mut store,
            |page: ChangeFeedPage<Document>| async move {
                let mut ids: Vec<_> = page
                    .items()
                    .iter()
                    .map(|change| change.current().unwrap().id.as_str())
                    .collect();
                ids.sort_unstable();
                assert_eq!(ids, vec!["item-0", "item-1", "item-2"]);
                assert!(page.diagnostics().effective_status().is_some());
                Ok(())
            },
        )
        .await?;
    assert_eq!(report.confirmed_batches(), 1);
    assert_eq!(
        report.lease().checkpoint(),
        durable.lock().unwrap().checkpoint()
    );
    Ok(())
}

#[tokio::test]
async fn modes_starts_and_continuations_follow_driver_semantics() -> Result<(), Box<dyn Error>> {
    let processor = setup(1).await?;
    let latest = ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    let mut reader = processor.engine.open_reader(latest.clone()).await?;
    let first = reader.read_page().await?;
    let mut resumed = processor
        .engine
        .open_reader(
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Now)
                .with_continuation(first.continuation().clone())
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap())),
        )
        .await?;
    let second: ChangeFeedPage<Document> = resumed.read_page().await?.try_into()?;
    assert_eq!(second.items().len(), 1);
    assert_ne!(second.items()[0].current().unwrap().id, "item-0");
    let avad = ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Now)
        .with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut avad_reader = processor.engine.open_reader(avad.clone()).await?;
    let initial = avad_reader.read_page().await?;
    assert_eq!(initial.state(), BatchState::Idle);
    let mixed = avad.with_continuation(first.continuation().clone());
    assert!(
        processor.engine.open_reader(mixed).await.is_err(),
        "mode-mismatched continuation must fail"
    );
    let rejected = ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
        .with_mode(ChangeFeedMode::AllVersionsAndDeletes);
    let mut reader = processor.engine.open_reader(rejected).await?;
    assert!(
        reader.read_page().await.is_err(),
        "AVAD Beginning must retain service rejection"
    );
    assert!(reader.to_continuation_token().is_err());
    assert!(
        reader.read_page().await.is_err(),
        "failed reader must not skip forward"
    );
    Ok(())
}

#[test]
fn explicit_text_binary_and_presplit_body_shapes() {
    let envelope = json!({"Documents": [
        {"current": {"id": "a"}, "metadata": {"lsn": 1}},
        {"previous": {"id": "deleted"}, "metadata": {"operationType": "delete"}}
    ]});
    let text = serde_json::to_vec(&envelope).unwrap();
    let binary = azure_data_cosmos_driver::binary_json::to_vec(&envelope).unwrap();
    for bytes in [text, binary] {
        let events =
            decode_items::<Document>(ResponseBody::from_bytes(bytes), StatusCode::Ok).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].current().unwrap().id, "a");
        assert!(events[1].current().is_none());
        assert_eq!(events[1].previous().unwrap().id, "deleted");
    }
    let events = decode_items::<Document>(
        ResponseBody::Items(vec![
            Bytes::from_static(br#"{"current":{"id":"a"}}"#),
            Bytes::from(
                azure_data_cosmos_driver::binary_json::to_vec(&json!({"current":{"id":"b"}}))
                    .unwrap(),
            ),
        ]),
        StatusCode::Ok,
    )
    .unwrap();
    assert_eq!(
        events
            .iter()
            .map(|e| e.current().unwrap().id.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "b"]
    );
    assert!(decode_items::<Document>(ResponseBody::NoPayload, StatusCode::Ok).is_err());
    assert!(
        decode_items::<Document>(ResponseBody::NoPayload, StatusCode::NotModified)
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn empty_non_idle_page_is_delivered_not_treated_as_idle() -> Result<(), Box<dyn Error>> {
    let (processor, driver) = setup_with_driver(1).await?;
    let (lease, _pages) = pages().await?;
    let container = driver
        .resolve_container("feed-db", "feed", OperationOptions::default())
        .await?;
    let response = driver
        .execute_operation(
            CosmosOperation::query_items(container, Some(FeedRange::full())).with_body(
                serde_json::to_vec(&json!({"query": "SELECT * FROM c WHERE c.id = 'absent'"}))?,
            ),
            OperationOptions::default(),
        )
        .await?
        .ok_or("query should return a successful empty page")?;
    let raw = RawChangeFeedPage::from((response, lease.checkpoint().clone()));
    assert_eq!(raw.state(), BatchState::Page);
    let typed: ChangeFeedPage<Document> = raw.clone().try_into()?;
    assert!(typed.items().is_empty());
    drop(processor);
    let h = launch(
        lease,
        vec![raw],
        vec![Write::Confirm],
        vec![Handle::Complete],
        options(1),
    );
    assert_eq!(h.task.await??.confirmed_batches(), 1);
    assert_eq!(h.handled.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test]
async fn exhausted_safe_retries_and_source_failure_are_errors() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let h = launch(
        lease.clone(),
        pages.clone(),
        vec![Write::Retryable, Write::Retryable],
        vec![Handle::Complete],
        options(2).with_checkpoint_attempts(NonZeroU32::new(2).unwrap()),
    );
    let error = h.task.await?.unwrap_err();
    assert_eq!(error.report().confirmed_batches(), 0);
    assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
    assert_eq!(h.handled.load(Ordering::SeqCst), 1);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    assert_eq!(h.writes.load(Ordering::SeqCst), 2);
    assert!(error.error().diagnostics().is_some());
    let h = launch(lease.clone(), vec![], vec![], vec![], options(1));
    let error = h.task.await?.unwrap_err();
    assert_eq!(error.report().phase(), Some(LeaseRunPhase::Reading));
    assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
    assert!(error.report().candidate().is_none());
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn ownership_loss_during_persistence_retains_an_unconfirmed_candidate(
) -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (late_commit, persistence) = oneshot::channel();
    let mut h = launch(
        lease.clone(),
        pages,
        vec![Write::Gate(persistence)],
        vec![Handle::Complete],
        options(2),
    );
    for expected in ["read", "handler", "checkpoint"] {
        event(&mut h, expected).await;
    }
    h.control.lose_ownership();
    let report = h.task.await??;
    assert_eq!(report.outcome(), LeaseRunOutcome::OwnershipLost);
    assert_eq!(report.phase(), Some(LeaseRunPhase::Checkpointing));
    assert_eq!(report.lease().checkpoint(), lease.checkpoint());
    assert_eq!(report.confirmed_batches(), 0);
    assert!(report.candidate().is_some());
    assert!(late_commit.send(()).is_err());
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    Ok(())
}

#[tokio::test]
async fn preexisting_stop_and_total_deadline_prevent_work() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let h = launch(lease.clone(), pages.clone(), vec![], vec![], options(1));
    h.control.stop();
    assert_eq!(h.task.await??.outcome(), LeaseRunOutcome::StoppedDrained);
    assert_eq!(h.reads.load(Ordering::SeqCst), 0);
    let h = launch(
        lease,
        pages,
        vec![],
        vec![],
        options(1).with_max_duration(Duration::ZERO),
    );
    assert_eq!(h.task.await??.outcome(), LeaseRunOutcome::RunTimedOut);
    assert_eq!(h.reads.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn avad_reader_delivers_created_envelope_after_initial_idle() -> Result<(), Box<dyn Error>> {
    use azure_data_cosmos_driver::models::{ItemReference, PartitionKey};
    let (processor, driver) = setup_with_driver(1).await?;
    let mut reader = processor
        .engine
        .open_reader(
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Now)
                .with_mode(ChangeFeedMode::AllVersionsAndDeletes),
        )
        .await?;
    assert_eq!(reader.read_page().await?.state(), BatchState::Idle);
    let container = driver
        .resolve_container("feed-db", "feed", OperationOptions::default())
        .await?;
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk"),
                "new-item",
            ))
            .with_body(serde_json::to_vec(
                &json!({"id":"new-item","pk":"pk","value":10}),
            )?),
            OperationOptions::default(),
        )
        .await?;
    let raw = reader.read_page().await?;
    let page: ChangeFeedPage<Document> = raw.try_into()?;
    assert_eq!(page.items().len(), 1);
    assert_eq!(page.items()[0].current().unwrap().id, "new-item");
    assert_eq!(
        page.items()[0].operation_type(),
        Some(super::ChangeFeedOperationType::Create)
    );
    assert!(page.items()[0].previous().is_none());
    assert!(!page.continuation().is_empty());
    Ok(())
}

#[tokio::test]
async fn read_admission_options_are_forwarded() -> Result<(), Box<dyn Error>> {
    use azure_data_cosmos_driver::options::PlanOptions;
    let processor = setup(1).await?;
    let reader = ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning)
        .with_plan_options(PlanOptions::default().with_max_fan_out(0));
    let error = processor
        .engine
        .open_reader(reader)
        .await
        .err()
        .ok_or("fan-out guard must reject")?;
    assert_eq!(
        error.status(),
        azure_data_cosmos_driver::error::status_codes::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED
    );
    Ok(())
}

#[tokio::test]
async fn unwinding_callback_panics_are_failures_without_checkpoint_or_next_read(
) -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    for action in [Handle::PanicOnCall, Handle::PanicOnPoll] {
        let h = launch(
            lease.clone(),
            pages.clone(),
            vec![],
            vec![action],
            options(2),
        );
        let error = h.task.await?.unwrap_err();
        assert_eq!(error.report().outcome(), LeaseRunOutcome::Failed);
        assert_eq!(error.report().phase(), Some(LeaseRunPhase::Handling));
        assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
        assert_eq!(error.report().candidate(), Some(pages[0].continuation()));
        assert!(error.error().to_string().contains("panicked"));
        assert!(error.error().diagnostics().is_some());
        assert_eq!(h.writes.load(Ordering::SeqCst), 0);
        assert_eq!(h.reads.load(Ordering::SeqCst), 1);
        assert_eq!(h.durable.lock().unwrap().checkpoint(), lease.checkpoint());
    }
    Ok(())
}

#[tokio::test]
async fn aborting_pending_callback_does_not_persist_or_advance() -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (late_ack, completion) = oneshot::channel();
    let mut h = launch(
        lease.clone(),
        pages,
        vec![],
        vec![Handle::Gate(completion)],
        options(2),
    );
    event(&mut h, "read").await;
    event(&mut h, "handler").await;
    h.task.abort();
    assert!(h.task.await.unwrap_err().is_cancelled());
    assert!(late_ack.send(()).is_err());
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    assert_eq!(h.durable.lock().unwrap().checkpoint(), lease.checkpoint());
    Ok(())
}

#[tokio::test]
async fn cancelled_callback_completion_is_reported_as_failure_without_checkpoint(
) -> Result<(), Box<dyn Error>> {
    let (lease, pages) = pages().await?;
    let (sender, completion) = oneshot::channel();
    let mut h = launch(
        lease.clone(),
        pages,
        vec![],
        vec![Handle::Gate(completion)],
        options(2),
    );
    event(&mut h, "read").await;
    event(&mut h, "handler").await;
    drop(sender);
    let error = h.task.await?.unwrap_err();
    assert_eq!(error.report().phase(), Some(LeaseRunPhase::Handling));
    assert_eq!(error.report().lease().checkpoint(), lease.checkpoint());
    assert!(error
        .error()
        .to_string()
        .contains("handler completion channel closed"));
    assert_eq!(h.writes.load(Ordering::SeqCst), 0);
    assert_eq!(h.reads.load(Ordering::SeqCst), 1);
    Ok(())
}
