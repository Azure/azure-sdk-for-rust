// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end routing guarantees for PATCH Read-Modify-Write verification reads.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use azure_core::http::{headers::HeaderName, Method, Request, Url};
use azure_data_cosmos_driver::diagnostics::RequestEventType;
#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, RequestObserver,
    VirtualAccountConfig, VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, ItemReference, PartitionKey,
    PatchInstructions, PatchOperation,
};
#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::options::ExcludedRegions;
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions,
    OperationOptionsBuilder, PartitionFailoverOptions, PatchStrategy, Region,
};
use azure_data_cosmos_driver::{CosmosDriver, CosmosDriverRuntime};

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
#[cfg(feature = "fault_injection")]
const CENTRAL_URL: &str = "https://centralus.emulator.local";
const EAST_HOST: &str = "eastus.emulator.local";
const WEST_HOST: &str = "westus.emulator.local";
const DB_NAME: &str = "testdb";
const CONTAINER_NAME: &str = "testcoll";
const PK: &str = "pk1";
const READ_CONSISTENCY_STRATEGY: HeaderName =
    HeaderName::from_static("x-ms-cosmos-read-consistency-strategy");
const SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");

#[derive(Clone, Debug, PartialEq, Eq)]
struct RecordedRequest {
    method: Method,
    host: String,
    read_consistency_strategy: Option<String>,
    session_token: Option<String>,
}

#[derive(Debug, Default)]
struct ItemRequestRecorder {
    requests: Mutex<Vec<RecordedRequest>>,
}

impl ItemRequestRecorder {
    fn clear(&self) {
        self.requests.lock().unwrap().clear();
    }

    fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }
}

impl RequestObserver for ItemRequestRecorder {
    fn on_request(&self, request: &Request) {
        if !request.url().path().contains("/docs/") {
            return;
        }
        self.requests.lock().unwrap().push(RecordedRequest {
            method: request.method(),
            host: request.url().host_str().unwrap_or_default().to_owned(),
            read_consistency_strategy: request
                .headers()
                .get_optional_str(&READ_CONSISTENCY_STRATEGY)
                .map(str::to_owned),
            session_token: request
                .headers()
                .get_optional_str(&SESSION_TOKEN)
                .map(str::to_owned),
        });
    }
}

struct Fixture {
    _emulator: Arc<InMemoryEmulatorHttpClient>,
    recorder: Arc<ItemRequestRecorder>,
    driver: Arc<CosmosDriver>,
    container: ContainerReference,
}

fn build_emulator_with_regions(
    recorder: Arc<ItemRequestRecorder>,
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(regions)
        .unwrap()
        .with_write_mode(write_mode)
        .with_consistency(ConsistencyLevel::Session)
        .with_replication_config(ReplicationConfig::immediate());
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(recorder.clone()));
    let store = emulator.store();
    store.create_database(DB_NAME);
    store.create_container(
        DB_NAME,
        CONTAINER_NAME,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );
    emulator
}

fn build_emulator(recorder: Arc<ItemRequestRecorder>) -> Arc<InMemoryEmulatorHttpClient> {
    build_emulator_with_regions(
        recorder,
        vec![
            VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap()),
            VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()),
        ],
        WriteMode::Single,
    )
}

async fn complete_fixture(
    emulator: Arc<InMemoryEmulatorHttpClient>,
    recorder: Arc<ItemRequestRecorder>,
    runtime: Arc<CosmosDriverRuntime>,
    preferred_regions: Vec<Region>,
    ppcb_enabled: bool,
) -> Fixture {
    let account =
        AccountReference::with_master_key(Url::parse(EAST_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver_options = DriverOptions::builder(account)
        .with_preferred_regions(preferred_regions)
        .with_partition_failover_options(
            PartitionFailoverOptions::builder()
                .with_circuit_breaker_enabled(ppcb_enabled)
                .build()
                .unwrap(),
        )
        .build();
    let driver = runtime
        .create_driver(driver_options)
        .await
        .expect("driver initializes against emulator metadata");
    let container = driver
        .resolve_container(DB_NAME, CONTAINER_NAME, OperationOptions::default())
        .await
        .expect("container resolves");

    let item = ItemReference::from_name(&container, PartitionKey::from(PK), "item1");
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item).with_body(
                serde_json::json!({"id": "item1", "pk": PK, "value": "before"})
                    .to_string()
                    .into_bytes(),
            ),
            OperationOptions::default(),
        )
        .await
        .expect("seed write succeeds");
    recorder.clear();

    Fixture {
        _emulator: emulator,
        recorder,
        driver,
        container,
    }
}

