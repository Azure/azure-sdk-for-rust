// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use azure_core::http::StatusCode;

use super::*;
use crate::{
    diagnostics::RequestSentStatus,
    error::SubStatusCode,
    models::{
        AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
        CosmosResponseHeaders, CosmosStatus, DatabaseReference, ItemReference, PartitionKey,
        PartitionKeyDefinition, SystemProperties,
    },
    pipeline::operation::{
        recovery,
        recovery::{
            error::MAX_SERVICE_DETAIL_LEN,
            state::{
                RetryWithRetryState, BACKEND_FAILOVER_JITTER_RATIO, BACKEND_FAILOVER_MAX_BACKOFF,
                BACKEND_FAILOVER_MAX_TOTAL_DELAY,
            },
        },
        routing,
        routing::{LocationEffect, UnavailablePartition, UnavailableReason},
    },
};

/// Lower/upper bounds of the jittered *second* backend-failover delay. The
/// first retry is immediate, so the 1s exponential base applies from the
/// second retry onward (base 1s ±25%).
fn second_backend_failover_delay_bounds() -> (Duration, Duration) {
    let base = Duration::from_millis(1000);
    let lo = base.mul_f64(1.0 - BACKEND_FAILOVER_JITTER_RATIO);
    let hi = base.mul_f64(1.0 + BACKEND_FAILOVER_JITTER_RATIO);
    (lo, hi)
}

/// Asserts a delay is a plausible jittered backend-failover backoff: never
/// above the per-retry cap plus jitter. The first retry is legitimately
/// zero, so only the upper bound is enforced.
fn assert_within_backoff_cap(delay: Duration) {
    let max = BACKEND_FAILOVER_MAX_BACKOFF.mul_f64(1.0 + BACKEND_FAILOVER_JITTER_RATIO);
    assert!(
        delay <= max,
        "backoff delay {delay:?} must stay within capped+jitter {max:?}"
    );
}

#[cfg(feature = "unstable_dtx")]
use crate::pipeline::operation::recovery::state::{
    MAX_DTX_COORDINATOR_RETRIES, MAX_DTX_INFRA_RETRIES,
};

fn make_create_item_operation() -> CosmosOperation {
    let account = AccountReference::with_account_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );
    let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
    let props = ContainerProperties {
        id: "testcontainer".into(),
        partition_key: pk_def,
        system_properties: SystemProperties::default(),
    };
    let container = ContainerReference::new(
        account,
        "testdb",
        "testdb_rid",
        "testcontainer",
        "testcontainer_rid",
        &props,
    );
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
    CosmosOperation::create_item(item).with_body(b"{}".to_vec())
}

fn make_read_operation() -> CosmosOperation {
    let account = AccountReference::with_account_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==", // base64 "test"
    );
    let db_ref = DatabaseReference::from_name(account, "testdb".to_owned());
    CosmosOperation::read_database(db_ref)
}

fn make_create_operation() -> CosmosOperation {
    // create_database uses OperationType::Create which is NOT idempotent
    let account = AccountReference::with_account_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );
    CosmosOperation::create_database(account)
}

#[cfg(feature = "unstable_dtx")]
fn make_dtx_operation() -> CosmosOperation {
    make_dtx_operation_for(crate::models::DistributedTransactionType::Write)
}

#[cfg(feature = "unstable_dtx")]
fn make_dtx_operation_for(
    transaction_type: crate::models::DistributedTransactionType,
) -> CosmosOperation {
    let account = AccountReference::with_account_key(
        url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        "dGVzdA==",
    );
    CosmosOperation::distributed_transaction(account, transaction_type)
}

fn make_success_result() -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::Success {
            status: CosmosStatus::new(StatusCode::Ok),
            cosmos_headers: CosmosResponseHeaders::default(),
            body: b"{}".to_vec(),
        },
    }
}

fn make_transport_error(sent: RequestSentStatus) -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::TransportError {
            status: crate::error::status_codes::TRANSPORT_GENERATED_503,
            error: crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::TRANSPORT_GENERATED_503)
                .with_message("connection refused")
                .build(),
            request_sent: sent,
        },
    }
}

fn make_http_error(status_code: StatusCode) -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::HttpError {
            status: CosmosStatus::new(status_code),
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    }
}

fn make_http_error_status(status: CosmosStatus) -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::HttpError {
            status,
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    }
}

#[test]
fn retry_families_preserve_consumed_recreation_budget() {
    let operation = make_read_operation();
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com/").unwrap());
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.container_recreation_retry_attempted = true;

    let responses = [
        make_transport_error(RequestSentStatus::NotSent),
        make_http_error(StatusCode::ServiceUnavailable),
        make_read_session_not_available_result(),
        make_449_error(),
    ];
    for result in responses {
        let (action, _) = recovery::classify_result(&operation, &endpoint, result, &state);
        let next = match action {
            OperationAction::FailoverRetry { new_state, .. }
            | OperationAction::SessionRetry { new_state }
            | OperationAction::InRegionRetry { new_state, .. } => new_state,
            _ => panic!("expected retry proposal"),
        };
        assert!(next.container_recreation_retry_attempted);
        assert!(!recreation::container_recreation_refresh_eligible(
            &operation,
            &crate::pipeline::operation::types::OperationOverrides::default(),
            None,
            next.container_recreation_retry_attempted,
        ));
    }
}

#[cfg(feature = "unstable_dtx")]
fn make_dtx_http_error(
    status: CosmosStatus,
    body: Vec<u8>,
    retry_after_ms: Option<u64>,
) -> TransportResult {
    let cosmos_headers = CosmosResponseHeaders {
        retry_after_ms,
        ..Default::default()
    };
    TransportResult {
        outcome: TransportOutcome::HttpError {
            status,
            cosmos_headers,
            body,
            request_sent: RequestSentStatus::Sent,
        },
    }
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_449_5352_uses_coordinator_retry_budget() {
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(
            StatusCode::from(449_u16),
            Some(crate::error::status_codes::substatus::DTC_COORDINATOR_RACE_CONFLICT),
        ),
        Vec::new(),
        Some(250),
    );
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(effects.is_empty());
    match action {
        OperationAction::DtxRetry { new_state, delay } => {
            assert_eq!(new_state.dtx_coordinator_retry_count, 1);
            assert_eq!(new_state.dtx_infra_retry_count, 0);
            assert_eq!(delay, std::time::Duration::from_millis(250));
        }
        other => panic!("expected DtxRetry, got {other:?}"),
    }
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_500_5411_uses_infra_retry_budget() {
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(
            StatusCode::InternalServerError,
            Some(crate::error::status_codes::substatus::DTC_LEDGER_FAILURE),
        ),
        Vec::new(),
        None,
    );
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(effects.is_empty());
    match action {
        OperationAction::DtxRetry { new_state, delay } => {
            assert_eq!(new_state.dtx_coordinator_retry_count, 0);
            assert_eq!(new_state.dtx_infra_retry_count, 1);
            assert_eq!(delay, std::time::Duration::from_millis(100));
        }
        other => panic!("expected DtxRetry, got {other:?}"),
    }
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_retry_budget_exhaustion_completes_response() {
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(
            StatusCode::from(449_u16),
            Some(crate::error::status_codes::substatus::DTC_COORDINATOR_RACE_CONFLICT),
        ),
        Vec::new(),
        None,
    );
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.dtx_coordinator_retry_count = MAX_DTX_COORDINATOR_RETRIES;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(effects.is_empty());
    assert!(matches!(action, OperationAction::Complete(_)));
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_infra_retry_budget_exhaustion_completes_response() {
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(
            StatusCode::InternalServerError,
            Some(crate::error::status_codes::substatus::DTC_LEDGER_FAILURE),
        ),
        Vec::new(),
        None,
    );
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.dtx_infra_retry_count = MAX_DTX_INFRA_RETRIES;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(effects.is_empty());
    assert!(matches!(action, OperationAction::Complete(_)));
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_body_bearing_449_completes_for_outer_loop() {
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(
            StatusCode::from(449_u16),
            Some(crate::error::status_codes::substatus::UNKNOWN),
        ),
        br#"{"isRetriable":true}"#.to_vec(),
        None,
    );
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(effects.is_empty());
    assert!(matches!(action, OperationAction::Complete(_)));
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_write_forbidden_refreshes_topology_before_dtx_classification() {
    for transaction_type in [
        crate::models::DistributedTransactionType::Write,
        crate::models::DistributedTransactionType::Read,
    ] {
        let op = make_dtx_operation_for(transaction_type);
        let result = make_dtx_http_error(
            CosmosStatus::from_parts(StatusCode::Forbidden, Some(SubStatusCode::new(3))),
            Vec::new(),
            None,
        );
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );

        let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "{transaction_type:?} DTX 403/3 should failover-retry"
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, LocationEffect::RefreshAccountProperties)),
            "{transaction_type:?} DTX 403/3 should refresh topology"
        );
        assert!(
            effects
                .iter()
                .any(|effect| matches!(effect, LocationEffect::MarkPartitionUnavailable(_))),
            "{transaction_type:?} DTX 403/3 should mark partition unavailable"
        );
    }
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_bodyless_database_account_not_found_refreshes_topology_before_dtx_classification() {
    // 403/1008 DatabaseAccountNotFound applies to every op type, including
    // DTX writes (PR #4590): it must refresh account properties and fail
    // over rather than being swallowed by the DTX classification into a
    // stale-topology `Complete`.
    let op = make_dtx_operation();
    let result = make_dtx_http_error(
        CosmosStatus::from_parts(StatusCode::Forbidden, Some(SubStatusCode::new(1008))),
        Vec::new(),
        None,
    );
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|effect| matches!(effect, LocationEffect::RefreshAccountProperties)));
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn dtx_http_outcomes_never_failover_session_or_hedge() {
    // DTX is a write-bearing resource routed through the shared pipeline.
    // The DTX classification in `evaluate_http_outcome` keeps DTX responses
    // out of the cross-region failover / session-retry / hedging machinery,
    // which is unsafe for writes (see PR #4432). This pins that guard except
    // for 403/3 WriteForbidden and 403/1008 DatabaseAccountNotFound, which
    // must refresh topology first (both are handled before the DTX
    // short-circuit). By the time a DTX `429` reaches this classifier, the
    // transport-level throttle retry path has already propagated it (for
    // example, after exhausting throttle budget). Every other coordinator HTTP
    // outcome must resolve to `DtxRetry` (bodyless coordinator/infra retry) or
    // `Complete` (body handed to the outer coordinator loop) — never
    // `FailoverRetry`, `SessionRetry`, or `Hedge`, and never a location effect.
    let op = make_dtx_operation();
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    // Statuses/sub-statuses that drive failover / session-retry / abort for a
    // normal (non-DTX) operation.
    let statuses = [
        CosmosStatus::new(StatusCode::from(503_u16)),
        CosmosStatus::from_parts(StatusCode::from(429_u16), Some(SubStatusCode::new(3092))),
        CosmosStatus::from_parts(StatusCode::from(404_u16), Some(SubStatusCode::new(1002))),
        CosmosStatus::from_parts(
            StatusCode::from(449_u16),
            Some(crate::error::status_codes::substatus::UNKNOWN),
        ),
        CosmosStatus::new(StatusCode::from(408_u16)),
        CosmosStatus::new(StatusCode::from(410_u16)),
        CosmosStatus::new(StatusCode::from(500_u16)),
    ];

    for status in statuses {
        let code = u16::from(status.status_code());
        for body in [Vec::new(), b"{}".to_vec()] {
            let body_len = body.len();
            let result = make_dtx_http_error(status, body, None);
            let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
            assert!(
                effects.is_empty(),
                "DTX status {code} (body_len={body_len}) emitted location effects",
            );
            assert!(
                matches!(
                    action,
                    OperationAction::DtxRetry { .. } | OperationAction::Complete(_)
                ),
                "DTX status {code} (body_len={body_len}) produced {action:?}",
            );
        }
    }
}

