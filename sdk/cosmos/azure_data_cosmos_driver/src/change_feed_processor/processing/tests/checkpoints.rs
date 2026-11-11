// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::{poll_feed, signal, CheckpointPolicy, SharedState};
use super::{
    fixture_with_observer, insert, next, poll_until, start, ChangeFeedFactoryMode, Context, Lease,
    Processor, RequestObserver, Update,
};
use crate::{
    change_feed_processor::bootstrap::store::ChangeFeedProcessorBootstrapStoredRecord as Stored,
    driver::dataflow::PipelineNodeState,
    in_memory_emulator::Epk,
    models::{ChangeFeedStartFrom, ContinuationToken, FeedRange, MaxItemCountHint, ResolvedToken},
    options::{OperationOptions, PlanOptions},
    Result,
};
use async_lock::Mutex;
use azure_core::{
    http::{Method, Request},
    Bytes,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex as StdMutex,
    },
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts {
    creates: u64,
    acquisitions: u64,
    checkpoints: u64,
    renewals: u64,
    releases: u64,
}

#[derive(Debug, Default)]
struct Writes {
    state: StdMutex<(HashMap<String, Value>, Counts)>,
    feed_reads: AtomicUsize,
}

impl RequestObserver for Writes {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get
            && request.url().path().ends_with("/docs")
            && request
                .headers()
                .iter()
                .any(|(name, _)| name.as_str() == "a-im")
        {
            self.feed_reads.fetch_add(1, Ordering::SeqCst);
        }
        if !matches!(request.method(), Method::Post | Method::Put) {
            return;
        }
        let bytes = Bytes::from(request.body());
        if bytes.is_empty() {
            return;
        }
        let value: Value = crate::models::ResponseBody::from_bytes(bytes)
            .into_single()
            .unwrap();
        if value.get("range").is_none() || value.get("workload").is_none() {
            return;
        }
        let id = value["id"].as_str().unwrap().to_owned();
        let mut state = self.state.lock().unwrap();
        let (previous, counts) = &mut *state;
        if request.method() == Method::Post {
            counts.creates += 1;
        } else if value.get("owner").is_none() {
            counts.releases += 1;
        } else if previous.get(&id).map(|v| &v["incarnation"]) != Some(&value["incarnation"]) {
            counts.acquisitions += 1;
        } else if previous.get(&id).map(|v| &v["continuation"]) != Some(&value["continuation"]) {
            counts.checkpoints += 1;
        } else {
            counts.renewals += 1;
        }
        previous.insert(id, value);
    }
}

impl Writes {
    fn counts(&self) -> Counts {
        self.state.lock().unwrap().1
    }

    fn reset(&self) {
        self.state.lock().unwrap().1 = Counts::default();
    }
}

async fn fixture(
    mode: ChangeFeedFactoryMode,
    policy: CheckpointPolicy,
) -> Result<(Arc<Context>, Arc<Writes>)> {
    let writes = Arc::new(Writes::default());
    let (mut context, _) = fixture_with_observer(mode, 1, Some(writes.clone())).await?;
    let options = &mut Arc::get_mut(&mut context).unwrap().options;
    options.start = ChangeFeedStartFrom::Now;
    options.expiration = Duration::from_secs(60);
    options.checkpoint_policy = policy;
    options.page_size = MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap());
    Ok((context, writes))
}

