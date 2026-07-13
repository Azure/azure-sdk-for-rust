// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection functionality.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::diagnostics::TransportKind;
use azure_data_cosmos_driver::fault_injection::*;
use azure_data_cosmos_driver::options::{
    OperationOptions, OperationOptionsBuilder, Region, ThrottlingRetryOptionsBuilder,
};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Tests that a rule with probability 0.0 never injects faults.
///
/// A read operation should succeed because the fault never fires.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_probability_zero_never_fails() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(0.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("zero-probability", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "item1", "pk1", item_json).await?;

        // With probability 0.0, the read should succeed
        let read_response = context
            .read_item(&container, "item1", "pk1")
            .await
            .expect("Read should succeed with probability 0.0");

        // Verify the rule was never hit
        assert_eq!(rule.hit_count(), 0, "Rule should not have been hit");

        // Verify evaluations are in diagnostics
        let diagnostics = read_response.diagnostics();
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        // At least one request should have evaluations showing the probability miss
        let has_probability_miss = requests.iter().any(|r| {
            r.fault_injection_evaluations().iter().any(|e| {
                matches!(e, FaultInjectionEvaluation::ProbabilityMiss { rule_id, .. } if rule_id == "zero-probability")
            })
        });
        assert!(
            has_probability_miss,
            "Diagnostics should contain ProbabilityMiss evaluation for the zero-probability rule"
        );

        Ok(())
    })
    .await
}

/// Tests that a ServiceUnavailable fault with probability 1.0 causes read failures.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_service_unavailable_causes_failure() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("always-503", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        // With probability 1.0, the read should fail
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail with ServiceUnavailable injected"
        );

        let err_msg = read_result.unwrap_err().to_string();
        assert!(
            err_msg.contains("503")
                || err_msg.contains("Service Unavailable")
                || err_msg.contains("ServiceUnavailable"),
            "Error should indicate 503 Service Unavailable, got: {err_msg}"
        );

        // Verify the rule was hit
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Tests that fault injection respects the operation type filter.
///
/// A rule targeting only ReadItem should not affect CreateItem operations.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_operation_type_filter() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("read-only-fault", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    Box::pin(DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // CreateItem should succeed (rule only targets ReadItem)
        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        let create_response = context
            .create_item(&container, "item1", "pk1", item_json)
            .await
            .expect("CreateItem should succeed when rule targets ReadItem");

        // CreateItem should show OperationMismatch for the read-only rule
        let create_diagnostics = create_response.diagnostics();
        let create_requests = create_diagnostics.requests();

        let has_op_mismatch = create_requests.iter().any(|r| {
            r.fault_injection_evaluations().iter().any(|e| {
                matches!(e, FaultInjectionEvaluation::OperationMismatch { rule_id } if rule_id == "read-only-fault")
            })
        });
        assert!(
            has_op_mismatch,
            "CreateItem diagnostics should contain OperationMismatch evaluation"
        );

        // ReadItem should fail (matches the rule)
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "ReadItem should fail when targeted by fault injection"
        );

        Ok(())
    }))
    .await
}

/// Tests that fault injection stops after the hit limit is reached.
///
/// A rule with a hit limit should only inject faults up to that limit,
/// then allow operations to succeed normally.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_hit_limit_stops_after_n_faults() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hit-limit-test", result)
            .with_condition(condition)
            .with_hit_limit(2)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(
        rules,
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
            context.create_item(&container, "item1", "pk1", item_json).await?;

            // Execute reads to consume the hit limit.
            // Due to internal retries, the limit may be exhausted within fewer
            // top-level calls than the limit value.
            for _ in 0..5 {
                let result = context.read_item(&container, "item1", "pk1").await;
                if result.is_ok() {
                    // Hit limit exhausted — reads succeed now
                    break;
                }
            }

            // Verify the rule was hit exactly the limit number of times
            assert_eq!(
                rule.hit_count(),
                2,
                "Rule should have been hit exactly the hit limit"
            );

            // After hitting the limit, reads should succeed
            let final_response = context
                .read_item(&container, "item1", "pk1")
                .await
                .expect("Reads should succeed after hit limit is exhausted");

            // Verify diagnostics contain HitLimitExhausted evaluation
            let final_diagnostics = final_response.diagnostics();
            let final_requests = final_diagnostics.requests();

            let has_hit_limit = final_requests.iter().any(|r| {
                r.fault_injection_evaluations().iter().any(|e| {
                    matches!(e, FaultInjectionEvaluation::HitLimitExhausted { rule_id, .. } if rule_id == "hit-limit-test")
                })
            });
            assert!(
                has_hit_limit,
                "Diagnostics should contain HitLimitExhausted evaluation after limit reached"
            );

            Ok(())
        },
    )
    .await
}