#[test]
fn success_completes() {
    let op = make_read_operation();
    let result = make_success_result();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::Complete(_)));
    assert!(effects.is_empty());
}

#[test]
fn transport_error_not_sent_retries() {
    let op = make_create_operation();
    let result = make_transport_error(RequestSentStatus::NotSent);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    // Not-sent marks the endpoint only; Gateway connect failures are not partition-specific.
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

#[test]
fn transport_error_sent_idempotent_retries() {
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::Sent);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    // Sent → endpoint is reachable, only mark partition (not endpoint).
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

/// Locks in the driver's deliberate divergence from the other Cosmos SDKs
/// (see Spec 0006: Error codes and retries): a non-idempotent write is retried
/// even when the request may already have been sent. Python/Java/.NET abort
/// here.
#[test]
fn transport_error_sent_non_idempotent_retries() {
    let op = make_create_operation();
    let result = make_transport_error(RequestSentStatus::Sent);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    // Sent → endpoint is reachable, only mark partition (not endpoint).
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

/// The stored-procedure carve-out must not leak onto ordinary writes: a
/// non-idempotent `Create` still retries on the same ambiguous statuses.
#[test]
fn non_idempotent_write_retries_on_timeout_and_server_errors() {
    let op = make_create_operation();
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    for status_code in [
        StatusCode::RequestTimeout,
        StatusCode::InternalServerError,
        StatusCode::BadGateway,
        StatusCode::GatewayTimeout,
        StatusCode::ServiceUnavailable,
    ] {
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let (action, _) =
            recovery::classify_result(&op, &endpoint, make_http_error(status_code), &state);
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "{status_code:?} must still fail over for a non-idempotent write, got {action:?}"
        );
    }
}

mod server_side_patch_retries {
    use super::*;

    fn make_patch_operation(retry_safe: Option<bool>) -> CosmosOperation {
        let account = AccountReference::with_account_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "testcontainer".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &props,
        );
        let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::patch_item(item).with_body(
            br#"{"operations":[{"op":"increment","path":"/visits","value":1}]}"#.to_vec(),
        );
        match retry_safe {
            Some(retry_safe) => operation.with_patch_retry_safe(retry_safe),
            None => operation,
        }
    }

    fn evaluate(op: &CosmosOperation, result: TransportResult) -> OperationAction {
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        recovery::classify_result(op, &endpoint, result, &state).0
    }

    #[test]
    fn unsafe_and_unresolved_patch_abort_after_possible_execution() {
        for retry_safe in [None, Some(false)] {
            for request_sent in [RequestSentStatus::Sent, RequestSentStatus::Unknown] {
                let action = evaluate(
                    &make_patch_operation(retry_safe),
                    make_transport_error(request_sent),
                );
                assert!(
                    matches!(action, OperationAction::Abort { .. }),
                    "retry_safe={retry_safe:?}, request_sent={request_sent:?}: got {action:?}"
                );
            }
        }
    }

    #[test]
    fn definitively_unsent_unsafe_patch_retries() {
        let action = evaluate(
            &make_patch_operation(Some(false)),
            make_transport_error(RequestSentStatus::NotSent),
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "a definitively unsent PATCH is safe to retry, got {action:?}"
        );
    }

    #[test]
    fn retry_safe_patch_retries_after_possible_execution() {
        let action = evaluate(
            &make_patch_operation(Some(true)),
            make_transport_error(RequestSentStatus::Sent),
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "a convergent PATCH may be retried, got {action:?}"
        );
    }

    #[test]
    fn unsafe_patch_request_timeout_aborts() {
        let action = evaluate(
            &make_patch_operation(Some(false)),
            make_http_error(StatusCode::RequestTimeout),
        );
        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "408 leaves PATCH execution ambiguous, got {action:?}"
        );
    }
}

/// Stored procedure execution supplies the exhaustive status matrix for
/// operations the driver refuses to retry after an ambiguous failure. The
/// PATCH-specific tests above verify classification into the same shared
/// retry gate without duplicating every status case here.
mod stored_procedure_retries {
    use super::*;
    use crate::models::{FeedRange, OperationType, StoredProcedureReference};

    fn make_execute_stored_procedure_operation() -> CosmosOperation {
        let account = AccountReference::with_account_key(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "testcontainer".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let container = ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &props,
        );
        let sproc = StoredProcedureReference::from_name(&container, "mysproc");
        let target = FeedRange::for_partition(
            PartitionKey::from("pk1"),
            container.partition_key_definition(),
        );
        let resource_ref: crate::models::CosmosResourceReference = sproc.into();
        CosmosOperation::new(OperationType::Execute, resource_ref, Some(target))
    }

    fn evaluate(op: &CosmosOperation, result: TransportResult) -> OperationAction {
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let endpoint = CosmosEndpoint::global(
            url::Url::parse("https://test.documents.azure.com:443/").unwrap(),
        );
        recovery::classify_result(op, &endpoint, result, &state).0
    }

    #[test]
    fn execute_does_not_allow_ambiguous_outcome_retry() {
        assert!(!make_execute_stored_procedure_operation().allows_ambiguous_outcome_retry());
        // Every other operation type does, including the non-idempotent ones.
        assert!(make_create_operation().allows_ambiguous_outcome_retry());
        assert!(make_create_item_operation().allows_ambiguous_outcome_retry());
        assert!(make_read_operation().allows_ambiguous_outcome_retry());
    }

    #[test]
    fn transport_error_sent_aborts() {
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_transport_error(RequestSentStatus::Sent),
        );
        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "a sent stored procedure must not be retried, got {action:?}"
        );
    }

    #[test]
    fn transport_error_unknown_aborts() {
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_transport_error(RequestSentStatus::Unknown),
        );
        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "an unknown send status is not proof of non-execution, got {action:?}"
        );
    }

    #[test]
    fn transport_error_not_sent_retries() {
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_transport_error(RequestSentStatus::NotSent),
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "a definitively unsent stored procedure is safe to retry, got {action:?}"
        );
    }

    #[test]
    fn request_timeout_aborts() {
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_http_error(StatusCode::RequestTimeout),
        );
        assert!(
            matches!(action, OperationAction::Abort { .. }),
            "408 leaves the outcome unknown, got {action:?}"
        );
    }

    #[test]
    fn internal_server_errors_abort() {
        for status_code in [
            StatusCode::InternalServerError,
            StatusCode::BadGateway,
            StatusCode::GatewayTimeout,
        ] {
            let action = evaluate(
                &make_execute_stored_procedure_operation(),
                make_http_error(status_code),
            );
            assert!(
                matches!(action, OperationAction::Abort { .. }),
                "{status_code:?} leaves the outcome unknown, got {action:?}"
            );
        }
    }

    #[test]
    fn service_unavailable_retries() {
        // Cosmos returns 503 only for requests it did not process, so the
        // procedure provably did not run.
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_http_error(StatusCode::ServiceUnavailable),
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "503 proves non-execution and stays retryable, got {action:?}"
        );
    }

    #[test]
    fn gone_retries() {
        // 410 is a routing rejection — the partition moved before execution.
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_http_error(StatusCode::Gone),
        );
        assert!(
            matches!(action, OperationAction::FailoverRetry { .. }),
            "410 is a pre-execution rejection, got {action:?}"
        );
    }

    #[test]
    fn retry_with_still_retries_in_region() {
        // 449 means the request never completed; aborting it would break
        // stored procedure execution under ordinary write contention.
        let action = evaluate(
            &make_execute_stored_procedure_operation(),
            make_http_error_status(CosmosStatus::new(StatusCode::from(449_u16))),
        );
        assert!(
            matches!(action, OperationAction::InRegionRetry { .. }),
            "449 is a pre-execution rejection, got {action:?}"
        );
    }
}

