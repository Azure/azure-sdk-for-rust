// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{sync::Arc, time::Duration};

use azure_core::http::headers::{HeaderName, HeaderValue};
use url::Url;

#[cfg(feature = "fault_injection")]
use crate::pipeline::transport::cosmos_transport_client::HttpRequest;
use crate::{
    diagnostics::{ExecutionContext, PipelineKind},
    driver::cache::ContainerRoutingMap,
    models::{
        request_header_names, AccountReference, ActivityId, ContainerProperties,
        ContainerReference, CosmosOperation, CosmosResponseHeaders, DatabaseReference,
        DefaultConsistencyLevel, EffectivePartitionKey, FeedRange, ItemReference, PartitionKey,
        PartitionKeyDefinition, PartitionKeyValue, SessionToken, SystemProperties,
    },
    options::{PriorityLevel, Region, ResolvedThroughputControl},
    pipeline::{
        operation::{
            dispatch, recovery,
            recovery::recreation,
            request,
            request::TransportRequestContext,
            routing,
            routing::{
                partition_key_range_id::PartitionKeyRangeId, AccountEndpointState, CosmosEndpoint,
                LocationEffect, LocationIndex, LocationSnapshot, RoutingDecision,
            },
            session,
            session::{manager::SessionManager, ResolvedSession},
            types::OperationOverrides,
        },
        transport::{
            types::TransportMode, wrap_request_for_gateway_v2, EndpointKey, GatewayV2Headers,
        },
    },
    rntbd::{
        tokens::{ReadFeedKeyType, RntbdRequestToken},
        RntbdRequestFrame, Token,
    },
};

fn test_account() -> AccountReference {
    AccountReference::with_account_key(
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

fn capture_session_token(manager: &SessionManager, operation: &CosmosOperation, token: &str) {
    manager.capture_session_token(
        operation,
        &CosmosResponseHeaders {
            session_token: Some(SessionToken::new(token.to_string())),
            owner_id: Some("testcontainer_rid".to_string()),
            owner_full_name: Some("dbs/testdb/colls/testcontainer".to_string()),
            ..Default::default()
        },
    );
}

/// Selects one representation from the shared resolver for focused session assertions.
fn resolved_session_token(
    manager: &SessionManager,
    operation: &CosmosOperation,
    mode: TransportMode,
    partition: Option<&PartitionKeyRangeId>,
    overrides: &OperationOverrides,
) -> Option<SessionToken> {
    let resolved = session::resolve(manager, operation, partition, overrides, true, false);
    match mode {
        TransportMode::Gateway => resolved.gateway,
        TransportMode::GatewayV2 => resolved.gateway_v2,
    }
}

fn test_session_topology(ranges: serde_json::Value) -> Arc<ContainerRoutingMap> {
    Arc::new(
        ContainerRoutingMap::try_create(serde_json::from_value(ranges).unwrap(), None, None)
            .unwrap()
            .unwrap(),
    )
}

#[test]
fn patch_verification_read_suppresses_both_hedging_entry_points() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let ordinary_read = CosmosOperation::read_item(item.clone());
    let patch_read = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let overrides = OperationOverrides::default();

    assert!(
        !crate::pipeline::operation::hedging::hedging_suppressed_for_attempt(
            &ordinary_read,
            &overrides
        )
    );
    assert!(
        crate::pipeline::operation::hedging::hedging_suppressed_for_attempt(
            &patch_read,
            &overrides
        )
    );
}

#[test]
fn automatic_session_token_resolution_respects_operation_and_topology() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let write = CosmosOperation::create_item(item.clone()).with_body(b"{}".to_vec());
    let read = CosmosOperation::read_item(item);
    let batch = CosmosOperation::batch(test_container(), PartitionKey::from("pk1"));

    assert_eq!(
        crate::pipeline::operation::session::read_consistency_strategy_for_operation(
            &write,
            crate::options::ReadConsistencyStrategy::Session,
        ),
        crate::options::ReadConsistencyStrategy::Default
    );
    assert_eq!(
        crate::pipeline::operation::session::read_consistency_strategy_for_operation(
            &read,
            crate::options::ReadConsistencyStrategy::Eventual,
        ),
        crate::options::ReadConsistencyStrategy::Eventual
    );
    assert!(
        crate::pipeline::operation::session::operation_allows_automatic_session_token_resolution(
            &read, false
        )
    );
    assert!(
        !crate::pipeline::operation::session::operation_allows_automatic_session_token_resolution(
            &write, false
        )
    );
    assert!(
        crate::pipeline::operation::session::operation_allows_automatic_session_token_resolution(
            &write, true
        )
    );
    assert!(
        crate::pipeline::operation::session::operation_allows_automatic_session_token_resolution(
            &batch, false
        )
    );
}

#[test]
fn patch_read_routing_hint_is_restored_after_generic_failover_reset() {
    use crate::pipeline::operation::recovery::state::SessionRetryRouting;

    let mut state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    )
    .advance_failover();
    assert_eq!(
        state.session_retry_routing,
        SessionRetryRouting::PreferredEndpoints
    );
    state.ppcb_active = true;

    recovery::state::apply_read_routing_strategy(&mut state, true);

    assert_eq!(
        state.session_retry_routing,
        SessionRetryRouting::PreferredWriteEndpoints
    );
    assert!(state.ppcb_active);
}

#[test]
fn patch_verification_read_suppresses_shared_location_effects() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let patch_read = CosmosOperation::read_item(item.clone()).as_patch_read_sub_operation();
    let ordinary_read = CosmosOperation::read_item(item);
    let endpoint = make_endpoint("eastus");
    let effects = vec![
        LocationEffect::MarkPartitionUnavailable(
            crate::pipeline::operation::routing::UnavailablePartition {
                partition_key_range_id: Some("0".parse().unwrap()),
                region: Some("eastus".into()),
                is_read: true,
                is_partitioned_resource: true,
            },
        ),
        LocationEffect::MarkEndpointUnavailable {
            endpoint,
            reason: crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        },
        LocationEffect::RefreshAccountProperties,
    ];

    let patch_effects =
        routing::location_effects_for_operation(&patch_read, effects.clone().into());
    assert_eq!(patch_effects.len(), 1);
    assert!(matches!(
        patch_effects[0],
        LocationEffect::RefreshAccountProperties
    ));

    let ordinary_effects = routing::location_effects_for_operation(&ordinary_read, effects.into());
    assert!(ordinary_effects
        .iter()
        .any(|effect| matches!(effect, LocationEffect::MarkPartitionUnavailable(_))));
    assert!(ordinary_effects
        .iter()
        .any(|effect| matches!(effect, LocationEffect::MarkEndpointUnavailable { .. })));
}

/// Supplying more partition-key components than the container's single-path
/// definition declares must surface as a typed client `BadRequest` from the
/// EPK precomputation, never silently hash the extras into a broken EPK.
/// This validation runs in the operation pipeline (before the transport
/// pipeline) so wire layers receive a ready-to-encode EPK.
#[test]
fn effective_partition_key_rejects_too_many_components() {
    let partition_key = PartitionKey::try_from(vec![
        PartitionKeyValue::from("tenant1".to_string()),
        PartitionKeyValue::from("extra".to_string()),
    ])
    .unwrap();
    let item = ItemReference::from_name(&test_container(), partition_key, "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let error =
        crate::pipeline::operation::request::effective_partition_key_for_request(&operation)
            .expect_err("too many components must error");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::BadRequest
    );
    assert_eq!(
        error.status().sub_status(),
        Some(crate::error::status_codes::substatus::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
    );
}

/// A single-component key matching the single-path definition yields a
/// point EPK (no error, `Some` value).
#[test]
fn effective_partition_key_for_matching_components() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let epk = crate::pipeline::operation::request::effective_partition_key_for_request(&operation)
        .expect("matching components must compute")
        .expect("partition key present yields Some");

    assert!(!epk.as_bytes().is_empty());
}

fn test_routing() -> RoutingDecision {
    let endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    RoutingDecision {
        endpoint,
        routing_fallback: None,
    }
}

/// Prepares a real operation request and applies dispatch's gateway decision without sending I/O.
pub(super) fn dispatched_request(
    operation: &CosmosOperation,
    route: &RoutingDecision,
    session: ResolvedSession,
    pipeline_type: PipelineKind,
    account_name_present: bool,
    gateway_v2_allowed: bool,
) -> (
    crate::pipeline::transport::types::TransportRequest,
    EndpointKey,
) {
    let activity_id = ActivityId::from_string("dispatch-test".to_owned());
    let request = request::prepare(
        operation,
        &OperationOverrides::default(),
        None,
        &TransportRequestContext {
            routing: route,
            activity_id: &activity_id,
            execution_context: ExecutionContext::Initial,
            deadline: None,
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            throughput_control: None,
        },
        false,
    )
    .unwrap();
    dispatch::prepare_transport(
        request,
        operation,
        session,
        pipeline_type,
        account_name_present,
        gateway_v2_allowed,
    )
}

#[test]
fn cached_session_downgrade_requires_unknown_physical_partition() {
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let physical_id: PartitionKeyRangeId = "0".parse().unwrap();

    for cached_token in [
        None,
        Some("0:1#100#1=10"),
        Some("0:1#100#1=10,1:1#200#1=20"),
    ] {
        for explicit_token in [false, true] {
            let manager = SessionManager::new();
            let mut operation = CosmosOperation::read_item(item.clone());
            if let Some(token) = cached_token {
                capture_session_token(&manager, &operation, token);
            }
            if explicit_token {
                operation = operation.with_session_token(SessionToken::new("0:1#50#1=5"));
            }

            for (lazy, active, logical, known_partition) in [
                (true, true, true, false),
                (false, true, true, false),
                (true, false, true, false),
                (true, true, false, false),
                (true, true, true, true),
            ] {
                let overrides = OperationOverrides {
                    logical_partition_key_target: logical,
                    ..Default::default()
                };
                let partition = known_partition.then_some(&physical_id);
                let resolved =
                    session::resolve(&manager, &operation, partition, &overrides, active, lazy);

                let downgraded = lazy
                    && active
                    && logical
                    && !known_partition
                    && cached_token.is_some()
                    && !explicit_token;
                assert_eq!(resolved.requires_gateway, downgraded);
                if downgraded {
                    assert_eq!(
                        resolved.gateway.as_ref().map(SessionToken::as_str),
                        cached_token
                    );
                }
                let (request, key) = dispatched_request(
                    &operation,
                    &routing::select(&endpoint),
                    resolved,
                    PipelineKind::DataPlane,
                    true,
                    true,
                );
                assert_eq!(
                    request.transport_mode,
                    if downgraded {
                        TransportMode::Gateway
                    } else {
                        TransportMode::GatewayV2
                    }
                );
                assert_eq!(
                    request.url.origin(),
                    endpoint.selected_url(!downgraded).origin()
                );
                assert_eq!(
                    key,
                    EndpointKey::try_from(endpoint.selected_url(!downgraded)).unwrap()
                );
            }
        }
    }
}

#[test]
fn resolved_logical_partition_identity_is_not_emitted_as_range_header() {
    let overrides = OperationOverrides {
        resolved_partition_key_range_id: Some("7".to_string()),
        resolved_partition_key_range_parents: vec!["6".to_string()],
        partition_key: Some(PartitionKey::from("pk")),
        ..Default::default()
    };
    let mut headers = azure_core::http::headers::Headers::new();

    overrides
        .apply_headers(&mut headers, false)
        .expect("apply_headers should succeed");

    assert_eq!(overrides.effective_partition_key_range_id(), Some("7"));
    assert_eq!(
        overrides.effective_partition_key_range_parents("7"),
        &["6".to_string()]
    );
    assert!(headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::PARTITION_KEY_RANGE_ID
        ))
        .is_none());
    assert!(headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::PARTITION_KEY
        ))
        .is_some());
}

#[test]
fn dispatch_selects_gateway_and_session_representation_together() {
    let original = "0:1#100#1=10,1:1#200#1=20";
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ))
    .with_session_token(original);
    let manager = SessionManager::new();
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        Region::WEST_US_2,
        Url::parse("https://test.documents.azure.com:443/").unwrap(),
        Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
    );
    let overrides = OperationOverrides {
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"0","minInclusive":"","maxExclusive":"80"},
            {"id":"1","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    let id: PartitionKeyRangeId = "0".parse().unwrap();

    for partition in [None, Some(&id)] {
        for (pipeline, account_known, allowed) in [
            (PipelineKind::DataPlane, true, true),
            (PipelineKind::Metadata, true, true),
            (PipelineKind::DataPlane, false, true),
            (PipelineKind::DataPlane, true, false),
        ] {
            let session =
                session::resolve(&manager, &operation, partition, &overrides, false, false);
            let (request, key) = dispatched_request(
                &operation,
                &routing::select(&endpoint),
                session,
                pipeline,
                account_known,
                allowed,
            );
            let v2 = partition.is_some() && pipeline.is_data_plane() && account_known && allowed;
            assert_eq!(
                request.transport_mode,
                if v2 {
                    TransportMode::GatewayV2
                } else {
                    TransportMode::Gateway
                }
            );
            assert_eq!(request.url.origin(), endpoint.selected_url(v2).origin());
            assert_eq!(
                key,
                EndpointKey::try_from(endpoint.selected_url(v2)).unwrap()
            );
            assert_eq!(
                request.headers.get_optional_str(&HeaderName::from_static(
                    request_header_names::SESSION_TOKEN
                )),
                Some(if v2 { "0:1#100#1=10" } else { original }),
            );
            assert_eq!(
                request.headers.get_optional_str(&HeaderName::from_static(
                    request_header_names::INTENDED_COLLECTION_RID
                )),
                if v2 {
                    None
                } else {
                    Some(operation.container().unwrap().rid())
                },
            );
        }
    }
}

#[test]
fn dispatch_uses_session_snapshot_after_cache_changes() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#100#1=10,1:1#200#1=20");
    let id: PartitionKeyRangeId = "0".parse().unwrap();
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        Region::WEST_US_2,
        Url::parse("https://test.documents.azure.com:443/").unwrap(),
        Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
    );

    let session = session::resolve(
        &manager,
        &operation,
        Some(&id),
        &OperationOverrides::default(),
        true,
        false,
    );
    assert_eq!(
        session.gateway.as_ref().unwrap().as_str(),
        "0:1#100#1=10,1:1#200#1=20"
    );
    assert_eq!(
        session.gateway_v2.as_ref().unwrap().as_str(),
        "0:1#100#1=10"
    );
    capture_session_token(&manager, &operation, "0:1#900#1=90");

    let (request, _) = dispatched_request(
        &operation,
        &routing::select(&endpoint),
        session,
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::SESSION_TOKEN
        )),
        Some("0:1#100#1=10"),
    );
    let next = session::resolve(
        &manager,
        &operation,
        Some(&id),
        &OperationOverrides::default(),
        true,
        false,
    );
    assert_eq!(next.gateway_v2.unwrap().as_str(), "0:1#900#1=90");
}

#[test]
fn dispatch_preserves_encoded_paths_and_custom_headers() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "literal%2F @+=",
    ));
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        Region::WEST_US_2,
        Url::parse("https://test.documents.azure.com:443/").unwrap(),
        Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
    );
    for allowed in [false, true] {
        let mut prepared = transport_request(&operation);
        prepared.endpoint = endpoint.clone();
        prepared.url.set_host(endpoint.url().host_str()).unwrap();
        let path = prepared.url.path().to_owned();
        assert!(path.contains("%252F%20"));
        prepared
            .headers
            .insert(request_header_names::SESSION_TOKEN, "custom-session");
        prepared
            .headers
            .insert(request_header_names::INTENDED_COLLECTION_RID, "custom-rid");

        let (request, _) = dispatch::prepare_transport(
            prepared,
            &operation,
            ResolvedSession::default(),
            PipelineKind::DataPlane,
            true,
            allowed,
        );
        assert_eq!(request.url.path(), path);
        assert_eq!(
            request.url.origin(),
            endpoint.selected_url(allowed).origin()
        );
        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                request_header_names::SESSION_TOKEN
            )),
            Some("custom-session"),
        );
        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                request_header_names::INTENDED_COLLECTION_RID
            )),
            Some(if allowed {
                "custom-rid"
            } else {
                operation.container().unwrap().rid()
            }),
        );
    }
}

#[test]
fn dispatch_keeps_patch_writer_checks_independent_of_cached_session() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ))
    .as_patch_read_sub_operation();
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#100#1=10");
    let session = session::resolve(
        &manager,
        &operation,
        None,
        &OperationOverrides::default(),
        true,
        false,
    );
    assert!(session.gateway.is_some());
    let (request, _) = dispatched_request(
        &operation,
        &test_routing(),
        session,
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::SESSION_TOKEN
        ),)
        .is_none());
}

#[test]
fn operation_pipeline_seeds_resolved_logical_partition_identity() {
    let overrides = OperationOverrides {
        resolved_partition_key_range_id: Some("7".to_string()),
        ..Default::default()
    };

    let resolved = session::partition_key_range_id_from_overrides(&overrides);

    assert_eq!(resolved.as_ref().map(|id| id.as_str()), Some("7"));
}

#[test]
fn routed_physical_identity_takes_precedence_over_internal_identity() {
    let overrides = OperationOverrides {
        partition_key_range_id: Some("routed".to_string()),
        resolved_partition_key_range_id: Some("internal".to_string()),
        ..Default::default()
    };

    let resolved = session::partition_key_range_id_from_overrides(&overrides);

    assert_eq!(resolved.as_ref().map(|id| id.as_str()), Some("routed"));
}

#[test]
fn classic_gateway_request_scopes_session_without_changing_logical_routing() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#100#1=10,1:1#200#1=20");
    let overrides = OperationOverrides {
        logical_partition_key_target: true,
        resolved_partition_key_range_id: Some("1".to_owned()),
        partition_key: Some(PartitionKey::from("pk")),
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"0","minInclusive":"","maxExclusive":"80"},
            {"id":"1","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    let range = session::partition_key_range_id_from_overrides(&overrides);
    let session = session::resolve(
        &manager,
        &operation,
        range.as_ref(),
        &overrides,
        true,
        false,
    );
    let routing = test_routing();
    let activity_id = ActivityId::from_string("session-scoping".to_owned());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request = request::build_transport_request(&operation, &overrides, None, &ctx).unwrap();
    let (request, _) = dispatch::prepare_transport(
        request,
        &operation,
        session,
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::SESSION_TOKEN
        )),
        Some("1:1#200#1=20"),
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::PARTITION_KEY
        )),
        Some(r#"["pk"]"#),
    );
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::PARTITION_KEY_RANGE_ID
        ),)
        .is_none());
}

#[test]
fn explicit_tokens_are_prepared_per_attempt_without_cached_progress() {
    let original = "0:1#10#1=9,1:1#20#2=19";
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ))
    .with_session_token(original);
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#900#1=899,1:1#800#2=799");
    let cached = manager
        .resolve_session_token(&operation, None, None)
        .unwrap();
    let overrides = OperationOverrides {
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"0","minInclusive":"","maxExclusive":"80"},
            {"id":"1","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    for automatic in [false, true] {
        for (mode, id, expected) in [
            (TransportMode::Gateway, "0", original),
            (TransportMode::GatewayV2, "0", "0:1#10#1=9"),
            (TransportMode::GatewayV2, "1", "1:1#20#2=19"),
            (TransportMode::Gateway, "1", original),
        ] {
            let id = PartitionKeyRangeId::from(id.to_owned());
            let resolved = session::resolve(
                &manager,
                &operation,
                Some(&id),
                &overrides,
                automatic,
                false,
            );
            assert!(!resolved.requires_gateway);
            let token = if mode == TransportMode::Gateway {
                resolved.gateway
            } else {
                resolved.gateway_v2
            }
            .unwrap();
            assert_eq!(token.as_str(), expected);
            assert_eq!(
                operation
                    .request_headers()
                    .session_token
                    .as_ref()
                    .unwrap()
                    .as_str(),
                original
            );
            assert_eq!(
                manager
                    .resolve_session_token(&operation, None, None)
                    .unwrap()
                    .as_str(),
                cached.as_str()
            );
        }
    }
}

#[test]
fn non_compound_tokens_and_disabled_automatic_resolution_keep_existing_behavior() {
    let manager = SessionManager::new();
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    capture_session_token(&manager, &operation, "0:1#900");
    let resolved = session::resolve(
        &manager,
        &operation,
        None,
        &OperationOverrides::default(),
        false,
        false,
    );
    assert!(resolved.gateway.is_none());
    assert!(resolved.gateway_v2.is_none());
    assert!(!resolved.requires_gateway);
    for original in ["0:1#10", "opaque", ""] {
        let operation = operation.clone().with_session_token(original);
        let resolved = session::resolve(
            &manager,
            &operation,
            None,
            &OperationOverrides::default(),
            false,
            false,
        );
        assert!(!resolved.requires_gateway);
        assert_eq!(resolved.gateway.unwrap().as_str(), original);
        assert_eq!(resolved.gateway_v2.unwrap().as_str(), original);
    }
}

#[test]
fn unresolved_explicit_scope_preserves_original_and_classic_endpoint() {
    let original = "0:1#10,1:1#20";
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ))
    .with_session_token(original);
    let manager = SessionManager::new();
    let id = PartitionKeyRangeId::from("0".to_owned());
    for automatic in [false, true] {
        let resolved = session::resolve(
            &manager,
            &operation,
            Some(&id),
            &OperationOverrides::default(),
            automatic,
            false,
        );
        assert!(resolved.requires_gateway);
        assert_eq!(resolved.gateway.as_ref().unwrap().as_str(), original);
        let endpoint = CosmosEndpoint::regional_with_gateway_v2(
            Region::WEST_US_2,
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
        );
        let (request, key) = dispatched_request(
            &operation,
            &routing::select(&endpoint),
            resolved,
            PipelineKind::DataPlane,
            true,
            true,
        );
        assert_eq!(request.transport_mode, TransportMode::Gateway);
        assert_eq!(request.url.origin(), endpoint.url().origin());
        assert_eq!(key, endpoint.endpoint_key());
        assert_eq!(
            request.headers.get_optional_str(&HeaderName::from_static(
                request_header_names::SESSION_TOKEN
            )),
            Some(original)
        );
    }
}

