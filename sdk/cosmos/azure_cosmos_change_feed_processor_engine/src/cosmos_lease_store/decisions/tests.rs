// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    authority_deadline, classify_checkpoint_failure, confirm_lease_write, plan_acquisition,
    plan_checkpoint, plan_release, validate_record, AuthorityState, WriteTiming,
};
use crate::cosmos_lease_store::{invalid, owned, LeaseIdentity, LeaseRecord};
use crate::CheckpointError;
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    models::{ContinuationToken, FeedRange},
    CosmosError, CosmosStatus,
};
use std::{collections::BTreeMap, time::Duration};
use tokio::time::Instant;

fn record(owner: Option<&str>, generation: u64) -> LeaseRecord {
    LeaseRecord {
        id: "lease".into(),
        version: 1,
        ownership: LeaseIdentity {
            owner: owner.map(str::to_owned),
            generation,
        },
        range: FeedRange::full(),
        checkpoint: "durable".into(),
        lease_duration_ms: 2000,
        extra: BTreeMap::from([("pk".into(), serde_json::json!("P"))]),
    }
}

#[test]
fn acquisition_eligibility_uses_explicit_elapsed_and_preserves_recovery() {
    let observed = record(Some("previous"), 4);
    assert!(plan_acquisition(
        &observed,
        "next",
        Duration::from_millis(1999),
        Duration::from_secs(2)
    )
    .unwrap()
    .is_none());
    let proposed = plan_acquisition(
        &observed,
        "next",
        Duration::from_secs(2),
        Duration::from_secs(2),
    )
    .unwrap()
    .unwrap();
    assert_eq!(proposed.ownership.owner.as_deref(), Some("next"));
    assert_eq!(proposed.ownership.generation, 5);
    assert_eq!(proposed.range, observed.range);
    assert_eq!(proposed.checkpoint, "durable");
    assert_eq!(proposed.extra, observed.extra);
    assert_eq!(observed.ownership.owner.as_deref(), Some("previous"));
    let released = record(None, 4);
    assert!(
        plan_acquisition(&released, "next", Duration::ZERO, Duration::from_secs(2))
            .unwrap()
            .is_some()
    );
    assert!(plan_acquisition(&released, " ", Duration::ZERO, Duration::from_secs(2)).is_err());
    assert!(plan_acquisition(
        &record(None, u64::MAX),
        "next",
        Duration::ZERO,
        Duration::from_secs(2)
    )
    .is_err());
}

#[test]
fn checkpoint_and_release_plans_preserve_ownership_or_progress_as_required() {
    let current = record(Some("A"), 3);
    let expected = owned(&current, "revision-before-renewal").unwrap();
    let candidate = ContinuationToken::from_string("candidate".into());
    let proposed = plan_checkpoint(&current, &expected, &candidate).unwrap();
    assert_eq!(proposed.checkpoint, "candidate");
    assert_eq!(proposed.ownership.owner.as_deref(), Some("A"));
    assert_eq!(proposed.ownership.generation, 3);
    assert_eq!(proposed.extra, current.extra);
    assert_eq!(proposed.range, current.range);
    assert_eq!(current.checkpoint, "durable");
    for obsolete in [record(Some("B"), 3), record(Some("A"), 4)] {
        assert!(plan_checkpoint(&obsolete, &expected, &candidate).is_err());
    }
    let mut advanced = current.clone();
    advanced.checkpoint = "other-candidate".into();
    assert!(plan_checkpoint(&advanced, &expected, &candidate).is_err());
    assert!(plan_checkpoint(
        &current,
        &expected,
        &ContinuationToken::from_string(String::new())
    )
    .is_err());
    let released = plan_release(&current);
    assert!(released.ownership.owner.is_none());
    assert_eq!(released.ownership.generation, 3);
    assert_eq!(released.checkpoint, "durable");
    assert_eq!(released.extra, current.extra);
}

#[test]
fn authority_requires_active_state_before_the_exact_deadline() {
    let now = Instant::now();
    let deadline = now + Duration::from_secs(2);
    let active = AuthorityState::Active {
        safe_until: deadline,
    };
    assert_eq!(authority_deadline(active, now, false).unwrap(), deadline);
    assert!(authority_deadline(active, deadline, false).is_err());
    assert!(authority_deadline(active, now, true).is_err());
    assert!(authority_deadline(
        AuthorityState::WriteInFlight {
            safe_until: deadline
        },
        now,
        false
    )
    .is_err());
    assert!(authority_deadline(AuthorityState::Revoked, now, false).is_err());
}

#[test]
fn write_confirmation_fences_revisions_loss_and_old_and_new_time_windows() {
    let began = Instant::now();
    let old_deadline = began + Duration::from_millis(500);
    let safe_duration = Duration::from_secs(1);
    let confirm = |completed, previous_deadline, revision, lost| {
        confirm_lease_write(
            "old",
            revision,
            WriteTiming {
                started: began,
                completed,
                previous_deadline,
            },
            safe_duration,
            lost,
        )
    };
    assert_eq!(
        confirm(
            began + Duration::from_millis(100),
            Some(old_deadline),
            "new",
            false
        )
        .unwrap(),
        began + safe_duration
    );
    assert!(confirm(old_deadline, Some(old_deadline), "new", false).is_err());
    assert!(confirm(began + safe_duration, None, "new", false).is_err());
    assert!(confirm(began, Some(old_deadline), "old", false).is_err());
    assert!(confirm(began, Some(old_deadline), "", false).is_err());
    assert!(confirm(began, Some(old_deadline), "new", true).is_err());
}

#[test]
fn invalid_records_and_checkpoint_failure_classification_are_explicit() {
    let current = record(Some("A"), 1);
    validate_record(&current, "lease", 2000).unwrap();
    assert!(validate_record(&current, "another-lease", 2000).is_err());
    assert!(validate_record(&current, "lease", 3000).is_err());
    assert!(validate_record(&record(Some("A"), 0), "lease", 2000).is_err());
    let conflict = CosmosError::builder()
        .with_status(CosmosStatus::new(StatusCode::PreconditionFailed))
        .with_message("CAS conflict")
        .build();
    assert!(matches!(
        classify_checkpoint_failure(conflict),
        CheckpointError::Rejected(_)
    ));
    assert!(matches!(
        classify_checkpoint_failure(invalid("unknown outcome")),
        CheckpointError::Ambiguous(_)
    ));
}