#[test]
fn build_transport_error_forwards_inner_diagnostics() {
    // Wrapping must preserve the inner diagnostics `Arc`.
    let diag: std::sync::Arc<crate::diagnostics::DiagnosticsContext> = std::sync::Arc::new(
        crate::diagnostics::DiagnosticsContextBuilder::new(
            crate::models::ActivityId::new_uuid(),
            std::sync::Arc::new(crate::options::DiagnosticsOptions::default()),
        )
        .complete(),
    );
    let inner = crate::error::CosmosError::builder()
        .with_status(crate::error::status_codes::TRANSPORT_GENERATED_503)
        .with_message("inner transport failure")
        .with_diagnostics(std::sync::Arc::clone(&diag))
        .build();

    let outer = recovery::error::build_transport_error(
        &crate::error::status_codes::TRANSPORT_GENERATED_503,
        inner,
    );

    let outer_diag = outer
        .diagnostics()
        .expect("outer error must inherit inner diagnostics");
    assert!(
        std::sync::Arc::ptr_eq(&outer_diag, &diag),
        "outer diagnostics must be the same Arc as the inner's"
    );
}

#[test]
fn transport_abort_error_includes_status_kind_and_details() {
    let op = make_create_operation();
    let result = TransportResult {
        outcome: TransportOutcome::TransportError {
            status: crate::error::status_codes::TRANSPORT_GENERATED_503,
            error: crate::error::CosmosError::builder()
                .with_status(crate::error::status_codes::TRANSPORT_GENERATED_503)
                .with_message("failed to execute `reqwest` request")
                .with_source(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "socket reset",
                ))
                .build(),
            request_sent: RequestSentStatus::Unknown,
        },
    };
    let state = OperationRetryState::initial(0, false, Vec::new(), 0, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, _effects) = recovery::classify_result(&op, &endpoint, result, &state);

    match action {
        OperationAction::Abort { error } => {
            // `.status()` compiling proves this stayed a typed `CosmosError`.
            assert_eq!(
                error.status(),
                crate::error::status_codes::TRANSPORT_GENERATED_503
            );
            let text = error.to_string();
            assert!(text.contains("HTTP 503/20003"));
            assert!(text.contains("TransportGenerated503"));
            assert!(text.contains("failed to execute `reqwest` request"));
            assert!(text.contains("socket reset"));
        }
        other => panic!("expected abort, got {other:?}"),
    }
}

#[test]
fn transport_error_over_budget_aborts() {
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::NotSent);
    let state = OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: crate::pipeline::operation::routing::LocationIndex::initial(0),
        failover_retry_count: 1,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: std::time::Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 1,
        max_backend_failover_retries: 120,
        max_session_retries: 1,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: Vec::new(),
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
    };

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, _effects) = recovery::classify_result(&op, &endpoint, result, &state);
    match action {
        OperationAction::Abort { error } => {
            assert_eq!(
                error.status(),
                crate::error::status_codes::TRANSPORT_GENERATED_503
            );
        }
        other => panic!("expected abort, got {other:?}"),
    }
}

#[test]
fn http_error_aborts() {
    let op = make_read_operation();
    let result = make_http_error(StatusCode::BadRequest);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);

    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let (action, _effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::Abort { .. }));
}

#[test]
fn partition_topology_gone_aborts_for_dataflow_handling() {
    let op = make_read_operation();
    let result =
        make_http_error_status(CosmosStatus::new(StatusCode::Gone).with_sub_status(
            crate::error::status_codes::substatus::PARTITION_KEY_RANGE_GONE.value(),
        ));
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    match action {
        OperationAction::Abort { error, .. } => {
            assert_eq!(
                error.status(),
                CosmosStatus::new(StatusCode::Gone).with_sub_status(
                    crate::error::status_codes::substatus::PARTITION_KEY_RANGE_GONE.value()
                )
            );
        }
        other => panic!("expected abort, got {other:?}"),
    }
    assert!(effects.is_empty());
}

#[test]
fn non_topology_gone_still_retries() {
    let op = make_read_operation();
    let result = make_http_error_status(
        CosmosStatus::new(StatusCode::Gone)
            .with_sub_status(crate::error::status_codes::substatus::NAME_CACHE_STALE.value()),
    );
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn write_forbidden_triggers_failover_and_refresh_effect() {
    let op = make_create_operation();
    let result = TransportResult {
        outcome: TransportOutcome::HttpError {
            status: crate::error::status_codes::WRITE_FORBIDDEN,
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    };
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    // PPCB disabled (default) → endpoint mark is emitted so non-PPCB
    // accounts still get region-wide failover for the 60 s TTL.
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
        "non-PPCB 403/3 must mark the endpoint unavailable"
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
        "403/3 must always mark the partition unavailable"
    );
}

#[test]
fn write_forbidden_when_ppcb_managed_skips_endpoint_mark() {
    // PPCB-managed 403/3 must not mark the whole endpoint unavailable.
    let op = make_create_item_operation();
    let mut state = OperationRetryState::initial(0, true /* multi-write */, Vec::new(), 3, 1);
    state.ppcb_active = true;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
        "403/3 must still refresh account properties"
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
        "PPCB-managed 403/3 must still mark the partition unavailable"
    );
    assert!(
        !effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
        "PPCB-managed 403/3 must NOT mark the endpoint unavailable; \
         per-partition counter drives failover"
    );
}

fn http_error_status(status: CosmosStatus) -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::HttpError {
            status,
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    }
}

#[test]
fn database_account_not_found_on_write_emits_refresh_mark_endpoint_and_failover() {
    // 403/1008 must refresh topology, mark endpoint/partition, and failover-retry.
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    assert!(
        matches!(
            action,
            OperationAction::FailoverRetry { delay: Some(_), .. }
        ),
        "expected FailoverRetry with backend-failover delay, got {:?}",
        action
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
        "missing RefreshAccountProperties effect; effects={:?}",
        effects
    );
    assert!(
        effects.iter().any(|e| matches!(
            e,
            LocationEffect::MarkEndpointUnavailable {
                reason: UnavailableReason::DatabaseAccountNotFound,
                ..
            }
        )),
        "missing MarkEndpointUnavailable{{DatabaseAccountNotFound}}; effects={:?}",
        effects
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
        "missing MarkPartitionUnavailable; effects={:?}",
        effects
    );
}

#[test]
fn database_account_not_found_on_read_uses_op_read_only_for_partition_mark() {
    // 1008 fires on reads too; partition marks must preserve operation direction.
    let op = make_read_operation();
    let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    let mark = effects.iter().find_map(|e| match e {
        LocationEffect::MarkPartitionUnavailable(p) => Some(p),
        _ => None,
    });
    let mark = mark.expect("expected MarkPartitionUnavailable for 1008 on read");
    assert!(
        mark.is_read,
        "1008 on a read op must mark the partition with is_read=true so PPCB \
         credits the read-direction failure counter, not the write counter"
    );
}

#[test]
fn database_account_not_found_aborts_when_backend_failover_budget_exhausted() {
    // Exhausting the 1008 budget must bubble up the original backend status.
    let op = make_create_operation();
    // Multi-write account so the handler uses the dedicated
    // backend-failover budget rather than the generic one.
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    // Drive the backend-failover counter directly to the cap rather than
    // looping through `advance_backend_failover`.
    state.backend_failover_retry_count = state.max_backend_failover_retries;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    match action {
        OperationAction::Abort { error } => {
            assert_eq!(
                error.status(),
                crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND,
                "1008 exhausted-budget bubble-up must surface the original status unchanged"
            );
        }
        other => panic!(
            "expected Abort once backend-failover budget is exhausted, got {:?}",
            other
        ),
    }
}

#[test]
fn write_forbidden_aborts_when_backend_failover_budget_exhausted() {
    // Exhausting the 403/3 backend-failover budget must bubble up the original status.
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    state.backend_failover_retry_count = state.max_backend_failover_retries;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    match action {
        OperationAction::Abort { error } => {
            assert_eq!(
                error.status(),
                crate::error::status_codes::WRITE_FORBIDDEN,
                "403/3 exhausted-budget bubble-up must surface the original status unchanged"
            );
        }
        other => panic!("expected Abort, got {:?}", other),
    }
}

#[test]
fn backend_failover_aborts_when_cumulative_delay_budget_exhausted() {
    // Even with retry-count headroom, a spent cumulative delay budget must
    // bubble up the original topology status rather than retry forever.
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    state.backend_failover_retry_count = 1;
    state.backend_failover_cumulative_delay = BACKEND_FAILOVER_MAX_TOTAL_DELAY;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    assert!(
        matches!(action, OperationAction::Abort { .. }),
        "spent cumulative delay budget must abort even with retry-count headroom, got {action:?}"
    );
}

