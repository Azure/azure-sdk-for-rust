// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod managed_lifecycle;

use super::{ChangeFeedProcessorBuilder, ContainerBinding, ProcessorEngine};
use crate::{
    BootstrapPlan, ChangeFeedMode, ChangeFeedProcessor, ChangeFeedReadOptions, InitialLease,
    LeaseOwnershipOptions, LeaseRunOptions, ReadChangesOptions,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    http::{headers::HeaderName, Method, Request, StatusCode},
    time::OffsetDateTime,
};
use azure_data_cosmos_driver::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{
        ChangeFeedStartFrom, ContinuationToken, CosmosOperation, FeedRange, ItemReference,
        PartitionKey,
    },
    options::{OperationOptions, Region},
    CosmosError,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

#[derive(Debug)]
struct Credential {
    token: &'static str,
    calls: AtomicU32,
    initial_delay: Duration,
}
impl Credential {
    fn new(token: &'static str) -> Arc<Self> {
        Arc::new(Self {
            token,
            calls: AtomicU32::new(0),
            initial_delay: Duration::ZERO,
        })
    }
}
#[async_trait::async_trait]
impl TokenCredential for Credential {
    async fn get_token(
        &self,
        _: &[&str],
        _: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        if self.calls.fetch_add(1, Ordering::SeqCst) == 0 {
            tokio::time::sleep(self.initial_delay).await;
        }
        Ok(AccessToken {
            token: Secret::new(self.token),
            expires_on: OffsetDateTime::now_utc() + azure_core::time::Duration::hours(1),
        })
    }
}

#[derive(Debug, Default)]
struct Traffic(Mutex<Vec<(Method, String, String)>>);
impl RequestObserver for Traffic {
    fn on_request(&self, request: &Request) {
        self.0.lock().unwrap().push((
            request.method(),
            request.url().path().to_owned(),
            request
                .headers()
                .get_optional_str(&HeaderName::from_static("authorization"))
                .unwrap_or_default()
                .to_owned(),
        ));
    }
}
impl Traffic {
    fn metadata_reads(&self) -> usize {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|(method, path, _)| {
                *method == Method::Get && path.trim_end_matches('/').ends_with("/colls/container")
            })
            .count()
    }
    fn feed_reads(&self) -> usize {
        self.0
            .lock()
            .unwrap()
            .iter()
            .filter(|(method, path, _)| {
                *method == Method::Get && path.trim_end_matches('/').ends_with("/docs")
            })
            .count()
    }
}

fn account_binding(
    endpoint: &str,
    credential: Arc<dyn TokenCredential>,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> Result<(ContainerBinding, Arc<Traffic>), Box<dyn Error>> {
    let endpoint: url::Url = endpoint.parse()?;
    let traffic = Arc::new(Traffic::default());
    let emulator = Arc::new(
        InMemoryEmulatorHttpClient::new(VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            endpoint.clone(),
        )])?)
        .with_request_observer(traffic.clone()),
    );
    emulator.store().create_database("db");
    emulator.store().create_container_with_config(
        "db",
        "container",
        serde_json::from_value(json!({"paths":["/workload"],"kind":"Hash","version":2}))?,
        ContainerConfig::new().with_partition_count(1).build()?,
    );
    let runtime = emulator.runtime_builder_with_fault_rules(rules);
    Ok((
        ContainerBinding::with_token_credential(endpoint, "db", "container", credential)?
            .with_runtime_builder(runtime),
        traffic,
    ))
}

fn policy() -> LeaseOwnershipOptions {
    LeaseOwnershipOptions::new(
        Duration::from_secs(2),
        Duration::from_millis(200),
        Duration::from_millis(500),
        Duration::from_millis(300),
    )
    .unwrap()
}

fn rule(operation: FaultOperationType, status: StatusCode) -> Arc<FaultInjectionRule> {
    Arc::new(
        FaultInjectionRuleBuilder::new(
            "binding-failure",
            FaultInjectionResultBuilder::new()
                .with_probability(1.0)
                .with_custom_response(
                    CustomResponseBuilder::new(status)
                        .with_body(
                            br#"{"code":"Forbidden","message":"binding-test-denied"}"#.to_vec(),
                        )
                        .build(),
                )
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(operation)
                .build(),
        )
        .build(),
    )
}