/// Tests that a ConnectionError fault causes read failures.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_connection_error() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("connection-error", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        // With a connection error injected, the read should fail
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail with ConnectionError injected"
        );

        let err_msg = read_result.unwrap_err().to_string();
        assert!(
            err_msg.to_lowercase().contains("connection"),
            "Error should indicate a connection error, got: {err_msg}"
        );

        // Verify the rule was hit
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

// ----------------------------------------------------------------------------
// Gateway 2.0 fault injection coverage
// ----------------------------------------------------------------------------
//
// The following three tests lock in the retry/failover behavior the Gateway
// 2.0 transport must exhibit when the underlying Gateway 2.0 connection fails.
// Each test exercises a distinct failure shape:
//
//   - 503 Service Unavailable → regional failover
//   - 408 Request Timeout     → cross-region for reads / local-only for writes
//   - 404/1002 Read Session   → remote-preferred + no PKRange refresh
//
// Each rule is scoped to `TransportKind::GatewayV2` via
// `with_transport_kind(...)` so it only fires on Gateway 2.0 traffic. The
// emulator does not expose Gateway 2.0 endpoints, so these tests are gated
// behind the `gateway_v2` test category and rely on the
// `Session SingleRegion GatewayV2` CI matrix entry pointing at a
// pre-provisioned Gateway 2.0 account (see `sdk/cosmos/ci.yml` and the
// `AZURE_COSMOS_GW_V2_ENDPOINT` / `AZURE_COSMOS_GW_V2_KEY` plumbing in the
// driver test framework's `resolve_test_env`).

/// Gateway 2.0 503 Service Unavailable should trigger regional failover.
///
/// The rule is scoped to [`TransportKind::GatewayV2`] so it does not also
/// fire on standard-gateway requests issued during account discovery. The
/// emulator does not yet expose Gateway 2.0 endpoints, so this test is
/// gated behind the `gateway_v2` test category until CI gains a Gateway 2.0
/// account.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_service_unavailable_triggers_regional_failover(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-503-failover", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        // The read should fail (single region, fault always fires) but the
        // failover machinery must have been invoked. Once `RequestDiagnostics`
        // exposes per-attempt endpoint selection, assert that the diagnostics
        // record at least one regional failover attempt.
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail when 503 fires on every attempt"
        );

        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0 408 Request Timeout should retry across regions for reads,