#[test]
fn backend_failover_delay_stays_within_cap_across_retries() {
    // Walk several successive multi-write 403/3 retries through the full
    // evaluate path; every scheduled delay must stay within the jittered cap.
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    loop {
        let (action, _effects) = recovery::classify_result(
            &op,
            &endpoint,
            http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
            &state,
        );
        match action {
            OperationAction::FailoverRetry {
                new_state,
                delay: Some(delay),
            } => {
                assert_within_backoff_cap(delay);
                state = new_state;
            }
            OperationAction::Abort { .. } => break,
            other => panic!("expected paced FailoverRetry or Abort, got {other:?}"),
        }
    }
    assert_eq!(
        state.backend_failover_cumulative_delay,
        BACKEND_FAILOVER_MAX_TOTAL_DELAY
    );
    assert!(
        state.backend_failover_retry_count == 4,
        "5s budget should be exhausted in 4 retries (first is immediate), got {}",
        state.backend_failover_retry_count
    );
}

#[test]
fn backend_failover_second_retry_uses_exponential_base() {
    // The first retry is immediate; the 1s exponential base starts at the
    // second retry so a persistent topology error still backs off.
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (first, _) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );
    let OperationAction::FailoverRetry {
        new_state,
        delay: Some(first_delay),
    } = first
    else {
        panic!("expected paced FailoverRetry");
    };
    assert_eq!(first_delay, Duration::ZERO);

    let (second, _) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &new_state,
    );
    let OperationAction::FailoverRetry {
        delay: Some(second_delay),
        ..
    } = second
    else {
        panic!("expected paced FailoverRetry");
    };
    let (lo, hi) = second_backend_failover_delay_bounds();
    assert!(
        second_delay >= lo && second_delay <= hi,
        "second retry must use the ~1s exponential base (jittered), got {second_delay:?}"
    );
}

#[test]
fn database_account_not_found_does_not_consume_generic_failover_budget() {
    // Backend-failover retries must not consume the generic failover budget.
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    match action {
        OperationAction::FailoverRetry { new_state, delay } => {
            assert_eq!(new_state.failover_retry_count, 0);
            assert_eq!(new_state.backend_failover_retry_count, 1);
            let delay = delay.expect("multi-write 1008 must pace retries with a backoff delay");
            assert_eq!(
                delay,
                Duration::ZERO,
                "multi-write 1008 first retry is immediate; the account refresh already \
                 supplies the new region"
            );
            assert_eq!(
                new_state.backend_failover_cumulative_delay, delay,
                "scheduled delay must be accumulated toward the budget"
            );
        }
        other => panic!("expected FailoverRetry, got {:?}", other),
    }
}

#[test]
fn write_forbidden_does_not_consume_generic_failover_budget() {
    // Mirror of the 1008 test for 403/3 on a multi-write account.
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    match action {
        OperationAction::FailoverRetry { new_state, delay } => {
            assert_eq!(new_state.failover_retry_count, 0);
            assert_eq!(new_state.backend_failover_retry_count, 1);
            let delay = delay.expect("multi-write 403/3 must pace retries with a backoff delay");
            assert_eq!(
                delay,
                Duration::ZERO,
                "multi-write 403/3 first retry is immediate; the account refresh already \
                 supplies the new region"
            );
            assert_eq!(
                new_state.backend_failover_cumulative_delay, delay,
                "scheduled delay must be accumulated toward the budget"
            );
        }
        other => panic!("expected FailoverRetry, got {:?}", other),
    }
}

#[test]
fn write_forbidden_on_single_write_uses_backend_failover_budget() {
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    match action {
        OperationAction::FailoverRetry { new_state, delay } => {
            assert_eq!(new_state.failover_retry_count, 0);
            assert_eq!(new_state.backend_failover_retry_count, 1);
            let delay = delay.expect("single-write 403/3 must pace retries with a backoff delay");
            assert_eq!(
                delay,
                Duration::ZERO,
                "single-write 403/3 first retry is immediate; the account refresh already \
                 supplies the new region"
            );
            assert_eq!(new_state.backend_failover_cumulative_delay, delay);
        }
        other => panic!("expected FailoverRetry, got {:?}", other),
    }
}

#[test]
fn write_forbidden_on_single_write_ignores_exhausted_generic_budget() {
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.failover_retry_count = state.max_failover_retries;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    match action {
        OperationAction::FailoverRetry { new_state, delay } => {
            assert_eq!(new_state.failover_retry_count, state.max_failover_retries);
            assert_eq!(new_state.backend_failover_retry_count, 1);
            assert!(delay.is_some());
        }
        other => panic!("expected backend FailoverRetry, got {other:?}"),
    }
}

#[test]
fn write_forbidden_on_single_write_aborts_when_backend_budget_exhausted() {
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.backend_failover_cumulative_delay = BACKEND_FAILOVER_MAX_TOTAL_DELAY;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::Abort { .. }));
}

#[test]
fn database_account_not_found_on_single_write_uses_backend_failover_budget() {
    // Single-write 1008 also uses backend-failover budget + delay; the topology-not-found
    // signal is identical to multi-write and needs the same convergence window.
    let op = make_create_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, _effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    match action {
        OperationAction::FailoverRetry { new_state, delay } => {
            assert_eq!(new_state.failover_retry_count, 0);
            assert_eq!(new_state.backend_failover_retry_count, 1);
            let delay = delay.expect("single-write 1008 must pace retries with a backoff delay");
            assert_eq!(
                delay,
                Duration::ZERO,
                "single-write 1008 first retry is immediate; the account refresh already \
                 supplies the new region"
            );
            assert_eq!(
                new_state.backend_failover_cumulative_delay, delay,
                "scheduled delay must be accumulated toward the budget"
            );
        }
        other => panic!("expected FailoverRetry, got {:?}", other),
    }
}

#[test]
fn database_account_not_found_when_ppcb_managed_skips_endpoint_mark() {
    // PPCB-managed 1008 must not mark the whole endpoint unavailable;
    // the per-partition counter drives failover.
    let op = make_create_item_operation();
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    state.ppcb_active = true;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        http_error_status(crate::error::status_codes::DATABASE_ACCOUNT_NOT_FOUND),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
        "1008 must still refresh account properties"
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
        "PPCB-managed 1008 must still mark the partition unavailable"
    );
    assert!(
        !effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
        "PPCB-managed 1008 must NOT mark the endpoint unavailable; \
         per-partition counter drives failover"
    );
}

#[test]
fn read_session_not_available_triggers_session_retry() {
    let op = make_read_operation();
    let result = TransportResult {
        outcome: TransportOutcome::HttpError {
            status: crate::error::status_codes::READ_SESSION_NOT_AVAILABLE,
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    };
    let state = OperationRetryState::initial(0, true, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::SessionRetry { .. }));
    assert!(effects.is_empty());
}