#[tokio::test]
async fn build_is_metadata_only_and_retains_separately_bound_resources(
) -> Result<(), Box<dyn Error>> {
    let feed_credential = Credential::new("feed-token");
    let lease_credential = Credential::new("lease-token");
    let (feed, feed_traffic) = account_binding(
        "https://feed.emulator.local",
        feed_credential.clone(),
        vec![],
    )?;
    let (leases, lease_traffic) = account_binding(
        "https://leases.emulator.local",
        lease_credential.clone(),
        vec![],
    )?;
    let mut feed_defaults = OperationOptions::default();
    feed_defaults.end_to_end_latency_policy = Some(Duration::from_secs(3).into());
    let mut lease_defaults = OperationOptions::default();
    lease_defaults.end_to_end_latency_policy = Some(Duration::from_secs(7).into());
    let p = ChangeFeedProcessor::builder()
        .build(
            "group",
            feed.with_operation_options(feed_defaults)
                .with_preferred_regions(vec![Region::from("East US")]),
            leases
                .with_operation_options(lease_defaults)
                .with_preferred_regions(vec![Region::from("East US")]),
        )
        .await?;
    assert_eq!(
        p.feed_binding()
            .unwrap()
            .operation_options()
            .end_to_end_latency_policy
            .as_ref()
            .unwrap()
            .timeout(),
        Duration::from_secs(3)
    );
    assert_eq!(
        p.lease_binding()
            .unwrap()
            .operation_options()
            .end_to_end_latency_policy
            .as_ref()
            .unwrap()
            .timeout(),
        Duration::from_secs(7)
    );
    for (traffic, token) in [
        (&feed_traffic, "feed-token"),
        (&lease_traffic, "lease-token"),
    ] {
        let requests = traffic.0.lock().unwrap();
        assert!(!requests.is_empty());
        assert!(requests.iter().all(|(method, _, _)| *method == Method::Get));
        assert!(requests.iter().all(|(_, _, auth)| auth.contains(token)));
    }
    let baseline = (
        feed_traffic.metadata_reads(),
        lease_traffic.metadata_reads(),
    );
    assert_eq!(baseline, (1, 1));
    let page = p.read_page::<Value>(&ReadChangesOptions::default()).await?;
    let plan = p
        .bootstrap_plan(
            ChangeFeedMode::LatestVersion,
            ChangeFeedStartFrom::Beginning,
            FeedRange::full(),
            vec![InitialLease::new(
                FeedRange::full(),
                ContinuationToken::from_string(page.continuation().to_owned()),
            )?],
        )
        .await?;
    for _ in 0..3 {
        p.lease_store(PartitionKey::from("lease"), "lease", policy())
            .await?;
        p.bootstrap_store(plan.clone(), policy()).await?;
        p.read_page::<Value>(&ReadChangesOptions::default().with_continuation(page.continuation()))
            .await?;
    }
    assert_eq!(
        (
            feed_traffic.metadata_reads(),
            lease_traffic.metadata_reads()
        ),
        baseline
    );
    assert!(feed_credential.calls.load(Ordering::SeqCst) > 0);
    assert!(lease_credential.calls.load(Ordering::SeqCst) > 0);
    Ok(())
}

#[tokio::test]
async fn same_endpoint_and_template_do_not_mix_credential_cache_namespaces(
) -> Result<(), Box<dyn Error>> {
    let a = Credential::new("identity-a");
    let b = Credential::new("identity-b");
    let (feed, traffic) = account_binding("https://same.emulator.local", a, vec![])?;
    let leases =
        ContainerBinding::with_token_credential(feed.endpoint().clone(), "db", "container", b)?
            .with_runtime_builder(feed.runtime_builder().clone());
    let p = ChangeFeedProcessor::builder()
        .build("group", feed, leases)
        .await?;
    {
        let requests = traffic.0.lock().unwrap();
        let metadata: Vec<_> = requests
            .iter()
            .filter(|(_, path, _)| path.trim_end_matches('/').ends_with("/colls/container"))
            .collect();
        assert_eq!(metadata.len(), 2);
        assert!(metadata[0].2.contains("identity-a"));
        assert!(metadata[1].2.contains("identity-b"));
    }
    let resolved = &p.prepared.as_ref().unwrap().leases;
    let wrong_driver = p.feed_binding().unwrap().clone().prepare().await?;
    assert!(
        ProcessorEngine::from_resolved(wrong_driver.driver, resolved.container.clone()).is_err()
    );
    Ok(())
}

