// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{plan_equal_lease_balance, BalanceActionKind, BalanceLease};
use std::collections::BTreeMap;

fn owned(id: u32, owner: &str) -> BalanceLease {
    BalanceLease::new(format!("lease-{id:02}"))
        .unwrap()
        .with_owner(owner)
}

#[test]
fn zero_one_worker_and_rotated_unused_candidates() {
    assert!(plan_equal_lease_balance(&[], "A", 0).unwrap().is_none());
    let leases: Vec<_> = (0..3)
        .map(|id| BalanceLease::new(format!("lease-{id}")).unwrap())
        .collect();
    for rotation in 0..3 {
        let action = plan_equal_lease_balance(&leases, "A", rotation)
            .unwrap()
            .unwrap();
        assert_eq!(action.lease_id(), format!("lease-{rotation}"));
        assert_eq!(action.kind(), BalanceActionKind::Pickup);
    }
    let all: Vec<_> = (0..3).map(|id| owned(id, "A")).collect();
    assert!(plan_equal_lease_balance(&all, "A", 0).unwrap().is_none());
}

#[test]
fn balanced_uneven_counts_and_more_workers_than_leases_do_not_steal() {
    let distribution: Vec<_> = (0..10)
        .map(|id| {
            owned(
                id,
                if id < 4 {
                    "A"
                } else if id < 7 {
                    "B"
                } else {
                    "C"
                },
            )
        })
        .collect();
    for worker in ["A", "B", "C"] {
        assert!(plan_equal_lease_balance(&distribution, worker, 0)
            .unwrap()
            .is_none());
    }
    let few = vec![owned(0, "A"), owned(1, "B")];
    assert!(plan_equal_lease_balance(&few, "C", 0).unwrap().is_none());
}

#[test]
fn new_zero_lease_worker_converges_by_one_transfer_per_cycle() {
    let mut leases: Vec<_> = (0..10)
        .map(|id| owned(id, if id < 5 { "A" } else { "B" }))
        .collect();
    let mut moves = 0;
    for cycle in 0..20 {
        let Some(action) = plan_equal_lease_balance(&leases, "C", cycle).unwrap() else {
            break;
        };
        assert_eq!(action.kind(), BalanceActionKind::Transfer);
        let lease = leases
            .iter_mut()
            .find(|lease| lease.id() == action.lease_id())
            .unwrap();
        *lease = BalanceLease::new(lease.id().to_owned())
            .unwrap()
            .with_owner("C");
        moves += 1;
    }
    assert_eq!(moves, 3);
    let mut counts = BTreeMap::new();
    for lease in &leases {
        *counts.entry(lease.owner().unwrap()).or_insert(0) += 1;
    }
    let mut values: Vec<_> = counts.values().copied().collect();
    values.sort_unstable();
    assert_eq!(values, vec![3, 3, 4]);
    for worker in ["A", "B", "C"] {
        assert!(plan_equal_lease_balance(&leases, worker, 0)
            .unwrap()
            .is_none());
    }
}

#[test]
fn expired_and_unowned_leases_are_preferred_and_ineligible_work_is_excluded() {
    let snapshot = vec![
        owned(0, "A"),
        owned(1, "A"),
        owned(2, "dead").with_expired(true),
        owned(3, "other").with_eligible(false),
    ];
    let action = plan_equal_lease_balance(&snapshot, "B", 0)
        .unwrap()
        .unwrap();
    assert_eq!(action.lease_id(), "lease-02");
    assert_eq!(action.kind(), BalanceActionKind::Pickup);
    let excluded = vec![owned(0, "A").with_eligible(false)];
    assert!(plan_equal_lease_balance(&excluded, "B", 0)
        .unwrap()
        .is_none());
}

#[test]
fn invalid_membership_or_duplicate_inventory_fails_instead_of_balancing() {
    assert!(plan_equal_lease_balance(&[], " ", 0).is_err());
    assert!(plan_equal_lease_balance(&[owned(0, "A"), owned(0, "B")], "C", 0).is_err());
    assert!(plan_equal_lease_balance(&[owned(0, "")], "C", 0).is_err());
}

#[test]
fn stable_multi_worker_policy_simulation_converges_with_bounded_movement() {
    let mut leases: Vec<_> = (0..10)
        .map(|id| BalanceLease::new(format!("lease-{id:02}")).unwrap())
        .collect();
    let mut moves = 0;
    for cycle in 0..30 {
        let mut changed = false;
        for worker in ["A", "B", "C"] {
            if let Some(action) = plan_equal_lease_balance(&leases, worker, cycle).unwrap() {
                let lease = leases
                    .iter_mut()
                    .find(|lease| lease.id() == action.lease_id())
                    .unwrap();
                *lease = BalanceLease::new(lease.id().to_owned())
                    .unwrap()
                    .with_owner(worker);
                moves += 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let mut counts = BTreeMap::new();
    for lease in &leases {
        *counts.entry(lease.owner().unwrap()).or_insert(0) += 1;
    }
    let mut counts: Vec<_> = counts.values().copied().collect();
    counts.sort_unstable();
    assert_eq!(counts, vec![3, 3, 4]);
    assert!(
        moves <= 20,
        "each round permits at most one move per worker, not unbounded reassignment"
    );
    for rotation in 0..5 {
        for worker in ["A", "B", "C"] {
            assert!(plan_equal_lease_balance(&leases, worker, rotation)
                .unwrap()
                .is_none());
        }
    }
}