#[test]
fn range_requests_keep_exact_scope_without_logical_selectors_on_every_attempt() {
    let mut properties = test_container_props();
    properties.partition_key = serde_json::from_value(serde_json::json!({
        "paths": ["/tenant", "/user", "/session"], "kind": "MultiHash", "version": 2
    }))
    .unwrap();
    let container = ContainerReference::new(
        test_account(),
        "testdb",
        "testdb_rid",
        "testcontainer",
        "testcontainer_rid",
        &properties,
    );
    let prefix = FeedRange::for_partition(PartitionKey::from("tenant"), &properties.partition_key);
    let operation = CosmosOperation::query_items(container, Some(prefix.clone()));
    let overrides = OperationOverrides {
        partition_key_range_id: Some("7".into()),
        feed_range: Some(prefix.clone()),
        pkrange_bounds: Some(FeedRange::full()),
        continuation: Some("page-two".into()),
        ..Default::default()
    };
    let custom_headers = [(
        HeaderName::from_static(request_header_names::PARTITION_KEY),
        HeaderValue::from_static(r#"["stale"]"#),
    )]
    .into_iter()
    .collect();
    let activity_id = ActivityId::from_string("hpk-prefix".into());
    for mode in [TransportMode::GatewayV2, TransportMode::Gateway] {
        for execution_context in [ExecutionContext::Initial, ExecutionContext::OperationRetry] {
            let routing = routing::select(&CosmosEndpoint::regional_with_gateway_v2(
                Region::WEST_US_2,
                Url::parse("https://test.documents.azure.com:443/").unwrap(),
                Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
            ));
            let context = TransportRequestContext {
                routing: &routing,
                activity_id: &activity_id,
                execution_context,
                deadline: None,
                effective_consistency: DefaultConsistencyLevel::Session,
                read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
                throughput_control: None,
            };
            let request = request::build_transport_request(
                &operation,
                &overrides,
                Some(&custom_headers),
                &context,
            )
            .unwrap();
            let (request, _) = dispatch::prepare_transport(
                request,
                &operation,
                ResolvedSession {
                    gateway: Some(SessionToken::new("7:1#100")),
                    gateway_v2: Some(SessionToken::new("7:1#100")),
                    requires_gateway: false,
                },
                PipelineKind::DataPlane,
                true,
                mode == TransportMode::GatewayV2,
            );
            assert_eq!(request.transport_mode, mode);
            assert!(request.effective_partition_key.is_none());
            assert!(request
                .headers
                .get_optional_str(&HeaderName::from_static(
                    request_header_names::PARTITION_KEY
                ))
                .is_none());
            for (name, expected) in [
                (
                    request_header_names::START_EPK,
                    prefix.min_inclusive().to_hex(),
                ),
                (
                    request_header_names::END_EPK,
                    prefix.max_exclusive().to_hex(),
                ),
                (request_header_names::PARTITION_KEY_RANGE_ID, "7".into()),
                (request_header_names::CONTINUATION, "page-two".into()),
                (request_header_names::SESSION_TOKEN, "7:1#100".into()),
                (
                    request_header_names::READ_FEED_KEY_TYPE,
                    request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE.into(),
                ),
            ] {
                assert_eq!(
                    request
                        .headers
                        .get_optional_str(&HeaderName::from_static(name)),
                    Some(expected.as_str()),
                    "{name} on {mode:?}/{execution_context:?}",
                );
            }
        }
    }
}

#[test]
fn shared_attempt_resolver_scopes_each_range_and_transport() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    let composite = "0:1#100#1=10,1:1#200#1=20";
    capture_session_token(&manager, &operation, composite);
    let overrides = OperationOverrides {
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"0","minInclusive":"","maxExclusive":"80"},
            {"id":"1","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    for (mode, id, expected) in [
        (TransportMode::Gateway, "0", "0:1#100#1=10"),
        (TransportMode::GatewayV2, "1", "1:1#200#1=20"),
        (TransportMode::Gateway, "1", "1:1#200#1=20"),
    ] {
        let range = PartitionKeyRangeId::from(id.to_owned());
        let token =
            resolved_session_token(&manager, &operation, mode, Some(&range), &overrides).unwrap();
        assert_eq!(token.as_str(), expected, "{mode:?}");
    }
    assert_eq!(
        manager
            .resolve_session_token(&operation, None, None)
            .unwrap()
            .as_str(),
        composite
    );
}

#[test]
fn shared_attempt_resolver_preserves_fallback_when_range_cannot_be_scoped() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    let composite = "0:1#100#1=10,1:1#200#1=20";
    capture_session_token(&manager, &operation, composite);
    let unknown = PartitionKeyRangeId::from("unknown".to_owned());
    let overrides = OperationOverrides {
        logical_partition_key_target: true,
        resolved_partition_key_range_id: Some("unknown".to_owned()),
        resolved_partition_key_range_parents: vec!["uncached-parent".to_owned()],
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"0","minInclusive":"","maxExclusive":"80"},
            {"id":"1","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    for range in [None, Some(&unknown)] {
        assert_eq!(
            resolved_session_token(
                &manager,
                &operation,
                TransportMode::Gateway,
                range,
                &overrides
            )
            .unwrap()
            .as_str(),
            composite,
        );
        assert!(resolved_session_token(
            &manager,
            &operation,
            TransportMode::GatewayV2,
            range,
            &overrides,
        )
        .is_none());
    }
}

#[test]
fn shared_attempt_resolver_preserves_explicit_tokens() {
    let caller_token = "1:1#300#1=30,0:1#400#1=40";
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ))
    .with_session_token(SessionToken::new(caller_token));
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#100#1=10,1:1#200#1=20");
    let known = PartitionKeyRangeId::from("1".to_owned());
    let unknown = PartitionKeyRangeId::from("unknown".to_owned());
    let overrides = OperationOverrides {
        logical_partition_key_target: true,
        ..Default::default()
    };
    for range in [None, Some(&known), Some(&unknown)] {
        let resolved = session::resolve(&manager, &operation, range, &overrides, true, false);
        assert!(resolved.requires_gateway);
        assert_eq!(resolved.gateway.unwrap().as_str(), caller_token);
        assert!(resolved.gateway_v2.is_none());
    }
}

#[test]
fn shared_attempt_resolver_merges_parents_only_for_matching_identity() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    let composite = "other:1#900#1=90,parent_a:1#100#1=10,parent_b:1#200#2=20";
    capture_session_token(&manager, &operation, composite);
    let merged = PartitionKeyRangeId::from("merged".to_owned());
    let different = PartitionKeyRangeId::from("different".to_owned());
    let overrides = OperationOverrides {
        logical_partition_key_target: true,
        resolved_partition_key_range_id: Some("merged".to_owned()),
        resolved_partition_key_range_parents: vec!["parent_a".to_owned(), "parent_b".to_owned()],
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"merged","minInclusive":"","maxExclusive":"80","parents":["parent_a","parent_b"]},
            {"id":"other","minInclusive":"80","maxExclusive":"FF"}
        ]))),
        ..Default::default()
    };
    for mode in [TransportMode::Gateway, TransportMode::GatewayV2] {
        assert_eq!(
            resolved_session_token(&manager, &operation, mode, Some(&merged), &overrides)
                .unwrap()
                .as_str(),
            "merged:1#200#1=10#2=20",
        );
        let token =
            resolved_session_token(&manager, &operation, mode, Some(&different), &overrides);
        assert_eq!(
            token.as_ref().map(SessionToken::as_str),
            if mode == TransportMode::Gateway {
                Some(composite)
            } else {
                None
            },
        );
    }
}

#[test]
fn gateway_v2_unresolved_logical_target_omits_cached_composite_token() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "0:1#100#1=10,1:1#200#1=20");
    let overrides = OperationOverrides {
        logical_partition_key_target: true,
        ..Default::default()
    };

    assert!(resolved_session_token(
        &manager,
        &operation,
        TransportMode::GatewayV2,
        None,
        &overrides,
    )
    .is_none());
    assert!(resolved_session_token(
        &manager,
        &operation,
        TransportMode::Gateway,
        None,
        &overrides,
    )
    .is_some());

    let explicit = operation
        .clone()
        .with_session_token(SessionToken::new("explicit"));
    assert_eq!(
        resolved_session_token(
            &manager,
            &explicit,
            TransportMode::GatewayV2,
            None,
            &overrides,
        )
        .unwrap()
        .as_str(),
        "explicit"
    );
}

#[test]
fn shared_attempt_resolver_rekeys_parent_token_for_split_child() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let manager = SessionManager::new();
    capture_session_token(&manager, &operation, "parent:1#100#1=10");
    let overrides = OperationOverrides {
        partition_key_range_id: Some("child".to_string()),
        resolved_partition_key_range_parents: vec!["parent".to_string()],
        session_token_routing_map: Some(test_session_topology(serde_json::json!([
            {"id":"child","minInclusive":"","maxExclusive":"FF","parents":["parent"]}
        ]))),
        ..Default::default()
    };
    let child = PartitionKeyRangeId::from("child".to_string());

    for mode in [TransportMode::Gateway, TransportMode::GatewayV2] {
        let token =
            resolved_session_token(&manager, &operation, mode, Some(&child), &overrides).unwrap();
        assert_eq!(token.as_str(), "child:1#100#1=10");
    }
}

#[test]
fn internal_identity_allows_container_recreation_retry_in_place() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk"),
        "item",
    ));
    let overrides = OperationOverrides {
        resolved_partition_key_range_id: Some("old-generation".to_string()),
        ..Default::default()
    };

    assert!(recreation::container_recreation_retry_eligible(
        &operation, &overrides, None, false
    ));
    assert!(!overrides.requires_plan_rebuild_after_container_recreation());
}

#[test]
fn apply_headers_pk_range_only_omits_read_key_type() {
    let overrides = OperationOverrides {
        partition_key_range_id: Some("0".to_string()),
        ..Default::default()
    };
    let mut headers = azure_core::http::headers::Headers::new();
    overrides
        .apply_headers(&mut headers, false)
        .expect("apply_headers should succeed");

    assert_eq!(
        headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::PARTITION_KEY_RANGE_ID
            ))
            .map(|s| s.to_string()),
        Some("0".to_string())
    );
    assert!(
        headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::READ_FEED_KEY_TYPE
            ))
            .is_none(),
        "whole-PK-range targets must not emit x-ms-read-key-type"
    );
    assert!(headers
        .get_optional_str(&HeaderName::from_static(request_header_names::START_EPK))
        .is_none());
    assert!(headers
        .get_optional_str(&HeaderName::from_static(request_header_names::END_EPK))
        .is_none());
}

#[test]
fn apply_headers_feed_range_emits_read_key_type_and_epk_bounds() -> crate::error::Result<()> {
    let feed_range = FeedRange::new(
        EffectivePartitionKey::try_from("10")?,
        EffectivePartitionKey::try_from("20")?,
    )
    .unwrap();
    let overrides = OperationOverrides {
        partition_key_range_id: Some("pkrange".to_string()),
        feed_range: Some(feed_range),
        ..Default::default()
    };
    let mut headers = azure_core::http::headers::Headers::new();
    overrides
        .apply_headers(&mut headers, false)
        .expect("apply_headers should succeed");

    assert_eq!(
        headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::READ_FEED_KEY_TYPE
            ))
            .map(|s| s.to_string()),
        Some(request_header_names::READ_FEED_KEY_TYPE_EPK_RANGE.to_string())
    );
    assert_eq!(
        headers
            .get_optional_str(&HeaderName::from_static(request_header_names::START_EPK))
            .map(|s| s.to_string()),
        Some("10".to_string())
    );
    assert_eq!(
        headers
            .get_optional_str(&HeaderName::from_static(request_header_names::END_EPK))
            .map(|s| s.to_string()),
        Some("20".to_string())
    );
    Ok(())
}

#[test]
fn build_transport_request_feed_path_is_resolved() {
    let operation = CosmosOperation::read_all_databases(test_account());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert_eq!(request.url.path(), "/dbs");
}

#[test]
fn build_transport_request_single_resource_path_is_resolved() {
    let db = DatabaseReference::from_name(test_account(), "mydb");
    let operation = CosmosOperation::read_database(db);

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert_eq!(request.url.path(), "/dbs/mydb");
}

#[cfg(feature = "fault_injection")]
#[test]
fn fault_injection_headers_require_an_active_fault_client() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let endpoint = CosmosEndpoint::regional(
        Region::EAST_US,
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let routing = RoutingDecision {
        endpoint,
        routing_fallback: None,
    };
    let activity_id = ActivityId::from_string("activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let operation_header = HeaderName::from_static(
        crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_OPERATION,
    );
    let region_header = HeaderName::from_static(
        crate::models::cosmos_headers::fault_injection_header_names::FAULT_INJECTION_REGION,
    );

    let disabled = request::prepare(
        &operation,
        &OperationOverrides::default(),
        None,
        &ctx,
        false,
    )
    .expect("request should build");
    assert!(disabled
        .headers
        .get_optional_str(&operation_header)
        .is_none());
    assert!(disabled.headers.get_optional_str(&region_header).is_none());

    let enabled = request::prepare(&operation, &OperationOverrides::default(), None, &ctx, true)
        .expect("request should build");
    assert!(enabled
        .headers
        .get_optional_str(&operation_header)
        .is_some());
    assert!(enabled.headers.get_optional_str(&region_header).is_some());
}

#[test]
fn classic_gateway_emits_intended_collection_rid_for_name_addressing() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let routing = test_routing();
    let activity_id = ActivityId::from_string("activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };

    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    let (request, _) = dispatch::prepare_transport(
        request,
        &operation,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::INTENDED_COLLECTION_RID,
        )),
        Some(test_container().rid()),
    );
}

#[test]
fn gateway_v2_does_not_emit_intended_collection_rid() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let routing = routing::select(&CosmosEndpoint::regional_with_gateway_v2(
        Region::WEST_US_2,
        Url::parse("https://test.documents.azure.com:443/").unwrap(),
        Url::parse("https://thin.documents.azure.com:10250/").unwrap(),
    ));
    let activity_id = ActivityId::from_string("activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };

    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    let (request, _) = dispatch::prepare_transport(
        request,
        &operation,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::GatewayV2);
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::INTENDED_COLLECTION_RID,
        ))
        .is_none());
}

/// Builds a transport request for `operation` with default routing/context.
fn transport_request(
    operation: &CosmosOperation,
) -> crate::pipeline::transport::types::TransportRequest {
    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        throughput_control: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
    };
    request::build_transport_request(operation, &OperationOverrides::default(), None, &ctx)
        .expect("request should build")
}

/// Returns the final `Url::path()` after `set_path` has reprocessed it.
/// Used to assert the raw-vs-percent-encoded seam in `build_transport_request`.
fn transport_request_path(operation: &CosmosOperation) -> String {
    transport_request(operation).url.path().to_owned()
}

#[test]
fn build_transport_request_rid_path_is_sent_raw() {
    // A RID-addressed path must reach the gateway raw: the base64 `=` padding
    // survives `set_path` un-encoded (not `%3D`), otherwise the gateway rejects
    // the RID-based signature with a `401`.
    let db = DatabaseReference::from_rid(test_account(), "qjQBAA==");
    let operation = CosmosOperation::read_database(db);
    assert_eq!(transport_request_path(&operation), "/dbs/qjQBAA==");
}

#[test]
fn build_transport_request_rid_path_with_plus_is_sent_raw() {
    // RIDs use a base64 variant that can contain `+`; it must also survive raw
    // (not `%2B`) on a RID-addressed path.
    let db = DatabaseReference::from_rid(test_account(), "ab+cdEAA=");
    let operation = CosmosOperation::read_database(db);
    assert_eq!(transport_request_path(&operation), "/dbs/ab+cdEAA=");
}

#[test]
fn build_transport_request_name_path_encodes_only_url_structural_chars() {
    // Names reach the wire via `Url::set_path`, which encodes only the
    // characters that are structurally significant in a URL path: a space
    // becomes `%20`. Path-legal characters such as `+` survive raw so the
    // outer path still matches the raw resource link we signed (and, on
    // Gateway 2.0, its RNTBD target).
    let db = DatabaseReference::from_name(test_account(), "my db+x");
    let operation = CosmosOperation::read_database(db);
    assert_eq!(transport_request_path(&operation), "/dbs/my%20db+x");
}

#[test]
fn build_transport_request_name_with_at_sign_is_sent_raw() {
    // Regression for the Gateway 2.0 outer-path-vs-RNTBD mismatch: an item id
    // (or any name) containing `@` must reach the wire raw (`@`, not `%40`),
    // because the gateway rebuilds its RNTBD target from the raw resource link
    // and compares the two byte-for-byte.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "Item@1-abc");
    let operation = CosmosOperation::read_item(item);
    assert_eq!(
        transport_request_path(&operation),
        "/dbs/testdb/colls/testcontainer/docs/Item@1-abc"
    );
}

#[test]
fn build_transport_request_escapes_literal_percent_without_changing_signing_link() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "item%41");
    let operation = CosmosOperation::read_item(item);
    let request = transport_request(&operation);

    assert_eq!(
        request.url.path(),
        "/dbs/testdb/colls/testcontainer/docs/item%2541"
    );
    assert_eq!(
        request.auth_context.resource_link.as_str(),
        "dbs/testdb/colls/testcontainer/docs/item%41"
    );
}

#[test]
fn build_transport_request_offer_path_is_sent_raw() {
    // Offers are signed over the lowercased RID, so the `/offers/{rid}` path is
    // RID-addressed and must be sent raw — reserved base64 characters in the
    // RID survive un-encoded.
    let operation = CosmosOperation::read_offer(test_account(), "ab+QBAA==");
    assert_eq!(transport_request_path(&operation), "/offers/ab+QBAA==");
}

#[test]
fn build_transport_request_uses_operation_activity_id_when_present() {
    let operation = CosmosOperation::read_all_databases(test_account())
        .with_activity_id(ActivityId::from_string("operation-activity".to_string()));

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    let activity_header = request
        .headers
        .get_optional_str(&HeaderName::from_static("x-ms-activity-id"))
        .expect("activity id should be set");
    assert_eq!(activity_header, "operation-activity");
}

#[test]
fn build_transport_request_adds_partition_key_header_for_item_operation() {
    let item_ref = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item_ref);

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let overrides = OperationOverrides {
        partition_key: Some(PartitionKey::from("pk1")),
        ..Default::default()
    };
    let request = request::build_transport_request(&operation, &overrides, None, &ctx)
        .expect("request should build");

    let partition_key_header = request
        .headers
        .get_optional_str(&HeaderName::from_static("x-ms-documentdb-partitionkey"))
        .expect("partition key header should be set");
    assert_eq!(partition_key_header, "[\"pk1\"]");
}

#[test]
fn build_transport_request_change_feed_continuation_uses_if_none_match() {
    let pk_def = test_partition_key_definition("/partition_key");
    let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
    let operation = CosmosOperation::change_feed(test_container(), Some(target));
    assert!(operation.is_change_feed());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let overrides = OperationOverrides {
        partition_key: Some(PartitionKey::from("pk1")),
        continuation: Some("\"etag-123\"".to_string()),
        ..Default::default()
    };
    let request = request::build_transport_request(&operation, &overrides, None, &ctx)
        .expect("request should build");

    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::IF_NONE_MATCH
            ))
            .map(|s| s.to_string()),
        Some("\"etag-123\"".to_string()),
        "change feed continuation must be sent as If-None-Match"
    );
    assert!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static(request_header_names::CONTINUATION))
            .is_none(),
        "change feed must not send x-ms-continuation"
    );
    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION
            ))
            .map(|s| s.to_string()),
        Some(request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15.to_string()),
        "change feed must send the wire-format-version header"
    );
}

#[test]
fn change_feed_transport_uses_only_restored_start_headers() {
    let starts = [
        crate::models::ChangeFeedStartFrom::Beginning,
        crate::models::ChangeFeedStartFrom::Now,
        crate::models::ChangeFeedStartFrom::PointInTime(
            time::macros::datetime!(2026-08-21 12:34:56 UTC),
        ),
    ];
    let routing = test_routing();
    let activity_id = ActivityId::from_string("start-restoration".to_owned());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    for previous in &starts {
        for start in &starts {
            let operation = CosmosOperation::change_feed(
                test_container(),
                Some(crate::models::FeedRange::full()),
            )
            .with_change_feed_start(previous.clone())
            .with_change_feed_start(start.clone());
            for continuation in [None, Some("\"saved-etag\"".to_owned())] {
                let overrides = OperationOverrides {
                    continuation: continuation.clone(),
                    partition_key_range_id: Some("0".to_owned()),
                    ..Default::default()
                };
                let request =
                    request::build_transport_request(&operation, &overrides, None, &ctx).unwrap();
                let expected_etag = continuation.as_deref().or_else(|| {
                    matches!(start, crate::models::ChangeFeedStartFrom::Now).then_some("*")
                });
                assert_eq!(
                    request.headers.get_optional_str(&HeaderName::from_static(
                        request_header_names::IF_NONE_MATCH
                    )),
                    expected_etag,
                );
                let expected_time =
                    matches!(start, crate::models::ChangeFeedStartFrom::PointInTime(_))
                        .then_some("Fri, 21 Aug 2026 12:34:56 GMT");
                assert_eq!(
                    request.headers.get_optional_str(&HeaderName::from_static(
                        request_header_names::IF_MODIFIED_SINCE
                    )),
                    expected_time,
                );
                assert!(request
                    .headers
                    .get_optional_str(&HeaderName::from_static(request_header_names::CONTINUATION))
                    .is_none());
            }
        }
    }
}

