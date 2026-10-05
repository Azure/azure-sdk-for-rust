// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    build, normalize, query_body, selectors, ReadManyRequest, ReadManySelection, MAX_BATCH_BYTES,
    MAX_BATCH_SELECTIONS,
};
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineKind, TransportHttpVersion,
        TransportKind, TransportSecurity,
    },
    driver::dataflow::{
        mocks::{
            gone_error, response, response_with_charge, response_with_continuation,
            MockRequestExecutor, MockTopologyProvider, NoopRequestExecutor, NoopTopologyProvider,
        },
        PipelineContext, ResolvedRange,
    },
    models::{
        AccountReference, ActivityId, ContainerProperties, ContainerReference, CosmosOperation,
        CosmosResponse, CosmosStatus, FeedRange, PartitionKeyDefinition, PartitionKeyValue,
        RequestCharge,
    },
    options::DiagnosticsOptions,
};
use azure_core::http::StatusCode;
use serde_json::{json, Value};
use std::sync::Arc;

fn container() -> ContainerReference {
    ContainerReference::new(
        AccountReference::with_master_key(
            "https://test.documents.azure.com".parse().unwrap(),
            "dGVzdA==",
        ),
        "db",
        "db_rid",
        "items",
        "items_rid",
        &ContainerProperties {
            id: "items".into(),
            partition_key: PartitionKeyDefinition::new(vec!["/pk".into()]),
            system_properties: Default::default(),
        },
    )
}

fn full_range() -> ResolvedRange {
    ResolvedRange {
        partition_key_range_id: "0".into(),
        parents: Vec::new(),
        range: FeedRange::full(),
    }
}

#[tokio::test]
async fn rid_addressed_singletons_use_queries_with_or_without_filters() {
    for filter in [None, Some("true".parse().unwrap())] {
        let operation = Arc::new(CosmosOperation::read_many(
            container().into_rid_addressed(),
            ReadManySelection::Items(vec![("pk".into(), "item".into())]),
            filter,
        ));
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
        let mut pipeline = build(
            operation.clone(),
            operation.read_many.as_ref().unwrap(),
            &mut topology,
        )
        .await
        .unwrap();
        let mut executor = MockRequestExecutor::new(vec![Ok(response(
            br#"{"Documents":[{"id":"item","pk":"pk"}]}"#,
        ))]);
        let mut context = PipelineContext::new(&mut executor, None);
        let page = pipeline.next_page(&mut context).await.unwrap().unwrap();
        assert_eq!(
            page.into_body().into_items::<Value>().unwrap(),
            vec![json!({"id":"item","pk":"pk"})]
        );
        assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
        let body: Value = serde_json::from_slice(
            executor.query_bodies[0]
                .as_ref()
                .expect("RID container must use a query"),
        )
        .unwrap();
        assert_eq!(
            body["parameters"],
            json!([{"name":"@__read_many_0","value":"item"},{"name":"@__read_many_1","value":"pk"}])
        );
    }
}