/// but stay local-only for writes (single-region writes can't safely retry
/// across regions without risking duplicates).
///
/// The rule is scoped to [`TransportKind::GatewayV2`] so it does not affect
/// standard-gateway traffic. The emulator does not yet expose Gateway 2.0
/// endpoints, so this test is gated behind the `gateway_v2` test category.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_request_timeout_cross_region_for_reads() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-408-cross-region", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should ultimately fail when 408 fires on every attempt"
        );

        // TODO: once diagnostics expose retry attempts, assert that
        // a single-region account exhausts local-only retries while a
        // multi-region account performs at least one cross-region attempt.
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0 404/1002 ReadSessionNotAvailable must trigger a
/// remote-preferred retry path **without** invalidating the partition-key
/// range (PKRange) cache. The 404/1002 substatus indicates a session-token
/// mismatch, which is unrelated to the routing topology — refreshing PKRange
/// would be a wasted metadata round-trip.
///
/// The rule is scoped to [`TransportKind::GatewayV2`] so it does not also
/// fire on standard-gateway requests. The emulator does not yet expose
/// Gateway 2.0 endpoints, so this test is gated behind the `gateway_v2`
/// test category until CI gains a Gateway 2.0 account.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_read_session_not_available_remote_preferred() -> Result<(), Box<dyn Error>>
{
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ReadSessionNotAvailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-1002-remote-preferred", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail when 404/1002 fires on every attempt"
        );

        // TODO: once diagnostics record metadata-cache hits, assert
        // that the PKRange cache was NOT refreshed during these retries (a
        // 404/1002 is a session-token issue, not a routing-topology issue).
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0: unknown RNTBD response tokens must be silently skipped.
///
/// Forward-compat contract: when the proxy/backend adds a new RNTBD response
/// token that this client release does not recognize, the client MUST skip
/// it and continue parsing the rest of the frame — including recognized
/// tokens that appear AFTER the unknown one and the document body that
/// follows. The unit test
/// `unknown_token_id_is_silently_skipped` in
/// `src/driver/transport/rntbd/response.rs` covers the parser in isolation;
/// this test exercises the full end-to-end path:
///
///   driver -> HTTP/2 transport (fault client) -> synthetic RNTBD frame
///   -> `unwrap_response_for_gateway_v2` -> `CosmosResponse` returned to the caller
///
/// The frame is built as raw bytes so the test does NOT depend on the
/// `pub(crate)` codec types, and is shaped so the unknown token (id `0xFFFE`)
/// sits BETWEEN two recognized tokens. The post-unknown `RequestCharge`
/// (id `0x0015`, type `Double`) surfacing in `CosmosResponse::headers()`
/// proves the parser resumed correctly after the skip.
///
/// The Cosmos DB emulator does not support Gateway 2.0, so this test is
/// gated behind the `gateway_v2` test category and requires a real
/// Gateway 2.0-enabled account in CI; the SDK in-memory emulator does not
/// implement Gateway 2.0 either.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_unknown_rntbd_response_token_is_silently_skipped(
) -> Result<(), Box<dyn Error>> {
    const ITEM_JSON: &[u8] = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
    const EXPECTED_RU: f64 = 3.5;

    let synthetic_response = build_rntbd_response_with_unknown_token(EXPECTED_RU, ITEM_JSON);

    let custom = CustomResponseBuilder::new(azure_core::http::StatusCode::Ok)
        .with_body(synthetic_response)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_custom_response(custom)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-unknown-rntbd-token", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // No need to seed the item: the fault rule short-circuits the read
        // with our synthetic response on every G2 attempt.
        let response = context.read_item(&container, "item1", "pk1").await.expect(
            "read must succeed: unknown RNTBD response token must be skipped, \
                 and the recognized tokens around it must still parse",
        );

        // The body the SDK sees is the inner RNTBD body (post-unwrap), which
        // is exactly the JSON we packed.
        let body_bytes = match response.body() {
            azure_data_cosmos_driver::ResponseBody::Bytes(b) => b.as_ref(),
            other => panic!(
                "expected single-payload Bytes body, got {:?}: inner RNTBD body must survive the unknown-token skip intact",
                other
            ),
        };
        assert_eq!(
            body_bytes, ITEM_JSON,
            "inner RNTBD body must survive the unknown-token skip intact"
        );

        // The post-unknown-token RequestCharge surfacing here proves the
        // parser resumed correctly after skipping the unknown token: a parser
        // that stopped at the unknown token would NOT see the RequestCharge
        // that follows it on the wire.
        let request_charge =
            response.headers().request_charge.as_ref().expect(
                "RequestCharge token (placed AFTER the unknown token) must have been parsed",
            );
        assert!(
            (request_charge.value() - EXPECTED_RU).abs() < f64::EPSILON,
            "expected RequestCharge {EXPECTED_RU}, got {}",
            request_charge.value()
        );

        assert!(
            rule.hit_count() > 0,
            "fault rule must have fired on the read attempt"
        );

        Ok(())
    })
    .await
}