async fn build_fixture() -> Fixture {
    let recorder = Arc::new(ItemRequestRecorder::default());
    let emulator = build_emulator(recorder.clone());
    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime builds against the in-memory emulator");
    // Ordinary reads prefer West; the single write region remains East.
    complete_fixture(
        emulator,
        recorder,
        runtime,
        vec![Region::WEST_US, Region::EAST_US],
        false,
    )
    .await
}

#[cfg(feature = "fault_injection")]
async fn build_fault_fixture(
    rules: Vec<Arc<FaultInjectionRule>>,
    preferred_regions: Vec<Region>,
    ppcb_enabled: bool,
) -> Fixture {
    let recorder = Arc::new(ItemRequestRecorder::default());
    let emulator = build_emulator(recorder.clone());
    let runtime = emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("fault-enabled runtime builds against the in-memory emulator");
    complete_fixture(emulator, recorder, runtime, preferred_regions, ppcb_enabled).await
}

fn aggressive_hedging_options() -> OperationOptions {
    let hedging = HedgingStrategy::new(
        HedgeThreshold::new(Duration::from_nanos(1)).expect("threshold is non-zero"),
    );
    OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(hedging))
        .build()
}

async fn patch(fixture: &Fixture) -> azure_data_cosmos_driver::models::CosmosResponse {
    patch_with_options(fixture, aggressive_hedging_options()).await
}

async fn patch_with_options(
    fixture: &Fixture,
    mut options: OperationOptions,
) -> azure_data_cosmos_driver::models::CosmosResponse {
    // These tests exercise the RMW helper Read's routing, so keep PATCH off the
    // one-request server path regardless of the supplied cross-cutting options.
    options.patch_strategy = Some(PatchStrategy::ClientSide);
    let item = ItemReference::from_name(&fixture.container, PartitionKey::from(PK), "item1");
    let patch = PatchInstructions::from(vec![PatchOperation::set(
        "/value",
        serde_json::json!("after"),
    )]);
    fixture
        .driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&patch).unwrap()),
            options,
        )
        .await
        .expect("PATCH succeeds")
}

#[tokio::test]
async fn patch_read_uses_write_region_latest_committed_and_no_hedging() {
    let fixture = build_fixture().await;

    let response = patch(&fixture).await;

    let requests = fixture.recorder.requests();
    assert_eq!(
        requests.len(),
        2,
        "PATCH must issue one Read and one Replace"
    );
    assert_eq!(requests[0].method, Method::Get);
    assert_eq!(requests[0].host, EAST_HOST);
    assert_eq!(
        requests[0].read_consistency_strategy.as_deref(),
        Some("LatestCommitted")
    );
    assert_eq!(requests[0].session_token, None);
    assert_eq!(requests[1].method, Method::Put);
    assert_eq!(requests[1].host, EAST_HOST);
    assert_eq!(
        response.diagnostics().regions_contacted(),
        vec![Region::EAST_US],
        "a hedge leg would add another contacted region"
    );

    let diagnostics = response.diagnostics().requests();
    assert_eq!(
        diagnostics
            .iter()
            .filter(|request| request.operation_name() == Some("patch_read_item"))
            .count(),
        1,
        "PATCH must dispatch exactly one read attempt when the write region is healthy"
    );
    let read = diagnostics
        .iter()
        .find(|request| request.operation_name() == Some("patch_read_item"))
        .expect("aggregated diagnostics include the PATCH read");
    assert_eq!(read.region(), Some(&Region::EAST_US));
    assert!(read
        .events()
        .iter()
        .all(|event| event.event_type() != &RequestEventType::RoutingFallback));
}

