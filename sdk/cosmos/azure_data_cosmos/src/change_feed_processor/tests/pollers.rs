// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    containers, containers_with_observer, containers_with_partitions, lease_records, pin_start,
    ChangeFeedProcessorContext,
};
use crate::{
    feed::ContinuationToken,
    models::ChangeFeedItem,
    options::{ChangeFeedMode, MaxItemCountHint},
    ChangeFeedCheckpointPolicy, ChangeFeedProcessorBuilder, CosmosError, CosmosStatus,
};
use azure_core::http::{Method, Request, StatusCode};
use azure_data_cosmos_driver::in_memory_emulator::RequestObserver;
use futures::{
    channel::{mpsc, oneshot},
    StreamExt,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashSet,
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

type Entered = (String, oneshot::Sender<()>);
type ReportedError = (Option<String>, String);
type ReportedStatus = (Option<String>, StatusCode);

#[derive(Deserialize)]
struct NumericId {
    #[serde(rename = "id")]
    _id: u64,
}

#[tokio::test]
async fn callback_read_diagnostics_survive_processor_shutdown() -> Result<(), Box<dyn Error>> {
    static OBSERVED: Mutex<Option<oneshot::Sender<ChangeFeedProcessorContext>>> = Mutex::new(None);
    let (source, leases) = containers().await?;
    source
        .create_item("one", "one", json!({"id":"one"}), None)
        .await?;
    let (sent, received) = oneshot::channel();
    *OBSERVED.lock().unwrap() = Some(sent);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases,
        "diagnostics",
        "worker",
    )
    .with_start_from_beginning()
    .handle_change_feed(|context, changes: Vec<ChangeFeedItem<Value>>| async move {
        assert_eq!(changes.len(), 1);
        if let Some(sent) = OBSERVED.lock().unwrap().take() {
            sent.send(context).unwrap();
        }
        Ok(())
    })
    .build()?;
    processor.start().await?;
    let context = tokio::time::timeout(Duration::from_secs(5), received).await??;
    let diagnostics = context.diagnostics().expect("batch diagnostics");
    let activity = diagnostics.activity_id().clone();
    processor.stop().await?;
    drop(processor);
    assert!(diagnostics.is_completed());
    assert!(!diagnostics.is_failure());
    assert!(!diagnostics.requests().is_empty());
    let retained = context.diagnostics().unwrap();
    assert!(Arc::ptr_eq(&diagnostics, &retained));
    assert_eq!(retained.activity_id(), &activity);
    Ok(())
}

#[tokio::test]
async fn deserialization_failure_reports_without_invoking_callback() -> Result<(), Box<dyn Error>> {
    static ERRORS: Mutex<Option<oneshot::Sender<ChangeFeedProcessorContext>>> = Mutex::new(None);
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let (source, leases) = containers().await?;
    source
        .create_item("one", "one", json!({"id":"one"}), None)
        .await?;
    let (error, reported) = oneshot::channel();
    *ERRORS.lock().unwrap() = Some(error);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "decode",
        "worker",
    )
    .with_start_from_beginning()
    .handle_error(|context, _| async move {
        if let Some(error) = ERRORS.lock().unwrap().take() {
            error.send(context.unwrap()).unwrap();
        }
    })
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<NumericId>>| async {
        CALLS.fetch_add(1, Ordering::SeqCst);
        Ok(())
    })
    .build()?;
    processor.start().await?;
    let context = tokio::time::timeout(Duration::from_secs(5), reported).await??;
    processor.abort().await?;
    let diagnostics = context
        .diagnostics()
        .expect("failed decode retains the read diagnostics");
    assert!(diagnostics.is_completed());
    assert!(!diagnostics.requests().is_empty());
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert!(lease_records(&leases)
        .await?
        .iter()
        .find(|r| r["id"] == context.lease_token())
        .unwrap()
        .get("continuation")
        .is_none());
    Ok(())
}

#[tokio::test]
async fn callback_panic_is_reported_without_saving_progress() -> Result<(), Box<dyn Error>> {
    static ERRORS: Mutex<Option<oneshot::Sender<String>>> = Mutex::new(None);
    let (source, leases) = containers().await?;
    source
        .create_item("one", "one", json!({"id":"one"}), None)
        .await?;
    let (error, reported) = oneshot::channel();
    *ERRORS.lock().unwrap() = Some(error);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "panic",
        "worker",
    )
    .with_start_from_beginning()
    .handle_error(|context, error| async move {
        if error.to_string().contains("Change feed callback panicked.") {
            if let Some(error) = ERRORS.lock().unwrap().take() {
                error
                    .send(context.unwrap().lease_token().to_owned())
                    .unwrap();
            }
        }
    })
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<Value>>| async { panic!("application panic") })
    .build()?;
    processor.start().await?;
    let lease = tokio::time::timeout(Duration::from_secs(5), reported).await??;
    processor.abort().await?;
    assert!(lease_records(&leases)
        .await?
        .iter()
        .find(|r| r["id"] == lease)
        .unwrap()
        .get("continuation")
        .is_none());
    Ok(())
}