#[test]
fn service_unavailable_marks_endpoint_unavailable() {
    let op = make_read_operation();
    let result = make_http_error(StatusCode::ServiceUnavailable);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

/// Regression guard: a cross-region hedge
/// can only be spawned from a region-changing retry action, and for a
/// 429 that path is gated to sub-status `3092`
/// (`SystemResourceUnavailable`). Every other throttle sub-status —
/// `3200` (`RU_BUDGET_EXCEEDED`), `3210` (`RU_BUDGET_EXCEEDED_FOR_MASTER`),
/// and `3214` (`HOT_PARTITION_KEY_THROTTLED`) — must instead `Abort`:
/// failing them over to another region cannot conjure throughput or
/// cool a hot logical partition and only spreads the throttle. Pinning
/// this here ensures a future widening of the retry-trigger group
/// cannot silently begin hedging RU-exhaustion or hot-partition throttles.
#[test]
fn throttle_substatus_gates_hedge_eligibility() {
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let throttle_result = |sub: SubStatusCode| TransportResult {
        outcome: TransportOutcome::HttpError {
            status: CosmosStatus::from_parts(StatusCode::TooManyRequests, Some(sub)),
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    };

    // 429/3092 — transient backend pressure → failover-eligible, hence
    // the only 429 the hedge upgrade in `maybe_upgrade_to_hedge` may act on.
    let op = make_read_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let (action, _) = recovery::classify_result(
        &op,
        &endpoint,
        throttle_result(crate::error::status_codes::substatus::SYSTEM_RESOURCE_UNAVAILABLE),
        &state,
    );
    assert!(
        matches!(action, OperationAction::FailoverRetry { .. }),
        "429/3092 SystemResourceUnavailable must be failover-eligible; got {action:?}",
    );

    // Every other throttle sub-status must NOT become a region-changing
    // retry, so it can never be upgraded into a cross-region hedge.
    for sub in [
        crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED, // 3200
        crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED_FOR_MASTER, // 3210
        crate::error::status_codes::substatus::HOT_PARTITION_KEY_THROTTLED, // 3214
    ] {
        let op = make_read_operation();
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
        let (action, _) = recovery::classify_result(&op, &endpoint, throttle_result(sub), &state);
        assert!(
            !matches!(
                action,
                OperationAction::FailoverRetry { .. } | OperationAction::SessionRetry { .. }
            ),
            "429/{sub:?} must not become a region-changing retry; \
             got {action:?}",
        );
    }
}

#[test]
fn service_unavailable_non_idempotent_write_retries() {
    let op = make_create_operation();
    let result = make_http_error(StatusCode::ServiceUnavailable);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn service_unavailable_non_idempotent_retries_with_ppaf() {
    // With PPAF enabled, behavior is the same as without — non-idempotent
    // writes always retry. This test validates PPAF doesn't interfere.
    let op = make_create_operation();
    let result = make_http_error(StatusCode::ServiceUnavailable);
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.ppaf_write_retry_allowed = true;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn transport_error_non_idempotent_retries_with_ppaf() {
    // With PPAF enabled, behavior is the same as without — non-idempotent
    // writes always retry. Sent → only partition mark (no endpoint mark).
    let op = make_create_operation();
    let result = make_transport_error(RequestSentStatus::Sent);
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.ppaf_write_retry_allowed = true;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn deadline_exceeded_aborts_with_timeout_status() {
    let op = make_read_operation();
    let result = TransportResult {
        outcome: TransportOutcome::DeadlineExceeded {
            request_sent: RequestSentStatus::Unknown,
        },
    };
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    match action {
        OperationAction::Abort { error } => {
            let status = error.status();
            assert_eq!(status.status_code(), StatusCode::RequestTimeout);
            assert_eq!(
                status.sub_status(),
                Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT)
            );
        }
        _ => panic!("expected timeout to abort"),
    }
    assert!(effects.is_empty());
}

#[test]
fn internal_server_error_on_read_fails_over() {
    let op = make_read_operation();
    let result = make_http_error(StatusCode::InternalServerError);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn internal_server_error_on_read_marks_partition_unavailable() {
    let op = make_read_operation();
    let result = make_http_error(StatusCode::InternalServerError);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

#[test]
fn transport_error_not_sent_marks_endpoint_only() {
    // Not-sent marks the endpoint only; Gateway connect failures are endpoint-wide.
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::NotSent);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

#[test]
fn transport_error_not_sent_with_ppcb_still_marks_endpoint() {
    // Not-sent with PPCB active → endpoint is unreachable regardless of
    // PPCB state. Connection failures are endpoint-wide, so the endpoint
    // mark must not be suppressed by PPCB. The per-partition mark is
    // intentionally not emitted in either case for this branch.
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::NotSent);
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    state.ppcb_active = true;
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

#[test]
fn transport_error_unknown_sent_status_marks_partition_only() {
    // Unknown sent status is treated as "possibly sent" → endpoint is
    // potentially reachable, so only partition-level marking is applied.
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::Unknown);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn transport_error_not_sent_over_budget_aborts_with_marks() {
    // Even over budget, not-sent updates endpoint routing state only.
    let op = make_read_operation();
    let result = make_transport_error(RequestSentStatus::NotSent);
    let state = OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: crate::pipeline::operation::routing::LocationIndex::initial(0),
        failover_retry_count: 1,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: std::time::Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 1,
        max_backend_failover_retries: 120,
        max_session_retries: 1,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: Vec::new(),
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
    };
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::Abort { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

#[test]
fn request_timeout_from_server_marks_partition_and_endpoint_unavailable() {
    let op = make_read_operation();
    let result = make_http_error(StatusCode::RequestTimeout);
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, &state);
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(effects.iter().any(|e| matches!(
        e,
        LocationEffect::MarkEndpointUnavailable {
            reason: UnavailableReason::RequestTimeout,
            ..
        }
    )));
}

// ── is_region_confirming_status ───────────────────────────────────

fn status_with_substatus(code: StatusCode, sub: SubStatusCode) -> CosmosStatus {
    CosmosStatus::from_parts(code, Some(sub))
}

#[test]
fn region_confirming_true_for_2xx() {
    // 200 OK, 201 Created — typical write success codes.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Ok
    )));
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Created
    )));
    // 202 Accepted — used by long-running control-plane operations
    // (e.g., container offer adjustments) that complete asynchronously.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Accepted
    )));
    // 204 No Content — used by deletes and some replace operations.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::NoContent
    )));
    // 207 MultiStatus — used by transactional batch responses; every
    // sub-operation result is encoded in the body but the outer status
    // still proves the region processed the batch.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::from(207u16)
    )));
}

#[test]
fn region_confirming_true_for_definitive_4xx() {
    // 409 Conflict — server processed and rejected the write.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Conflict
    )));
    // 412 Precondition Failed — server processed and rejected.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::PreconditionFailed
    )));
    // 413 Payload Too Large — server processed and rejected.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::PayloadTooLarge
    )));
    // 400/401 — server processed and rejected.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::BadRequest
    )));
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Unauthorized
    )));
    // 404 with no sub-status (404/0) — server confirms the resource is
    // gone. Distinct from 404/1002 (ReadSessionNotAvailable), which is
    // routed to `SessionRetry` rather than `Abort`.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::NotFound
    )));
    // Explicit 404/0 (sub-status 0) construction — same outcome.
    assert!(routing::is_region_confirming_status(
        &status_with_substatus(StatusCode::NotFound, SubStatusCode::from(0u16))
    ));
}

#[test]
fn region_confirming_false_for_retry_trigger_statuses() {
    // 503 ServiceUnavailable
    assert!(!routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::ServiceUnavailable
    )));
    // 408 RequestTimeout
    assert!(!routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::RequestTimeout
    )));
    // 410 Gone
    assert!(!routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Gone
    )));
    // 429/3092 SystemResourceUnavailable
    assert!(!routing::is_region_confirming_status(
        &status_with_substatus(
            StatusCode::TooManyRequests,
            crate::error::status_codes::substatus::SYSTEM_RESOURCE_UNAVAILABLE
        )
    ));
    // 403/3 WriteForbidden
    assert!(!routing::is_region_confirming_status(
        &status_with_substatus(
            StatusCode::Forbidden,
            crate::error::status_codes::substatus::WRITE_FORBIDDEN
        )
    ));
    // 403/1008 is a topology signal; the dedicated handler refreshes and fails over.
    assert!(!routing::is_region_confirming_status(
        &status_with_substatus(
            StatusCode::Forbidden,
            crate::error::status_codes::substatus::DATABASE_ACCOUNT_NOT_FOUND
        )
    ));
    // 410/1008 is partition migration, not DatabaseAccountNotFound.
    assert!(!routing::is_region_confirming_status(
        &status_with_substatus(
            StatusCode::Gone,
            crate::error::status_codes::substatus::COMPLETING_PARTITION_MIGRATION
        )
    ));
}

#[test]
fn region_confirming_false_for_client_synthesized_timeout() {
    assert!(!routing::is_region_confirming_status(
        &status_with_substatus(
            StatusCode::RequestTimeout,
            crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT
        )
    ));
}

// ── partition_effects_for_deferral ────────────────────────────────

fn endpoint_for_test() -> CosmosEndpoint {
    CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
}

#[test]
fn deferral_passes_all_effects_through_for_reads() {
    let effects = vec![
        LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
            partition_key_range_id: None,
            region: None,
            is_read: true,
            is_partitioned_resource: true,
        }),
        LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint_for_test(),
            reason: UnavailableReason::ServiceUnavailable,
        },
        LocationEffect::RefreshAccountProperties,
    ];
    let (immediate, deferred) =
        routing::partition_effects_for_deferral(true, false, false, effects.into());
    assert_eq!(immediate.len(), 3);
    assert!(deferred.is_empty());
}

#[test]
fn deferral_extracts_partition_marks_for_writes() {
    let effects = vec![
        LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
            partition_key_range_id: None,
            region: None,
            is_read: false,
            is_partitioned_resource: true,
        }),
        LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint_for_test(),
            reason: UnavailableReason::ServiceUnavailable,
        },
        LocationEffect::RefreshAccountProperties,
    ];
    // Single-master write, non-PPAF: partition mark is deferred,
    // endpoint mark stays immediate.
    let (immediate, deferred) =
        routing::partition_effects_for_deferral(false, false, false, effects.into());
    assert_eq!(immediate.len(), 2);
    assert!(immediate
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(immediate
        .iter()
        .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    assert_eq!(deferred.len(), 1);
    assert!(matches!(
        deferred[0],
        LocationEffect::MarkPartitionUnavailable(_)
    ));
}

#[test]
fn deferral_with_no_partition_marks_returns_empty_deferred() {
    let effects = vec![
        LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint_for_test(),
            reason: UnavailableReason::ServiceUnavailable,
        },
        LocationEffect::RefreshAccountProperties,
    ];
    // Single-master write, non-PPAF: endpoint mark stays immediate.
    let (immediate, deferred) =
        routing::partition_effects_for_deferral(false, false, false, effects.into());
    assert_eq!(immediate.len(), 2);
    assert!(deferred.is_empty());
}

#[test]
fn deferral_defers_endpoint_mark_for_ppaf_single_master_writes() {
    // PPAF on single-master account: a transient write failure must NOT
    // immediately mark the only known write region as unavailable.
    // Both partition and endpoint marks must be deferred until the write
    // definitively reaches a region.
    let effects = vec![
        LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
            partition_key_range_id: None,
            region: None,
            is_read: false,
            is_partitioned_resource: true,
        }),
        LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint_for_test(),
            reason: UnavailableReason::TransportError,
        },
        LocationEffect::RefreshAccountProperties,
    ];
    let (immediate, deferred) =
        routing::partition_effects_for_deferral(false, false, true, effects.into());
    // Only RefreshAccountProperties should be applied immediately.
    assert_eq!(immediate.len(), 1);
    assert!(matches!(
        immediate[0],
        LocationEffect::RefreshAccountProperties
    ));
    // Both partition and endpoint marks are deferred.
    assert_eq!(deferred.len(), 2);
    assert!(deferred
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(deferred
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
}

#[test]
fn deferral_passes_all_effects_through_for_multi_master_writes() {
    // Multi-master writes are PPCB-managed: failures must be applied
    // immediately so the per-partition write-failure counter can drive
    // threshold-based failover. The PPAF flag is irrelevant because
    // PPAF only applies to single-master accounts.
    let effects = vec![
        LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
            partition_key_range_id: None,
            region: None,
            is_read: false,
            is_partitioned_resource: true,
        }),
        LocationEffect::MarkEndpointUnavailable {
            endpoint: endpoint_for_test(),
            reason: UnavailableReason::ServiceUnavailable,
        },
        LocationEffect::RefreshAccountProperties,
    ];
    let (immediate, deferred) =
        routing::partition_effects_for_deferral(false, true, false, effects.into());
    assert_eq!(immediate.len(), 3);
    assert!(deferred.is_empty());
}

