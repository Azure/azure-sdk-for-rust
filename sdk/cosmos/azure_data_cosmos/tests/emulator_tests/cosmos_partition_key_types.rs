// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-hash partition-key value-type sanity coverage.
//!
//! Cosmos single-hash (`Hash`) containers accept any scalar JSON type as the
//! partition-key value — string, number, or boolean. The rest of the emulator
//! suite exercises single-hash keys almost exclusively with *string* values;
//! numeric and boolean values are only covered as *components* of a
//! hierarchical key (see `cosmos_hpk::hpk_numeric_and_bool_components`). These
//! tests close that gap by driving the full point-operation lifecycle
//! (create / read / replace / query / delete) with top-level numeric and
//! boolean partition keys, and by asserting that distinct scalar key values
//! route to distinct logical partitions. This mirrors the per-type partition
//! key sanity tests the Python and .NET SDKs maintain.

// Use the shared test framework declared in `tests/emulator_tests/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::PartitionKey;
use framework::{TestClient, TestOptions, TestRunContext};
use serde::{Deserialize, Serialize};

/// Creates a single-hash container keyed on `/pk`.
///
/// The key *path* is fixed; the key *value* type (string / number / bool) is
/// determined per-item by what each test writes, which is exactly the behavior
/// under test.
async fn create_pk_container(
    run_context: &TestRunContext,
    db_client: &DatabaseClient,
    container_id: &'static str,
) -> azure_data_cosmos::Result<ContainerClient> {
    let properties = ContainerProperties::new(container_id, "/pk".into());
    run_context
        .create_container(db_client, properties, None)
        .await
}

/// An item keyed by a numeric partition key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct NumericItem {
    id: String,
    pk: i64,
    label: String,
}

/// An item keyed by a boolean partition key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct BoolItem {
    id: String,
    pk: bool,
    label: String,
}

/// An item keyed by a floating-point partition key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct FloatItem {
    id: String,
    pk: f64,
    label: String,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn numeric_partition_key_lifecycle() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container =
                create_pk_container(run_context, db_client, "NumericPkContainer").await?;

            let item = NumericItem {
                id: "n1".to_string(),
                pk: 42,
                label: "created".to_string(),
            };
            let pk = PartitionKey::from(item.pk);

            // Create.
            container
                .create_item(pk.clone(), &item.id, &item, None)
                .await?;

            // Read back by numeric partition key.
            let read = run_context
                .read_item(&container, pk.clone(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<NumericItem>()?, item);

            // Replace, keyed by the same numeric partition key.
            let replaced = NumericItem {
                label: "replaced".to_string(),
                ..item.clone()
            };
            container
                .replace_item(pk.clone(), &item.id, &replaced, None)
                .await?;
            let reread = run_context
                .read_item(&container, pk.clone(), &item.id, None)
                .await?;
            assert_eq!(reread.into_model::<NumericItem>()?, replaced);

            // Query scoped to the numeric partition returns exactly the item.
            let queried: Vec<NumericItem> = run_context
                .query_items(&container, "SELECT * FROM c", pk.clone())
                .await?;
            assert_eq!(queried, vec![replaced]);

            // Delete, then confirm the item is gone.
            container.delete_item(pk.clone(), &item.id, None).await?;
            match container.read_item(pk.clone(), &item.id, None).await {
                Err(e) if e.status().status_code() == StatusCode::NotFound => {}
                Ok(_) => return Err("item still readable after delete".into()),
                Err(e) => return Err(e.into()),
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn numeric_partition_keys_route_independently() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container =
                create_pk_container(run_context, db_client, "NumericRoutingContainer").await?;

            // Two items with distinct numeric partition keys must land in
            // distinct logical partitions.
            let first = NumericItem {
                id: "a".to_string(),
                pk: 1,
                label: "one".to_string(),
            };
            let second = NumericItem {
                id: "b".to_string(),
                pk: 2,
                label: "two".to_string(),
            };
            container
                .create_item(PartitionKey::from(first.pk), &first.id, &first, None)
                .await?;
            container
                .create_item(PartitionKey::from(second.pk), &second.id, &second, None)
                .await?;

            // Each per-partition query sees only its own item.
            let in_p1: Vec<NumericItem> = run_context
                .query_items(&container, "SELECT * FROM c", PartitionKey::from(1))
                .await?;
            assert_eq!(in_p1, vec![first]);

            let in_p2: Vec<NumericItem> = run_context
                .query_items(&container, "SELECT * FROM c", PartitionKey::from(2))
                .await?;
            assert_eq!(in_p2, vec![second]);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn bool_partition_key_lifecycle() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_pk_container(run_context, db_client, "BoolPkContainer").await?;

            // One item in each of the two boolean partitions.
            let truthy = BoolItem {
                id: "t".to_string(),
                pk: true,
                label: "active".to_string(),
            };
            let falsy = BoolItem {
                id: "f".to_string(),
                pk: false,
                label: "inactive".to_string(),
            };
            container
                .create_item(PartitionKey::from(truthy.pk), &truthy.id, &truthy, None)
                .await?;
            container
                .create_item(PartitionKey::from(falsy.pk), &falsy.id, &falsy, None)
                .await?;

            // Read each back by its boolean partition key.
            let read_true = run_context
                .read_item(&container, PartitionKey::from(true), &truthy.id, None)
                .await?;
            assert_eq!(read_true.into_model::<BoolItem>()?, truthy);

            // `true` and `false` are distinct partitions: each query sees one item.
            let in_true: Vec<BoolItem> = run_context
                .query_items(&container, "SELECT * FROM c", PartitionKey::from(true))
                .await?;
            assert_eq!(in_true, vec![truthy]);

            let in_false: Vec<BoolItem> = run_context
                .query_items(&container, "SELECT * FROM c", PartitionKey::from(false))
                .await?;
            assert_eq!(in_false, vec![falsy.clone()]);

            // Delete the `false`-keyed item and confirm it is gone.
            container
                .delete_item(PartitionKey::from(false), &falsy.id, None)
                .await?;
            match container
                .read_item(PartitionKey::from(false), &falsy.id, None)
                .await
            {
                Err(e) if e.status().status_code() == StatusCode::NotFound => {}
                Ok(_) => return Err("false-keyed item still readable after delete".into()),
                Err(e) => return Err(e.into()),
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn float_partition_key_round_trips() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_pk_container(run_context, db_client, "FloatPkContainer").await?;

            let item = FloatItem {
                id: "pi".to_string(),
                pk: 3.5,
                label: "fractional".to_string(),
            };
            let pk = PartitionKey::from(item.pk);

            container
                .create_item(pk.clone(), &item.id, &item, None)
                .await?;

            let read = run_context
                .read_item(&container, pk.clone(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<FloatItem>()?, item);

            let queried: Vec<FloatItem> = run_context
                .query_items(&container, "SELECT * FROM c", pk.clone())
                .await?;
            assert_eq!(queried, vec![item]);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
