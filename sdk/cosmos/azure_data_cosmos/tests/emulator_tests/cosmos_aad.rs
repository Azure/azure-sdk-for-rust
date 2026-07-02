// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore enableaadauthentication

//! Integration tests exercising Entra ID (AAD) authentication against Azure
//! Cosmos DB.
//!
//! These tests use a **dual-client** pattern: a key-auth client (provided by the
//! framework) performs database/container management, while a separate
//! AAD-authenticated client performs all data-plane item operations. This
//! mirrors the data-plane RBAC role provisioned in `test-resources.bicep`, which
//! grants item/metadata data actions but **not** management-plane permissions.
//!
//! Because the standard `test_category="emulator"` gate is used, the same tests
//! run against the local emulator (Build stage, with the emulator started using
//! `/enableaadauthentication`) and against live accounts (LiveTest stage), where
//! the framework selects a real Entra ID credential via
//! `azure_core_test::credentials::from_env`.

use super::framework;

use azure_core::http::StatusCode;
use azure_core::Uuid;
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::{PartitionKey, Query};
use framework::{TestClient, TestRunContext};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// The scope the Cosmos driver requests when acquiring an AAD token.
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
struct AadTestItem {
    id: String,
    partition_key: String,
    value: i64,
}

/// Drives a full item CRUD round-trip through an AAD-authenticated client.
///
/// Setup (database + container) and teardown run through the framework's
/// key-auth client; only the item operations use the AAD client. On the
/// emulator we additionally assert the bespoke fake-JWT credential was actually
/// invoked for the Cosmos scope, guarding against silently exercising key auth.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn aad_item_crud_roundtrip() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context: &TestRunContext, db_client| {
            // Key client creates the container (management-plane operation).
            let container_id = format!("aad-container-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;

            // Build the AAD-authenticated client and address the same container.
            let (aad_client, recorder) = run_context.aad_client().await?;
            let aad_container = aad_client
                .database_client(db_client.id())
                .container_client(&container_id)
                .await?;

            let unique = Uuid::new_v4().to_string();
            let pk = format!("pk-{unique}");
            let item_id = format!("item-{unique}");
            let mut item = AadTestItem {
                id: item_id.clone(),
                partition_key: pk.clone(),
                value: 1,
            };

            // Create via AAD.
            let create = aad_container
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_eq!(
                create.status(),
                StatusCode::Created,
                "AAD create_item should succeed"
            );

            // Point read via AAD.
            let read = aad_container.read_item(&pk, &item_id, None).await?;
            assert_eq!(read.status(), StatusCode::Ok);
            assert_eq!(
                read.into_model::<AadTestItem>()?,
                item,
                "round-tripped item should match"
            );

            // Replace via AAD.
            item.value = 2;
            let replace = aad_container
                .replace_item(&pk, &item_id, &item, None)
                .await?;
            assert_eq!(replace.status(), StatusCode::Ok);

            // Query via AAD, scoped to the item's partition.
            let query =
                Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", &item_id)?;
            let found: Vec<AadTestItem> = aad_container
                .query_items::<AadTestItem>(
                    query,
                    FeedScope::partition(PartitionKey::from(&pk)),
                    None,
                )
                .await?
                .try_collect()
                .await?;
            assert_eq!(found.len(), 1, "query should return exactly the one item");
            assert_eq!(found[0], item);

            // Delete via AAD.
            let delete = aad_container.delete_item(&pk, &item_id, None).await?;
            assert_eq!(delete.status(), StatusCode::NoContent);

            // On the emulator, prove the AAD credential was actually exercised.
            if let Some(recorder) = recorder {
                assert!(
                    recorder.call_count() > 0,
                    "emulator AAD credential get_token was never called"
                );
                assert!(
                    recorder.requested_scope(COSMOS_AAD_SCOPE),
                    "expected the Cosmos scope ({COSMOS_AAD_SCOPE}) to be requested, got {:?}",
                    recorder.requested_scopes()
                );
            }

            Ok(())
        },
        None,
    )
    .await
}

/// Verifies an AAD-authenticated client can read container metadata, exercising
/// the `readMetadata` data action the SDK requires on its first request.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn aad_read_container_metadata() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context: &TestRunContext, db_client| {
            let container_id = format!("aad-meta-{}", Uuid::new_v4());
            run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    None,
                )
                .await?;

            let (aad_client, recorder) = run_context.aad_client().await?;
            let aad_container = aad_client
                .database_client(db_client.id())
                .container_client(&container_id)
                .await?;

            let properties = aad_container.read(None).await?.into_model()?;
            assert_eq!(
                properties.id, container_id,
                "AAD container read should return the same container"
            );

            if let Some(recorder) = recorder {
                assert!(
                    recorder.requested_scope(COSMOS_AAD_SCOPE),
                    "expected the Cosmos scope to be requested via AAD"
                );
            }

            Ok(())
        },
        None,
    )
    .await
}
