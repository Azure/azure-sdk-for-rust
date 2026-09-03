// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    Method, Request, StatusCode, Url,
};
use azure_data_cosmos_driver::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
        VirtualRegion,
    },
    models::{AccountReference, CosmosOperation, ItemReference, PartitionKey, SubStatusCode},
    options::{
        DriverOptions, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions, Region,
    },
    CosmosDriver,
};

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const WEST_GATEWAY_URL: &str = "https://westus.emulator.local";
const DATABASE_NAME: &str = "recreation-db";
const CONTAINER_NAME: &str = "recreation-coll";
const ITEM_ID: &str = "replacement-item";
const PARTITION_KEY_VALUE: &str = "pk1";

static PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");

fn response_rule(
    id: &'static str,
    status: StatusCode,
    sub_status: u16,
    hit_limit: u32,
) -> Arc<FaultInjectionRule> {
    let response = CustomResponseBuilder::new(status)
        .with_sub_status(sub_status)
        .with_body(br#"{"code":"Injected","message":"container metadata is stale"}"#.to_vec())
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_custom_response(response)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .with_hit_limit(hit_limit)
            .build(),
    );
    rule.disable();
    rule
}

async fn setup(
    rule: Arc<FaultInjectionRule>,
) -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<CosmosDriver>,
    azure_data_cosmos_driver::models::ContainerReference,
) {
    setup_with_observer(rule, None).await
}

async fn setup_with_observer(
    rule: Arc<FaultInjectionRule>,
    observer: Option<Arc<dyn RequestObserver>>,
) -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<CosmosDriver>,
    azure_data_cosmos_driver::models::ContainerReference,
) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    setup_with_config(vec![rule], observer, config, None).await
}

async fn setup_with_config(
    rules: Vec<Arc<FaultInjectionRule>>,
    observer: Option<Arc<dyn RequestObserver>>,
    config: VirtualAccountConfig,
    preferred_regions: Option<Vec<Region>>,
) -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<CosmosDriver>,
    azure_data_cosmos_driver::models::ContainerReference,
) {
    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator = Arc::new(match observer {
        Some(observer) => emulator.with_request_observer(observer),
        None => emulator,
    });
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        CONTAINER_NAME,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );
    let runtime = emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .unwrap();
    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let mut driver_options = DriverOptions::builder(account);
    if let Some(preferred_regions) = preferred_regions {
        driver_options = driver_options.with_preferred_regions(preferred_regions);
    }
    let driver = runtime.create_driver(driver_options.build()).await.unwrap();
    let old_container = driver
        .resolve_container(DATABASE_NAME, CONTAINER_NAME, OperationOptions::default())
        .await
        .unwrap();
    (emulator, driver, old_container)
}

struct EnableRuleAfterContainerRefresh {
    armed: AtomicBool,
    rule: Arc<FaultInjectionRule>,
}

