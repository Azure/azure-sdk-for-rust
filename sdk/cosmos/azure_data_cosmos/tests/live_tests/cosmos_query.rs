// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::framework::{self, test_client::TEST_MODE_ENV_VAR, test_data, TestClient, TestOptions};
use azure_data_cosmos::{
    feed::FeedScope,
    models::{
        ContainerProperties, FullTextIndex, FullTextPath, FullTextPolicy, IndexingMode,
        IndexingPolicy, ThroughputProperties, VectorDataType, VectorDistanceFunction,
        VectorEmbedding, VectorEmbeddingPolicy, VectorIndex, VectorIndexType,
    },
    options::{
        BinaryEncodingOptions, CreateContainerOptions, FullTextScoreScope, MaxItemCountHint,
        OperationOptions, QueryOptions, QueryPlanMode, Region,
    },
    AccountReference, CosmosClient, Query, RoutingStrategy,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchDocument {
    id: String,
    partition_key: String,
    text: String,
    embedding: [f32; 2],
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn live_ranked_full_text_and_hybrid_search() -> Result<(), Box<dyn Error>> {
    assert!(
        framework::resolve_connection_string().is_some() && !framework::targets_emulator(),
        "ranked full-text coverage requires a live Cosmos DB account"
    );
    TestClient::run_with_unique_db(
        async |run_context, db| {
            let indexing = IndexingPolicy::default()
                .with_indexing_mode(IndexingMode::Consistent)
                .with_included_path("/*")
                .with_excluded_path("/embedding/*")
                .with_full_text_index(FullTextIndex::new("/text"))
                .with_vector_index(VectorIndex::new("/embedding", VectorIndexType::Flat));
            let properties = ContainerProperties::new("SearchContainer", "/partitionKey".into())
                .with_full_text_policy(
                    FullTextPolicy::new("en-US")
                        .with_full_text_path(FullTextPath::new("/text")),
                )
                .with_vector_embedding_policy(VectorEmbeddingPolicy::default().with_embedding(
                    VectorEmbedding::new(
                        "/embedding",
                        VectorDataType::Float32,
                        2,
                        VectorDistanceFunction::Euclidean,
                    ),
                ))
                .with_indexing_policy(indexing);
            let container = run_context
                .create_container(
                    db,
                    properties,
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(11_000)),
                    ),
                )
                .await?;
            assert!(
                container.read_feed_ranges(None).await?.len() > 1,
                "ranked search test requires multiple physical partitions"
            );
            for (id, partition, text, embedding) in [
                ("one", "a", "blue bicycle", [0.0, 0.0]),
                ("two", "b", "red bicycle bicycle", [0.2, 0.0]),
                ("three", "a", "blue mountain bicycle", [0.5, 0.0]),
                ("four", "b", "red skateboard", [1.0, 0.0]),
            ] {
                let document = SearchDocument {
                    id: id.to_owned(),
                    partition_key: partition.to_owned(),
                    text: text.to_owned(),
                    embedding,
                };
                container.create_item(partition, id, &document, None).await?;
            }
            for binary in [false, true] {
                let mut operation = OperationOptions::default();
                operation.binary_encoding =
                    Some(BinaryEncodingOptions::new().with_enabled(binary));
                let options = QueryOptions::default()
                    .with_query_plan_mode(QueryPlanMode::GatewayOnly)
                    .with_operation_options(operation)
                    .with_max_item_count(MaxItemCountHint::Limit(
                        std::num::NonZeroU32::new(1).unwrap(),
                    ));
            for (sql, scope, scope_option, expected) in [
                (
                    "SELECT TOP 3 * FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    FeedScope::full_container(),
                    FullTextScoreScope::Global,
                    &["two", "one", "three"][..],
                ),
                (
                    "SELECT TOP 3 * FROM c ORDER BY RANK RRF(FullTextScore(c.text, @term), FullTextScore(c.text, @second), [2, 1])",
                    FeedScope::full_container(),
                    FullTextScoreScope::Global,
                    &["two", "one", "three"],
                ),
                (
                    "SELECT TOP 3 * FROM c ORDER BY RANK RRF(VectorDistance(c.embedding, @vector), FullTextScore(c.text, @term), [2, 1])",
                    FeedScope::full_container(),
                    FullTextScoreScope::Global,
                    &["one", "two", "three"],
                ),
                (
                    "SELECT TOP 2 * FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    FeedScope::partition("a"),
                    FullTextScoreScope::Local,
                    &["one", "three"],
                ),
                (
                    "SELECT TOP 2 * FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    FeedScope::partition("a"),
                    FullTextScoreScope::Global,
                    &["one", "three"],
                ),
                (
                    "SELECT * FROM c ORDER BY RANK FullTextScore(c.text, @term) OFFSET 1 LIMIT 2",
                    FeedScope::full_container(),
                    FullTextScoreScope::Global,
                    &["one", "three"],
                ),
                (
                    "SELECT TOP @bound * FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    FeedScope::full_container(),
                    FullTextScoreScope::Global,
                    &["two", "one", "three"],
                ),
            ] {
                let query = Query::from(sql)
                    .with_parameter("@term", "bicycle")?
                    .with_parameter("@second", "blue")?
                    .with_parameter("@vector", vec![0.0_f32, 0.0])?
                    .with_parameter("@bound", 3)?;
                let mut pages = container
                    .query_items::<SearchDocument>(
                        query,
                        scope,
                        Some(options.clone().with_full_text_score_scope(scope_option)),
                    )
                    .await?
                    .into_pages();
                assert!(pages.to_continuation_token().is_err());
                let mut ids = Vec::new();
                while let Some(page) = pages.next().await {
                    ids.extend(page?.into_items().into_iter().map(|item| item.id));
                }
                assert_eq!(
                    ids.iter().map(String::as_str).collect::<Vec<_>>(),
                    expected,
                    "ranked query mismatch with binary={binary}: {sql}"
                );
            }
            }
            for (sql, expected) in [
                (
                    "SELECT TOP 2 c.id FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    serde_json::json!({"id": "two"}),
                ),
                (
                    "SELECT TOP 2 VALUE c.id FROM c ORDER BY RANK FullTextScore(c.text, @term)",
                    serde_json::json!("two"),
                ),
            ] {
                let query = Query::from(sql).with_parameter("@term", "bicycle")?;
                let mut pages = container
                    .query_items::<serde_json::Value>(
                        query,
                        FeedScope::full_container(),
                        Some(QueryOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly)),
                    )
                    .await?
                    .into_pages();
                let mut values = Vec::new();
                while let Some(page) = pages.next().await {
                    values.extend(page?.into_items());
                }
                assert_eq!(values.first(), Some(&expected), "{sql}");
                assert_eq!(values.len(), 2, "{sql}");
            }
            Ok(())
        },
        Some(TestOptions::default().with_timeout(std::time::Duration::from_secs(180))),
    )
    .await
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn live_distinct_admission_and_per_query_options() -> Result<(), Box<dyn Error>> {
    let connection = framework::resolve_connection_string()
        .ok_or("live tests require a valid AZURE_COSMOS_CONNECTION_STRING")?;
    assert!(
        !framework::targets_emulator(),
        "live DISTINCT admission coverage requires a live account, not an emulator"
    );
    assert!(
        !std::env::var(TEST_MODE_ENV_VAR).is_ok_and(|mode| mode.eq_ignore_ascii_case("skipped")),
        "explicitly selected live tests cannot use AZURE_COSMOS_TEST_MODE=skipped"
    );
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            println!("Live DISTINCT test database: {}", run_context.db_name());
            test_data::create_container_with_items(
                db_client,
                test_data::generate_mock_items(4, 3),
                None,
            )
            .await?;
            let account = AccountReference::with_authentication_key(
                connection.account_endpoint().parse()?,
                connection.account_key().clone(),
            );
            let expected: Vec<String> = (0..4).map(|i| format!("partition{i}")).collect();
            let unbounded = "SELECT DISTINCT VALUE c.partitionKey FROM c";
            for mode in [QueryPlanMode::LocalPreferred, QueryPlanMode::GatewayOnly] {
                let client = CosmosClient::builder()
                    .build(
                        account.clone(),
                        RoutingStrategy::ProximityTo(Region::EAST_US),
                    )
                    .await?;
                let container = client
                    .database_client(run_context.db_name())
                    .container_client("TestContainer", None)
                    .await?;
                for binary in [false, true] {
                    let mut operation = OperationOptions::default();
                    operation.binary_encoding =
                        Some(BinaryEncodingOptions::new().with_enabled(binary));
                    let options = QueryOptions::default()
                        .with_query_plan_mode(mode)
                        .with_operation_options(operation)
                        .with_max_item_count(MaxItemCountHint::Limit(
                            std::num::NonZeroU32::new(1).unwrap(),
                        ));
                    for (query, maximum, denied) in [
                        (unbounded, None, true),
                        (unbounded, Some(u64::MAX), true),
                        (
                            "SELECT DISTINCT TOP 1001 VALUE c.partitionKey FROM c",
                            None,
                            true,
                        ),
                        (
                            "SELECT DISTINCT TOP 1000 VALUE c.partitionKey FROM c",
                            None,
                            false,
                        ),
                        (
                            "SELECT DISTINCT TOP 1001 VALUE c.partitionKey FROM c",
                            Some(1001),
                            false,
                        ),
                        (
                            "SELECT DISTINCT TOP 4 VALUE c.partitionKey FROM c",
                            Some(3),
                            true,
                        ),
                        (
                            "SELECT DISTINCT TOP 4 VALUE c.partitionKey FROM c",
                            Some(4),
                            false,
                        ),
                    ] {
                        let mut options = options.clone();
                        if let Some(maximum) = maximum {
                            options = options.with_max_buffered_query_window(maximum);
                        }
                        let result = container
                            .query_items::<String>(
                                query,
                                FeedScope::full_container(),
                                Some(options),
                            )
                            .await;
                        if denied {
                            let error = match result {
                                Err(error) => error,
                                Ok(_) => {
                                    panic!("DISTINCT outside finite window must fail at admission")
                                }
                            };
                            assert_eq!(
                                error.status(),
                                azure_data_cosmos_driver::error::status_codes::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW,
                                "{mode:?}, maximum={maximum:?}, binary={binary}"
                            );
                        } else {
                            let mut pages = result?.into_pages();
                            assert_eq!(
                                pages.to_continuation_token().unwrap_err().status(),
                                azure_data_cosmos_driver::error::status_codes::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED
                            );
                            let mut actual = Vec::new();
                            while let Some(page) = pages.next().await {
                                actual.extend(page?.into_items());
                            }
                            actual.sort();
                            assert_eq!(actual, expected);
                        }
                    }
                    for query in [
                        "SELECT DISTINCT TOP @bound VALUE c.partitionKey FROM c",
                        "SELECT DISTINCT VALUE c.partitionKey FROM c OFFSET 1 LIMIT @bound",
                    ] {
                        let mut pages = container
                            .query_items::<String>(
                                Query::from(query).with_parameter("@bound", 2)?,
                                FeedScope::full_container(),
                                Some(options.clone().with_max_buffered_query_window(3)),
                            )
                            .await?
                            .into_pages();
                        let mut actual = Vec::new();
                        while let Some(page) = pages.next().await {
                            actual.extend(page?.into_items());
                        }
                        assert_eq!(actual.len(), 2);
                        actual.sort();
                        actual.dedup();
                        assert_eq!(actual.len(), 2);
                        assert!(actual.iter().all(|value| expected.contains(value)));
                    }
                }
            }
            Ok(())
        },
        Some(TestOptions::default()),
    )
    .await
}
