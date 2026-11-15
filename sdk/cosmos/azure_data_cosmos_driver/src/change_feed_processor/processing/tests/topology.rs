// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::{poll_feed, signal, PollError, SharedState};
#[cfg(feature = "fault_injection")]
use super::{
    super::{balance, supervise},
    emulator, fault_fixture, fixture_with_runtime, CheckpointPolicy,
};
use super::{
    bootstrap, fixture_with_emulator, next, poll_until, start, ChangeFeedBatch,
    ChangeFeedFactoryMode, Context, LeaseDocument, LeaseState, Processor, ProcessorOptions, Update,
};
#[cfg(feature = "fault_injection")]
use crate::fault_injection::{
    CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use crate::{
    change_feed_processor::bootstrap::{lease_id, RangePlan},
    driver::dataflow::{PipelineNodeState, RangedToken},
    in_memory_emulator::Epk,
    models::{
        ContinuationToken, CosmosOperation, CosmosResponse, FeedRange, ItemReference,
        MaxItemCountHint, PartitionKey, PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion,
        ResolvedToken,
    },
    options::OperationOptions,
    Result,
};
use async_lock::Mutex;
use azure_core::http::{Etag, StatusCode};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroU32,
    sync::Arc,
    time::Duration,
};

fn range(min: &str, max: &str) -> FeedRange {
    FeedRange::new(Epk::try_from(min).unwrap(), Epk::try_from(max).unwrap()).unwrap()
}

fn config(context: &Context, worker: &str, limit: u32) -> ProcessorOptions {
    let mut options = context
        .options
        .clone()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()))
        .with_lease_counts(0, limit);
    options.worker_name = worker.into();
    options.expiration = Duration::from_millis(600);
    options.verify_leases_on_restart = true;
    options
}

pub(super) fn keys(ranges: &[FeedRange]) -> Vec<String> {
    let mut grouped = vec![Vec::new(); ranges.len()];
    for candidate in 0..4096 {
        let key = format!("key-{candidate}");
        let epk = Epk::compute(
            &[PartitionKeyValue::from(key.clone())],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        let index = ranges
            .iter()
            .position(|range| range.min_inclusive() <= &epk && &epk < range.max_exclusive())
            .unwrap();
        if grouped[index].len() < 2 {
            grouped[index].push(key);
        }
        if grouped.iter().all(|keys| keys.len() == 2) {
            return grouped.into_iter().flatten().collect();
        }
    }
    panic!("could not select keys in every target slice")
}

pub(super) async fn publish(
    context: &Context,
    keys: &[String],
    groups: std::ops::Range<u64>,
    expected: &mut BTreeMap<String, Value>,
) -> Result<()> {
    for group in groups {
        for key in keys {
            let id = format!("{key}-{group}");
            let doc = json!({"id":id, "pk":key, "group":group});
            context
                .source_driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(ItemReference::from_name(
                        &context.source,
                        PartitionKey::from(key.clone()),
                        id.clone(),
                    ))
                    .with_body(serde_json::to_vec(&doc)?),
                    OperationOptions::default(),
                )
                .await?;
            expected.insert(id, doc);
        }
    }
    Ok(())
}

#[derive(Default)]
struct Sink {
    seen: BTreeSet<String>,
    next_group: BTreeMap<String, u64>,
}

impl Sink {
    fn observe(
        &mut self,
        response: CosmosResponse,
        expected: &BTreeMap<String, Value>,
    ) -> Result<()> {
        let mut items = Vec::new();
        for value in response.into_body().into_items::<Value>()? {
            if let Some(documents) = value.get("Documents") {
                items.extend(documents.as_array().expect("feed envelope").iter().cloned());
            } else {
                items.push(value);
            }
        }
        assert!(!items.is_empty());
        for item in items {
            let document = item.get("current").unwrap_or(&item);
            let id = document["id"].as_str().expect("document id");
            let expected = expected.get(id).expect("unexpected delivery");
            assert_eq!(document["pk"], expected["pk"]);
            assert_eq!(document["group"], expected["group"]);
            if self.seen.insert(id.to_owned()) {
                let next = self
                    .next_group
                    .entry(document["pk"].as_str().unwrap().to_owned())
                    .or_default();
                assert_eq!(
                    document["group"].as_u64().unwrap(),
                    *next,
                    "out-of-order first delivery for {id}"
                );
                *next += 1;
            }
        }
        Ok(())
    }

    fn complete(&self, expected: &BTreeMap<String, Value>) {
        assert_eq!(self.seen, expected.keys().cloned().collect());
    }
}

