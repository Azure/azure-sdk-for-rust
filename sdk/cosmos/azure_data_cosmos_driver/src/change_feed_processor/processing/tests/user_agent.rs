// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::super::CheckpointPolicy;
use super::{
    bootstrap, fixture_with_runtime, insert, next, poll_until, start, ChangeFeedFactoryMode, Lease,
    RequestObserver, ENDPOINT,
};
use crate::{
    in_memory_emulator::{InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion},
    models::{ChangeFeedStartFrom, CosmosOperation, FeedRange, ResponseBody},
    options::{DriverOptions, OperationOptions, PlanOptions, UserAgentProperty, UserAgentSuffix},
    Result,
};
use azure_core::{
    http::{Method, Request},
    Bytes,
};
use serde_json::Value;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Debug)]
struct Observed {
    method: Method,
    path: String,
    agent: String,
    body: Option<Value>,
    feed: bool,
    wire_format: Option<String>,
}

#[derive(Debug, Default)]
struct Requests(Mutex<Vec<Observed>>);

impl RequestObserver for Requests {
    fn on_request(&self, request: &Request) {
        let bytes = Bytes::from(request.body());
        let body = if bytes.is_empty() {
            None
        } else {
            Some(ResponseBody::from_bytes(bytes).into_single().unwrap())
        };
        let agent = request
            .headers()
            .iter()
            .find(|(name, _)| name.as_str() == "user-agent")
            .expect("user agent")
            .1
            .as_str()
            .to_owned();
        self.0.lock().unwrap().push(Observed {
            method: request.method(),
            path: request.url().path().to_owned(),
            agent,
            body,
            feed: request
                .headers()
                .iter()
                .any(|(name, _)| name.as_str() == "a-im"),
            wire_format: request
                .headers()
                .iter()
                .find(|(name, _)| name.as_str() == "x-ms-cosmos-changefeed-wire-format-version")
                .map(|(_, value)| value.as_str().to_owned()),
        });
    }
}

impl Requests {
    fn take(&self) -> Vec<Observed> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }
}

fn assert_tagged(requests: &[Observed], source_agent: &str, lease_agent: &str) {
    assert!(!requests.is_empty());
    for request in requests {
        assert!(
            request.agent == format!("{source_agent} ChangeFeedProcessor")
                || request.agent == format!("{lease_agent} ChangeFeedProcessor"),
            "{request:?}"
        );
        assert!(request.agent.len() <= 255);
    }
}

