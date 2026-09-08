// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live single- and cross-partition vector query coverage.
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
        ContainerProperties, CosmosStatus, IndexingMode, IndexingPolicy, ThroughputProperties,
        VectorDataType, VectorDistanceFunction, VectorEmbedding, VectorEmbeddingPolicy,
        VectorIndex, VectorIndexType,
    },
    options::{CreateContainerOptions, MaxItemCountHint, QueryOptions},
    Query,
};
use framework::{TestClient, TestOptions};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const SEARCH_PARTITION: &str = "tenant-a";
const OTHER_PARTITION: &str = "tenant-b";
const CROSS_PARTITION_THROUGHPUT: usize = 11_000;
const QUERY_VECTOR: [f32; 2] = [0.0, 0.0];
const PRECOMPUTED_VECTOR_DIMENSIONS: usize = 300;
const PRECOMPUTED_VECTOR_TOP: usize = 9;
const PRECOMPUTED_VECTOR_FIXTURE: &str = include_str!("data/precomputed_vector_query.json");
const PRECOMPUTED_EXPECTED_ORDER: [&str; PRECOMPUTED_VECTOR_TOP] =
    ["0", "8", "1", "3", "7", "5", "6", "2", "4"];
const PRECOMPUTED_EXPECTED_MATCHES: [(&str, &str); PRECOMPUTED_VECTOR_TOP] = [
    ("0", "sayVERB"),
    ("8", "know_VERB"),
    ("1", "go_VERB"),
    ("3", "get_VERB"),
    ("7", "take_VERB"),
    ("5", "see_VERB"),
    ("6", "time_NOUN"),
    ("2", "make_VERB"),
    ("4", "one_NUM"),
];
const PRECOMPUTED_FIXTURE_SOURCE: &str = "https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/tests/Microsoft.Azure.Cosmos.EmulatorTests/Query/NonStreamingOrderByQueryTests.cs";
const PRECOMPUTED_ADAPTATION_NOTE: &str = "The malformed upstream year_NOUN entry was intentionally omitted because its embedding contains 279 rather than 300 components; all included document embeddings are copied without modification.";

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrecomputedVectorFixture {
    provenance: PrecomputedVectorProvenance,
    query_vector: Vec<f32>,
    expected_order: Vec<String>,
    documents: Vec<PrecomputedVectorDocument>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrecomputedVectorProvenance {
    source: String,
    copyright: String,
    license: String,
    adaptation_note: String,
}

#[derive(Deserialize)]
struct PrecomputedVectorDocument {
    id: String,
    word: String,
    embedding: Vec<f32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PrecomputedVectorItem<'a> {
    id: &'a str,
    word: &'a str,
    partition_key: &'static str,
    euclidean_embedding: &'a [f32],
    cosine_embedding: &'a [f32],
    dot_product_embedding: &'a [f32],
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrecomputedVectorMatch {
    id: String,
    word: String,
    partition_key: String,
    score: f64,
}

#[derive(Clone, Copy)]
enum PrecomputedDistance {
    Euclidean,
    Cosine,
    DotProduct,
}

impl PrecomputedDistance {
    fn path(self) -> &'static str {
        match self {
            Self::Euclidean => "euclideanEmbedding",
            Self::Cosine => "cosineEmbedding",
            Self::DotProduct => "dotProductEmbedding",
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Euclidean => "Euclidean",
            Self::Cosine => "Cosine",
            Self::DotProduct => "DotProduct",
        }
    }

    fn policy(self) -> VectorDistanceFunction {
        match self {
            Self::Euclidean => VectorDistanceFunction::Euclidean,
            Self::Cosine => VectorDistanceFunction::Cosine,
            Self::DotProduct => VectorDistanceFunction::DotProduct,
        }
    }
}

fn precomputed_vector_fixture() -> PrecomputedVectorFixture {
    let fixture: PrecomputedVectorFixture =
        serde_json::from_str(PRECOMPUTED_VECTOR_FIXTURE).expect("fixture should be valid JSON");

    assert_eq!(fixture.provenance.source, PRECOMPUTED_FIXTURE_SOURCE);
    assert_eq!(
        fixture.provenance.copyright,
        "Copyright (c) Microsoft Corporation. All rights reserved."
    );
    assert_eq!(fixture.provenance.license, "MIT License");
    assert_eq!(
        fixture.provenance.adaptation_note,
        PRECOMPUTED_ADAPTATION_NOTE
    );
    assert_eq!(fixture.query_vector.len(), PRECOMPUTED_VECTOR_DIMENSIONS);
    assert!(fixture.query_vector.iter().all(|value| value.is_finite()));
    assert_eq!(
        fixture
            .expected_order
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        PRECOMPUTED_EXPECTED_ORDER
    );
    assert_eq!(fixture.documents.len(), PRECOMPUTED_VECTOR_TOP);

    let mut document_ids = HashSet::new();
    for document in &fixture.documents {
        assert!(
            document_ids.insert(document.id.as_str()),
            "fixture contains duplicate document {}",
            document.id
        );
        assert_eq!(
            document.embedding.len(),
            PRECOMPUTED_VECTOR_DIMENSIONS,
            "fixture document {} has the wrong vector dimensions",
            document.id
        );
        assert!(
            document.embedding.iter().all(|value| value.is_finite()),
            "fixture document {} contains a non-finite vector component",
            document.id
        );
    }
    assert_eq!(
        document_ids,
        PRECOMPUTED_EXPECTED_ORDER.into_iter().collect()
    );
    for (expected_id, expected_word) in PRECOMPUTED_EXPECTED_MATCHES {
        let document = fixture
            .documents
            .iter()
            .find(|document| document.id == expected_id)
            .unwrap_or_else(|| panic!("missing fixture document {expected_id}"));
        assert_eq!(document.word, expected_word);
    }

    fixture
}

fn vector_documents() -> [VectorDocument; 7] {
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
            embedding: [0.5, 0.0],
        },
        VectorDocument {
            id: "other-partition-near",
            partition_key: OTHER_PARTITION,
            active: true,
            embedding: [1.5, 0.0],
        },
    ]
}

