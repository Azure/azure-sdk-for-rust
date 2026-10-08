// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use azure_data_cosmos_driver::{models::ContinuationToken, CosmosError, Result};
use tokio::time::Instant;

use super::{invalid, LeaseRecord};
use crate::{CheckpointError, OwnedLease};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AuthorityState {
    Active { safe_until: Instant },
    WriteInFlight { safe_until: Instant },
    Revoked,
}

pub(super) fn validate_record(record: &LeaseRecord, id: &str, duration_ms: u64) -> Result<()> {
    if record.id != id
        || record.version != 1
        || record.checkpoint.is_empty()
        || record.lease_duration_ms != duration_ms
        || record
            .ownership
            .owner
            .as_ref()
            .is_some_and(|owner| owner.trim().is_empty())
        || (record.ownership.owner.is_some() && record.ownership.generation == 0)
        || record.range.is_logical_partition()
        || record.range.min_inclusive() >= record.range.max_exclusive()
    {
        return Err(invalid(
            "lease record identity, version, range, checkpoint, or duration is invalid",
        ));
    }
    Ok(())
}

pub(super) fn plan_acquisition(
    observed: &LeaseRecord,
    owner: &str,
    elapsed: Duration,
    duration: Duration,
) -> Result<Option<LeaseRecord>> {
    if owner.trim().is_empty() {
        return Err(invalid("lease owner cannot be empty"));
    }

    if observed.ownership.owner.is_some() && elapsed < duration {
        return Ok(None);
    }
    propose_owner(observed, owner).map(Some)
}

pub(super) fn plan_transfer(observed: &LeaseRecord, owner: &str) -> Result<LeaseRecord> {
    if observed.ownership.owner.is_none() || observed.ownership.owner.as_deref() == Some(owner) {
        return Err(invalid(
            "active transfer requires a different observed owner",
        ));
    }
    propose_owner(observed, owner)
}

fn propose_owner(observed: &LeaseRecord, owner: &str) -> Result<LeaseRecord> {
    if owner.trim().is_empty() {
        return Err(invalid("lease owner cannot be empty"));
    }
    let mut proposed = observed.clone();
    proposed.ownership.generation = proposed
        .ownership
        .generation
        .checked_add(1)
        .ok_or_else(|| invalid("lease generation exhausted"))?;
    proposed.ownership.owner = Some(owner.to_owned());
    Ok(proposed)
}

pub(super) fn authority_deadline(
    authority: AuthorityState,
    now: Instant,
    loss_observed: bool,
) -> Result<Instant> {
    match authority {
        AuthorityState::Active { safe_until } if !loss_observed && now < safe_until => {
            Ok(safe_until)
        }
        _ => Err(invalid(
            "lease authority is lost, uncertain, or past its safety deadline",
        )),
    }
}

pub(super) fn plan_checkpoint(
    current: &LeaseRecord,
    expected: &OwnedLease,
    candidate: &ContinuationToken,
) -> Result<LeaseRecord> {
    if expected.id() != current.id
        || Some(expected.owner()) != current.ownership.owner.as_deref()
        || expected.epoch().get() != current.ownership.generation
        || expected.range() != &current.range
        || expected.checkpoint().as_str() != current.checkpoint
        || candidate.as_str().is_empty()
    {
        return Err(invalid("checkpoint has obsolete ownership or progress"));
    }
    let mut proposed = current.clone();
    proposed.checkpoint = candidate.as_str().to_owned();
    Ok(proposed)
}

pub(super) fn plan_release(current: &LeaseRecord) -> LeaseRecord {
    let mut proposed = current.clone();
    proposed.ownership.owner = None;
    proposed
}

pub(super) struct WriteTiming {
    pub started: Instant,
    pub completed: Instant,
    pub previous_deadline: Option<Instant>,
}

pub(super) fn confirm_lease_write(
    expected_revision: &str,
    received_revision: &str,
    timing: WriteTiming,
    safe_duration: Duration,
    loss_observed: bool,
) -> Result<Instant> {
    let safe_until = timing
        .started
        .checked_add(safe_duration)
        .ok_or_else(|| invalid("lease duration exceeds the timer range"))?;
    if received_revision.is_empty() || received_revision == expected_revision {
        return Err(invalid("lease write did not confirm a new store revision"));
    }
    if loss_observed
        || timing.completed < timing.started
        || timing.completed >= safe_until
        || timing
            .previous_deadline
            .is_some_and(|deadline| timing.completed >= deadline)
    {
        return Err(invalid(
            "lease write completed after local ownership became unsafe",
        ));
    }
    Ok(safe_until)
}

pub(super) fn classify_checkpoint_failure(error: CosmosError) -> CheckpointError {
    // A final 412 cannot exclude a committed earlier driver attempt.
    CheckpointError::Ambiguous(error)
}

#[cfg(test)]
mod tests;
