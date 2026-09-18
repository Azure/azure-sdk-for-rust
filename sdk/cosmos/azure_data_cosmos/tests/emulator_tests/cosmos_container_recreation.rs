// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::framework;

use std::{error::Error, num::NonZeroU32, sync::Arc, time::Duration};

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    diagnostics::{DiagnosticsContext, TransportKind},
    feed::{FeedRange, FeedScope},
    models::{
        ContainerProperties, EffectivePartitionKey, PartitionKeyDefinition, ThroughputProperties,
    },
    options::{CreateContainerOptions, ItemReadOptions, MaxItemCountHint, QueryOptions},
    CosmosError, PartitionKey, Query, SubStatusCode, TransactionalBatch,
};
use futures::{StreamExt, TryStreamExt};

use framework::{TestClient, TestOptions, TestRunContext};

const CONTAINER_NAME: &str = "ContainerRecreation";
const PARTITION_KEY_VALUE: &str = "pk1";

fn container_properties(partition_key: PartitionKeyDefinition) -> ContainerProperties {
    ContainerProperties::new(CONTAINER_NAME, partition_key)
}

fn create_options(throughput: u64) -> CreateContainerOptions {
    CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(throughput))
}

fn item_for_path(id: &str, partition_key_path: &str, value: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        (partition_key_path): PARTITION_KEY_VALUE,
        "value": value,
    })
}

async fn recreate_container(
    run_context: &TestRunContext,
    db_client: &DatabaseClient,
    partition_key: PartitionKeyDefinition,
    throughput: u64,
) -> Result<(ContainerProperties, ContainerClient), Box<dyn Error>> {
    let management = run_context.fresh_management_client().await?;
    let management_db = management.database_client(db_client.id());
    management_db
        .container_client(CONTAINER_NAME, None)
        .await?
        .delete(None)
        .await?;
    let properties = management_db
        .create_container(
            container_properties(partition_key),
            Some(create_options(throughput)),
        )
        .await?
        .into_model()?;

    let readiness = run_context.fresh_management_client().await?;
    let readiness_db = readiness.database_client(db_client.id());
    let replacement =
        TestRunContext::wait_for_container_ready(&readiness_db, CONTAINER_NAME).await?;
    Ok((properties, replacement))
}

fn recreation_test_options() -> TestOptions {
    TestOptions::for_emulator()
        .with_gateway_v2_disabled(true)
        .with_timeout(Duration::from_secs(240))
}

fn assert_classic_gateway(diagnostics: Arc<DiagnosticsContext>) {
    assert!(!diagnostics.requests().is_empty());
    for request in diagnostics.requests().iter() {
        assert_eq!(request.transport_kind(), TransportKind::Gateway);
    }
}

fn assert_recreation_signal(error: &CosmosError) {
    let status = error.status();
    assert!(
        matches!(
            (status.status_code(), status.sub_status()),
            (
                StatusCode::BadRequest,
                Some(SubStatusCode::COLLECTION_RID_MISMATCH)
            ) | (
                StatusCode::NotFound,
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
            ) | (StatusCode::Gone, Some(SubStatusCode::NAME_CACHE_STALE))
        ),
        "expected a container recreation signal, got {status}"
    );
}

