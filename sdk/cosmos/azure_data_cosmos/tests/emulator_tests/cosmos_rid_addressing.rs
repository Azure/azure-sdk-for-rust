// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::Uuid;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, ThroughputProperties},
    options::CreateContainerOptions,
    CosmosStatus, Query, ResourceId,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};

use framework::{TestClient, TestOptions};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct RidItem {
    id: String,
    pk: String,
    value: i32,
}

/// Collects every item produced by a query into a `Vec`, draining the pager.
async fn collect_items(
    container: &ContainerClient,
    query: Query,
    scope: FeedScope,
) -> Result<Vec<RidItem>, Box<dyn Error>> {
    let mut pager = container.query_items::<RidItem>(query, scope, None).await?;
    let mut items = Vec::new();
    while let Some(item) = pager.try_next().await? {
        items.push(item);
    }
    Ok(items)
}

/// Exercises the full RID-addressing flow end to end: create a database and
/// container by name, discover their service-assigned `_rid`s, then re-address
/// both purely by RID and confirm every read/write/query operation resolves to
/// the same resources.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: RID addressing not verified"
)]
pub async fn database_and_container_addressed_by_rid() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_name = format!("rid-container-{}", Uuid::new_v4());
            let name_container = run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_name.clone(), "/pk".into()),
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(400)),
                    ),
                )
                .await?;

            // Capture the service-assigned RIDs (`_rid`) for both the database
            // and the container — these are what callers would address by.
            let db_rid = db_client
                .read(None)
                .await?
                .into_model()?
                .system_properties
                .resource_id
                .expect("database read should return a _rid");
            let container_rid = name_container
                .read(None)
                .await?
                .into_model()?
                .system_properties
                .resource_id
                .expect("container read should return a _rid");

            // Re-address the same database purely by RID.
            let rid_db_client = run_context
                .client()
                .database_client(ResourceId::from(db_rid.clone()));
            assert_eq!(
                Some(db_rid.as_str()),
                rid_db_client.rid().map(ResourceId::as_str),
                "RID-addressed db client should expose the RID"
            );
            assert!(
                rid_db_client.name().is_none(),
                "RID-addressed db client should not expose a name"
            );

            // ...and the container by RID under that RID-addressed database.
            let rid_container = rid_db_client
                .container_client(ResourceId::from(container_rid.clone()))
                .await?;

            // Reading by RID resolves back to the same container.
            let read_back = rid_container.read(None).await?.into_model()?;
            assert_eq!(container_name, read_back.id);

            // Throughput is reachable by RID.
            let throughput = rid_container
                .read_throughput(None)
                .await?
                .expect("throughput should be present");
            assert_eq!(Some(400), throughput.throughput());

            // Create an item through the RID-addressed container.
            let item = RidItem {
                id: format!("item-{}", Uuid::new_v4()),
                pk: "pk-1".to_string(),
                value: 7,
            };
            rid_container
                .create_item(&item.pk, &item.id, &item, None)
                .await?;

            // Point-read it back by RID.
            let fetched: RidItem = rid_container
                .read_item(&item.pk, &item.id, None)
                .await?
                .into_model()?;
            assert_eq!(item, fetched);

            // Single-partition query against the RID-addressed container.
            let single = collect_items(
                &rid_container,
                Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", &item.id)?,
                FeedScope::partition(&item.pk),
            )
            .await?;
            assert_eq!(vec![item.clone()], single);

            // Cross-partition query against the RID-addressed container.
            let cross = collect_items(
                &rid_container,
                Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", &item.id)?,
                FeedScope::full_container(),
            )
            .await?;
            assert_eq!(vec![item.clone()], cross);

            // Listing containers under the RID-addressed database also works.
            let mut container_ids = Vec::new();
            let mut container_pager = rid_db_client
                .query_containers(
                    Query::from("SELECT * FROM root r WHERE r.id = @id")
                        .with_parameter("@id", &container_name)?,
                    None,
                )
                .await?;
            while let Some(c) = container_pager.try_next().await? {
                container_ids.push(c.id);
            }
            assert_eq!(vec![container_name.clone()], container_ids);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A database addressed by name and a container addressed by RID (or any other
/// mix) must be rejected before any network call: addressing modes cannot be
/// mixed.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: RID addressing not verified"
)]
pub async fn mixed_name_and_rid_addressing_is_rejected() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_name = format!("rid-mixed-{}", Uuid::new_v4());
            let name_container = run_context
                .create_container(
                    db_client,
                    ContainerProperties::new(container_name.clone(), "/pk".into()),
                    None,
                )
                .await?;
            let container_rid = name_container
                .read(None)
                .await?
                .into_model()?
                .system_properties
                .resource_id
                .expect("container read should return a _rid");

            // `db_client` is name-addressed; addressing the container by RID
            // mixes the two modes and must be rejected.
            let Err(err) = db_client
                .container_client(ResourceId::from(container_rid))
                .await
            else {
                panic!("expected mixed name/RID addressing to be rejected");
            };
            assert_eq!(CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING, err.status());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A container RID that belongs to a different database than the one addressed
/// must be rejected, so callers cannot accidentally reach into another database.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: RID addressing not verified"
)]
pub async fn container_rid_from_another_database_is_rejected() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // db1 is the unique database created by the harness.
            let db1_rid = db_client
                .read(None)
                .await?
                .into_model()?
                .system_properties
                .resource_id
                .expect("db1 read should return a _rid");

            // db2 + a container in db2, created out of band.
            let db2_name = format!("rid-otherdb-{}", Uuid::new_v4());
            let _ = run_context
                .client()
                .create_database(&db2_name, None)
                .await?;
            let db2_client = run_context.client().database_client(db2_name.as_str());
            let container2_name = format!("rid-otherc-{}", Uuid::new_v4());
            let container2 = run_context
                .create_container(
                    &db2_client,
                    ContainerProperties::new(container2_name.clone(), "/pk".into()),
                    None,
                )
                .await?;
            let container2_rid = container2
                .read(None)
                .await?
                .into_model()?
                .system_properties
                .resource_id
                .expect("container2 read should return a _rid");

            // Address db1 by RID but hand it a container RID that belongs to db2.
            let rid_db1_client = run_context
                .client()
                .database_client(ResourceId::from(db1_rid));
            let result = rid_db1_client
                .container_client(ResourceId::from(container2_rid))
                .await;

            // Clean up db2 regardless of the assertion outcome below.
            db2_client.delete(None).await?;

            let Err(err) = result else {
                panic!("expected a container RID from another database to be rejected");
            };
            assert_eq!(CosmosStatus::CLIENT_INVALID_RESOURCE_ID, err.status());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
