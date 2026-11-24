// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Checkpoints are bound to the feed scope they were issued for.

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use azure_core::http::{Request, Url};
use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::in_memory_emulator::{
    ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    effective_partition_key::EffectivePartitionKey, AccountReference, ContainerReference,
    CosmosOperation, FeedRange, ItemReference, MaxItemCountHint, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, PlanOptions};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

#[derive(Debug, Default)]
struct RequestCounter(AtomicUsize);

impl RequestObserver for RequestCounter {
    fn on_request(&self, _request: &Request) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

async fn setup() -> (Arc<CosmosDriver>, ContainerReference, Arc<RequestCounter>) {
    let counter = Arc::new(RequestCounter::default());
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap();
    let observer: Arc<dyn RequestObserver> = counter.clone();
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(observer));
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec![std::borrow::Cow::Borrowed("/pk")]),
        ContainerConfig::new()
            .with_partition_count(2)
            .build()
            .unwrap(),
    );
    let runtime = emulator.runtime_builder().build().await.unwrap();
    let account =
        AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .unwrap();
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    for i in 0..20 {
        let item = ItemReference::from_name(
            &container,
            PartitionKey::from(format!("pk{i}")),
            i.to_string(),
        );
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item).with_body(
                    serde_json::to_vec(
                        &serde_json::json!({ "id": i.to_string(), "pk": format!("pk{i}") }),
                    )
                    .unwrap(),
                ),
                OperationOptions::default(),
            )
            .await
            .unwrap();
    }
    (driver, container, counter)
}

fn range(min: &str, max: &str) -> FeedRange {
    FeedRange::new(
        EffectivePartitionKey::try_from(min).unwrap(),
        EffectivePartitionKey::try_from(max).unwrap(),
    )
    .unwrap()
}

fn query(container: &ContainerReference, scope: FeedRange) -> CosmosOperation {
    CosmosOperation::query_items(container.clone(), Some(scope))
        .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec())
        .with_max_item_count(MaxItemCountHint::Limit(
            std::num::NonZeroU32::new(1).unwrap(),
        ))
}

fn change_feed(container: &ContainerReference, scope: FeedRange) -> CosmosOperation {
    CosmosOperation::change_feed(container.clone(), Some(scope)).with_max_item_count(
        MaxItemCountHint::Limit(std::num::NonZeroU32::new(1).unwrap()),
    )
}

async fn assert_resume_is_scope_bound(
    build: fn(&ContainerReference, FeedRange) -> CosmosOperation,
) {
    let (driver, container, counter) = setup().await;
    let options = OperationOptions::default();
    let issued = range("", "80");
    let mut plan = Box::pin(driver.plan_operation(
        build(&container, issued.clone()),
        &options,
        None,
        &PlanOptions::default(),
    ))
    .await
    .unwrap();
    driver
        .execute_plan(&mut plan, Some(container.clone()), options.clone())
        .await
        .unwrap();
    let token = plan.to_continuation_token().unwrap();

    for other in [range("", "FF"), range("", "40"), range("40", "80")] {
        counter.0.store(0, Ordering::SeqCst);
        let error = Box::pin(driver.plan_operation(
            build(&container, other),
            &options,
            Some(&token),
            &PlanOptions::default(),
        ))
        .await
        .err()
        .expect("a different scope must be rejected");
        assert!(error.to_string().contains("feed scope"), "{error}");
        assert_eq!(
            counter.0.load(Ordering::SeqCst),
            0,
            "no request before rejection"
        );
    }

    let mut resumed = Box::pin(driver.plan_operation(
        build(&container, issued),
        &options,
        Some(&token),
        &PlanOptions::default(),
    ))
    .await
    .expect("the original scope resumes");
    driver
        .execute_plan(&mut resumed, Some(container), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn query_checkpoint_rejects_a_different_feed_scope() {
    assert_resume_is_scope_bound(query).await;
}

#[tokio::test]
async fn change_feed_checkpoint_rejects_a_different_feed_scope() {
    assert_resume_is_scope_bound(change_feed).await;
}
