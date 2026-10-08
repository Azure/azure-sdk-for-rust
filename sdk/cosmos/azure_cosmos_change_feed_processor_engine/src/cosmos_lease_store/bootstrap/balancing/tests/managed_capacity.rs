// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{fixture, BalanceCycle, LeaseBalancer};
use std::{error::Error, num::NonZeroU32, time::Duration};

#[tokio::test]
async fn host_capacity_stops_acquisitions_without_changing_global_lease_counts(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut balancer =
        LeaseBalancer::new(f.a, "A")?.with_max_owned_leases(NonZeroU32::new(1).unwrap());
    let acquired = match balancer.cycle().await? {
        BalanceCycle::Acquired { session, .. } => session,
        _ => panic!("first available lease should acquire"),
    };
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    assert_eq!(
        f.b.discover()
            .await?
            .iter()
            .filter(|record| record.ownership.owner.is_some())
            .count(),
        1
    );
    acquired.release().await?;
    Ok(())
}

#[tokio::test]
async fn a_deferred_poison_lease_does_not_prevent_a_different_lease_from_acquiring(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready =
        f.a.ensure_initialized("initializer", Duration::from_secs(10))
            .await?;
    let mut balancer = LeaseBalancer::new(f.a, "A")?.with_tie_break(0);
    balancer.exclude_lease(ready.lease_ids()[0].clone())?;
    let mut acquired = None;
    for _ in 0..3 {
        if let BalanceCycle::Acquired { session, .. } = balancer.cycle().await? {
            acquired = Some(session);
            break;
        }
    }
    let acquired = acquired.expect("rotation should select the other eligible lease");
    assert_ne!(acquired.lease().await.id(), ready.lease_ids()[0]);
    acquired.release().await?;
    balancer.include_lease(&ready.lease_ids()[0]);
    Ok(())
}
