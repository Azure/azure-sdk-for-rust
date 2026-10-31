// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::{signal, BatchReceipt, CheckpointPolicy, ProcessorOptions};
use super::{
    fixture_with_observer, insert, next, start, topology, ChangeFeedFactoryMode, RequestObserver,
};
use crate::{models::MaxItemCountHint, Result};
use azure_core::http::Request;
use futures::{channel::oneshot, FutureExt};
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    num::NonZeroU32,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Debug, Default)]
struct Reads(AtomicUsize);

impl RequestObserver for Reads {
    fn on_request(&self, request: &Request) {
        if request
            .url()
            .path()
            .trim_end_matches('/')
            .ends_with("/docs")
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
async fn reservations_queue_and_processing_share_capacity_and_fair_turns() -> Result<()> {
    for policy in [
        CheckpointPolicy::EveryBatch,
        CheckpointPolicy::Coalesced {
            max_interval: Duration::from_millis(100),
        },
    ] {
        let reads = Arc::new(Reads::default());
        let (context, _) =
            fixture_with_observer(ChangeFeedFactoryMode::Incremental, 4, Some(reads.clone()))
                .await?;
        let ranges = context
            .list()
            .await?
            .into_iter()
            .map(|lease| lease.base.range)
            .collect::<Vec<_>>();
        let keys = topology::keys(&ranges);
        topology::publish(&context, &keys, 0..3, &mut BTreeMap::new()).await?;
        let options = context
            .options
            .clone()
            .with_max_outstanding_batches(2)
            .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()))
            .with_checkpoint_policy(policy);
        let admission = context.source_driver.batch_admission();
        admission.observe_limit(2);
        let first = admission.acquire().await;
        let second = admission.acquire().await;
        let processor = start(&context, options).await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            while admission.counts() != (2, 4) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(reads.0.load(Ordering::SeqCst), 0);
        drop((first, second));
        let mut active = VecDeque::from([next(&processor).await, next(&processor).await]);
        tokio::time::timeout(Duration::from_secs(5), async {
            while admission.counts() != (2, 2) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(reads.0.load(Ordering::SeqCst), 2);
        assert!(processor.next_batch().now_or_never().is_none());
        let before = context.list().await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let after = context.list().await.unwrap();
                if after
                    .iter()
                    .zip(&before)
                    .all(|(a, b)| a.renewed_at != b.renewed_at)
                {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(reads.0.load(Ordering::SeqCst), 2);
        let mut order = Vec::new();
        for index in 0..8 {
            assert_eq!(admission.counts().0, 2);
            let (id, response, receipt) = active.pop_front().unwrap().into_parts();
            drop(response);
            receipt.acknowledge().await?;
            order.push(id.clone());
            active.push_back(next(&processor).await);
            if index == 0 {
                // This lease is parked behind peers holding every data permit.
                tokio::time::timeout(Duration::from_secs(5), async {
                    loop {
                        let leases = context.list().await.unwrap();
                        if leases
                            .iter()
                            .find(|l| l.base.record.id == id)
                            .unwrap()
                            .base
                            .continuation
                            .is_some()
                        {
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                })
                .await
                .unwrap();
                assert_eq!(admission.counts().0, 2);
            }
        }
        assert_eq!(order[..4].iter().collect::<HashSet<_>>().len(), 4);
        assert_eq!(order[..4], order[4..]);
        drop(active);
        processor.abort().await?;
        assert_eq!(admission.counts(), (0, 0));
    }
    Ok(())
}

#[tokio::test]
async fn highest_worker_limit_survives_stop_and_restart() -> Result<()> {
    let (context, _) = fixture_with_observer(ChangeFeedFactoryMode::Incremental, 1, None).await?;
    for id in ["one", "two", "three"] {
        insert(&context, id).await?;
    }
    let options = context
        .options
        .clone()
        .with_max_outstanding_batches(1)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));
    let first = start(&context, options.clone()).await?;
    let held = next(&first).await;
    let mut second_options = options.clone().with_max_outstanding_batches(2);
    second_options.processor_name = "second-workload".into();
    let second = start(&context, second_options).await?;
    let second_held = next(&second).await;
    let admission = context.source_driver.batch_admission();
    assert_eq!(admission.counts().0, 2);
    second.abort().await?;
    drop(second_held);
    assert_eq!(admission.counts().0, 1);
    let spare = admission
        .acquire()
        .now_or_never()
        .expect("capacity retained after worker stop");
    assert_eq!(admission.counts().0, 2);
    drop(spare);
    let (_, response, receipt) = held.into_parts();
    drop(response);
    receipt.acknowledge().await?;
    let held = next(&first).await;
    assert_eq!(admission.counts().0, 1);
    drop(held);
    first.abort().await?;
    assert_eq!(admission.counts(), (0, 0));
    let reservation = admission.acquire().await;
    let restarted = start(&context, options).await?;
    let held = next(&restarted).await;
    assert_eq!(admission.counts().0, 2);
    drop(held);
    restarted.abort().await?;
    drop(reservation);
    assert_eq!(admission.counts(), (0, 0));
    Ok(())
}

#[tokio::test]
async fn zero_is_rejected_before_admission_or_bootstrap() -> Result<()> {
    let (context, _) = fixture_with_observer(ChangeFeedFactoryMode::Incremental, 1, None).await?;
    let options = ProcessorOptions::new(ChangeFeedFactoryMode::Incremental, "unused", "worker")
        .with_max_outstanding_batches(0);
    assert!(start(&context, options).await.is_err());
    assert_eq!(context.source_driver.batch_admission().counts(), (0, 0));
    Ok(())
}

#[tokio::test]
async fn payload_capacity_is_released_before_a_durable_checkpoint_reply() -> Result<()> {
    let admission = Arc::new(crate::change_feed_processor::BatchAdmission::default());
    admission.observe_limit(1);
    let (sender, received) = oneshot::channel();
    let (_cancel, cancelled) = signal();
    let receipt = BatchReceipt {
        sender,
        cancelled,
        permit: Some(admission.acquire().await),
    };
    let mut checkpointing = Box::pin(receipt.checkpoint());
    assert!(futures::poll!(checkpointing.as_mut()).is_pending());
    let acknowledgment = received.await.unwrap();
    assert!(acknowledgment.durable);
    assert_eq!(admission.counts(), (0, 0));
    let other_lease = admission.acquire().now_or_never().unwrap();
    assert!(futures::poll!(checkpointing.as_mut()).is_pending());
    acknowledgment.result.send(Ok(())).unwrap();
    checkpointing.await?;
    drop(other_lease);
    assert_eq!(admission.counts(), (0, 0));
    Ok(())
}

#[tokio::test]
async fn parked_leases_stop_without_waiting_for_other_workers_capacity() -> Result<()> {
    for policy in [
        CheckpointPolicy::EveryBatch,
        CheckpointPolicy::Coalesced {
            max_interval: Duration::from_secs(60),
        },
    ] {
        let reads = Arc::new(Reads::default());
        let (context, _) =
            fixture_with_observer(ChangeFeedFactoryMode::Incremental, 4, Some(reads.clone()))
                .await?;
        let admission = context.source_driver.batch_admission();
        admission.observe_limit(1);
        let reservation = admission.acquire().await;
        let processor = start(
            &context,
            context
                .options
                .clone()
                .with_max_outstanding_batches(1)
                .with_checkpoint_policy(policy),
        )
        .await?;
        tokio::time::timeout(Duration::from_secs(5), async {
            while admission.counts() != (1, 4) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        tokio::time::timeout(Duration::from_secs(5), processor.stop())
            .await
            .unwrap()?;
        assert_eq!(admission.counts(), (1, 0));
        assert_eq!(reads.0.load(Ordering::SeqCst), 0);
        assert!(context.list().await?.iter().all(|l| l.base.owner.is_none()));
        drop(reservation);
        assert_eq!(admission.counts(), (0, 0));
    }
    Ok(())
}
