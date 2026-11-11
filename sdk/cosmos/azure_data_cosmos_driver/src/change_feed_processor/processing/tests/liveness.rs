// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    balancing, fixture_with_emulator, insert, leases, next, now_millis, options, start,
    ChangeFeedFactoryMode, CheckpointPolicy, ContinuationToken, Lease, Result, Update,
};
use std::{sync::Arc, time::Duration};

#[test]
fn processing_expiration_uses_exact_policy_thresholds() -> Result<()> {
    let now = 1_000;
    assert_eq!(
        CheckpointPolicy::EveryBatch.processing_deadline(now)?,
        601_000
    );
    assert_eq!(
        CheckpointPolicy::coalesced().processing_deadline(now)?,
        601_000
    );
    assert_eq!(
        CheckpointPolicy::Coalesced {
            max_interval: Duration::from_secs(45),
        }
        .processing_deadline(now)?,
        91_000
    );
    assert_eq!(
        CheckpointPolicy::Coalesced {
            max_interval: Duration::from_nanos(1),
        }
        .processing_deadline(now)?,
        1_001
    );
    assert!(CheckpointPolicy::Coalesced {
        max_interval: Duration::MAX,
    }
    .validate()
    .is_err());
    assert!(CheckpointPolicy::EveryBatch
        .processing_deadline(i64::MAX)
        .is_err());
    Ok(())
}

#[test]
fn processing_expiry_is_independent_of_renewal_and_respects_owner_limits() {
    let mut all = leases(&[Some("a"), Some("b")]);
    all[0].processing_deadline = Some(1_000);
    assert!(balancing::select(&all, &options("b"), 999).is_empty());
    assert_eq!(balancing::select(&all, &options("b"), 1_000), vec![0]);
    assert!(balancing::select(&all, &options("b").with_lease_counts(0, 1), 1_000).is_empty());

    all.push(leases(&[None]).remove(0));
    assert!(balancing::select(&all, &options("a").with_lease_counts(0, 1), 1_000).is_empty());
}

#[test]
fn balancing_uses_the_persisted_owner_deadline_not_the_observers_policy() {
    let mut all = leases(&[Some("a"), Some("b")]);
    all[0].processing_deadline = Some(2_000);
    let observer = options("b").with_checkpoint_policy(CheckpointPolicy::Coalesced {
        max_interval: Duration::from_millis(1),
    });
    assert!(balancing::select(&all, &observer, 1_001).is_empty());
    all[0].renewed_at = Some(2_000);
    all[1].renewed_at = Some(2_000);
    assert_eq!(balancing::select(&all, &observer, 2_000), vec![0]);
}