/// Builds a synthetic RNTBD response frame whose token stream contains an
/// unknown token ID (`0xFFFE`) sandwiched between two recognized tokens.
///
/// Wire layout (matches `RntbdResponse::deserialize` in
/// `src/driver/transport/rntbd/response.rs`):
///
/// ```text
///   u32 LE  total_len            (24 + metadata bytes; does NOT include body)
///   u16 LE  http status          (200 OK)
///   u16 LE  reserved             (0)
///   16 B    activity_id          (nil UUID; format-agnostic when all zeros)
///   token   unknown id 0xFFFE, type SmallString (0x07), value "future-feature"
///   token   id 0x0015, type Double (0x0E), value `request_charge`   (RequestCharge)
///   token   id 0x0000, type Byte   (0x00), value 1                  (PayloadPresent)
///   --- total_len boundary ---
///   u32 LE  body_len
///   bytes   document body
/// ```
///
/// Token IDs / type bytes are wire constants taken from
/// `src/driver/transport/rntbd/tokens.rs`. The intentional ordering
/// (unknown -> recognized) guarantees that a parser that stopped at the
/// unknown token would NOT surface `request_charge`, making the assertion
/// in the test load-bearing.
fn build_rntbd_response_with_unknown_token(request_charge: f64, body: &[u8]) -> Vec<u8> {
    const HTTP_STATUS_OK: u16 = 200;
    const TOKEN_TYPE_SMALL_STRING: u8 = 0x07;
    const TOKEN_TYPE_DOUBLE: u8 = 0x0E;
    const TOKEN_TYPE_BYTE: u8 = 0x00;
    const UNKNOWN_TOKEN_ID: u16 = 0xFFFE;
    const REQUEST_CHARGE_TOKEN_ID: u16 = 0x0015;
    const PAYLOAD_PRESENT_TOKEN_ID: u16 = 0x0000;

    let unknown_value: &[u8] = b"future-feature";
    assert!(
        unknown_value.len() <= u8::MAX as usize,
        "test fixture SmallString must fit in u8 length prefix"
    );

    let mut frame = Vec::with_capacity(64 + body.len());

    // Frame header.
    frame.extend_from_slice(&0_u32.to_le_bytes()); // total_len placeholder
    frame.extend_from_slice(&HTTP_STATUS_OK.to_le_bytes());
    frame.extend_from_slice(&0_u16.to_le_bytes()); // reserved / padding
    frame.extend_from_slice(&[0_u8; 16]); // activity_id = nil UUID

    // Unknown token first.
    frame.extend_from_slice(&UNKNOWN_TOKEN_ID.to_le_bytes());
    frame.push(TOKEN_TYPE_SMALL_STRING);
    frame.push(unknown_value.len() as u8);
    frame.extend_from_slice(unknown_value);

    // Recognized RequestCharge token AFTER the unknown one.
    frame.extend_from_slice(&REQUEST_CHARGE_TOKEN_ID.to_le_bytes());
    frame.push(TOKEN_TYPE_DOUBLE);
    frame.extend_from_slice(&request_charge.to_le_bytes());

    // PayloadPresent token signals a body follows after the metadata section.
    frame.extend_from_slice(&PAYLOAD_PRESENT_TOKEN_ID.to_le_bytes());
    frame.push(TOKEN_TYPE_BYTE);
    frame.push(if body.is_empty() { 0 } else { 1 });

    // Patch total_len to mark the end of the metadata section (before the body).
    let total_len = u32::try_from(frame.len()).expect("synthetic frame fits in u32");
    frame[0..4].copy_from_slice(&total_len.to_le_bytes());

    if !body.is_empty() {
        frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
        frame.extend_from_slice(body);
    }
    frame
}

