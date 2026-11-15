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
use serde::{Deserialize, Deserializer};
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

static DESERIALIZATION_FAILURES: AtomicUsize = AtomicUsize::new(0);

#[derive(Deserialize)]
struct NumericId {
    #[serde(rename = "id", deserialize_with = "numeric_id")]
    _id: u64,
}

fn numeric_id<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    let result = u64::deserialize(deserializer);
    if result.is_err() {
        DESERIALIZATION_FAILURES.fetch_add(1, Ordering::SeqCst);
    }
    result
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
        |context, changes: Vec<ChangeFeedItem<Value>>| async move {
            assert_eq!(changes.len(), 1);
            if let Some(sent) = OBSERVED.lock().unwrap().take() {
                sent.send(context).unwrap();
            }
            Ok(())
        },
    )
    .with_start_from_beginning()
    .build()?;
    processor.start().await?;
    let context = tokio::time::timeout(Duration::from_secs(5), received).await??;
    let diagnostics = context.diagnostics();
    let activity = diagnostics.activity_id().clone();
    processor.stop().await?;
    drop(processor);
    assert!(diagnostics.is_completed());
    assert!(!diagnostics.is_failure());
    assert!(!diagnostics.requests().is_empty());
    let retained = context.diagnostics();
    assert!(Arc::ptr_eq(&diagnostics, &retained));
    assert_eq!(retained.activity_id(), &activity);
    let weak = Arc::downgrade(&diagnostics);
    drop(retained);
    drop(diagnostics);
    drop(context);
    assert!(
        weak.upgrade().is_none(),
        "completed diagnostics must not be retained by the processor"
    );
    Ok(())
}

#[tokio::test]
async fn deserialization_failure_releases_lease_without_invoking_callback(
) -> Result<(), Box<dyn Error>> {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let (source, leases) = containers().await?;
    source
        .create_item("one", "one", json!({"id":"one"}), None)
        .await?;
    DESERIALIZATION_FAILURES.store(0, Ordering::SeqCst);
    CALLS.store(0, Ordering::SeqCst);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "decode",
        "worker",
        |_, _: Vec<ChangeFeedItem<NumericId>>| async {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Ok(())
        },
    )
    .with_start_from_beginning()
    .build()?;
    processor.start().await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if DESERIALIZATION_FAILURES.load(Ordering::SeqCst) > 0
                && lease_records(&leases)
                    .await
                    .unwrap()
                    .iter()
                    .any(|r| r.get("range").is_some() && r.get("owner").is_none())
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await?;
    processor.stop().await?;
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert!(lease_records(&leases)
        .await?
        .iter()
        .find(|r| r.get("range").is_some())
        .unwrap()
        .get("continuation")
        .is_none());
    Ok(())
}

