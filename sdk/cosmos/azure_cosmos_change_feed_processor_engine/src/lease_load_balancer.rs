// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos_driver::{CosmosError, Result};
use std::collections::{BTreeMap, HashSet};

/// Policy inputs for one logical lease; expiry and eligibility are explicit.
#[derive(Clone, Debug)]
pub struct BalanceLease {
    id: String,
    owner: Option<String>,
    expired: bool,
    eligible: bool,
}
impl BalanceLease {
    /// Creates an eligible unowned candidate.
    ///
    /// # Errors
    ///
    /// Rejects an empty lease identity.
    pub fn new(id: impl Into<String>) -> Result<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(invalid("balancing lease identity cannot be empty"));
        }
        Ok(Self {
            id,
            owner: None,
            expired: false,
            eligible: true,
        })
    }
    /// Sets an observed owner; an empty owner is rejected by planning.
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }
    /// Sets whether unchanged ownership has exceeded its observation interval.
    pub fn with_expired(mut self, expired: bool) -> Self {
        self.expired = expired;
        self
    }
    /// Sets whether workload/generation/lifecycle permits processing.
    pub fn with_eligible(mut self, eligible: bool) -> Self {
        self.eligible = eligible;
        self
    }
    /// Returns the logical lease identity.
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Returns the observed owner.
    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }
    /// Returns explicit expiry state.
    pub fn expired(&self) -> bool {
        self.expired
    }
    /// Returns explicit eligibility state.
    pub fn eligible(&self) -> bool {
        self.eligible
    }
}

/// The reason to attempt one conditional ownership change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BalanceActionKind {
    /// Prefer unused or expired logical work.
    Pickup,
    /// Transfer one active lease from an owner at least two leases ahead.
    Transfer,
}

/// A selected candidate, not permission to start processing.
#[derive(Clone, Debug)]
pub struct BalanceAction {
    id: String,
    kind: BalanceActionKind,
}
impl BalanceAction {
    /// Returns the selected logical lease.
    pub fn lease_id(&self) -> &str {
        &self.id
    }
    /// Returns pickup versus active-owner transfer.
    pub fn kind(&self) -> BalanceActionKind {
        self.kind
    }
}

fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

/// Selects at most one equal-count balancing candidate without I/O or clock reads.
///
/// Membership is distinct non-expired owners plus this worker, including when
/// it owns zero leases. Other zero-lease workers are not globally discovered.
/// The ceiling target is not a mandate to move work in an already balanced
/// distribution; active transfer requires a donor/recipient gap of at least two.
///
/// `tie_break` explicitly rotates deterministic ID/owner order for controlled
/// tests or contention avoidance. Selection does not grant ownership.
///
/// # Errors
///
/// Rejects empty workers/owners and duplicate snapshot identities.
pub fn plan_equal_lease_balance(
    snapshot: &[BalanceLease],
    worker: &str,
    tie_break: u64,
) -> Result<Option<BalanceAction>> {
    if worker.trim().is_empty() {
        return Err(invalid("balancing requires a worker incarnation"));
    }
    let mut ids = HashSet::new();
    for lease in snapshot {
        if !ids.insert(&lease.id)
            || lease
                .owner
                .as_ref()
                .is_some_and(|owner| owner.trim().is_empty())
        {
            return Err(invalid(
                "balancing snapshot has duplicate identities or invalid owners",
            ));
        }
    }
    let eligible: Vec<_> = snapshot.iter().filter(|lease| lease.eligible).collect();
    if eligible.is_empty() {
        return Ok(None);
    }
    let mut counts = BTreeMap::from([(worker, 0_u64)]);
    for lease in &eligible {
        if !lease.expired {
            if let Some(owner) = lease.owner.as_deref() {
                *counts.entry(owner).or_default() += 1;
            }
        }
    }
    let target = (eligible.len() as u64).div_ceil(counts.len() as u64);
    let current = counts[worker];
    if current >= target {
        return Ok(None);
    }
    let mut unused: Vec<_> = eligible
        .iter()
        .copied()
        .filter(|lease| lease.expired || lease.owner.is_none())
        .collect();
    unused.sort_by(|a, b| a.id.cmp(&b.id));
    if !unused.is_empty() {
        let lease = unused[(tie_break % unused.len() as u64) as usize];
        return Ok(Some(BalanceAction {
            id: lease.id.clone(),
            kind: BalanceActionKind::Pickup,
        }));
    }
    let maximum = counts
        .values()
        .copied()
        .max()
        .expect("current worker is in membership");
    if maximum < current + 2 {
        return Ok(None);
    }
    let donors: Vec<_> = counts
        .iter()
        .filter(|(owner, count)| **owner != worker && **count == maximum)
        .map(|(owner, _)| *owner)
        .collect();
    let donor = donors[(tie_break % donors.len() as u64) as usize];
    let mut candidates: Vec<_> = eligible
        .into_iter()
        .filter(|lease| lease.owner.as_deref() == Some(donor))
        .collect();
    candidates.sort_by(|a, b| a.id.cmp(&b.id));
    let lease = candidates[(tie_break % candidates.len() as u64) as usize];
    Ok(Some(BalanceAction {
        id: lease.id.clone(),
        kind: BalanceActionKind::Transfer,
    }))
}

#[cfg(test)]
mod tests;
