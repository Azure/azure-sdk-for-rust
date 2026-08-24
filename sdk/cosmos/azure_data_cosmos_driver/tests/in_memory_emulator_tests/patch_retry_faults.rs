// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Retry behavior of the two patch execution paths under an ambiguous failure.
//!
//! The safety argument for running a patch server-side rests on one claim: when
//! a request fails in a way that leaves the outcome unknown, the driver sends it
//! again only if doing so cannot change the result. These tests exercise that
//! claim end to end rather than asserting it on a predicate.
//!
//! The fault is [`FaultInjectionErrorType::ResponseTimeout`], which the
//! framework injects with [`RequestSentStatus::Unknown`] — the request may
//! already have reached the backend. That is the exact condition
//! `CosmosOperation::allows_ambiguous_outcome_retry` governs.
//!
//! `rule.hit_count()` counts how many times a request reached the fault, so it
//! is the attempt count: `1` means the driver gave up immediately, `> 1` means
//! it resent.
//!
//! **Scope note.** Injection short-circuits above the emulator store, so the
//! mutation never lands and these tests cannot observe a literal double-apply
//! on the failing attempt. What they do cover is the decision the driver
//! actually owns — whether to resend — plus, on the recovery tests, that a
//! successful retry applies the operation exactly once.

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
    PatchInstructions, PatchOperation,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, PatchStrategy};
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
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");
    let item = ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
    let body = serde_json::json!({ "id": ITEM_ID, "pk": PK, "visits": 1, "name": "before" });
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
    strategy: PatchStrategy,
) -> Result<(), azure_data_cosmos_driver::error::CosmosError> {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let operation = CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&ops).unwrap());
    let mut options = OperationOptions::default();
    options.patch_strategy = Some(strategy);
    driver
        .execute_operation(operation, options)
        .await
        .map(|_| ())
}

async fn stored_visits(driver: &CosmosDriver, container: &ContainerReference) -> i64 {
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
    let doc: serde_json::Value = serde_json::from_slice(&bytes).expect("body is JSON");
    doc["visits"].as_i64().expect("visits is an integer")
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

// ── Server-side: the operation list decides whether a resend happens ──

/// The core safety property. An `increment` sent to the service and lost to an
/// ambiguous failure must not be resent — the first attempt may already have
/// applied it, and a second would double it.
#[tokio::test]
async fn server_side_unsafe_patch_is_not_retried_after_ambiguous_failure() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, increment(), PatchStrategy::ServerSide).await;

    assert!(
        outcome.is_err(),
        "the operation must surface the failure rather than silently retrying"
    );
    assert_eq!(
        rule.hit_count(),
        1,
        "an unsafe server-side patch must be attempted exactly once; \
         {} attempts means the driver resent a mutation whose outcome was unknown",
        rule.hit_count()
    );
}

/// The counterpart: the block is a property of the operations, not of
/// server-side patch as such. A `set` is safe to resend, so the driver still
/// spends its failover budget on it.
#[tokio::test]
async fn server_side_safe_patch_is_retried_after_ambiguous_failure() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, set_name(), PatchStrategy::ServerSide).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "a retry-safe server-side patch must still be retried, got {} attempt(s)",
        rule.hit_count()
    );
}

/// A transient blip should not be fatal: when only the first attempt fails, the
/// retry succeeds and the value is applied once, not twice.
#[tokio::test]
async fn server_side_safe_patch_recovers_when_only_the_first_attempt_fails() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, Some(1));
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    patch(&driver, &container, set_name(), PatchStrategy::ServerSide)
        .await
        .expect("the retry must recover from a single transient failure");

    assert_eq!(rule.hit_count(), 1, "only the first attempt was faulted");
    assert_eq!(
        stored_visits(&driver, &container).await,
        1,
        "a `set` retry must not disturb unrelated fields"
    );
}

// ── Client-side and Auto keep retrying unsafe operations ─────────────

/// The read-modify-write loop re-reads before each attempt, so an `increment`
/// is safe to retry there. This is the behavior that existed before
/// server-side patch and must be preserved.
#[tokio::test]
async fn client_side_unsafe_patch_is_still_retried_after_ambiguous_failure() {
    // The loop's mutation is an inner Replace, so that is what to fault.
    let rule = ambiguous_failure_rule("replace-timeout", FaultOperationType::ReplaceItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, increment(), PatchStrategy::ClientSide).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "the client-side loop must keep retrying an increment, got {} attempt(s)",
        rule.hit_count()
    );
}

/// End-to-end proof that the client-side path applies an increment exactly once
/// when it recovers — the duplicate this whole design is guarding against.
#[tokio::test]
async fn client_side_unsafe_patch_applies_once_when_it_recovers() {
    let rule = ambiguous_failure_rule("replace-timeout", FaultOperationType::ReplaceItem, Some(1));
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    patch(&driver, &container, increment(), PatchStrategy::ClientSide)
        .await
        .expect("the loop must recover from a single transient failure");

    assert_eq!(rule.hit_count(), 1, "only the first Replace was faulted");
    assert_eq!(
        stored_visits(&driver, &container).await,
        2,
        "the increment must land exactly once after the retry"
    );
}

/// `Auto` routes an increment to the client-side loop, so a fault armed on the
/// service's patch endpoint never fires at all — the request is not sent there.
/// This is what makes `Auto` safe by default without disabling retries.
#[tokio::test]
async fn auto_keeps_unsafe_patches_off_the_service_patch_path() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    patch(&driver, &container, increment(), PatchStrategy::Auto)
        .await
        .expect("Auto must fall back to the loop and succeed");

    assert_eq!(
        rule.hit_count(),
        0,
        "Auto must not send an unsafe patch to the service patch endpoint"
    );
    assert_eq!(
        stored_visits(&driver, &container).await,
        2,
        "the increment must land exactly once"
    );
}

/// `Auto` with safe operations does use the service, so the same fault fires
/// and — because the list is retry-safe — is retried.
#[tokio::test]
async fn auto_sends_safe_patches_to_the_service_and_retries_them() {
    let rule = ambiguous_failure_rule("patch-timeout", FaultOperationType::PatchItem, None);
    let driver = build_driver(vec![Arc::clone(&rule)]).await;
    let container = seed(&driver).await;

    let outcome = patch(&driver, &container, set_name(), PatchStrategy::Auto).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "a safe patch under Auto goes to the service and is retried, got {} attempt(s)",
        rule.hit_count()
    );
}

/// A failure that definitively never left the client is safe for every
/// operation type, so even an unsafe server-side patch is retried. This
/// separates "unsafe to resend" from "never retry", which are different
/// claims — only the ambiguous case is blocked.
#[tokio::test]
async fn server_side_unsafe_patch_is_retried_when_the_request_was_never_sent() {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::PatchItem)
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

    let outcome = patch(&driver, &container, increment(), PatchStrategy::ServerSide).await;

    assert!(outcome.is_err(), "every attempt was faulted, so it fails");
    assert!(
        rule.hit_count() > 1,
        "a definitively-unsent request is safe to retry for any operation, got {} attempt(s)",
        rule.hit_count()
    );
}