#[tokio::test]
async fn callback_panic_does_not_save_progress() -> Result<(), Box<dyn Error>> {
    static ENTERED: Mutex<Option<oneshot::Sender<String>>> = Mutex::new(None);
    let (source, leases) = containers().await?;
    source
        .create_item("one", "one", json!({"id":"one"}), None)
        .await?;
    let (entered, received) = oneshot::channel();
    *ENTERED.lock().unwrap() = Some(entered);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "panic",
        "worker",
        |context, _: Vec<ChangeFeedItem<Value>>| async move {
            if let Some(entered) = ENTERED.lock().unwrap().take() {
                entered.send(context.lease_token().to_owned()).unwrap();
            }
            panic!("application panic")
        },
    )
    .with_start_from_beginning()
    .build()?;
    processor.start().await?;
    let lease = tokio::time::timeout(Duration::from_secs(5), received).await??;
    processor.stop().await?;
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
async fn instances_process_independently_with_one_callback_each_in_both_modes(
) -> Result<(), Box<dyn Error>> {
    static ENTERED: Mutex<Option<mpsc::UnboundedSender<Entered>>> = Mutex::new(None);
    for mode in [
        ChangeFeedMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes,
    ] {
        for policy in [
            ChangeFeedCheckpointPolicy::Eager,
            ChangeFeedCheckpointPolicy::Deferred {
                max_interval: Duration::from_secs(60),
            },
        ] {
            for instance_count in [1, 2] {
                let reads = Arc::new(FeedReads::default());
                let (source, leases) = containers_with_observer(2, Some(reads.clone())).await?;
                if policy != ChangeFeedCheckpointPolicy::Eager {
                    pin_start(mode, &source, &leases, "pool", 2).await?;
                }
                let (entered, mut callbacks) = mpsc::unbounded();
                *ENTERED.lock().unwrap() = Some(entered);
                let mut processors = Vec::new();
                for instance in 0..instance_count {
                    let processor = ChangeFeedProcessorBuilder::new(
                        mode,
                        source.clone(),
                        leases.clone(),
                        "pool",
                        format!("worker-{instance}"),
                        |context, changes: Vec<ChangeFeedItem<Value>>| async move {
                            assert_eq!(changes.len(), 1);
                            let diagnostics = context.diagnostics();
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
                        },
                    )
                    .with_feed_poll_delay(Duration::from_millis(5))
                    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::MIN))
                    .with_max_scale_count(2 / instance_count)
                    .with_checkpoint_policy(policy)
                    .build()?;
                    processor.start().await?;
                    processors.push(processor);
                }
                tokio::time::timeout(Duration::from_secs(5), async {
                    loop {
                        let records = lease_records(&leases).await.unwrap();
                        let owned: Vec<_> = records
                            .iter()
                            .filter(|r| r.get("range").is_some())
                            .collect();
                        if owned.len() == 2
                            && owned.iter().all(|r| r.get("continuation").is_some())
                            && owned
                                .iter()
                                .filter_map(|r| r["owner"].as_str())
                                .collect::<HashSet<_>>()
                                .len()
                                == instance_count as usize
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
                for _ in 0..instance_count {
                    held.push(
                        tokio::time::timeout(Duration::from_secs(5), callbacks.next())
                            .await?
                            .unwrap(),
                    );
                }
                assert_eq!(
                    held.iter().map(|(id, _)| id).collect::<HashSet<_>>().len(),
                    instance_count as usize
                );
                let records = lease_records(&leases).await?;
                let owners: HashSet<_> = held
                    .iter()
                    .map(|(id, _)| {
                        records.iter().find(|r| r["id"] == *id).unwrap()["owner"]
                            .as_str()
                            .unwrap()
                    })
                    .collect();
                assert_eq!(owners.len(), instance_count as usize);
                let reads_while_full = reads.0.load(Ordering::SeqCst);
                assert!(
                    tokio::time::timeout(Duration::from_millis(50), callbacks.next())
                        .await
                        .is_err()
                );
                assert_eq!(
                    reads.0.load(Ordering::SeqCst),
                    reads_while_full,
                    "instances with busy callbacks must not prefetch"
                );
                if instance_count > 1 {
                    let slow = held[0].0.clone();
                    held.pop().unwrap().1.send(()).unwrap();
                    let next = tokio::time::timeout(Duration::from_secs(5), callbacks.next())
                        .await?
                        .unwrap();
                    assert_ne!(
                        next.0, slow,
                        "another instance must progress while a callback is held"
                    );
                    held.push(next);
                }
                let mut stopping = Box::pin(futures::future::join_all(
                    processors.iter().map(|processor| processor.stop()),
                ));
                assert!(futures::poll!(stopping.as_mut()).is_pending());
                for (_, release) in held {
                    release.send(()).unwrap();
                }
                for result in tokio::time::timeout(Duration::from_secs(5), stopping).await? {
                    result?;
                }
                assert!(processors.iter().all(|processor| !processor.is_started()));
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
async fn callback_error_does_not_block_other_leases() -> Result<(), Box<dyn Error>> {
    static FAILED: Mutex<Option<oneshot::Sender<String>>> = Mutex::new(None);
    static CALLBACKS: Mutex<Option<mpsc::UnboundedSender<Entered>>> = Mutex::new(None);
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let (source, leases) = containers_with_partitions(4).await?;
    for index in 0..64 {
        let id = format!("item-{index}");
        source
            .create_item(id.clone(), &id, json!({"id":id}), None)
            .await?;
    }
    let (failed, received) = oneshot::channel();
    let (entered, mut callbacks) = mpsc::unbounded();
    *FAILED.lock().unwrap() = Some(failed);
    *CALLBACKS.lock().unwrap() = Some(entered);
    CALLS.store(0, Ordering::SeqCst);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "errors",
        "worker",
        |context, _: Vec<ChangeFeedItem<Value>>| async move {
            if CALLS.fetch_add(1, Ordering::SeqCst) == 0 {
                FAILED
                    .lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .send(context.lease_token().to_owned())
                    .unwrap();
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
        },
    )
    .with_start_from_beginning()
    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::MIN))
    .build()?;
    processor.start().await?;
    let failed_lease = tokio::time::timeout(Duration::from_secs(5), received).await??;
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
    *CALLBACKS.lock().unwrap() = None;
    Ok(())
}

#[tokio::test]
async fn invalid_continuation_releases_lease_without_invoking_callback(
) -> Result<(), Box<dyn Error>> {
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    let (source, leases) = containers().await?;
    CALLS.store(0, Ordering::SeqCst);
    let processor = ChangeFeedProcessorBuilder::new(
        ChangeFeedMode::LatestVersion,
        source,
        leases.clone(),
        "driver-errors",
        "worker",
        |_, _: Vec<ChangeFeedItem<Value>>| async {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Ok(())
        },
    )
    .with_start_continuation(ContinuationToken::from_string(
        "invalid-continuation".into(),
    ))
    .build()?;
    processor.start().await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if lease_records(&leases).await.unwrap().iter().any(|r| {
                r.get("range").is_some()
                    && r.get("renewed_at").is_some()
                    && r.get("owner").is_none()
            }) {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await?;
    processor.stop().await?;
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    assert!(lease_records(&leases)
        .await?
        .iter()
        .filter(|r| r.get("range").is_some())
        .all(|r| r.get("continuation").is_none()));
    assert!(!processor.is_started());
    Ok(())
}