async fn wait_for(
    context: &Context,
    predicate: impl Fn(&[LeaseDocument]) -> bool,
) -> Vec<LeaseDocument> {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let leases = context.list().await.unwrap();
            if predicate(&leases) {
                return leases;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("lease state did not converge")
}

async fn next_from(first: &Processor, second: &Processor) -> ChangeFeedBatch {
    tokio::time::timeout(Duration::from_secs(10), async {
        tokio::select! {
            batch = first.read_next_change_feed_batch() => batch.unwrap(),
            batch = second.read_next_change_feed_batch() => batch.unwrap(),
        }
    })
    .await
    .expect("both processors stopped making progress")
}

async fn stored(context: &Context, id: &str) -> (LeaseDocument, Etag) {
    context
        .store()
        .read(id)
        .await
        .unwrap()
        .expect("lease exists")
}

async fn assert_deleted(context: &Context, id: &str) {
    assert!(
        context
            .store()
            .read::<LeaseDocument>(id)
            .await
            .unwrap()
            .is_none(),
        "split parent remains: {id}"
    );
}

fn token(context: &Context, scope: &FeedRange, value: &str) -> ContinuationToken {
    ContinuationToken::encode(
        &context.operation(scope),
        &PipelineNodeState::UnorderedMerge {
            active_tokens: vec![RangedToken {
                min_epk: scope.min_inclusive().to_hex(),
                max_epk: scope.max_exclusive().to_hex(),
                server_continuation: value.into(),
            }],
            start_from: Some(context.options.start.clone()),
            next_poll_min_epk: None,
        },
    )
    .unwrap()
}

#[tokio::test]
async fn first_poll_split_preserves_saved_progress_without_delivery() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let (context, emulator) = fixture_with_emulator(mode, 1).await?;
        let root = context.list().await?.remove(0);
        let mut owned = context.acquire(&root).await?;
        let checkpoint = token(&context, &root.range, "saved-before-split");
        context
            .update(&mut owned, Update::Checkpoint(&checkpoint))
            .await?;
        let before = stored(&context, &root.record.id).await;
        emulator
            .store()
            .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
            .await?;
        let state = Arc::new(SharedState::default());
        let session = state
            .distributor
            .acquire(root.record.id.clone(), owned.value.incarnation.clone())?;
        let (_cancel, cancelled) = signal();
        let (_stop, stopping) = signal();
        let (_force, forced) = signal();
        let owned = Arc::new(Mutex::new(owned));
        let error = tokio::time::timeout(Duration::from_secs(5), async {
            tokio::select! {
                result = poll_feed(
                    context.clone(),
                    state.clone(),
                    &session,
                    owned.clone(),
                    cancelled,
                    stopping,
                    forced,
                ) => result,
                _ = state.distributor.read_next_change_feed_batch() => panic!("Split feed unexpectedly delivered"),
            }
        })
        .await
        .unwrap()
        .unwrap_err();
        let PollError::SourceTopologyChange(error) = error else {
            panic!("Source split did not produce a source-topology failure: {error:?}");
        };
        assert_eq!(error.status().status_code(), StatusCode::Gone);
        assert_eq!(stored(&context, &root.record.id).await.1, before.1);
        assert_eq!(owned.lock().await.value.continuation, Some(checkpoint));
        assert!(
            futures::poll!(Box::pin(state.distributor.read_next_change_feed_batch())).is_pending()
        );
        context
            .update(&mut *owned.lock().await, Update::Release)
            .await?;
    }
    Ok(())
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn checkpoint_topology_failure_does_not_recover_the_source() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "lease-checkpoint-topology",
                FaultInjectionResultBuilder::new()
                    .with_custom_response(
                        CustomResponseBuilder::new(StatusCode::Gone)
                            .with_header("x-ms-substatus", "1002")
                            .with_body(br#"{"message":"injected lease topology failure"}"#.to_vec())
                            .build(),
                    )
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReplaceItem)
                    .build(),
            )
            // Singleton lease writes refresh routing once before surfacing a topology failure.
            .with_hit_limit(2)
            .build(),
        );
        rule.disable();
        let emulator = emulator()?;
        let mut context = fixture_with_runtime(
            &emulator,
            emulator.runtime_builder_with_fault_rules(vec![rule.clone()]),
            mode,
            1,
        )
        .await?;
        Arc::get_mut(&mut context)
            .unwrap()
            .options
            .checkpoint_policy = CheckpointPolicy::Deferred {
            max_interval: Duration::from_millis(10),
        };
        let root = context.list().await?.remove(0);
        let mut owned = context.acquire(&root).await?;
        let durable = token(&context, &root.range, "durable-before-split");
        context
            .update(&mut owned, Update::Checkpoint(&durable))
            .await?;
        owned.acknowledge(token(&context, &root.range, "pending-before-split"), 1);
        owned.pending.as_mut().unwrap().since = std::time::Instant::now() - Duration::from_secs(1);
        emulator
            .store()
            .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
            .await?;
        let errors = Arc::new(std::sync::Mutex::new(Vec::new()));
        let reported = errors.clone();
        let state = Arc::new(SharedState {
            error_observer: Some(Arc::new(move |_, err| {
                reported.lock().unwrap().push(err);
            })),
            ..Default::default()
        });
        let (_stop, stopping) = signal();
        let (_force, forced) = signal();
        rule.enable();
        // No receive demand exists: the only operation that can fail is the overdue checkpoint.
        tokio::time::timeout(
            Duration::from_secs(5),
            supervise(context.clone(), state, owned, stopping, forced),
        )
        .await
        .expect("lease supervision did not finish");
        assert_eq!(rule.hit_count(), 2);
        {
            let errors = errors.lock().unwrap();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].status().status_code(), StatusCode::Gone);
            assert!(errors[0].status().is_partition_topology_change());
        }
        let leases = context.list().await?;
        assert_eq!(leases.len(), 1);
        assert_eq!(leases[0].record.id, root.record.id);
        assert!(leases[0].owner.is_none());
        assert_eq!(leases[0].continuation, Some(durable));
    }
    Ok(())
}

