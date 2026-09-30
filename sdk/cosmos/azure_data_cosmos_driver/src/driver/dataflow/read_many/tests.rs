// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    build, normalize, query_body, selectors, ReadManyRequest, ReadManySelection,
    MAX_BATCH_SELECTIONS,
};
use crate::{
    driver::dataflow::{
        mocks::{
            gone_error, response, response_with_charge, response_with_continuation,
            MockRequestExecutor, MockTopologyProvider, NoopRequestExecutor, NoopTopologyProvider,
        },
        PipelineContext, ResolvedRange,
    },
    models::{
        AccountReference, ContainerProperties, ContainerReference, CosmosOperation, CosmosResponse,
        CosmosStatus, FeedRange, PartitionKeyDefinition, PartitionKeyValue,
    },
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
        "SELECT * FROM c WHERE ((NOT IS_DEFINED(c[\"nested/name\"][\"key\"])))"
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