#[derive(Debug, Default)]
struct FeedReads(AtomicUsize);

impl RequestObserver for FeedReads {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get
            && request.url().path().ends_with("/docs")
            && request
                .headers()
                .iter()
                .any(|(name, _)| name.as_str() == "a-im")
        {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
}

#[tokio::test]
async fn poller_limit_covers_receive_callback_and_drain_in_both_modes() -> Result<(), Box<dyn Error>>
{
    static ENTERED: Mutex<Option<mpsc::UnboundedSender<Entered>>> = Mutex::new(None);
    for mode in [
        ChangeFeedMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes,
    ] {
        for policy in [
            ChangeFeedCheckpointPolicy::EveryBatch,
            ChangeFeedCheckpointPolicy::Coalesced {
                max_interval: Duration::from_secs(60),
            },
        ] {
            for limit in [1, 2] {
                let reads = Arc::new(FeedReads::default());
                let (source, leases) = containers_with_observer(4, Some(reads.clone())).await?;
                if policy != ChangeFeedCheckpointPolicy::EveryBatch {
                    pin_start(mode, &source, &leases, "pool", 4).await?;
                }
                let (entered, mut callbacks) = mpsc::unbounded();
                *ENTERED.lock().unwrap() = Some(entered);
                let mut builder = ChangeFeedProcessorBuilder::new(
                    mode,
                    source.clone(),
                    leases.clone(),
                    "pool",
                    "worker",
                )
                .with_feed_poll_delay(Duration::from_millis(5))
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::MIN))
                .with_checkpoint_policy(policy);
                if limit > 1 {
                    builder = builder.with_max_concurrency(NonZeroU32::new(limit).unwrap());
                }
                let processor = builder
                    .handle_change_feed(|context, changes: Vec<ChangeFeedItem<Value>>| async move {
                        assert_eq!(changes.len(), 1);
                        let diagnostics = context.diagnostics().expect("batch read diagnostics");
                        assert!(diagnostics.is_completed());
                        assert!(!diagnostics.is_failure());
                        assert!(!diagnostics.requests().is_empty());
                        let (release, released) = oneshot::channel();
                        ENTERED
                            .lock()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .unbounded_send((context.lease_token().to_owned(), release))
                            .unwrap();
                        released.await.unwrap();
                        Ok(())
                    })
                    .build()?;
                processor.start().await?;
                tokio::time::timeout(Duration::from_secs(5), async {
                    loop {
                        let records = lease_records(&leases).await.unwrap();
                        let owned: Vec<_> = records
                            .iter()
                            .filter(|r| r.get("range").is_some())
                            .collect();
                        if owned.len() == 4 && owned.iter().all(|r| r.get("continuation").is_some())
                        {
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                })
                .await?;
                for index in 0..64 {
                    let id = format!("item-{index}");
                    source
                        .create_item(id.clone(), &id, json!({"id":id}), None)
                        .await?;
                }
                let mut held = Vec::new();
                for _ in 0..limit {
                    held.push(
                        tokio::time::timeout(Duration::from_secs(5), callbacks.next())
                            .await?
                            .unwrap(),
                    );
                }
                assert_eq!(
                    held.iter().map(|(id, _)| id).collect::<HashSet<_>>().len(),
                    limit as usize
                );
                let reads_while_full = reads.0.load(Ordering::SeqCst);
                assert!(
                    tokio::time::timeout(Duration::from_millis(50), callbacks.next())
                        .await
                        .is_err()
                );
                assert_eq!(
                    reads.0.load(Ordering::SeqCst),
                    reads_while_full,
                    "busy SDK pollers must not prefetch"
                );
                if limit > 1 {
                    let slow = held[0].0.clone();
                    held.pop().unwrap().1.send(()).unwrap();
                    let next = tokio::time::timeout(Duration::from_secs(5), callbacks.next())
                        .await?
                        .unwrap();
                    assert_ne!(next.0, slow, "a held lease must not consume another poller");
                    held.push(next);
                }
                let mut stopping = Box::pin(processor.stop());
                assert!(futures::poll!(stopping.as_mut()).is_pending());
                for (_, release) in held {
                    release.send(()).unwrap();
                }
                tokio::time::timeout(Duration::from_secs(5), stopping).await??;
                assert!(!processor.is_started());
                assert!(callbacks.try_recv().is_err());
                assert!(lease_records(&leases)
                    .await?
                    .iter()
                    .filter(|r| r.get("range").is_some())
                    .all(|r| r.get("owner").is_none()));
                *ENTERED.lock().unwrap() = None;
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn slow_error_hook_does_not_block_data_and_graceful_stop_drains_it(
) -> Result<(), Box<dyn Error>> {
    static ERRORS: Mutex<Option<mpsc::UnboundedSender<ReportedError>>> = Mutex::new(None);
    static ERROR_GATE: Mutex<Option<oneshot::Receiver<()>>> = Mutex::new(None);
    static CALLBACKS: Mutex<Option<mpsc::UnboundedSender<Entered>>> = Mutex::new(None);
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let (source, leases) = containers_with_partitions(4).await?;
    for index in 0..64 {
        let id = format!("item-{index}");
        source
            .create_item(id.clone(), &id, json!({"id":id}), None)
            .await?;
    }
    let (errors, mut reported) = mpsc::unbounded();
    let (entered, mut callbacks) = mpsc::unbounded();
    let (release_error, error_gate) = oneshot::channel();
    *ERRORS.lock().unwrap() = Some(errors);
    *CALLBACKS.lock().unwrap() = Some(entered);
    *ERROR_GATE.lock().unwrap() = Some(error_gate);
    CALLS.store(0, Ordering::SeqCst);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "errors",
        "worker",
    )
    .with_start_from_beginning()
    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::MIN))
    .handle_error(|context, error| async move {
        ERRORS
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .unbounded_send((
                context.map(|c| c.lease_token().to_owned()),
                error.to_string(),
            ))
            .unwrap();
        let gate = ERROR_GATE.lock().unwrap().take();
        if let Some(gate) = gate {
            let _ = gate.await;
        }
    })
    .handle_change_feed(|context, _: Vec<ChangeFeedItem<Value>>| async move {
        if CALLS.fetch_add(1, Ordering::SeqCst) == 0 {
            return Err(CosmosError::builder()
                .with_status(CosmosStatus::new(StatusCode::BadRequest))
                .with_message("Application rejected this batch.")
                .build());
        }
        let (release, released) = oneshot::channel();
        CALLBACKS
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .unbounded_send((context.lease_token().to_owned(), release))
            .unwrap();
        let _ = released.await;
        Ok(())
    })
    .build()?;
    processor.start().await?;
    let (failed_lease, error) = tokio::time::timeout(Duration::from_secs(5), reported.next())
        .await?
        .unwrap();
    assert!(error.contains("Application rejected this batch."));
    let failed_lease = failed_lease.unwrap();
    let (successful_lease, release) =
        tokio::time::timeout(Duration::from_secs(5), callbacks.next())
            .await?
            .unwrap();
    assert_ne!(failed_lease, successful_lease);
    tokio::time::timeout(Duration::from_secs(5), async {
        while lease_records(&leases)
            .await
            .unwrap()
            .iter()
            .any(|r| r["id"] == failed_lease && r.get("owner").is_some())
        {
            tokio::task::yield_now().await;
        }
    })
    .await?;
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    release.send(()).unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(50), stopping.as_mut())
            .await
            .is_err()
    );
    release_error.send(()).unwrap();
    tokio::time::timeout(Duration::from_secs(5), stopping).await??;
    let records = lease_records(&leases).await?;
    assert!(records
        .iter()
        .find(|r| r["id"] == failed_lease)
        .unwrap()
        .get("continuation")
        .is_none());
    assert!(records
        .iter()
        .find(|r| r["id"] == successful_lease)
        .unwrap()
        .get("continuation")
        .is_some());
    *ERRORS.lock().unwrap() = None;
    *CALLBACKS.lock().unwrap() = None;
    Ok(())
}