async fn pinned(context: &Context) -> Result<Processor> {
    let processor = start(
        context,
        context
            .options
            .clone()
            .with_checkpoint_policy(CheckpointPolicy::EveryBatch),
    )
    .await?;
    poll_until(
        &processor,
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if context
                    .list()
                    .await
                    .unwrap()
                    .iter()
                    .all(|l| l.base.continuation.is_some())
                {
                    break;
                }
                tokio::task::yield_now().await;
            }
        }),
    )
    .await
    .unwrap();
    if context.options.checkpoint_policy == CheckpointPolicy::EveryBatch {
        return Ok(processor);
    }
    processor.stop().await?;
    let processor = start(context, context.options.clone()).await?;
    tokio::time::timeout(Duration::from_secs(5), async {
        while context
            .list()
            .await
            .unwrap()
            .iter()
            .any(|lease| lease.base.owner.is_none())
        {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    Ok(processor)
}

async fn record(context: &Context, id: &str) -> Stored<Lease> {
    context.store().read(id).await.unwrap().unwrap()
}

fn coalesced() -> CheckpointPolicy {
    CheckpointPolicy::Coalesced {
        max_interval: Duration::from_secs(60),
    }
}

#[tokio::test]
async fn stop_after_demand_assignment_does_not_start_a_feed_read() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let root = context.list().await?.remove(0);
    let owned = context.acquire(&root).await?;
    let state = Arc::new(SharedState::default());
    let session = state
        .distributor
        .acquire(root.base.record.id.clone(), owned.value.incarnation.clone())?;
    let lease = Arc::new(Mutex::new(owned));
    let (_cancel, cancelled) = signal();
    let (_stop, stopped) = signal();
    let (_force, forced) = signal();
    let mut polling = Box::pin(poll_feed(
        context.clone(),
        state.clone(),
        &session,
        lease.clone(),
        cancelled,
        stopped,
        forced,
    ));
    assert!(futures::poll!(polling.as_mut()).is_pending());
    let mut receiving = Box::pin(state.distributor.next_batch());
    assert!(futures::poll!(receiving.as_mut()).is_pending());
    state.close(false);
    polling.await?;
    assert!(receiving.await.is_none());
    assert_eq!(writes.feed_reads.load(Ordering::SeqCst), 0);
    context
        .update(&mut *lease.lock().await, Update::Release)
        .await?;
    Ok(())
}

#[tokio::test]
async fn canceled_receive_discards_unclaimed_changes_and_replays_saved_progress() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        for policy in [CheckpointPolicy::EveryBatch, coalesced()] {
            let (context, writes) = fixture(mode, policy).await?;
            let processor = pinned(&context).await?;
            processor.stop().await?;
            insert(&context, "unclaimed").await?;
            let root = context.list().await?.remove(0);
            let owned = context.acquire(&root).await?;
            let before = owned.value.base.continuation.clone();
            let checkpoints = writes.counts().checkpoints;
            let reads = writes.feed_reads.load(Ordering::SeqCst);
            let state = Arc::new(SharedState::default());
            let session = state
                .distributor
                .acquire(root.base.record.id.clone(), owned.value.incarnation.clone())?;
            let lease = Arc::new(Mutex::new(owned));
            let (_cancel, cancelled) = signal();
            let (_stop, stopped) = signal();
            let (_force, forced) = signal();
            let mut polling = Box::pin(poll_feed(
                context.clone(),
                state.clone(),
                &session,
                lease.clone(),
                cancelled,
                stopped,
                forced,
            ));
            assert!(futures::poll!(polling.as_mut()).is_pending());
            let mut receiving = Box::pin(state.distributor.next_batch());
            assert!(futures::poll!(receiving.as_mut()).is_pending());
            tokio::time::timeout(Duration::from_secs(5), async {
                tokio::select! {
                    result = &mut polling => panic!("Read ended before receive cancellation: {result:?}"),
                    () = async {
                        while writes.feed_reads.load(Ordering::SeqCst) == reads {
                            tokio::task::yield_now().await;
                        }
                    } => {},
                }
            }).await.unwrap();
            drop(receiving);
            let error = tokio::time::timeout(Duration::from_secs(5), polling)
                .await
                .unwrap()
                .unwrap_err();
            assert_eq!(
                error.status().status_code(),
                azure_core::http::StatusCode::Conflict
            );
            assert_eq!(writes.counts().checkpoints, checkpoints);
            assert_eq!(
                record(&context, &root.base.record.id)
                    .await
                    .value
                    .base
                    .continuation,
                before
            );
            assert!(lease.lock().await.value.pending.is_none());
            context
                .update(&mut *lease.lock().await, Update::Release)
                .await?;
            drop(session);
            assert!(futures::poll!(Box::pin(state.distributor.next_batch())).is_pending());

            let replacement = start(&context, context.options.clone()).await?;
            let (_, response, receipt) = next(&replacement).await.into_parts();
            let body: Value = response.into_body().into_single()?;
            assert_eq!(body["Documents"][0]["current"]["id"], "unclaimed");
            receipt.checkpoint().await?;
            replacement.stop().await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn no_receive_demand_keeps_feed_idle_while_leases_are_renewed() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        for policy in [CheckpointPolicy::EveryBatch, coalesced()] {
            let (mut context, writes) = fixture(mode, policy).await?;
            Arc::get_mut(&mut context).unwrap().options.expiration = Duration::from_millis(600);
            let processor = start(&context, context.options.clone()).await?;
            tokio::time::timeout(Duration::from_secs(5), async {
                while writes.counts().renewals < 2 {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
            assert_eq!(writes.feed_reads.load(Ordering::SeqCst), 0);
            let lease = context.list().await?.remove(0);
            assert!(lease.base.owner.is_some());
            assert!(lease.base.continuation.is_none());
            processor.stop().await?;
            assert_eq!(writes.feed_reads.load(Ordering::SeqCst), 0);
            assert!(record(&context, &lease.base.record.id)
                .await
                .value
                .base
                .owner
                .is_none());
        }
    }
    Ok(())
}

#[tokio::test]
async fn acknowledgment_and_deadline_flush_do_not_authorize_another_read() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        for policy in [
            CheckpointPolicy::EveryBatch,
            CheckpointPolicy::Coalesced {
                max_interval: Duration::from_millis(50),
            },
        ] {
            let (mut context, writes) = fixture(mode, policy).await?;
            Arc::get_mut(&mut context).unwrap().options.expiration = Duration::from_millis(600);
            let processor = pinned(&context).await?;
            insert(&context, "first").await?;
            let (id, _, receipt) = next(&processor).await.into_parts();
            let before = record(&context, &id).await.value.base.continuation;
            let reads = writes.feed_reads.load(Ordering::SeqCst);
            let renewals = writes.counts().renewals;
            receipt.acknowledge().await?;
            tokio::time::timeout(Duration::from_secs(5), async {
                while writes.counts().renewals < renewals + 2 {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
            assert_eq!(writes.feed_reads.load(Ordering::SeqCst), reads);
            assert_ne!(record(&context, &id).await.value.base.continuation, before);
            insert(&context, "second").await?;
            let (_, response, receipt) = next(&processor).await.into_parts();
            let body: Value = response.into_body().into_single()?;
            assert_eq!(body["Documents"][0]["current"]["id"], "second");
            receipt.acknowledge().await?;
            processor.stop().await?;
        }
    }
    Ok(())
}

#[tokio::test]
async fn unchanged_idle_continuations_do_not_repeat_checkpoint_writes() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        for policy in [CheckpointPolicy::EveryBatch, coalesced()] {
            let (context, writes) = fixture(mode, policy).await?;
            let processor = pinned(&context).await?;
            let lease = context.list().await?.remove(0);
            let saved = lease.base.continuation;
            assert!(saved.is_some());
            assert_eq!(writes.counts().checkpoints, 1);
            let reads = writes.feed_reads.load(Ordering::SeqCst);
            poll_until(
                &processor,
                tokio::time::timeout(Duration::from_secs(5), async {
                    while writes.feed_reads.load(Ordering::SeqCst) < reads + 3 {
                        tokio::task::yield_now().await;
                    }
                }),
            )
            .await
            .unwrap();
            assert!(futures::poll!(Box::pin(processor.next_batch())).is_pending());
            processor.stop().await?;
            assert_eq!(
                writes.counts(),
                Counts {
                    creates: 1,
                    acquisitions: if policy == CheckpointPolicy::EveryBatch {
                        1
                    } else {
                        2
                    },
                    checkpoints: 1,
                    releases: if policy == CheckpointPolicy::EveryBatch {
                        1
                    } else {
                        2
                    },
                    ..Counts::default()
                }
            );
            let after = record(&context, &lease.base.record.id).await.value;
            assert_eq!(after.base.continuation, saved);
            assert!(after.base.owner.is_none());
        }
    }
    Ok(())
}

#[tokio::test]
async fn coalescing_reduces_checkpoint_writes_without_changing_delivery_in_both_modes() -> Result<()>
{
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        for (policy, expected_checkpoints) in [(CheckpointPolicy::EveryBatch, 12), (coalesced(), 1)]
        {
            let (context, writes) = fixture(mode, policy).await?;
            let processor = pinned(&context).await?;
            assert_eq!(
                writes.counts(),
                Counts {
                    creates: 1,
                    acquisitions: if policy == CheckpointPolicy::EveryBatch {
                        1
                    } else {
                        2
                    },
                    checkpoints: 1,
                    releases: u64::from(policy != CheckpointPolicy::EveryBatch),
                    ..Counts::default()
                }
            );
            writes.reset();
            let expected: Vec<_> = (0..12).map(|n| format!("item-{n}")).collect();
            for id in &expected {
                insert(&context, id).await?;
            }
            let mut delivered = Vec::new();
            for _ in &expected {
                let (_, response, receipt) = next(&processor).await.into_parts();
                let body: Value = response.into_body().into_single()?;
                let changes = body["Documents"].as_array().unwrap();
                assert_eq!(changes.len(), 1);
                delivered.push(changes[0]["current"]["id"].as_str().unwrap().to_owned());
                receipt.acknowledge().await?;
            }
            assert_eq!(delivered, expected);
            let reads = writes.feed_reads.load(Ordering::SeqCst);
            poll_until(
                &processor,
                tokio::time::timeout(Duration::from_secs(5), async {
                    while writes.feed_reads.load(Ordering::SeqCst) < reads + 3 {
                        tokio::task::yield_now().await;
                    }
                }),
            )
            .await
            .unwrap();
            assert_eq!(
                writes.counts().checkpoints,
                if policy == CheckpointPolicy::EveryBatch {
                    expected_checkpoints
                } else {
                    0
                }
            );
            processor.stop().await?;
            assert_eq!(
                writes.counts(),
                Counts {
                    checkpoints: expected_checkpoints,
                    releases: 1,
                    ..Counts::default()
                }
            );
        }
    }
    Ok(())
}

#[tokio::test]
async fn durable_barrier_and_graceful_drain_flush_only_successful_prefixes() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let processor = pinned(&context).await?;
    writes.reset();
    for id in ["one", "two", "three", "four"] {
        insert(&context, id).await?;
    }
    let (id, _, receipt) = next(&processor).await.into_parts();
    let initial = record(&context, &id).await.value.base.continuation;
    receipt.acknowledge().await?;
    let (_, _, receipt) = next(&processor).await.into_parts();
    assert_eq!(record(&context, &id).await.value.base.continuation, initial);
    assert_eq!(writes.counts().checkpoints, 0);
    receipt.checkpoint().await?;
    assert_eq!(writes.counts().checkpoints, 1);
    assert_ne!(record(&context, &id).await.value.base.continuation, initial);
    let (_, _, receipt) = next(&processor).await.into_parts();
    let mut stopping = Box::pin(processor.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    receipt.acknowledge().await?;
    stopping.await?;
    assert_eq!(writes.counts().checkpoints, 2);
    let restarted = start(&context, context.options.clone()).await?;
    let (_, response, receipt) = next(&restarted).await.into_parts();
    let body: Value = response.into_body().into_single()?;
    assert_eq!(body["Documents"][0]["current"]["id"], "four");
    receipt.checkpoint().await?;
    restarted.stop().await?;
    Ok(())
}

#[tokio::test]
async fn failure_flushes_only_prior_success_and_force_replays_unpersisted_successes() -> Result<()>
{
    for force in [false, true] {
        let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
        let processor = pinned(&context).await?;
        writes.reset();
        for id in ["one", "two"] {
            insert(&context, id).await?;
        }
        let (id, _, receipt) = next(&processor).await.into_parts();
        let initial = record(&context, &id).await.value.base.continuation;
        receipt.acknowledge().await?;
        let (_, _, receipt) = next(&processor).await.into_parts();
        let mut stopping = Box::pin(processor.stop());
        assert!(futures::poll!(stopping.as_mut()).is_pending());
        if force {
            processor.abort().await?;
            assert!(receipt.acknowledge().await.is_err());
        } else {
            drop(receipt);
        }
        assert!(stopping.await.is_err());
        let saved = record(&context, &id).await.value.base.continuation;
        if force {
            assert_eq!(saved, initial);
        } else {
            assert_ne!(saved, initial);
        }
        assert_eq!(writes.counts().checkpoints, u64::from(!force));
        let restarted = start(&context, context.options.clone()).await?;
        for expected in if force {
            vec!["one", "two"]
        } else {
            vec!["two"]
        } {
            let (_, response, receipt) = next(&restarted).await.into_parts();
            let body: Value = response.into_body().into_single()?;
            assert_eq!(body["Documents"][0]["current"]["id"], expected);
            receipt.checkpoint().await?;
        }
        restarted.stop().await?;
    }
    Ok(())
}

#[tokio::test]
async fn ownership_loss_fences_buffered_progress_and_durable_barriers() -> Result<()> {
    let (context, _) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let processor = pinned(&context).await?;
    for id in ["one", "two"] {
        insert(&context, id).await?;
    }
    let (id, _, receipt) = next(&processor).await.into_parts();
    receipt.acknowledge().await?;
    let (_, _, receipt) = next(&processor).await.into_parts();
    let observed = record(&context, &id).await.value;
    let replacement = context.acquire(&observed).await?;
    assert!(receipt.checkpoint().await.is_err());
    let _ = processor.abort().await;
    let saved = record(&context, &id).await.value;
    assert_eq!(saved.incarnation, replacement.value.incarnation);
    assert_eq!(saved.base.continuation, observed.base.continuation);
    assert_eq!(saved.base.owner, replacement.value.base.owner);
    Ok(())
}

#[tokio::test]
async fn deadline_flushes_while_work_is_pending_and_renewal_preserves_dirty_age() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
    owned
        .value
        .acknowledge(ContinuationToken::from_string("handled".into()), 3);
    let since = Instant::now() - Duration::from_secs(60);
    owned.value.pending.as_mut().unwrap().since = since;
    context.update(&mut owned, Update::Renew).await?;
    owned
        .value
        .acknowledge(ContinuationToken::from_string("latest-handled".into()), 4);
    assert_eq!(owned.value.pending.as_ref().unwrap().since, since);
    assert_eq!(owned.value.pending.as_ref().unwrap().handled_changes, 7);
    assert_eq!(
        owned
            .value
            .checkpoint_delay(coalesced(), since + Duration::from_secs(60)),
        Some(Duration::ZERO)
    );
    assert_eq!(
        owned
            .value
            .checkpoint_delay(coalesced(), since + Duration::from_secs(59)),
        Some(Duration::from_secs(1))
    );
    let id = owned.value.base.record.id.clone();
    let lease = Mutex::new(owned);
    let (_cancel, cancelled) = signal();
    let (force, forced) = signal();
    let waiting = context.wait_with_checkpoint(
        &lease,
        &cancelled,
        &forced,
        futures::future::pending::<Result<()>>(),
    );
    let verifying = async {
        tokio::time::timeout(Duration::from_secs(5), async {
            while writes.counts().checkpoints == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        let saved = record(&context, &id).await.value;
        assert_eq!(saved.base.continuation.unwrap().as_str(), "latest-handled");
        assert!(saved.pending.is_none());
        force.send(()).unwrap();
    };
    let (result, ()) = futures::join!(waiting, verifying);
    assert!(result?.is_none());
    assert!(lease.lock().await.value.pending.is_none());
    assert_eq!(writes.counts().checkpoints, 1);
    Ok(())
}

#[tokio::test]
async fn forced_cancellation_wins_over_an_expired_flush_deadline() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
    owned
        .value
        .acknowledge(ContinuationToken::from_string("never-persist".into()), 1);
    owned.value.pending.as_mut().unwrap().since = Instant::now() - Duration::from_secs(60);
    let lease = Mutex::new(owned);
    let (_cancel, cancelled) = signal();
    let (force, forced) = signal();
    force.send(()).unwrap();
    assert!(context
        .wait_with_checkpoint(
            &lease,
            &cancelled,
            &forced,
            futures::future::pending::<Result<()>>()
        )
        .await?
        .is_none());
    assert_eq!(writes.counts().checkpoints, 0);
    Ok(())
}

#[tokio::test]
async fn checkpoint_intervals_are_validated_before_acquisition() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    writes.reset();
    for max_interval in [Duration::ZERO, Duration::MAX] {
        let result = start(
            &context,
            context
                .options
                .clone()
                .with_checkpoint_policy(CheckpointPolicy::Coalesced { max_interval }),
        )
        .await;
        assert!(result.is_err());
        assert_eq!(writes.counts(), Counts::default());
    }
    Ok(())
}

