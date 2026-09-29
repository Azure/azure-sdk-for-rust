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
//! Because these tests need AAD data-plane access, they gate on the
//! `cosmos_aad_supported` cfg (set by the local emulator setup when started
//! with `/enableaadauthentication`, and by bicep-provisioned live accounts
//! that include the Cosmos data-plane role assignment). Fixed self-owned live
//! accounts without that role assignment do not set this cfg, so these tests
//! are skipped on those legs.

use super::framework;

use azure_core::http::StatusCode;
use azure_core::Uuid;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::{CosmosError, CosmosStatus, PartitionKey, Query};
use framework::{
    probe_data_plane_ready, read_item_with_readiness_retry, TestClient, TestRunContext,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::{error::Error, future::Future, time::Duration};

/// The scope the Cosmos driver requests when acquiring an AAD token.
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";
const QUERY_READINESS_TIMEOUT: Duration = Duration::from_secs(30);
const QUERY_READINESS_RETRY_DELAY: Duration = Duration::from_secs(1);

async fn retry_query_readiness<F, Fut>(mut probe: F) -> azure_data_cosmos::Result<()>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = azure_data_cosmos::Result<()>>,
{
    let deadline = tokio::time::Instant::now() + QUERY_READINESS_TIMEOUT;
    let mut last_error = None;
    loop {
        if tokio::time::Instant::now() >= deadline {
            break;
        }
        match tokio::time::timeout_at(deadline, probe()).await {
            Ok(Ok(())) => return Ok(()),
            Ok(Err(error)) if framework::test_client::rbac_name_based_data_not_ready(&error) => {
                println!("waiting for AAD query authorization: {error}");
                last_error = Some(error);
            }
            Ok(Err(error)) => return Err(error),
            Err(_) => break,
        }
        tokio::time::sleep_until(
            (tokio::time::Instant::now() + QUERY_READINESS_RETRY_DELAY).min(deadline),
        )
        .await;
    }
    Err(last_error.unwrap_or_else(|| {
        CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::RequestTimeout))
            .with_message("AAD query readiness probe timed out after 30 seconds")
            .build()
    }))
}

async fn probe_query_ready(
    container: &ContainerClient,
    partition_key: &str,
) -> azure_data_cosmos::Result<()> {
    let query = Query::from("SELECT * FROM c WHERE c.id = @id")
        .with_parameter("@id", &Uuid::new_v4().to_string())?;
    retry_query_readiness(|| async {
        let mut pages = container
            .query_items::<serde_json::Value>(
                query.clone(),
                FeedScope::partition(PartitionKey::from(partition_key.to_owned())),
                None,
            )
            .await?
            .into_pages();
        while let Some(page) = pages.try_next().await? {
            assert!(
                page.into_items().is_empty(),
                "readiness probe returned an item"
            );
        }
        Ok(())
    })
    .await
}

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
    any(not(cosmos_aad_supported), test_category = "emulator_inmemory"),
    ignore = "requires an AAD-enabled Cosmos target (emulator with /enableaadauthentication, or a live account with the Cosmos data-plane role assignment); hosted in-memory emulator authentication is deferred to a follow-up PR"
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
                .container_client(&container_id, None)
                .await?;

            // Metadata (5301) and name-based data (5302) authorize through
            // separate RBAC paths. `run_context.create_container` above only
            // warms the framework's key-auth client; this test's own
            // freshly-built AAD client has never issued a data-plane
            // request against this container, so its first item operation
            // can still race and return
            // `403/5302 RbacUnauthorizedNameBasedDataRequest`. Probe this
            // client's data path before exercising real assertions.
            probe_data_plane_ready("aad client", &aad_container).await?;

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
            let read = read_item_with_readiness_retry(&aad_container, &pk, &item_id, None).await?;
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
            // Item readiness does not establish the separate executeQuery permission.
            probe_query_ready(&aad_container, &pk).await?;
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
    any(not(cosmos_aad_supported), test_category = "emulator_inmemory"),
    ignore = "requires an AAD-enabled Cosmos target (emulator with /enableaadauthentication, or a live account with the Cosmos data-plane role assignment); hosted in-memory emulator authentication is deferred to a follow-up PR"
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
                .container_client(&container_id, None)
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

#[cfg(test)]
mod tests {
    use super::{retry_query_readiness, QUERY_READINESS_TIMEOUT};
    use azure_core::http::StatusCode;
    use azure_data_cosmos::{CosmosError, CosmosStatus};
    use std::{cell::Cell, future::pending, time::Duration};

    fn error(status: StatusCode, sub_status: u16) -> CosmosError {
        CosmosError::builder()
            .with_status(CosmosStatus::new(status).with_sub_status(sub_status))
            .with_message("query authorization failure")
            .build()
    }

    #[tokio::test(start_paused = true)]
    async fn query_readiness_retries_transient_authorization() {
        let attempts = Cell::new(0);
        retry_query_readiness(|| {
            attempts.set(attempts.get() + 1);
            async {
                if attempts.get() < 3 {
                    Err(error(StatusCode::Forbidden, 5302))
                } else {
                    Ok(())
                }
            }
        })
        .await
        .unwrap();
        assert_eq!(attempts.get(), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn query_readiness_preserves_persistent_authorization_failure() {
        let start = tokio::time::Instant::now();
        let attempts = Cell::new(0);
        let failure = retry_query_readiness(|| {
            attempts.set(attempts.get() + 1);
            async { Err(error(StatusCode::Forbidden, 5302)) }
        })
        .await
        .unwrap_err();
        assert_eq!(
            failure.status(),
            error(StatusCode::Forbidden, 5302).status()
        );
        assert_eq!(
            failure.to_string(),
            error(StatusCode::Forbidden, 5302).to_string()
        );
        assert_eq!(start.elapsed(), QUERY_READINESS_TIMEOUT);
        assert_eq!(attempts.get(), 30);
    }

    #[tokio::test(start_paused = true)]
    async fn query_readiness_does_not_retry_other_errors() {
        for (status, sub_status) in [
            (StatusCode::Forbidden, 5301),
            (StatusCode::Forbidden, 0),
            (StatusCode::Unauthorized, 0),
            (StatusCode::BadRequest, 0),
        ] {
            let start = tokio::time::Instant::now();
            let attempts = Cell::new(0);
            let failure = retry_query_readiness(|| {
                attempts.set(attempts.get() + 1);
                async { Err(error(status, sub_status)) }
            })
            .await
            .unwrap_err();
            assert_eq!(failure.status(), error(status, sub_status).status());
            assert_eq!(attempts.get(), 1);
            assert_eq!(start.elapsed(), Duration::ZERO);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn query_readiness_bounds_hanging_requests() {
        let start = tokio::time::Instant::now();
        let failure = retry_query_readiness(pending).await.unwrap_err();
        assert_eq!(failure.status().status_code(), StatusCode::RequestTimeout);
        assert_eq!(start.elapsed(), QUERY_READINESS_TIMEOUT);
    }
}