#[tokio::test]
async fn driver_errors_reach_async_hook_and_forced_stop_cancels_it() -> Result<(), Box<dyn Error>> {
    static ENTERED: Mutex<Option<oneshot::Sender<ReportedStatus>>> = Mutex::new(None);
    static GATE: Mutex<Option<oneshot::Receiver<()>>> = Mutex::new(None);
    let (source, leases) = containers().await?;
    let (entered, reported) = oneshot::channel();
    let (release, gate) = oneshot::channel();
    *ENTERED.lock().unwrap() = Some(entered);
    *GATE.lock().unwrap() = Some(gate);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases,
        "driver-errors",
        "worker",
    )
    .with_start_continuation(ContinuationToken::from_string(
        "invalid-continuation".into(),
    ))
    .handle_change_feed(|_, _: Vec<ChangeFeedItem<Value>>| async {
        panic!("invalid continuation cannot deliver data")
    })
    .handle_error(|context, error| async move {
        if let Some(entered) = ENTERED.lock().unwrap().take() {
            entered
                .send((
                    context.map(|c| c.lease_token().to_owned()),
                    error.status().status_code(),
                ))
                .unwrap();
        }
        let gate = GATE.lock().unwrap().take();
        if let Some(gate) = gate {
            let _ = gate.await;
        }
    })
    .build()?;
    processor.start().await?;
    let (lease, status) = tokio::time::timeout(Duration::from_secs(5), reported).await??;
    assert!(lease.is_some());
    assert_eq!(status, StatusCode::BadRequest);
    tokio::time::timeout(Duration::from_secs(5), processor.abort()).await??;
    assert!(release.is_canceled());
    assert!(!processor.is_started());
    Ok(())
}