#[tokio::test]
async fn one_idle_slice_does_not_flush_a_still_busy_composite_lease() -> Result<()> {
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, coalesced()).await?;
    let mut tokens = Vec::new();
    for (min, max) in [("", "80"), ("80", "FF")] {
        let range = FeedRange::new(Epk::try_from(min)?, Epk::try_from(max)?)?;
        let mut plan = context
            .source_driver
            .plan_operation(
                context.operation(&range),
                &OperationOptions::default(),
                None,
                &PlanOptions {
                    change_feed_processor: true,
                    ..Default::default()
                },
            )
            .await?;
        let response = context
            .source_driver
            .execute_plan(
                &mut plan,
                Some(context.source.clone()),
                OperationOptions::default(),
            )
            .await?
            .unwrap();
        assert_eq!(
            response.status().status_code(),
            azure_core::http::StatusCode::NotModified
        );
        let ResolvedToken::Client(state) = plan.to_continuation_token()?.resolve()? else {
            panic!("composite token");
        };
        let PipelineNodeState::UnorderedMerge { active_tokens, .. } = state.into_root_node_state()
        else {
            panic!("change feed state");
        };
        tokens.extend(active_tokens);
    }
    assert_eq!(tokens.len(), 2);
    let checkpoint = ContinuationToken::encode(
        &context.operation(&FeedRange::full()),
        &PipelineNodeState::UnorderedMerge {
            active_tokens: tokens,
            start_from: Some(ChangeFeedStartFrom::Now),
            next_poll_min_epk: None,
        },
    )?;
    let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
    context
        .update(&mut owned, Update::Checkpoint(&checkpoint))
        .await?;
    context.update(&mut owned, Update::Release).await?;
    writes.reset();
    insert(&context, "one").await?;
    insert(&context, "two").await?;
    let processor = start(&context, context.options.clone()).await?;
    let (id, _, receipt) = next(&processor).await.into_parts();
    receipt.acknowledge().await?;
    let (_, response, receipt) = next(&processor).await.into_parts();
    let body: Value = response.into_body().into_single()?;
    assert_eq!(body["Documents"][0]["current"]["id"], "two");
    assert_eq!(
        record(&context, &id).await.value.base.continuation,
        Some(checkpoint)
    );
    assert_eq!(writes.counts().checkpoints, 0);
    receipt.acknowledge().await?;
    let reads = writes.feed_reads.load(Ordering::SeqCst);
    poll_until(
        &processor,
        tokio::time::timeout(Duration::from_secs(5), async {
            while writes.feed_reads.load(Ordering::SeqCst) < reads + 3 {
                tokio::task::yield_now().await;
            }
        }),
    )
    .await
    .unwrap();
    assert_eq!(writes.counts().checkpoints, 0);
    processor.stop().await?;
    assert_eq!(writes.counts().checkpoints, 1);
    Ok(())
}