#[tokio::test]
async fn explicitly_reusing_one_provider_is_supported() -> Result<(), Box<dyn Error>> {
    let credential = Credential::new("shared-identity");
    let (feed, _) = account_binding("https://feed.emulator.local", credential.clone(), vec![])?;
    let (leases, _) = account_binding("https://leases.emulator.local", credential.clone(), vec![])?;
    let p = ChangeFeedProcessorBuilder::default()
        .build("group", feed, leases)
        .await?;
    assert_eq!(p.group(), Some("group"));
    assert!(credential.calls.load(Ordering::SeqCst) > 0);
    Ok(())
}

#[tokio::test]
async fn preparation_failures_identify_the_side_and_preserve_cosmos_status(
) -> Result<(), Box<dyn Error>> {
    for feed_fails in [true, false] {
        let failure = rule(
            FaultOperationType::MetadataReadContainer,
            StatusCode::Forbidden,
        );
        let (feed, _) = account_binding(
            "https://feed.emulator.local",
            Credential::new("feed"),
            if feed_fails {
                vec![failure.clone()]
            } else {
                vec![]
            },
        )?;
        let (leases, _) = account_binding(
            "https://leases.emulator.local",
            Credential::new("leases"),
            if feed_fails { vec![] } else { vec![failure] },
        )?;
        let error = ChangeFeedProcessor::builder()
            .build("group", feed, leases)
            .await
            .err()
            .unwrap();
        let cosmos = error
            .source()
            .unwrap()
            .downcast_ref::<CosmosError>()
            .unwrap();
        assert_eq!(cosmos.status().status_code(), StatusCode::Forbidden);
        assert!(format!("{cosmos:?}").contains(if feed_fails {
            "preparing feed container binding"
        } else {
            "preparing lease container binding"
        }));
    }
    Ok(())
}

#[tokio::test]
async fn both_bindings_share_one_overall_preparation_deadline() -> Result<(), Box<dyn Error>> {
    let lease_credential = Arc::new(Credential {
        token: "delayed",
        calls: AtomicU32::new(0),
        initial_delay: Duration::from_secs(30),
    });
    let (feed, _) = account_binding(
        "https://feed.emulator.local",
        Credential::new("feed"),
        vec![],
    )?;
    let (leases, _) = account_binding(
        "https://leases.emulator.local",
        lease_credential.clone(),
        vec![],
    )?;
    let started = std::time::Instant::now();
    let error = tokio::time::timeout(
        Duration::from_secs(6),
        ChangeFeedProcessor::builder()
            .with_preparation_timeout(Duration::from_secs(4))
            .build("group", feed, leases),
    )
    .await?
    .err()
    .unwrap();
    let cosmos = error
        .source()
        .unwrap()
        .downcast_ref::<CosmosError>()
        .unwrap();
    assert_eq!(cosmos.status().status_code(), StatusCode::RequestTimeout);
    assert!(lease_credential.calls.load(Ordering::SeqCst) > 0);
    assert!(started.elapsed() < Duration::from_millis(4500));
    assert!(
        format!("{cosmos:?}").contains("preparing lease container binding"),
        "{cosmos:?}"
    );
    Ok(())
}

#[tokio::test]
async fn source_only_constructor_never_falls_back_to_feed_credentials() -> Result<(), Box<dyn Error>>
{
    let (p, _) = crate::tests::setup_with_driver(1).await?;
    assert!(p
        .lease_store(PartitionKey::from("lease"), "lease", policy())
        .await
        .is_err());
    assert!(p.lease_binding().is_none());
    Ok(())
}