#[tokio::test]
async fn split_recovery_clips_physical_children_to_the_lease_slice() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let mut slice = context.list().await?.remove(0);
    let root_id = slice.record.id.clone();
    context
        .store()
        .execute(CosmosOperation::delete_item(ItemReference::from_name(
            &context.leases,
            PartitionKey::from(root_id.clone()),
            root_id,
        )))
        .await?;
    // A lease retained after a physical merge can cover only part of a partition.
    slice.range = range("10", "30");
    slice.record.id = lease_id(&context.namespace, &slice.range);
    assert!(context.store().create(&slice.record.id, &slice).await?);
    let mut owned = context.acquire(&slice).await?;
    let checkpoint = token(&context, &slice.range, "slice-progress");
    context
        .update(&mut owned, Update::Checkpoint(&checkpoint))
        .await?;
    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
        .await?;
    let owned = Arc::new(Mutex::new(owned));
    assert!(context.recover(owned.clone()).await?);
    let expected = [range("10", "20"), range("20", "30")];
    let children = context.list().await?;
    assert_eq!(
        children
            .iter()
            .map(|lease| lease.range.clone())
            .collect::<Vec<_>>(),
        expected
    );
    for child in children {
        let ResolvedToken::Client(token) = child.continuation.unwrap().resolve()? else {
            panic!("expected a composite continuation")
        };
        let PipelineNodeState::UnorderedMerge { active_tokens, .. } = token.into_root_node_state()
        else {
            panic!("expected change feed continuation")
        };
        assert_eq!(
            active_tokens,
            [RangedToken {
                min_epk: child.range.min_inclusive().to_hex(),
                max_epk: child.range.max_exclusive().to_hex(),
                server_continuation: "slice-progress".into(),
            }]
        );
        assert!(child.owner.is_none());
        assert!(child.incarnation.is_none());
    }
    assert_deleted(&context, &slice.record.id).await;
    assert_eq!(owned.lock().await.value.continuation, Some(checkpoint));
    Ok(())
}

#[tokio::test]
async fn split_with_pending_batch_preserves_coverage_and_rebalances_children() -> Result<()> {
    exercise_split(super::super::CheckpointPolicy::Eager).await
}

#[tokio::test]
async fn split_flushes_deferred_progress_before_creating_children() -> Result<()> {
    exercise_split(super::super::CheckpointPolicy::Deferred {
        max_interval: Duration::from_secs(60),
    })
    .await
}

async fn exercise_split(policy: super::super::CheckpointPolicy) -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let (mut context, emulator) = fixture_with_emulator(mode, 1).await?;
        Arc::get_mut(&mut context)
            .unwrap()
            .options
            .checkpoint_policy = policy;
        let children = vec![range("", "20"), range("20", "FF")];
        let keys = keys(&children);
        if policy != super::super::CheckpointPolicy::Eager {
            super::pin_start(&context).await?;
        }
        let first = start(&context, config(&context, "first", 1)).await?;
        poll_until(
            &first,
            wait_for(&context, |leases| {
                leases.iter().all(|lease| lease.continuation.is_some())
            }),
        )
        .await;
        let second = start(&context, config(&context, "second", 1)).await?;
        let mut expected = BTreeMap::new();
        publish(&context, &keys, 0..6, &mut expected).await?;
        let (parent_id, response, receipt) = next(&first).await.into_parts();
        let mut sink = Sink::default();
        sink.observe(response, &expected)?;
        let before = stored(&context, &parent_id).await.0.continuation;
        emulator
            .store()
            .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
            .await?;
        assert_eq!(stored(&context, &parent_id).await.0.continuation, before);
        assert_eq!(context.list().await?.len(), 1);
        assert!(tokio::time::timeout(
            Duration::from_millis(50),
            first.read_next_change_feed_batch()
        )
        .await
        .is_err());
        receipt.acknowledge().await?;
        let converging = wait_for(&context, |leases| {
            leases.len() == 2
                && leases
                    .iter()
                    .filter_map(|lease| lease.owner.as_ref())
                    .collect::<BTreeSet<_>>()
                    .len()
                    == 2
        });
        tokio::pin!(converging);
        let (active, mut pending) = tokio::select! {
            active = &mut converging => (active, None),
            batch = next_from(&first, &second) => (converging.await, Some(batch)),
        };
        assert_eq!(
            active
                .iter()
                .map(|lease| lease.range.clone())
                .collect::<Vec<_>>(),
            children
        );
        assert_deleted(&context, &parent_id).await;
        assert!(active.iter().all(|lease| lease.continuation.is_some()));
        assert!(active
            .iter()
            .all(|lease| lease.record.workload == context.identity));
        assert!(first.is_running());
        assert!(second.is_running());
        let mut delivered_leases = BTreeSet::new();
        while sink.seen.len() < expected.len() {
            let batch = match pending.take() {
                Some(batch) => batch,
                None => next_from(&first, &second).await,
            };
            let (id, response, receipt) = batch.into_parts();
            delivered_leases.insert(id);
            sink.observe(response, &expected)?;
            receipt.checkpoint().await?;
        }
        sink.complete(&expected);
        assert_eq!(
            delivered_leases,
            active.iter().map(|lease| lease.record.id.clone()).collect()
        );
        assert!(first.is_running());
        assert!(second.is_running());
        first.abort().await?;
        second.abort().await?;
        assert_eq!(context.list().await?.len(), 2);
    }
    Ok(())
}

