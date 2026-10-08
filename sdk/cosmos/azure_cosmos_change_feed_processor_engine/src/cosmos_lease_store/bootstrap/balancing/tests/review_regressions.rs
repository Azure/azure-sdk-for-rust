// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{fixture_with_observer, policy, BalanceCycle, LeaseBalancer};
use azure_core::http::{headers::HeaderName, Request};
use azure_data_cosmos_driver::{
    fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRule,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::RequestObserver,
    models::{CosmosOperation, ItemReference},
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
use tokio::{
    sync::{oneshot, Notify},
    time::{sleep, Instant},
};

#[derive(Debug)]
struct SlowLaterPages {
    active: AtomicBool,
    pages: AtomicU32,
    first_page: Notify,
    delay: Arc<FaultInjectionRule>,
}
impl RequestObserver for SlowLaterPages {
    fn on_request(&self, request: &Request) {
        if self.active.load(Ordering::SeqCst)
            && request
                .headers()
                .get_optional_str(&HeaderName::from_static("x-ms-documentdb-isquery"))
                == Some("True")
            && self.pages.fetch_add(1, Ordering::SeqCst) == 0
        {
            self.delay.enable();
            self.first_page.notify_one();
        }
    }
}

#[tokio::test]
async fn slow_paginated_inventory_cannot_revoke_independently_renewed_local_sessions(
) -> Result<(), Box<dyn Error>> {
    let delay = Arc::new(
        FaultInjectionRuleBuilder::new(
            "slow-later-inventory-pages",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_delay(Duration::from_millis(110))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::QueryItem)
                .build(),
        )
        .build(),
    );
    delay.disable();
    let observer = Arc::new(SlowLaterPages {
        active: AtomicBool::new(false),
        pages: AtomicU32::new(0),
        first_page: Notify::new(),
        delay: delay.clone(),
    });
    let f = fixture_with_observer(vec![delay.clone()], 0, Some(observer.clone())).await?;
    f.a.ensure_initialized("initializer", Duration::from_secs(10))
        .await?;
    let mut balancer =
        LeaseBalancer::new(f.a.clone(), "A")?.with_page_size(NonZeroU32::new(1).unwrap());
    let mut sessions = Vec::new();
    for _ in 0..2 {
        match balancer.cycle().await? {
            BalanceCycle::Acquired { session, .. } => sessions.push(session),
            _ => panic!("both available logical leases must be acquired"),
        }
    }
    let first = sessions[0].lease().await;
    let mut retired = f.a.discover().await?.remove(0);
    retired.ownership.owner = None;
    retired.extra.remove("_etag");
    retired.extra.insert("leaseState".into(), json!("Retired"));
    for index in 0..20 {
        retired.id = format!("zz-retired-{index:02}");
        f.a.store
            .execute(
                CosmosOperation::create_item(ItemReference::from_name(
                    &f.a.store.container,
                    f.a.store.partition_key.clone(),
                    retired.id.clone(),
                ))
                .with_body(serde_json::to_vec(&retired)?),
            )
            .await?;
    }
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    let (finished, finish) = oneshot::channel();
    observer.active.store(true, Ordering::SeqCst);
    let began = Instant::now();
    let (cycle, renewal) = tokio::join!(
        async {
            let result = balancer.cycle().await;
            finished.send(()).unwrap();
            result
        },
        async {
            observer.first_page.notified().await;
            tokio::pin!(finish);
            loop {
                tokio::select! {
                    biased;
                    _ = &mut finish => break,
                    _ = sleep(policy().renew_interval()) => {},
                }
                for session in &sessions {
                    session.renew().await?;
                }
            }
            Ok::<_, Box<dyn Error>>(())
        },
    );
    renewal?;
    assert!(matches!(
        cycle?,
        BalanceCycle::NoAction | BalanceCycle::Conflict(_)
    ));
    assert!(began.elapsed() > policy().duration());
    assert!(observer.pages.load(Ordering::SeqCst) >= 22);
    assert_ne!(sessions[0].lease().await.revision(), first.revision());
    assert!(sessions.iter().all(|session| !session.control().is_lost()));
    assert_eq!(balancer.sessions.len(), 2);
    delay.disable();
    for session in sessions {
        session.release().await?;
    }
    Ok(())
}

#[tokio::test]
async fn inventory_replacement_still_revokes_the_obsolete_local_session(
) -> Result<(), Box<dyn Error>> {
    let f = super::fixture(vec![], 0).await?;
    let ready =
        f.a.ensure_initialized("initializer", Duration::from_secs(10))
            .await?;
    let mut balancer = LeaseBalancer::new(f.a, "A")?;
    let mut local = Vec::new();
    for _ in 0..2 {
        match balancer.cycle().await? {
            BalanceCycle::Acquired { session, .. } => local.push(session),
            _ => panic!("available leases should be acquired"),
        }
    }
    let old = local[0].lease().await;
    let store = f.b.work_lease_store(&ready, old.id())?;
    let replacement = store.transfer(&store.observe().await?, "B").await?;
    assert!(matches!(balancer.cycle().await?, BalanceCycle::NoAction));
    assert!(local[0].control().is_lost());
    assert!(!local[1].control().is_lost());
    assert_eq!(balancer.sessions.len(), 1);
    assert_eq!(
        replacement.lease().await.epoch().get(),
        old.epoch().get() + 1
    );
    replacement.release().await?;
    local[1].release().await?;
    Ok(())
}
