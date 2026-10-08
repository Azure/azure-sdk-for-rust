// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::HashSet;

use azure_data_cosmos_driver::{models::FeedRange, CosmosError, Result};

use crate::OwnedLease;

/// An unchanged logical lease and the physical partitions that now overlap it.
///
/// This plan does not replace assignments or authorize new owners.
#[derive(Clone, Debug)]
pub struct LeaseTopologyAssignment {
    lease: OwnedLease,
    physical_ranges: Vec<FeedRange>,
}

impl LeaseTopologyAssignment {
    /// Returns the original lease, including its unchanged durable checkpoint.
    pub fn lease(&self) -> &OwnedLease {
        &self.lease
    }
    /// Returns the overlapping physical ranges in EPK order.
    pub fn physical_ranges(&self) -> &[FeedRange] {
        &self.physical_ranges
    }
    /// Returns whether one logical lease now spans multiple physical partitions.
    ///
    /// This does not imply that child lease checkpoints have been derived.
    pub fn spans_multiple_partitions(&self) -> bool {
        self.physical_ranges.len() > 1
    }
}

fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

/// Maps existing durable logical leases onto a refreshed physical topology.
///
/// Every lease keeps its range, ownership state, revision, and checkpoint.
/// Splits therefore preserve recoverable progress without increasing lease-level
/// parallelism. After a merge, separate logical leases remain separate even when
/// they overlap the same physical partition.
///
/// This foundation validates supplied lease coverage but does not bootstrap
/// missing assignments or persist handoffs. It never parses or edits tokens.
///
/// # Errors
///
/// Rejects duplicate IDs, overlapping leases, invalid/overlapping physical ranges,
/// and any lease not fully covered by the supplied topology. Inputs must be
/// explicit non-empty EPK intervals, not logical partition-key point scopes.
pub fn plan_lease_topology(
    leases: &[OwnedLease],
    physical_ranges: &[FeedRange],
) -> Result<Vec<LeaseTopologyAssignment>> {
    let valid = |range: &FeedRange| {
        !range.is_logical_partition() && range.min_inclusive() < range.max_exclusive()
    };
    if physical_ranges.is_empty() || physical_ranges.iter().any(|range| !valid(range)) {
        return Err(invalid(
            "topology must contain non-empty explicit EPK ranges",
        ));
    }
    let mut topology: Vec<_> = physical_ranges.iter().collect();
    topology.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
    if topology
        .windows(2)
        .any(|pair| pair[0].max_exclusive() > pair[1].min_inclusive())
    {
        return Err(invalid("physical topology contains overlapping ranges"));
    }
    let mut ids = HashSet::new();
    for lease in leases {
        if !ids.insert(lease.id()) || !valid(lease.range()) {
            return Err(invalid(
                "leases require unique IDs and non-empty explicit EPK ranges",
            ));
        }
    }
    let mut ordered: Vec<_> = leases.iter().collect();
    ordered.sort_by(|a, b| a.range().min_inclusive().cmp(b.range().min_inclusive()));
    if ordered
        .windows(2)
        .any(|pair| pair[0].range().max_exclusive() > pair[1].range().min_inclusive())
    {
        return Err(invalid("logical lease assignments overlap"));
    }
    leases
        .iter()
        .map(|lease| {
            let range = lease.range();
            let overlaps: Vec<_> = topology
                .iter()
                .copied()
                .filter(|physical| physical.overlaps(range))
                .collect();
            let mut covered = range.min_inclusive().clone();
            for physical in &overlaps {
                if physical.min_inclusive() > &covered {
                    return Err(invalid(
                        "topology leaves a gap in recoverable lease coverage",
                    ));
                }
                covered = physical.max_exclusive().min(range.max_exclusive()).clone();
            }
            if &covered != range.max_exclusive() {
                return Err(invalid("topology does not cover the entire lease range"));
            }
            Ok(LeaseTopologyAssignment {
                lease: lease.clone(),
                physical_ranges: overlaps.into_iter().cloned().collect(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::plan_lease_topology;
    use crate::OwnedLease;
    use azure_data_cosmos_driver::models::{ContinuationToken, FeedRange};
    use std::num::NonZeroU64;

    fn range(min: &str, max: &str) -> FeedRange {
        FeedRange::new(min.try_into().unwrap(), max.try_into().unwrap()).unwrap()
    }
    fn lease(id: &str, bounds: FeedRange, token: &str) -> OwnedLease {
        OwnedLease::new(
            id,
            "owner",
            NonZeroU64::new(1).unwrap(),
            "revision",
            bounds,
            ContinuationToken::from_string(token.to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn split_keeps_parent_checkpoint_and_logical_authority() {
        let parent = lease("parent", FeedRange::full(), "opaque-parent-position");
        let partitions = vec![range("", "80"), range("80", "FF")];
        let plan = plan_lease_topology(std::slice::from_ref(&parent), &partitions).unwrap();
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].lease().id(), "parent");
        assert_eq!(plan[0].lease().range(), parent.range());
        assert_eq!(plan[0].lease().checkpoint(), parent.checkpoint());
        assert_eq!(plan[0].lease().revision(), parent.revision());
        assert_eq!(plan[0].physical_ranges(), partitions);
        assert!(plan[0].spans_multiple_partitions());
    }

    #[test]
    fn merge_retains_distinct_ranges_and_positions() {
        let leases = vec![
            lease("left", range("", "80"), "left-position"),
            lease("right", range("80", "FF"), "right-position"),
        ];
        let plan = plan_lease_topology(&leases, &[FeedRange::full()]).unwrap();
        assert_eq!(plan.len(), 2);
        for (assignment, original) in plan.iter().zip(&leases) {
            assert_eq!(assignment.lease().id(), original.id());
            assert_eq!(assignment.lease().range(), original.range());
            assert_eq!(assignment.lease().checkpoint(), original.checkpoint());
            assert_eq!(assignment.physical_ranges(), &[FeedRange::full()]);
            assert!(!assignment.spans_multiple_partitions());
        }
    }

    #[test]
    fn gaps_duplicates_and_overlapping_authority_are_rejected() {
        let full = lease("full", FeedRange::full(), "position");
        assert!(plan_lease_topology(
            std::slice::from_ref(&full),
            &[range("", "40"), range("80", "FF")]
        )
        .is_err());
        assert!(plan_lease_topology(std::slice::from_ref(&full), &[range("", "80")]).is_err());
        assert!(plan_lease_topology(&[full.clone(), full.clone()], &[FeedRange::full()]).is_err());
        assert!(plan_lease_topology(
            &[full.clone(), lease("overlap", range("", "80"), "other")],
            &[FeedRange::full()]
        )
        .is_err());
        assert!(plan_lease_topology(&[full], &[range("", "C0"), range("80", "FF")]).is_err());
    }
}