// -----------------------------------------------------------------------
// Hub-region-processing-only latch tests.
//
// See Spec 0010, Hub-region processing header, §3 / public spec §4.1 for the
// shape these cases are meant to cover (T-1..T-5, T-AC-8, T-8, T-9).
//
// All tests drive `evaluate_transport_result` against a 1002 response and
// inspect the `OperationAction::SessionRetry { new_state }`. Per
// `build_session_retry_state`, the latch is set when ALL four conditions
// hold:
//
//   1. `is_dataplane`                          (AC-8)
//   2. `!can_use_multiple_write_locations`     (AC-4)
//   3. `session_token_retry_count == 0`        (AC-3, first-1002-only)
//   4. `!hub_region_processing_only`           (idempotency / sticky)
//
// `OperationRetryState::initial(..)` defaults both `is_dataplane` and
// `hub_region_processing_only` to `false`, so tests that want the
// latch-on path mutate `is_dataplane = true` explicitly. This mirrors the
// production wiring in `operation::execute`, which sets
// `retry_state.is_dataplane = pipeline_type.is_data_plane()` immediately
// after constructing the state.
// -----------------------------------------------------------------------

fn make_read_session_not_available_result() -> TransportResult {
    TransportResult {
        outcome: TransportOutcome::HttpError {
            status: crate::error::status_codes::READ_SESSION_NOT_AVAILABLE,
            cosmos_headers: CosmosResponseHeaders::default(),
            body: vec![],
            request_sent: RequestSentStatus::Sent,
        },
    }
}

fn test_endpoint() -> CosmosEndpoint {
    CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
}

/// Drives one 1002 against `state` and returns the resulting
/// `new_state` from `SessionRetry`. Panics if the action isn't a
/// `SessionRetry` so callers don't have to repeat that pattern.
fn session_retry_state_for_1002(state: &OperationRetryState) -> OperationRetryState {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let result = make_read_session_not_available_result();

    let (action, effects) = recovery::classify_result(&op, &endpoint, result, state);
    assert!(
        effects.is_empty(),
        "1002 should not emit location effects, got {effects:?}",
    );
    match action {
        OperationAction::SessionRetry { new_state } => new_state,
        other => panic!("expected SessionRetry, got {other:?}"),
    }
}

/// T-1 — Single-master, data-plane, first 1002 sets the latch.
/// Covers AC-1 of Spec 0010: Hub-region processing header.
#[test]
fn hub_region_latch_sets_on_first_1002_single_master_dataplane() {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;

    let new_state = session_retry_state_for_1002(&state);

    assert!(
        new_state.hub_region_processing_only,
        "first 1002 on single-master data-plane should latch",
    );
    // The session-retry counter advanced — the latch decision happened
    // pre-increment, so reading `== 0` on `state` was correct.
    assert_eq!(new_state.session_token_retry_count, 1);
}

/// T-2 — Multi-master 1002s never latch (AC-4).
#[test]
fn hub_region_latch_does_not_set_on_multi_master_1002() {
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3);
    state.is_dataplane = true;

    let new_state = session_retry_state_for_1002(&state);

    assert!(
        !new_state.hub_region_processing_only,
        "multi-master 1002 must not latch the hub-region header",
    );
}

/// T-3 — Latch is sticky across subsequent 1002s (AC-2).
/// The second 1002 must NOT clear the latch even though
/// `session_token_retry_count` is no longer 0.
#[test]
fn hub_region_latch_stays_set_on_subsequent_1002() {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;

    let after_first = session_retry_state_for_1002(&state);
    assert!(after_first.hub_region_processing_only);

    let after_second = session_retry_state_for_1002(&after_first);
    assert!(
        after_second.hub_region_processing_only,
        "latch must persist across subsequent 1002 retries",
    );
    assert_eq!(after_second.session_token_retry_count, 2);
}

/// T-4 — Non-1002 responses on a single-master data-plane state never
/// latch (AC-5). Drives 200, 410, and 503 to confirm the trigger is
/// scoped to the 1002 arm.
#[test]
fn hub_region_latch_does_not_set_on_non_1002_responses() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;

    // 200: completes — no latch ever runs, but assert via state pass-through.
    let (action, _) = recovery::classify_result(&op, &endpoint, make_success_result(), &state);
    assert!(matches!(action, OperationAction::Complete(_)));

    // 410 (Gone). On a read this fails over but does not latch.
    let (action, _) =
        recovery::classify_result(&op, &endpoint, make_http_error(StatusCode::Gone), &state);
    match action {
        OperationAction::FailoverRetry { new_state, .. } => {
            assert!(!new_state.hub_region_processing_only);
        }
        OperationAction::Abort { .. } => {
            // Acceptable terminal outcome; either way the latch wasn't set.
        }
        other => panic!("unexpected action for 410: {other:?}"),
    }

    // 503 (ServiceUnavailable) on a read fails over but does not latch.
    let (action, _) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error(StatusCode::ServiceUnavailable),
        &state,
    );
    match action {
        OperationAction::FailoverRetry { new_state, .. } => {
            assert!(!new_state.hub_region_processing_only);
        }
        OperationAction::Abort { .. } => {
            // Terminal outcome — latch can't be observed but it cannot
            // have been set because the 503 path does not run the
            // 1002 trigger.
        }
        other => panic!("unexpected action for 503: {other:?}"),
    }
}

/// T-5 — Boundary at `session_token_retry_count >= 2`: the second 1002
/// still latches the previously-set flag (sticky), and the third 1002
/// aborts. Validates AC-3 boundary semantics inherited from the
/// existing `>= 2` abort check.
#[test]
fn hub_region_latch_state_at_budget_exhaustion() {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;

    let after_first = session_retry_state_for_1002(&state);
    assert!(after_first.hub_region_processing_only);
    let after_second = session_retry_state_for_1002(&after_first);
    assert!(after_second.hub_region_processing_only);
    assert_eq!(after_second.session_token_retry_count, 2);

    let op = make_read_operation();
    let endpoint = test_endpoint();
    let result = make_read_session_not_available_result();
    let (action, _) = recovery::classify_result(&op, &endpoint, result, &after_second);
    assert!(
        matches!(action, OperationAction::Abort { .. }),
        "third 1002 must abort, got {action:?}",
    );
}

/// T-AC-8 — Metadata-pipeline 1002s never latch (AC-8). Same shape as
/// T-1 but with `is_dataplane = false`.
#[test]
fn hub_region_latch_does_not_set_on_metadata_pipeline_1002() {
    // is_dataplane defaults to false from `initial(..)`.
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    assert!(!state.is_dataplane);

    let new_state = session_retry_state_for_1002(&state);

    assert!(
        !new_state.hub_region_processing_only,
        "metadata-pipeline 1002 must not latch the hub-region header",
    );
}

/// T-8 — Independent operations don't share latch state (AC-6).
/// `OperationRetryState::initial(..)` is fresh per call, so a latch on
/// one operation can't leak to another even when they go through the
/// same evaluate path.
#[test]
fn hub_region_latch_independent_operations_do_not_share_state() {
    let mut op_a = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    op_a.is_dataplane = true;
    let mut op_b = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    op_b.is_dataplane = true;

    let op_a_after = session_retry_state_for_1002(&op_a);
    assert!(op_a_after.hub_region_processing_only);
    // op_b is unrelated state and its latch is still false.
    assert!(!op_b.hub_region_processing_only);
    // Driving op_b independently sets its own latch but op_a_after is unchanged.
    let op_b_after = session_retry_state_for_1002(&op_b);
    assert!(op_b_after.hub_region_processing_only);
    assert!(op_a_after.hub_region_processing_only);
}

/// T-9 — Latch survives mixed-cause retries (AC-2). Flip the latch on a
/// 1002, then drive a 503 that takes the failover path. `..self` in
/// `advance_failover` propagates the flag.
#[test]
fn hub_region_latch_survives_failover_after_latch() {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;

    let after_1002 = session_retry_state_for_1002(&state);
    assert!(after_1002.hub_region_processing_only);

    // Now drive a 503 read — should fail over. Latch must propagate.
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let (action, _) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error(StatusCode::ServiceUnavailable),
        &after_1002,
    );
    match action {
        OperationAction::FailoverRetry { new_state, .. } => {
            assert!(
                new_state.hub_region_processing_only,
                "latch must propagate through `..self` in advance_failover",
            );
        }
        OperationAction::Abort { .. } => {
            // Terminal abort path — the latch is on retry_state, which
            // the abort path doesn't expose. The structural argument
            // (every `advance_*` uses `..self`) still holds.
        }
        other => panic!("unexpected action for 503 after latch: {other:?}"),
    }
}

// ── 449 RetryWith ─────────────────────────────────────────────────────
//
// The handler-level tests below lock in the behavior described in
// `try_handle_retry_with`:
//
//   1. 449 alone triggers an in-region FailoverRetry with a non-zero delay.
//   2. The retry-with state is initialized on first 449 and advanced on
//      every subsequent 449 hit.
//   3. Once the cumulative-wait budget is exhausted, the 449 propagates
//      to the caller (no further retries scheduled).
//   4. is_region_confirming_status excludes 449 so deferred location
//      effects don't flush on RetryWith.

fn make_449_error() -> TransportResult {
    make_http_error_status(CosmosStatus::new(StatusCode::from(449u16)))
}

