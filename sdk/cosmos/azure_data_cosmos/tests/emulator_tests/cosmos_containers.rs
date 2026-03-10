// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::{
    models::{
        ContainerProperties, IndexingMode, IndexingPolicy, PartitionKeyKind, PropertyPath,
        ThroughputProperties,
    },
    CreateContainerOptions, Query,
};
use futures::TryStreamExt;

use framework::TestClient;

#[tokio::test]
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
            assert_eq!(
                vec![String::from("/id")],
                created_properties.partition_key.paths
            );
            assert_eq!(
                PartitionKeyKind::new(PartitionKeyKind::HASH),
                created_properties.partition_key.kind
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

            let mut query_pager = db_client.query_containers(
                Query::from("SELECT * FROM root r WHERE r.id = @id")
                    .with_parameter("@id", &properties.id)?,
                None,
            )?;
            let mut ids = vec![];
            while let Some(db) = query_pager.try_next().await? {
                ids.push(db.id);
            }
            assert_eq!(vec![properties.id.clone()], ids);

            let container_client = db_client.container_client(&properties.id).await;
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

            let current_throughput = container_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(400), current_throughput.throughput());

            let new_throughput = ThroughputProperties::manual(500);
            let throughput_response = container_client
                .replace_throughput(new_throughput, None)
                .await?
                .into_model()?;
            assert_eq!(Some(500), throughput_response.throughput());

            container_client.delete(None).await?;

            query_pager = db_client.query_containers(
                Query::from("SELECT * FROM root r WHERE r.id = @id")
                    .with_parameter("@id", &properties.id)?,
                None,
            )?;
            let mut ids = vec![];
            while let Some(db) = query_pager.try_next().await? {
                ids.push(db.id);
            }
            assert!(ids.is_empty());

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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
            assert_eq!(
                vec![
                    String::from("/parent"),
                    String::from("/child"),
                    String::from("/grandchild")
                ],
                created_properties.partition_key.paths
            );
            assert_eq!(
                PartitionKeyKind::new(PartitionKeyKind::MULTI_HASH),
                created_properties.partition_key.kind
            );

            Ok(())
        },
        None,
    )
    .await
}
