// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::Uuid;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::FeedScope,
    models::{ContainerProperties, PatchInstructions, PatchOperation, ThroughputProperties},
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

/// Exercises the RID-addressing flow end to end: create a database and
/// container by name, discover their service-assigned `_rid`s, then re-address
/// both purely by RID and confirm every operation whose URI is RID-addressed
/// end to end resolves to the same resources.
///
/// Cosmos classifies a request as name-based or RID-based from the `dbs`
/// segment alone, so a RID-addressed path must be RID-addressed all the way
/// down. Operations whose URI stops at the container (container read,
/// throughput, create/upsert, queries, feed reads) therefore work by RID. Point
/// operations also work when the item itself is addressed by RID; an item name
/// under a RID-addressed parent remains invalid.
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

            // Throughput is reachable by RID. Reading an offer is a
            // control-plane operation that the data-plane RBAC role used in
            // AAD mode cannot perform, so route it through the management (key)
            // client — still addressed purely by RID. In key mode the
            // management client is the same as the primary client.
            let mgmt_rid_container = run_context
                .management_client()
                .database_client(ResourceId::from(db_rid.clone()))
                .container_client(ResourceId::from(container_rid.clone()))
                .await?;
            let throughput = mgmt_rid_container
                .read_throughput(None)
                .await?
                .expect("throughput should be present");
            assert_eq!(Some(400), throughput.throughput());

            // Create an item through the RID-addressed container. Create POSTs
            // to the collection URL, so the item id never reaches the wire and
            // the service never tries to parse it as a ResourceId.
            let mut item = RidItem {
                id: format!("item-{}", Uuid::new_v4()),
                pk: "pk-1".to_string(),
                value: 7,
            };
            rid_container
                .create_item(&item.pk, &item.id, &item, None)
                .await?;

            // A point operation addressed by *name* cannot work under a
            // RID-addressed parent: the service classifies the URI as RID-based
            // from its `dbs` segment alone and then fails to parse the item name
            // as a ResourceId (`400 Failed to parse the value '{name}' as
            // ResourceId`). The driver rejects it client-side instead, before
            // signing or sending anything.
            for result in [
                rid_container.read_item(&item.pk, &item.id, None).await.err(),
                rid_container
                    .delete_item(&item.pk, &item.id, None)
                    .await
                    .err(),
            ] {
                let err = result.expect(
                    "a name-addressed point operation on a RID-addressed container must be rejected",
                );
                assert_eq!(CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING, err.status());
            }

            // The item really was created through the RID-addressed container:
            // it is readable through the name-addressed one.
            let fetched: RidItem = name_container
                .read_item(&item.pk, &item.id, None)
                .await?
                .into_model()?;
            assert_eq!(item, fetched);
            let item_rid = name_container
                .read_item(&item.pk, &item.id, None)
                .await?
                .into_model::<serde_json::Value>()?
                .get("_rid")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
                .expect("item read should return a _rid");

            let fetched_by_rid: RidItem = rid_container
                .read_item(&item.pk, ResourceId::from(item_rid.clone()), None)
                .await?
                .into_model()?;
            assert_eq!(item, fetched_by_rid);

            item.value = 8;
            rid_container
                .replace_item(
                    &item.pk,
                    ResourceId::from(item_rid.clone()),
                    &item,
                    None,
                )
                .await?;
            let replaced: RidItem = rid_container
                .read_item(&item.pk, ResourceId::from(item_rid.clone()), None)
                .await?
                .into_model()?;
            assert_eq!(item, replaced);

            // PATCH by RID exercises the driver's read-modify-write loop, whose
            // internal Read and Replace sub-operations must each preserve the
            // leaf RID rather than re-deriving a name.
            item.value = 9;
            let patched: RidItem = rid_container
                .patch_item(
                    &item.pk,
                    ResourceId::from(item_rid.clone()),
                    PatchInstructions::from(vec![PatchOperation::set(
                        "/value",
                        serde_json::json!(9),
                    )]),
                    None,
                )
                .await?
                .into_model()?;
            assert_eq!(item, patched);

            // Client-side RID validation: these all fail before any network
            // call, so a bad RID never reaches the service as a misrouted
            // request. A malformed RID and a well-formed RID that decodes to a
            // non-document (here the container's own RID) are both rejected.
            for bad_rid in ["not-a-rid", container_rid.as_str()] {
                let err = rid_container
                    .read_item(&item.pk, ResourceId::from(bad_rid), None)
                    .await
                    .expect_err("a non-document RID must be rejected client-side");
                assert_eq!(CosmosStatus::CLIENT_INVALID_RESOURCE_ID, err.status());
            }

            // A RID item id under a *name*-addressed container is the mirror of
            // the name-under-RID case above and is equally invalid.
            let err = name_container
                .read_item(&item.pk, ResourceId::from(item_rid.clone()), None)
                .await
                .expect_err("a RID item id requires a RID-addressed container");
            assert_eq!(CosmosStatus::CLIENT_MIXED_NAME_RID_ADDRESSING, err.status());

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

            rid_container
                .delete_item(&item.pk, ResourceId::from(item_rid), None)
                .await?;

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

            // db2 + a container in db2, created out of band. Database
            // create/delete is management-plane and is not granted by the
            // data-plane RBAC role used in AAD mode, so both go through the
            // management (key) client.
            let db2_name = format!("rid-otherdb-{}", Uuid::new_v4());
            let _ = run_context
                .management_client()
                .create_database(&db2_name, None)
                .await?;
            let db2_client = run_context
                .management_client()
                .database_client(db2_name.as_str());
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