#[tokio::test]
async fn split_during_pending_batch_replays_after_shutdown_and_restart() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let mut expected = BTreeMap::new();
    let keys = keys(&[range("", "20"), range("20", "FF")]);
    publish(&context, &keys, 0..4, &mut expected).await?;
    let first = start(&context, config(&context, "first", 1)).await?;
    let (id, response, receipt) = next(&first).await.into_parts();
    let mut sink = Sink::default();
    sink.observe(response, &expected)?;
    let checkpoint = stored(&context, &id).await.0.continuation;
    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
        .await?;
    first.abort().await?;
    assert!(receipt.checkpoint().await.is_err());
    assert_eq!(stored(&context, &id).await.0.continuation, checkpoint);
    let restarted = start(&context, config(&context, "replacement", 2)).await?;
    while sink.seen.len() < expected.len() {
        let (_, response, receipt) = next(&restarted).await.into_parts();
        sink.observe(response, &expected)?;
        receipt.checkpoint().await?;
    }
    sink.complete(&expected);
    restarted.abort().await?;
    assert_eq!(context.list().await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn graceful_exit_during_split_saves_parent_progress_for_the_next_owner() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let mut expected = BTreeMap::new();
    let keys = keys(&[range("", "20"), range("20", "FF")]);
    publish(&context, &keys, 0..4, &mut expected).await?;
    let first = start(&context, config(&context, "first", 1)).await?;
    let (id, response, receipt) = next(&first).await.into_parts();
    let mut sink = Sink::default();
    sink.observe(response, &expected)?;
    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
        .await?;
    let mut stopping = Box::pin(first.stop());
    assert!(futures::poll!(stopping.as_mut()).is_pending());
    receipt.checkpoint().await?;
    stopping.await?;
    let parent = stored(&context, &id).await.0;
    assert!(parent.owner.is_none());
    assert!(parent.continuation.is_some());
    assert!(
        context.list().await?.len() == 1,
        "the stopping worker must not start topology recovery"
    );
    let replacement = start(&context, config(&context, "replacement", 2)).await?;
    while sink.seen.len() < expected.len() {
        let (_, response, receipt) = next(&replacement).await.into_parts();
        sink.observe(response, &expected)?;
        receipt.checkpoint().await?;
    }
    sink.complete(&expected);
    replacement.abort().await?;
    assert_eq!(context.list().await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn merge_retains_uneven_predecessor_checkpoints_across_pending_batches_and_restart(
) -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        exercise_merge(mode, super::super::CheckpointPolicy::Eager).await?;
    }
    Ok(())
}

#[tokio::test]
async fn deferred_merge_recovery_keeps_independent_lease_positions() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        exercise_merge(
            mode,
            super::super::CheckpointPolicy::Deferred {
                max_interval: Duration::from_secs(60),
            },
        )
        .await?;
    }
    Ok(())
}

async fn exercise_merge(
    mode: ChangeFeedFactoryMode,
    policy: super::super::CheckpointPolicy,
) -> Result<()> {
    let (mut context, emulator) = fixture_with_emulator(mode, 2).await?;
    Arc::get_mut(&mut context)
        .unwrap()
        .options
        .checkpoint_policy = policy;
    let initial = context.list().await?;
    let initial_ids: BTreeSet<_> = initial
        .iter()
        .map(|lease| lease.record.id.clone())
        .collect();
    let keys = keys(
        &initial
            .iter()
            .map(|lease| lease.range.clone())
            .collect::<Vec<_>>(),
    );
    if policy != super::super::CheckpointPolicy::Eager {
        super::pin_start(&context).await?;
    }
    let first = start(&context, config(&context, "first", 1)).await?;
    let second = start(&context, config(&context, "second", 1)).await?;
    poll_until(
        &first,
        poll_until(
            &second,
            wait_for(&context, |leases| {
                leases.iter().all(|lease| lease.continuation.is_some())
            }),
        ),
    )
    .await;
    let mut expected = BTreeMap::new();
    publish(&context, &keys, 0..6, &mut expected).await?;
    let (held_id, response, held_receipt) = next_from(&first, &second).await.into_parts();
    let mut sink = Sink::default();
    sink.observe(response, &expected)?;
    for _ in 0..3 {
        let (id, response, receipt) = next_from(&first, &second).await.into_parts();
        assert_ne!(id, held_id);
        sink.observe(response, &expected)?;
        receipt.checkpoint().await?;
    }
    let (other_id, response, other_receipt) = next_from(&first, &second).await.into_parts();
    sink.observe(response, &expected)?;
    let held_checkpoint = stored(&context, &held_id).await.0.continuation;
    let other_checkpoint = stored(&context, &other_id).await.0.continuation;
    assert_ne!(held_checkpoint, other_checkpoint);
    emulator
        .store()
        .merge_partitions_and_wait("polling", "source", 0, 1, Duration::ZERO)
        .await?;
    assert_eq!(
        stored(&context, &held_id).await.0.continuation,
        held_checkpoint
    );
    assert_eq!(
        stored(&context, &other_id).await.0.continuation,
        other_checkpoint
    );
    other_receipt.acknowledge().await?;
    first.abort().await?;
    second.abort().await?;
    assert!(held_receipt.checkpoint().await.is_err());
    let restarted = start(&context, config(&context, "replacement", 2)).await?;
    publish(&context, &keys, 6..9, &mut expected).await?;
    while sink.seen.len() < expected.len() {
        let (_, response, receipt) = next(&restarted).await.into_parts();
        sink.observe(response, &expected)?;
        receipt.checkpoint().await?;
    }
    sink.complete(&expected);
    restarted.abort().await?;
    let active = context.list().await?;
    assert_eq!(
        active
            .iter()
            .map(|lease| lease.record.id.clone())
            .collect::<BTreeSet<_>>(),
        initial_ids
    );
    assert_eq!(
        context
            .source_driver
            .resolve_all_partition_key_ranges(&context.source, true)
            .await?
            .unwrap()
            .len(),
        1
    );
    let merged = context
        .source_driver
        .resolve_all_partition_key_ranges(&context.source, false)
        .await?
        .unwrap();
    emulator
        .store()
        .split_partition_and_wait(
            "polling",
            "source",
            merged[0].id.parse().unwrap(),
            Epk::try_from("10")?,
            Duration::ZERO,
        )
        .await?;
    publish(&context, &keys, 9..12, &mut expected).await?;
    let restarted = start(&context, config(&context, "after-merge-split", 3)).await?;
    while sink.seen.len() < expected.len() {
        let (_, response, receipt) = next(&restarted).await.into_parts();
        sink.observe(response, &expected)?;
        receipt.checkpoint().await?;
    }
    sink.complete(&expected);
    restarted.abort().await?;
    assert_eq!(context.list().await?.len(), 3);
    Ok(())
}