#[test]
fn prefix_change_feed_transport_preserves_leaf_bounds_in_gateway_v2() {
    let mut properties = test_container_props();
    properties.partition_key = ("/tenant", "/user", "/device").into();
    let container = ContainerReference::new(
        test_account(),
        "testdb",
        "testdb_rid",
        "testcontainer",
        "testcontainer_rid",
        &properties,
    );
    let prefix = FeedRange::for_partition(
        PartitionKey::from("A"),
        container.partition_key_definition(),
    );
    let operation = CosmosOperation::change_feed(container.clone(), Some(prefix.clone()));
    let middle = EffectivePartitionKey::from_bytes({
        let mut bytes = prefix.min_inclusive().as_bytes().to_vec();
        bytes.push(0x80);
        bytes
    });
    let slices = [
        FeedRange::new(prefix.min_inclusive().clone(), middle.clone()).unwrap(),
        FeedRange::new(middle, prefix.max_exclusive().clone()).unwrap(),
        FeedRange::full(),
    ];
    let routing = test_routing();
    let activity_id = ActivityId::from_string("00000000-0000-4000-8000-000000000001".into());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    for slice in slices {
        let overrides = OperationOverrides {
            partition_key_range_id: Some("merged".into()),
            feed_range: (slice != FeedRange::full()).then_some(slice.clone()),
            pkrange_bounds: Some(FeedRange::full()),
            continuation: Some("parent-etag".into()),
            ..Default::default()
        };
        let mut request =
            request::build_transport_request(&operation, &overrides, None, &ctx).unwrap();
        assert!(request.effective_partition_key.is_none());
        request.headers.insert(
            azure_core::http::headers::AUTHORIZATION,
            "test-authorization",
        );
        request.headers.insert(
            HeaderName::from_static("x-ms-date"),
            "Wed, 21 Oct 2015 07:28:00 GMT",
        );
        let inputs = GatewayV2Headers {
            auth_context: &request.auth_context,
            operation_type: operation.operation_type(),
            resource_type: operation.resource_type(),
            effective_partition_key: request.effective_partition_key.as_ref(),
            effective_consistency: DefaultConsistencyLevel::Session,
            read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
            account_name: Some("test"),
            collection_rid: None,
        };
        let wrapped = wrap_request_for_gateway_v2(
            HttpRequest {
                url: request.url,
                method: request.method,
                headers: request.headers,
                body: request.body,
                timeout: None,
                #[cfg(feature = "fault_injection")]
                evaluation_collector: None,
            },
            &inputs,
        )
        .unwrap();
        let frame = RntbdRequestFrame::read(wrapped.body.as_ref().unwrap()).unwrap();
        let bounds: Vec<_> = frame
            .metadata
            .iter()
            .filter(|token| {
                token.id == RntbdRequestToken::StartEpkHash.into()
                    || token.id == RntbdRequestToken::EndEpkHash.into()
            })
            .cloned()
            .collect();
        assert_eq!(
            bounds,
            vec![
                Token::start_epk_hash(slice.min_inclusive().as_bytes().to_vec()),
                Token::end_epk_hash(slice.max_exclusive().as_bytes().to_vec()),
            ]
        );
        let filters: Vec<_> = frame
            .metadata
            .iter()
            .filter(|token| {
                token.id == RntbdRequestToken::StartEpk.into()
                    || token.id == RntbdRequestToken::EndEpk.into()
                    || token.id == RntbdRequestToken::ReadFeedKeyType.into()
            })
            .cloned()
            .collect();
        assert_eq!(
            filters,
            if slice == FeedRange::full() {
                vec![]
            } else {
                vec![
                    Token::start_epk(slice.min_inclusive().to_hex()),
                    Token::end_epk(slice.max_exclusive().to_hex()),
                    Token::read_feed_key_type(ReadFeedKeyType::EffectivePartitionKeyRange),
                ]
            }
        );
        assert!(!frame.metadata.iter().any(|token| token.id
            == RntbdRequestToken::EffectivePartitionKey.into()
            || token.id == RntbdRequestToken::PartitionKey.into()));
        assert!(frame
            .metadata
            .contains(&Token::partition_key_range_id("merged".into())));
        assert!(frame
            .metadata
            .contains(&Token::match_condition("parent-etag".into())));
    }
    let complete = PartitionKey::from(("A", "one", "x"));
    let operation = CosmosOperation::change_feed(
        container.clone(),
        Some(FeedRange::for_partition(
            complete.clone(),
            container.partition_key_definition(),
        )),
    );
    assert!(
        crate::pipeline::operation::request::effective_partition_key_for_request(&operation)
            .unwrap()
            .is_some()
    );
    let query = CosmosOperation::query_items(container, Some(prefix));
    assert!(
        crate::pipeline::operation::request::effective_partition_key_for_request(&query)
            .unwrap()
            .is_some()
    );
}

#[test]
fn build_transport_request_change_feed_continuation_keeps_if_modified_since() {
    // PointInTime start sets If-Modified-Since on the shared operation.
    // Merged partitions require the original timestamp alongside every
    // ETag continuation so the backend can filter interleaved parent LSNs.
    let pk_def = test_partition_key_definition("/partition_key");
    let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
    let operation = CosmosOperation::change_feed(test_container(), Some(target))
        .with_if_modified_since("Mon, 01 Jan 2024 00:00:00 GMT".to_string());
    assert!(operation.is_change_feed());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let overrides = OperationOverrides {
        partition_key: Some(PartitionKey::from("pk1")),
        continuation: Some("\"etag-123\"".to_string()),
        ..Default::default()
    };
    let request = request::build_transport_request(&operation, &overrides, None, &ctx)
        .expect("request should build");

    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::IF_NONE_MATCH
            ))
            .map(|s| s.to_string()),
        Some("\"etag-123\"".to_string()),
        "continuation must be sent as If-None-Match"
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::IF_MODIFIED_SINCE
        )),
        Some("Mon, 01 Jan 2024 00:00:00 GMT"),
        "PointInTime start marker must remain alongside the continuation"
    );
}

#[test]
fn build_transport_request_change_feed_keeps_if_modified_since_without_continuation() {
    // A partition that has never been polled (no continuation) must still
    // honor the PointInTime start marker.
    let pk_def = test_partition_key_definition("/partition_key");
    let target = crate::models::FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
    let operation = CosmosOperation::change_feed(test_container(), Some(target))
        .with_if_modified_since("Mon, 01 Jan 2024 00:00:00 GMT".to_string());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::OperationRetry,
        deadline: Some(std::time::Instant::now() + Duration::from_secs(5)),
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let overrides = OperationOverrides {
        partition_key: Some(PartitionKey::from("pk1")),
        ..Default::default()
    };
    let request = request::build_transport_request(&operation, &overrides, None, &ctx)
        .expect("request should build");

    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static(
                request_header_names::IF_MODIFIED_SINCE
            ))
            .map(|s| s.to_string()),
        Some("Mon, 01 Jan 2024 00:00:00 GMT".to_string()),
        "PointInTime start marker must be kept when no continuation is present"
    );
}

#[test]
fn build_transport_request_leaves_gateway_selection_to_dispatch() {
    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let selected_url = Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap();
    let routing = RoutingDecision {
        endpoint: CosmosEndpoint::regional_with_gateway_v2(
            "westus2".into(),
            Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
            selected_url.clone(),
        ),
        routing_fallback: None,
    };

    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert_eq!(
        request.url.as_str(),
        "https://test-westus2.documents.azure.com/dbs/mydb"
    );
}

#[test]
fn build_transport_request_uses_default_url_for_global_endpoint() {
    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let routing = RoutingDecision {
        endpoint,
        routing_fallback: None,
    };

    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert_eq!(
        request.url.as_str(),
        "https://test.documents.azure.com/dbs/mydb"
    );
}

#[test]
fn patch_verification_read_strips_custom_session_token_headers() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let routing = test_routing();
    let activity_id = ActivityId::from_string("patch-read-headers".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::LatestCommitted,
        throughput_control: None,
    };
    let session_header = HeaderName::from_static(request_header_names::SESSION_TOKEN);
    let custom_headers = std::collections::HashMap::from([(
        session_header.clone(),
        azure_core::http::headers::HeaderValue::from_static("0:1#42"),
    )]);

    // Dataflow-supplied custom headers are inserted while the request is built.
    let mut request = request::build_transport_request(
        &operation,
        &OperationOverrides::default(),
        Some(&custom_headers),
        &ctx,
    )
    .expect("request should build");
    crate::pipeline::operation::request::apply_optional_request_headers(
        &mut request,
        &operation,
        &crate::options::OperationOptionsView::new(None, None, None, None),
    );
    assert!(request.headers.get_optional_str(&session_header).is_none());

    // Layered runtime/account/operation custom headers are applied afterward.
    let mut options = crate::options::OperationOptions::default();
    options.custom_headers = Some(custom_headers);
    let view = crate::options::OperationOptionsView::new(None, None, None, Some(&options));
    let mut request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");
    crate::pipeline::operation::request::apply_optional_request_headers(
        &mut request,
        &operation,
        &view,
    );
    assert!(request.headers.get_optional_str(&session_header).is_none());
}

#[test]
fn content_response_enabled_strips_custom_prefer_headers() {
    let operation = CosmosOperation::patch_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let routing = test_routing();
    let activity_id = ActivityId::from_string("patch-response-headers".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let prefer_header = HeaderName::from_static(request_header_names::PREFER);
    let custom_headers = std::collections::HashMap::from([(
        prefer_header.clone(),
        azure_core::http::headers::HeaderValue::from_static("return=minimal"),
    )]);
    let content_response_options = crate::options::OperationOptions {
        content_response_on_write: Some(crate::options::ContentResponseOnWrite::Enabled),
        ..Default::default()
    };

    let mut request = request::build_transport_request(
        &operation,
        &OperationOverrides::default(),
        Some(&custom_headers),
        &ctx,
    )
    .expect("request should build");
    let view = crate::options::OperationOptionsView::new(
        None,
        None,
        None,
        Some(&content_response_options),
    );
    crate::pipeline::operation::request::apply_optional_request_headers(
        &mut request,
        &operation,
        &view,
    );
    assert!(request.headers.get_optional_str(&prefer_header).is_none());

    let layered_options = crate::options::OperationOptions {
        content_response_on_write: Some(crate::options::ContentResponseOnWrite::Enabled),
        custom_headers: Some(custom_headers),
        ..Default::default()
    };
    let view = crate::options::OperationOptionsView::new(None, None, None, Some(&layered_options));
    let mut request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");
    crate::pipeline::operation::request::apply_optional_request_headers(
        &mut request,
        &operation,
        &view,
    );
    assert!(request.headers.get_optional_str(&prefer_header).is_none());
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
        preferred_read_endpoints: vec![read_endpoint].into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: write_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 1,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: Vec::new(),
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
    };

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, write_endpoint);
}

#[test]
fn resolve_endpoint_uses_write_region_for_patch_verification_read() {
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
        preferred_read_endpoints: vec![read_endpoint].into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: write_endpoint.clone(),
    }));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.session_retry_routing =
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, write_endpoint);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_read_falls_back_when_write_endpoint_is_unavailable() {
    use crate::pipeline::operation::routing::RoutingFallbackReason;

    let write_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.session_retry_routing =
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints;

    for reason in [
        crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        crate::pipeline::operation::routing::UnavailableReason::WriteForbidden,
    ] {
        let mut unavailable = std::collections::HashMap::new();
        unavailable.insert(
            write_endpoint.url().clone(),
            (std::time::Instant::now(), reason),
        );
        let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![read_endpoint.clone()].into(),
            preferred_write_endpoints: vec![write_endpoint.clone()].into(),
            account_write_endpoints: vec![write_endpoint.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: false,
            default_endpoint: read_endpoint.clone(),
        }));

        let routing = routing::resolve(
            &crate::clock::Clock::default(),
            &operation,
            &retry_state,
            &location,
            Duration::from_secs(60),
        );

        assert_eq!(routing.endpoint, read_endpoint);
        assert_eq!(
            routing.routing_fallback,
            Some(RoutingFallbackReason::PatchVerificationReadWriteEndpointUnavailableOrExcluded)
        );
    }
}

#[test]
fn patch_verification_read_recycled_writer_is_not_reported_as_fallback() {
    let write_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        write_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![write_endpoint.clone()].into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: write_endpoint.clone(),
    }));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, write_endpoint);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_read_falls_back_when_write_endpoint_is_excluded() {
    use crate::pipeline::operation::routing::RoutingFallbackReason;

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
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: read_endpoint.clone(),
    }));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        vec![crate::options::Region::EAST_US],
        3,
        2,
    );
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, read_endpoint);
    assert_eq!(
        routing.routing_fallback,
        Some(RoutingFallbackReason::PatchVerificationReadWriteEndpointUnavailableOrExcluded)
    );
}

#[test]
fn patch_verification_read_rotates_across_multi_write_endpoints() {
    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone(), west.clone()].into(),
        account_write_endpoints: vec![east.clone(), west.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: east,
    }));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state =
        crate::pipeline::operation::recovery::state::OperationRetryState::initial(
            0,
            true,
            Vec::new(),
            3,
            2,
        )
        .advance_failover()
        .advance_location(2, 0);
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, west);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_read_uses_ppaf_partition_writer() {
    use crate::pipeline::operation::routing::partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
    };

    let account_writer = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let partition_gateway = Url::parse("https://test-westus2.documents.azure.com:443/").unwrap();
    let partition_writer = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        partition_gateway.clone(),
        Url::parse("https://current-westus2-thin.documents.azure.com:444/").unwrap(),
    );
    let stale_partition_writer = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        partition_gateway,
        Url::parse("https://stale-westus2-thin.documents.azure.com:444/").unwrap(),
    );
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![account_writer.clone(), partition_writer.clone()].into(),
        preferred_write_endpoints: vec![account_writer.clone()].into(),
        account_write_endpoints: vec![account_writer.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: account_writer,
    });
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        "0".parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: stale_partition_writer.clone(),
            first_failed_endpoint: stale_partition_writer,
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some("0".parse().unwrap());
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, partition_writer);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_read_skips_unusable_ppaf_writer_for_account_writer() {
    use crate::pipeline::operation::routing::partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
    };

    let account_writer = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let unavailable_partition_writer = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        unavailable_partition_writer.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![
            account_writer.clone(),
            unavailable_partition_writer.clone(),
        ]
        .into(),
        preferred_write_endpoints: vec![account_writer.clone()].into(),
        account_write_endpoints: vec![account_writer.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: account_writer.clone(),
    });
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        "0".parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: unavailable_partition_writer,
            first_failed_endpoint: account_writer.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some("0".parse().unwrap());
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, account_writer);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_read_skips_failed_writer_after_excluded_head() {
    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![east.clone(), west.clone(), central.clone()].into(),
        account_write_endpoints: vec![east, west.clone(), central.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: west.clone(),
    }));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        vec![crate::options::Region::EAST_US],
        3,
        2,
    );
    retry_state
        .patch_verification_failed_endpoint_urls
        .push(west.url().clone());
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, central);
    assert_eq!(routing.routing_fallback, None);
}

#[test]
fn patch_verification_fallback_ignores_ppcb_override() {
    use crate::pipeline::operation::routing::{
        partition_endpoint_state::{HealthStatus, PartitionEndpointState, PartitionFailoverEntry},
        RoutingFallbackReason,
    };

    let unavailable_writer = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let normal_read = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let ppcb_override = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        unavailable_writer.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![normal_read.clone(), ppcb_override.clone()].into(),
        preferred_write_endpoints: vec![unavailable_writer.clone()].into(),
        account_write_endpoints: vec![unavailable_writer.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: unavailable_writer.clone(),
    });
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_circuit_breaker_enabled = true;
    partitions.circuit_breaker_overrides.insert(
        "0".parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: ppcb_override,
            first_failed_endpoint: unavailable_writer,
            failed_endpoints: Default::default(),
            read_failure_count: i32::MAX,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some("0".parse().unwrap());
    recovery::state::apply_read_routing_strategy(&mut retry_state, true);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(routing.endpoint, normal_read);
    assert_eq!(
        routing.routing_fallback,
        Some(RoutingFallbackReason::PatchVerificationReadWriteEndpointUnavailableOrExcluded)
    );
}

#[test]
fn patch_verification_write_routing_wins_over_ppcb_read_override() {
    use crate::pipeline::operation::routing::partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
    };

    let write_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let ppcb_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![write_endpoint.clone(), ppcb_endpoint.clone()].into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: write_endpoint.clone(),
    });
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_circuit_breaker_enabled = true;
    partitions.circuit_breaker_overrides.insert(
        "0".parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: ppcb_endpoint.clone(),
            first_failed_endpoint: write_endpoint.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: i32::MAX,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let ordinary_read = CosmosOperation::read_item(item.clone());
    let patch_read = CosmosOperation::read_item(item).as_patch_read_sub_operation();
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some("0".parse().unwrap());

    let ordinary_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &ordinary_read,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(ordinary_routing.endpoint, ppcb_endpoint);

    retry_state.session_retry_routing =
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints;
    let patch_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &patch_read,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(patch_routing.endpoint, write_endpoint);
    assert_eq!(patch_routing.routing_fallback, None);
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn resolve_endpoint_uses_account_write_region_order_for_read_dtx() {
    let operation = CosmosOperation::distributed_transaction(
        test_account(),
        crate::models::DistributedTransactionType::Read,
    );
    let account_write_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let preferred_write_endpoint = CosmosEndpoint::regional(
        "westus3".into(),
        Url::parse("https://test-westus3.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint].into(),
        preferred_write_endpoints: vec![preferred_write_endpoint].into(),
        account_write_endpoints: vec![account_write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: account_write_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        1,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, account_write_endpoint);
}

#[cfg(feature = "unstable_dtx")]
#[test]
fn resolve_endpoint_uses_account_write_region_order_for_write_dtx() {
    let operation = CosmosOperation::distributed_transaction(
        test_account(),
        crate::models::DistributedTransactionType::Write,
    );
    let account_write_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let preferred_write_endpoint = CosmosEndpoint::regional(
        "westus3".into(),
        Url::parse("https://test-westus3.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint].into(),
        preferred_write_endpoints: vec![preferred_write_endpoint].into(),
        account_write_endpoints: vec![account_write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: account_write_endpoint.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        1,
    );
    retry_state.ppaf_write_retry_allowed = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, account_write_endpoint);
}

#[test]
fn resolve_endpoint_deprioritizes_unavailable_over_global_fallback() {
    let operation = CosmosOperation::read_all_databases(test_account());
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        read_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::TransportError,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![default_endpoint.clone()].into(),
        account_write_endpoints: vec![default_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
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

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    // Unavailable regional endpoint is de-prioritized but still preferred
    // over the global fallback.
    assert_eq!(routing.endpoint, read_endpoint);
}

#[test]
fn resolve_endpoint_ignores_write_forbidden_for_reads() {
    let operation = CosmosOperation::read_all_databases(test_account());
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        read_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::WriteForbidden,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![read_endpoint.clone()].into(),
        account_write_endpoints: vec![read_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: read_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
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

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, read_endpoint);
}

#[test]
fn stale_generation_advances_across_refreshed_endpoint_list() {
    let operation = CosmosOperation::read_all_databases(test_account());
    let endpoint_a = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let endpoint_b = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let endpoint_c = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 1,
        preferred_read_endpoints: vec![endpoint_a.clone(), endpoint_b.clone(), endpoint_c.clone()]
            .into(),
        preferred_write_endpoints: vec![endpoint_a.clone(), endpoint_b.clone(), endpoint_c.clone()]
            .into(),
        account_write_endpoints: vec![endpoint_a.clone(), endpoint_b.clone(), endpoint_c.clone()]
            .into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: endpoint_a.clone(),
    }));

    let stale_retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0).next(3),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 3,
        can_use_multiple_write_locations: true,
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

    let first_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &stale_retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(first_routing.endpoint, endpoint_a);

    let advanced_state = stale_retry_state
        .advance_failover()
        .advance_location(3, location.account.generation);

    let second_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &advanced_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(second_routing.endpoint, endpoint_b);
}

// ── Hub region cache warm-path ─────────────────────────────────────

/// Builds a single-master `LocationSnapshot` with regional
/// endpoints and a populated hub-region cache pointing the supplied
/// partition key range at `hub_endpoint`.
fn hub_cache_location_snapshot(
    pk_range_id: &str,
    hub_endpoint: &CosmosEndpoint,
) -> (LocationSnapshot, CosmosEndpoint, CosmosEndpoint) {
    use crate::pipeline::operation::routing::partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
    };

    let eastus = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let westus = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
        preferred_write_endpoints: vec![eastus.clone()].into(),
        account_write_endpoints: vec![eastus.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: eastus.clone(),
    });

    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range_id.parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: hub_endpoint.clone(),
            first_failed_endpoint: hub_endpoint.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );

    (
        LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions)),
        eastus,
        westus,
    )
}