/// Gateway 2.0 server-response-delay fault injection.
///
/// A rule scoped to [`TransportKind::GatewayV2`] with `with_delay(...)` and
/// no error type should cause every matched G2 request to take at least the
/// configured delay before returning. This test asserts the rule fires and
/// the read still completes — the latency assertion is loose (only confirms
/// the delay was on the order of magnitude requested) because CI clocks
/// are noisy.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_server_response_delay_is_injected() -> Result<(), Box<dyn Error>> {
    const INJECTED_DELAY: Duration = Duration::from_millis(500);

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_delay(INJECTED_DELAY)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-response-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        let start = Instant::now();
        let read_result = context.read_item(&container, "item1", "pk1").await;
        let elapsed = start.elapsed();

        // The injected delay must have been applied (even if the read
        // ultimately succeeds or fails, the rule must have fired and the
        // operation must have waited at least the configured delay before
        // making forward progress). We assert at least 80% of the configured
        // delay to tolerate clock-resolution slack on slow CI agents.
        let min_expected = INJECTED_DELAY * 4 / 5;
        assert!(
            elapsed >= min_expected,
            "read should have waited at least {min_expected:?} for the injected delay, got {elapsed:?}; \
             result was {read_result:?}"
        );

        assert!(rule.hit_count() > 0, "delay rule must have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0 hit-limit caps fault firing.
///
/// A rule with `with_hit_limit(N)` scoped to [`TransportKind::GatewayV2`]
/// fires for the first N matching G2 attempts and then stops. The test
/// drives more than N reads and asserts the recorded hit count equals N
/// (never exceeds the limit).
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_hit_limit_caps_fault_count() -> Result<(), Box<dyn Error>> {
    const HIT_LIMIT: u32 = 2;
    const TOTAL_READS: u32 = 5;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-hit-limit", result)
            .with_condition(condition)
            .with_hit_limit(HIT_LIMIT)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        // Drive more reads than HIT_LIMIT. Reads may succeed (after the
        // rule stops firing) or fail (while the rule is active); we only
        // care about the hit count being capped.
        for _ in 0..TOTAL_READS {
            let _ = context.read_item(&container, "item1", "pk1").await;
        }

        assert_eq!(
            rule.hit_count(),
            HIT_LIMIT,
            "rule must fire exactly hit_limit ({HIT_LIMIT}) times across {TOTAL_READS} reads"
        );

        Ok(())
    })
    .await
}

// ── 449 RetryWith policy — live-account fault-injection coverage ──────────────
//
// 449 RetryWith is the Cosmos backend's signal for transient concurrency
// conflicts that the client must retry in the same region. The driver
// implements the policy in `try_handle_retry_with` (in
// `driver::pipeline::retry_evaluation`): 10ms initial delay + a small random
// salt, exponential backoff up to ~1s per retry, ~30s cumulative cap.
//
// These tests use the same fault-injection harness as the other live-account
// tests in this file; the `RetryWith` variant on `FaultInjectionErrorType`
// makes the backend response indistinguishable from a real 449 on the wire.

/// Verifies that the driver transparently retries 449 RetryWith and the
/// caller sees success once the rule's hit-limit is exhausted.
///
/// Hit-limit = 3 → the rule fires 3 times, the driver retries 3 times
/// (well within its ~30s cumulative-wait budget), the 4th attempt finds the
/// rule disabled and succeeds. The caller's `read_item` returns `Ok` even
/// though three 449s were injected mid-operation. The rule's `hit_count` is
/// asserted to land exactly at the limit, proving the retries happened.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn fault_injection_449_retry_with_succeeds_after_hit_limit() -> Result<(), Box<dyn Error>>
{
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::RetryWith)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("449-retry-with-succeeds", result)
            .with_condition(condition)
            .with_hit_limit(3)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async move |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "retry-with"}"#;
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

        // The driver retries 449 in-region with exponential backoff. With
        // hit_limit=3 the rule fires three times — well within the ~30s
        // cumulative budget — and the next read succeeds because the rule
        // is disabled.
        let response = context
            .read_item(&container, "item1", "pk1")
            .await
            .expect("449 RetryWith with hit_limit=3 must be retried until the rule is disabled");

        assert_eq!(
            rule.hit_count(),
            3,
            "driver must retry 449 exactly hit_limit times",
        );

        // Diagnostics should record the rule firing on the early attempts.
        let diagnostics = response.diagnostics();
        let saw_fault = diagnostics.requests().iter().any(|r| {
            r.fault_injection_evaluations().iter().any(|e| {
                matches!(
                    e,
                    FaultInjectionEvaluation::Applied { rule_id, .. } if rule_id == "449-retry-with-succeeds"
                )
            })
        });
        assert!(
            saw_fault,
            "diagnostics must record at least one Applied fault for the 449 rule",
        );

        Ok(())
    })
    .await
}