impl fmt::Debug for EnableRuleAfterContainerRefresh {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EnableRuleAfterContainerRefresh")
            .field("armed", &self.armed.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl RequestObserver for EnableRuleAfterContainerRefresh {
    fn on_request(&self, request: &Request) {
        let expected_path = format!("/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}");
        if self.armed.load(Ordering::SeqCst)
            && request.method() == Method::Get
            && request.url().path() == expected_path
        {
            self.rule.enable();
        }
    }
}

async fn recreate_and_seed(emulator: &InMemoryEmulatorHttpClient) {
    let container_url = Url::parse(&format!(
        "{GATEWAY_URL}/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}"
    ))
    .unwrap();
    let response = emulator
        .execute_request(&Request::new(container_url, Method::Delete))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NoContent);

    emulator.store().create_container(
        DATABASE_NAME,
        CONTAINER_NAME,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    seed_at(emulator, GATEWAY_URL).await;
}

async fn seed_at(emulator: &InMemoryEmulatorHttpClient, gateway_url: &str) {
    let mut request = Request::new(
        Url::parse(&format!(
            "{gateway_url}/dbs/{DATABASE_NAME}/colls/{CONTAINER_NAME}/docs"
        ))
        .unwrap(),
        Method::Post,
    );
    request.headers_mut().insert(
        PARTITION_KEY.clone(),
        HeaderValue::from_static(r#"["pk1"]"#),
    );
    request.set_body(
        serde_json::to_vec(&serde_json::json!({
            "id": ITEM_ID,
            "pk": PARTITION_KEY_VALUE,
            "value": 1
        }))
        .unwrap(),
    );
    let response = emulator.execute_request(&request).await.unwrap();
    assert_eq!(response.status(), StatusCode::Created);
}

async fn verify_recovery(rule: Arc<FaultInjectionRule>, expected_fault_count: u32) {
    let (emulator, driver, old_container) = setup(rule.clone()).await;
    recreate_and_seed(&emulator).await;
    rule.enable();

    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &old_container,
                PartitionKey::from(PARTITION_KEY_VALUE),
                ITEM_ID.to_owned(),
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(rule.hit_count(), expected_fault_count);
    assert_eq!(
        response.diagnostics().request_count(),
        expected_fault_count as usize + 1
    );
}

#[tokio::test]
async fn name_cache_stale_410_1000_refreshes_and_retries_once() {
    let rule = response_rule("recreation-410-1000", StatusCode::Gone, 1000, 1);
    verify_recovery(rule, 1).await;
}

#[tokio::test]
async fn exhausted_read_session_404_1002_refreshes_and_retries_once() {
    let rule = response_rule("recreation-404-1002", StatusCode::NotFound, 1002, 3);
    verify_recovery(rule, 3).await;
}

#[tokio::test]
async fn repeated_recreation_signal_does_not_receive_a_second_recovery_budget() {
    let rule = response_rule(
        "recreation-410-1000-after-refresh",
        StatusCode::Gone,
        1000,
        u32::MAX,
    );
    let observer = Arc::new(EnableRuleAfterContainerRefresh {
        armed: AtomicBool::new(false),
        rule: Arc::clone(&rule),
    });
    let (emulator, driver, old_container) =
        setup_with_observer(rule.clone(), Some(observer.clone())).await;
    recreate_and_seed(&emulator).await;
    observer.armed.store(true, Ordering::SeqCst);

    let error = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &old_container,
                PartitionKey::from(PARTITION_KEY_VALUE),
                ITEM_ID.to_owned(),
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap_err();

    assert_eq!(error.status().status_code(), StatusCode::Gone);
    assert_eq!(
        error.status().sub_status().map(|value| value.into()),
        Some(1000)
    );
    // The replacement request plus four ordinary 410 routing retries consume
    // five hits. Any additional hit means the plan coordinator incorrectly
    // opened another recreation recovery after that sequence terminated.
    assert_eq!(rule.hit_count(), 5);
}

#[tokio::test]
async fn recreation_refresh_honors_excluded_regions() {
    let recreation_response = CustomResponseBuilder::new(StatusCode::Gone)
        .with_sub_status(1000)
        .with_body(br#"{"code":"Injected","message":"container metadata is stale"}"#.to_vec())
        .build();
    let recreation_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "recreation-options-excluded-region",
            FaultInjectionResultBuilder::new()
                .with_custom_response(recreation_response)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::WEST_US)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );
    recreation_rule.disable();
    let metadata_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "recreation-metadata-excluded-region",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(1))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::MetadataReadContainer)
                .with_region(Region::EAST_US)
                .build(),
        )
        .build(),
    );
    metadata_rule.disable();
    let west_metadata_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "recreation-metadata-included-region",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(1))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::MetadataReadContainer)
                .with_region(Region::WEST_US)
                .build(),
        )
        .build(),
    );
    west_metadata_rule.disable();
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(WEST_GATEWAY_URL).unwrap()),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let (emulator, driver, old_container) = setup_with_config(
        vec![
            recreation_rule.clone(),
            metadata_rule.clone(),
            west_metadata_rule.clone(),
        ],
        None,
        config,
        Some(vec![Region::EAST_US, Region::WEST_US]),
    )
    .await;
    recreate_and_seed(&emulator).await;
    emulator
        .store()
        .set_write_mode(azure_data_cosmos_driver::in_memory_emulator::WriteMode::Multi);
    seed_at(&emulator, WEST_GATEWAY_URL).await;
    emulator
        .store()
        .set_write_mode(azure_data_cosmos_driver::in_memory_emulator::WriteMode::Single);
    recreation_rule.enable();
    metadata_rule.enable();
    west_metadata_rule.enable();

    let mut options = OperationOptions::default();
    options.excluded_regions = Some(ExcludedRegions::from_iter([Region::EAST_US]));
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &old_container,
                PartitionKey::from(PARTITION_KEY_VALUE),
                ITEM_ID.to_owned(),
            )),
            options,
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(recreation_rule.hit_count(), 1);
    assert_eq!(west_metadata_rule.hit_count(), 1);
    assert_eq!(
        metadata_rule.hit_count(),
        0,
        "the container refresh must not use an excluded region"
    );
}

#[tokio::test]
async fn recreation_refresh_shares_original_deadline() {
    let recreation_response = CustomResponseBuilder::new(StatusCode::Gone)
        .with_sub_status(1000)
        .with_body(br#"{"code":"Injected","message":"container metadata is stale"}"#.to_vec())
        .build();
    let recreation_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "recreation-options-deadline",
            FaultInjectionResultBuilder::new()
                .with_custom_response(recreation_response)
                .with_delay(Duration::from_millis(600))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .build(),
        )
        .with_hit_limit(1)
        .build(),
    );
    recreation_rule.disable();
    let metadata_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "recreation-metadata-deadline",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(600))
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::MetadataReadContainer)
                .build(),
        )
        .build(),
    );
    metadata_rule.disable();
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let (emulator, driver, old_container) = setup_with_config(
        vec![recreation_rule.clone(), metadata_rule.clone()],
        None,
        config,
        None,
    )
    .await;
    recreate_and_seed(&emulator).await;
    recreation_rule.enable();
    metadata_rule.enable();

    let mut options = OperationOptions::default();
    options.end_to_end_latency_policy =
        Some(EndToEndOperationLatencyPolicy::new(Duration::from_secs(1)));
    let result = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &old_container,
                PartitionKey::from(PARTITION_KEY_VALUE),
                ITEM_ID.to_owned(),
            )),
            options,
        )
        .await;

    assert_eq!(recreation_rule.hit_count(), 1);
    assert_eq!(metadata_rule.hit_count(), 1);
    let error = result.unwrap_err();
    assert_eq!(error.status().status_code(), StatusCode::RequestTimeout);
    assert_eq!(
        error.status().sub_status(),
        Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
    );
}