#[tokio::test]
async fn bootstrap_rejects_a_foreign_source_even_with_the_same_group() -> Result<(), Box<dyn Error>>
{
    let (feed, _) = account_binding(
        "https://feed.emulator.local",
        Credential::new("feed"),
        vec![],
    )?;
    let (leases, traffic) = account_binding(
        "https://leases.emulator.local",
        Credential::new("leases"),
        vec![],
    )?;
    let p = ChangeFeedProcessor::builder()
        .build("group", feed, leases)
        .await?;
    let page = p.read_page::<Value>(&ReadChangesOptions::default()).await?;
    let plan = BootstrapPlan::new(
        "different-rid",
        "group",
        ChangeFeedMode::LatestVersion,
        ChangeFeedStartFrom::Beginning,
        FeedRange::full(),
        vec![InitialLease::new(
            FeedRange::full(),
            ContinuationToken::from_string(page.continuation().to_owned()),
        )?],
    )?;
    let before = traffic.0.lock().unwrap().len();
    assert!(p.bootstrap_store(plan, policy()).await.is_err());
    assert_eq!(traffic.0.lock().unwrap().len(), before);
    Ok(())
}

#[tokio::test]
async fn lease_authorization_failure_after_callback_success_prevents_next_feed_read(
) -> Result<(), Box<dyn Error>> {
    let denial = rule(FaultOperationType::ReplaceItem, StatusCode::Forbidden);
    denial.disable();
    let (feed, feed_traffic) = account_binding(
        "https://feed.emulator.local",
        Credential::new("feed"),
        vec![],
    )?;
    let setup = feed.clone().prepare().await?;
    let (leases, _) = account_binding(
        "https://leases.emulator.local",
        Credential::new("leases"),
        vec![denial.clone()],
    )?;
    let p = ChangeFeedProcessor::builder()
        .build("group", feed, leases)
        .await?;
    let seed = p.read_page::<Value>(&ReadChangesOptions::default()).await?;
    setup
        .driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &setup.container,
                PartitionKey::from("doc"),
                "doc",
            ))
            .with_body(serde_json::to_vec(&json!({"id":"doc","workload":"doc"}))?),
            OperationOptions::default(),
        )
        .await?;
    let bound = &p.prepared.as_ref().unwrap().leases;
    bound
        .driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &bound.container,
                PartitionKey::from("lease"),
                "lease",
            ))
            .with_body(serde_json::to_vec(&json!({
                "id":"lease","workload":"lease","version":1,
                "ownership":{"owner":null,"generation":0}, "range":FeedRange::full(),
                "checkpoint":seed.continuation(),"lease_duration_ms":2000,
            }))?),
            OperationOptions::default(),
        )
        .await?;
    let store = p
        .lease_store(PartitionKey::from("lease"), "lease", policy())
        .await?;
    let session = store.try_acquire("worker").await?.unwrap();
    let before = feed_traffic.feed_reads();
    let calls = AtomicU32::new(0);
    let result = p
        .run_with_lease_session::<Value, _, _>(
            &session,
            ChangeFeedReadOptions::new(FeedRange::full(), ChangeFeedStartFrom::Beginning),
            &LeaseRunOptions::new(NonZeroU32::new(2).unwrap()),
            |_| {
                calls.fetch_add(1, Ordering::SeqCst);
                denial.enable();
                async { Ok(()) }
            },
        )
        .await;
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(result.processing().is_err());
    assert_eq!(feed_traffic.feed_reads(), before + 1);
    assert!(denial.hit_count() > 0);
    denial.disable();
    assert_eq!(store.observe().await?.checkpoint(), seed.continuation());
    Ok(())
}

#[test]
fn account_key_debug_is_redacted() -> Result<(), Box<dyn Error>> {
    let binding = ContainerBinding::with_key(
        "https://account.example".parse()?,
        "db",
        "leases",
        Secret::new("never-print-this-key"),
    )?;
    assert!(!format!("{binding:?}").contains("never-print-this-key"));
    Ok(())
}