/// Builds a retry state representing a single-master data-plane read
/// that has latched the hub-region header on the first 1002 and has
/// resolved a partition key range ID from the response.
fn read_state_with_hub_latch(
    pk_range_id: &str,
) -> crate::pipeline::operation::recovery::state::OperationRetryState {
    crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 1,
        max_failover_retries: 3,
        max_session_retries: 3,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_backend_failover_retries:
            crate::pipeline::operation::recovery::state::MAX_BACKEND_FAILOVER_RETRIES,
        can_use_multiple_write_locations: false,
        is_dataplane: true,
        hub_region_processing_only: true,
        excluded_regions: Vec::new(),
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints,
        partition_key_range_id: Some(pk_range_id.parse().unwrap()),
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
        retry_with_state: None,
    }
}

#[test]
fn resolve_endpoint_routes_to_cached_hub_when_latch_set() {
    let pk_range = "0";
    let hub = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );
    let (location, _eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let retry_state = read_state_with_hub_latch(pk_range);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(
        routing.endpoint, hub,
        "warm cache hit must route directly to the cached hub region",
    );
}

#[test]
fn resolve_endpoint_ignores_hub_cache_when_latch_not_set() {
    let pk_range = "0";
    let hub = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );
    let (location, eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let mut retry_state = read_state_with_hub_latch(pk_range);
    retry_state.hub_region_processing_only = false; // latch off
    retry_state.session_retry_routing =
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints;
    retry_state.session_token_retry_count = 0;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_eq!(
        routing.endpoint, eastus,
        "without the latch, normal selection picks the first preferred read endpoint",
    );
}

#[test]
fn resolve_endpoint_ignores_hub_cache_when_pk_range_id_missing() {
    let pk_range = "0";
    let hub = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );
    let (location, _eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let mut retry_state = read_state_with_hub_latch(pk_range);
    // PK range ID not yet captured from a response header.
    retry_state.partition_key_range_id = None;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_ne!(
        routing.endpoint, hub,
        "without partition_key_range_id we cannot key into the cache",
    );
}

/// Hub-region warm-path routing is gated on
/// `per_partition_automatic_failover_enabled` — even when the latch is
/// set and the cache contains an entry, an account without PPAF must
/// not be silently routed by the hub cache. Mirrors the
/// `cache_hub_region_skips_when_ppaf_disabled` writer-side gate.
#[test]
fn resolve_endpoint_ignores_hub_cache_when_ppaf_disabled() {
    use crate::pipeline::operation::routing::partition_endpoint_state::{
        HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
    };

    let pk_range = "0";
    let eastus = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let westus = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
        preferred_write_endpoints: vec![eastus.clone()].into(),
        account_write_endpoints: vec![eastus.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: eastus.clone(),
    });

    // PPAF is OFF on this partition state but the cache nevertheless
    // contains a hub entry (e.g., a stale entry from a feature flip).
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_automatic_failover_enabled = false;
    partitions.failover_overrides.insert(
        pk_range.parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: westus.clone(),
            first_failed_endpoint: westus.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );

    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let retry_state = read_state_with_hub_latch(pk_range);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_ne!(
        routing.endpoint, westus,
        "warm-path hub cache must not route when PPAF is disabled on the partition state",
    );
}

/// Hub-region warm-path routing honors the caller's `excluded_regions`:
/// even with the latch set, PPAF enabled, and a cache entry present, a
/// cached hub endpoint whose region the caller excluded must not be
/// selected. Selection falls through to the default path, which also
/// honors the exclusion. Mirrors the PPCB read-side exclusion gate.
#[test]
fn resolve_endpoint_hub_cache_honors_excluded_regions() {
    let pk_range = "0";
    let hub = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );
    let (location, eastus, _westus) = hub_cache_location_snapshot(pk_range, &hub);

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let mut retry_state = read_state_with_hub_latch(pk_range);
    // Caller pins reads away from the cached hub region.
    retry_state.excluded_regions = vec![crate::options::Region::from("westus")];

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_ne!(
        routing.endpoint, hub,
        "warm-path hub cache must not route to an excluded region",
    );
    assert_eq!(
        routing.endpoint, eastus,
        "selection must fall through to the non-excluded preferred region",
    );
}

/// Hub-region warm-path routing skips a cached hub endpoint that has been
/// marked unavailable account-wide and falls through to the next available
/// read endpoint. (Validates #604.)
#[test]
fn resolve_endpoint_hub_cache_skips_unavailable_hub() {
    use crate::pipeline::operation::routing::{
        partition_endpoint_state::{HealthStatus, PartitionEndpointState, PartitionFailoverEntry},
        UnavailableReason,
    };

    let pk_range = "0";
    let eastus = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let westus = CosmosEndpoint::regional(
        "westus".into(),
        Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
    );

    // The cache points the partition at the westus hub, but westus has
    // just been marked unavailable for a non-write reason.
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        westus.url().clone(),
        (
            std::time::Instant::now(),
            UnavailableReason::ServiceUnavailable,
        ),
    );
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![eastus.clone(), westus.clone()].into(),
        preferred_write_endpoints: vec![eastus.clone()].into(),
        account_write_endpoints: vec![eastus.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: eastus.clone(),
    });
    let mut partitions = PartitionEndpointState::default();
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range.parse().unwrap(),
        PartitionFailoverEntry {
            current_endpoint: westus.clone(),
            first_failed_endpoint: westus.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let operation =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));
    let retry_state = read_state_with_hub_latch(pk_range);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    assert_ne!(
        routing.endpoint, westus,
        "warm-path hub cache must not route to an unavailable hub endpoint",
    );
    assert_eq!(
        routing.endpoint, eastus,
        "with the hub unavailable, selection falls through to the next available read endpoint",
    );
}

#[test]
fn hub_region_cache_populate_target_gates_emission() {
    let pk = "0";
    let write_op = CosmosOperation::create_database(test_account());
    let read_op =
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "mydb"));

    // All three conditions met → returns Some(pk).
    let all_met = read_state_with_hub_latch(pk);
    assert_eq!(
        routing::hub_region_cache_populate_target(&all_met, &read_op),
        Some(pk.parse().unwrap()),
        "populate gate must fire when latch + pk_range_id + read are all present",
    );

    // Latch off → None.
    let mut no_latch = read_state_with_hub_latch(pk);
    no_latch.hub_region_processing_only = false;
    assert!(
        routing::hub_region_cache_populate_target(&no_latch, &read_op).is_none(),
        "populate gate must NOT fire when the hub-region latch is off",
    );

    // No partition key range → None.
    let mut no_pk = read_state_with_hub_latch(pk);
    no_pk.partition_key_range_id = None;
    assert!(
        routing::hub_region_cache_populate_target(&no_pk, &read_op).is_none(),
        "populate gate must NOT fire without a partition_key_range_id (cache cannot be keyed)",
    );

    // Write op → None (writes use PPAF write-side routing, not the hub cache).
    assert!(
        routing::hub_region_cache_populate_target(&all_met, &write_op).is_none(),
        "populate gate must NOT fire on write operations",
    );
}

mod should_capture_session_token_from_status_tests {
    use azure_core::http::StatusCode;

    use crate::{
        models::{CosmosResponseHeaders, CosmosStatus},
        pipeline::{operation::session, transport::types::TransportOutcome},
    };

    fn success_outcome() -> TransportOutcome {
        TransportOutcome::Success {
            status: CosmosStatus::new(StatusCode::Ok),
            cosmos_headers: CosmosResponseHeaders::default(),
            body: Vec::new(),
        }
    }

    fn http_error_outcome(status: StatusCode) -> TransportOutcome {
        TransportOutcome::HttpError {
            status: CosmosStatus::new(status),
            cosmos_headers: CosmosResponseHeaders::default(),
            body: Vec::new(),
            request_sent: crate::diagnostics::RequestSentStatus::Sent,
        }
    }

    #[test]
    fn captures_on_success() {
        let outcome = success_outcome();
        assert!(session::should_capture_session_token_from_status(
            None, &outcome
        ));
    }

    #[test]
    fn captures_on_409_conflict() {
        let outcome = http_error_outcome(StatusCode::Conflict);
        assert!(session::should_capture_session_token_from_status(
            None, &outcome
        ));
    }

    #[test]
    fn captures_on_412_precondition_failed() {
        let outcome = http_error_outcome(StatusCode::PreconditionFailed);
        assert!(session::should_capture_session_token_from_status(
            None, &outcome
        ));
    }

    #[test]
    fn skips_on_404_with_substatus_1002() {
        let outcome = http_error_outcome(StatusCode::NotFound);
        let substatus = crate::error::status_codes::substatus::READ_SESSION_NOT_AVAILABLE;
        assert!(!session::should_capture_session_token_from_status(
            Some(&substatus),
            &outcome
        ));
    }

    #[test]
    fn captures_on_404_without_substatus_1002() {
        let outcome = http_error_outcome(StatusCode::NotFound);
        assert!(session::should_capture_session_token_from_status(
            None, &outcome
        ));
    }

    #[test]
    fn skips_on_500_internal_server_error() {
        let outcome = http_error_outcome(StatusCode::InternalServerError);
        assert!(!session::should_capture_session_token_from_status(
            None, &outcome
        ));
    }
}

mod effective_consistency_tests {
    use crate::{models::DefaultConsistencyLevel, options::ReadConsistencyStrategy};

    #[test]
    fn default_strategy_with_session_account() {
        assert!(
            ReadConsistencyStrategy::Default.is_session_effective(DefaultConsistencyLevel::Session)
        );
    }

    #[test]
    fn default_strategy_with_strong_account() {
        assert!(
            !ReadConsistencyStrategy::Default.is_session_effective(DefaultConsistencyLevel::Strong)
        );
    }

    #[test]
    fn session_strategy_overrides_account() {
        assert!(
            ReadConsistencyStrategy::Session.is_session_effective(DefaultConsistencyLevel::Strong)
        );
    }

    #[test]
    fn eventual_strategy_never_session() {
        assert!(!ReadConsistencyStrategy::Eventual
            .is_session_effective(DefaultConsistencyLevel::Session));
    }

    #[test]
    fn consistent_prefix_not_session() {
        assert!(!ReadConsistencyStrategy::Default
            .is_session_effective(DefaultConsistencyLevel::ConsistentPrefix));
    }
}

#[test]
fn dispatch_prefers_gateway_v2_for_dataplane_reads() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, endpoint);
    let (request, _) = dispatched_request(
        &operation,
        &routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::GatewayV2);
    assert_eq!(
        request.url.as_str(),
        "https://test-westus2-thin.documents.azure.com:444/dbs/testdb/colls/testcontainer/docs/doc1"
    );
}

#[test]
fn dispatch_falls_back_to_gateway_for_rid_addressed_operations() {
    // Gateway 2.0 derives its DatabaseName/CollectionName routing tokens by
    // parsing the signing link, but a RID-addressed feed signs over a bare
    // lowercased RID with no `dbs`/`colls` segments. Routing such an
    // operation to Gateway 2.0 would fail the wrap locally with
    // CLIENT_BAD_REQUEST before the request is ever sent, so it must fall
    // back to standard Gateway (which routes raw RID paths natively).
    let rid_container = ContainerReference::new_by_rid(
        test_account(),
        "testdb_rid",
        "testcontainer",
        "testcontainer_rid",
        &test_container_props(),
    );
    let operation = CosmosOperation::read_item(ItemReference::from_rid(
        &rid_container,
        PartitionKey::from("pk1"),
        "doc1",
    ));
    assert!(operation.resource_reference().is_rid_addressed());

    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    let (request, _) = dispatched_request(
        &operation,
        &routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::Gateway);
    assert_eq!(request.url.origin(), endpoint.url().origin());
}

#[test]
fn dispatch_falls_back_to_gateway_when_op_ineligible_for_gateway_v2() {
    let operation = CosmosOperation::read_all_databases(test_account());
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    let (request, _) = dispatched_request(
        &operation,
        &routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::Gateway);
    assert_eq!(request.url.origin(), endpoint.url().origin());
}

#[test]
fn dispatch_falls_back_to_gateway_for_full_fidelity_change_feed() {
    // A full-fidelity (AllVersionsAndDeletes) change feed is a
    // `Document`/`ReadFeed` op — otherwise Gateway 2.0 eligible — but must
    // route through the standard gateway because Gateway 2.0 does not
    // forward the `A-IM` header. An incremental change feed on the same
    // endpoint stays on Gateway 2.0.
    let full_fidelity = CosmosOperation::change_feed_all_versions_and_deletes(
        test_container(),
        Some(FeedRange::full()),
    );
    let incremental = CosmosOperation::change_feed(test_container(), Some(FeedRange::full()));
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let full_fidelity_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &full_fidelity,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    let (request, _) = dispatched_request(
        &full_fidelity,
        &full_fidelity_routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(
        request.transport_mode,
        TransportMode::Gateway,
        "full-fidelity change feed must fall back to the standard gateway"
    );
    assert_eq!(request.url.origin(), endpoint.url().origin());

    let incremental_routing = routing::resolve(
        &crate::clock::Clock::default(),
        &incremental,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    let (request, _) = dispatched_request(
        &incremental,
        &incremental_routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(
        request.transport_mode,
        TransportMode::GatewayV2,
        "incremental change feed remains Gateway 2.0 eligible"
    );
}

#[test]
fn dispatch_falls_back_to_gateway_when_account_name_unparseable() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    let (request, _) = dispatched_request(
        &operation,
        &routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        false,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::Gateway);
    assert_eq!(request.url.origin(), endpoint.url().origin());
}

#[test]
fn dispatch_uses_gateway_v2_authority_for_endpoint_key() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let gateway_v2_url = Url::parse("https://central.gateway_v2.azure.com:444/").unwrap();
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "centralus".into(),
        Url::parse("https://central.documents.azure.com:443/").unwrap(),
        gateway_v2_url.clone(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: endpoint,
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    let (request, key) = dispatched_request(
        &operation,
        &routing,
        ResolvedSession::default(),
        PipelineKind::DataPlane,
        true,
        true,
    );
    assert_eq!(request.transport_mode, TransportMode::GatewayV2);
    assert_eq!(request.url.host_str(), Some("central.gateway_v2.azure.com"));
    assert_eq!(key, EndpointKey::try_from(&gateway_v2_url).unwrap());
}

#[test]
fn resolve_endpoint_skips_unavailable_region_when_gateway_v2_is_present() {
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let endpoint = CosmosEndpoint::regional_with_gateway_v2(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
        Url::parse("https://test-westus2-thin.documents.azure.com:444/").unwrap(),
    );
    let fallback_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::TransportError,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![endpoint.clone(), fallback_endpoint.clone()].into(),
        preferred_write_endpoints: vec![endpoint.clone()].into(),
        account_write_endpoints: vec![endpoint].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: true,
        default_endpoint: fallback_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        3,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, fallback_endpoint);
}

#[test]
fn resolve_endpoint_metadata_falls_back_to_hub_when_all_excluded() {
    // Pipeline-routed metadata operations (e.g. read_all_databases,
    // CreateDatabase, CreateContainer, Offer reads) must fall back to
    // the hub regional endpoint when every preferred region is excluded
    // — never the global endpoint. The global endpoint is only used by
    // account-topology fetches, which bypass this routing path.
    let operation = CosmosOperation::read_all_databases(test_account());
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let hub_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
        account_write_endpoints: vec![hub_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        retry_with_state: None,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: vec!["westus2".into(), "eastus".into()],
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
    };

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, hub_endpoint);
    assert!(!routing.endpoint.is_global());
}

#[test]
fn resolve_endpoint_dataplane_excluded_read_region_routes_to_hub() {
    // Only the read region is excluded — the hub write region is still
    // available, so resolve_endpoint should pick it directly from the
    // write endpoint list (not via the last-resort fallback path).
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let hub_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
        account_write_endpoints: vec![hub_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: vec!["westus2".into()],
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
        retry_with_state: None,
    };
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, hub_endpoint);
    assert!(!routing.endpoint.is_global());
}

#[test]
fn resolve_endpoint_dataplane_all_regions_excluded_falls_back_to_hub() {
    // Both the read region AND the hub write region are excluded.
    // resolve_endpoint must still use the hub write region via the
    // last-resort fallback (preferred_write_endpoints[0]), never the
    // global endpoint.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let hub_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
        account_write_endpoints: vec![hub_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: vec!["westus2".into(), "eastus".into()],
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
        retry_with_state: None,
    };
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    // Even with all regions excluded, the hub write region is used as
    // the last-resort fallback for data-plane operations.
    assert_eq!(routing.endpoint, hub_endpoint);
    assert!(!routing.endpoint.is_global());
}

#[test]
fn resolve_endpoint_dataplane_excluded_and_unavailable_still_uses_hub() {
    // When a read endpoint is both user-excluded AND marked unavailable,
    // the data-plane fallback must still use the hub write region.
    // Note: if an endpoint is only unavailable (not excluded),
    // try_select_endpoint already returns it as a deprioritized fallback
    // — this test validates the combined exclusion + unavailability case.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let hub_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let read_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    // Mark the only read endpoint as unavailable.
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        read_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![read_endpoint.clone()].into(),
        preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
        account_write_endpoints: vec![hub_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        excluded_regions: vec!["westus2".into()],
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
        retry_with_state: None,
    };
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    // Excluded + unavailable: data-plane op must get the hub write
    // region, not the global endpoint.
    assert_eq!(routing.endpoint, hub_endpoint);
    assert!(!routing.endpoint.is_global());
}

#[test]
fn resolve_endpoint_dataplane_isolated_region_unavailable_retries_same_region() {
    // Region-isolation guarantee: when the caller excludes every region
    // EXCEPT one (intending to confine the workload to that single
    // region), the SDK must NOT silently fall back to the hub region
    // when the chosen region becomes unavailable. Instead the chosen
    // region is returned as a deprioritized-but-not-replaced endpoint
    // (`first_unavailable`), preserving isolation. The caller's retry
    // budget eventually surfaces the error, which is the correct
    // signal — a workload pinned to one region should fail, not silently
    // cross a region boundary.
    //
    // This is the scenario where the customer wants the hub region
    // excluded (e.g. compliance / cost / blast-radius reasons) and the
    // only retained region (WestUS2 here) is unavailable. We must NOT
    // route to EastUS.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let hub_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let isolated_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    // Mark the caller's chosen region as unavailable.
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        isolated_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    // Account topology: both regions are readable; hub is the write
    // region. The caller's preference puts the isolated region first.
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![isolated_endpoint.clone(), hub_endpoint.clone()].into(),
        preferred_write_endpoints: vec![hub_endpoint.clone()].into(),
        account_write_endpoints: vec![hub_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState {
        container_recreation_retry_attempted: false,
        throughput_control: None,
        location: LocationIndex::initial(0),
        failover_retry_count: 0,
        session_token_retry_count: 0,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 3,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: false,
        hub_region_processing_only: false,
        // Caller excludes the hub region — they want isolation to
        // WestUS2 only.
        excluded_regions: vec!["eastus".into()],
        patch_verification_failed_endpoint_urls: Vec::new(),
        session_retry_routing:
            crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
        partition_key_range_id: None,
        ppaf_write_retry_allowed: false,
        ppcb_active: false,
        pending_write_effects: Default::default(),
        hedge_already_fired: false,
        retry_with_state: None,
    };
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );

    // The isolated region is returned even though it's unavailable —
    // crucially NOT the hub region. This is what preserves
    // region-isolation intent: the caller's retry policy will eventually
    // exhaust and surface the error, but no request ever crosses into
    // the excluded hub region.
    assert_eq!(
        routing.endpoint, isolated_endpoint,
        "region-isolation broken: resolve_endpoint silently fell back \
         to a region the caller excluded"
    );
    assert_ne!(
        routing.endpoint, hub_endpoint,
        "region-isolation broken: resolve_endpoint chose the excluded hub region"
    );
    assert!(!routing.endpoint.is_global());
}

#[test]
fn resolve_endpoint_multi_write_isolated_region_unavailable_read_returns_isolated_not_excluded() {
    // Multi-write (multi-master) account, read op.
    // Topology: preferred_read = [r1, r2], preferred_write = [r1, r2]
    //           (both regions readable AND writable).
    // Caller config: excluded_regions = [r1] (pin workload to r2).
    // Service state: r2 marked unavailable in account.unavailable_endpoints.
    //
    // Expected: r2 is returned as `first_unavailable`. We must NOT leak to
    // r1 — even though r1 is in the preferred list and is the "first
    // preferred region", it was hard-excluded by the caller and must
    // never be selected.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);

    let r1 = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let r2 = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        r2.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![r1.clone(), r2.clone()].into(),
        preferred_write_endpoints: vec![r1.clone(), r2.clone()].into(),
        account_write_endpoints: vec![r1.clone(), r2.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: true,
        default_endpoint: r1.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        vec!["eastus".into()],
        3,
        2,
    );
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, r2,
        "isolation broken on read: resolve_endpoint did not return the \
         caller's isolated region (r2)"
    );
    assert_ne!(
        routing.endpoint, r1,
        "isolation broken on read: resolve_endpoint silently routed to \
         the excluded region (r1)"
    );
}

