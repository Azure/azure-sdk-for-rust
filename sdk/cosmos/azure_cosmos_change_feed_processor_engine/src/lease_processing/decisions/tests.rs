// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{checkpoint_retry, validate_checkpoint_receipt, CheckpointRetry};
use crate::{CheckpointError, OwnedLease};
use azure_data_cosmos_driver::{
    models::{ContinuationToken, FeedRange},
    CosmosError,
};
use std::num::{NonZeroU32, NonZeroU64};

fn lease(owner: &str, epoch: u64, revision: &str, checkpoint: &str) -> OwnedLease {
    OwnedLease::new(
        "lease",
        owner,
        NonZeroU64::new(epoch).unwrap(),
        revision,
        FeedRange::full(),
        ContinuationToken::from_string(checkpoint.into()),
    )
    .unwrap()
}

#[test]
fn receipt_must_confirm_exact_candidate_new_revision_and_same_authority() {
    let previous = lease("A", 1, "old-revision", "durable");
    let candidate = ContinuationToken::from_string("candidate".into());
    let good = lease("A", 1, "new-revision", "candidate");
    validate_checkpoint_receipt(&previous, &candidate, &good).unwrap();
    for bad in [
        lease("A", 1, "old-revision", "candidate"),
        lease("A", 1, "new-revision", "different-progress"),
        lease("B", 1, "new-revision", "candidate"),
        lease("A", 2, "new-revision", "candidate"),
    ] {
        assert!(validate_checkpoint_receipt(&previous, &candidate, &bad).is_err());
    }
    let wrong_range = OwnedLease::new(
        "lease",
        "A",
        NonZeroU64::new(1).unwrap(),
        "new-revision",
        FeedRange::new("40".try_into().unwrap(), "80".try_into().unwrap()).unwrap(),
        candidate.clone(),
    )
    .unwrap();
    assert!(validate_checkpoint_receipt(&previous, &candidate, &wrong_range).is_err());
}

#[test]
fn only_definitely_not_persisted_failures_retry_within_the_attempt_budget() {
    let error = || {
        CosmosError::builder()
            .with_message("persistence outcome")
            .build()
    };
    let limit = NonZeroU32::new(3).unwrap();
    assert_eq!(
        checkpoint_retry(&CheckpointError::Retryable(error()), 1, limit),
        CheckpointRetry::Retry
    );
    assert_eq!(
        checkpoint_retry(&CheckpointError::Retryable(error()), 2, limit),
        CheckpointRetry::Retry
    );
    assert_eq!(
        checkpoint_retry(&CheckpointError::Retryable(error()), 3, limit),
        CheckpointRetry::Fail
    );
    assert_eq!(
        checkpoint_retry(&CheckpointError::Ambiguous(error()), 1, limit),
        CheckpointRetry::Fail
    );
    assert_eq!(
        checkpoint_retry(&CheckpointError::Rejected(error()), 1, limit),
        CheckpointRetry::Fail
    );
}
