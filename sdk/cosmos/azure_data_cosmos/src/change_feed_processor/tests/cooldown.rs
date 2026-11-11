// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{containers_with_runtime, lease_records, pin_start};
use crate::{
    models::ChangeFeedItem,
    options::{ChangeFeedMode, MaxItemCountHint},
    ChangeFeedCheckpointPolicy, ChangeFeedProcessorBuilder,
};
use azure_core::http::{Method, Request, StatusCode};
use azure_data_cosmos_driver::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::RequestObserver,
};
use futures::{
    channel::{mpsc, oneshot},
    StreamExt,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

#[derive(Debug, Default)]
struct Reads(AtomicUsize);

impl RequestObserver for Reads {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get
            && request.url().path().ends_with("/docs")
            && request
                .headers()
                .iter()
                .any(|(name, _)| name.as_str() == "a-im")
        {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
}

type Callback = (String, oneshot::Sender<()>);

#[tokio::test]
async fn one_sdk_worker_skips_throttled_lease_and_recovers_only_on_new_demand(
) -> Result<(), Box<dyn Error>> {
    static CALLBACKS: Mutex<Option<mpsc::UnboundedSender<Callback>>> = Mutex::new(None);
    static ERRORS: AtomicUsize = AtomicUsize::new(0);
    for mode in [
        ChangeFeedMode::LatestVersion,
        ChangeFeedMode::AllVersionsAndDeletes,
    ] {
        for policy in [
            ChangeFeedCheckpointPolicy::EveryBatch,
            ChangeFeedCheckpointPolicy::Coalesced {
                max_interval: Duration::from_secs(60),
            },
        ] {
            let rule = Arc::new(
                FaultInjectionRuleBuilder::new(
                    "slow-lease",
                    FaultInjectionResultBuilder::new()
                        .with_custom_response(
                            CustomResponseBuilder::new(StatusCode::TooManyRequests)
                                .with_header("x-ms-retry-after-ms", "1500")
                                .with_body(br#"{"message":"throttled"}"#.to_vec())
                                .build(),
                        )
                        .build(),
                )
                .with_condition(
                    FaultInjectionConditionBuilder::new()
                        .with_operation_type(FaultOperationType::ChangeFeedItem)
                        .build(),
                )
                .with_hit_limit(1)
                .build(),
            );
            rule.disable();
            let reads = Arc::new(Reads::default());
            let (source, leases) = containers_with_runtime(2, Some(reads.clone()), |emulator| {
                emulator.runtime_builder_with_fault_rules(vec![rule.clone()])
            })
            .await?;
            pin_start(mode, &source, &leases, "cooldown", 2).await?;
            let before = lease_records(&leases).await?;
            for index in 0..64 {
                let id = format!("item-{index}");
                source
                    .create_item(id.clone(), &id, json!({"id":id}), None)
                    .await?;
            }
            let (sender, mut callbacks) = mpsc::unbounded();
            *CALLBACKS.lock().unwrap() = Some(sender);
            ERRORS.store(0, Ordering::SeqCst);
            let processor =
                ChangeFeedProcessorBuilder::new(mode, source, leases.clone(), "cooldown", "worker")
                    .with_checkpoint_policy(policy)
                    .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::MIN))
                    .with_feed_poll_delay(Duration::from_millis(5))
                    .handle_error(|_, _| async {
                        ERRORS.fetch_add(1, Ordering::SeqCst);
                    })
                    .handle_change_feed(|context, changes: Vec<ChangeFeedItem<Value>>| async move {
                        assert_eq!(changes.len(), 1);
                        let (release, released) = oneshot::channel();
                        CALLBACKS
                            .lock()
                            .unwrap()
                            .as_ref()
                            .unwrap()
                            .unbounded_send((context.lease_token().to_owned(), release))
                            .unwrap();
                        let _ = released.await;
                        Ok(())
                    })
                    .build()?;
            rule.enable();
            processor.start().await?;
            let (healthy, release) = tokio::time::timeout(Duration::from_secs(1), callbacks.next())
                .await?
                .unwrap();
            assert_eq!(rule.hit_count(), 1);
            let slow = before
                .iter()
                .find(|r| r.get("range").is_some() && r["id"] != healthy)
                .unwrap();
            let slow_id = slow["id"].as_str().unwrap();
            let active = lease_records(&leases).await?;
            assert_eq!(
                active.iter().find(|r| r["id"] == slow_id).unwrap()["continuation"],
                slow["continuation"]
            );
            let count = reads.0.load(Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(1700)).await;
            assert_eq!(
                reads.0.load(Ordering::SeqCst),
                count,
                "cooldown expiry must not prefetch"
            );
            assert!(callbacks.try_recv().is_err());
            release.send(()).unwrap();
            let (recovered, release) =
                tokio::time::timeout(Duration::from_secs(1), callbacks.next())
                    .await?
                    .unwrap();
            assert_eq!(recovered, slow_id);
            let mut stopping = Box::pin(processor.stop());
            assert!(futures::poll!(stopping.as_mut()).is_pending());
            release.send(()).unwrap();
            tokio::time::timeout(Duration::from_secs(2), stopping).await??;
            let after = lease_records(&leases).await?;
            assert_ne!(
                after.iter().find(|r| r["id"] == slow_id).unwrap()["continuation"],
                slow["continuation"]
            );
            assert!(after
                .iter()
                .filter(|r| r.get("range").is_some())
                .all(|r| r.get("owner").is_none()));
            assert_eq!(ERRORS.load(Ordering::SeqCst), 0);
            *CALLBACKS.lock().unwrap() = None;
        }
    }
    Ok(())
}