/// Verifies that 449 RetryWith retries are scoped to `TransportKind::GatewayV2`
/// when the rule is so-scoped — i.e. the same policy works on the Gateway 2.0
/// transport, not just on the standard gateway.
///
/// The fault-injection condition adds `with_transport_kind(TransportKind::GatewayV2)`,
/// so the rule only fires on Gateway 2.0 traffic. With Gateway 2.0 enabled by
/// default for eligible data-plane reads, a `read_item` against a Gateway 2.0
/// account fires the rule for each in-region retry until the hit-limit is
/// exhausted, after which the read succeeds.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_449_retry_with_succeeds_after_hit_limit() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::RetryWith)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway_v2-449-retry-with-succeeds", result)
            .with_condition(condition)
            .with_hit_limit(3)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(
        rules,
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_json = br#"{"id": "item1", "pk": "pk1", "value": "gw_v2-retry-with"}"#;
            context
                .create_item(&container, "item1", "pk1", item_json)
                .await?;

            // The driver retries 449 in-region with exponential backoff. With
            // hit_limit=3 the rule fires three times — well within the ~30s
            // cumulative budget — and the next read succeeds because the rule
            // is disabled.
            let start = Instant::now();
            let response = context.read_item(&container, "item1", "pk1").await.expect(
                "Gateway 2.0 449 with hit_limit=3 must be retried until the rule is disabled",
            );
            let elapsed = start.elapsed();

            assert_eq!(
                rule.hit_count(),
                3,
                "driver must retry 449 exactly hit_limit times on Gateway 2.0",
            );
            // Three retries with base delays ≈ 10 + 20 + 40 ms (+ salt) should
            // each take well under a second; the total should be far below the
            // 30s cumulative cap.
            assert!(
                elapsed < Duration::from_secs(10),
                "Gateway 2.0 449 retry path should be quick (took {elapsed:?})",
            );

            let diagnostics = response.diagnostics();
            let saw_fault = diagnostics.requests().iter().any(|r| {
                r.fault_injection_evaluations().iter().any(|e| {
                    matches!(
                        e,
                        FaultInjectionEvaluation::Applied { rule_id, .. }
                            if rule_id == "gateway_v2-449-retry-with-succeeds"
                    )
                })
            });
            assert!(
                saw_fault,
                "diagnostics must record at least one Applied fault for the Gateway 2.0 449 rule",
            );

            Ok(())
        },
    )
    .await
}
/// End-to-end validation that the configurable 429 (throttle) retry budget —
/// [`ThrottlingRetryOptions::max_retry_count`](azure_data_cosmos_driver::options::ThrottlingRetryOptions::max_retry_count),
/// exposed on the nested `OperationOptions::throttling_retry_options` group —
/// is honored across the full driver stack against a live emulator account.
///
/// A fault rule injects an HTTP 429 (`TooManyRequests`) on **every**
/// `ReadItem` transport attempt with probability `1.0`. Because the injected
/// 429 carries no sub-status it is a *generic* throttle: the operation
/// pipeline classifies it as region-confirming (no cross-region failover), so
/// the entire retry budget is spent inside a single transport-pipeline
/// invocation. Counting `rule.hit_count()` therefore yields the exact number
/// of read attempts that reached the (faulted) transport client.
///
/// Wire-attempt accounting for `max_retry_count = N`:
///
/// * **Total = `N + 1`** attempts on the wire (1 initial + N retries) for
///   any N, including `N == 0`. The one-shot forced-final-retry safety net
///   in `execute_transport_pipeline` is gated on `attempt_count <
///   max_attempts`, so once the count budget is exhausted the safety net is
///   suppressed too — matching the .NET-parity
///   `MaxRetryAttemptsOnRateLimitedRequests` semantic.
/// * The forced-final retry still fires when the *cumulative-wait* budget
///   (rather than the count) is the limiter; this test uses a generous
///   300-second wait budget so the count is always the sole limiter.
///
/// This mirrors the unit-level coverage in
/// `transport_pipeline::tests::execute_transport_pipeline_honors_configured_max_throttle_attempts`
/// but exercises the real option-resolution → fault-injection → transport
/// retry path instead of driving the loop directly.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_429_honors_configurable_throttle_retry_count(
) -> Result<(), Box<dyn Error>> {
    // (configured throttle-retry budget, expected total ReadItem attempts).
    // Total = 1 initial + N retries (the forced-final retry is suppressed
    // once the count budget is exhausted; it only fires when the
    // cumulative-wait budget is the limiter).
    for (max_throttle_retry_count, expected_hits) in [(0_u32, 1_u32), (1, 2), (3, 4), (5, 6)] {
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "always-429",
                FaultInjectionResultBuilder::new()
                    .with_error(FaultInjectionErrorType::TooManyRequests)
                    .with_probability(1.0)
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReadItem)
                    .build(),
            )
            .build(),
        );

        // Pin the throttle-retry budget at the runtime layer of the option
        // view. A generous cumulative-wait budget keeps the attempt count the
        // sole limiter for these small retry counts. No end-to-end latency
        // policy is set, so the transport request carries no deadline and the
        // forced-final retry is immediate.
        let operation_options = OperationOptionsBuilder::new()
            .with_throttling_retry_options(
                ThrottlingRetryOptionsBuilder::new()
                    .with_max_retry_count(max_throttle_retry_count)
                    .with_max_retry_wait_time(Duration::from_secs(300))
                    .build(),
            )
            .build();

        let rule_for_assert = Arc::clone(&rule);
        Box::pin(
            DriverTestClient::run_with_unique_db_and_fault_injection_options(
                vec![rule],
                operation_options,
                async move |context, database| {
                    let container_name = context.unique_container_name();
                    let container = context
                        .create_container(&database, &container_name, "/pk")
                        .await?;

                    // Seed the item with a write. The fault rule targets only
                    // ReadItem, so the seeding write is unaffected.
                    let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
                    context
                        .create_item(&container, "item1", "pk1", item_json)
                        .await?;

                    // The read always observes 429 and ultimately fails once
                    // the throttle budget is exhausted.
                    let read_result = context.read_item(&container, "item1", "pk1").await;
                    assert!(
                        read_result.is_err(),
                        "read must fail once the throttle budget is exhausted \
                         (max_throttle_retry_count={max_throttle_retry_count})",
                    );

                    assert_eq!(
                        rule_for_assert.hit_count(),
                        expected_hits,
                        "max_throttle_retry_count={max_throttle_retry_count} must yield \
                         {expected_hits} ReadItem attempts on the wire, but the 429 fault \
                         rule fired {} time(s)",
                        rule_for_assert.hit_count(),
                    );

                    Ok(())
                },
            ),
        )
        .await?;
    }

    Ok(())
}

