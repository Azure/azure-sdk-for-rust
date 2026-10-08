// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{child_id, BootstrapStore, InitialLease};
use crate::cosmos_lease_store::bootstrap::tests::{error_rule, fixture_with_observer, policy};
use azure_core::http::{headers::HeaderName, Request};
use azure_data_cosmos_driver::{
    fault_injection::{FaultInjectionRule, FaultOperationType},
    in_memory_emulator::RequestObserver,
    models::FeedRange,
    options::OperationOptions,
};
use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

#[derive(Debug)]
struct InterruptActivation {
    armed: AtomicBool,
    failure: Arc<FaultInjectionRule>,
}
impl RequestObserver for InterruptActivation {
    fn on_request(&self, request: &Request) {
        if request
            .headers()
            .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request"))
            == Some("True")
            && self.armed.swap(false, Ordering::SeqCst)
        {
            self.failure.enable();
        }
    }
}

#[tokio::test]
async fn interrupted_handoff_keeps_children_pending_then_recovers_the_saved_positions(
) -> Result<(), Box<dyn Error>> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let failure = error_rule("activation-interrupted", FaultOperationType::BatchItem);
        let observer = Arc::new(InterruptActivation {
            armed: AtomicBool::new(false),
            failure: failure.clone(),
        });
        let f = fixture_with_observer(vec![failure.clone()], 0, Some(observer.clone())).await?;
        let ready =
            f.a.ensure_initialized("initializer", Duration::from_secs(10))
                .await?;
        let parent_id = ready.lease_ids()[0].clone();
        let parent =
            f.a.topology_candidates()
                .await?
                .into_iter()
                .find(|lease| lease.id == parent_id)
                .unwrap();
        let children = vec![
            FeedRange::new(
                parent.range.min_inclusive().clone(),
                "40".try_into().unwrap(),
            )?,
            FeedRange::new(
                "40".try_into().unwrap(),
                parent.range.max_exclusive().clone(),
            )?,
        ];
        let source = f
            .driver
            .resolve_container("db", "source", OperationOptions::default())
            .await?;
        let derived = f.driver.derive_change_feed_checkpoints(
            &source,
            &parent.checkpoint,
            &parent.range,
            &children,
            false,
        )?;
        let seeds: Vec<_> = children
            .into_iter()
            .zip(derived)
            .map(|(range, token)| InitialLease::new(range, token))
            .collect::<Result<_, _>>()?;
        observer.armed.store(true, Ordering::SeqCst);
        assert!(f
            .a
            .begin_handoff(&parent_id, &parent.checkpoint, seeds.clone())
            .await
            .is_err());
        let records = f.a.discover().await?;
        assert_eq!(
            records
                .iter()
                .find(|record| record.id == parent_id)
                .unwrap()
                .extra["leaseState"],
            "Transitioning"
        );
        for seed in &seeds {
            let child = records
                .iter()
                .find(|record| record.id == child_id(seed))
                .unwrap();
            assert_eq!(child.extra["leaseState"], "Pending");
            assert_eq!(child.checkpoint, seed.checkpoint());
            let mut store = f.a.store.clone();
            store.id = child.id.clone();
            store.item = azure_data_cosmos_driver::models::ItemReference::from_name(
                &store.container,
                store.partition_key.clone(),
                store.id.clone(),
            );
            store.readiness = Some(super::super::ReadinessGate {
                bootstrap: Arc::new(f.a.store.clone()),
                plan: ready.plan().clone(),
            });
            assert!(store.try_acquire("premature-worker").await.is_err());
        }
        failure.disable();
        let recovered = BootstrapStore::load_existing_single_writer(
            f.driver,
            f.a.store.container.clone(),
            ready.plan().source(),
            ready.plan().group(),
            ready.plan().mode(),
            ready.plan().start_policy(),
            policy(),
        )
        .await?
        .unwrap();
        let final_ready = recovered
            .ensure_initialized("recovery-worker", Duration::from_secs(10))
            .await?;
        assert_eq!(final_ready.lease_ids().len(), 3);
        assert!(!final_ready.lease_ids().contains(&parent_id));
        for seed in &seeds {
            let store = recovered.work_lease_store(&final_ready, &child_id(seed))?;
            assert_eq!(store.observe().await?.checkpoint(), seed.checkpoint());
            let session = store.try_acquire("processing-worker").await?.unwrap();
            session.release().await?;
        }
        let retired = recovered
            .discover()
            .await?
            .into_iter()
            .find(|record| record.id == parent_id)
            .unwrap();
        assert_eq!(retired.extra["leaseState"], "Retired");
        assert!(retired.extra.contains_key("handoff"));
        assert_eq!(retired.checkpoint, parent.checkpoint.as_str());
        Ok::<_, Box<dyn Error>>(())
    })
    .await??;
    Ok(())
}
