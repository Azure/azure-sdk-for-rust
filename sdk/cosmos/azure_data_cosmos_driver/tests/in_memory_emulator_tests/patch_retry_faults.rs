// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Retry behavior of the two patch execution paths under an ambiguous failure.
//!
//! The safety argument for running a patch server-side rests on one claim: when
//! a request fails in a way that leaves the outcome unknown, the driver sends it
//! again only if doing so cannot change the result. These tests exercise that
//! claim end to end rather than asserting it on a predicate.
//!
//! Most resend-decision tests use [`FaultInjectionErrorType::ResponseTimeout`],
//! which fails before the request reaches the emulator but reports
//! [`RequestSentStatus::Unknown`]. The exactly-once test uses
//! [`FaultInjectionErrorType::ResponseTimeoutAfterService`] instead: the
//! emulator commits the matching write and then the fault client discards its
//! response.
//!
//! `rule.hit_count()` counts how many times a request reached the fault, so it
//! is the attempt count: `1` means the driver gave up immediately, `> 1` means
//! it resent.
//!
//! That post-service case observes the literal failure mode the tracking
//! protocol protects: a committed mutation whose response never reaches the
//! caller.

use std::sync::Arc;

use azure_core::http::Url;

use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, ItemReference, PartitionKey,
    PatchInstructions, PatchOperation, PatchTrackingId,
};
use azure_data_cosmos_driver::options::{
    DriverOptions, OperationOptions, OperationOptionsBuilder, PatchStrategy,
};
use azure_data_cosmos_driver::CosmosDriver;

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const PK: &str = "pk1";
const ITEM_ID: &str = "retry-item";

/// Builds a rule that fails every matching request with an ambiguous
/// post-send timeout, optionally only for the first `hit_limit` attempts.
fn ambiguous_failure_rule(
    id: &str,
    operation: FaultOperationType,
    hit_limit: Option<u32>,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeout)
        .with_probability(1.0)
        .build();
    let mut builder = FaultInjectionRuleBuilder::new(id, result).with_condition(condition);
    if let Some(limit) = hit_limit {
        builder = builder.with_hit_limit(limit);
    }
    Arc::new(builder.build())
}

fn post_service_timeout_rule(id: &str, operation: FaultOperationType) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ResponseTimeoutAfterService)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    )
}

async fn build_driver(rules: Vec<Arc<FaultInjectionRule>>) -> Arc<CosmosDriver> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
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
        .expect("runtime should build against the in-memory emulator");

    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .expect("driver should initialize")
}

async fn seed(driver: &CosmosDriver) -> ContainerReference {
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    let item = ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
    let body = serde_json::json!({
        "id": ITEM_ID,
        "pk": PK,
        "visits": 1,
        "name": "before",
        "tags": [],
        "a": { "b": 0 },
    });
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(body.to_string().into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("seeding must succeed before faults are armed");
    container
}

async fn patch(
    driver: &CosmosDriver,
    container: &ContainerReference,
    ops: PatchInstructions,
) -> Result<(), azure_data_cosmos_driver::error::CosmosError> {
    execute_patch(driver, container, ops, None)
        .await
        .map(|_| ())
}

async fn execute_patch(
    driver: &CosmosDriver,
    container: &ContainerReference,
    ops: PatchInstructions,
    tracking_id: Option<PatchTrackingId>,
) -> Result<
    azure_data_cosmos_driver::models::CosmosResponse,
    azure_data_cosmos_driver::error::CosmosError,
> {
    execute_patch_with_strategy(driver, container, ops, tracking_id, PatchStrategy::Auto).await
}

async fn execute_patch_with_strategy(
    driver: &CosmosDriver,
    container: &ContainerReference,
    ops: PatchInstructions,
    tracking_id: Option<PatchTrackingId>,
    strategy: PatchStrategy,
) -> Result<
    azure_data_cosmos_driver::models::CosmosResponse,
    azure_data_cosmos_driver::error::CosmosError,
> {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let mut operation =
        CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&ops).unwrap());
    if let Some(tracking_id) = tracking_id {
        operation = operation.with_patch_tracking_id(tracking_id);
    }
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(strategy)
        .build();
    let response = driver.execute_operation(operation, options).await?;
    Ok(response.expect("PATCH must return a singleton response"))
}