#[tokio::test]
async fn patch_read_falls_back_to_normal_routing_with_diagnostics() {
    let fixture = build_fixture().await;
    assert!(
        fixture
            .driver
            .mark_region_endpoint_unavailable_for_testing(&Region::EAST_US),
        "write endpoint must exist in the routing snapshot"
    );

    let response = patch(&fixture).await;

    let requests = fixture.recorder.requests();
    assert_eq!(
        requests.len(),
        2,
        "PATCH must issue one Read and one Replace"
    );
    assert_eq!(requests[0].method, Method::Get);
    assert_eq!(requests[0].host, WEST_HOST);
    assert_eq!(requests[0].read_consistency_strategy, None);
    assert!(requests[0].session_token.is_some());
    assert_eq!(requests[1].method, Method::Put);
    assert_eq!(requests[1].host, EAST_HOST);
    assert_eq!(
        response.diagnostics().regions_contacted(),
        vec![Region::WEST_US, Region::EAST_US]
    );

    let diagnostics = response.diagnostics().requests();
    assert_eq!(
        diagnostics
            .iter()
            .filter(|request| request.operation_name() == Some("patch_read_item"))
            .count(),
        1,
        "fallback must remain sequential rather than racing a hedge leg"
    );
    let read = diagnostics
        .iter()
        .find(|request| request.operation_name() == Some("patch_read_item"))
        .expect("aggregated diagnostics include the PATCH read");
    assert_eq!(read.region(), Some(&Region::WEST_US));
    let fallback = read
        .events()
        .iter()
        .find(|event| event.event_type() == &RequestEventType::RoutingFallback)
        .expect("fallback must be visible in request diagnostics");
    assert_eq!(
        fallback.details(),
        Some("patch_verification_read_write_endpoint_unavailable_or_excluded")
    );
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn delayed_ordinary_read_hedges_but_patch_read_does_not() {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(200))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-verification-east-delay", result)
            .with_condition(condition)
            .build(),
    );
    let fixture = build_fault_fixture(
        vec![Arc::clone(&rule)],
        vec![Region::EAST_US, Region::WEST_US],
        false,
    )
    .await;

    // Control: with the same account, options, and delayed East primary, an
    // ordinary read enters the hedge path and West wins.
    let item = ItemReference::from_name(&fixture.container, PartitionKey::from(PK), "item1");
    let ordinary = fixture
        .driver
        .execute_singleton_operation(
            CosmosOperation::read_item(item),
            aggressive_hedging_options(),
        )
        .await
        .expect("ordinary hedged read succeeds");
    assert!(
        ordinary.diagnostics().hedge_diagnostics().is_some(),
        "the control read must prove this fixture can dispatch a hedge"
    );

    fixture.recorder.clear();
    let response = patch(&fixture).await;

    let requests = fixture.recorder.requests();
    assert_eq!(
        requests
            .iter()
            .map(|request| (request.method, request.host.as_str()))
            .collect::<Vec<_>>(),
        vec![(Method::Get, EAST_HOST), (Method::Put, EAST_HOST)],
        "PATCH must wait for its delayed write-region read instead of hedging West"
    );
    assert_eq!(
        response
            .diagnostics()
            .requests()
            .iter()
            .filter(|request| request.operation_name() == Some("patch_read_item"))
            .count(),
        1
    );
    assert!(
        rule.hit_count() >= 2,
        "both control and PATCH reads hit the delay"
    );
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn real_write_region_failures_fall_back_with_ppcb_enabled() {
    for (id, error) in [
        (
            "patch-verification-east-503",
            FaultInjectionErrorType::ServiceUnavailable,
        ),
        (
            "patch-verification-east-403-3",
            FaultInjectionErrorType::WriteForbidden,
        ),
    ] {
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReadItem)
            .with_region(Region::EAST_US)
            .build();
        let result = FaultInjectionResultBuilder::new()
            .with_error(error)
            .with_probability(1.0)
            .build();
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(id, result)
                .with_condition(condition)
                .with_hit_limit(1)
                .build(),
        );
        let fixture = build_fault_fixture(
            vec![Arc::clone(&rule)],
            vec![Region::EAST_US, Region::WEST_US],
            true,
        )
        .await;

        let response = patch(&fixture).await;

        assert_eq!(rule.hit_count(), 1, "only the first East read is faulted");
        let diagnostics = response.diagnostics().requests();
        let patch_reads: Vec<_> = diagnostics
            .iter()
            .filter(|request| request.operation_name() == Some("patch_read_item"))
            .collect();
        assert_eq!(patch_reads.len(), 2, "East failure then West fallback");
        assert_eq!(patch_reads[0].region(), Some(&Region::EAST_US));
        assert_eq!(patch_reads[1].region(), Some(&Region::WEST_US));
        let fallback = patch_reads[1]
            .events()
            .iter()
            .find(|event| event.event_type() == &RequestEventType::RoutingFallback)
            .expect("retry must record degraded normal-read routing");
        assert_eq!(
            fallback.details(),
            Some("patch_verification_read_write_endpoint_unavailable_or_excluded")
        );

        fixture.recorder.clear();
        let item = ItemReference::from_name(&fixture.container, PartitionKey::from(PK), "item1");
        fixture
            .driver
            .execute_singleton_operation(
                CosmosOperation::read_item(item),
                OperationOptions::default(),
            )
            .await
            .expect("unrelated read succeeds");
        assert_eq!(
            fixture.recorder.requests()[0].host,
            EAST_HOST,
            "a PATCH verification failure must not mark East unavailable for unrelated reads"
        );
    }
}

