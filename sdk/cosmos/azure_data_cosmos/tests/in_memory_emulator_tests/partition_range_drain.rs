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