async fn stored_visits(driver: &CosmosDriver, container: &ContainerReference) -> i64 {
    stored_item(driver, container).await["visits"]
        .as_i64()
        .expect("visits is an integer")
}

async fn stored_item(driver: &CosmosDriver, container: &ContainerReference) -> serde_json::Value {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let response = driver
        .execute_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .expect("read back must succeed")
        .expect("read must return a response");
    let bytes = response.into_body().single().expect("point read body");
    super::parse_json_body(&bytes).expect("body is JSON")
}

fn increment() -> PatchInstructions {
    PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)])
}

fn set_name() -> PatchInstructions {
    PatchInstructions::from(vec![PatchOperation::set(
        "/name",
        serde_json::json!("after"),
    )])
}

fn overlapping_replace_then_set() -> PatchInstructions {
    PatchInstructions::from(vec![
        PatchOperation::replace("/a/b", serde_json::json!(1)),
        PatchOperation::set("/a", serde_json::json!({})),
    ])
}

#[tokio::test]
async fn unsafe_server_side_patch_is_not_retried_after_ambiguous_failure() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = execute_patch_with_strategy(
        &driver,
        &container,
        increment(),
        None,
        PatchStrategy::ServerSide,
    )
    .await;

    assert!(outcome.is_err());
    assert_eq!(
        rule.hit_count(),
        1,
        "unsafe server PATCH must not be resent"
    );
}

#[tokio::test]
async fn safe_server_side_patch_is_retried_after_ambiguous_failure() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = execute_patch_with_strategy(
        &driver,
        &container,
        set_name(),
        None,
        PatchStrategy::ServerSide,
    )
    .await;

    assert!(outcome.is_err(), "every attempt is faulted");
    assert!(rule.hit_count() > 1, "retry-safe server PATCH should retry");
}

#[tokio::test]
async fn unsafe_server_side_patch_commits_once_when_response_is_lost() {
    let rule = post_service_timeout_rule(
        "unsafe-patch-post-service-timeout",
        FaultOperationType::PatchItem,
    );
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = execute_patch_with_strategy(
        &driver,
        &container,
        increment(),
        None,
        PatchStrategy::ServerSide,
    )
    .await;

    assert!(
        outcome.is_err(),
        "an unsafe server PATCH must surface ambiguous committed response loss"
    );
    assert_eq!(
        rule.hit_count(),
        1,
        "unsafe server PATCH must not be resent"
    );
    assert_eq!(
        stored_visits(&driver, &container).await,
        2,
        "the committed increment must be applied exactly once"
    );
}

#[tokio::test]
async fn safe_server_side_patch_retries_after_committed_response_is_lost() {
    let rule = post_service_timeout_rule(
        "safe-patch-post-service-timeout",
        FaultOperationType::PatchItem,
    );
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let response = execute_patch_with_strategy(
        &driver,
        &container,
        set_name(),
        None,
        PatchStrategy::ServerSide,
    )
    .await
    .expect("retry-safe server PATCH must recover from committed response loss");

    assert_eq!(rule.hit_count(), 1, "only the first response is discarded");
    assert_eq!(
        response.diagnostics().request_count(),
        2,
        "safe server PATCH must include the faulted request and successful retry"
    );
    assert_eq!(stored_item(&driver, &container).await["name"], "after");
}

#[tokio::test]
async fn auto_keeps_unsafe_patch_off_the_server_endpoint() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    execute_patch_with_strategy(&driver, &container, increment(), None, PatchStrategy::Auto)
        .await
        .expect("Auto should use tracked RMW");

    assert_eq!(rule.hit_count(), 0);
    assert_eq!(stored_visits(&driver, &container).await, 2);
}

/// The read-modify-write loop re-reads before each attempt, so an `increment`
/// keeps retrying when every inner Replace fails before reaching the service.
#[tokio::test]
async fn unsafe_patch_is_retried_after_ambiguous_failure() {
    // The loop's mutation is an inner Replace, so that is what to fault.
    let rule = ambiguous_failure_rule("replace-timeout", FaultOperationType::ReplaceItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, increment()).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "the RMW loop must keep retrying an increment, got {} attempt(s)",
        rule.hit_count()
    );
}

