// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{BalanceCycle, BalanceRunOptions, LeaseBalancer};
use crate::cosmos_lease_store::bootstrap::tests::{
    error_rule, fixture, fixture_with_observer, policy,
};
use crate::{plan_equal_lease_balance, BalanceActionKind, BalanceLease, LeaseControl};
use azure_core::http::{headers::HeaderName, Request, StatusCode};
use azure_data_cosmos_driver::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRule,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::RequestObserver,
    models::{CosmosOperation, ItemReference, Precondition},
};
use serde_json::json;
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::sleep;

#[derive(Debug)]
struct FailAfterFirstPage {
    active: AtomicBool,
    pages: AtomicU32,
    rule: Arc<FaultInjectionRule>,
}
impl RequestObserver for FailAfterFirstPage {
    fn on_request(&self, request: &Request) {
        if self.active.load(Ordering::SeqCst)
            && request
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-documentdb-isquery"))
                == Some("True")
            && self.pages.fetch_add(1, Ordering::SeqCst) == 0
        {
            self.rule.enable();
        }
    }
}

#[tokio::test]
async fn independent_policy_contenders_select_same_lease_but_only_one_store_cas_wins(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready =
        f.a.ensure_initialized("initializer", Duration::from_secs(10))
            .await?;
    let snapshot: Vec<_> = ready
        .lease_ids()
        .iter()
        .map(|id| BalanceLease::new(id.clone()).unwrap())
        .collect();
    let a = plan_equal_lease_balance(&snapshot, "A", 0)?.unwrap();
    let b = plan_equal_lease_balance(&snapshot, "B", 0)?.unwrap();
    assert_eq!(a.lease_id(), b.lease_id());
    let store_a = f.a.work_lease_store(&ready, a.lease_id())?;
    let store_b = f.b.work_lease_store(&ready, b.lease_id())?;
    let observed_a = store_a.observe().await?;
    let observed_b = store_b.observe().await?;
    let (a, b) = tokio::join!(
        store_a.acquire(&observed_a, "A"),
        store_b.acquire(&observed_b, "B")
    );
    let winner = match (a, b) {
        (Ok(Some(winner)), Err(error)) | (Err(error), Ok(Some(winner))) => {
            assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
            winner
        }
        _ => panic!("exactly one conditional acquisition must succeed"),
    };
    winner.release().await?;
    Ok(())
}

#[tokio::test]
async fn active_transfer_preserves_progress_and_fences_old_checkpoint_and_release(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready =
        f.a.ensure_initialized("initializer", Duration::from_secs(10))
            .await?;
    let mut old = Vec::new();
    for id in ready.lease_ids() {
        old.push(
            f.a.work_lease_store(&ready, id)?
                .try_acquire("A")
                .await?
                .unwrap(),
        );
    }
    let mut balancer = LeaseBalancer::new(f.b.clone(), "B")?
        .with_tie_break(0)
        .with_page_size(NonZeroU32::new(1).unwrap());
    let (action, next) = match balancer.cycle().await? {
        BalanceCycle::Acquired { action, session } => (action, session),
        _ => panic!("worker below target should transfer from the two-lease donor"),
    };
    assert_eq!(action.kind(), BalanceActionKind::Transfer);
    let previous = old[0].lease().await;
    assert_eq!(next.lease().await.checkpoint(), previous.checkpoint());
    assert_eq!(next.lease().await.epoch().get(), previous.epoch().get() + 1);
    assert!(old[0]
        .checkpoint(&previous, previous.checkpoint())
        .await
        .is_err());
    assert!(old[0].release().await.is_err());
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    next.release().await?;
    old[1].release().await?;
    Ok(())
}

#[tokio::test]
async fn expired_owner_recovery_uses_observation_time_and_preserves_checkpoint(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    let ready =
        f.a.ensure_initialized("initializer", Duration::from_secs(10))
            .await?;
    let records = f.a.discover().await?;
    let mut retired = records[1].clone();
    retired.extra.insert("leaseState".into(), json!("Retired"));
    f.a.store
        .execute(
            CosmosOperation::replace_item(ItemReference::from_name(
                &f.a.store.container,
                f.a.store.partition_key.clone(),
                retired.id.clone(),
            ))
            .with_precondition(Precondition::if_match(
                retired.extra["_etag"].as_str().unwrap().to_owned(),
            ))
            .with_body(serde_json::to_vec(&retired)?),
        )
        .await?;
    let old =
        f.a.work_lease_store(&ready, &records[0].id)?
            .try_acquire("A")
            .await?
            .unwrap();
    let previous = old.lease().await;
    let mut balancer = LeaseBalancer::new(f.b, "B")?.with_tie_break(0);
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    sleep(policy().duration() + Duration::from_millis(20)).await;
    let next = match balancer.cycle().await? {
        BalanceCycle::Acquired { action, session } => {
            assert_eq!(action.kind(), BalanceActionKind::Pickup);
            session
        }
        _ => panic!("expired owner should be recovered"),
    };
    assert_eq!(next.lease().await.checkpoint(), previous.checkpoint());
    assert!(old.release().await.is_err());
    next.release().await?;
    Ok(())
}