#[tokio::test]
async fn activity_is_published_by_renewal_without_persisting_pending_progress() -> Result<()> {
    for policy in [CheckpointPolicy::EveryBatch, CheckpointPolicy::coalesced()] {
        let (mut context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
        Arc::get_mut(&mut context)
            .unwrap()
            .options
            .checkpoint_policy = policy;
        let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
        let id = owned.value.base.record.id.clone();
        let initial_deadline = owned.value.processing_deadline.unwrap();
        let processed_at = now_millis() + 1_000;
        owned
            .value
            .acknowledge(ContinuationToken::from_string("pending".into()), 1);
        owned.value.processed(policy, processed_at)?;
        let before = context.store().read::<Lease>(&id).await?.unwrap();
        assert!(before.value.last_processed_at.is_none());
        assert!(before.value.base.continuation.is_none());

        context.update(&mut owned, Update::Renew).await?;
        let stored = context.store().read::<Lease>(&id).await?.unwrap();
        assert_eq!(stored.value.last_processed_at, Some(processed_at));
        assert_eq!(
            stored.value.processing_deadline,
            Some(policy.processing_deadline(processed_at)?)
        );
        assert!(stored.value.processing_deadline.unwrap() > initial_deadline);
        assert!(stored.value.base.continuation.is_none());
        assert!(stored.value.pending.is_none());
        assert_eq!(
            owned.value.pending.as_ref().unwrap().token.as_str(),
            "pending"
        );

        let deadline = owned.value.processing_deadline;
        let dirty_since = owned.value.pending.as_ref().unwrap().since;
        owned
            .value
            .acknowledge(ContinuationToken::from_string("idle".into()), 0);
        context.update(&mut owned, Update::Renew).await?;
        assert_eq!(owned.value.last_processed_at, Some(processed_at));
        assert_eq!(owned.value.processing_deadline, deadline);
        assert_eq!(owned.value.pending.as_ref().unwrap().since, dirty_since);
        assert!(owned.value.base.continuation.is_none());

        let token = ContinuationToken::from_string("durable".into());
        context
            .update(&mut owned, Update::Checkpoint(&token))
            .await?;
        assert_eq!(owned.value.last_processed_at, Some(processed_at));
        assert_eq!(owned.value.processing_deadline, deadline);
        assert_eq!(owned.value.base.continuation, Some(token));
    }
    Ok(())
}

#[tokio::test]
async fn stale_activity_cannot_replace_newer_activity_or_reacquire_recovered_work() -> Result<()> {
    let (context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    let mut owned = context.acquire(&context.list().await?.remove(0)).await?;
    owned.value.processing_deadline = Some(now_millis() - 1);
    let mut owned = context
        .store()
        .replace(&owned.value.base.record.id, &owned.value, owned.etag)
        .await?;
    let expired = owned.value.clone();
    let mut stale = context
        .store()
        .read::<Lease>(&owned.value.base.record.id)
        .await?
        .unwrap();
    let now = now_millis();
    owned
        .value
        .processed(context.options.checkpoint_policy, now)?;
    context.update(&mut owned, Update::Renew).await?;
    context.update(&mut stale, Update::Renew).await?;
    assert_eq!(stale.value.last_processed_at, Some(now));
    assert_eq!(
        stale.value.processing_deadline,
        owned.value.processing_deadline
    );
    assert!(context
        .acquire(&expired)
        .await
        .err()
        .expect("stale acquisition")
        .status()
        .is_precondition_failed());
    Ok(())
}

#[tokio::test]
async fn stalled_callback_is_reclaimed_before_renewal_expires_and_old_receipt_is_fenced(
) -> Result<()> {
    let (mut context, _) = fixture_with_emulator(ChangeFeedFactoryMode::Incremental, 1).await?;
    Arc::get_mut(&mut context)
        .unwrap()
        .options
        .checkpoint_policy = CheckpointPolicy::Coalesced {
        max_interval: Duration::from_millis(100),
    };
    insert(&context, "replayed").await?;
    let first = start(&context, context.options.clone()).await?;
    let (id, _, receipt) = next(&first).await.into_parts();
    let original = context.store().read::<Lease>(&id).await?.unwrap().value;
    assert!(original.base.continuation.is_none());
    assert!(original.last_processed_at.is_none());

    let mut second_options = context.options.clone();
    second_options.instance_name = "second".into();
    second_options.checkpoint_policy = CheckpointPolicy::EveryBatch;
    let second = start(&context, second_options).await?;
    let (replacement_id, response, replacement_receipt) = next(&second).await.into_parts();
    assert_eq!(replacement_id, id);
    let body: serde_json::Value = response.into_body().into_single()?;
    assert_eq!(body["Documents"][0]["current"]["id"], "replayed");
    let replacement = context.store().read::<Lease>(&id).await?.unwrap().value;
    assert_ne!(replacement.incarnation, original.incarnation);
    assert_eq!(replacement.base.owner.as_deref(), Some("second"));
    assert!(replacement.base.continuation.is_none());
    assert!(replacement.processing_deadline.unwrap() > now_millis() + 500_000);

    assert!(receipt.checkpoint().await.is_err());
    let _ = first.abort().await;
    replacement_receipt.acknowledge().await?;
    let saved = context.store().read::<Lease>(&id).await?.unwrap().value;
    assert_eq!(saved.incarnation, replacement.incarnation);
    assert!(saved.last_processed_at.is_some());
    assert!(saved.base.continuation.is_some());
    second.stop().await?;
    Ok(())
}