#[tokio::test]
async fn split_retries_partial_creation_without_resetting_a_competing_child() -> Result<()> {
    for partial in [false, true] {
        let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
        let root = context.list().await?.remove(0);
        let mut owned = context.acquire(&root).await?;
        let checkpoint = token(&context, &root.range, "parent-acknowledged");
        context
            .update(&mut owned, Update::Checkpoint(&checkpoint))
            .await?;
        let children = [range("", "20"), range("20", "FF")];
        let left_id = lease_id(&context.namespace, &children[0]);
        let right_id = lease_id(&context.namespace, &children[1]);
        assert!(context
            .store()
            .read::<LeaseDocument>(&left_id)
            .await?
            .is_none());
        assert!(context
            .store()
            .read::<LeaseDocument>(&right_id)
            .await?
            .is_none());
        let inventory = context.list().await?;
        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory[0].record.id, root.record.id);
        assert_eq!(inventory[0].owner, owned.value.owner);
        assert!(context
            .store()
            .read::<LeaseDocument>(&left_id)
            .await?
            .is_none());
        assert!(context
            .store()
            .read::<LeaseDocument>(&right_id)
            .await?
            .is_none());
        let advanced = token(&context, &children[0], "child-advanced");
        if partial {
            let mut left = owned.value.clone();
            left.record.id = left_id.clone();
            left.range = children[0].clone();
            left.continuation = Some(advanced.clone());
            left.owner = Some("competing-worker".into());
            left.incarnation = Some("competing-acquisition".into());
            assert!(context.store().create(&left_id, &left).await?);
        }
        context.split(&mut owned, &children).await?;
        assert_eq!(context.list().await?.len(), 2);
        assert_deleted(&context, &root.record.id).await;
        if partial {
            let child = stored(&context, &left_id).await.0;
            assert_eq!(child.owner.as_deref(), Some("competing-worker"));
            assert_eq!(child.incarnation.as_deref(), Some("competing-acquisition"));
        }
        assert_eq!(
            stored(&context, &left_id).await.0.continuation,
            Some(if partial {
                advanced
            } else {
                context.checkpoint(&owned.value, &children[0])?
            })
        );
        assert_eq!(
            stored(&context, &right_id).await.0.continuation,
            Some(context.checkpoint(&owned.value, &children[1])?)
        );
        let left_before = stored(&context, &left_id).await;
        let right_before = stored(&context, &right_id).await;
        bootstrap(
            &context.source_driver,
            &context.source,
            &context.lease_driver,
            &context.leases,
            context.options.mode,
            &context.options.workload_name,
            &context.options.worker_name,
        )
        .await?;
        context.verify_leases().await?;
        assert_deleted(&context, &root.record.id).await;
        assert_eq!(stored(&context, &left_id).await.1, left_before.1);
        assert_eq!(stored(&context, &right_id).await.1, right_before.1);
        let plan = context
            .store()
            .read::<RangePlan>(&format!("{}.plan", context.namespace))
            .await?
            .unwrap();
        assert_eq!(plan.0.ranges, [FeedRange::full()]);
        assert_eq!(context.list().await?.len(), 2);
    }
    Ok(())
}

#[tokio::test]
async fn stale_owner_cannot_split_parent_or_advance_after_takeover() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let mut old = context.acquire(&root).await?;
    let mut new = context.acquire(&old.value).await?;
    let children = [range("", "20"), range("20", "FF")];
    assert!(context
        .split(&mut old, &children)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert_eq!(context.list().await?.len(), 1);
    context.split(&mut new, &children).await?;
    let checkpoint = token(&context, &root.range, "stale");
    for update in [
        Update::Renew,
        Update::Release,
        Update::Checkpoint(&checkpoint),
    ] {
        assert!(context
            .update(&mut old, update)
            .await
            .unwrap_err()
            .status()
            .is_precondition_failed());
    }
    let mut incompatible = new.value;
    incompatible.record.workload.version += 1;
    assert!(context.validate(&incompatible).is_err());
    Ok(())
}

