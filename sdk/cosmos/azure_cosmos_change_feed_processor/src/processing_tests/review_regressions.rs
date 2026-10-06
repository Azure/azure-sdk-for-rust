// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{launch, options, pages, LeaseRunOutcome, OwnedLease, Write};
use std::{error::Error, sync::atomic::Ordering, time::Duration};

#[tokio::test]
async fn final_idle_checkpoint_completes_without_waiting_for_another_poll(
) -> Result<(), Box<dyn Error>> {
    let (lease, mut pages) = pages().await?;
    let idle = pages.pop().unwrap();
    let checkpoint = idle.continuation().clone();
    let harness = launch(
        lease,
        vec![idle],
        vec![Write::Confirm],
        vec![],
        options(1)
            .with_idle_delay(Duration::from_secs(10))
            .with_max_duration(Duration::from_secs(1)),
    );
    let report = harness.task.await?.unwrap();
    assert_eq!(report.outcome(), LeaseRunOutcome::BatchLimitReached);
    assert_eq!(report.confirmed_batches(), 1);
    assert_eq!(report.lease().checkpoint(), &checkpoint);
    assert_eq!(harness.reads.load(Ordering::SeqCst), 1);
    assert_eq!(harness.writes.load(Ordering::SeqCst), 1);
    assert_eq!(harness.handled.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn an_idle_checkpoint_still_delays_an_additional_read() -> Result<(), Box<dyn Error>> {
    let (lease, mut pages) = pages().await?;
    let idle = pages.pop().unwrap();
    let checkpoint = idle.continuation().clone();
    let harness = launch(
        lease,
        vec![idle],
        vec![Write::Confirm],
        vec![],
        options(2)
            .with_idle_delay(Duration::from_secs(10))
            .with_max_duration(Duration::from_millis(50)),
    );
    let report = harness.task.await?.unwrap();
    assert_eq!(report.outcome(), LeaseRunOutcome::RunTimedOut);
    assert_eq!(report.confirmed_batches(), 1);
    assert_eq!(report.lease().checkpoint(), &checkpoint);
    assert_eq!(harness.reads.load(Ordering::SeqCst), 1);
    assert_eq!(harness.writes.load(Ordering::SeqCst), 1);
    assert_eq!(harness.handled.load(Ordering::SeqCst), 0);
    Ok(())
}

#[tokio::test]
async fn unchanged_idle_continuation_reuses_durable_progress_without_a_checkpoint_write(
) -> Result<(), Box<dyn Error>> {
    let (lease, mut pages) = pages().await?;
    let idle = pages.pop().unwrap();
    let checkpoint = idle.continuation().clone();
    let lease = OwnedLease::new(
        lease.id(),
        lease.owner(),
        lease.epoch(),
        lease.revision(),
        lease.range().clone(),
        checkpoint.clone(),
    )?;
    let harness = launch(lease, vec![idle], vec![], vec![], options(1));
    let report = harness.task.await?.unwrap();
    assert_eq!(report.outcome(), LeaseRunOutcome::BatchLimitReached);
    assert_eq!(report.lease().checkpoint(), &checkpoint);
    assert_eq!(harness.reads.load(Ordering::SeqCst), 1);
    assert_eq!(harness.writes.load(Ordering::SeqCst), 0);
    assert_eq!(harness.handled.load(Ordering::SeqCst), 0);
    Ok(())
}