#[test]
fn resolve_endpoint_multi_write_isolated_region_unavailable_write_retry_returns_isolated_not_excluded(
) {
    // Multi-write (multi-master) account, write op, simulating attempt 2
    // after the first attempt against r2 failed.
    // Topology: preferred_read = [r1, r2], preferred_write = [r1, r2].
    // Caller config: excluded_regions = [r1] (pin workload to r2).
    // Service state: r2 marked unavailable in account.unavailable_endpoints.
    // In-flight state: pending_write_effects contains a partition mark
    //                  for r2, so the in-flight skip set on this attempt
    //                  is [r2].
    //
    // Trace through resolve_endpoint:
    //   pass 1 (skip=[r2]): r1 excluded → continue; r2 in skip →
    //                       continue; returns None.
    //   pass 2 (skip=[]):   r1 excluded → continue; r2 not excluded,
    //                       not in skip, unavailable → recorded as
    //                       first_unavailable; returns Some(r2).
    //   selected = Some(r2). The last-resort
    //   `preferred_write_endpoints[0]` fallback never fires because
    //   pass 2 returned Some.
    //
    // Expected: r2 returned. We must NOT leak to r1.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let r1 = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let r2 = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        r2.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![r1.clone(), r2.clone()].into(),
        preferred_write_endpoints: vec![r1.clone(), r2.clone()].into(),
        account_write_endpoints: vec![r1.clone(), r2.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: true,
        default_endpoint: r1.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        vec!["eastus".into()],
        3,
        2,
    );
    retry_state.is_dataplane = true;
    // Simulate one prior failed write attempt against r2 (westus2).
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("westus2"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, r2,
        "isolation broken on write retry: resolve_endpoint did not \
         return the caller's isolated region (r2)"
    );
    assert_ne!(
        routing.endpoint, r1,
        "isolation broken on write retry: resolve_endpoint silently \
         routed to the excluded region (r1) via the last-resort fallback"
    );
}

#[test]
fn resolve_endpoint_single_write_excluded_hub_satellite_unavailable_read_returns_satellite() {
    // Single-write (single-master) account, read op.
    // Topology: preferred_read = [hub, satellite], preferred_write = [hub]
    //           (only hub is writable; both readable).
    // Caller config: excluded_regions = [hub] (pin reads to satellite).
    // Service state: satellite marked unavailable.
    //
    // For READS on a single-write account, the routing primary list is
    // preferred_read_endpoints (which contains BOTH hub and satellite).
    // Excluding hub leaves satellite as the only eligible region, and
    // even though satellite is unavailable, `first_unavailable` returns
    // it. We must NOT leak to hub on reads — the caller asked for
    // isolation, and the satellite-unavailable case is exactly when the
    // caller wants to see the error, not a silent cross-region hop.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::read_item(item);

    let hub = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let satellite = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        satellite.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![hub.clone(), satellite.clone()].into(),
        preferred_write_endpoints: vec![hub.clone()].into(),
        account_write_endpoints: vec![hub.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: hub.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        vec!["eastus".into()],
        3,
        2,
    );
    retry_state.is_dataplane = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, satellite,
        "isolation broken on single-write read: resolve_endpoint did \
         not return the caller's isolated region (satellite)"
    );
    assert_ne!(
        routing.endpoint, hub,
        "isolation broken on single-write read: resolve_endpoint \
         silently routed to the excluded hub region"
    );
}

#[test]
fn resolve_endpoint_picks_first_available_over_unavailable() {
    let operation = CosmosOperation::read_all_databases(test_account());
    let default_endpoint =
        CosmosEndpoint::global(Url::parse("https://test.documents.azure.com:443/").unwrap());
    let unavailable_endpoint = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let available_endpoint = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        unavailable_endpoint.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![unavailable_endpoint.clone(), available_endpoint.clone()]
            .into(),
        preferred_write_endpoints: vec![default_endpoint.clone()].into(),
        account_write_endpoints: vec![default_endpoint.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: default_endpoint.clone(),
    }));

    let retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    // Available endpoint is preferred over the unavailable one.
    assert_eq!(routing.endpoint, available_endpoint);
}

/// The endpoint-unavailability lifecycle has two halves. The *membership*
/// half (mark -> cooldown -> probe -> restore) is a deterministic in-memory
/// unit test in `pipeline::operation::routing::location_state_store`. This test pins the
/// *routing* half — how a mark is consumed by `resolve_endpoint` — in one
/// `[r1, r2]` multi-write topology, deterministically and in-memory (no
/// network, no sleeps). It asserts, as one ordered sequence, both the
/// demote-to-tail-not-removal behavior and the read/write/both distinction
/// that the single-purpose tests above each cover only one facet of:
///   * a both-affecting reason (`ServiceUnavailable`) demotes r1 for BOTH
///     reads and writes (r2 chosen);
///   * a write-only reason (`WriteForbidden`) demotes r1 for writes only —
///     reads still route to r1;
///   * when every candidate is marked, the marked head is still returned
///     (`first_unavailable`), never dropped — demotion is to the tail, not
///     removal;
///   * once a mark ages past the TTL the endpoint is available again.
#[test]
fn resolve_endpoint_unavailability_demotes_to_tail_by_reason_for_reads_and_writes() {
    let r1 = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let r2 = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let ttl = Duration::from_secs(60);

    let make_location = |unavailable: std::collections::HashMap<
        Url,
        (
            std::time::Instant,
            crate::pipeline::operation::routing::UnavailableReason,
        ),
    >| {
        LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![r1.clone(), r2.clone()].into(),
            preferred_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            account_write_endpoints: vec![r1.clone(), r2.clone()].into(),
            unavailable_endpoints: unavailable,
            multiple_write_locations_enabled: true,
            default_endpoint: r1.clone(),
        }))
    };

    let read_op = CosmosOperation::read_all_databases(test_account());
    let write_item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let write_op = CosmosOperation::create_item(write_item).with_body(b"{}".to_vec());

    // Multi-write retry state so reads use the read list and writes use the
    // write list, both `[r1, r2]`.
    let state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );

    let resolve = |op: &CosmosOperation, loc: &LocationSnapshot| {
        routing::resolve(&crate::clock::Clock::default(), op, &state, loc, ttl).endpoint
    };

    // Case A — both-affecting reason (ServiceUnavailable) demotes r1 for
    // reads AND writes.
    let mut both = std::collections::HashMap::new();
    both.insert(
        r1.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let loc = make_location(both);
    assert_eq!(
        routing::resolve(
            &crate::clock::Clock::default(),
            &read_op,
            &state,
            &loc,
            Duration::ZERO
        )
        .endpoint,
        r1,
        "an operation with zero TTL can use the marked endpoint",
    );
    assert_eq!(
        resolve(&read_op, &loc),
        r2,
        "another operation's TTL still demotes the same unmodified mark",
    );
    assert_eq!(
        resolve(&write_op, &loc),
        r2,
        "a both-affecting mark on r1 must demote it for writes",
    );

    // Case B — write-only reason (WriteForbidden): reads keep r1, writes
    // demote to r2.
    let mut write_forbidden = std::collections::HashMap::new();
    write_forbidden.insert(
        r1.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::WriteForbidden,
        ),
    );
    let loc = make_location(write_forbidden);
    assert_eq!(
        resolve(&read_op, &loc),
        r1,
        "WriteForbidden must not demote r1 for reads",
    );
    assert_eq!(
        resolve(&write_op, &loc),
        r2,
        "WriteForbidden must demote r1 for writes",
    );

    // Case C — every candidate marked: the head is still returned (present,
    // tail-of-rotation), never removed.
    let mut all_marked = std::collections::HashMap::new();
    all_marked.insert(
        r1.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    all_marked.insert(
        r2.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let loc = make_location(all_marked);
    assert_eq!(
        resolve(&read_op, &loc),
        r1,
        "when all candidates are marked, the marked head must still be \
         returned rather than dropped from rotation",
    );

    // Case D — an aged mark (older than the TTL) makes r1 available again.
    let mut expired = std::collections::HashMap::new();
    expired.insert(
        r1.url().clone(),
        (
            std::time::Instant::now() - (ttl + Duration::from_secs(1)),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let loc = make_location(expired);
    assert_eq!(
        resolve(&read_op, &loc),
        r1,
        "a mark older than the TTL must no longer demote r1",
    );
}

// ── PPAF write-retry cross-region fallback ─────────────────────────

fn make_pending_partition_mark_for_region(
    region: &'static str,
) -> crate::pipeline::operation::routing::LocationEffect {
    crate::pipeline::operation::routing::LocationEffect::MarkPartitionUnavailable(
        crate::pipeline::operation::routing::UnavailablePartition {
            partition_key_range_id: None,
            region: Some(region.into()),
            is_read: false,
            is_partitioned_resource: true,
        },
    )
}

#[test]
fn resolve_endpoint_skips_in_flight_failed_write_region() {
    // Multi-write account: writes can rotate across both write regions.
    // After the first attempt to eastus fails, the in-flight skip set
    // should route the next attempt to westus2 instead of repeating eastus.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone(), west.clone()].into(),
        account_write_endpoints: vec![east.clone(), west.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    // Simulate one prior failed attempt against eastus that deferred
    // a partition mark for that region.
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("eastus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, west,
        "in-flight skip set must route the retry to westus2"
    );
}

#[test]
fn routing_fallback_revisits_pending_failures_without_relaxing_exclusions() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());
    let east = make_endpoint("eastus");
    let west = make_endpoint("westus2");
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone(), west.clone()].into(),
        account_write_endpoints: vec![east.clone(), west.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: east.clone(),
    }));
    let mut state =
        recovery::state::OperationRetryState::initial(0, true, vec!["eastus".into()], 3, 2);
    state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("westus2"));
    let clock = crate::clock::Clock::default();
    let candidates = routing::resolve_candidate_endpoints(
        &location,
        &state,
        &routing::RoutingPolicy::new(&operation),
    );

    assert!(routing::try_select_endpoint(
        &clock,
        &operation,
        &state,
        &location.account,
        &candidates,
        routing::InFlightFailures::Exclude,
        Duration::from_secs(60),
    )
    .is_none());
    let (selected, reason) = routing::fallback(
        &clock,
        &operation,
        &state,
        &location.account,
        candidates,
        Duration::from_secs(60),
    );
    assert_eq!(selected, west);
    assert_eq!(reason, None);

    state.excluded_regions.push("westus2".into());
    let exhausted = routing::resolve(
        &clock,
        &operation,
        &state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(exhausted.endpoint, east);
    assert_eq!(exhausted.routing_fallback, None);
}

#[test]
fn resolve_endpoint_honors_excluded_regions() {
    // Pins `excluded_regions` as a hard dataplane write filter.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone(), west.clone()].into(),
        account_write_endpoints: vec![east.clone(), west.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        vec!["westus2".into()],
        3,
        2,
    );
    retry_state.is_dataplane = true;
    // West is excluded; only East should be eligible.
    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, east,
        "excluded_regions must gate selection — got {:?}",
        routing.endpoint
    );
}

#[test]
fn resolve_endpoint_ppaf_write_uses_read_endpoints_as_primary_list() {
    // Single-master account with PPAF: preferred_write_endpoints = [eastus]
    // only, but preferred_read_endpoints = [westus2, eastus] (different
    // ordering — westus2 is the user's preferred region). With PPAF
    // enabled, a write must iterate over the READ list so the first
    // attempt targets westus2 (the user's preferred region) and probes for
    // the actual write region, rather than blindly hitting the only entry
    // in the write list.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![west.clone(), east.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, west,
        "PPAF write must use the read endpoint list (preferred order) as the primary candidate set"
    );
}

#[test]
fn resolve_endpoint_ppaf_falls_back_to_read_region_when_write_exhausted() {
    // Single-master account with PPAF: write_endpoints = [eastus] only.
    // After eastus has failed, PPAF write retry should fall back to a
    // read region (westus2) for cross-regional write region discovery.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("eastus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, west,
        "PPAF write retry must fall back to a read region when all write regions are in the in-flight skip set"
    );
}

#[test]
fn resolve_endpoint_ppaf_override_skipped_when_current_endpoint_failed_in_flight() {
    // Regression test for PPAF write failback infinite-retry bug.
    //
    // Scenario: an earlier successful PPAF discovery recorded an
    // override pointing at `centralus`. A new write begins:
    //   1. resolve_endpoint sees the PPAF override → routes to centralus.
    //   2. centralus is failing back (returns 403/3) → the failure-path
    //      effects are deferred into `pending_write_effects` because
    //      PPAF SM defers writes until success/region-confirming abort.
    //   3. The persistent `failover_overrides[pk_range_id].current_endpoint`
    //      still says centralus (deferred effects haven't been flushed).
    //   4. On retry, the PPAF override branch in resolve_endpoint MUST
    //      consult the in-flight skip set and fall through to the primary
    //      selection (which has already rotated past centralus) — otherwise
    //      every retry hammers the same failed override region.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone()].into(),
        account_write_endpoints: vec![north.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: north.clone(),
    });

    // Build a partition state with PPAF enabled and a stale override
    // entry that points at the now-failing centralus region.
    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: central.clone(),
            first_failed_endpoint: central.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;
    retry_state.partition_key_range_id = Some(pk_range_id);
    // Simulate that the current operation already failed against centralus
    // (override target) and the deferred mark is sitting in the buffer.
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("centralus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, north,
        "PPAF override pointing at a region already in the in-flight skip set must be skipped, \
         so cross-region retry can rotate to a different region instead of looping on the failed override"
    );
}

#[test]
fn resolve_endpoint_ppaf_override_refreshes_current_endpoint_url() {
    // Counterpart to the regression test above: when the override's
    // current_endpoint is NOT in the in-flight skip set, the PPAF
    // override MUST be honored (otherwise prior failover decisions
    // would be silently ignored on every retry).
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional_with_gateway_v2(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        Url::parse("https://current-centralus-thin.documents.azure.com:444/").unwrap(),
    );
    let stale_central = CosmosEndpoint::regional_with_gateway_v2(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
        Url::parse("https://stale-centralus-thin.documents.azure.com:444/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone()].into(),
        account_write_endpoints: vec![north.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: north.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: stale_central,
            first_failed_endpoint: north.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;
    retry_state.partition_key_range_id = Some(pk_range_id);
    // pending_write_effects is empty — first attempt of the operation,
    // so the override should be applied as-is.

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, central,
        "PPAF override with a healthy current_endpoint must be honored"
    );
}

// ── PPCB override skip-closure ────────────────────────────────────

/// Builds a PPCB override that is already over the write-failover threshold.
fn make_partition_state_with_ppcb_override(
    pk_range_id: &crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId,
    override_target: CosmosEndpoint,
) -> crate::pipeline::operation::routing::partition_endpoint_state::PartitionEndpointState {
    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let config = PartitionFailoverOptions::default();
    let mut partitions = PartitionEndpointState::new(config);
    partitions.per_partition_circuit_breaker_enabled = true;
    partitions.circuit_breaker_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: override_target.clone(),
            first_failed_endpoint: override_target,
            failed_endpoints: Default::default(),
            read_failure_count: 100,
            write_failure_count: 100,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    partitions
}

#[test]
fn resolve_endpoint_ppcb_override_skipped_when_in_excluded_regions() {
    // PPCB overrides must not route to caller-excluded regions.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
        account_write_endpoints: vec![north.clone(), central.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: north.clone(),
    });

    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);
    retry_state.excluded_regions = vec![crate::options::Region::from("centralus")];

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, north,
        "PPCB override pointing at a caller-excluded region must be skipped; \
         fall-through to primary selection should pick the non-excluded region"
    );
}

#[test]
fn resolve_endpoint_ppcb_override_skipped_when_current_endpoint_not_in_topology() {
    // Topology refresh does not prune PPCB overrides; this pins the stale-region skip.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    // Post-refresh topology dropped `central`; the PPCB override still points there.
    let account = Arc::new(AccountEndpointState {
        generation: 1,
        preferred_read_endpoints: vec![north.clone()].into(),
        preferred_write_endpoints: vec![north.clone()].into(),
        account_write_endpoints: vec![north.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: north.clone(),
    });

    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);
    // No excluded_regions, no in-flight failures, no endpoint mark —
    // `region_not_in_topology` is the only gate that can fire.

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, north,
        "PPCB override pointing at a region dropped from the post-refresh \
         topology must be skipped; fall-through to primary selection should \
         pick the only surviving region (`north`). The override map is not \
         pruned by topology refresh, so `region_not_in_topology` in \
         `ppcb_should_skip` is the only thing preventing a 1008 loop."
    );
}

#[test]
fn resolve_endpoint_ppcb_override_skipped_when_endpoint_marked_unavailable() {
    // PPCB overrides must not resurrect transport-dead endpoints.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        central.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::TransportError,
        ),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
        account_write_endpoints: vec![north.clone(), central.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: true,
        default_endpoint: north.clone(),
    });

    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, north,
        "PPCB override pointing at an endpoint marked unavailable (e.g. transport-dead) \
         must be skipped so the next attempt does not repeat the same connect failure"
    );
}

#[test]
fn resolve_endpoint_ppcb_override_skipped_when_current_endpoint_failed_in_flight() {
    // PPCB skips override targets already failed by this operation.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone(), central.clone()].into(),
        account_write_endpoints: vec![north.clone(), central.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: north.clone(),
    });

    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let partitions = make_partition_state_with_ppcb_override(&pk_range_id, central.clone());
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("centralus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, north,
        "PPCB override pointing at a region in the in-flight skip set must be skipped"
    );
}

// ── PPAF override does NOT honor excluded / unavailable ────────────

#[test]
fn resolve_endpoint_ppaf_override_honored_even_when_in_excluded_regions() {
    // PPAF targets the single current write region, even if the caller excluded it.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone()].into(),
        account_write_endpoints: vec![north.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: north.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: central.clone(),
            first_failed_endpoint: north.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;
    retry_state.partition_key_range_id = Some(pk_range_id);
    retry_state.excluded_regions = vec![crate::options::Region::from("centralus")];

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, central,
        "PPAF override target must be honored even when in excluded_regions: \
         for a single-master account there is no other write region, so the SDK \
         routes to the PPAF target and surfaces the failure to the caller"
    );
}

#[test]
fn resolve_endpoint_ppaf_override_honored_even_when_endpoint_marked_unavailable() {
    // PPAF must still try the only write region even when it is marked unavailable.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let north = CosmosEndpoint::regional(
        "northcentralus".into(),
        Url::parse("https://test-northcentralus.documents.azure.com:443/").unwrap(),
    );
    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );

    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        central.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::TransportError,
        ),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![north.clone(), central.clone()].into(),
        preferred_write_endpoints: vec![north.clone()].into(),
        account_write_endpoints: vec![north.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: north.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_automatic_failover_enabled = true;
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: central.clone(),
            first_failed_endpoint: north.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.ppaf_write_retry_allowed = true;
    retry_state.partition_key_range_id = Some(pk_range_id);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, central,
        "PPAF override target must be honored even when marked unavailable: \
         single-master account has no other write region to fail over to"
    );
}

#[test]
fn resolve_endpoint_ppcb_override_probe_skipped_when_first_failed_endpoint_failed_in_flight() {
    // ProbeCandidate retries must skip the probe target after it fails in-flight.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let east = CosmosEndpoint::regional(
        "eastus2".into(),
        Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
        preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
        account_write_endpoints: vec![central.clone(), east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: central.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_circuit_breaker_enabled = true;
    partitions.circuit_breaker_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            // Probe routes to `first_failed_endpoint` regardless of
            // `current_endpoint`; both point at centralus here.
            current_endpoint: east.clone(),
            first_failed_endpoint: central.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::ProbeCandidate,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true, // multi-write
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("centralus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_ne!(
        routing.endpoint, central,
        "PPCB ProbeCandidate must skip the probe target when its region is already in \
         the in-flight skip set; otherwise retry pins to the failing probe region"
    );
}

#[test]
fn resolve_endpoint_ppcb_override_honored_when_current_endpoint_not_failed() {
    // Fresh attempts must honor PPCB overrides until the target fails in-flight.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let east = CosmosEndpoint::regional(
        "eastus2".into(),
        Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
        preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
        account_write_endpoints: vec![central.clone(), east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: central.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_circuit_breaker_enabled = true;
    let write_threshold = partitions.config.write_failure_threshold();
    partitions.circuit_breaker_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: east.clone(),
            first_failed_endpoint: central.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: write_threshold as i32 + 10,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);
    // pending_write_effects empty — first attempt of the operation.

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, east,
        "PPCB override with a healthy current_endpoint must be honored on a fresh attempt"
    );
}

#[test]
fn resolve_endpoint_ppcb_override_pinned_to_unavailable_endpoint_reproduces_prod_403_3_loop() {
    // Production-shape repro: PPCB override threshold met while central is unavailable.
    // Multi-write leaves `pending_write_effects` empty, so availability is the only guard.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let east = CosmosEndpoint::regional(
        "eastus2".into(),
        Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
    );

    // Multi-write 403/3 applied the endpoint mark before this retry.
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        central.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::WriteForbidden,
        ),
    );

    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
        preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
        account_write_endpoints: vec![central.clone(), east.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: true,
        default_endpoint: central.clone(),
    });

    use crate::{
        options::PartitionFailoverOptions,
        pipeline::operation::routing::partition_endpoint_state::{
            HealthStatus, PartitionEndpointState, PartitionFailoverEntry,
        },
    };
    let pk_range_id: crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId = "0".parse().unwrap();
    let mut partitions = PartitionEndpointState::new(PartitionFailoverOptions::default());
    partitions.per_partition_circuit_breaker_enabled = true;
    // PPCB override still points at the now-unavailable endpoint.
    partitions.circuit_breaker_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: central.clone(),
            first_failed_endpoint: central.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            // Above the default threshold so PPCB failover triggers.
            write_failure_count: 10,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );
    let location = LocationSnapshot::for_tests_with_partitions(account, Arc::new(partitions));

    // Multi-write applies attempt effects immediately, leaving no in-flight skip set.
    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    retry_state.partition_key_range_id = Some(pk_range_id);

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_ne!(
        routing.endpoint, central,
        "BUG: PPCB override pinned the partition to centralus even though \
         centralus is in account.unavailable_endpoints. This is the \
         production 4-attempt-all-to-central failure mode. The PPCB \
         override-skip path in resolve_endpoint must honor \
         account.unavailable_endpoints (and excluded_regions) the same \
         way try_select_endpoint does."
    );
    assert_eq!(
        routing.endpoint, east,
        "with central in unavailable_endpoints and east the only other write \
         region, resolve_endpoint must route the next attempt to east"
    );
}