#[tokio::test]
async fn batches_queries_instead_of_issuing_point_reads() {
    let items = (0..MAX_BATCH_SELECTIONS + 1)
        .map(|i| ("pk".into(), format!("d{i}")))
        .collect();
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Items(items),
        None,
    ));
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .unwrap();
    assert_eq!(pipeline.fan_out_width(), 1);
    let mut executor = MockRequestExecutor::new(vec![
        Ok(response(br#"{"Documents":[]}"#)),
        Ok(response(br#"{"Documents":[]}"#)),
    ]);
    let mut context = PipelineContext::new(&mut executor, None);
    assert!(pipeline.next_page(&mut context).await.unwrap().is_some());
    assert!(pipeline.next_page(&mut context).await.unwrap().is_some());
    assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
    assert_eq!(executor.query_bodies.len(), 2);
    for (index, count) in [(0, 64), (1, 1)] {
        let body: Value =
            serde_json::from_slice(executor.query_bodies[index].as_ref().unwrap()).unwrap();
        assert_eq!(body["parameters"].as_array().unwrap().len(), count * 2);
    }
}

#[tokio::test]
async fn serialized_query_byte_limit_is_inclusive() {
    for size in [MAX_BATCH_BYTES - 1, MAX_BATCH_BYTES, MAX_BATCH_BYTES + 1] {
        let selection = ReadManySelection::Partitions(vec!["pk".into()]);
        let filter = "c.payload = @payload"
            .parse::<crate::read_many::ReadManyFilter>()
            .unwrap()
            .with_parameter("@payload", json!(""))
            .unwrap();
        let mut request = ReadManyRequest {
            selection: selection.clone(),
            filter: Some(filter),
        };
        let definition = container().partition_key_definition().clone();
        let identities = normalize(&selection, &definition).unwrap();
        let selectors = selectors(&definition).unwrap();
        let predicate = request.filter.as_ref().unwrap().query_predicate().unwrap();
        let overhead = query_body(&identities, &selectors, &request, Some(&predicate))
            .unwrap()
            .len();
        request
            .filter
            .as_mut()
            .unwrap()
            .parameters
            .insert("@payload".into(), json!("x".repeat(size - overhead)));
        assert_eq!(
            query_body(&identities, &selectors, &request, Some(&predicate))
                .unwrap()
                .len(),
            size
        );
        let operation = Arc::new(CosmosOperation::read_many(
            container(),
            selection,
            request.filter,
        ));
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
        let planned = build(
            operation.clone(),
            operation.read_many.as_ref().unwrap(),
            &mut topology,
        )
        .await;
        if size > MAX_BATCH_BYTES {
            let error = planned.err().expect("oversized single predicate must fail");
            assert_eq!(
                error.status(),
                crate::error::status_codes::CLIENT_BAD_REQUEST
            );
            assert!(error.to_string().contains("exceed the query batch size"));
        } else {
            let mut pipeline = planned.unwrap();
            let mut executor = MockRequestExecutor::new(vec![Ok(response(br#"{"Documents":[]}"#))]);
            let mut context = PipelineContext::new(&mut executor, None);
            assert!(pipeline.next_page(&mut context).await.unwrap().is_some());
            assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
            assert_eq!(executor.query_bodies[0].as_ref().unwrap().len(), size);
        }
    }
}

#[tokio::test]
async fn byte_split_preserves_every_identity_once_across_batches() {
    let expected = (0..5)
        .map(|i| format!("{i}{}", "x".repeat(90_000)))
        .collect::<Vec<_>>();
    let mut items = expected
        .iter()
        .map(|id| ("pk".into(), id.clone()))
        .collect::<Vec<_>>();
    items.push(items[0].clone());
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Items(items),
        None,
    ));
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .unwrap();
    let mut executor = MockRequestExecutor::new(
        (0..5)
            .map(|_| Ok(response(br#"{"Documents":[]}"#)))
            .collect(),
    );
    let mut context = PipelineContext::new(&mut executor, None);
    while pipeline.next_page(&mut context).await.unwrap().is_some() {}
    assert_eq!(executor.query_bodies.len(), 3);
    let mut actual = Vec::new();
    for body in executor.query_bodies {
        let body = body.unwrap();
        assert!(body.len() <= MAX_BATCH_BYTES);
        let value: Value = serde_json::from_slice(&body).unwrap();
        let parameters = value["parameters"].as_array().unwrap();
        for pair in parameters.chunks_exact(2) {
            actual.push(pair[0]["value"].as_str().unwrap().to_owned());
            assert_eq!(pair[1]["value"], "pk");
        }
    }
    assert_eq!(actual, expected);
}

#[tokio::test]
async fn oversized_individual_identity_fails_instead_of_being_dropped() {
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Items(vec![
            ("pk".into(), "x".repeat(MAX_BATCH_BYTES)),
            ("pk".into(), "small".into()),
        ]),
        None,
    ));
    let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
    let error = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .err()
    .expect("oversized identity must fail");
    assert_eq!(
        error.status(),
        crate::error::status_codes::CLIENT_BAD_REQUEST
    );
    assert!(error.to_string().contains("exceed the query batch size"));
}

#[tokio::test]
async fn empty_selection_never_reads_metadata_or_items() {
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Partitions(Vec::new()),
        None,
    ));
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut NoopTopologyProvider,
    )
    .await
    .unwrap();
    let mut executor = NoopRequestExecutor;
    let mut context = PipelineContext::new(&mut executor, None);
    assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
    assert!(pipeline.snapshot_state().is_err());
}

#[test]
fn query_matches_full_identity_and_undefined_not_null() {
    let definition = PartitionKeyDefinition::new(vec!["/id".into()]);
    let request = ReadManyRequest {
        selection: ReadManySelection::Items(vec![("wrong".into(), "id".into())]),
        filter: None,
    };
    let identities = normalize(&request.selection, &definition).unwrap();
    let body: Value = serde_json::from_slice(
        &query_body(
            &identities,
            &selectors(&definition).unwrap(),
            &request,
            None,
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        body["query"],
        "SELECT * FROM c WHERE ((c.id = @__read_many_0 AND c[\"id\"] = @__read_many_1))"
    );
    assert_eq!(
        body["parameters"],
        json!([{"name":"@__read_many_0","value":"id"},{"name":"@__read_many_1","value":"wrong"}])
    );
    let definition = PartitionKeyDefinition::new(vec!["/\"nested/name\"/key".into()]);
    let identities = vec![(PartitionKeyValue::UNDEFINED.into(), None)];
    let body: Value = serde_json::from_slice(
        &query_body(
            &identities,
            &selectors(&definition).unwrap(),
            &request,
            None,
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(
        body["query"],
        "SELECT * FROM c WHERE (((NOT IS_DEFINED(c[\"nested/name\"][\"key\"]) OR IS_OBJECT(c[\"nested/name\"][\"key\"]))))"
    );
}

#[tokio::test]
async fn split_replacements_preserve_predicate_and_continuation() {
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Partitions(vec!["one".into()]),
        Some("c.rank > 0".parse().unwrap()),
    ));
    let child = |id: &str, min: &str, max: &str| ResolvedRange {
        partition_key_range_id: id.into(),
        parents: vec!["0".into()],
        range: FeedRange::new(min.try_into().unwrap(), max.try_into().unwrap()).unwrap(),
    };
    let mut topology = MockTopologyProvider::new(vec![
        Ok(vec![full_range()]),
        Ok(vec![child("1", "", "80"), child("2", "80", "FF")]),
    ]);
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .unwrap();
    let mut executor = MockRequestExecutor::new(vec![
        Ok(response_with_continuation(
            br#"{"Documents":[]}"#,
            Some("page-2"),
        )),
        Err(gone_error()),
        Ok(response(br#"{"Documents":[{"id":"a"}]}"#)),
        Ok(response(br#"{"Documents":[{"id":"b"}]}"#)),
    ]);
    let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
    let mut items = Vec::<Value>::new();
    while let Some(page) = pipeline.next_page(&mut context).await.unwrap() {
        items.extend(page.into_body().into_items::<Value>().unwrap());
    }
    assert_eq!(items, vec![json!({"id":"a"}), json!({"id":"b"})]);
    assert_eq!(
        executor.continuation_calls,
        vec![
            None,
            Some("page-2".into()),
            Some("page-2".into()),
            Some("page-2".into())
        ]
    );
    assert!(executor
        .query_bodies
        .iter()
        .all(|body| body == &executor.query_bodies[0]));
    assert!(pipeline.snapshot_state().is_err());
}

#[tokio::test]
async fn missing_item_charge_is_preserved_but_session_failure_is_not_suppressed() {
    for substatus in [0, 1002] {
        let operation = Arc::new(CosmosOperation::read_many(
            container(),
            ReadManySelection::Items(vec![("pk".into(), "missing".into())]),
            None,
        ));
        let mut topology = MockTopologyProvider::new(vec![Ok(vec![full_range()])]);
        let mut pipeline = build(
            operation.clone(),
            operation.read_many.as_ref().unwrap(),
            &mut topology,
        )
        .await
        .unwrap();
        let charged = response_with_charge(b"{}", 2.5);
        let status = CosmosStatus::new(StatusCode::NotFound).with_sub_status(substatus);
        let wire = CosmosResponse::new(
            Vec::new(),
            charged.headers().clone(),
            status,
            charged.diagnostics(),
        );
        let error = crate::CosmosError::builder().with_response(wire).build();
        let mut executor = MockRequestExecutor::new(vec![Err(error)]);
        let mut context = PipelineContext::new(&mut executor, None);
        let result = pipeline.next_page(&mut context).await;
        if substatus == 0 {
            let page = result.unwrap().unwrap();
            assert_eq!(page.headers().request_charge.unwrap().value(), 2.5);
            assert!(page.into_body().items().unwrap().is_empty());
            assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.status(), status);
            assert_eq!(
                pipeline.next_page(&mut context).await.unwrap_err().status(),
                status
            );
        }
        assert_eq!(executor.query_bodies, vec![None]);
    }
}

fn charged_error(status: CosmosStatus, charge: f64) -> crate::CosmosError {
    let mut diagnostics = DiagnosticsContextBuilder::new(
        ActivityId::new_uuid(),
        Arc::new(DiagnosticsOptions::default()),
    );
    let endpoint = crate::driver::routing::CosmosEndpoint::global(
        "https://test.documents.azure.com".parse().unwrap(),
    );
    let handle = diagnostics.start_request(
        ExecutionContext::Initial,
        PipelineKind::DataPlane,
        TransportSecurity::Secure,
        TransportKind::Gateway,
        TransportHttpVersion::Http11,
        &endpoint,
    );
    diagnostics.update_request(handle, |request| {
        request.with_charge(RequestCharge::new(charge));
        request.complete(status.status_code(), status.sub_status());
    });
    diagnostics.set_operation_status(status.status_code(), status.sub_status());
    crate::CosmosError::builder()
        .with_status(status)
        .with_message("scripted topology failure")
        .with_diagnostics(Arc::new(diagnostics.complete()))
        .build()
}

#[tokio::test]
async fn split_retry_exhaustion_preserves_all_attempts_and_stays_terminal() {
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Partitions(vec!["pk".into()]),
        None,
    ));
    let mut topology = MockTopologyProvider::new(
        (0..=11)
            .map(|generation| {
                let mut range = full_range();
                range.partition_key_range_id = generation.to_string();
                Ok(vec![range])
            })
            .collect(),
    );
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .unwrap();
    let split_status = gone_error().status();
    let mut executor = MockRequestExecutor::new(
        (0..11)
            .map(|_| Err(charged_error(split_status, 2.0)))
            .collect(),
    );
    let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
    for _ in 0..2 {
        let error = pipeline.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::status_codes::CLIENT_SPLIT_RETRIES_EXHAUSTED
        );
        let diagnostics = error
            .diagnostics()
            .expect("split exhaustion must retain attempts");
        assert_eq!(diagnostics.request_count(), 11);
        assert_eq!(diagnostics.total_request_charge().value(), 22.0);
        assert_eq!(diagnostics.effective_status(), Some(error.status()));
        assert_eq!(
            diagnostics
                .requests()
                .iter()
                .map(|request| (*request.status(), request.request_charge().value()))
                .collect::<Vec<_>>(),
            vec![(split_status, 2.0); 11],
        );
    }
    assert_eq!(executor.query_bodies.len(), 11);
    assert_eq!(topology.refresh_calls.len(), 12);
}

#[tokio::test]
async fn accepted_split_at_retry_limit_counts_prior_attempts_only_once() {
    let operation = Arc::new(CosmosOperation::read_many(
        container(),
        ReadManySelection::Partitions(vec!["pk".into()]),
        None,
    ));
    let mut topology_results = (0..10).map(|_| Ok(vec![full_range()])).collect::<Vec<_>>();
    topology_results.push(Ok(vec![
        ResolvedRange {
            partition_key_range_id: "left".into(),
            parents: vec!["0".into()],
            range: FeedRange::new("".try_into().unwrap(), "80".try_into().unwrap()).unwrap(),
        },
        ResolvedRange {
            partition_key_range_id: "right".into(),
            parents: vec!["0".into()],
            range: FeedRange::new("80".try_into().unwrap(), "FF".try_into().unwrap()).unwrap(),
        },
    ]));
    let mut topology = MockTopologyProvider::new(topology_results);
    let mut pipeline = build(
        operation.clone(),
        operation.read_many.as_ref().unwrap(),
        &mut topology,
    )
    .await
    .unwrap();
    let split_status = gone_error().status();
    let mut responses = (0..10)
        .map(|_| Err(charged_error(split_status, 2.0)))
        .collect::<Vec<_>>();
    for charge in [3.0, 4.0] {
        let response = response_with_charge(br#"{"Documents":[]}"#, charge);
        responses.push(Ok(CosmosResponse::new(
            response.body().clone(),
            response.headers().clone(),
            CosmosStatus::new(StatusCode::Ok),
            charged_error(CosmosStatus::new(StatusCode::Ok), charge)
                .diagnostics()
                .unwrap(),
        )));
    }
    let mut executor = MockRequestExecutor::new(responses);
    let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
    for (count, charge) in [(11, 23.0), (1, 4.0)] {
        let page = pipeline.next_page(&mut context).await.unwrap().unwrap();
        let diagnostics = page.diagnostics();
        assert_eq!(diagnostics.request_count(), count);
        assert_eq!(diagnostics.total_request_charge().value(), charge);
    }
    assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
    assert_eq!(executor.query_bodies.len(), 12);
    assert_eq!(topology.refresh_calls.len(), 11);
}

#[tokio::test]
async fn failed_split_repair_retains_current_and_inherited_attempt_diagnostics() {
    for cascading in [false, true] {
        let operation = Arc::new(CosmosOperation::read_many(
            container(),
            ReadManySelection::Partitions(vec!["pk".into()]),
            None,
        ));
        let terminal = crate::error::status_codes::TRANSPORT_CONNECTION_FAILED;
        let mut topology_results = vec![Ok(vec![full_range()])];
        if cascading {
            let mut replacement = full_range();
            replacement.partition_key_range_id = "1".into();
            replacement.parents = vec!["0".into()];
            topology_results.push(Ok(vec![replacement]));
        }
        topology_results.push(Err(charged_error(terminal, 5.0)));
        let mut topology = MockTopologyProvider::new(topology_results);
        let mut pipeline = build(
            operation.clone(),
            operation.read_many.as_ref().unwrap(),
            &mut topology,
        )
        .await
        .unwrap();
        let split_status = gone_error().status();
        let mut responses = vec![Err(charged_error(split_status, 2.0))];
        if cascading {
            responses.push(Err(charged_error(split_status, 3.0)));
        }
        let mut executor = MockRequestExecutor::new(responses);
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let error = pipeline.next_page(&mut context).await.unwrap_err();
        assert_eq!(error.status(), terminal);
        let diagnostics = error.diagnostics().unwrap();
        assert_eq!(diagnostics.request_count(), if cascading { 3 } else { 2 });
        assert_eq!(
            diagnostics.total_request_charge().value(),
            if cascading { 10.0 } else { 7.0 }
        );
        let again = pipeline.next_page(&mut context).await.unwrap_err();
        assert_eq!(
            again.diagnostics().unwrap().total_request_charge(),
            diagnostics.total_request_charge()
        );
        assert_eq!(executor.query_bodies.len(), if cascading { 2 } else { 1 });
    }
}

#[tokio::test]
async fn singleton_topology_retry_preserves_attempts_on_missing_and_terminal_errors() {
    for terminal in [
        CosmosStatus::new(StatusCode::NotFound),
        CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002),
        CosmosStatus::new(StatusCode::RequestTimeout).with_sub_status(20001),
        crate::error::status_codes::TRANSPORT_CONNECTION_FAILED,
    ] {
        let operation = Arc::new(CosmosOperation::read_many(
            container(),
            ReadManySelection::Items(vec![("pk".into(), "missing".into())]),
            None,
        ));
        let mut replacement = full_range();
        replacement.partition_key_range_id = "1".into();
        replacement.parents = vec!["0".into()];
        let mut topology = MockTopologyProvider::new(vec![
            Ok(vec![full_range()]),
            Ok(vec![full_range()]),
            Ok(vec![replacement]),
        ]);
        let mut pipeline = build(
            operation.clone(),
            operation.read_many.as_ref().unwrap(),
            &mut topology,
        )
        .await
        .unwrap();
        let split_status = gone_error().status();
        let prior = charged_error(split_status, 2.0);
        let terminal_error = charged_error(terminal, 3.0);
        let wire = response_with_charge(br#"{"code":"terminal"}"#, 3.0);
        let headers = wire.headers().clone();
        let terminal_error = if terminal.status_code() == StatusCode::ServiceUnavailable {
            terminal_error
        } else {
            crate::CosmosError::builder()
                .with_message("terminal retry response")
                .with_response(CosmosResponse::new(
                    wire.into_body(),
                    headers.clone(),
                    terminal,
                    terminal_error.diagnostics().unwrap(),
                ))
                .build()
        };
        let mut executor = MockRequestExecutor::new(vec![Err(prior), Err(terminal_error)]);
        let mut context = PipelineContext::new(&mut executor, Some(&mut topology));
        let result = pipeline.next_page(&mut context).await;
        let diagnostics = if terminal == CosmosStatus::new(StatusCode::NotFound) {
            let page = result.unwrap().unwrap();
            assert_eq!(page.status().status_code(), StatusCode::Ok);
            assert_eq!(page.headers().request_charge, headers.request_charge);
            let diagnostics = page.diagnostics();
            assert!(page.into_body().items().unwrap().is_empty());
            assert!(pipeline.next_page(&mut context).await.unwrap().is_none());
            diagnostics
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.status(), terminal);
            if terminal.status_code() == StatusCode::ServiceUnavailable {
                assert!(error.response().is_none());
                assert!(error.to_string().contains("scripted topology failure"));
            } else {
                let response = error
                    .response()
                    .expect("wire error payload must survive aggregation");
                assert_eq!(response.status(), terminal);
                assert_eq!(response.headers().request_charge, headers.request_charge);
                assert_eq!(
                    response.clone().into_body().single().unwrap().as_ref(),
                    br#"{"code":"terminal"}"#
                );
                assert!(error.to_string().contains("terminal retry response"));
            }
            let repeated = pipeline.next_page(&mut context).await.unwrap_err();
            assert_eq!(repeated.status(), terminal);
            assert_eq!(
                repeated
                    .diagnostics()
                    .unwrap()
                    .total_request_charge()
                    .value(),
                5.0
            );
            error.diagnostics().unwrap()
        };
        assert_eq!(diagnostics.request_count(), 2, "{terminal:?}");
        assert_eq!(
            diagnostics.total_request_charge().value(),
            5.0,
            "{terminal:?}"
        );
        assert_eq!(
            diagnostics
                .requests()
                .iter()
                .map(|request| (*request.status(), request.request_charge().value()))
                .collect::<Vec<_>>(),
            vec![(split_status, 2.0), (terminal, 3.0)],
        );
        assert_eq!(executor.query_bodies, vec![None, None]);
        assert_eq!(
            executor.refresh_calls,
            vec![
                super::PartitionRoutingRefresh::UseCached,
                super::PartitionRoutingRefresh::ForceRefresh,
            ]
        );
    }
}