#[tokio::test]
async fn five_minute_coalescing_does_not_flush_initial_or_repeated_idle_reads() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let (context, writes) = fixture(mode, CheckpointPolicy::coalesced()).await?;
        let processor = start(&context, context.options.clone()).await?;
        poll_until(
            &processor,
            tokio::time::timeout(Duration::from_secs(5), async {
                while writes.feed_reads.load(Ordering::SeqCst) < 3 {
                    tokio::task::yield_now().await;
                }
            }),
        )
        .await
        .unwrap();
        let stored = context.list().await?.remove(0);
        assert!(stored.base.owner.is_some());
        assert!(stored.base.continuation.is_none());
        assert!(stored.last_processed_at.is_none());
        assert_eq!(writes.counts().checkpoints, 0);
        processor.stop().await?;
        let stored = record(&context, &stored.base.record.id).await.value;
        assert!(stored.base.owner.is_none());
        assert!(stored.base.continuation.is_some());
        assert!(stored.last_processed_at.is_none());
        assert_eq!(writes.counts().checkpoints, 1);
    }
    Ok(())
}

#[tokio::test]
async fn five_minute_deadline_flushes_without_demand_or_new_acknowledgment() -> Result<()> {
    let policy = CheckpointPolicy::coalesced();
    let (context, writes) = fixture(ChangeFeedFactoryMode::Incremental, policy).await?;
    let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
    owned
        .value
        .acknowledge(ContinuationToken::from_string("handled".into()), 1);
    let since = owned.value.pending.as_ref().unwrap().since;
    assert_eq!(
        owned
            .value
            .checkpoint_delay(policy, since + Duration::from_secs(299)),
        Some(Duration::from_secs(1))
    );
    assert_eq!(
        owned
            .value
            .checkpoint_delay(policy, since + Duration::from_secs(300)),
        Some(Duration::ZERO)
    );
    owned.value.pending.as_mut().unwrap().since = Instant::now() - Duration::from_secs(300);
    let id = owned.value.base.record.id.clone();
    let lease = Mutex::new(owned);
    let (_cancel, cancelled) = signal();
    let (force, forced) = signal();
    let waiting = context.wait_with_checkpoint(
        &lease,
        &cancelled,
        &forced,
        futures::future::pending::<Result<()>>(),
    );
    let verifying = async {
        tokio::time::timeout(Duration::from_secs(5), async {
            while writes.counts().checkpoints == 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert_eq!(
            record(&context, &id)
                .await
                .value
                .base
                .continuation
                .unwrap()
                .as_str(),
            "handled"
        );
        assert_eq!(writes.feed_reads.load(Ordering::SeqCst), 0);
        force.send(()).unwrap();
    };
    let (result, ()) = futures::join!(waiting, verifying);
    assert!(result?.is_none());
    assert_eq!(writes.counts().checkpoints, 1);
    Ok(())
}