#[test]
fn retry_with_449_returns_in_region_retry_with_delay() {
    let op = make_read_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());

    let (action, effects) = recovery::classify_result(&op, &endpoint, make_449_error(), &state);
    match action {
        OperationAction::InRegionRetry { new_state, delay } => {
            // First retry: 10ms initial + [0,5)ms salt → strictly < 15ms.
            assert!(delay >= Duration::from_millis(10));
            assert!(delay < Duration::from_millis(15));
            let rw = new_state
                .retry_with_state
                .expect("first 449 must initialize retry_with_state");
            assert_eq!(rw.attempt_count, 1);
            // The delay must be recorded in the cumulative bookkeeping
            // verbatim — no drift between the budget-checked delay
            // and the cumulative delta.
            assert_eq!(rw.cumulative_delay, delay);
        }
        other => panic!("expected InRegionRetry, got {other:?}"),
    }
    // 449 is region-local — no MarkEndpointUnavailable / MarkPartitionUnavailable.
    assert!(
        effects.is_empty(),
        "449 must not emit location effects (in-region retry only)",
    );
}

#[test]
fn retry_with_449_advances_existing_retry_with_state() {
    let op = make_read_operation();
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    // Pretend we've already retried 3 times.
    state.retry_with_state = Some(RetryWithRetryState {
        attempt_count: 3,
        cumulative_delay: Duration::from_millis(100),
        ..RetryWithRetryState::new()
    });

    let (action, _) = recovery::classify_result(&op, &endpoint, make_449_error(), &state);
    match action {
        OperationAction::InRegionRetry { new_state, .. } => {
            let rw = new_state.retry_with_state.expect("state must persist");
            assert_eq!(rw.attempt_count, 4);
        }
        other => panic!("expected InRegionRetry, got {other:?}"),
    }
}

#[test]
fn retry_with_449_aborts_when_cumulative_budget_exhausted() {
    let op = make_read_operation();
    let endpoint =
        CosmosEndpoint::global(url::Url::parse("https://test.documents.azure.com:443/").unwrap());
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    // Budget already fully spent — next delay can't fit.
    state.retry_with_state = Some(RetryWithRetryState {
        attempt_count: 11,
        cumulative_delay: Duration::from_secs(30),
        ..RetryWithRetryState::new()
    });

    let (action, _) = recovery::classify_result(&op, &endpoint, make_449_error(), &state);
    // Falls through to Abort (no other handler claims 449).
    assert!(
        matches!(action, OperationAction::Abort { .. }),
        "exhausted RetryWith budget must surface 449 to the caller, got {action:?}",
    );
}

#[test]
fn is_region_confirming_status_excludes_449() {
    // 449 RetryWith is a same-region retry trigger — the request never
    // completed and deferred write-path effects must NOT flush.
    let status_449 = CosmosStatus::new(StatusCode::from(449u16));
    assert!(
        !routing::is_region_confirming_status(&status_449),
        "449 RetryWith must not be region-confirming (else deferred effects would flush mid-retry)",
    );

    // Sanity: 200 still confirms, 503 still does not.
    assert!(routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::Ok
    )));
    assert!(!routing::is_region_confirming_status(&CosmosStatus::new(
        StatusCode::ServiceUnavailable
    )));
}

#[test]
fn try_handle_retry_with_returns_none_for_non_449_status() {
    // Sanity: only 449 triggers this policy; every other status must
    // fall through so other handlers see it.
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    for status in [
        CosmosStatus::new(StatusCode::Ok),
        CosmosStatus::new(StatusCode::ServiceUnavailable),
        CosmosStatus::new(StatusCode::TooManyRequests),
        CosmosStatus::new(StatusCode::Gone),
        CosmosStatus::new(StatusCode::RequestTimeout),
    ] {
        assert!(
            recovery::retry_with::recover(&state, &status).is_none(),
            "non-449 status {status:?} must not be claimed by try_handle_retry_with",
        );
    }
}

// ── Hub region caching ────────────────────────────────────────────

/// Helper: builds an OperationRetryState configured as if the
/// hub-region-processing-only latch is currently set for a single-master
/// data-plane read with a known partition key range ID.
fn state_with_hub_latch(pk_range_id: &str) -> OperationRetryState {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    state.hub_region_processing_only = true;
    state.partition_key_range_id = Some(pk_range_id.parse().unwrap());
    state
}

/// Hub-region branch in `try_handle_write_forbidden`: when the latch is
/// set, the request is read-only, and a partition key range ID is known,
/// the 403/3 emits an `AdvanceHubRegionDiscovery` effect (and only that
/// effect) — no `MarkEndpointUnavailable` or
/// `RefreshAccountProperties`, since the endpoint is healthy for non-hub
/// reads of other partitions and the topology is unchanged.
#[test]
fn read_403_3_with_hub_latch_emits_advance_effect() {
    let op = make_read_operation();
    let state = state_with_hub_latch("0");
    let endpoint = test_endpoint();
    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert_eq!(effects.len(), 1, "exactly one effect: advance discovery");
    assert!(matches!(
        effects[0],
        LocationEffect::AdvanceHubRegionDiscovery { .. }
    ));
    // Defense-in-depth: confirm the standard 403/3 effects are NOT present.
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
}

/// Regression: 403/3 for a write operation must keep emitting the existing
/// write-forbidden effects (`RefreshAccountProperties`,
/// `MarkEndpointUnavailable`, `MarkPartitionUnavailable`), even if the
/// retry state happens to carry the hub-region latch (which it wouldn't
/// in production for a write — defense in depth).
#[test]
fn write_403_3_with_hub_latch_keeps_existing_behavior() {
    let op = make_create_operation();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    state.hub_region_processing_only = true; // defense in depth
    state.partition_key_range_id = Some("0".parse().unwrap());
    let endpoint = test_endpoint();

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::AdvanceHubRegionDiscovery { .. })));
}

/// Regression: 403/3 for a read without the hub latch must keep emitting
/// the existing effects unchanged.
#[test]
fn read_403_3_without_hub_latch_keeps_existing_behavior() {
    let op = make_read_operation();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let endpoint = test_endpoint();

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)));
    assert!(effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!effects
        .iter()
        .any(|e| matches!(e, LocationEffect::AdvanceHubRegionDiscovery { .. })));
}

/// Hub-region read with the latch active but no partition key range ID
/// falls back to a plain failover retry with no location effects, and
/// never panics. (Validates #618.)
#[test]
fn read_403_3_with_hub_latch_no_pk_range_id_falls_back_gracefully() {
    let op = make_read_operation();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    state.hub_region_processing_only = true;
    // partition_key_range_id intentionally left as None.

    let endpoint = test_endpoint();
    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    // Graceful fallback: failover retry with no effects (no panic).
    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(
        effects.is_empty(),
        "missing pk_range_id must not emit any location effects, got {effects:?}",
    );
}

/// Hub-region 403/3 read aborts once the failover budget is exhausted
/// rather than retrying indefinitely. (Validates #579.)
#[test]
fn read_403_3_with_hub_latch_aborts_when_failover_budget_exhausted() {
    let op = make_read_operation();
    let mut state = state_with_hub_latch("0");
    state.failover_retry_count = state.max_failover_retries;
    let endpoint = test_endpoint();

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(
        matches!(action, OperationAction::Abort { .. }),
        "hub-region 403/3 must abort once the failover budget is exhausted, got {action:?}",
    );
    // No routing-state mutations on the terminal abort.
    assert!(
        effects.is_empty(),
        "aborted hub-region 403/3 must emit no effects"
    );
}

/// Defense-in-depth (multi-master invariant): even if the hub-region
/// latch is somehow set on a multi-master account, a 403/3 read must NOT
/// enter the single-master hub-region branch. It falls through to the
/// multi-write budget, emitting the standard write-forbidden effects
/// (`RefreshAccountProperties`) rather than `AdvanceHubRegionDiscovery`.
#[test]
fn read_403_3_with_hub_latch_on_multi_master_skips_hub_branch() {
    let op = make_read_operation();
    // Multi-master account: can_use_multiple_write_locations = true.
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3);
    state.is_dataplane = true;
    state.hub_region_processing_only = true; // stray latch, must be ignored
    state.partition_key_range_id = Some("0".parse().unwrap());
    let endpoint = test_endpoint();

    let (action, effects) = recovery::classify_result(
        &op,
        &endpoint,
        make_http_error_status(crate::error::status_codes::WRITE_FORBIDDEN),
        &state,
    );

    assert!(matches!(action, OperationAction::FailoverRetry { .. }));
    assert!(
        !effects
            .iter()
            .any(|e| matches!(e, LocationEffect::AdvanceHubRegionDiscovery { .. })),
        "multi-master 403/3 must not take the hub-region branch",
    );
    assert!(
        effects
            .iter()
            .any(|e| matches!(e, LocationEffect::RefreshAccountProperties)),
        "multi-master 403/3 must take the standard write-forbidden path",
    );
}

/// Classification may propose a latch but must leave its input snapshot unchanged.
#[test]
fn session_retry_proposal_does_not_mutate_input() {
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    let after = session_retry_state_for_1002(&state);
    assert!(after.hub_region_processing_only);
    assert_eq!(after.session_token_retry_count, 1);
    assert!(!state.hub_region_processing_only);
    assert_eq!(state.session_token_retry_count, 0);
}

