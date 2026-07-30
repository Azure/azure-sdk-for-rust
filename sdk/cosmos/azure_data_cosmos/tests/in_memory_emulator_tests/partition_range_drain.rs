// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Large partition-range metadata drains through the public SDK.

use std::sync::{Arc, Mutex};

use azure_core::{
    credentials::Secret,
    http::{headers::HeaderName, Request, Url},
};
use azure_data_cosmos::{
    options::Region, AccountEndpoint, AccountReference, CosmosClientBuilder, CosmosRuntimeBuilder,
    RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver,
    VirtualAccountConfig, VirtualRegion,
};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const PARTITION_COUNT: usize = 25_000;
const PKRANGE_PAGE_SIZE: usize = 1_000;
const IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
const A_IM: HeaderName = HeaderName::from_static("a-im");

#[cfg(feature = "fault_injection")]
struct PartitionRangeFaultHarness {
    runtime: azure_data_cosmos::CosmosRuntime,
}

#[cfg(feature = "fault_injection")]
impl PartitionRangeFaultHarness {
    const DATABASE: &'static str = "range-fault-db";
    const CONTAINER: &'static str = "range-fault-container";

    async fn new() -> Self {
        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
        )])
        .unwrap()
        .with_consistency(ConsistencyLevel::Session);
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        let store = emulator.store();
        store.create_database(Self::DATABASE);
        store.create_container(
            Self::DATABASE,
            Self::CONTAINER,
            serde_json::from_value(serde_json::json!({
                "paths": ["/pk"],
                "kind": "Hash",
                "version": 2,
            }))
            .unwrap(),
        );
        let runtime = CosmosRuntimeBuilder::from(emulator.runtime_builder())
            .build()
            .await
            .unwrap();
        Self { runtime }
    }

    async fn container(
        &self,
        rules: Vec<Arc<azure_data_cosmos::fault_injection::FaultInjectionRule>>,
    ) -> azure_data_cosmos::ContainerClient {
        let account = AccountReference::with_authentication_key(
            EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
            Secret::new("dGVzdGtleQ=="),
        );
        let client = CosmosClientBuilder::new()
            .with_runtime(self.runtime.clone())
            .with_fault_injection_rules(rules)
            .unwrap()
            .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
            .await
            .unwrap();
        client
            .database_client(Self::DATABASE)
            .container_client(Self::CONTAINER)
            .await
            .unwrap()
    }

    fn rule(
        id: impl Into<String>,
        response: azure_data_cosmos::fault_injection::CustomResponse,
        hit_limit: Option<u32>,
    ) -> Arc<azure_data_cosmos::fault_injection::FaultInjectionRule> {
        use azure_data_cosmos::fault_injection::{
            FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
            FaultOperationType,
        };

        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(response)
            .build();
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
            .build();
        let mut builder = FaultInjectionRuleBuilder::new(id, result).with_condition(condition);
        if let Some(hit_limit) = hit_limit {
            builder = builder.with_hit_limit(hit_limit);
        }
        Arc::new(builder.build())
    }
}

#[derive(Clone, Debug)]
struct PartitionRangeRequest {
    if_none_match: Option<String>,
    a_im: Option<String>,
}

#[derive(Debug, Default)]
struct PartitionRangeObserver {
    requests: Mutex<Vec<PartitionRangeRequest>>,
}

impl RequestObserver for PartitionRangeObserver {
    fn on_request(&self, request: &Request) {
        if !request.url().path().ends_with("/pkranges") {
            return;
        }

        self.requests.lock().unwrap().push(PartitionRangeRequest {
            if_none_match: request
                .headers()
                .get_optional_str(&IF_NONE_MATCH)
                .map(str::to_owned),
            a_im: request.headers().get_optional_str(&A_IM).map(str::to_owned),
        });
    }
}