async fn seed_vector_container(
    run_context: &framework::TestRunContext,
    db_client: &azure_data_cosmos::clients::DatabaseClient,
    throughput: Option<usize>,
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

    let options = throughput.map(|throughput| {
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(throughput))
    });
    let container = run_context
        .create_container(db_client, properties, options)
        .await?;
    for document in vector_documents() {
        container
            .create_item(document.partition_key, document.id, &document, None)
            .await?;
    }
    Ok(container)
}

async fn seed_precomputed_vector_container(
    run_context: &framework::TestRunContext,
    db_client: &azure_data_cosmos::clients::DatabaseClient,
    fixture: &PrecomputedVectorFixture,
) -> azure_data_cosmos::Result<ContainerClient> {
    let distances = [
        PrecomputedDistance::Euclidean,
        PrecomputedDistance::Cosine,
        PrecomputedDistance::DotProduct,
    ];
    let mut indexing_policy = IndexingPolicy::default()
        .with_indexing_mode(IndexingMode::Consistent)
        .with_included_path("/*");
    let mut embedding_policy = VectorEmbeddingPolicy::default();
    for distance in distances {
        let path = format!("/{}", distance.path());
        indexing_policy = indexing_policy
            .with_excluded_path(format!("{path}/*"))
            .with_vector_index(VectorIndex::new(path.clone(), VectorIndexType::Flat));
        embedding_policy = embedding_policy.with_embedding(VectorEmbedding::new(
            path,
            VectorDataType::Float32,
            PRECOMPUTED_VECTOR_DIMENSIONS as u32,
            distance.policy(),
        ));
    }
    indexing_policy.automatic = true;

    let properties =
        ContainerProperties::new("PrecomputedVectorQueryContainer", "/partitionKey".into())
            .with_vector_embedding_policy(embedding_policy)
            .with_indexing_policy(indexing_policy);
    let options = CreateContainerOptions::default()
        .with_throughput(ThroughputProperties::manual(CROSS_PARTITION_THROUGHPUT));
    let container = run_context
        .create_container(db_client, properties, Some(options))
        .await?;

    for (index, document) in fixture.documents.iter().enumerate() {
        let partition_key = if index % 2 == 0 {
            SEARCH_PARTITION
        } else {
            OTHER_PARTITION
        };
        let item = PrecomputedVectorItem {
            id: &document.id,
            word: &document.word,
            partition_key,
            euclidean_embedding: &document.embedding,
            cosine_embedding: &document.embedding,
            dot_product_embedding: &document.embedding,
        };
        container
            .create_item(partition_key, document.id.as_str(), &item, None)
            .await?;
    }

    Ok(container)
}