// ────────────────────────────────────────────────────────────────
// `evaluate_hedge_leg_effects` — non-consuming hedge-leg evaluator
// ────────────────────────────────────────────────────────────────
//
// Mirrors the same `(operation, endpoint, retry_state, result)`
// tuple as `evaluate_transport_result` and emits the same
// `LocationEffect`s, minus the consumed `OperationAction`. These
// tests pin that mirror so a future refactor of either function
// can't silently diverge their side-effect surface.

/// Success transport result emits no effects and no session-retry
/// signal — same as `evaluate_transport_result`.
#[test]
fn hedge_leg_effects_success_is_empty() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = make_success_result();

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(eval.effects.is_empty());
    assert!(!eval.observed_session_unavailable);
}

/// 503 ServiceUnavailable on a read emits `MarkPartitionUnavailable`
/// + `MarkEndpointUnavailable` — exactly what the non-hedged path
/// would have applied.
#[test]
fn hedge_leg_effects_503_emits_partition_and_endpoint_marks() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = make_http_error(StatusCode::ServiceUnavailable);

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(eval
        .effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(eval
        .effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })));
    assert!(!eval.observed_session_unavailable);
}

/// Transport error with `RequestSentStatus::Sent` means the endpoint was
/// reachable but this partition had an issue — emits `MarkPartitionUnavailable`
/// ONLY (no endpoint mark), matching the sent branch of
/// `evaluate_transport_layer_outcome`.
#[test]
fn hedge_leg_effects_transport_sent_marks_partition_only() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = make_transport_error(RequestSentStatus::Sent);

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(eval
        .effects
        .iter()
        .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(
        !eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
        "sent transport error must not mark the endpoint — siblings are unaffected",
    );
}

/// Transport error with `RequestSentStatus::NotSent` is an endpoint-wide
/// failure — emits `MarkEndpointUnavailable` (and no partition mark, so
/// PPCB's partition counter is not inflated), matching the
/// `definitely_not_sent` branch in `evaluate_transport_layer_outcome`.
/// This is what drives G2 fail-fast: an unreachable thin-client proxy
/// rotates its region out instead of being retried.
#[test]
fn hedge_leg_effects_transport_not_sent_marks_endpoint() {
    let op = make_create_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = make_transport_error(RequestSentStatus::NotSent);

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(
        eval.effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkEndpointUnavailable { .. })),
        "not-sent transport error must mark the endpoint unavailable",
    );
    assert!(
        !eval
            .effects
            .iter()
            .any(|e| matches!(e, LocationEffect::MarkPartitionUnavailable(_))),
        "not-sent transport error must not inflate the partition counter",
    );
}

/// 404/1002 on a single-master data-plane read with budget remaining
/// surfaces `observed_session_unavailable=true` AND emits no
/// effects (matches `try_handle_read_session_not_available`'s
/// empty-effects contract). The hedge race coordinator uses this
/// signal to flip the parent `hub_region_processing_only` latch at
/// the `BothTransient` upgrade boundary.
#[test]
fn hedge_leg_effects_1002_signals_session_unavailable() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    let result = make_read_session_not_available_result();

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(eval.effects.is_empty());
    assert!(
        eval.observed_session_unavailable,
        "first 1002 on single-master dataplane should signal session-unavailable",
    );
}

/// 404/1002 on a multi-master account does NOT signal
/// `observed_session_unavailable` — matches
/// `build_session_retry_state`'s 4-condition trigger (AC-4
/// per Spec 0010: Hub-region processing header, §7.1).
#[test]
fn hedge_leg_effects_1002_multi_master_no_signal() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let mut state = OperationRetryState::initial(0, true, Vec::new(), 3, 3); // multi-master
    state.is_dataplane = true;
    let result = make_read_session_not_available_result();

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(
        !eval.observed_session_unavailable,
        "multi-master 1002 must not flip the latch (AC-4)",
    );
}

/// Already-latched state does not re-signal `observed_session_unavailable`
/// — idempotency boundary matching the non-hedged path.
#[test]
fn hedge_leg_effects_1002_no_signal_when_already_latched() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let mut state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    state.is_dataplane = true;
    state.hub_region_processing_only = true;
    let result = make_read_session_not_available_result();

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(
        !eval.observed_session_unavailable,
        "already-latched state should not re-signal",
    );
}

/// DeadlineExceeded transport result emits no effects — matches
/// `evaluate_deadline_exceeded_outcome`.
#[test]
fn hedge_leg_effects_deadline_exceeded_is_empty() {
    let op = make_read_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = TransportResult {
        outcome: TransportOutcome::DeadlineExceeded {
            request_sent: RequestSentStatus::Sent,
        },
    };

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(eval.effects.is_empty());
    assert!(!eval.observed_session_unavailable);
}

/// Final HTTP errors that aren't classified by any per-status helper
/// (e.g. 409 Conflict) emit no effects from the hedge-leg
/// evaluator — same as the consuming path's `Abort` fallthrough.
#[test]
fn hedge_leg_effects_409_conflict_is_empty() {
    let op = make_create_operation();
    let endpoint = test_endpoint();
    let state = OperationRetryState::initial(0, false, Vec::new(), 3, 3);
    let result = make_http_error(StatusCode::Conflict);

    let eval = recovery::hedge::evaluate_hedge_leg_effects(&op, &endpoint, &state, &result);
    assert!(
        eval.effects.is_empty(),
        "409 Conflict has no per-status handler; emits no effects",
    );
}

/// The service's explanation is folded into the error message so a bare
/// `{err}` log line says *why* the request was rejected, not just
/// `HTTP 400: Unknown`.
#[test]
fn service_error_message_includes_service_detail() {
    let body = br#"{"code":"BadRequest","message":"Message: {\"Errors\":[\"The retention duration in the Change Feed policy should not be set when continuous backup mode is enabled for the database account.\"]}\r\nActivityId: 36327452-0aa6-41fa-8f85-491f9755c870, Request URI: /apps/x/services/y, RequestStats: , SDK: Microsoft.Azure.Documents.Common/2.14.0"}"#;
    let err = recovery::error::build_service_error(
        &CosmosStatus::from_parts(StatusCode::BadRequest, None),
        &CosmosResponseHeaders::default(),
        body,
    );

    let rendered = err.to_string();
    assert!(
        rendered.contains(
            "The retention duration in the Change Feed policy should not be set \
             when continuous backup mode is enabled for the database account."
        ),
        "expected the service explanation in {rendered}"
    );
    // The activity ID is already typed state on the error, and the request
    // URI / SDK banner name service-internal replicas — the trailer stays
    // in the raw body rather than bloating the message.
    assert!(
        !rendered.contains("Request URI"),
        "expected the request trailer to be stripped from {rendered}"
    );
}

#[test]
fn service_body_detail_falls_back_to_raw_text() {
    assert_eq!(
        Some("not json at all".to_string()),
        recovery::error::service_body_detail(b"not json at all")
    );
    assert_eq!(None, recovery::error::service_body_detail(b""));
    assert_eq!(None, recovery::error::service_body_detail(b"   "));
}

#[test]
fn service_body_detail_truncates_long_payloads() {
    let long = "x".repeat(MAX_SERVICE_DETAIL_LEN * 2);
    let body = serde_json::json!({ "code": "BadRequest", "message": long }).to_string();

    let detail = recovery::error::service_body_detail(body.as_bytes()).expect("detail");
    // The ellipsis is counted against the budget, so the cap is never exceeded.
    assert_eq!(MAX_SERVICE_DETAIL_LEN, detail.len());
    assert!(detail.ends_with("..."));
}

/// Truncating must not split a multi-byte character, and the result must
/// still respect the byte cap.
#[test]
fn service_body_detail_truncates_on_char_boundary() {
    // 'é' is two bytes, so the naive cut point lands mid-character.
    let long = "é".repeat(MAX_SERVICE_DETAIL_LEN);
    let body = serde_json::json!({ "code": "BadRequest", "message": long }).to_string();

    let detail = recovery::error::service_body_detail(body.as_bytes()).expect("detail");
    assert!(detail.len() <= MAX_SERVICE_DETAIL_LEN);
    assert!(detail.ends_with("..."));
    assert!(detail.trim_end_matches('.').chars().all(|c| c == 'é'));
}

/// The body is service-controlled, so newlines and other control characters
/// must not reach `CosmosError`'s single-line `Display` — otherwise a
/// crafted payload could forge or mangle log records.
#[test]
fn service_body_detail_normalizes_control_characters() {
    let body = serde_json::json!({
        "code": "BadRequest",
        "message": "first line\r\nERROR: forged\tentry\u{7f}\u{1}  and   spaces  ",
    })
    .to_string();

    let detail = recovery::error::service_body_detail(body.as_bytes()).expect("detail");
    assert_eq!("first line ERROR: forged entry and spaces", detail);
    assert!(!detail.chars().any(char::is_control));
}

/// The same normalization applies on the raw-text fallback path, where the
/// body never went through JSON parsing.
#[test]
fn service_body_detail_normalizes_raw_text_fallback() {
    let detail =
        recovery::error::service_body_detail(b"not json\nsecond line\r\nthird").expect("detail");
    assert_eq!("not json second line third", detail);
}

/// A body that is nothing but control characters carries no usable text.
#[test]
fn service_body_detail_rejects_control_only_body() {
    assert_eq!(None, recovery::error::service_body_detail(b"\r\n\t  \x01"));
}

/// A body with no `message` field (or an unexpected shape) still surfaces
/// verbatim rather than being dropped.
#[test]
fn service_body_detail_keeps_unrecognized_json() {
    assert_eq!(
        Some(r#"{"code":"BadRequest"}"#.to_string()),
        recovery::error::service_body_detail(br#"{"code":"BadRequest"}"#)
    );
}
