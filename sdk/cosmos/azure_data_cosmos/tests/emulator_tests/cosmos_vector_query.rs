// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live single-logical-partition vector query coverage.
//!
//! Vector search is unavailable on the classic, vnext, and in-memory emulators,
//! so this module skips those targets at runtime. Live accounts use the default
//! `emulator` test category and enable `EnableNoSQLVectorSearch` in
//! `sdk/cosmos/test-resources.bicep`.

use super::framework;

use std::{collections::HashSet, error::Error, num::NonZeroU32};

use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{
        ContainerProperties, IndexingMode, IndexingPolicy, VectorDataType, VectorDistanceFunction,
        VectorEmbedding, VectorEmbeddingPolicy, VectorIndex, VectorIndexType,
    },
    options::{MaxItemCountHint, QueryOptions},
    Query,
};
use framework::{TestClient, TestOptions};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const SEARCH_PARTITION: &str = "tenant-a";
const OTHER_PARTITION: &str = "tenant-b";
const QUERY_VECTOR: [f32; 2] = [0.0, 0.0];

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct VectorDocument {
    id: &'static str,
    partition_key: &'static str,
    active: bool,
    embedding: [f32; 2],
}

#[derive(Debug, Deserialize)]
struct VectorMatch {
    id: String,
    score: f64,
}

fn vector_documents() -> [VectorDocument; 6] {
    [
        VectorDocument {
            id: "origin",
            partition_key: SEARCH_PARTITION,
            active: true,
            embedding: [0.0, 0.0],
        },
        VectorDocument {
            id: "near",
            partition_key: SEARCH_PARTITION,
            active: true,
            embedding: [1.0, 0.0],
        },
        VectorDocument {
            id: "filtered",
            partition_key: SEARCH_PARTITION,
            active: false,
            embedding: [1.0, 1.0],
        },
        VectorDocument {
            id: "far",
            partition_key: SEARCH_PARTITION,
            active: true,
            embedding: [2.0, 0.0],
        },
        VectorDocument {
            id: "farthest",
            partition_key: SEARCH_PARTITION,
            active: true,
            embedding: [3.0, 0.0],
        },
        VectorDocument {
            id: "other-partition-origin",
            partition_key: OTHER_PARTITION,
            active: true,
            embedding: [0.0, 0.0],
        },
    ]
}

async fn seed_vector_container(
    run_context: &framework::TestRunContext,
    db_client: &azure_data_cosmos::clients::DatabaseClient,
) -> azure_data_cosmos::Result<ContainerClient> {
    let mut indexing_policy = IndexingPolicy::default()
        .with_indexing_mode(IndexingMode::Consistent)
        .with_included_path("/*")
        .with_excluded_path("/embedding/*")
        .with_vector_index(VectorIndex::new("/embedding", VectorIndexType::Flat));
    indexing_policy.automatic = true;

    let properties = ContainerProperties::new("VectorQueryContainer", "/partitionKey".into())
        .with_vector_embedding_policy(VectorEmbeddingPolicy::default().with_embedding(
            VectorEmbedding::new(
                "/embedding",
                VectorDataType::Float32,
                2,
                VectorDistanceFunction::Euclidean,
            ),
        ))
        .with_indexing_policy(indexing_policy);

    let container = run_context
        .create_container(db_client, properties, None)
        .await?;
    for document in vector_documents() {
        container
            .create_item(document.partition_key, document.id, &document, None)
            .await?;
    }
    Ok(container)
}

fn vector_query(is_brute_force: bool) -> azure_data_cosmos::Result<Query> {
    let text = if is_brute_force {
        "SELECT TOP 4 c.id, VectorDistance(c.embedding, @queryVector, true) AS score \
         FROM c WHERE c.active = true \
         ORDER BY VectorDistance(c.embedding, @queryVector, true)"
    } else {
        "SELECT TOP 4 c.id, VectorDistance(c.embedding, @queryVector, false) AS score \
         FROM c WHERE c.active = true \
         ORDER BY VectorDistance(c.embedding, @queryVector, false)"
    };
    Query::from(text).with_parameter("@queryVector", QUERY_VECTOR.as_slice())
}

fn assert_vector_matches(matches: &[VectorMatch]) {
    let ids: Vec<&str> = matches.iter().map(|item| item.id.as_str()).collect();
    assert_eq!(ids, ["origin", "near", "far", "farthest"]);

    let expected_scores = [0.0, 1.0, 2.0, 3.0];
    for (item, expected) in matches.iter().zip(expected_scores) {
        assert!(
            (item.score - expected).abs() <= 1e-6,
            "unexpected score for {}: expected {expected}, got {}",
            item.id,
            item.score
        );
    }
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn single_partition_vector_search() -> Result<(), Box<dyn Error>> {
    if framework::targets_emulator() {
        eprintln!(
            "skipping vector query test: local Cosmos DB emulators do not support \
             EnableNoSQLVectorSearch"
        );
        return Ok(());
    }

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_vector_container(run_context, db_client).await?;
            let options = QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
                NonZeroU32::new(1).expect("page size is non-zero"),
            ));
            let mut pages = container
                .query_items::<VectorMatch>(
                    vector_query(false)?,
                    FeedScope::partition(SEARCH_PARTITION),
                    Some(options),
                )
                .await?
                .into_pages();

            let mut indexed_matches = Vec::new();
            let mut seen_ids = HashSet::new();
            while let Some(page) = pages.next().await {
                for item in page?.into_items() {
                    assert!(
                        seen_ids.insert(item.id.clone()),
                        "vector query returned duplicate item {}",
                        item.id
                    );
                    indexed_matches.push(item);
                }
            }
            assert_vector_matches(&indexed_matches);

            let mut brute_force_pages = container
                .query_items::<VectorMatch>(
                    vector_query(true)?,
                    FeedScope::partition(SEARCH_PARTITION),
                    None,
                )
                .await?
                .into_pages();
            let mut brute_force_matches = Vec::new();
            while let Some(page) = brute_force_pages.next().await {
                brute_force_matches.extend(page?.into_items());
            }
            assert_vector_matches(&brute_force_matches);

            Ok(())
        },
        Some(TestOptions::default()),
    )
    .await
}
