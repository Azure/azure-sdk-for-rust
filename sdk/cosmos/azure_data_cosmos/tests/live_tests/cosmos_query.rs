// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::framework::{self, test_client::TEST_MODE_ENV_VAR, test_data, TestClient, TestOptions};
use azure_data_cosmos::{
    clients::ContainerClient,
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
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchDocument {
    id: String,
    partition_key: String,
    text: String,
    embedding: [f32; 2],
}

#[derive(Deserialize)]
struct SearchIndex {
    index: u32,
}

fn collection_create_in_progress(error: &azure_data_cosmos::CosmosError) -> bool {
    error.status().status_code() == azure_core::http::StatusCode::NotFound
        && error.status().sub_status()
            == Some(
                azure_data_cosmos_driver::error::status_codes::substatus::COLLECTION_CREATE_IN_PROGRESS,
            )
}

async fn create_search_item(
    container: &ContainerClient,
    partition: &str,
    id: &str,
    item: &impl Serialize,
) -> azure_data_cosmos::Result<()> {
    for attempt in 0..60 {
        match container
            .create_item(partition.to_owned(), id, item, None)
            .await
        {
            Ok(_) => return Ok(()),
            Err(error) if attempt < 59 && collection_create_in_progress(&error) => {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(error) => return Err(error),
        }
    }
    unreachable!("the final attempt returns above")
}

async fn collect_ranked_items<T: DeserializeOwned + Send + 'static>(
    container: &ContainerClient,
    query: Query,
    scope: FeedScope,
    options: QueryOptions,
) -> azure_data_cosmos::Result<(Vec<T>, usize)> {
    for attempt in 0..60 {
        let result = async {
            let mut pages = container
                .query_items::<T>(query.clone(), scope.clone(), Some(options.clone()))
                .await?
                .into_pages();
            assert!(pages.to_continuation_token().is_err());
            let mut items = Vec::new();
            let mut page_count = 0;
            while let Some(page) = pages.next().await {
                page_count += 1;
                items.extend(page?.into_items());
            }
            Ok((items, page_count))
        }
        .await;
        match result {
            Ok(items) => return Ok(items),
            Err(error) if attempt < 59 && collection_create_in_progress(&error) => {
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
            Err(error) => return Err(error),
        }
    }
    unreachable!("the final attempt returns above")
}

async fn ranked_indices(
    container: &ContainerClient,
    query: Query,
    scope: FeedScope,
    options: QueryOptions,
) -> azure_data_cosmos::Result<(Vec<u32>, usize)> {
    let (items, page_count) =
        collect_ranked_items::<SearchIndex>(container, query, scope, options).await?;
    Ok((
        items.into_iter().map(|item| item.index).collect(),
        page_count,
    ))
}

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn live_ranked_search_with_python_dataset() -> Result<(), Box<dyn Error>> {
    assert!(
        framework::resolve_connection_string().is_some() && !framework::targets_emulator(),
        "ranked full-text coverage requires a live Cosmos DB account"
    );
    TestClient::run_with_unique_db(
        async |run_context, db| {
            // The original 1536-dimensional vectors are queried without an index, as in Python;
            // this account supports vector indexes only up to 505 dimensions.
            let indexing = IndexingPolicy::default()
                .with_indexing_mode(IndexingMode::Consistent)
                .with_included_path("/*")
                .with_full_text_index(FullTextIndex::new("/text"))
                .with_full_text_index(FullTextIndex::new("/title"));
            let properties = ContainerProperties::new("SearchDataset", "/pk".into())
                .with_full_text_policy(
                    FullTextPolicy::new("en-US")
                        .with_full_text_path(FullTextPath::new("/text"))
                        .with_full_text_path(FullTextPath::new("/title")),
                )
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
            let ranges = container.read_feed_ranges(None).await?;
            assert!(
                ranges.len() > 1,
                "test requires multiple physical partitions"
            );

            // Derived from Azure/azure-sdk-for-python at 36703029b76ef5e40e87f2122e65e983f97c451e,
            // sdk/cosmos/azure-cosmos/tests/hybrid_search_data.py:get_hybrid_search_items.
            let mut items: Vec<serde_json::Value> =
                serde_json::from_str(include_str!("fixtures/hybrid_search_data.json"))?;
            assert_eq!(items.len(), 100);
            assert!(items.iter().all(|item| item["vector"]
                .as_array()
                .is_some_and(|vector| vector.len() == 1536)));
            for (position, item) in items.iter_mut().enumerate() {
                let id = position.to_string();
                // Spread matches over multiple ranges; the check below verifies actual placement.
                let pk = (position % 8).to_string();
                item["id"] = serde_json::json!(id);
                item["pk"] = serde_json::json!(pk);
                create_search_item(&container, &pk, &id, item).await?;
            }

            let expected = [61, 49, 51, 24, 54, 75, 77, 76, 80, 2, 22, 57, 85];
            let text_query = "SELECT TOP 13 c.index FROM c WHERE FullTextContains(c.text, @term) \
                ORDER BY RANK FullTextScore(c.text, @term)";
            let physical_options =
                QueryOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly);
            let mut matching_ranges = 0;
            for range in ranges {
                let (indices, _) = ranked_indices(
                    &container,
                    Query::from(text_query).with_parameter("@term", "United States")?,
                    FeedScope::range(range),
                    physical_options.clone(),
                )
                .await?;
                matching_ranges += usize::from(!indices.is_empty());
            }
            assert!(
                matching_ranges > 1,
                "matching items must span multiple physical partitions"
            );

            for binary in [false, true] {
                let mut operation = OperationOptions::default();
                operation.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(binary));
                let options = QueryOptions::default()
                    .with_query_plan_mode(QueryPlanMode::GatewayOnly)
                    .with_operation_options(operation)
                    .with_max_item_count(MaxItemCountHint::Limit(
                        std::num::NonZeroU32::new(5).unwrap(),
                    ));
                let (ranked, page_count) = ranked_indices(
                    &container,
                    Query::from(text_query).with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(ranked, expected, "text ranking with binary={binary}");
                assert!(page_count > 1, "page-size hint should paginate 13 results");

                let (all_indices, page_count) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT TOP 100 c.index FROM c \
                        ORDER BY RANK FullTextScore(c.text, @term)",
                    )
                    .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(all_indices.len(), 100);
                assert!(page_count > 1);
                let mut all_members = all_indices;
                all_members.sort_unstable();
                assert_eq!(all_members, (1..=100).collect::<Vec<_>>());

                for scope in [FullTextScoreScope::Local, FullTextScoreScope::Global] {
                    let (ranked, _) = ranked_indices(
                        &container,
                        Query::from(text_query).with_parameter("@term", "United States")?,
                        FeedScope::partition("0"),
                        options.clone().with_full_text_score_scope(scope),
                    )
                    .await?;
                    assert_eq!(ranked, [49, 57], "scoped text ranking with {scope:?}");
                }

                let rrf_query = "SELECT c.index FROM c WHERE FullTextContains(c.text, @term) \
                    ORDER BY RANK RRF(FullTextScore(c.title, @name), \
                    FullTextScore(c.text, @term), [2, 1]) OFFSET 0 LIMIT 13";
                let (fused, _) = ranked_indices(
                    &container,
                    Query::from(rrf_query)
                        .with_parameter("@name", "John")?
                        .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(fused.len(), 13, "RRF must retain all matching items");
                let mut members = fused.clone();
                members.sort_unstable();
                let mut expected_members = expected.to_vec();
                expected_members.sort_unstable();
                assert_eq!(members, expected_members);

                let (rescaled, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT c.index FROM c WHERE FullTextContains(c.text, @term) \
                        ORDER BY RANK RRF(FullTextScore(c.title, @name), \
                        FullTextScore(c.text, @term), [4, 2]) OFFSET 0 LIMIT 13",
                    )
                    .with_parameter("@name", "John")?
                    .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(
                    rescaled, fused,
                    "uniform weight scaling must preserve RRF rank"
                );

                let (parameterized, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT c.index FROM c WHERE FullTextContains(c.text, @term) \
                        ORDER BY RANK RRF(FullTextScore(c.title, @name), \
                        FullTextScore(c.text, @term), @weights) OFFSET 0 LIMIT 13",
                    )
                    .with_parameter("@name", "John")?
                    .with_parameter("@term", "United States")?
                    .with_parameter("@weights", [2, 1])?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(parameterized, fused, "parameterized RRF weights");

                let (text_weighted, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT c.index FROM c WHERE FullTextContains(c.text, @term) \
                        ORDER BY RANK RRF(FullTextScore(c.title, @name), \
                        FullTextScore(c.text, @term), [1, 2]) OFFSET 0 LIMIT 13",
                    )
                    .with_parameter("@name", "John")?
                    .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_ne!(
                    text_weighted, fused,
                    "changing component weights must change the RRF ranking"
                );

                let (window, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT c.index FROM c WHERE FullTextContains(c.text, @term) \
                        ORDER BY RANK RRF(FullTextScore(c.title, @name), \
                        FullTextScore(c.text, @term), [2, 1]) OFFSET 5 LIMIT 5",
                    )
                    .with_parameter("@name", "John")?
                    .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(
                    window,
                    fused[5..10],
                    "RRF offset window with binary={binary}"
                );

                let vector = items[50]["vector"].as_array().unwrap();
                let (vector_only, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT TOP 10 c.index FROM c \
                        ORDER BY RANK VectorDistance(c.vector, @vector)",
                    )
                    .with_parameter("@vector", vector)?,
                    FeedScope::full_container(),
                    options.clone(),
                )
                .await?;
                assert_eq!(vector_only.first(), Some(&51));
                let (hybrid, _) = ranked_indices(
                    &container,
                    Query::from(
                        "SELECT TOP 10 c.index FROM c ORDER BY RANK \
                        RRF(VectorDistance(c.vector, @vector), \
                        FullTextScore(c.text, @term), [2, 1])",
                    )
                    .with_parameter("@vector", vector)?
                    .with_parameter("@term", "United States")?,
                    FeedScope::full_container(),
                    options,
                )
                .await?;
                assert_eq!(hybrid.len(), 10);
                assert!(
                    hybrid.contains(&51),
                    "exact vector match should rank in top 10"
                );
            }
            Ok(())
        },
        Some(TestOptions::default().with_timeout(std::time::Duration::from_secs(600))),
    )
    .await
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
                create_search_item(&container, partition, id, &document).await?;
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
                    let (items, _) = collect_ranked_items::<SearchDocument>(
                        &container,
                        query,
                        scope,
                        options.clone().with_full_text_score_scope(scope_option),
                    )
                    .await?;
                    let ids: Vec<_> = items.into_iter().map(|item| item.id).collect();
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
                let (values, _) = collect_ranked_items::<serde_json::Value>(
                    &container,
                    query,
                    FeedScope::full_container(),
                    QueryOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly),
                )
                .await?;
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