#[tokio::test]
#[cfg_attr(
    any(
        not(any(test_category = "emulator", test_category = "emulator_inmemory")),
        test_category = "emulator_inmemory_gateway_v2"
    ),
    ignore = "requires classic Gateway; Gateway V2 recreation is tracked separately"
)]
pub async fn changed_definition_recovers_supported_operations() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let stale = run_context
                .create_container(
                    db_client,
                    container_properties("/pk".into()),
                    Some(create_options(400)),
                )
                .await?;
            let original = stale.read(None).await?.into_model()?;

            let (replacement, _) =
                recreate_container(run_context, db_client, "/replacementPk".into(), 600).await?;
            assert_ne!(
                original.system_properties.resource_id,
                replacement.system_properties.resource_id
            );
            assert_eq!(
                replacement.partition_key.paths()[0].as_ref(),
                "/replacementPk"
            );
            let point = item_for_path("point-new-definition", "replacementPk", 1);
            let created = stale
                .create_item(PARTITION_KEY_VALUE, "point-new-definition", &point, None)
                .await?;
            assert_eq!(created.status(), StatusCode::Created);
            assert_eq!(
                created.diagnostics().request_count(),
                2,
                "classic Gateway recovery should issue one stale attempt and one retry"
            );
            assert_classic_gateway(created.diagnostics());
            let read: serde_json::Value = stale
                .read_item(PARTITION_KEY_VALUE, "point-new-definition", None)
                .await?
                .into_model()?;
            assert_eq!(read["id"], point["id"]);
            assert_eq!(read["replacementPk"], point["replacementPk"]);

            recreate_container(run_context, db_client, "/batchPk".into(), 700).await?;
            let batch_item = item_for_path("batch-new-definition", "batchPk", 2);
            let batch = TransactionalBatch::new(PARTITION_KEY_VALUE)
                .create_item(&batch_item)?
                .read_item("batch-new-definition", None);
            let batch = stale
                .execute_transactional_batch(batch, None)
                .await?
                .into_model()?;
            assert_eq!(
                batch
                    .results()
                    .iter()
                    .map(|result| result.status_code())
                    .collect::<Vec<_>>(),
                vec![201, 200]
            );

            let (_, query_generation) =
                recreate_container(run_context, db_client, "/queryPk".into(), 800).await?;
            for (id, value) in [("query-new-definition-1", 3), ("query-new-definition-2", 4)] {
                let item = item_for_path(id, "queryPk", value);
                let created = query_generation
                    .create_item(PARTITION_KEY_VALUE, id, item, None)
                    .await?;
                assert_classic_gateway(created.diagnostics());
            }
            let queried: Vec<serde_json::Value> = Box::pin(stale.query_items(
                Query::from("SELECT * FROM c"),
                FeedScope::partition(PARTITION_KEY_VALUE),
                None,
            ))
            .await?
            .try_collect()
            .await?;
            let mut ids = queried
                .iter()
                .filter_map(|item| item["id"].as_str())
                .collect::<Vec<_>>();
            ids.sort_unstable();
            assert_eq!(
                ids,
                vec!["query-new-definition-1", "query-new-definition-2"]
            );

            let (_, throughput_generation) =
                recreate_container(run_context, db_client, "/throughputPk".into(), 900).await?;
            let throughput = query_generation
                .read_throughput(None)
                .await?
                .expect("replacement container has dedicated throughput");
            assert_eq!(throughput.throughput(), Some(900));

            recreate_container(run_context, db_client, "/replaceThroughputPk".into(), 1000).await?;
            let replaced = throughput_generation
                .begin_replace_throughput(ThroughputProperties::manual(1100), None)
                .await?
                .await?
                .into_model()?;
            assert_eq!(replaced.throughput(), Some(1100));

            Ok(())
        },
        Some(recreation_test_options()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    any(
        not(any(test_category = "emulator", test_category = "emulator_inmemory")),
        test_category = "emulator_inmemory_gateway_v2"
    ),
    ignore = "requires classic Gateway; Gateway V2 recreation is tracked separately"
)]
pub async fn explicit_tokens_do_not_cross_recreation() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let stale = run_context
                .create_container(
                    db_client,
                    container_properties("/pk".into()),
                    Some(create_options(400)),
                )
                .await?;
            let first = item_for_path("old-1", "pk", 1);
            let first_response = stale
                .create_item(PARTITION_KEY_VALUE, "old-1", first, None)
                .await?;
            let old_session = first_response
                .headers()
                .session_token()
                .expect("create response has a session token")
                .clone();
            stale
                .create_item(
                    PARTITION_KEY_VALUE,
                    "old-2",
                    item_for_path("old-2", "pk", 2),
                    None,
                )
                .await?;

            let mut pages = stale
                .query_items::<serde_json::Value>(
                    Query::from("SELECT * FROM c"),
                    FeedScope::partition(PARTITION_KEY_VALUE),
                    Some(
                        QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
                            NonZeroU32::new(1).unwrap(),
                        )),
                    ),
                )
                .await?
                .into_pages();
            pages.next().await.expect("query returns one page")?;
            let old_continuation = pages.to_continuation_token()?;

            let (_, replacement) =
                recreate_container(run_context, db_client, "/replacementPk".into(), 500).await?;
            let session_error = stale
                .read_item(
                    PARTITION_KEY_VALUE,
                    "old-1",
                    Some(ItemReadOptions::default().with_session_token(old_session)),
                )
                .await
                .expect_err("an explicit session token must not cross generations");
            assert_recreation_signal(&session_error);

            let resume = stale
                .query_items::<serde_json::Value>(
                    Query::from("SELECT * FROM c"),
                    FeedScope::partition(PARTITION_KEY_VALUE),
                    Some(QueryOptions::default().with_continuation_token(old_continuation)),
                )
                .await;
            let continuation_error = match resume {
                Err(error) => error,
                Ok(pager) => Box::pin(pager)
                    .try_collect::<Vec<_>>()
                    .await
                    .expect_err("an old continuation must not return replacement data"),
            };
            assert_recreation_signal(&continuation_error);

            let missing = replacement
                .read_item(PARTITION_KEY_VALUE, "old-1", None)
                .await
                .expect_err("old-generation items must not appear in the replacement");
            assert_eq!(missing.status().status_code(), StatusCode::NotFound);
            Ok(())
        },
        Some(recreation_test_options()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    any(
        not(any(test_category = "emulator", test_category = "emulator_inmemory")),
        test_category = "emulator_inmemory_gateway_v2"
    ),
    ignore = "requires classic Gateway; Gateway V2 recreation is tracked separately"
)]
pub async fn stale_range_and_partition_shape_do_not_cross_recreation() -> Result<(), Box<dyn Error>>
{
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let old_definition: PartitionKeyDefinition = ("/tenant", "/user").into();
            let stale = run_context
                .create_container(
                    db_client,
                    container_properties(old_definition.clone()),
                    Some(create_options(400)),
                )
                .await?;
            let old_logical_range = FeedRange::for_partition(
                PartitionKey::from(("tenant-a", "user-a")),
                &old_definition,
            );
            let stale_range = FeedRange::new(
                old_logical_range.min_inclusive().clone(),
                EffectivePartitionKey::MAX,
            )?;

            let (_, replacement) =
                recreate_container(run_context, db_client, "/tenant".into(), 500).await?;
            let replacement_item = replacement
                .create_item(
                    "tenant-a",
                    "replacement",
                    serde_json::json!({"id": "replacement", "tenant": "tenant-a"}),
                    None,
                )
                .await?;
            assert_classic_gateway(replacement_item.diagnostics());

            let range_result = stale
                .query_items::<serde_json::Value>(
                    Query::from("SELECT * FROM c"),
                    FeedScope::range(stale_range),
                    None,
                )
                .await;
            let range_error = match range_result {
                Err(error) => error,
                Ok(pager) => Box::pin(pager)
                    .try_collect::<Vec<_>>()
                    .await
                    .expect_err("a stale EPK range must not return replacement data"),
            };
            assert_eq!(range_error.status().status_code(), StatusCode::BadRequest);
            assert_eq!(
                range_error.status().sub_status(),
                Some(SubStatusCode::COLLECTION_RID_MISMATCH)
            );

            let shape_error = stale
                .create_item(
                    PartitionKey::from(("tenant-a", "user-a")),
                    "incompatible-shape",
                    serde_json::json!({"id": "incompatible-shape", "tenant": "tenant-a"}),
                    None,
                )
                .await
                .expect_err("the old two-component key must fail before replacement dispatch");
            assert_eq!(shape_error.status().status_code(), StatusCode::BadRequest);
            assert_eq!(
                shape_error.status().sub_status(),
                Some(SubStatusCode::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
            );

            let missing = replacement
                .read_item("tenant-a", "incompatible-shape", None)
                .await
                .expect_err("the incompatible item must not reach the replacement");
            assert_eq!(missing.status().status_code(), StatusCode::NotFound);
            Ok(())
        },
        Some(recreation_test_options()),
    )
    .await
}