#[tokio::test]
async fn read_feed_ranges_drains_25k_partition_container() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let observer = Arc::new(PartitionRangeObserver::default());
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(observer.clone()));
    let store = emulator.store();

    store.create_database("large-db");
    store.create_container_with_config(
        "large-db",
        "large-container",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2,
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(PARTITION_COUNT as u32)
            .with_partition_key_range_page_size(PKRANGE_PAGE_SIZE as u32)
            .build()
            .unwrap(),
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();
    let container = client
        .database_client("large-db")
        .container_client("large-container")
        .await
        .unwrap();

    let ranges = container.read_feed_ranges(None).await.unwrap();

    assert_eq!(ranges.len(), PARTITION_COUNT);
    let requests = observer.requests.lock().unwrap();
    assert_eq!(requests.len(), PARTITION_COUNT / PKRANGE_PAGE_SIZE + 1);
    assert!(requests[0].if_none_match.is_none());
    assert!(requests[1..]
        .iter()
        .all(|request| request.if_none_match.is_some()));
    assert!(requests
        .iter()
        .all(|request| request.a_im.as_deref() == Some("Incremental Feed")));
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn read_feed_ranges_preserves_partition_range_wire_errors() {
    use azure_core::http::StatusCode;
    use azure_data_cosmos::fault_injection::CustomResponseBuilder;
    use azure_data_cosmos_driver::models::ResponseBody;

    struct Case {
        name: &'static str,
        status: StatusCode,
        substatus: Option<u16>,
        activity_id: &'static str,
        body: &'static [u8],
    }

    let cases = [
        Case {
            name: "not-found",
            status: StatusCode::NotFound,
            substatus: None,
            activity_id: "pkrange-not-found",
            body: br#"{"code":"NotFound","message":"injected pkrange 404"}"#,
        },
        Case {
            name: "service-unavailable",
            status: StatusCode::ServiceUnavailable,
            substatus: Some(20003),
            activity_id: "pkrange-service-unavailable",
            body: br#"{"code":"ServiceUnavailable","message":"injected pkrange 503"}"#,
        },
    ];

    let harness = PartitionRangeFaultHarness::new().await;

    for case in cases {
        let mut response = CustomResponseBuilder::new(case.status)
            .with_header("x-ms-activity-id", case.activity_id)
            .with_body(case.body);
        if let Some(substatus) = case.substatus {
            response = response.with_sub_status(substatus);
        }
        let response = response.build();
        let rule =
            PartitionRangeFaultHarness::rule(format!("pkrange-{}", case.name), response, None);
        let container = harness.container(vec![Arc::clone(&rule)]).await;

        let error = container
            .read_feed_ranges(None)
            .await
            .expect_err("partition range service error must reach the public API");
        assert_eq!(error.status().status_code(), case.status, "{}", case.name);
        assert_eq!(
            error.status().sub_status().map(|status| status.value()),
            case.substatus,
            "{}",
            case.name
        );
        let response = error
            .response()
            .expect("the original wire response must remain available");
        assert_eq!(
            response
                .headers()
                .activity_id
                .as_ref()
                .map(|id| id.as_str()),
            Some(case.activity_id),
            "{}",
            case.name
        );
        match response.body() {
            ResponseBody::Bytes(body) => {
                assert_eq!(body.as_ref(), case.body, "{}", case.name)
            }
            body => panic!("{}: expected byte response body, got {body:?}", case.name),
        }
        assert!(
            rule.hit_count() > 0,
            "{}: pkrange fault must have been injected",
            case.name
        );
    }
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn read_feed_ranges_maps_malformed_success_to_serialization_error() {
    use azure_core::http::StatusCode;
    use azure_data_cosmos::fault_injection::CustomResponseBuilder;

    let response = CustomResponseBuilder::new(StatusCode::Ok)
        .with_body(br#"{"PartitionKeyRanges":"not-an-array"}"#)
        .build();
    let rule = PartitionRangeFaultHarness::rule("pkrange-malformed-success", response, None);
    let harness = PartitionRangeFaultHarness::new().await;
    let container = harness.container(vec![Arc::clone(&rule)]).await;

    let error = container
        .read_feed_ranges(None)
        .await
        .expect_err("malformed pkrange success must fail");
    assert_eq!(
        error.status(),
        azure_data_cosmos::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID
    );
    assert!(
        error.response().is_none(),
        "a client-side decode error must not masquerade as a wire error"
    );
    assert_eq!(rule.hit_count(), 1);
}

#[cfg(feature = "fault_injection")]
#[tokio::test]
async fn read_feed_ranges_maps_empty_success_to_existing_synthetic_error() {
    use azure_core::http::StatusCode;
    use azure_data_cosmos::fault_injection::CustomResponseBuilder;

    let empty_response = CustomResponseBuilder::new(StatusCode::Ok)
        .with_header("etag", "empty-etag")
        .with_body(br#"{"PartitionKeyRanges":[]}"#)
        .build();
    let empty_rule =
        PartitionRangeFaultHarness::rule("pkrange-empty-success", empty_response, Some(1));
    let not_modified_response = CustomResponseBuilder::new(StatusCode::NotModified)
        .with_header("etag", "empty-etag")
        .build();
    let not_modified_rule =
        PartitionRangeFaultHarness::rule("pkrange-empty-terminal", not_modified_response, None);

    let harness = PartitionRangeFaultHarness::new().await;
    let container = harness
        .container(vec![
            Arc::clone(&empty_rule),
            Arc::clone(&not_modified_rule),
        ])
        .await;

    let error = container
        .read_feed_ranges(None)
        .await
        .expect_err("an empty pkrange feed must remain a synthetic topology error");
    assert_eq!(
        error.status(),
        azure_data_cosmos::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID
    );
    assert_eq!(
        error.to_string(),
        format!(
            "{}: failed to resolve routing map for container",
            azure_data_cosmos::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID
        )
    );
    assert!(
        error.response().is_none(),
        "the empty-map invariant error must not masquerade as a wire error"
    );
    assert_eq!(empty_rule.hit_count(), 1);
    assert!(
        not_modified_rule.hit_count() >= 2,
        "the empty result must be drained, then retried once by the public API"
    );
}