#[tokio::test]
async fn descendant_split_projects_saved_state_and_keeps_disjoint_coverage() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::FullFidelity, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.range, "pinned");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    context
        .split(&mut parent, &[range("", "20"), range("20", "FF")])
        .await?;
    let leaves = context.list().await?;
    let mut left = context.acquire(&leaves[0]).await?;
    let checkpoint = token(&context, &left.value.range, "left-advanced");
    context
        .update(&mut left, Update::Checkpoint(&checkpoint))
        .await?;
    context
        .split(&mut left, &[range("", "10"), range("10", "20")])
        .await?;
    let leaves = context.list().await?;
    assert_eq!(
        leaves
            .iter()
            .map(|lease| lease.range.clone())
            .collect::<Vec<_>>(),
        [range("", "10"), range("10", "20"), range("20", "FF")]
    );
    for (lease, expected) in leaves
        .iter()
        .zip(["left-advanced", "left-advanced", "pinned"])
    {
        let ResolvedToken::Client(state) = lease.continuation.as_ref().unwrap().resolve()? else {
            panic!("composite token")
        };
        let PipelineNodeState::UnorderedMerge { active_tokens, .. } = state.into_root_node_state()
        else {
            panic!("change feed token")
        };
        assert_eq!(active_tokens.len(), 1);
        assert_eq!(active_tokens[0].server_continuation, expected);
        assert_eq!(
            active_tokens[0].min_epk,
            lease.range.min_inclusive().to_hex()
        );
        assert_eq!(
            active_tokens[0].max_epk,
            lease.range.max_exclusive().to_hex()
        );
    }
    assert_deleted(&context, &root.record.id).await;
    assert_deleted(&context, &left.value.record.id).await;
    context.verify_leases().await?;
    assert!(context
        .split(&mut parent, &[range("", "20"), range("20", "FF")])
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert_deleted(&context, &root.record.id).await;
    assert_deleted(&context, &left.value.record.id).await;
    assert_eq!(context.list().await?.len(), 3);
    Ok(())
}

#[tokio::test]
async fn unfinished_split_surfaces_1007_without_changing_a_lease() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let owned = Arc::new(Mutex::new(context.acquire(&root).await?));
    let mut plan = context
        .source_driver
        .plan_operation(
            context.operation(&root.range),
            &OperationOptions::default(),
            None,
            &crate::options::PlanOptions {
                change_feed_processor: true,
                ..Default::default()
            },
        )
        .await?;
    let transition = emulator.store().begin_manual_split_partition(
        "polling",
        "source",
        0,
        Epk::try_from("20")?,
    )?;
    let error = context
        .source_driver
        .execute_plan(
            &mut plan,
            Some(context.source.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap_err();
    assert_eq!(error.status(), crate::error::status_codes::COMPLETING_SPLIT);
    let before = stored(&context, &root.record.id).await;
    assert!(!context.recover(owned.clone()).await?);
    assert_eq!(stored(&context, &root.record.id).await.1, before.1);
    transition.complete().await?;
    assert!(context.recover(owned).await?);
    assert_eq!(context.list().await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn concurrent_split_handling_preserves_child_checkpoints() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let mut first = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.range, "acknowledged");
    context
        .update(&mut first, Update::Checkpoint(&checkpoint))
        .await?;
    let mut second: LeaseState = stored(&context, &root.record.id).await.into();
    let children = [range("", "20"), range("20", "FF")];
    let (one, two) = futures::join!(
        context.split(&mut first, &children),
        context.split(&mut second, &children),
    );
    assert!(one.is_ok() || two.is_ok());
    for rejected in [one, two].into_iter().filter_map(Result::err) {
        assert!(rejected.status().is_precondition_failed());
    }
    assert_deleted(&context, &root.record.id).await;
    for child in &children {
        assert_eq!(
            stored(&context, &lease_id(&context.namespace, child))
                .await
                .0
                .continuation,
            Some(context.checkpoint(&first.value, child)?)
        );
    }
    assert_eq!(context.list().await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn split_rejects_gaps_overlaps_and_non_splits_without_mutation() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let mut owned = context.acquire(&root).await?;
    let before = owned.etag.clone();
    for children in [
        vec![],
        vec![FeedRange::full()],
        vec![range("", "10"), range("20", "FF")],
        vec![range("", "20"), range("10", "FF")],
        vec![range("", "10"), range("10", "20")],
    ] {
        assert!(context.split(&mut owned, &children).await.is_err());
        let actual = stored(&context, &root.record.id).await;
        assert_eq!(actual.1, before);
        assert_eq!(context.list().await?.len(), 1);
    }
    Ok(())
}

#[tokio::test]
async fn recovery_creates_only_affected_children_before_returning() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 2).await?;
    let roots = context.list().await?;
    let mut parent = context.acquire(&roots[0]).await?;
    let checkpoint = token(&context, &roots[0].range, "acknowledged");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    let unrelated_id = &roots[1].record.id;
    let mut unrelated = stored(&context, unrelated_id).await;
    unrelated.0.record.ttl = 42;
    let unrelated = context
        .store()
        .replace(unrelated_id, &unrelated.0, unrelated.1)
        .await?;
    let plan_id = format!("{}.plan", context.namespace);
    let mut plan = context.store().read::<RangePlan>(&plan_id).await?.unwrap();
    plan.0.ranges.clear();
    context.store().replace(&plan_id, &plan.0, plan.1).await?;

    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("10")?, Duration::ZERO)
        .await?;
    let parent = Arc::new(Mutex::new(parent));
    assert!(context.recover(parent.clone()).await?);
    let parent = parent.lock().await;
    let children = [
        range("", "10"),
        FeedRange::new(Epk::try_from("10")?, roots[0].range.max_exclusive().clone())?,
    ];
    for range in &children {
        let child = stored(&context, &lease_id(&context.namespace, range))
            .await
            .0;
        assert_eq!(child.range, *range);
        assert_eq!(
            child.continuation,
            Some(context.checkpoint(&parent.value, range)?)
        );
        assert!(child.owner.is_none());
        assert!(child.incarnation.is_none());
    }
    assert_eq!(stored(&context, unrelated_id).await.1, unrelated.1);
    assert_deleted(&context, &parent.value.record.id).await;
    Ok(())
}