#[test]
fn resolve_endpoint_rotates_to_next_region_via_location_index_on_multi_write() {
    // Pins LocationIndex rotation when multi-write PPCB-managed statuses
    // suppress endpoint marks and leave the deferred bucket empty.
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let central = CosmosEndpoint::regional(
        "centralus".into(),
        Url::parse("https://test-centralus.documents.azure.com:443/").unwrap(),
    );
    let east = CosmosEndpoint::regional(
        "eastus2".into(),
        Url::parse("https://test-eastus2.documents.azure.com:443/").unwrap(),
    );

    // The bumped LocationIndex is the only signal that central failed.
    let account = Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![central.clone(), east.clone()].into(),
        preferred_write_endpoints: vec![central.clone(), east.clone()].into(),
        account_write_endpoints: vec![central.clone(), east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: true,
        default_endpoint: central.clone(),
    });
    let location = LocationSnapshot::for_tests(account);

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        2,
    );
    // Mirror production advancement while preserving the empty multi-write bucket.
    retry_state.location = retry_state.location.next_for_generation(2, 0);
    assert_eq!(
        retry_state.location.index(),
        1,
        "precondition: LocationIndex must have advanced from 0 to 1"
    );
    assert!(
        retry_state.pending_write_effects.is_empty(),
        "precondition: multi-write deferred bucket must be empty"
    );

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        routing.endpoint, east,
        "BUG: with LocationIndex=1 and empty in_flight_failed, \
         resolve_endpoint must route to east. The multi-write \
         PPCB-managed path has no other skip signal — if LocationIndex \
         rotation regresses, the next attempt will pin to the just-failed \
         centralus and reproduce the same-region-retry loop."
    );
}

#[test]
fn resolve_endpoint_does_not_skip_for_reads() {
    // The in-flight skip set is only consulted for writes — reads should
    // continue to use the standard preferred-endpoint selection even when
    // pending_write_effects is non-empty (which can never occur
    // for a read in practice, but the invariant should hold defensively).
    let operation = CosmosOperation::read_all_databases(test_account());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );

    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state
        .pending_write_effects
        .push(make_pending_partition_mark_for_region("eastus"));

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, east);
}

// ── flush_pending_write_effects: skip already-applied ──────────────

#[test]
fn already_applied_skips_endpoint_mark_when_endpoint_in_unavailable_set() {
    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        east.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::TransportError,
        ),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let effect = LocationEffect::MarkEndpointUnavailable {
        endpoint: east,
        reason: crate::pipeline::operation::routing::UnavailableReason::TransportError,
    };
    assert!(
        routing::is_effect_already_applied(&effect, &location),
        "endpoint already in unavailable set must be considered already applied"
    );
}

#[test]
fn already_applied_returns_false_when_endpoint_not_in_unavailable_set() {
    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let effect = LocationEffect::MarkEndpointUnavailable {
        endpoint: east,
        reason: crate::pipeline::operation::routing::UnavailableReason::TransportError,
    };
    assert!(!routing::is_effect_already_applied(&effect, &location));
}

#[test]
fn already_applied_skips_partition_mark_when_override_already_moved_past_failed_region() {
    use crate::pipeline::operation::routing::{
        partition_endpoint_state::{HealthStatus, PartitionEndpointState, PartitionFailoverEntry},
        partition_key_range_id::PartitionKeyRangeId,
    };

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );
    let pk_range_id = PartitionKeyRangeId::from(String::from("0"));

    let mut partitions = PartitionEndpointState {
        per_partition_automatic_failover_enabled: true,
        ..Default::default()
    };
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: west.clone(), // already moved off eastus
            first_failed_endpoint: east.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );

    let location = LocationSnapshot::for_tests_with_partitions(
        Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            account_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }),
        Arc::new(partitions),
    );

    let effect = LocationEffect::MarkPartitionUnavailable(
        crate::pipeline::operation::routing::UnavailablePartition {
            partition_key_range_id: Some(pk_range_id),
            region: Some("eastus".into()),
            is_read: false,
            is_partitioned_resource: true,
        },
    );
    assert!(
        routing::is_effect_already_applied(&effect, &location),
        "PPAF override that has already moved past the failed region must be considered already applied"
    );
}

#[test]
fn already_applied_returns_false_when_partition_override_still_on_failed_region() {
    use crate::pipeline::operation::routing::{
        partition_endpoint_state::{HealthStatus, PartitionEndpointState, PartitionFailoverEntry},
        partition_key_range_id::PartitionKeyRangeId,
    };

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let pk_range_id = PartitionKeyRangeId::from(String::from("0"));

    let mut partitions = PartitionEndpointState {
        per_partition_automatic_failover_enabled: true,
        ..Default::default()
    };
    partitions.failover_overrides.insert(
        pk_range_id.clone(),
        PartitionFailoverEntry {
            current_endpoint: east.clone(), // still on the failed region
            first_failed_endpoint: east.clone(),
            failed_endpoints: Default::default(),
            read_failure_count: 0,
            write_failure_count: 0,
            first_failure_time: std::time::Instant::now(),
            last_failure_time: std::time::Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        },
    );

    let location = LocationSnapshot::for_tests_with_partitions(
        Arc::new(AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: vec![east.clone()].into(),
            preferred_write_endpoints: vec![east.clone()].into(),
            account_write_endpoints: vec![east.clone()].into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: east.clone(),
        }),
        Arc::new(partitions),
    );

    let effect = LocationEffect::MarkPartitionUnavailable(
        crate::pipeline::operation::routing::UnavailablePartition {
            partition_key_range_id: Some(pk_range_id),
            region: Some("eastus".into()),
            is_read: false,
            is_partitioned_resource: true,
        },
    );
    assert!(
        !routing::is_effect_already_applied(&effect, &location),
        "override still pointing at the failed region must NOT be skipped"
    );
}

#[test]
fn already_applied_returns_false_when_no_partition_override_exists() {
    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let effect = LocationEffect::MarkPartitionUnavailable(
        crate::pipeline::operation::routing::UnavailablePartition {
            partition_key_range_id: Some(
                crate::pipeline::operation::routing::partition_key_range_id::PartitionKeyRangeId::from(
                    String::from("0"),
                ),
            ),
            region: Some("eastus".into()),
            is_read: false,
            is_partitioned_resource: true,
        },
    );
    assert!(!routing::is_effect_already_applied(&effect, &location));
}

#[test]
fn build_transport_request_sets_is_upsert_header() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::upsert_item(item).with_body(b"{}".to_vec());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    let is_upsert = request
        .headers
        .get_optional_str(&HeaderName::from_static("x-ms-documentdb-is-upsert"))
        .expect("is-upsert header should be set");
    assert_eq!(is_upsert, "true");

    // Upsert targets the collection feed URL, not the individual document.
    assert_eq!(
        request.url.path(),
        "/dbs/testdb/colls/testcontainer/docs",
        "upsert should POST to the collection feed, not /docs/doc1"
    );
}

#[test]
fn build_transport_request_omits_is_upsert_header_for_create() {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-documentdb-is-upsert"))
            .is_none(),
        "is-upsert header should not be set for create"
    );

    // Create targets the collection feed URL, not the individual document.
    assert_eq!(
        request.url.path(),
        "/dbs/testdb/colls/testcontainer/docs",
        "create should POST to the collection feed, not /docs/doc1"
    );
}

#[test]
fn build_transport_request_sets_batch_headers() {
    let operation = CosmosOperation::batch(test_container(), PartitionKey::from("pk1"))
        .with_body(b"[]".to_vec());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        throughput_control: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request")),
        Some("True"),
        "is-batch-request header should be set"
    );
    assert_eq!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-cosmos-batch-atomic")),
        Some("True"),
        "batch-atomic header should be set"
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            "x-ms-cosmos-batch-continue-on-error"
        )),
        Some("False"),
        "batch-continue-on-error header should be set"
    );
}

#[test]
fn build_transport_request_omits_batch_headers_for_create() {
    let container = test_container();
    let operation = CosmosOperation::create_item(ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "doc1",
    ))
    .with_body(b"{}".to_vec());

    let routing = test_routing();
    let activity_id = ActivityId::from_string("default-activity".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        throughput_control: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .expect("request should build");

    assert!(
        request
            .headers
            .get_optional_str(&HeaderName::from_static("x-ms-cosmos-is-batch-request"))
            .is_none(),
        "batch headers should not be set for create"
    );
}

#[test]
fn build_transport_request_sets_priority_level_header() {
    let container = test_container();
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let routing = test_routing();
    let activity_id = ActivityId::new_uuid();

    let throughput_control = Some(ResolvedThroughputControl {
        throughput_bucket: None,
        priority_level: Some(PriorityLevel::Low),
    });

    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .unwrap();

    let priority = request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::PRIORITY_LEVEL,
        ))
        .expect("priority level header should be set");
    assert_eq!(priority, "Low");
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::THROUGHPUT_BUCKET
        ))
        .is_none());
}

#[test]
fn build_transport_request_sets_throughput_bucket_header() {
    let container = test_container();
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let routing = test_routing();
    let activity_id = ActivityId::new_uuid();

    let throughput_control = Some(ResolvedThroughputControl {
        throughput_bucket: Some(42),
        priority_level: None,
    });

    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .unwrap();

    let bucket = request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::THROUGHPUT_BUCKET,
        ))
        .expect("throughput bucket header should be set");
    assert_eq!(bucket, "42");
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::PRIORITY_LEVEL
        ))
        .is_none());
}

#[test]
fn build_transport_request_sets_both_throughput_headers() {
    let container = test_container();
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &container,
        PartitionKey::from("pk1"),
        "doc1",
    ));
    let routing = test_routing();
    let activity_id = ActivityId::new_uuid();

    let throughput_control = Some(ResolvedThroughputControl {
        throughput_bucket: Some(100),
        priority_level: Some(PriorityLevel::High),
    });

    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control,
    };
    let request =
        request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
            .unwrap();

    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::PRIORITY_LEVEL
        )),
        Some("High")
    );
    assert_eq!(
        request.headers.get_optional_str(&HeaderName::from_static(
            request_header_names::THROUGHPUT_BUCKET
        )),
        Some("100")
    );
}

#[test]
fn build_transport_request_auto_emits_query_headers_for_query_operations() {
    let container = test_container();
    let pk_def = container.partition_key_definition().clone();

    // Single-partition item query (scoped to a logical partition via FeedRange)
    let feed_range = FeedRange::for_partition(PartitionKey::from("pk1"), &pk_def);
    let op = CosmosOperation::query_items(container.clone(), Some(feed_range))
        .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
    assert_query_headers_present(&op, "query_items (single partition)");

    // Cross-partition item query (full container via FeedRange::full)
    let op = CosmosOperation::query_items(container, Some(FeedRange::full()))
        .with_body(br#"{"query":"SELECT * FROM c"}"#.to_vec());
    assert_query_headers_present(&op, "query_items (cross partition)");

    // Offer query (used by find_offer / throughput poller path)
    let op = CosmosOperation::query_offers(test_account())
        .with_body(br#"{"query":"SELECT * FROM root"}"#.to_vec());
    assert_query_headers_present(&op, "query_offers");
}

#[test]
fn build_transport_request_sets_patch_content_type() {
    let operation = CosmosOperation::patch_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ))
    .with_body(br#"{"operations":[]}"#.to_vec());
    let routing = test_routing();
    let activity_id = ActivityId::new_uuid();
    let context = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };

    let request = request::build_transport_request(
        &operation,
        &OperationOverrides::default(),
        None,
        &context,
    )
    .expect("PATCH request should build");

    assert_eq!(
        request
            .headers
            .get_optional_str(&azure_core::http::headers::CONTENT_TYPE),
        Some(crate::models::cosmos_headers::PATCH_CONTENT_TYPE)
    );
}

/// Helper: builds a transport request from `op` and asserts the two
/// well-known query headers (`x-ms-documentdb-isquery` and the
/// `application/query+json` content type) are auto-emitted by the pipeline.
fn assert_query_headers_present(op: &CosmosOperation, label: &str) {
    let routing = test_routing();
    let activity_id = ActivityId::new_uuid();
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    let req = request::build_transport_request(op, &OperationOverrides::default(), None, &ctx)
        .expect("request should build");
    assert_eq!(
        req.headers
            .get_optional_str(&HeaderName::from_static(request_header_names::IS_QUERY)),
        Some("True"),
        "{label}: x-ms-documentdb-isquery should be 'True'"
    );
    assert_eq!(
        req.headers
            .get_optional_str(&azure_core::http::headers::CONTENT_TYPE),
        Some(crate::models::cosmos_headers::QUERY_CONTENT_TYPE),
        "{label}: Content-Type should be application/query+json"
    );
}
// ── compute_execution_context ─────────────────────────────────────

fn retry_state_with_counts(
    failover_retry_count: u32,
    session_token_retry_count: u32,
) -> crate::pipeline::operation::recovery::state::OperationRetryState {
    let mut state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        1,
    );
    state.failover_retry_count = failover_retry_count;
    state.session_token_retry_count = session_token_retry_count;
    state
}

#[test]
fn execution_context_initial_when_no_retries() {
    let state = retry_state_with_counts(0, 0);
    assert!(matches!(
        crate::pipeline::operation::diagnostics::execution_context(&state),
        ExecutionContext::Initial
    ));
}

#[test]
fn execution_context_retry_when_session_retry_active() {
    // Session-retry takes precedence over failover-retry: when both
    // counters are non-zero, the most recent advance was the session
    // retry, so the attempt is annotated as an `OperationRetry`.
    let state = retry_state_with_counts(1, 1);
    assert!(matches!(
        crate::pipeline::operation::diagnostics::execution_context(&state),
        ExecutionContext::OperationRetry
    ));

    let state = retry_state_with_counts(0, 1);
    assert!(matches!(
        crate::pipeline::operation::diagnostics::execution_context(&state),
        ExecutionContext::OperationRetry
    ));
}

#[test]
fn execution_context_region_failover_when_only_failover_active() {
    let state = retry_state_with_counts(1, 0);
    assert!(matches!(
        crate::pipeline::operation::diagnostics::execution_context(&state),
        ExecutionContext::RegionFailover
    ));
}

// ── apply_hub_region_header ──────────────────────────────────────
//
// See Spec 0010, Hub-region processing header, §3.4 / public spec §4.2.
// The emission logic itself is a 4-line conditional; these tests
// exercise both branches so AC-1/AC-5 don't drift on a refactor.

fn build_minimal_transport_request() -> crate::pipeline::transport::types::TransportRequest {
    let operation = CosmosOperation::read_all_databases(test_account());
    let routing = test_routing();
    let activity_id = ActivityId::from_string("hub-region-test".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
        throughput_control: None,
    };
    request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
        .expect("request should build")
}

/// T-6 — When the latch is set on `retry_state`, the helper emits
/// `x-ms-cosmos-hub-region-processing-only: True` on the transport
/// request (AC-1).
#[test]
fn transport_request_emits_hub_region_header_when_latched() {
    let mut request = build_minimal_transport_request();
    let mut state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        1,
    );
    state.is_dataplane = true;
    state.hub_region_processing_only = true;

    crate::pipeline::operation::request::apply_hub_region_header(
        &mut request,
        state.hub_region_processing_only,
        None,
    );

    let value = request.headers.get_optional_str(&HeaderName::from_static(
        request_header_names::HUB_REGION_PROCESSING_ONLY,
    ));
    assert_eq!(value, Some("True"));
}

/// T-7 — When the latch is NOT set, the helper does not emit the
/// header. Covers AC-5 / cross-operation isolation guarantee at the
/// emission layer.
#[test]
fn transport_request_omits_hub_region_header_when_not_latched() {
    let mut request = build_minimal_transport_request();
    let state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        1,
    );
    assert!(!state.hub_region_processing_only);

    crate::pipeline::operation::request::apply_hub_region_header(
        &mut request,
        state.hub_region_processing_only,
        None,
    );

    let value = request.headers.get_optional_str(&HeaderName::from_static(
        request_header_names::HUB_REGION_PROCESSING_ONLY,
    ));
    assert!(
        value.is_none(),
        "hub-region header must not be present when latch is unset, got {value:?}",
    );
}

/// Multi-master invariant: the hedge-fallback setter must NOT latch
/// `hub_region_processing_only` on a multi-master account (mirrors
/// `build_session_retry_state`'s topology gate), so the header never
/// leaks and the hub-region paths stay single-master-only. The
/// session-retry counter is still advanced — that cap applies to all
/// account types.
#[test]
fn propagate_hedge_session_unavailable_does_not_latch_on_multi_master() {
    let mut state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        true,
        Vec::new(),
        3,
        3,
    );
    state.is_dataplane = true;

    recovery::hedge::propagate_hedge_session_unavailable(&mut state, true);

    assert!(
        !state.hub_region_processing_only,
        "multi-master must not latch hub_region_processing_only",
    );
    assert_eq!(
        state.session_token_retry_count, 1,
        "session-retry counter must still advance on multi-master",
    );
    assert_eq!(
        state.session_retry_routing,
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredEndpoints,
    );
}

/// Single-master: the hedge-fallback setter latches
/// `hub_region_processing_only` (and advances the session-retry counter),
/// matching the non-hedged `build_session_retry_state` path.
#[test]
fn propagate_hedge_session_unavailable_latches_on_single_master() {
    let mut state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        3,
    );
    state.is_dataplane = true;

    recovery::hedge::propagate_hedge_session_unavailable(&mut state, true);

    assert!(
        state.hub_region_processing_only,
        "single-master must latch hub_region_processing_only",
    );
    assert_eq!(state.session_token_retry_count, 1);
    assert_eq!(
        state.session_retry_routing,
        crate::pipeline::operation::recovery::state::SessionRetryRouting::PreferredWriteEndpoints,
    );
}

/// A failed hedge race that observed 404/1002 must route the fallback to
/// the write region, not the next read region, so the hub-only retry does
/// not hit a non-hub region and receive 403/3.
#[test]
fn hedge_session_unavailable_fallback_routes_to_write_region() {
    let read_endpoints: Vec<CosmosEndpoint> = ["region-r1", "region-r2", "region-r3", "region-w"]
        .into_iter()
        .map(make_endpoint)
        .collect();
    let write_endpoint = make_endpoint("region-w");
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: read_endpoints.clone().into(),
        preferred_write_endpoints: vec![write_endpoint.clone()].into(),
        account_write_endpoints: vec![write_endpoint.clone()].into(),
        unavailable_endpoints: Default::default(),
        multiple_write_locations_enabled: false,
        default_endpoint: write_endpoint.clone(),
    }));
    let mut state = make_advance_test_state(0, read_endpoints.len());
    let operation = CosmosOperation::read_item(ItemReference::from_name(
        &test_container(),
        PartitionKey::from("pk1"),
        "doc1",
    ));

    recovery::hedge::propagate_hedge_session_unavailable(&mut state, true);
    recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        operation.is_read_only(),
        Some(&crate::options::Region::new("region-r1")),
        Some(&crate::options::Region::new("region-r2")),
        dummy_last_error(),
    )
    .expect("budget should allow advance");

    let routing = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(routing.endpoint, write_endpoint);
    assert!(state.hub_region_processing_only);
}

// ── apply_tentative_writes_header ────────────────────────────────
//
// Required on multi-write accounts. Without
// `x-ms-cosmos-allow-tentative-writes: true`, satellite write
// regions reject writes with `403 / 3 (WriteForbidden)`.

fn build_write_transport_request() -> crate::pipeline::transport::types::TransportRequest {
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());
    let routing = test_routing();
    let activity_id = ActivityId::from_string("tentative-writes-test".to_string());
    let ctx = TransportRequestContext {
        routing: &routing,
        activity_id: &activity_id,
        execution_context: ExecutionContext::Initial,
        deadline: None,
        throughput_control: None,
        effective_consistency: DefaultConsistencyLevel::Session,
        read_consistency_strategy: crate::options::ReadConsistencyStrategy::Default,
    };
    request::build_transport_request(&operation, &OperationOverrides::default(), None, &ctx)
        .expect("request should build")
}

fn tentative_writes_header(
    request: &crate::pipeline::transport::types::TransportRequest,
) -> Option<&str> {
    request.headers.get_optional_str(&HeaderName::from_static(
        request_header_names::ALLOW_TENTATIVE_WRITES,
    ))
}

/// Multi-write account + write op → header MUST be emitted as "true".
/// Without it, satellite write regions return
/// `403 / 3 (WriteForbidden)`.
#[test]
fn tentative_writes_header_set_for_write_on_multi_write_account() {
    let mut request = build_write_transport_request();
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    crate::pipeline::operation::request::apply_tentative_writes_header(
        &mut request,
        &operation,
        true,
    );

    assert_eq!(tentative_writes_header(&request), Some("true"));
}

/// Reads on a multi-write account never need the tentative-writes
/// header — they're idempotent and route per the read policy.
#[test]
fn tentative_writes_header_omitted_for_read_on_multi_write_account() {
    let mut request = build_minimal_transport_request();
    let operation = CosmosOperation::read_all_databases(test_account());

    crate::pipeline::operation::request::apply_tentative_writes_header(
        &mut request,
        &operation,
        true,
    );

    assert!(tentative_writes_header(&request).is_none());
}

