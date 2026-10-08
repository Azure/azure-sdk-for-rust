// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    fixture_with_layout, options, wait_for, BootstrapStartPolicy, BootstrapStore, ChangeFeedMode,
    ChangeFeedReadOptions, FeedRange, InitialLease, RawChangeHandler,
};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn running_reconciliation_subdivides_a_saved_logical_parent_using_supported_checkpoints(
) -> Result<(), Box<dyn Error>> {
    timeout(Duration::from_secs(15), async {
        let f = fixture_with_layout(None, Vec::new(), 2).await?;
        let mut reader = f
            .engine
            .open_reader(ChangeFeedReadOptions::new(
                FeedRange::full(),
                BootstrapStartPolicy::Beginning,
            ))
            .await?;
        reader.read_page().await?;
        let checkpoint = reader.to_continuation_token()?;
        let plan = f
            .engine
            .bootstrap_plan(
                "group",
                ChangeFeedMode::LatestVersion,
                BootstrapStartPolicy::Beginning,
                FeedRange::full(),
                vec![InitialLease::new(FeedRange::full(), checkpoint)?],
            )
            .await?;
        let opts = options().with_coverage_interval(Duration::from_millis(20));
        let workload = BootstrapStore::from_resolved_single_writer(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            plan,
            opts.ownership_policy().clone(),
        )?;
        let initial = workload
            .ensure_initialized("initial", Duration::from_secs(10))
            .await?;
        assert_eq!(initial.lease_ids().len(), 1);
        let original = initial.lease_ids()[0].clone();
        let handler: RawChangeHandler = Arc::new(|_| Box::pin(async { Ok(()) }));
        let mut running = f
            .engine
            .start_managed(f.lease_driver, f.lease_container, "group", opts, handler)
            .await?;
        let ready = loop {
            let ready = workload
                .ensure_initialized("verifier", Duration::from_secs(10))
                .await?;
            if ready.lease_ids().len() == 2 {
                break ready;
            }

            if running.snapshot().last_error().is_some() {
                panic!("periodic handoff failed");
            }
            sleep(Duration::from_millis(10)).await;
        };
        assert!(!ready.lease_ids().contains(&original));
        assert_eq!(
            ready.plan().initial_leases().len(),
            1,
            "winning seeds remain recovery evidence"
        );
        let report = running.stop().await;
        assert!(running.is_complete());
        assert!(report.errors().is_empty());
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}

#[tokio::test]
async fn splitting_one_owned_lease_keeps_unrelated_lease_owner_and_epoch(
) -> Result<(), Box<dyn Error>> {
    let f = fixture_with_layout(None, Vec::new(), 2).await?;
    let opts = options().with_coverage_interval(Duration::from_millis(20));
    let mut running = f
        .engine
        .start_managed(
            f.lease_driver.clone(),
            f.lease_container.clone(),
            "group",
            opts.clone(),
            Arc::new(|_| Box::pin(async { Ok(()) })),
        )
        .await?;
    wait_for(&running, |p| {
        p.snapshot().leases().len() == 2
            && p.snapshot()
                .leases()
                .iter()
                .all(|lease| lease.last_feed().is_some())
    })
    .await;
    let workload = BootstrapStore::load_existing_single_writer(
        f.lease_driver,
        f.lease_container,
        &f.engine.source_identity(),
        "group",
        opts.mode(),
        opts.start_policy(),
        opts.ownership_policy().clone(),
    )
    .await?
    .unwrap();
    let initial = workload
        .ensure_initialized("verifier", Duration::from_secs(10))
        .await?;
    let mut before = std::collections::BTreeMap::new();
    for id in initial.lease_ids() {
        let observation = workload.work_lease_store(&initial, id)?.observe().await?;
        before.insert(
            id.clone(),
            (
                observation.owner().map(str::to_owned),
                observation.generation(),
            ),
        );
    }
    f.emulator
        .store()
        .split_partition("db", "source", 0, Duration::ZERO);
    f.emulator.store().wait_for_split("db", "source", 0).await;
    let ready = timeout(Duration::from_secs(10), async {
        loop {
            let ready = workload
                .ensure_initialized("verifier", Duration::from_secs(10))
                .await?;
            if ready.lease_ids().len() == 3 {
                break Ok::<_, Box<dyn Error>>(ready);
            }
            tokio::task::yield_now().await;
        }
    })
    .await??;
    let retained: Vec<_> = ready
        .lease_ids()
        .iter()
        .filter(|id| before.contains_key(*id))
        .collect();
    assert_eq!(retained.len(), 1);
    let id = retained[0];
    let observation = workload.work_lease_store(&ready, id)?.observe().await?;
    assert_eq!(
        (
            observation.owner().map(str::to_owned),
            observation.generation()
        ),
        before[id]
    );
    assert!(running.snapshot().last_error().is_none());
    let report = running.stop().await;
    assert!(running.is_complete());
    assert!(report.errors().is_empty());
    assert!(report
        .leases()
        .iter()
        .all(|lease| matches!(lease.release(), crate::LeaseReleaseOutcome::Released)));
    Ok(())
}