/// End-to-end proof that the client-side path applies an increment exactly once
/// when it recovers — the duplicate this whole design is guarding against.
#[tokio::test]
async fn unsafe_patch_applies_once_when_it_recovers() {
    let rule = ambiguous_failure_rule("replace-timeout", FaultOperationType::ReplaceItem, Some(1));
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    patch(&driver, &container, increment())
        .await
        .expect("the loop must recover from a single transient failure");

    assert_eq!(rule.hit_count(), 1, "only the first Replace was faulted");
    assert_eq!(
        stored_visits(&driver, &container).await,
        2,
        "the increment must land exactly once after the retry"
    );
}

/// A Replace can commit even when its response is lost. The persisted marker
/// lets the RMW loop attribute the following 412 to its own prior commit, and
/// lets an application retry with the same token complete with only a Read.
#[tokio::test]
async fn unsafe_patch_commits_once_when_response_is_lost() {
    let rule = post_service_timeout_rule(
        "replace-post-service-timeout",
        FaultOperationType::ReplaceItem,
    );
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;
    let tracking_id = "7f5241c9-d7c2-4071-97a3-43bdebf6ef8f"
        .parse::<PatchTrackingId>()
        .unwrap();

    let recovered_response = execute_patch(&driver, &container, increment(), Some(tracking_id))
        .await
        .expect("the marker must prove that the timed-out Replace committed");

    assert_eq!(
        recovered_response.diagnostics().operation_name(),
        Some("patch_item")
    );
    let recovered_requests = recovered_response.diagnostics().requests();
    assert_eq!(
        recovered_requests
            .iter()
            .map(|request| request.operation_name())
            .collect::<Vec<_>>(),
        vec![
            Some("patch_read_item"),
            Some("patch_replace_item"),
            Some("patch_replace_item"),
            Some("patch_read_item"),
        ],
        "response-loss recovery must name both Replace attempts and the verification Read"
    );

    assert_eq!(rule.hit_count(), 1, "one committed response was discarded");
    let after_lost_response = stored_item(&driver, &container).await;
    assert_eq!(after_lost_response["visits"], 2);
    assert_eq!(
        after_lost_response["_azsdkPatchTracking"][0]["trackingId"],
        tracking_id.to_string()
    );
    let committed_etag = after_lost_response["_etag"].clone();

    let retry_response = execute_patch(&driver, &container, increment(), Some(tracking_id))
        .await
        .expect("reusing the tracking ID must recognize the committed operation");

    assert_eq!(
        retry_response.diagnostics().request_count(),
        1,
        "the application retry must perform only the verification Read"
    );
    assert_eq!(
        retry_response.diagnostics().operation_name(),
        Some("patch_item")
    );
    assert_eq!(
        retry_response.diagnostics().requests()[0].operation_name(),
        Some("patch_read_item"),
        "marker recognition is local, so the only helper span is the verification Read"
    );
    let after_application_retry = stored_item(&driver, &container).await;
    assert_eq!(after_application_retry["visits"], 2);
    assert_eq!(after_application_retry["_etag"], committed_etag);
}

#[tokio::test]
async fn overlapping_safe_ops_use_tracking_when_response_is_lost() {
    let rule = post_service_timeout_rule(
        "overlapping-ops-post-service-timeout",
        FaultOperationType::ReplaceItem,
    );
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    execute_patch(&driver, &container, overlapping_replace_then_set(), None)
        .await
        .expect("the marker must prevent replay after the committed response is lost");

    assert_eq!(rule.hit_count(), 1);
    let stored = stored_item(&driver, &container).await;
    assert_eq!(stored["a"], serde_json::json!({}));
    assert!(stored.get("_azsdkPatchTracking").is_some());
}

/// A failure that definitively never left the client is safe for every
/// operation type, so the inner Replace is retried.
#[tokio::test]
async fn unsafe_patch_is_retried_when_the_replace_was_never_sent() {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReplaceItem)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("patch-connect-fail", result)
            .with_condition(condition)
            .build(),
    );

    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, increment()).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "a definitively-unsent request is safe to retry for any operation, got {} attempt(s)",
        rule.hit_count()
    );
}
