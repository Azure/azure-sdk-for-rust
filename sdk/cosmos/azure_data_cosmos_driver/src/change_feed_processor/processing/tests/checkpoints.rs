// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::{signal, CheckpointPolicy};
use super::{
    fixture_with_observer, insert, next, start, ChangeFeedFactoryMode, Context, Lease, Processor,
    RequestObserver, Update,
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
    sync::{Arc, Mutex as StdMutex},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts {
    creates: u64,
    acquisitions: u64,
    checkpoints: u64,
    renewals: u64,
    releases: u64,
    splits: u64,
}

#[derive(Debug, Default)]
struct Writes {
    state: StdMutex<(HashMap<String, Value>, Counts)>,
}

impl RequestObserver for Writes {
    fn on_request(&self, request: &Request) {
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
        } else if value.get("children").is_some() {
            counts.splits += 1;
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
    let processor = start(context, context.options.clone()).await?;
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
                    acquisitions: 1,
                    checkpoints: 1,
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
            tokio::time::timeout(Duration::from_secs(5), async {
                while writes.counts().checkpoints != expected_checkpoints {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
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
        let ResolvedToken::ClientV1(state) = plan.to_continuation_token()?.resolve()? else {
            panic!("composite token");
        };
        let PipelineNodeState::UnorderedMerge { active_tokens, .. } = state.into_root_node_state()
        else {
            panic!("change feed state");
        };
        tokens.extend(active_tokens);
    }
    assert_eq!(tokens.len(), 2);
    let checkpoint = ContinuationToken::encode_v1(
        &context.operation(&FeedRange::full()),
        &PipelineNodeState::UnorderedMerge {
            active_tokens: tokens,
            start_from: Some(ChangeFeedStartFrom::Now),
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
    tokio::time::timeout(Duration::from_secs(5), async {
        while writes.counts().checkpoints == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    processor.stop().await?;
    assert_eq!(writes.counts().checkpoints, 1);
    Ok(())
}