/// Single-master accounts must not see the header even on writes; the
/// hub region rejects unknown headers in some build configurations
/// and this header is only meaningful for multi-write topologies.
#[test]
fn tentative_writes_header_omitted_for_write_on_single_master_account() {
    let mut request = build_write_transport_request();
    let item = ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1");
    let operation = CosmosOperation::create_item(item).with_body(b"{}".to_vec());

    crate::pipeline::operation::request::apply_tentative_writes_header(
        &mut request,
        &operation,
        false,
    );

    assert!(tentative_writes_header(&request).is_none());
}

// ── apply_failover_delay ──────────────────────────────────────────

#[tokio::test]
async fn failover_delay_none_returns_immediately() {
    let start = std::time::Instant::now();
    crate::pipeline::operation::deadline::apply_failover_delay(
        &crate::clock::Clock::default(),
        None,
        None,
    )
    .await;
    // Allow generous slack for CI scheduling jitter; the goal is to
    // confirm that `None` does not invoke the sleep path at all.
    assert!(start.elapsed() < Duration::from_millis(50));
}

#[tokio::test]
async fn failover_delay_zero_returns_immediately() {
    let start = std::time::Instant::now();
    crate::pipeline::operation::deadline::apply_failover_delay(
        &crate::clock::Clock::default(),
        Some(Duration::ZERO),
        None,
    )
    .await;
    assert!(start.elapsed() < Duration::from_millis(50));
}

#[tokio::test(start_paused = true)]
async fn failover_delay_real_value_actually_sleeps() {
    // Use tokio's pause-time to verify the sleep path is taken
    // without making the test wall-clock-slow.
    let start = tokio::time::Instant::now();
    crate::pipeline::operation::deadline::apply_failover_delay(
        &crate::clock::Clock::default(),
        Some(Duration::from_secs(5)),
        None,
    )
    .await;
    assert!(start.elapsed() >= Duration::from_secs(5));
}

#[tokio::test]
async fn failover_delay_returns_immediately_when_deadline_elapsed() {
    let start = std::time::Instant::now();
    crate::pipeline::operation::deadline::apply_failover_delay(
        &crate::clock::Clock::default(),
        Some(Duration::from_secs(5)),
        Some(std::time::Instant::now()),
    )
    .await;
    assert!(start.elapsed() < Duration::from_millis(50));
}

#[tokio::test(start_paused = true)]
async fn failover_delay_is_capped_to_future_deadline() {
    let start = tokio::time::Instant::now();
    let deadline = std::time::Instant::now() + Duration::from_millis(250);

    crate::pipeline::operation::deadline::apply_failover_delay(
        &crate::clock::Clock::default(),
        Some(Duration::from_secs(5)),
        Some(deadline),
    )
    .await;

    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_millis(200) && elapsed < Duration::from_secs(1),
        "sleep should stop near the future deadline, got {elapsed:?}"
    );
}

// ── enforce_deadline_or_timeout ───────────────────────────────────

fn empty_options_view() -> crate::options::OperationOptionsView<'static> {
    crate::options::OperationOptionsView::new(None, None, None, None)
}

fn test_diagnostics() -> crate::diagnostics::DiagnosticsContextBuilder {
    crate::diagnostics::DiagnosticsContextBuilder::new(
        crate::models::ActivityId::from_string("test-deadline".to_owned()),
        std::sync::Arc::new(crate::options::DiagnosticsOptions::default()),
    )
}

#[test]
fn global_strong_account_validation_preserves_zero_request_diagnostics() {
    let error = crate::pipeline::operation::response::global_strong_account_validation_error(
        test_diagnostics(),
    );

    assert_eq!(
        error.status(),
        crate::error::status_codes::CLIENT_BAD_REQUEST
    );
    assert!(error.response().is_none());
    let diagnostics = error
        .diagnostics()
        .expect("client-side validation must preserve diagnostics");
    assert_eq!(diagnostics.request_count(), 0);
    assert_eq!(
        diagnostics
            .effective_status()
            .map(|status| status.status_code()),
        Some(azure_core::http::StatusCode::BadRequest)
    );
}

#[test]
fn enforce_deadline_none_is_ok() {
    let options = empty_options_view();
    let diagnostics = test_diagnostics();
    let result = crate::pipeline::operation::deadline::enforce_deadline_or_timeout(
        &crate::clock::Clock::default(),
        None,
        &options,
        diagnostics,
    );
    assert!(result.is_ok());
}

#[test]
fn enforce_deadline_in_future_is_ok() {
    let options = empty_options_view();
    let diagnostics = test_diagnostics();
    let deadline = std::time::Instant::now() + Duration::from_secs(60);
    let result = crate::pipeline::operation::deadline::enforce_deadline_or_timeout(
        &crate::clock::Clock::default(),
        Some(deadline),
        &options,
        diagnostics,
    );
    assert!(result.is_ok());
}

#[test]
fn enforce_deadline_in_past_returns_timeout_error_with_diagnostics() {
    let options = empty_options_view();
    let diagnostics = test_diagnostics();
    let deadline = std::time::Instant::now() - Duration::from_millis(1);
    let result = crate::pipeline::operation::deadline::enforce_deadline_or_timeout(
        &crate::clock::Clock::default(),
        Some(deadline),
        &options,
        diagnostics,
    );
    let err = result.expect_err("past deadline should produce an error");
    let msg = err.to_string();
    assert!(
        msg.contains("end-to-end operation timeout exceeded"),
        "unexpected error message: {msg}"
    );
    // Diagnostics must be attached so callers reading
    // `error.diagnostics()` on a timeout outcome get the
    // pipeline's tracked retry history rather than `None`.
    assert!(
        err.diagnostics().is_some(),
        "timeout error must carry finalized diagnostics"
    );
}

// ── hedging::classify_hedge_result (Part 4b) ────────────────────────────────

fn http_result(
    status_code: u16,
    sub_status: Option<u32>,
) -> crate::pipeline::transport::types::TransportResult {
    use azure_core::http::StatusCode;
    let mut status = crate::models::CosmosStatus::new(StatusCode::from(status_code));
    if let Some(v) = sub_status {
        status = status.with_sub_status(v as u16);
    }
    crate::pipeline::transport::types::TransportResult::from_http_response(
        status,
        crate::models::CosmosResponseHeaders::default(),
        Vec::new(),
    )
}

#[test]
fn container_recreation_signals_match_verified_statuses() {
    let available = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        1,
    );
    let exhausted = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        0,
    );

    assert!(recovery::recreation::is_container_recreation_signal(
        &http_result(400, Some(1024)),
        &available,
    ));
    assert!(recovery::recreation::is_container_recreation_signal(
        &http_result(410, Some(1000)),
        &available,
    ));
    assert!(!recovery::recreation::is_container_recreation_signal(
        &http_result(404, Some(1002)),
        &available,
    ));
    assert!(recovery::recreation::is_container_recreation_signal(
        &http_result(404, Some(1002)),
        &exhausted,
    ));
    assert!(!recovery::recreation::is_container_recreation_signal(
        &http_result(410, Some(1024)),
        &available,
    ));
}

