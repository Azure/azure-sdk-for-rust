// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator_tests/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{
    models::PartitionKeyKind,
    models::{
        ContainerProperties, FullTextPath, FullTextPolicy, IndexingMode, IndexingPolicy,
        PropertyPath, QuantizerType, ThroughputProperties, VectorDataType, VectorDistanceFunction,
        VectorEmbedding, VectorEmbeddingPolicy, VectorIndex, VectorIndexType,
    },
    Query,
};
use futures::TryStreamExt;

use framework::{TestClient, TestOptions};

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
#[cfg_attr(
    test_category = "emulator_inmemory",
    ignore = "hosted in-memory emulator does not yet support replacing container properties"
)]
pub async fn container_crud_simple() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create the container
            let properties = ContainerProperties::new("TheContainer", "/id".into())
                .with_indexing_policy(
                    IndexingPolicy::default()
                        .with_included_path("/*")
                        .with_excluded_path(r#"/"_etag"/?"#)
                        .with_indexing_mode(IndexingMode::Consistent),
                );

            let throughput = ThroughputProperties::manual(400);

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Read the container to get its properties
            let created_properties = container_client.read(None).await?.into_model()?;

            assert_eq!(&properties.id, &created_properties.id);
            assert_eq!(1, created_properties.partition_key.paths().len());
            assert_eq!("/id", created_properties.partition_key.paths()[0].as_ref());
            assert_eq!(
                PartitionKeyKind::Hash,
                created_properties.partition_key.kind()
            );
            let indexing_policy = created_properties
                .indexing_policy
                .expect("created container should have an indexing policy");
            assert_eq!(
                vec![PropertyPath::from("/*")],
                indexing_policy.included_paths
            );
            assert_eq!(
                vec![PropertyPath::from(r#"/"_etag"/?"#)],
                indexing_policy.excluded_paths
            );
            assert!(indexing_policy.automatic);
            assert_eq!(
                IndexingMode::Consistent,
                indexing_policy.indexing_mode.unwrap()
            );

            let mut query_pager = db_client
                .query_containers(
                    Query::from("SELECT * FROM root r WHERE r.id = @id")
                        .with_parameter("@id", &properties.id)?,
                    None,
                )
                .await?;
            let mut ids = vec![];
            while let Some(db) = query_pager.try_next().await? {
                ids.push(db.id);
            }
            assert_eq!(vec![properties.id.clone()], ids);

            let container_client = db_client
                .container_client(properties.id.as_ref(), None)
                .await?;
            let mut updated_indexing_policy = IndexingPolicy::default();
            updated_indexing_policy.automatic = false;
            updated_indexing_policy.indexing_mode = Some(IndexingMode::None);
            let updated_properties =
                ContainerProperties::new(properties.id.clone(), properties.partition_key.clone())
                    .with_indexing_policy(updated_indexing_policy);
            let update_response = container_client
                .replace(updated_properties, None)
                .await?
                .into_model()?;
            let updated_indexing_policy = update_response.indexing_policy.unwrap();
            assert!(updated_indexing_policy.included_paths.is_empty());
            assert!(updated_indexing_policy.excluded_paths.is_empty());
            assert!(!updated_indexing_policy.automatic);
            assert_eq!(
                Some(IndexingMode::None),
                updated_indexing_policy.indexing_mode
            );

            let current_throughput = run_context
                .management_container_client(db_client, "TheContainer")
                .await?
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(400), current_throughput.throughput());

            let new_throughput = ThroughputProperties::manual(500);
            let throughput_response = run_context
                .management_container_client(db_client, "TheContainer")
                .await?
                .begin_replace_throughput(new_throughput, None)
                .await?
                .await?
                .into_model()?;
            assert_eq!(Some(500), throughput_response.throughput());

            container_client.delete(None).await?;

            query_pager = db_client
                .query_containers(
                    Query::from("SELECT * FROM root r WHERE r.id = @id")
                        .with_parameter("@id", &properties.id)?,
                    None,
                )
                .await?;
            let mut ids = vec![];
            while let Some(db) = query_pager.try_next().await? {
                ids.push(db.id);
            }
            assert!(ids.is_empty());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn container_crud_hierarchical_pk() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create the container
            let properties = ContainerProperties::new(
                "TheContainer",
                ("/parent", "/child", "/grandchild").into(),
            )
            .with_indexing_policy(
                IndexingPolicy::default()
                    .with_included_path("/*")
                    .with_excluded_path(r#"/"_etag"/?"#)
                    .with_indexing_mode(IndexingMode::Consistent),
            );

            let container_client = run_context
                .create_container(db_client, properties.clone(), None)
                .await?;

            // Read the container to get its properties
            let created_properties = container_client.read(None).await?.into_model()?;

            assert_eq!(&properties.id, &created_properties.id);
            let paths: Vec<&str> = created_properties
                .partition_key
                .paths()
                .iter()
                .map(|p| p.as_ref())
                .collect();
            assert_eq!(vec!["/parent", "/child", "/grandchild"], paths);
            assert_eq!(
                PartitionKeyKind::MultiHash,
                created_properties.partition_key.kind()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Vector search and full text search are account-level capabilities
/// (`EnableNoSQLVectorSearch` / `EnableNoSQLFullTextSearch`, enabled for the live
/// test accounts by `sdk/cosmos/test-resources.bicep`). None of the local
/// emulators offer them, so a container carrying these policies can only be
/// created against a live account.
///
/// Returns `true` (and logs) when the current target is any local emulator, so
/// the test can skip cleanly there while still running in the live legs.
fn skip_vector_and_full_text_on_emulator() -> bool {
    let is_emulator = framework::targets_emulator();
    if is_emulator {
        eprintln!(
            "skipping vector/full-text container policy test: requires the \
             EnableNoSQLVectorSearch and EnableNoSQLFullTextSearch account \
             capabilities, which the emulators do not support. Runs against live \
             accounts; serialization is covered by unit tests in \
             src/models/container_properties.rs and src/models/indexing_policy.rs."
        );
    }
    is_emulator
}

/// Creates a container carrying a vector embedding policy, a full text policy,
/// vector indexes (including tuning options) and full text indexes, then
/// verifies they survive both a read and a read-modify-replace round trip.
///
/// The replace half is the regression guard for the round-trip data loss this
/// SDK used to have: `replace` serializes [`ContainerProperties`] verbatim, so
/// any policy the model failed to capture on read was silently stripped from the
/// container.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn container_vector_and_full_text_policies_round_trip() -> Result<(), Box<dyn Error>> {
    if skip_vector_and_full_text_on_emulator() {
        return Ok(());
    }

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("VectorAndFullText", "/id".into())
                .with_vector_embedding_policy(
                    VectorEmbeddingPolicy::default()
                        .with_embedding(VectorEmbedding::new(
                            "/flatVector",
                            VectorDataType::Float32,
                            8,
                            VectorDistanceFunction::Cosine,
                        ))
                        .with_embedding(VectorEmbedding::new(
                            "/quantizedVector",
                            VectorDataType::Float32,
                            16,
                            VectorDistanceFunction::DotProduct,
                        ))
                        .with_embedding(VectorEmbedding::new(
                            "/diskAnnVector",
                            VectorDataType::Float32,
                            32,
                            VectorDistanceFunction::Cosine,
                        )),
                )
                .with_full_text_policy(
                    FullTextPolicy::new("en-US")
                        .with_full_text_path(FullTextPath::new("/title").with_language("en-US"))
                        .with_full_text_path("/body"),
                )
                .with_indexing_policy(
                    IndexingPolicy::default()
                        .with_included_path("/*")
                        // Vector paths should not be covered by the regular index.
                        .with_excluded_path("/flatVector/*")
                        .with_excluded_path("/quantizedVector/*")
                        .with_excluded_path("/diskAnnVector/*")
                        .with_indexing_mode(IndexingMode::Consistent)
                        .with_vector_index(VectorIndex::new("/flatVector", VectorIndexType::Flat))
                        .with_vector_index(
                            VectorIndex::new("/quantizedVector", VectorIndexType::QuantizedFlat)
                                .with_quantizer_type(QuantizerType::Product)
                                .with_quantization_byte_size(4),
                        )
                        .with_vector_index(
                            VectorIndex::new("/diskAnnVector", VectorIndexType::DiskANN)
                                .with_quantizer_type(QuantizerType::Product)
                                .with_quantization_byte_size(8)
                                .with_indexing_search_list_size(50)
                                .with_shard_key_path("/country/city"),
                        )
                        .with_full_text_index("/title")
                        .with_full_text_index("/body"),
                );

            let container_client = run_context
                .create_container(db_client, properties.clone(), None)
                .await?;

            let created = container_client.read(None).await?.into_model()?;

            assert_vector_and_full_text_policies(&created, "after create");

            // Read-modify-replace: send the properties we just read straight back.
            // Anything the model dropped on read would be permanently lost here.
            let replaced = container_client
                .replace(created.clone(), None)
                .await?
                .into_model()?;

            assert_vector_and_full_text_policies(&replaced, "after replace");

            // And confirm the service actually persisted it, rather than just
            // echoing our request body back.
            let reread = container_client.read(None).await?.into_model()?;
            assert_vector_and_full_text_policies(&reread, "after re-read");

            container_client.delete(None).await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Shared assertions for [`container_vector_and_full_text_policies_round_trip`].
///
/// `stage` identifies which round trip is being checked so a failure points at
/// the operation that lost the data.
fn assert_vector_and_full_text_policies(properties: &ContainerProperties, stage: &str) {
    let vector_policy = properties
        .vector_embedding_policy
        .as_ref()
        .unwrap_or_else(|| panic!("{stage}: vector embedding policy should be present"));
    let mut embedding_paths: Vec<&str> = vector_policy
        .embeddings
        .iter()
        .map(|e| e.path.as_str())
        .collect();
    embedding_paths.sort_unstable();
    assert_eq!(
        vec!["/diskAnnVector", "/flatVector", "/quantizedVector"],
        embedding_paths,
        "{stage}: vector embedding paths"
    );
    let flat_embedding = vector_policy
        .embeddings
        .iter()
        .find(|embedding| embedding.path == "/flatVector")
        .unwrap_or_else(|| panic!("{stage}: flat vector embedding should be present"));
    assert_eq!(
        8, flat_embedding.dimensions,
        "{stage}: flat vector dimensions"
    );
    let quantized_embedding = vector_policy
        .embeddings
        .iter()
        .find(|embedding| embedding.path == "/quantizedVector")
        .unwrap_or_else(|| panic!("{stage}: quantized vector embedding should be present"));
    assert_eq!(
        VectorDistanceFunction::DotProduct,
        quantized_embedding.distance_function,
        "{stage}: quantized vector distance function"
    );

    let full_text_policy = properties
        .full_text_policy
        .as_ref()
        .unwrap_or_else(|| panic!("{stage}: full text policy should be present"));
    assert_eq!(
        Some("en-US"),
        full_text_policy.default_language.as_deref(),
        "{stage}: full text default language"
    );
    let mut full_text_paths: Vec<&str> = full_text_policy
        .full_text_paths
        .iter()
        .map(|p| p.path.as_str())
        .collect();
    full_text_paths.sort_unstable();
    assert_eq!(
        vec!["/body", "/title"],
        full_text_paths,
        "{stage}: full text paths"
    );
    let title_path = full_text_policy
        .full_text_paths
        .iter()
        .find(|path| path.path == "/title")
        .unwrap_or_else(|| panic!("{stage}: title full text path should be present"));
    assert_eq!(
        Some("en-US"),
        title_path.language.as_deref(),
        "{stage}: title full text language"
    );
    let body_path = full_text_policy
        .full_text_paths
        .iter()
        .find(|path| path.path == "/body")
        .unwrap_or_else(|| panic!("{stage}: body full text path should be present"));
    assert_eq!(
        None, body_path.language,
        "{stage}: body should inherit the default full text language"
    );

    let indexing_policy = properties
        .indexing_policy
        .as_ref()
        .unwrap_or_else(|| panic!("{stage}: indexing policy should be present"));
    let mut vector_index_paths: Vec<&str> = indexing_policy
        .vector_indexes
        .iter()
        .map(|i| i.path.as_str())
        .collect();
    vector_index_paths.sort_unstable();
    assert_eq!(
        vec!["/diskAnnVector", "/flatVector", "/quantizedVector"],
        vector_index_paths,
        "{stage}: vector index paths"
    );
    let flat_index = indexing_policy
        .vector_indexes
        .iter()
        .find(|index| index.path == "/flatVector")
        .unwrap_or_else(|| panic!("{stage}: flat vector index should be present"));
    assert_eq!(
        VectorIndexType::Flat,
        flat_index.index_type,
        "{stage}: flat vector index type"
    );
    let quantized_index = indexing_policy
        .vector_indexes
        .iter()
        .find(|index| index.path == "/quantizedVector")
        .unwrap_or_else(|| panic!("{stage}: quantized vector index should be present"));
    assert_eq!(
        VectorIndexType::QuantizedFlat,
        quantized_index.index_type,
        "{stage}: quantized vector index type"
    );
    assert_eq!(
        Some(QuantizerType::Product),
        quantized_index.quantizer_type,
        "{stage}: quantized vector quantizer type"
    );
    assert_eq!(
        Some(4),
        quantized_index.quantization_byte_size,
        "{stage}: quantized vector byte size"
    );
    let disk_ann_index = indexing_policy
        .vector_indexes
        .iter()
        .find(|index| index.path == "/diskAnnVector")
        .unwrap_or_else(|| panic!("{stage}: DiskANN vector index should be present"));
    assert_eq!(
        VectorIndexType::DiskANN,
        disk_ann_index.index_type,
        "{stage}: DiskANN vector index type"
    );
    assert_eq!(
        Some(QuantizerType::Product),
        disk_ann_index.quantizer_type,
        "{stage}: DiskANN quantizer type"
    );
    assert_eq!(
        Some(8),
        disk_ann_index.quantization_byte_size,
        "{stage}: DiskANN quantization byte size"
    );
    assert_eq!(
        Some(50),
        disk_ann_index.indexing_search_list_size,
        "{stage}: DiskANN indexing search list size"
    );
    assert_eq!(
        vec!["/country/city"],
        disk_ann_index.vector_index_shard_key,
        "{stage}: DiskANN shard key"
    );

    let mut full_text_index_paths: Vec<&str> = indexing_policy
        .full_text_indexes
        .iter()
        .map(|i| i.path.as_str())
        .collect();
    full_text_index_paths.sort_unstable();
    assert_eq!(
        vec!["/body", "/title"],
        full_text_index_paths,
        "{stage}: full text index paths"
    );
}