/// Verifies that a transient failure on a force-refresh of the partition-key
/// range cache does NOT regress the cached routing map to empty.
///
/// Scenario:
/// 1. Install (but disable) a one-shot `ServiceUnavailable` fault on
///    `MetadataPartitionKeyRanges`.
/// 2. Warm the routing-map cache successfully (fault disabled).
/// 3. Enable the fault, then force-refresh the cache. The fetch fails.
/// 4. Assert the post-refresh routing map is still populated, proving the
///    cache kept the previously cached map rather than replacing it with an
///    empty placeholder that would break routing until the next explicit
///    invalidation.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn pkrange_refresh_transient_failure_preserves_cached_routing_map(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("pkrange-refresh-transient", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    // Start disabled so the warmup below isn't intercepted; we enable the
    // rule immediately before force-refreshing so the failure is guaranteed
    // to land on the refresh path under test.
    rule.disable();
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Warmup: fault is disabled, this populates the cache with real ranges.
        let warmed = context
            .resolve_all_partition_key_ranges(&container, false)
            .await?;
        assert!(
            warmed.is_some_and(|r| !r.is_empty()),
            "warmup resolve must populate the routing map"
        );
        assert_eq!(
            rule.hit_count(),
            0,
            "warmup must not have triggered the disabled fault"
        );

        // Arm the fault and force-refresh. With the fix in place, the refresh
        // sees the transient failure but preserves the previously cached map.
        rule.enable();
        let refreshed = context
            .resolve_all_partition_key_ranges(&container, true)
            .await?;

        assert!(
            refreshed.is_some(),
            "force-refresh on transient failure must not return None"
        );
        let ranges = refreshed.unwrap();
        assert!(
            !ranges.is_empty(),
            "force-refresh on transient failure must preserve the previously cached \
             routing map -- empty ranges indicate the cache regressed to empty"
        );

        assert_eq!(
            rule.hit_count(),
            1,
            "force-refresh must have triggered the fault exactly once"
        );

        // A subsequent non-refresh lookup must still see the populated cache.
        let after = context
            .resolve_all_partition_key_ranges(&container, false)
            .await?;
        assert!(
            after.is_some_and(|r| !r.is_empty()),
            "subsequent non-refresh lookup must observe the preserved routing map"
        );

        Ok(())
    })
    .await
}
