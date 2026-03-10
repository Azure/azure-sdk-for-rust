// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation pipeline: the core loop for executing Cosmos DB operations.
//!
//! Implements the 7-stage operation loop with multi-region failover,
//! session retry, endpoint unavailability tracking, and deadline
//! enforcement. No hedging or circuit breaker yet (planned for later steps).

use std::sync::Arc;
use std::time::{Duration, Instant};

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use azure_core::http::HttpClient;

use crate::{
    diagnostics::{DiagnosticsContextBuilder, ExecutionContext, PipelineType, TransportSecurity},
    driver::routing::{
        AccountEndpointState, CosmosEndpoint, LocationSnapshot, LocationStateStore,
    },
    models::{
        ActivityId, CosmosOperation, CosmosResponse, CosmosResponseHeaders, Credential,
        SubStatusCode,
    },
    options::{OperationOptions, RuntimeOptions},
};

use super::{
    components::{
        OperationAction, OperationRetryState, RoutingDecision, TransportOutcome, TransportRequest,
        TransportResult,
    },
    retry_evaluation::evaluate_transport_result,
};

use crate::driver::transport::{
    transport_pipeline::execute_transport_pipeline, AuthorizationContext,
};

/// Executes a Cosmos DB operation through the new pipeline architecture.
///
/// This is the entry point called by `CosmosDriver::execute_operation`.
/// It orchestrates the 7-stage operation loop described in the spec.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    _options: &OperationOptions,
    effective_options: &RuntimeOptions,
    location_state_store: &LocationStateStore,
    http_client: Arc<dyn HttpClient>,
    credential: &Credential,
    user_agent: &azure_core::http::headers::HeaderValue,
    activity_id: &ActivityId,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    let mut diagnostics = diagnostics;
    let location_snapshot = location_state_store.snapshot();
    let max_failover_retries = effective_options.max_failover_retry_count.unwrap_or(3);
    let max_session_retries = effective_options.max_session_retry_count.unwrap_or_else(|| {
        // Java SDK parity: 2 for single-write, endpoints.len() for multi-write.
        // Uses the original endpoint count (before unavailability filtering).
        if location_snapshot.account.multiple_write_locations_enabled {
            let endpoints_len = location_snapshot
                .account
                .preferred_endpoints(operation.is_read_only())
                .len();
            endpoints_len as u32
        } else {
            2
        }
    });

    let mut retry_state = OperationRetryState::initial(
        location_snapshot.account.generation,
        location_snapshot.account.multiple_write_locations_enabled,
        effective_options
            .excluded_regions
            .as_ref()
            .map(|r| r.0.clone())
            .unwrap_or_default(),
        max_failover_retries,
        max_session_retries,
    );

    let deadline = effective_options
        .end_to_end_latency_policy
        .as_ref()
        .map(|p| Instant::now() + p.timeout());

    loop {
        // ── STAGE 1: Acquire LocationSnapshot ──────────────────────────
        let location = location_state_store.snapshot();

        // ── STAGE 2: Resolve endpoint ──────────────────────────────────
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            &location,
            location_state_store.endpoint_unavailability_ttl(),
        );

        // ── STAGE 3: Build transport request ───────────────────────────
        let execution_context = if retry_state.failover_retry_count == 0
            && retry_state.session_token_retry_count == 0
        {
            ExecutionContext::Initial
        } else if retry_state.session_token_retry_count > 0 {
            ExecutionContext::Retry
        } else {
            ExecutionContext::RegionFailover
        };

        let transport_request = build_transport_request(
            operation,
            &routing,
            activity_id,
            execution_context,
            deadline,
        )?;

        // ── STAGE 4: Execute via transport pipeline ────────────────────
        let result = execute_transport_pipeline(
            transport_request,
            http_client.as_ref(),
            credential,
            user_agent,
            pipeline_type,
            transport_security,
            &mut diagnostics,
        )
        .await;

        // ── STAGE 5: Evaluate result → action ──────────────────────────
        let (action, effects) =
            evaluate_transport_result(operation, &routing.endpoint, result, &retry_state);

        // ── STAGE 6: Apply location effects ────────────────────────────
        location_state_store.apply(&effects).await;

        // ── STAGE 7: Act on the control-flow decision ──────────────────
        match action {
            OperationAction::Complete(result) => {
                return build_cosmos_response(result, diagnostics);
            }
            OperationAction::FailoverRetry { new_state, delay } => {
                if let Some(delay) = delay {
                    if let Ok(duration) = azure_core::time::Duration::try_from(delay) {
                        azure_core::sleep(duration).await;
                    }
                }

                let endpoints_len = preferred_endpoints_for_attempt(
                    location.account.as_ref(),
                    &new_state,
                    operation.is_read_only(),
                )
                .len();

                retry_state = new_state.advance_location(endpoints_len);

                // Check deadline before retrying
                if let Some(d) = deadline {
                    if Instant::now() >= d {
                        let timeout_duration = effective_options
                            .end_to_end_latency_policy
                            .as_ref()
                            .map(|p| p.timeout())
                            .unwrap_or_default();

                        diagnostics.set_operation_status(
                            azure_core::http::StatusCode::RequestTimeout,
                            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                        );
                        return Err(azure_core::Error::new(
                            azure_core::error::ErrorKind::Other,
                            format!("end-to-end operation timeout exceeded ({timeout_duration:?})"),
                        ));
                    }
                }
            }
            OperationAction::SessionRetry { new_state } => {
                let endpoints_len = preferred_endpoints_for_attempt(
                    location.account.as_ref(),
                    &new_state,
                    operation.is_read_only(),
                )
                .len();
                retry_state = new_state.advance_location(endpoints_len);

                // Check deadline before retrying
                if let Some(d) = deadline {
                    if Instant::now() >= d {
                        let timeout_duration = effective_options
                            .end_to_end_latency_policy
                            .as_ref()
                            .map(|p| p.timeout())
                            .unwrap_or_default();

                        diagnostics.set_operation_status(
                            azure_core::http::StatusCode::RequestTimeout,
                            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                        );
                        return Err(azure_core::Error::new(
                            azure_core::error::ErrorKind::Other,
                            format!("end-to-end operation timeout exceeded ({timeout_duration:?})"),
                        ));
                    }
                }
            }
            OperationAction::Abort { error, status } => {
                if let Some(cosmos_status) = status {
                    diagnostics.set_operation_status(
                        cosmos_status.status_code(),
                        cosmos_status.sub_status(),
                    );
                }
                return Err(error);
            }
        }
    }
}