#[test]
fn classify_hedge_result_success_is_final() {
    let tr = http_result(200, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
}

#[test]
fn classify_hedge_result_409_conflict_is_final() {
    // 409 is a final HTTP error — terminates hedging.
    let tr = http_result(409, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
}

#[test]
fn classify_hedge_result_503_is_transient() {
    // 503 ServiceUnavailable is transient — keeps the other side racing.
    let tr = http_result(503, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

// ── hedging::classify_secondary_hedge_result (metadata primary-authoritative) ──

#[test]
fn secondary_definitive_error_defers_to_primary_when_metadata() {
    // Metadata primary-authoritative: a secondary 404 (Final, non-success)
    // must NOT win — downgraded to Transient so the primary is awaited.
    let tr = http_result(404, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(Ok(tr), true, false),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

#[test]
fn secondary_definitive_error_wins_when_not_metadata() {
    // Data-plane (primary_authoritative = false) keeps first-Final-wins.
    let tr = http_result(404, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(Ok(tr), false, false),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
}

#[test]
fn secondary_definitive_success_wins_when_metadata() {
    // A secondary definitive success still wins under primary-authoritative
    // (the latency benefit).
    let tr = http_result(200, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(Ok(tr), true, false),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
}

#[test]
fn secondary_transient_stays_transient_when_metadata() {
    let tr = http_result(503, None);
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(Ok(tr), true, false),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

#[test]
fn secondary_write_forbidden_keeps_racing_only_on_hub_only_read() {
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(
            Ok(http_result(403, Some(3))),
            false,
            true
        ),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(
            Ok(http_result(403, Some(3))),
            false,
            false
        ),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
    // Only 403/3 means "not the hub"; other 403s stay final on a hub-only read.
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_secondary_hedge_result(
            Ok(http_result(403, Some(5))),
            false,
            true
        ),
        crate::pipeline::operation::hedging::HedgeClass::Final(_)
    ));
}

// ── OperationOverrides::region_pin ────────────────────────────────

#[test]
fn routing_decision_for_pinned_endpoint_routes_to_the_pinned_endpoint() {
    use crate::{options::Region, pipeline::operation::routing::CosmosEndpoint};
    let url = url::Url::parse("https://acct-westus2.documents.azure.com/").unwrap();
    let pinned = CosmosEndpoint::regional(Region::WEST_US_2, url.clone());

    let routing = routing::select(&pinned);

    assert_eq!(routing.endpoint.region(), Some(&Region::WEST_US_2));
    assert_eq!(*routing.endpoint.url(), url);
}

#[test]
fn no_region_pin_allows_hedging_and_normal_routing() {
    let overrides = crate::pipeline::operation::types::OperationOverrides::default();

    assert!(!overrides.hedging_suppressed());
    assert!(overrides.pinned_endpoint().is_none());
}

#[test]
fn region_pin_without_endpoint_suppresses_hedging_only() {
    // The degraded fallback: the serving region could not be identified, so
    // there is no region to force, but the page still carries a
    // region-affine ETag and must not be raced.
    let overrides = crate::pipeline::operation::types::OperationOverrides {
        region_pin: Some(Box::new(
            crate::pipeline::operation::types::RegionPin::default(),
        )),
        ..Default::default()
    };

    assert!(overrides.hedging_suppressed());
    assert!(overrides.pinned_endpoint().is_none());
}

#[test]
fn region_pin_with_endpoint_suppresses_hedging_and_pins_routing() {
    use crate::{options::Region, pipeline::operation::routing::CosmosEndpoint};
    let url = url::Url::parse("https://acct-westus2.documents.azure.com/").unwrap();
    let overrides = crate::pipeline::operation::types::OperationOverrides {
        region_pin: Some(Box::new(crate::pipeline::operation::types::RegionPin {
            endpoint: Some(CosmosEndpoint::regional(Region::WEST_US_2, url)),
        })),
        ..Default::default()
    };

    assert!(overrides.hedging_suppressed());
    assert_eq!(
        overrides.pinned_endpoint().and_then(|e| e.region()),
        Some(&Region::WEST_US_2)
    );
}

#[test]
fn region_pin_holds_page_two_on_its_region_across_a_failover_retry() {
    // The failure this guards: a `/pkranges` page 2 whose primary region has
    // just been marked unavailable by a failover retry. Normal routing would
    // move that attempt to the next preferred region and send the
    // region-affine change-feed ETag somewhere that never issued it. The
    // STAGE 2 pin must win over `resolve_endpoint` in that state.
    let operation = CosmosOperation::read_all_partition_key_ranges(test_container());

    let east = CosmosEndpoint::regional(
        "eastus".into(),
        Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
    );
    let west = CosmosEndpoint::regional(
        "westus2".into(),
        Url::parse("https://test-westus2.documents.azure.com:443/").unwrap(),
    );

    // East US — the region that served the cold page and issued the ETag —
    // has just been marked unavailable, exactly as an in-flight failover
    // retry would leave it.
    let mut unavailable = std::collections::HashMap::new();
    unavailable.insert(
        east.url().clone(),
        (
            std::time::Instant::now(),
            crate::pipeline::operation::routing::UnavailableReason::ServiceUnavailable,
        ),
    );
    let location = LocationSnapshot::for_tests(Arc::new(AccountEndpointState {
        generation: 0,
        preferred_read_endpoints: vec![east.clone(), west.clone()].into(),
        preferred_write_endpoints: vec![east.clone()].into(),
        account_write_endpoints: vec![east.clone()].into(),
        unavailable_endpoints: unavailable,
        multiple_write_locations_enabled: false,
        default_endpoint: east.clone(),
    }));

    let mut retry_state = crate::pipeline::operation::recovery::state::OperationRetryState::initial(
        0,
        false,
        Vec::new(),
        3,
        2,
    );
    retry_state.failover_retry_count = 1;

    // Baseline: without a pin this attempt leaves East US.
    let unpinned = routing::resolve(
        &crate::clock::Clock::default(),
        &operation,
        &retry_state,
        &location,
        Duration::from_secs(60),
    );
    assert_eq!(
        unpinned.endpoint, west,
        "sanity check: normal routing must fail over off the unavailable region, \
         otherwise this test proves nothing",
    );

    // With the pin, STAGE 2 bypasses `resolve_endpoint` entirely and the
    // page stays on the region that issued its continuation.
    let overrides = crate::pipeline::operation::types::OperationOverrides {
        region_pin: Some(Box::new(crate::pipeline::operation::types::RegionPin {
            endpoint: Some(east.clone()),
        })),
        ..Default::default()
    };
    let pinned = overrides
        .pinned_endpoint()
        .expect("a recorded pin carries its endpoint");
    let routing = routing::select(pinned);

    assert_eq!(
        routing.endpoint, east,
        "a pinned continuation page must stay on its issuing region even when \
         that region is unavailable and a failover retry is in flight",
    );
    assert!(
        overrides.hedging_suppressed(),
        "a pinned continuation page must also never be raced",
    );
}

#[test]
fn classify_hedge_result_404_1002_is_transient() {
    // 404/1002 ReadSessionNotAvailable is retriable.
    let tr = http_result(404, Some(1002));
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

#[test]
fn classify_hedge_result_deadline_exceeded_is_transient() {
    let tr = crate::pipeline::transport::types::TransportResult::deadline_exceeded(
        crate::diagnostics::RequestSentStatus::Sent,
    );
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

#[test]
fn classify_hedge_result_request_build_error_is_transient() {
    let err = crate::error::CosmosError::builder()
        .with_message("synthetic build error")
        .build();
    assert!(matches!(
        crate::pipeline::operation::hedging::classify_hedge_result(Err(err)),
        crate::pipeline::operation::hedging::HedgeClass::Transient
    ));
}

// ── hedging::result_is_final (Part 4b — pre-threshold PrimaryWin gating) ────
//
// Regression tests for the pre-threshold primary-completion branch
// in `hedging::execute`. Before the fix the branch recorded
// `PrimaryWin` unconditionally — including for `Transient` primary
// outcomes — which would perpetually reset the
// consecutive-hedge-win counter under sticky-fast-transient
// traffic, neutralizing the very PPCB signal the counter was
// designed to detect. `result_is_final` is the non-consuming
// classifier used to gate the recording; it must agree with
// `classify_hedge_result` for every input shape.

#[test]
fn result_is_final_success_is_true() {
    let tr = http_result(200, None);
    assert!(crate::pipeline::operation::hedging::result_is_final(&tr));
}

#[test]
fn result_is_final_409_conflict_is_true() {
    let tr = http_result(409, None);
    assert!(crate::pipeline::operation::hedging::result_is_final(&tr));
}

#[test]
fn result_is_final_503_is_false() {
    let tr = http_result(503, None);
    assert!(!crate::pipeline::operation::hedging::result_is_final(&tr));
}

#[test]
fn result_is_final_404_1002_is_false() {
    // 404/1002 ReadSessionNotAvailable is transient — the
    // pre-threshold branch must NOT record PrimaryWin (per the
    // c328cf50 over-correction).
    let tr = http_result(404, Some(1002));
    assert!(!crate::pipeline::operation::hedging::result_is_final(&tr));
}

#[test]
fn result_is_final_deadline_exceeded_is_false() {
    let tr = crate::pipeline::transport::types::TransportResult::deadline_exceeded(
        crate::diagnostics::RequestSentStatus::Sent,
    );
    assert!(!crate::pipeline::operation::hedging::result_is_final(&tr));
}

#[test]
fn result_is_final_429_ru_budget_and_hot_partition_are_final() {
    // RU-budget / hot-partition throttles cannot be relieved by racing a
    // second region, so they short-circuit the race.
    assert!(crate::pipeline::operation::hedging::result_is_final(
        &http_result(429, Some(3200))
    )); // RU_BUDGET_EXCEEDED
    assert!(crate::pipeline::operation::hedging::result_is_final(
        &http_result(429, Some(3210))
    )); // RU_BUDGET_EXCEEDED_FOR_MASTER
    assert!(crate::pipeline::operation::hedging::result_is_final(
        &http_result(429, Some(3214))
    )); // HOT_PARTITION_KEY_THROTTLED
}

#[test]
fn result_is_final_429_generic_and_3092_are_transient() {
    // A generic 429 and the transient-capacity 3092 sub-status keep the
    // other leg racing — another region may have spare capacity.
    assert!(!crate::pipeline::operation::hedging::result_is_final(
        &http_result(429, None)
    ));
    assert!(!crate::pipeline::operation::hedging::result_is_final(
        &http_result(429, Some(3092))
    ));
}

/// Cross-check: every `Ok(tr)` shape that `classify_hedge_result`
/// maps to `HedgeClass::Final` must also be `true` under
/// `result_is_final`, and vice versa. Keeping the two in lockstep
/// is the load-bearing invariant for the pre-threshold PrimaryWin
/// gating fix — drift between them re-opens the c328cf50 bug.
#[test]
fn result_is_final_agrees_with_classify_hedge_result() {
    let cases = [
        http_result(200, None),
        http_result(201, None),
        http_result(204, None),
        http_result(400, None),
        http_result(404, None),
        http_result(404, Some(1002)),
        http_result(409, None),
        http_result(412, None),
        http_result(429, None),
        http_result(429, Some(3092)),
        http_result(429, Some(3200)),
        http_result(429, Some(3210)),
        http_result(429, Some(3214)),
        http_result(500, None),
        http_result(503, None),
        crate::pipeline::transport::types::TransportResult::deadline_exceeded(
            crate::diagnostics::RequestSentStatus::Sent,
        ),
    ];
    for tr in cases {
        let by_peek = crate::pipeline::operation::hedging::result_is_final(&tr);
        let by_classify = matches!(
            crate::pipeline::operation::hedging::classify_hedge_result(Ok(tr)),
            crate::pipeline::operation::hedging::HedgeClass::Final(_)
        );
        assert_eq!(
            by_peek, by_classify,
            "result_is_final must agree with classify_hedge_result",
        );
    }
}

// ── response::finalize_hedge_attempt (Part 4b) ──────────────────────────────

#[test]
fn finalize_hedge_attempt_http_error_returns_error_with_status() {
    let tr = Box::new(http_result(409, None));
    let diagnostics = test_diagnostics();
    let err = crate::pipeline::operation::response::finalize_hedge_attempt(tr, diagnostics)
        .expect_err("409 should be surfaced as an error");
    assert_eq!(u16::from(err.status().status_code()), 409);
}

#[test]
fn finalize_hedge_attempt_deadline_returns_other_error() {
    let tr = Box::new(
        crate::pipeline::transport::types::TransportResult::deadline_exceeded(
            crate::diagnostics::RequestSentStatus::Sent,
        ),
    );
    let diagnostics = test_diagnostics();
    let err = crate::pipeline::operation::response::finalize_hedge_attempt(tr, diagnostics)
        .expect_err("deadline should produce an error");
    assert!(err.to_string().contains("deadline exceeded"));
}

// ── DiagnosticsContextBuilder hedge helpers (Part 4b) ─────────────

#[test]
fn diagnostics_clone_for_hedge_attempt_starts_empty() {
    let mut parent = test_diagnostics();
    let child = parent.clone_for_hedge_attempt();
    // A fresh sub-builder must not carry the parent's request list,
    // status, or accumulated hedge diagnostics.
    assert_eq!(child.request_count(), 0);
}

#[test]
fn diagnostics_merge_hedge_attempt_absorbs_requests() {
    use crate::diagnostics::{TransportHttpVersion, TransportKind};

    let mut parent = test_diagnostics();
    let mut child = parent.clone_for_hedge_attempt();

    let endpoint = crate::pipeline::operation::routing::CosmosEndpoint::global(
        url::Url::parse("https://acct.example/").unwrap(),
    );
    let _ = child.start_request(
        crate::diagnostics::ExecutionContext::Hedging,
        crate::diagnostics::PipelineKind::DataPlane,
        crate::diagnostics::TransportSecurity::Secure,
        TransportKind::Gateway,
        TransportHttpVersion::Http11,
        &endpoint,
    );

    assert_eq!(child.request_count(), 1);
    assert_eq!(parent.request_count(), 0);

    parent.merge_hedge_attempt(child);
    // After merge the parent reflects the absorbed request and the
    // child is consumed.
    assert_eq!(parent.request_count(), 1);
}

// ── application-cancel harvest (Part 4c) ───────────────────────────

#[test]
fn deadline_elapsed_none_is_false() {
    assert!(!crate::pipeline::operation::deadline::deadline_elapsed(
        &crate::clock::Clock::default(),
        None
    ));
}

#[test]
fn deadline_elapsed_future_is_false() {
    let d = std::time::Instant::now() + Duration::from_secs(60);
    assert!(!crate::pipeline::operation::deadline::deadline_elapsed(
        &crate::clock::Clock::default(),
        Some(d)
    ));
}

#[test]
fn deadline_elapsed_past_is_true() {
    let d = std::time::Instant::now() - Duration::from_millis(1);
    assert!(crate::pipeline::operation::deadline::deadline_elapsed(
        &crate::clock::Clock::default(),
        Some(d)
    ));
}

#[test]
fn application_cancelled_error_carries_app_cancel_message() {
    let err = crate::pipeline::operation::deadline::application_cancelled_error(test_diagnostics());
    let msg = err.to_string();
    assert!(
        msg.contains("cancelled by application deadline"),
        "unexpected error message: {msg}"
    );
    // Typed-status invariant: telemetry/retry-evaluation can
    // discriminate a client-side hedge cancel from a service 408.
    let status = err.status();
    assert_eq!(
        status.status_code(),
        azure_core::http::StatusCode::RequestTimeout
    );
    assert_eq!(
        status.sub_status(),
        Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT)
    );
    // Diagnostics-on-error invariant: the synthesized error must
    // carry the operation's diagnostics chain (cf. P0 #1).
    assert!(
        err.diagnostics().is_some(),
        "application_cancelled_error must graft diagnostics"
    );
}

#[test]
fn harvest_window_is_50ms() {
    // Load-bearing spec constant — surfaced as a test so the
    // HARVEST_WINDOW contract is enforced.
    assert_eq!(
        crate::pipeline::operation::deadline::HARVEST_WINDOW,
        Duration::from_millis(50)
    );
}

#[tokio::test]
async fn deadline_signal_none_does_not_complete() {
    // A `None` deadline must produce a never-resolving future so
    // `select` against deadline collapses to whatever the other arm
    // is awaiting (zero deadline observation overhead when unset).
    let fut = crate::pipeline::operation::deadline::deadline_signal(
        &crate::clock::Clock::default(),
        None,
    );
    let timer = Box::pin(azure_core::sleep(
        azure_core::time::Duration::try_from(Duration::from_millis(20)).unwrap(),
    ));
    futures::pin_mut!(fut);
    match futures::future::select(fut, timer).await {
        futures::future::Either::Right(((), _)) => { /* expected */ }
        futures::future::Either::Left(((), _)) => {
            panic!("deadline::deadline_signal(&crate::clock::Clock::default(), None) must never resolve");
        }
    }
}

#[tokio::test]
async fn deadline_signal_past_resolves_immediately() {
    let past = std::time::Instant::now() - Duration::from_millis(10);
    let fut = crate::pipeline::operation::deadline::deadline_signal(
        &crate::clock::Clock::default(),
        Some(past),
    );
    // A short timer that should NOT win this race.
    let timer = Box::pin(azure_core::sleep(
        azure_core::time::Duration::try_from(Duration::from_millis(50)).unwrap(),
    ));
    futures::pin_mut!(fut);
    match futures::future::select(fut, timer).await {
        futures::future::Either::Left(((), _)) => { /* expected */ }
        futures::future::Either::Right(((), _)) => {
            panic!("deadline::deadline_signal(&crate::clock::Clock::default(), past) must resolve before a 50ms sleep");
        }
    }
}

#[tokio::test]
async fn awaiting_hedge_partner_prioritizes_elapsed_operation_deadline() {
    let mut parent = test_diagnostics();
    let attempt = Box::pin(async {
        (
            Err::<crate::pipeline::transport::types::TransportResult, _>(
                crate::error::CosmosError::builder()
                    .with_message("simultaneous attempt timeout")
                    .build(),
            ),
            test_diagnostics(),
        )
    });

    let result = crate::pipeline::operation::deadline::await_attempt_or_deadline_harvest(
        &crate::clock::Clock::default(),
        attempt,
        Some(std::time::Instant::now() - Duration::from_millis(1)),
        &mut parent,
        crate::pipeline::operation::deadline::HARVEST_WINDOW,
    )
    .await;

    assert!(
        result.is_none(),
        "an elapsed operation deadline must win over a simultaneously ready attempt"
    );
}

#[tokio::test]
async fn harvest_remaining_attempt_merges_diagnostics_when_attempt_completes_in_window() {
    // Simulates a hedge attempt that produces a result quickly after
    // the app-cancel deadline fires — the harvest window MUST capture
    // its diagnostics into the parent.
    use crate::diagnostics::{TransportHttpVersion, TransportKind};

    let mut parent = test_diagnostics();
    let mut child = parent.clone_for_hedge_attempt();
    let endpoint = crate::pipeline::operation::routing::CosmosEndpoint::global(
        url::Url::parse("https://acct.example/").unwrap(),
    );
    let _ = child.start_request(
        crate::diagnostics::ExecutionContext::Hedging,
        crate::diagnostics::PipelineKind::DataPlane,
        crate::diagnostics::TransportSecurity::Secure,
        TransportKind::Gateway,
        TransportHttpVersion::Http11,
        &endpoint,
    );

    // The "attempt" completes immediately (well within HARVEST_WINDOW).
    let attempt = Box::pin(async move {
        (
            Err::<crate::pipeline::transport::types::TransportResult, _>(
                crate::error::CosmosError::builder()
                    .with_message("synthetic transport error")
                    .build(),
            ),
            child,
        )
    });

    crate::pipeline::operation::deadline::harvest_remaining_attempt(
        &crate::clock::Clock::default(),
        attempt,
        &mut parent,
        crate::pipeline::operation::deadline::HARVEST_WINDOW,
    )
    .await;
    assert_eq!(parent.request_count(), 1);
}

#[tokio::test(start_paused = true)]
async fn harvest_remaining_attempt_drops_attempt_when_window_exceeded() {
    // A hedge attempt that never completes within HARVEST_WINDOW
    // must be dropped — the parent diagnostics MUST NOT be mutated.
    use crate::diagnostics::{TransportHttpVersion, TransportKind};

    let mut parent = test_diagnostics();
    let mut child = parent.clone_for_hedge_attempt();
    let endpoint = crate::pipeline::operation::routing::CosmosEndpoint::global(
        url::Url::parse("https://acct.example/").unwrap(),
    );
    let _ = child.start_request(
        crate::diagnostics::ExecutionContext::Hedging,
        crate::diagnostics::PipelineKind::DataPlane,
        crate::diagnostics::TransportSecurity::Secure,
        TransportKind::Gateway,
        TransportHttpVersion::Http11,
        &endpoint,
    );

    // The "attempt" sleeps far beyond HARVEST_WINDOW.
    let attempt = Box::pin(async move {
        azure_core::sleep(azure_core::time::Duration::try_from(Duration::from_secs(60)).unwrap())
            .await;
        (
            Err::<crate::pipeline::transport::types::TransportResult, _>(
                crate::error::CosmosError::builder()
                    .with_message("should not reach here")
                    .build(),
            ),
            child,
        )
    });

    // Drive the test clock past HARVEST_WINDOW.
    let parent_before = parent.request_count();
    crate::pipeline::operation::deadline::harvest_remaining_attempt(
        &crate::clock::Clock::default(),
        attempt,
        &mut parent,
        crate::pipeline::operation::deadline::HARVEST_WINDOW,
    )
    .await;
    assert_eq!(parent.request_count(), parent_before);
}

// ── Shared hub-region latch (Part 5) ──────────────────────────

/// T-S5 — Eligibility predicate: data-plane + single-master → build.
#[test]
fn shared_hub_region_latch_eligibility_dataplane_single_master() {
    assert!(
        crate::pipeline::operation::request::should_build_shared_hub_region_latch(
            crate::diagnostics::PipelineKind::DataPlane,
            false, // single-master
        )
    );
}

/// T-S6 — Eligibility predicate: multi-master → skip. Mirrors AC-4.
#[test]
fn shared_hub_region_latch_eligibility_skip_multi_master() {
    assert!(
        !crate::pipeline::operation::request::should_build_shared_hub_region_latch(
            crate::diagnostics::PipelineKind::DataPlane,
            true, // multi-master
        )
    );
}

/// T-S7 — Eligibility predicate: metadata pipeline → skip. Mirrors
/// AC-8 and the data-plane scope gate of
/// Spec 0010: Hub-region processing header.
#[test]
fn shared_hub_region_latch_eligibility_skip_metadata() {
    assert!(
        !crate::pipeline::operation::request::should_build_shared_hub_region_latch(
            crate::diagnostics::PipelineKind::Metadata,
            false,
        )
    );
    assert!(
        !crate::pipeline::operation::request::should_build_shared_hub_region_latch(
            crate::diagnostics::PipelineKind::Metadata,
            true,
        )
    );
}

/// T-S8 — Emission helper: per-state latch alone emits.
#[test]
fn should_emit_hub_region_header_per_state_only() {
    assert!(crate::pipeline::operation::request::should_emit_hub_region_header(true, None));
}

/// T-S9 — Emission helper: shared latch alone (Acquire-load `true`)
/// emits, even when per-state latch is `false`. This is the
/// cross-hedge propagation rule.
#[test]
fn should_emit_hub_region_header_shared_only() {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    let shared = Arc::new(AtomicBool::new(true));
    assert!(
        crate::pipeline::operation::request::should_emit_hub_region_header(false, Some(&shared))
    );
    // Sanity-check the atomic ordering pairing.
    assert!(shared.load(Ordering::Acquire));
}

/// T-S10 — Emission helper: neither latch set → no header. Mirrors
/// AC-5 / `shared_hub_region_latch_no_1002_emits_no_header` from
/// integration tests.
#[test]
fn should_emit_hub_region_header_neither_latched() {
    use std::sync::{atomic::AtomicBool, Arc};
    let shared = Arc::new(AtomicBool::new(false));
    assert!(!crate::pipeline::operation::request::should_emit_hub_region_header(false, None));
    assert!(
        !crate::pipeline::operation::request::should_emit_hub_region_header(false, Some(&shared))
    );
}

/// A late coordinator observation cannot retroactively change the sent-header fact.
#[test]
fn sent_header_fact_survives_late_signal() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let signal = AtomicBool::new(false);
    let sent = request::should_emit_hub_region_header(false, Some(&signal));
    signal.store(true, Ordering::Release);
    assert!(!sent);
    assert!(request::should_emit_hub_region_header(false, Some(&signal)));
}

/// A separate sequential request cannot inherit another race's signal.
#[test]
fn sequential_header_uses_only_owned_state() {
    use std::sync::atomic::AtomicBool;
    let signal = AtomicBool::new(true);
    assert!(request::should_emit_hub_region_header(false, Some(&signal)));
    let mut request = build_minimal_transport_request();
    let state = super::recovery::state::OperationRetryState::initial(0, false, Vec::new(), 3, 1);
    request::apply_hub_region_header(&mut request, state.hub_region_processing_only, None);
    assert!(request
        .headers
        .get_optional_str(&HeaderName::from_static(
            request_header_names::HUB_REGION_PROCESSING_ONLY,
        ))
        .is_none());
}

// ── recovery::hedge::try_advance_after_both_transient (region-aware advancement) ───
//
// After the `evaluate_hedge_eligibility` secondary picker change
// (commit 5dea1c9) the raced secondary is no longer guaranteed
// to sit at `primary_index + 1`. These tests pin the
// `try_advance_after_both_transient` contract: the post-race
// `LocationIndex` must NOT land on either of the two regions the
// race already consumed, regardless of where the picker placed
// the secondary relative to the primary.
fn make_endpoint(region: &'static str) -> CosmosEndpoint {
    CosmosEndpoint::regional(
        region.into(),
        Url::parse(&format!("https://acc-{region}.documents.azure.com:443/"))
            .expect("test endpoint url should parse"),
    )
}

fn make_advance_test_state(
    location_index: usize,
    endpoint_count: usize,
) -> crate::pipeline::operation::recovery::state::OperationRetryState {
    let mut location = LocationIndex::initial(0);
    for _ in 0..location_index {
        location = location.next(endpoint_count);
    }
    crate::pipeline::operation::recovery::state::OperationRetryState {
        location,
        container_recreation_retry_attempted: false,
        throughput_control: None,
        failover_retry_count: 0,
        session_token_retry_count: 0,
        backend_failover_retry_count: 0,
        backend_failover_cumulative_delay: Duration::ZERO,
        #[cfg(feature = "unstable_dtx")]
        dtx_coordinator_retry_count: 0,
        #[cfg(feature = "unstable_dtx")]
        dtx_infra_retry_count: 0,
        max_failover_retries: 10,
        max_backend_failover_retries: 120,
        max_session_retries: 2,
        can_use_multiple_write_locations: false,
        is_dataplane: true,
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
        retry_with_state: None,
    }
}

fn make_advance_test_location(
    regions: &[&'static str],
) -> crate::pipeline::operation::routing::LocationSnapshot {
    let endpoints: Vec<CosmosEndpoint> = regions.iter().copied().map(make_endpoint).collect();
    let default = endpoints[0].clone();
    crate::pipeline::operation::routing::LocationSnapshot::for_tests(Arc::new(
        AccountEndpointState {
            generation: 0,
            preferred_read_endpoints: endpoints.clone().into(),
            preferred_write_endpoints: endpoints.clone().into(),
            account_write_endpoints: endpoints.into(),
            unavailable_endpoints: Default::default(),
            multiple_write_locations_enabled: false,
            default_endpoint: default,
        },
    ))
}

fn dummy_last_error() -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_message("test-both-transient")
        .build()
}

/// Regression: STAGE 7 picker now selects the secondary as the
/// first endpoint with a region/key different from the primary,
/// which can sit at an index earlier than the primary. The
/// post-race advance must skip BOTH raced regions instead of
/// blindly stepping `LocationIndex` by `+2`.
///
/// Endpoints: [A, B, C, D]. Primary promoted to index 2 (C);
/// picker walks from the front and lands on A (index 0) as the
/// secondary. After `BothTransient` the next attempt must target
/// D (the only untried region), not A again.
#[test]
fn try_advance_after_both_transient_skips_raced_regions_when_secondary_is_before_primary() {
    let regions = ["region-a", "region-b", "region-c", "region-d"];
    let location = make_advance_test_location(&regions);
    let mut state = make_advance_test_state(2, regions.len()); // primary = region-c

    let primary = crate::options::Region::new("region-c");
    let secondary = crate::options::Region::new("region-a");

    recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        true,
        Some(&primary),
        Some(&secondary),
        dummy_last_error(),
    )
    .expect("budget should allow advance");

    let landed = location.account.preferred_read_endpoints[state.location.index()].region();
    assert_eq!(
        landed.map(crate::options::Region::as_str),
        Some("region-d"),
        "post-BothTransient LocationIndex must skip the raced primary and \
         the raced secondary (no matter where it sat) and land on the \
         only untried region",
    );
    assert_eq!(state.failover_retry_count, 2);
}

/// Sanity: when the secondary sits immediately after the primary
/// (the historical layout the original `+2` advance assumed), the
/// new implementation still skips both raced regions and lands on
/// the next untried region.
#[test]
fn try_advance_after_both_transient_skips_raced_regions_when_secondary_is_after_primary() {
    let regions = ["region-a", "region-b", "region-c", "region-d"];
    let location = make_advance_test_location(&regions);
    let mut state = make_advance_test_state(0, regions.len()); // primary = region-a

    let primary = crate::options::Region::new("region-a");
    let secondary = crate::options::Region::new("region-b");

    recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        true,
        Some(&primary),
        Some(&secondary),
        dummy_last_error(),
    )
    .expect("budget should allow advance");

    let landed = location.account.preferred_read_endpoints[state.location.index()].region();
    assert_eq!(
        landed.map(crate::options::Region::as_str),
        Some("region-c"),
        "secondary-after-primary case must still skip both raced regions",
    );
}

/// Edge case: only two regions exist and both were raced. The
/// function must terminate (no infinite loop) and charge the two
/// slots against the failover budget. With nowhere untried left
/// to go, the `LocationIndex` is allowed to wrap; the next
/// iteration's `resolve_endpoint` relies on the unavailable-
/// endpoint de-prioritization to make a sensible choice.
#[test]
fn try_advance_after_both_transient_terminates_when_all_regions_are_raced() {
    let regions = ["region-a", "region-b"];
    let location = make_advance_test_location(&regions);
    let mut state = make_advance_test_state(0, regions.len());

    let primary = crate::options::Region::new("region-a");
    let secondary = crate::options::Region::new("region-b");

    let result = recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        true,
        Some(&primary),
        Some(&secondary),
        dummy_last_error(),
    );

    assert!(
        result.is_ok(),
        "should not surface a terminal error when budget remains"
    );
    assert_eq!(
        state.failover_retry_count, 2,
        "two slots are always charged regardless of layout",
    );
}

/// The hedge `BothTransient` boundary must leave the backend-failover
/// budget exactly as it found it: the two legs race concurrently and
/// incur no backoff, so charging them delay would shorten the topology
/// convergence window without any wall-clock time having been spent.
/// Resetting the budget would be equally wrong — a hedged operation would
/// get a fresh 5s on every race.
#[test]
fn try_advance_after_both_transient_preserves_backend_failover_budget() {
    let regions = ["region-a", "region-b", "region-c"];
    let location = make_advance_test_location(&regions);
    let mut state = make_advance_test_state(0, regions.len());
    // Partially-spent backend budget from a prior sequential 403/1008 round.
    state.backend_failover_retry_count = 3;
    state.backend_failover_cumulative_delay = Duration::from_millis(3_000);

    let primary = crate::options::Region::new("region-a");
    let secondary = crate::options::Region::new("region-b");

    let result = recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        true,
        Some(&primary),
        Some(&secondary),
        dummy_last_error(),
    );

    assert!(result.is_ok(), "budget remains, so the race must continue");
    assert_eq!(
        state.failover_retry_count, 2,
        "the race charges the generic failover budget",
    );
    assert_eq!(
        state.backend_failover_retry_count, 3,
        "the concurrent legs must neither consume nor reset the backend retry count",
    );
    assert_eq!(
        state.backend_failover_cumulative_delay,
        Duration::from_millis(3_000),
        "no backoff elapsed during the race, so no delay budget may be charged",
    );
}

/// Budget-exhaustion path remains untouched: when
/// `failover_retry_count + 2 > max_failover_retries`, the
/// function returns the supplied error without mutating
/// `LocationIndex`.
#[test]
fn try_advance_after_both_transient_surfaces_terminal_error_when_budget_exhausted() {
    let regions = ["region-a", "region-b", "region-c"];
    let location = make_advance_test_location(&regions);
    let mut state = make_advance_test_state(0, regions.len());
    state.max_failover_retries = 1;
    let starting_index = state.location.index();

    let primary = crate::options::Region::new("region-a");
    let secondary = crate::options::Region::new("region-b");

    let result = recovery::hedge::try_advance_after_both_transient(
        &mut state,
        &location,
        true,
        Some(&primary),
        Some(&secondary),
        dummy_last_error(),
    );

    assert!(result.is_err(), "exhausted budget must surface terminal");
    assert_eq!(
        state.location.index(),
        starting_index,
        "exhausted budget must not mutate LocationIndex",
    );
    assert_eq!(state.failover_retry_count, 0);
}

/// When a hedge race ends in `BothTransient` and the failover budget
/// is exhausted, the race is the terminal outcome, so the non-terminal
/// path must still carry `strategy_config` + the diagnostic regions on
/// the variant for the budget-exhausted handler to stamp them.
#[test]
fn both_transient_budget_exhausted_carries_hedge_diagnostics() {
    let threshold =
        crate::options::HedgeThreshold::new(std::time::Duration::from_millis(200)).unwrap();
    let strategy_config = crate::diagnostics::HedgingStrategyConfig::new(threshold);
    let primary_for_diag = crate::options::Region::new("region-a");
    let secondary_for_diag = crate::options::Region::new("region-b");

    // Non-terminal both-transient (deadline = None) must return the
    // `BothTransient` variant carrying the data the budget-exhausted
    // handler needs to stamp diagnostics.
    let race = crate::pipeline::operation::hedging::finalize_both_transient(
        &crate::clock::Clock::default(),
        &crate::models::ActivityId::from_string("both-transient-budget".to_owned()),
        None, // deadline not elapsed
        strategy_config,
        Some(crate::options::Region::new("region-a")),
        Some(crate::options::Region::new("region-b")),
        primary_for_diag.clone(),
        secondary_for_diag.clone(),
        test_diagnostics(),
        None,
        false,
    );

    let crate::pipeline::operation::hedging::HedgedRaceResult::BothTransient {
        strategy_config: carried_strategy,
        primary_region_for_diag,
        secondary_region_for_diag,
        mut diagnostics,
        ..
    } = race
    else {
        panic!("deadline-not-elapsed both-transient must return BothTransient");
    };

    // The variant must carry exactly what the handler stamps —
    // otherwise the hedge diagnostics would be silently dropped on
    // the budget-exhausted terminal error.
    assert_eq!(carried_strategy, strategy_config);
    assert_eq!(primary_region_for_diag, primary_for_diag);
    assert_eq!(secondary_region_for_diag, secondary_for_diag);

    // Reproduce the budget-exhausted handler step: stamp the
    // both-transient hedge result with `deadline_elapsed = false`
    // (the budget, not the deadline, ended the race).
    diagnostics.set_hedge_diagnostics(crate::diagnostics::HedgeDiagnostics::both_transient(
        carried_strategy,
        primary_region_for_diag,
        secondary_region_for_diag,
        false,
    ));
    let ctx = diagnostics.complete();

    let hedge = ctx
        .hedge_diagnostics()
        .expect("budget-exhausted both-transient must carry hedge diagnostics");
    assert_eq!(
        hedge.terminal_state(),
        crate::diagnostics::HedgeTerminalState::BothTransient {
            deadline_elapsed: false,
        },
    );
    assert_eq!(hedge.primary_region(), &primary_for_diag);
    assert_eq!(hedge.alternate_region(), Some(&secondary_for_diag));
    // No leg produced a final response in a both-transient terminal.
    assert_eq!(hedge.response_region(), None);
}