#[tokio::test]
async fn recovery_after_takeover_cannot_create_children_from_the_stale_owner() -> Result<()> {
    let (context, emulator) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let old = context.acquire(&root).await?;
    let new = context.acquire(&old.value).await?;
    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
        .await?;

    assert!(context
        .recover(Arc::new(Mutex::new(old)))
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    let children = [range("", "20"), range("20", "FF")];
    for child in &children {
        assert!(context
            .store()
            .read::<LeaseDocument>(&lease_id(&context.namespace, child))
            .await?
            .is_none());
    }
    assert!(context.recover(Arc::new(Mutex::new(new))).await?);
    assert_eq!(
        context
            .list()
            .await?
            .iter()
            .map(|lease| lease.range.clone())
            .collect::<Vec<_>>(),
        children
    );
    Ok(())
}

#[tokio::test]
async fn inventory_excludes_deleted_parents_and_other_workloads() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let children = [range("", "20"), range("20", "FF")];
    context.split(&mut parent, &children).await?;

    let mut foreign = root.clone();
    foreign.record.workload.workload_name = "another-workload".into();
    foreign.record.id = lease_id(&super::namespace(&foreign.record.workload)?, &foreign.range);
    context.store().create(&foreign.record.id, &foreign).await?;
    assert_deleted(&context, &root.record.id).await;
    let leases = context.list().await?;
    assert_eq!(
        leases
            .iter()
            .map(|lease| lease.range.clone())
            .collect::<Vec<_>>(),
        children
    );
    Ok(())
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn failed_child_creation_is_retried_without_blocking_other_acquisitions() -> Result<()> {
    let (context, rule) =
        fault_fixture(FaultOperationType::CreateItem, StatusCode::BadRequest, 1).await?;
    let roots = context.list().await?;
    let mut parent = context.acquire(&roots[0]).await?;
    let children = [
        range("", "10"),
        FeedRange::new(Epk::try_from("10")?, roots[0].range.max_exclusive().clone())?,
    ];
    let state = Arc::new(SharedState::default());
    let (_stop, stopping) = signal();
    let (_force, forced) = signal();
    rule.enable();
    assert_eq!(
        context
            .split(&mut parent, &children)
            .await
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::BadRequest
    );
    rule.disable();
    assert_eq!(rule.hit_count(), 1);
    let retained = stored(&context, &roots[0].record.id).await;
    assert_eq!(retained.0.owner, parent.value.owner);
    assert_eq!(retained.0.incarnation, parent.value.incarnation);
    let before = retained.1;
    let work = balance(
        context.clone(),
        state.clone(),
        [roots[0].record.id.clone()].into_iter().collect(),
        Duration::ZERO,
        stopping.clone(),
        forced.clone(),
    )
    .await;
    assert_eq!(
        work.iter()
            .map(|lease| &lease.value.record.id)
            .collect::<BTreeSet<_>>(),
        roots[1..]
            .iter()
            .map(|lease| &lease.record.id)
            .collect::<BTreeSet<_>>()
    );
    assert!(work.iter().all(|lease| lease.value.owner.is_some()));
    assert_eq!(stored(&context, &roots[0].record.id).await.1, before);
    for child in &children {
        assert!(context
            .store()
            .read::<LeaseDocument>(&lease_id(&context.namespace, child))
            .await?
            .is_none());
    }

    let plan_id = format!("{}.plan", context.namespace);
    context
        .store()
        .execute(CosmosOperation::delete_item(ItemReference::from_name(
            &context.leases,
            PartitionKey::from(plan_id.clone()),
            plan_id,
        )))
        .await?;
    context.update(&mut parent, Update::Release).await?;
    let mut next_owner = context.acquire(&parent.value).await?;
    context.split(&mut next_owner, &children).await?;
    assert_deleted(&context, &roots[0].record.id).await;
    let leases = context.list().await?;
    assert_eq!(leases.len(), roots.len() + 1);
    Ok(())
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn failed_parent_deletion_is_retried_without_rewriting_children() -> Result<()> {
    let (context, rule) =
        fault_fixture(FaultOperationType::DeleteItem, StatusCode::BadRequest, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.range, "saved-parent");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    let children = [
        range("", "10"),
        FeedRange::new(Epk::try_from("10")?, root.range.max_exclusive().clone())?,
    ];
    let stale_etag = parent.etag.clone();
    rule.enable();
    assert_eq!(
        context
            .split(&mut parent, &children)
            .await
            .unwrap_err()
            .status()
            .status_code(),
        StatusCode::BadRequest
    );
    rule.disable();
    assert_eq!(rule.hit_count(), 1);
    let retained = stored(&context, &root.record.id).await;
    assert_eq!(retained.0.owner, parent.value.owner);
    assert_eq!(retained.0.incarnation, parent.value.incarnation);
    assert_eq!(retained.0.continuation, Some(checkpoint));
    assert!(context
        .store()
        .delete(&root.record.id, stale_etag)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert_eq!(stored(&context, &root.record.id).await.1, retained.1);
    let mut before = Vec::new();
    for range in &children {
        let id = lease_id(&context.namespace, range);
        before.push((id.clone(), stored(&context, &id).await.1));
    }
    let mut next_owner = context.acquire(&retained.0).await?;
    assert!(context
        .split(&mut parent, &children)
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    context.split(&mut next_owner, &children).await?;
    assert_deleted(&context, &root.record.id).await;
    for (id, etag) in before {
        assert_eq!(stored(&context, &id).await.1, etag);
    }
    Ok(())
}

#[tokio::test]
async fn restart_verification_rejects_partial_or_overlapping_replacement_coverage() -> Result<()> {
    for overlapping in [false, true] {
        let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
        let root = context.list().await?.remove(0);
        let mut parent = context.acquire(&root).await?;
        let children = [range("", "20"), range("20", "FF")];
        context.split(&mut parent, &children).await?;
        let left_id = lease_id(&context.namespace, &children[0]);
        let left_before = stored(&context, &left_id).await;
        if overlapping {
            let mut overlap = left_before.0.clone();
            overlap.range = range("10", "30");
            overlap.record.id = lease_id(&context.namespace, &overlap.range);
            overlap.continuation = Some(token(&context, &overlap.range, "overlap"));
            context.store().create(&overlap.record.id, &overlap).await?;
        } else {
            let right = stored(&context, &lease_id(&context.namespace, &children[1])).await;
            context.store().delete(&right.0.record.id, right.1).await?;
        }
        assert_eq!(
            context
                .verify_leases()
                .await
                .err()
                .expect("invalid replacement coverage must fail")
                .status()
                .status_code(),
            StatusCode::Conflict
        );
        assert_deleted(&context, &root.record.id).await;
        assert_eq!(stored(&context, &left_id).await.1, left_before.1);
    }
    Ok(())
}

#[tokio::test]
async fn restart_accepts_an_unfinished_descendant_without_recreating_its_ancestor() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::FullFidelity, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.range, "pinned");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    context
        .split(&mut parent, &[range("", "20"), range("20", "FF")])
        .await?;
    let leaves = context.list().await?;
    let mut left = context.acquire(&leaves[0]).await?;
    let children = [range("", "10"), range("10", "20")];
    let mut partial = left.value.clone();
    partial.range = children[0].clone();
    partial.record.id = lease_id(&context.namespace, &partial.range);
    partial.continuation = Some(context.checkpoint(&left.value, &partial.range)?);
    partial.owner = None;
    partial.incarnation = None;
    context.store().create(&partial.record.id, &partial).await?;
    let etag = left.etag.clone();
    context.verify_leases().await?;
    assert_deleted(&context, &root.record.id).await;
    assert_eq!(stored(&context, &left.value.record.id).await.1, etag);
    context.split(&mut left, &children).await?;
    context.verify_leases().await?;
    assert_deleted(&context, &left.value.record.id).await;
    assert_deleted(&context, &root.record.id).await;
    assert_eq!(context.list().await?.len(), 3);
    Ok(())
}

#[tokio::test]
async fn retried_split_preserves_already_replaced_children() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.range, "root-progress");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    let children = [range("", "20"), range("20", "FF")];
    let mut left = parent.value.clone();
    left.range = children[0].clone();
    left.record.id = lease_id(&context.namespace, &left.range);
    left.continuation = Some(context.checkpoint(&parent.value, &left.range)?);
    left.owner = None;
    left.incarnation = None;
    context.store().create(&left.record.id, &left).await?;
    let mut left = context.acquire(&left).await?;
    let advanced = token(&context, &left.value.range, "left-advanced");
    context
        .update(&mut left, Update::Checkpoint(&advanced))
        .await?;
    context
        .split(&mut left, &[range("", "10"), range("10", "20")])
        .await?;
    assert_deleted(&context, &left.value.record.id).await;
    assert_eq!(
        stored(&context, &root.record.id).await.0.incarnation,
        parent.value.incarnation
    );
    context.split(&mut parent, &children).await?;
    assert_deleted(&context, &root.record.id).await;
    assert_deleted(&context, &left.value.record.id).await;
    let leaves = context.list().await?;
    assert_eq!(leaves.len(), 3);
    for leaf in leaves
        .iter()
        .filter(|l| l.range.max_exclusive() <= children[0].max_exclusive())
    {
        assert_eq!(
            leaf.continuation,
            Some(context.checkpoint(&left.value, &leaf.range)?)
        );
    }
    context.verify_leases().await?;
    assert_deleted(&context, &root.record.id).await;
    assert_deleted(&context, &left.value.record.id).await;
    Ok(())
}