async fn assert_seeded_across_physical_partitions(
    container: &ContainerClient,
    expected_item_count: usize,
) -> azure_data_cosmos::Result<()> {
    let ranges = container.read_feed_ranges(None).await?;
    assert_eq!(
        ranges.len(),
        2,
        "expected exactly two physical partitions with {CROSS_PARTITION_THROUGHPUT} RU/s"
    );

    let search_ranges = container
        .feed_range_from_partition_key(SEARCH_PARTITION, None)
        .await?;
    let other_ranges = container
        .feed_range_from_partition_key(OTHER_PARTITION, None)
        .await?;
    assert_eq!(search_ranges.len(), 1);
    assert_eq!(other_ranges.len(), 1);
    assert_ne!(
        search_ranges[0], other_ranges[0],
        "test logical partition keys must map to different physical partitions"
    );

    let mut seen_ids = HashSet::new();
    for range in ranges {
        let mut pages = container
            .query_items::<String>(
                Query::from("SELECT VALUE c.id FROM c"),
                FeedScope::range(range),
                None,
            )
            .await?
            .into_pages();
        let mut range_item_count = 0;
        while let Some(page) = pages.next().await {
            for id in page?.into_items() {
                assert!(
                    seen_ids.insert(id.clone()),
                    "item {id} was returned by more than one physical partition"
                );
                range_item_count += 1;
            }
        }
        assert!(
            range_item_count > 0,
            "each physical partition must contain seeded vector documents"
        );
    }
    assert_eq!(
        seen_ids.len(),
        expected_item_count,
        "physical-partition queries must return every seeded vector document"
    );
    Ok(())
}

fn precomputed_vector_query(
    distance: PrecomputedDistance,
    is_brute_force: bool,
    query_vector: &[f32],
) -> azure_data_cosmos::Result<Query> {
    let path = distance.path();
    let distance_function = distance.name();
    Query::from(format!(
        "SELECT TOP {PRECOMPUTED_VECTOR_TOP} c.id, c.word, c.partitionKey, \
         VectorDistance(c.{path}, @queryVector, {is_brute_force}, \
         {{distanceFunction:'{distance_function}'}}) AS score \
         FROM c ORDER BY VectorDistance(c.{path}, @queryVector, {is_brute_force}, \
         {{distanceFunction:'{distance_function}'}})"
    ))
    .with_parameter("@queryVector", query_vector)
}

fn assert_precomputed_vector_matches(matches: &[PrecomputedVectorMatch]) {
    let actual: Vec<(&str, &str)> = matches
        .iter()
        .map(|item| (item.id.as_str(), item.word.as_str()))
        .collect();
    assert_eq!(actual, PRECOMPUTED_EXPECTED_MATCHES);

    let ids: HashSet<&str> = matches.iter().map(|item| item.id.as_str()).collect();
    assert_eq!(ids.len(), PRECOMPUTED_VECTOR_TOP);
    assert_eq!(
        ids,
        PRECOMPUTED_EXPECTED_ORDER.into_iter().collect(),
        "vector query returned missing or unexpected documents"
    );
    let partitions: HashSet<&str> = matches
        .iter()
        .map(|item| item.partition_key.as_str())
        .collect();
    assert_eq!(
        partitions,
        [SEARCH_PARTITION, OTHER_PARTITION].into_iter().collect(),
        "vector query results must contain both logical partition keys"
    );
    assert!(
        matches.iter().all(|item| item.score.is_finite()),
        "vector query returned a non-finite score"
    );
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

fn cross_partition_vector_query(
    is_brute_force: bool,
    offset_limit: bool,
) -> azure_data_cosmos::Result<Query> {
    let brute_force = if is_brute_force { "true" } else { "false" };
    let window = if offset_limit { "OFFSET 1 LIMIT 3" } else { "" };
    let top = if offset_limit { "" } else { "TOP 5 " };
    Query::from(format!(
        "SELECT {top}c.id, VectorDistance(c.embedding, @queryVector, {brute_force}) AS score \
         FROM c WHERE c.active = true \
         ORDER BY VectorDistance(c.embedding, @queryVector, {brute_force}) {window}"
    ))
    .with_parameter("@queryVector", QUERY_VECTOR.as_slice())
}

fn assert_cross_partition_matches(matches: &[VectorMatch], offset_limit: bool) {
    let expected: &[(&str, f64)] = if offset_limit {
        &[
            ("other-partition-origin", 0.5),
            ("near", 1.0),
            ("other-partition-near", 1.5),
        ]
    } else {
        &[
            ("origin", 0.0),
            ("other-partition-origin", 0.5),
            ("near", 1.0),
            ("other-partition-near", 1.5),
            ("far", 2.0),
        ]
    };
    assert_eq!(matches.len(), expected.len());
    for (item, (expected_id, expected_score)) in matches.iter().zip(expected) {
        assert_eq!(&item.id, expected_id);
        assert!(
            (item.score - expected_score).abs() <= 1e-6,
            "unexpected score for {}: expected {expected_score}, got {}",
            item.id,
            item.score
        );
    }
    assert!(
        matches
            .iter()
            .any(|item| item.id.starts_with("other-partition-")),
        "cross-partition vector results must include the second logical partition"
    );
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
            let container = seed_vector_container(run_context, db_client, None).await?;
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

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn cross_partition_vector_search() -> Result<(), Box<dyn Error>> {
    if framework::targets_emulator() {
        eprintln!(
            "skipping vector query test: local Cosmos DB emulators do not support \
             EnableNoSQLVectorSearch"
        );
        return Ok(());
    }

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container =
                seed_vector_container(run_context, db_client, Some(CROSS_PARTITION_THROUGHPUT))
                    .await?;
            assert_seeded_across_physical_partitions(&container, vector_documents().len()).await?;
            for (is_brute_force, offset_limit) in [(false, false), (true, false), (false, true)] {
                let options = QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
                    NonZeroU32::new(2).expect("page size is non-zero"),
                ));
                let mut pages = container
                    .query_items::<VectorMatch>(
                        cross_partition_vector_query(is_brute_force, offset_limit)?,
                        FeedScope::full_container(),
                        Some(options),
                    )
                    .await?
                    .into_pages();
                let continuation_error = pages
                    .to_continuation_token()
                    .expect_err("buffered vector queries must not mint continuation tokens");
                assert_eq!(
                    continuation_error.status(),
                    CosmosStatus::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED
                );

                let mut matches = Vec::new();
                let mut seen_ids = HashSet::new();
                while let Some(page) = pages.next().await {
                    let items = page?.into_items();
                    assert!(
                        items.len() <= 2,
                        "buffered vector result page exceeded max_item_count"
                    );
                    for item in items {
                        assert!(
                            seen_ids.insert(item.id.clone()),
                            "vector query returned duplicate item {}",
                            item.id
                        );
                        matches.push(item);
                    }
                }
                assert_cross_partition_matches(&matches, offset_limit);
            }
            Ok(())
        },
        Some(TestOptions::default()),
    )
    .await
}