#[tokio::test]
async fn paginated_inventory_failure_does_not_grant_ownership_or_commit_partial_history(
) -> Result<(), Box<dyn Error>> {
    let rule = error_rule(
        "second-inventory-page-failed",
        FaultOperationType::QueryItem,
    );
    let observer = Arc::new(FailAfterFirstPage {
        active: AtomicBool::new(false),
        pages: AtomicU32::new(0),
        rule: rule.clone(),
    });
    let f = fixture_with_observer(vec![rule.clone()], 0, Some(observer.clone())).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut balancer = LeaseBalancer::new(f.a, "A")?.with_page_size(NonZeroU32::new(1).unwrap());
    observer.active.store(true, Ordering::SeqCst);
    assert!(balancer.cycle().await.is_err());
    assert_eq!(observer.pages.load(Ordering::SeqCst), 1);
    assert!(rule.hit_count() > 0);
    assert!(balancer.sessions.is_empty());
    assert!(balancer.seen.is_empty());
    assert!(f
        .b
        .discover()
        .await?
        .iter()
        .all(|lease| lease.ownership.owner.is_none()));
    observer.active.store(false, Ordering::SeqCst);
    rule.disable();
    let next = match balancer.cycle().await? {
        BalanceCycle::Acquired { session, .. } => session,
        _ => panic!("fresh complete inventory should allow acquisition"),
    };
    next.release().await?;
    Ok(())
}

#[tokio::test]
async fn ineligible_records_and_other_workloads_do_not_count_as_pickup_candidates(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let records = f.a.discover().await?;
    for (record, state) in records.iter().zip(["Pending", "Retired"]) {
        let mut record = record.clone();
        record.extra.insert("leaseState".into(), json!(state));
        f.a.store
            .execute(
                CosmosOperation::replace_item(ItemReference::from_name(
                    &f.a.store.container,
                    f.a.store.partition_key.clone(),
                    record.id.clone(),
                ))
                .with_precondition(Precondition::if_match(
                    record.extra["_etag"].as_str().unwrap().to_owned(),
                ))
                .with_body(serde_json::to_vec(&record)?),
            )
            .await?;
    }
    f.a.store
        .execute(
            CosmosOperation::create_item(ItemReference::from_name(
                &f.a.store.container,
                f.a.store.partition_key.clone(),
                "control",
            ))
            .with_body(serde_json::to_vec(&json!({
                "id":"control","workload":f.a.store.id,"recordKind":"Control"
            }))?),
        )
        .await?;
    let mut other_plan = f.a.plan.clone();
    other_plan.group = "different-workload".into();
    let other = super::super::super::BootstrapStore::new_single_writer(
        f.driver,
        "db",
        "leases",
        other_plan,
        policy(),
    )
    .await?;
    other
        .ensure_initialized("other-initializer", Duration::from_secs(10))
        .await?;
    let mut balancer = LeaseBalancer::new(f.b, "B")?;
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    Ok(())
}

#[tokio::test]
async fn acquisition_timeout_never_hands_off_a_session_and_next_cycle_reads_fresh_state(
) -> Result<(), Box<dyn Error>> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "delayed-pickup",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_delay(Duration::from_secs(1))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::BatchItem)
                .build(),
        )
        .build(),
    );
    rule.disable();
    let f = fixture(vec![rule.clone()], u32::MAX).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut balancer = LeaseBalancer::new(f.a, "A")?;
    rule.enable();
    assert!(balancer.cycle().await.is_err());
    assert!(balancer.sessions.is_empty());
    rule.disable();
    let next = match balancer.cycle().await? {
        BalanceCycle::Acquired { session, .. } => session,
        _ => panic!("fresh state should be reevaluated after uncertainty"),
    };
    next.release().await?;
    Ok(())
}

#[tokio::test]
async fn bounded_cycles_handoff_only_confirmed_sessions_and_external_stop_admits_no_work(
) -> Result<(), Box<dyn Error>> {
    let f = fixture(vec![], 0).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut balancer = LeaseBalancer::new(f.a, "A")?;
    let options = BalanceRunOptions::new(NonZeroU32::new(1).unwrap())
        .with_interval(Duration::from_millis(1))
        .with_jitter(Duration::ZERO);
    let report = balancer
        .run_cycles(&options, &LeaseControl::default(), |session| async move {
            assert_eq!(session.lease().await.owner(), "A");
            session.release().await
        })
        .await?;
    assert_eq!(report.cycles(), 1);
    assert_eq!(report.acquisitions(), 1);
    let stopped = LeaseControl::default();
    stopped.stop();
    let report = balancer
        .run_cycles(&options, &stopped, |_| async {
            panic!("no handoff after stop")
        })
        .await?;
    assert!(report.stopped());
    assert_eq!(report.cycles(), 0);
    Ok(())
}

#[tokio::test]
async fn cycle_conflict_is_reported_then_fresh_inventory_selects_other_available_work(
) -> Result<(), Box<dyn Error>> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "pause-first-acquisition",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_delay(Duration::from_millis(200))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::BatchItem)
                .build(),
        )
        .build(),
    );
    rule.disable();
    let f = fixture(vec![rule.clone()], u32::MAX).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut a = LeaseBalancer::new(f.a, "A")?.with_tie_break(0);
    let mut b = LeaseBalancer::new(f.b, "B")?.with_tie_break(0);
    rule.enable();
    let waiting = tokio::spawn(async move {
        let result = a.cycle().await;
        (a, result)
    });
    tokio::time::timeout(Duration::from_secs(2), async {
        while rule.hit_count() == 0 {
            sleep(Duration::from_millis(1)).await;
        }
    })
    .await?;
    let winner = match b.cycle().await? {
        BalanceCycle::Acquired { session, .. } => session,
        _ => panic!("independent contender should acquire while the first CAS is delayed"),
    };
    let (mut a, result) = waiting.await?;
    assert!(matches!(result?, BalanceCycle::Conflict(_)));
    assert!(a.sessions.is_empty());
    rule.disable();
    let other = match a.cycle().await? {
        BalanceCycle::Acquired { session, .. } => session,
        _ => panic!("conflict recovery must replan from fresh inventory"),
    };
    assert_ne!(other.lease().await.id(), winner.lease().await.id());
    other.release().await?;
    winner.release().await?;
    Ok(())
}