/// Resolves the endpoint for this attempt.
///
/// Step 2 resolves from `LocationSnapshot` and `AccountEndpointState`.
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    let account = location.account.as_ref();
    let endpoints = preferred_endpoints_for_attempt(account, retry_state, operation.is_read_only());

    let base_index = if retry_state.location.is_current(account.generation) {
        retry_state.location.index()
    } else {
        0
    };

    let now = Instant::now();
    let mut selected = None;
    let len = endpoints.len();
    for i in 0..len {
        let candidate = &endpoints[(base_index + i) % len];
        let excluded = candidate
            .region()
            .is_some_and(|r| retry_state.excluded_regions.iter().any(|e| e == r));
        let unavailable = account
            .unavailable_endpoints
            .get(candidate)
            .is_some_and(|(marked_at, _)| {
                now.saturating_duration_since(*marked_at) < endpoint_unavailability_ttl
            });
        if !excluded && !unavailable {
            selected = Some(candidate.clone());
            break;
        }
    }

    let selected = selected.unwrap_or_else(|| account.default_endpoint.clone());

    RoutingDecision { endpoint: selected }
}

fn preferred_endpoints_for_attempt<'a>(
    account: &'a AccountEndpointState,
    retry_state: &OperationRetryState,
    read_only: bool,
) -> &'a [CosmosEndpoint] {
    if read_only && retry_state.route_reads_to_write_endpoints() {
        &account.preferred_write_endpoints
    } else {
        account.preferred_endpoints(read_only)
    }
}

/// Builds a `TransportRequest` from the operation and routing decision.
fn build_transport_request(
    operation: &CosmosOperation,
    routing: &RoutingDecision,
    activity_id: &ActivityId,
    execution_context: ExecutionContext,
    deadline: Option<Instant>,
) -> azure_core::Result<TransportRequest> {
    let resource_ref = operation.resource_reference();
    let request_path = resource_ref.request_path();
    let url = {
        let mut base = routing.endpoint.url().clone();
        let normalized = if request_path.starts_with('/') {
            request_path.to_string()
        } else if request_path.is_empty() {
            String::new()
        } else {
            format!("/{}", request_path)
        };
        base.set_path(&normalized);
        base
    };

    let method = operation.operation_type().http_method();
    let resource_type = operation.resource_type();
    let resource_link = resource_ref.link_for_signing();
    let signing_link = resource_link.trim_start_matches('/');

    let auth_context = AuthorizationContext::new(method, resource_type, signing_link);

    // Build headers from the operation
    let mut headers = azure_core::http::headers::Headers::new();
    operation.request_headers().write_to_headers(&mut headers);

    // Add activity ID if not already set by the operation
    if operation.request_headers().activity_id.is_none() {
        headers.insert(
            HeaderName::from_static("x-ms-activity-id"),
            HeaderValue::from(activity_id.as_str().to_owned()),
        );
    }

    // Add partition key headers
    if let Some(pk) = operation.partition_key() {
        let pk_headers = pk.as_headers()?;
        for (name, value) in pk_headers {
            headers.insert(name, value);
        }
    }

    Ok(TransportRequest {
        method,
        endpoint: routing.endpoint.clone(),
        url,
        headers,
        body: operation.body().map(azure_core::Bytes::copy_from_slice),
        auth_context,
        execution_context,
        deadline,
    })
}