#[tokio::test]
async fn stale_fallback_session_retries_before_increment_replace() {
    let fixture = build_fixture().await;
    fixture._emulator.store().pause_replication("West US");

    let item = ItemReference::from_name(&fixture.container, PartitionKey::from(PK), "item1");
    let replace_response = fixture
        .driver
        .execute_singleton_operation(
            CosmosOperation::replace_item(item.clone()).with_body(
                serde_json::json!({
                    "id": "item1",
                    "pk": PK,
                    "value": "newer-east-value",
                    "counter": 1,
                    "preserved": "east-only"
                })
                .to_string()
                .into_bytes(),
            ),
            OperationOptions::default(),
        )
        .await
        .expect("East update succeeds while West replication is paused");
    let external_session_token = replace_response
        .headers()
        .session_token
        .clone()
        .expect("the East write returns a session token");

    // Create a separate driver after the write so its item-session cache is
    // empty. The only way it can preserve the first driver's session on a
    // reader fallback is through the explicit token on the PATCH operation.
    let fallback_runtime = fixture
        ._emulator
        .runtime_builder()
        .build()
        .await
        .expect("fallback runtime builds");
    let fallback_account =
        AccountReference::with_master_key(Url::parse(EAST_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let fallback_options = DriverOptions::builder(fallback_account)
        .with_preferred_regions(vec![Region::WEST_US, Region::EAST_US])
        .build();
    let fallback_driver = fallback_runtime
        .create_driver(fallback_options)
        .await
        .expect("fallback driver initializes");
    let fallback_container = fallback_driver
        .resolve_container(DB_NAME, CONTAINER_NAME, OperationOptions::default())
        .await
        .expect("fallback container resolves");
    let fallback_item =
        ItemReference::from_name(&fallback_container, PartitionKey::from(PK), "item1");

    fixture.recorder.clear();
    assert!(fallback_driver.mark_region_endpoint_unavailable_for_testing(&Region::EAST_US));

    let patch = PatchInstructions::from(vec![PatchOperation::increment("/counter", 1_i64)]);
    let response = fallback_driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(fallback_item)
                .with_body(serde_json::to_vec(&patch).unwrap())
                .with_session_token(external_session_token.clone()),
            aggressive_hedging_options(),
        )
        .await
        .expect("session retry reaches the writer and PATCH succeeds");

    let requests = fixture.recorder.requests();
    assert_eq!(
        requests
            .iter()
            .map(|request| (request.method, request.host.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (Method::Get, WEST_HOST),
            (Method::Get, EAST_HOST),
            (Method::Put, EAST_HOST),
        ],
        "the stale fallback must session-retry before issuing the Replace"
    );
    assert_eq!(
        requests[0].session_token.as_deref(),
        Some(external_session_token.0.as_ref())
    );
    assert_eq!(requests[0].read_consistency_strategy, None);
    assert_eq!(
        requests[1].read_consistency_strategy.as_deref(),
        Some("LatestCommitted")
    );
    assert_eq!(requests[1].session_token, None);

    let body: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .single()
            .expect("PATCH response has a body"),
    )
    .unwrap();
    assert_eq!(body["counter"], 2);
    assert_eq!(body["preserved"], "east-only");
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn sent_transport_failure_skips_selected_writer_after_excluded_head() {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::WEST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeout)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-verification-west-timeout", result)
            .with_condition(condition)
            .build(),
    );
    let recorder = Arc::new(ItemRequestRecorder::default());
    let emulator = build_emulator_with_regions(
        recorder.clone(),
        vec![
            VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap()),
            VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()),
            VirtualRegion::new("Central US", Url::parse(CENTRAL_URL).unwrap()),
        ],
        WriteMode::Multi,
    );
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![Arc::clone(&rule)])
        .build()
        .await
        .expect("fault-enabled runtime builds");
    let fixture = complete_fixture(
        emulator,
        recorder,
        runtime,
        vec![Region::EAST_US, Region::WEST_US, Region::CENTRAL_US],
        true,
    )
    .await;
    let mut options = aggressive_hedging_options();
    options.excluded_regions = Some(ExcludedRegions::new().with_region(Region::EAST_US));

    let response = patch_with_options(&fixture, options).await;

    assert!(
        rule.hit_count() >= 2,
        "the West transport pipeline should exhaust its local retry before failover"
    );
    let patch_read_regions: Vec<_> = response
        .diagnostics()
        .requests()
        .iter()
        .filter(|request| request.operation_name() == Some("patch_read_item"))
        .filter_map(|request| request.region().cloned())
        .collect();
    assert_eq!(
        patch_read_regions,
        vec![Region::WEST_US, Region::WEST_US, Region::CENTRAL_US],
        "East is excluded; West gets one transport-local retry, then operation-level routing must advance to Central"
    );
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn fallback_retry_skips_failed_reader_for_next_normal_reader() {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::WEST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeout)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-fallback-west-timeout", result)
            .with_condition(condition)
            .build(),
    );
    let recorder = Arc::new(ItemRequestRecorder::default());
    let emulator = build_emulator_with_regions(
        recorder.clone(),
        vec![
            VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap()),
            VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()),
            VirtualRegion::new("Central US", Url::parse(CENTRAL_URL).unwrap()),
        ],
        WriteMode::Single,
    );
    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![Arc::clone(&rule)])
        .build()
        .await
        .expect("fault-enabled runtime builds");
    let fixture = complete_fixture(
        emulator,
        recorder,
        runtime,
        vec![Region::WEST_US, Region::CENTRAL_US, Region::EAST_US],
        true,
    )
    .await;
    assert!(fixture
        .driver
        .mark_region_endpoint_unavailable_for_testing(&Region::EAST_US));

    let response = patch(&fixture).await;

    assert!(rule.hit_count() >= 2, "West gets one transport-local retry");
    let requests = response.diagnostics().requests();
    let patch_reads: Vec<_> = requests
        .iter()
        .filter(|request| request.operation_name() == Some("patch_read_item"))
        .collect();
    assert_eq!(
        patch_reads
            .iter()
            .filter_map(|request| request.region().cloned())
            .collect::<Vec<_>>(),
        vec![Region::WEST_US, Region::WEST_US, Region::CENTRAL_US]
    );
    assert!(patch_reads.iter().all(|request| {
        request
            .events()
            .iter()
            .any(|event| event.event_type() == &RequestEventType::RoutingFallback)
    }));
}
