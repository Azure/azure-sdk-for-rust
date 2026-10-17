// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    bootstrap, fixture_with_emulator, next, start, Batch, ChangeFeedProcessorMode, Context, Lease,
    Processor, ProcessorOptions, Update,
};
use crate::{
    change_feed_processor::bootstrap::{lease_id, store::Stored, RangePlan},
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
    options.bootstrap.instance_name = worker.into();
    options.expiration = Duration::from_millis(600);
    options.bootstrap.verify_leases_on_restart = true;
    options
}

fn keys(ranges: &[FeedRange]) -> Vec<String> {
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

async fn publish(
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

async fn wait_for(context: &Context, predicate: impl Fn(&[Lease]) -> bool) -> Vec<Lease> {
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

async fn next_from(first: &Processor, second: &Processor) -> Batch {
    tokio::time::timeout(Duration::from_secs(10), async {
        tokio::select! {
            batch = first.next_batch() => batch.unwrap(),
            batch = second.next_batch() => batch.unwrap(),
        }
    })
    .await
    .expect("both processors stopped making progress")
}

async fn stored(context: &Context, id: &str) -> Stored<Lease> {
    context
        .store()
        .read(id)
        .await
        .unwrap()
        .expect("lease exists")
}

fn token(context: &Context, scope: &FeedRange, value: &str) -> ContinuationToken {
    ContinuationToken::encode_v1(
        &context.operation(scope),
        &PipelineNodeState::UnorderedMerge {
            active_tokens: vec![RangedToken {
                min_epk: scope.min_inclusive().to_hex(),
                max_epk: scope.max_exclusive().to_hex(),
                server_continuation: value.into(),
            }],
            start_from: Some(context.options.start.clone()),
        },
    )
    .unwrap()
}

#[tokio::test]
async fn split_with_pending_batch_preserves_coverage_and_rebalances_children() -> Result<()> {
    for mode in [
        ChangeFeedProcessorMode::LatestVersion,
        ChangeFeedProcessorMode::AllVersionsAndDeletes,
    ] {
        let (context, emulator) = fixture_with_emulator(mode, 1).await?;
        let children = vec![range("", "20"), range("20", "FF")];
        let keys = keys(&children);
        let first = start(&context, config(&context, "first", 1)).await?;
        wait_for(&context, |leases| {
            leases.iter().all(|lease| lease.base.continuation.is_some())
        })
        .await;
        let second = start(&context, config(&context, "second", 1)).await?;
        let mut expected = BTreeMap::new();
        publish(&context, &keys, 0..6, &mut expected).await?;
        let (parent_id, response, receipt) = next(&first).await.into_parts();
        let mut sink = Sink::default();
        sink.observe(response, &expected)?;
        let before = stored(&context, &parent_id).await.value.base.continuation;
        emulator
            .store()
            .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
            .await?;
        assert_eq!(
            stored(&context, &parent_id).await.value.base.continuation,
            before
        );
        assert!(stored(&context, &parent_id).await.value.children.is_none());
        assert!(
            tokio::time::timeout(Duration::from_millis(50), first.next_batch())
                .await
                .is_err()
        );
        receipt.checkpoint().await?;
        let active = wait_for(&context, |leases| {
            leases.len() == 2
                && leases
                    .iter()
                    .filter_map(|lease| lease.base.owner.as_ref())
                    .collect::<BTreeSet<_>>()
                    .len()
                    == 2
        })
        .await;
        assert_eq!(
            active
                .iter()
                .map(|lease| lease.base.range.clone())
                .collect::<Vec<_>>(),
            children
        );
        let retired = stored(&context, &parent_id).await.value;
        assert_eq!(retired.children, Some(children));
        assert!(retired.base.owner.is_none());
        assert_eq!(retired.base.record.workload, context.identity);
        assert!(active
            .iter()
            .all(|lease| lease.base.record.workload == context.identity));
        assert!(first.is_running());
        assert!(second.is_running());
        let mut delivered_leases = BTreeSet::new();
        while sink.seen.len() < expected.len() {
            let (id, response, receipt) = next_from(&first, &second).await.into_parts();
            delivered_leases.insert(id);
            sink.observe(response, &expected)?;
            receipt.checkpoint().await?;
        }
        sink.complete(&expected);
        assert_eq!(
            delivered_leases,
            active
                .iter()
                .map(|lease| lease.base.record.id.clone())
                .collect()
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
    let (context, emulator) =
        fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
    let mut expected = BTreeMap::new();
    let keys = keys(&[range("", "20"), range("20", "FF")]);
    publish(&context, &keys, 0..4, &mut expected).await?;
    let first = start(&context, config(&context, "first", 1)).await?;
    let (id, response, receipt) = next(&first).await.into_parts();
    let mut sink = Sink::default();
    sink.observe(response, &expected)?;
    let checkpoint = stored(&context, &id).await.value.base.continuation;
    emulator
        .store()
        .split_partition_and_wait("polling", "source", 0, Epk::try_from("20")?, Duration::ZERO)
        .await?;
    first.abort().await?;
    assert!(receipt.checkpoint().await.is_err());
    assert_eq!(
        stored(&context, &id).await.value.base.continuation,
        checkpoint
    );
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
    let (context, emulator) =
        fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
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
    let parent = stored(&context, &id).await.value;
    assert!(parent.base.owner.is_none());
    assert!(parent.base.continuation.is_some());
    assert!(
        parent.children.is_none(),
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
        ChangeFeedProcessorMode::LatestVersion,
        ChangeFeedProcessorMode::AllVersionsAndDeletes,
    ] {
        exercise_merge(mode).await?;
    }
    Ok(())
}

async fn exercise_merge(mode: ChangeFeedProcessorMode) -> Result<()> {
    let (context, emulator) = fixture_with_emulator(mode, 2).await?;
    let initial = context.list().await?;
    let initial_ids: BTreeSet<_> = initial
        .iter()
        .map(|lease| lease.base.record.id.clone())
        .collect();
    let keys = keys(
        &initial
            .iter()
            .map(|lease| lease.base.range.clone())
            .collect::<Vec<_>>(),
    );
    let first = start(&context, config(&context, "first", 1)).await?;
    let second = start(&context, config(&context, "second", 1)).await?;
    wait_for(&context, |leases| {
        leases.iter().all(|lease| lease.base.continuation.is_some())
    })
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
    let held_checkpoint = stored(&context, &held_id).await.value.base.continuation;
    let other_checkpoint = stored(&context, &other_id).await.value.base.continuation;
    assert_ne!(held_checkpoint, other_checkpoint);
    emulator
        .store()
        .merge_partitions_and_wait("polling", "source", 0, 1, Duration::ZERO)
        .await?;
    assert_eq!(
        stored(&context, &held_id).await.value.base.continuation,
        held_checkpoint
    );
    assert_eq!(
        stored(&context, &other_id).await.value.base.continuation,
        other_checkpoint
    );
    other_receipt.checkpoint().await?;
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
            .map(|lease| lease.base.record.id.clone())
            .collect::<BTreeSet<_>>(),
        initial_ids
    );
    assert!(active.iter().all(|lease| lease.children.is_none()));
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
async fn frozen_parent_recovers_partial_creation_without_resetting_a_competing_child() -> Result<()>
{
    for partial in [false, true] {
        let (context, _) = fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
        let root = context.list().await?.remove(0);
        let mut owned = context.acquire(&root).await?;
        let checkpoint = token(&context, &root.base.range, "parent-acknowledged");
        context
            .update(&mut owned, Update::Checkpoint(&checkpoint))
            .await?;
        let stale = owned.value.clone();
        let children = [range("", "20"), range("20", "FF")];
        context.update(&mut owned, Update::Split(&children)).await?;
        assert!(context
            .acquire(&stale)
            .await
            .err()
            .expect("retired parent cannot be acquired")
            .status()
            .is_precondition_failed());
        let left_id = lease_id(&context.namespace, &children[0]);
        let right_id = lease_id(&context.namespace, &children[1]);
        assert!(context.store().read::<Lease>(&left_id).await?.is_none());
        assert!(context.store().read::<Lease>(&right_id).await?.is_none());
        let advanced = token(&context, &children[0], "child-advanced");
        if partial {
            let mut left = owned.value.clone();
            left.base.record.id = left_id.clone();
            left.base.range = children[0].clone();
            left.base.continuation = Some(advanced.clone());
            left.children = None;
            assert!(context.store().create(&left_id, &left).await?);
        }
        let (first, second) = futures::join!(context.list(), context.list());
        assert_eq!(first?.len(), 2);
        assert_eq!(second?.len(), 2);
        assert_eq!(
            stored(&context, &left_id).await.value.base.continuation,
            Some(if partial {
                advanced
            } else {
                context.checkpoint(&owned.value, &children[0])?
            })
        );
        assert_eq!(
            stored(&context, &right_id).await.value.base.continuation,
            Some(context.checkpoint(&owned.value, &children[1])?)
        );
        let before = stored(&context, &root.base.record.id).await;
        let mut options = context.options.bootstrap.clone();
        options.verify_leases_on_restart = true;
        bootstrap(
            &context.source_driver,
            &context.source,
            &context.lease_driver,
            &context.leases,
            &options,
        )
        .await?;
        let after = stored(&context, &root.base.record.id).await;
        assert_eq!(after.etag, before.etag);
        assert_eq!(
            after.value.base.continuation,
            before.value.base.continuation
        );
        let plan = context
            .store()
            .read::<RangePlan>(&format!("{}.plan", context.namespace))
            .await?
            .unwrap();
        assert_eq!(plan.value.ranges, [FeedRange::full()]);
        assert_eq!(context.list().await?.len(), 2);
    }
    Ok(())
}

#[tokio::test]
async fn stale_owner_cannot_retire_parent_or_advance_after_takeover() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
    let root = context.list().await?.remove(0);
    let mut old = context.acquire(&root).await?;
    let mut new = context.acquire(&old.value).await?;
    let children = [range("", "20"), range("20", "FF")];
    assert!(context
        .update(&mut old, Update::Split(&children))
        .await
        .unwrap_err()
        .status()
        .is_precondition_failed());
    assert!(stored(&context, &root.base.record.id)
        .await
        .value
        .children
        .is_none());
    context.update(&mut new, Update::Split(&children)).await?;
    let checkpoint = token(&context, &root.base.range, "stale");
    for update in [
        Update::Renew,
        Update::Release,
        Update::Checkpoint(&checkpoint),
        Update::Split(&children),
    ] {
        assert!(context
            .update(&mut old, update)
            .await
            .unwrap_err()
            .status()
            .is_precondition_failed());
    }
    let mut incompatible = new.value;
    incompatible.base.record.workload.version += 1;
    assert!(context.validate(&incompatible).is_err());
    Ok(())
}

#[tokio::test]
async fn descendant_split_projects_saved_state_and_keeps_disjoint_coverage() -> Result<()> {
    let (context, _) =
        fixture_with_emulator(ChangeFeedProcessorMode::AllVersionsAndDeletes, 1).await?;
    let root = context.list().await?.remove(0);
    let mut parent = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.base.range, "pinned");
    context
        .update(&mut parent, Update::Checkpoint(&checkpoint))
        .await?;
    context
        .update(
            &mut parent,
            Update::Split(&[range("", "20"), range("20", "FF")]),
        )
        .await?;
    let leaves = context.list().await?;
    let mut left = context.acquire(&leaves[0]).await?;
    let checkpoint = token(&context, &left.value.base.range, "left-advanced");
    context
        .update(&mut left, Update::Checkpoint(&checkpoint))
        .await?;
    context
        .update(
            &mut left,
            Update::Split(&[range("", "10"), range("10", "20")]),
        )
        .await?;
    let leaves = context.list().await?;
    assert_eq!(
        leaves
            .iter()
            .map(|lease| lease.base.range.clone())
            .collect::<Vec<_>>(),
        [range("", "10"), range("10", "20"), range("20", "FF")]
    );
    for (lease, expected) in leaves
        .iter()
        .zip(["left-advanced", "left-advanced", "pinned"])
    {
        let ResolvedToken::ClientV1(state) = lease.base.continuation.as_ref().unwrap().resolve()?
        else {
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
            lease.base.range.min_inclusive().to_hex()
        );
        assert_eq!(
            active_tokens[0].max_epk,
            lease.base.range.max_exclusive().to_hex()
        );
    }
    Ok(())
}

#[tokio::test]
async fn unfinished_split_surfaces_1007_without_retiring_a_lease() -> Result<()> {
    let (context, emulator) =
        fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
    let root = context.list().await?.remove(0);
    let owned = Arc::new(Mutex::new(context.acquire(&root).await?));
    let mut plan = context
        .source_driver
        .plan_operation(
            context.operation(&root.base.range),
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
    assert!(!context.recover(owned.clone()).await?);
    assert!(stored(&context, &root.base.record.id)
        .await
        .value
        .children
        .is_none());
    transition.complete().await?;
    assert!(context.recover(owned).await?);
    assert_eq!(context.list().await?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn concurrent_retirement_has_one_winner_and_cannot_replace_its_checkpoint() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
    let root = context.list().await?.remove(0);
    let mut first = context.acquire(&root).await?;
    let checkpoint = token(&context, &root.base.range, "acknowledged");
    context
        .update(&mut first, Update::Checkpoint(&checkpoint))
        .await?;
    let mut second = stored(&context, &root.base.record.id).await;
    let children = [range("", "20"), range("20", "FF")];
    let (one, two) = futures::join!(
        context.update(&mut first, Update::Split(&children)),
        context.update(&mut second, Update::Split(&children)),
    );
    assert_eq!(usize::from(one.is_ok()) + usize::from(two.is_ok()), 1);
    let rejected = one.err().or_else(|| two.err()).unwrap();
    assert!(rejected.status().is_precondition_failed());
    assert_eq!(
        stored(&context, &root.base.record.id)
            .await
            .value
            .base
            .continuation,
        Some(checkpoint)
    );
    let (one, two) = futures::join!(context.list(), context.list());
    assert_eq!(one?.len(), 2);
    assert_eq!(two?.len(), 2);
    Ok(())
}

#[tokio::test]
async fn retirement_rejects_gaps_overlaps_and_non_splits_without_mutation() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedProcessorMode::LatestVersion, 1).await?;
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
        assert!(context
            .update(&mut owned, Update::Split(&children))
            .await
            .is_err());
        let actual = stored(&context, &root.base.record.id).await;
        assert_eq!(actual.etag, before);
        assert!(actual.value.children.is_none());
    }
    Ok(())
}