#[tokio::test]
async fn processor_wire_format_is_preserved_on_fresh_and_resumed_reads() -> Result<()> {
    for mode in [
        ChangeFeedFactoryMode::Incremental,
        ChangeFeedFactoryMode::FullFidelity,
    ] {
        let requests = Arc::new(Requests::default());
        let (mut context, _) =
            super::fixture_with_observer(mode, 1, Some(requests.clone())).await?;
        Arc::get_mut(&mut context).unwrap().options.start = ChangeFeedStartFrom::Now;
        for (stage, id) in ["initial", "after-restart"].into_iter().enumerate() {
            requests.take();
            if stage == 1 {
                insert(&context, id).await?;
            }
            let processor = start(&context, context.options.clone()).await?;
            if stage == 0 {
                poll_until(
                    &processor,
                    tokio::time::timeout(Duration::from_secs(5), async {
                        while context.list().await.unwrap()[0].base.continuation.is_none() {
                            tokio::task::yield_now().await;
                        }
                    }),
                )
                .await
                .unwrap();
                insert(&context, id).await?;
            }
            let (_, response, receipt) = next(&processor).await.into_parts();
            let body: Value = response.into_body().into_single()?;
            assert_eq!(body["Documents"].as_array().unwrap().len(), 1);
            assert_eq!(body["Documents"][0]["current"]["id"], id);
            receipt.checkpoint().await?;
            processor.stop().await?;
            let feed: Vec<_> = requests
                .take()
                .into_iter()
                .filter(|request| request.feed)
                .collect();
            assert!(!feed.is_empty());
            for request in feed {
                assert_eq!(request.wire_format.as_deref(), Some("2021-09-15"));
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn processor_user_agent_covers_bootstrap_polling_maintenance_and_topology_only() -> Result<()>
{
    for policy in [
        CheckpointPolicy::EveryBatch,
        CheckpointPolicy::Coalesced {
            max_interval: Duration::from_secs(60),
        },
    ] {
        let requests = Arc::new(Requests::default());
        let emulator =
            InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                ENDPOINT.parse()?,
            )])?)
            .with_request_observer(requests.clone());
        let emulator = Arc::new(emulator);
        let mut context = fixture_with_runtime(
            &emulator,
            emulator
                .runtime_builder()
                .with_wrapping_sdk_identifier("azsdk-test-cosmos/1.0")
                .with_user_agent_property(UserAgentProperty::try_new("host", "worker")?)
                .with_user_agent_suffix(UserAgentSuffix::try_from("source-app")?),
            ChangeFeedFactoryMode::Incremental,
            1,
        )
        .await?;
        let lease_driver = emulator
            .runtime_builder()
            .with_wrapping_sdk_identifier("azsdk-native-cosmos/1.0")
            .with_user_agent_suffix(UserAgentSuffix::try_from("runtime-default")?)
            .build()
            .await?
            .create_driver(
                DriverOptions::builder(context.source_driver.account().clone())
                    .with_user_agent_suffix(UserAgentSuffix::try_from("lease-app")?)
                    .build(),
            )
            .await?;
        Arc::get_mut(&mut context).unwrap().lease_driver = lease_driver;
        let source_agent = context.source_driver.user_agent().as_str().to_owned();
        let lease_agent = context.lease_driver.user_agent().as_str().to_owned();
        assert!(source_agent.contains("host=worker"));
        assert!(lease_agent.ends_with("lease-app"));
        requests.take();

        bootstrap(
            &context.source_driver,
            &context.source,
            &context.lease_driver,
            &context.leases,
            context.options.mode,
            "tagged-bootstrap",
            "worker",
        )
        .await?;
        let bootstrap_requests = requests.take();
        assert_tagged(&bootstrap_requests, &source_agent, &lease_agent);
        assert!(bootstrap_requests
            .iter()
            .any(|r| r.path.ends_with("/pkranges")));
        assert!(bootstrap_requests
            .iter()
            .any(|r| r.method == Method::Delete));
        assert!(bootstrap_requests.iter().any(|r| r.method == Method::Post
            && r.body.as_ref().is_some_and(|b| b.get("range").is_some())));

        let lease = context.list().await?.remove(0);
        let stored = context
            .store()
            .read::<Lease>(&lease.base.record.id)
            .await?
            .unwrap();
        assert!(
            !context
                .recover(Arc::new(async_lock::Mutex::new(stored.into())))
                .await?
        );
        let recovery_requests = requests.take();
        assert_tagged(&recovery_requests, &source_agent, &lease_agent);
        assert!(recovery_requests
            .iter()
            .any(|r| r.path.ends_with("/pkranges")));

        insert(&context, "one").await?;
        let options = context.options.clone().with_checkpoint_policy(policy);
        requests.take();
        let processor = start(&context, options).await?;
        let (_, response, receipt) = next(&processor).await.into_parts();
        drop(response);
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let writes = requests
                    .0
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|r| {
                        r.method == Method::Put
                            && r.body.as_ref().is_some_and(|b| b.get("owner").is_some())
                    })
                    .count();
                if writes >= 2 {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("lease renews with a pending callback");
        let mut stopping = Box::pin(processor.stop());
        assert!(futures::poll!(stopping.as_mut()).is_pending());
        receipt.acknowledge().await?;
        stopping.await?;
        let processing_requests = requests.take();
        assert_tagged(&processing_requests, &source_agent, &lease_agent);
        assert!(processing_requests
            .iter()
            .any(|r| r.method == Method::Get && r.path.ends_with("/docs")));
        assert!(processing_requests.iter().any(|r| r.method == Method::Put
            && r.body
                .as_ref()
                .is_some_and(|b| b.get("continuation").is_some())));
        assert!(processing_requests.iter().any(|r| r.method == Method::Put
            && r.body.as_ref().is_some_and(|b| b.get("owner").is_none())));

        insert(&context, "ordinary").await?;
        let mut plan = context
            .source_driver
            .plan_operation(
                CosmosOperation::change_feed(context.source.clone(), Some(FeedRange::full()))
                    .with_change_feed_start(ChangeFeedStartFrom::Beginning),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await?;
        context
            .source_driver
            .execute_plan(
                &mut plan,
                Some(context.source.clone()),
                OperationOptions::default(),
            )
            .await?;
        context
            .source_driver
            .resolve_all_partition_key_ranges(&context.source, true)
            .await?;
        let ordinary_requests = requests.take();
        assert!(!ordinary_requests.is_empty());
        for request in ordinary_requests {
            assert_eq!(request.agent, source_agent);
        }
        assert_eq!(context.source_driver.user_agent().as_str(), source_agent);
        assert_eq!(context.lease_driver.user_agent().as_str(), lease_agent);
    }
    Ok(())
}