/// Builds a `CosmosResponse` from a successful `TransportResult`.
fn build_cosmos_response(
    result: TransportResult,
    mut diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    match result.outcome {
        TransportOutcome::Success {
            status,
            headers,
            body,
        } => {
            let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
            diagnostics.set_operation_status(status.status_code(), status.sub_status());

            let diagnostics_ctx = Arc::new(diagnostics.complete());

            Ok(CosmosResponse::new(
                body,
                cosmos_headers,
                status,
                diagnostics_ctx,
            ))
        }
        _ => {
            // This should only be called with a Complete(Success) result
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "build_cosmos_response called with non-success result",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use azure_core::http::headers::HeaderName;
    use url::Url;

    use super::build_transport_request;
    use crate::{
        diagnostics::ExecutionContext,
        driver::{
            pipeline::components::RoutingDecision,
            routing::{AccountEndpointState, CosmosEndpoint, LocationIndex, LocationSnapshot},
        },
        models::{
            AccountReference, ActivityId, ContainerProperties, ContainerReference, CosmosOperation,
            DatabaseReference, ItemReference, PartitionKey, PartitionKeyDefinition,
            SystemProperties,
        },
    };

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &test_container_props(),
        )
    }

    fn test_routing() -> RoutingDecision {
        RoutingDecision {
            endpoint: CosmosEndpoint::global(
                Url::parse("https://test.documents.azure.com:443/").unwrap(),
            ),
        }
    }

    #[test]
    fn build_transport_request_feed_path_is_resolved() {
        let operation = CosmosOperation::read_all_databases(test_account());

        let request = build_transport_request(
            &operation,
            &test_routing(),
            &ActivityId::from_string("default-activity".to_string()),
            ExecutionContext::Initial,
            None,
        )
        .expect("request should build");

        assert_eq!(request.url.path(), "/dbs");
    }

    #[test]
    fn build_transport_request_single_resource_path_is_resolved() {
        let db = DatabaseReference::from_name(test_account(), "mydb");
        let operation = CosmosOperation::read_database(db);

        let request = build_transport_request(
            &operation,
            &test_routing(),
            &ActivityId::from_string("default-activity".to_string()),
            ExecutionContext::Initial,
            None,
        )
        .expect("request should build");

        assert_eq!(request.url.path(), "/dbs/mydb");
    }

    #[test]
    fn build_transport_request_uses_operation_activity_id_when_present() {
        let operation = CosmosOperation::read_all_databases(test_account())
            .with_activity_id(ActivityId::from_string("operation-activity".to_string()));

        let request = build_transport_request(
            &operation,
            &test_routing(),
            &ActivityId::from_string("default-activity".to_string()),
            ExecutionContext::Initial,
            None,
        )
        .expect("request should build");

        let activity_header = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-activity-id"))
            .expect("activity id should be set");
        assert_eq!(activity_header, "operation-activity");
    }

    #[test]
    fn build_transport_request_adds_partition_key_header_for_item_operation() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
        let operation = CosmosOperation::read_item(item_ref);

        let request = build_transport_request(
            &operation,
            &test_routing(),
            &ActivityId::from_string("default-activity".to_string()),
            ExecutionContext::Retry,
            Some(std::time::Instant::now() + Duration::from_secs(5)),
        )
        .expect("request should build");

        let partition_key_header = request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-documentdb-partitionkey"))
            .expect("partition key header should be set");
        assert_eq!(partition_key_header, "[\"pk1\"]");
    }

    #[test]
    fn resolve_endpoint_uses_write_region_for_single_write_session_retry() {
        let operation = CosmosOperation::read_all_databases(test_account());
        let write_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let read_endpoint = CosmosEndpoint::regional(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        );

        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
                generation: 0,
                preferred_read_endpoints: vec![read_endpoint],
                preferred_write_endpoints: vec![write_endpoint.clone()],
                unavailable_endpoints: Default::default(),
                multiple_write_locations_enabled: false,
                default_endpoint: write_endpoint.clone(),
            }));

        let retry_state = crate::driver::pipeline::components::OperationRetryState {
            location: LocationIndex::initial(0),
            failover_retry_count: 0,
            session_token_retry_count: 1,
            max_failover_retries: 3,
            max_session_retries: 2,
            can_use_multiple_write_locations: false,
            excluded_regions: Vec::new(),
            session_retry_routing:
                crate::driver::pipeline::components::SessionRetryRouting::PreferredWriteEndpoints,
        };

        let routing = super::resolve_endpoint(&operation, &retry_state, &location, Duration::from_secs(60));
        assert_eq!(routing.endpoint, write_endpoint);
    }
}