#[test]
fn precomputed_vector_fixture_has_expected_shape() {
    precomputed_vector_fixture();
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
pub async fn precomputed_pure_vector_search() -> Result<(), Box<dyn Error>> {
    if framework::targets_emulator() {
        eprintln!(
            "skipping vector query test: local Cosmos DB emulators do not support \
             EnableNoSQLVectorSearch"
        );
        return Ok(());
    }

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let fixture = precomputed_vector_fixture();
            let container =
                seed_precomputed_vector_container(run_context, db_client, &fixture).await?;
            assert_seeded_across_physical_partitions(&container, fixture.documents.len()).await?;

            for distance in [
                PrecomputedDistance::Euclidean,
                PrecomputedDistance::Cosine,
                PrecomputedDistance::DotProduct,
            ] {
                for is_brute_force in [false, true] {
                    let options = QueryOptions::default().with_max_item_count(
                        MaxItemCountHint::Limit(NonZeroU32::new(3).expect("page size is non-zero")),
                    );
                    let mut pages = container
                        .query_items::<PrecomputedVectorMatch>(
                            precomputed_vector_query(
                                distance,
                                is_brute_force,
                                &fixture.query_vector,
                            )?,
                            FeedScope::full_container(),
                            Some(options),
                        )
                        .await?
                        .into_pages();

                    let mut matches = Vec::new();
                    let mut seen_ids = HashSet::new();
                    let mut page_count = 0;
                    while let Some(page) = pages.next().await {
                        page_count += 1;
                        assert!(
                            page_count <= PRECOMPUTED_VECTOR_TOP,
                            "{} vector query emitted too many pages",
                            distance.name()
                        );
                        let items = page?.into_items();
                        assert!(
                            items.len() <= 3,
                            "{} vector result page exceeded max_item_count",
                            distance.name()
                        );
                        for item in items {
                            assert!(
                                seen_ids.insert(item.id.clone()),
                                "{} vector query returned duplicate item {}",
                                distance.name(),
                                item.id
                            );
                            matches.push(item);
                        }
                    }
                    assert_precomputed_vector_matches(&matches);
                }
            }

            Ok(())
        },
        Some(TestOptions::default()),
    )
    .await
}
