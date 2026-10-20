// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{Lease, ProcessorOptions};
use rand::seq::SliceRandom;
use std::collections::BTreeMap;

pub(super) fn select(leases: &[Lease], options: &ProcessorOptions, now: i64) -> Vec<usize> {
    let mut owners = BTreeMap::new();
    let mut available = Vec::new();
    for (index, lease) in leases.iter().enumerate() {
        match (&lease.base.owner, lease.renewed_at) {
            (Some(owner), Some(updated))
                if !owner.is_empty()
                    && now.saturating_sub(updated) <= options.expiration.as_millis() as i64 =>
            {
                *owners.entry(owner.as_str()).or_insert(0usize) += 1;
            }
            _ => available.push(index),
        }
    }
    let mine = *owners
        .entry(options.bootstrap.instance_name.as_str())
        .or_insert(0);
    if leases.is_empty() {
        return Vec::new();
    }
    let mut target = leases.len().div_ceil(owners.len()).max(1);
    if options.max_leases != 0 {
        target = target.min(options.max_leases as usize);
    }
    target = target.max(options.min_leases as usize);
    let needed = target.saturating_sub(mine);
    if !available.is_empty() {
        let mut take = if options.max_leases == 0 {
            needed.max(1)
        } else {
            needed
        };
        if owners.len() > 1 {
            take = take.min(1);
        }
        available.shuffle(&mut rand::rng());
        available.truncate(take);
        return available;
    }
    if needed == 0 {
        return Vec::new();
    }
    let Some((donor, count)) = owners.iter().max_by_key(|(_, count)| **count) else {
        return Vec::new();
    };
    if *count > target - usize::from(needed > 1) {
        return leases
            .iter()
            .position(|lease| lease.base.owner.as_deref() == Some(*donor))
            .into_iter()
            .collect();
    }
    Vec::new()
}
