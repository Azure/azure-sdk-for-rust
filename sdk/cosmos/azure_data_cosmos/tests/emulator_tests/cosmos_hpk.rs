// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end emulator coverage for Hierarchical Partition Keys (HPK).
//!
//! These tests exercise the public `azure_data_cosmos` surface against
//! MultiHash (hierarchical) containers: point CRUD with full keys, partial-key
//! and too-many-component rejection, full-key and prefix queries, cross-partition
//! queries, component value edge cases (null / numeric / bool / empty string),
//! and transactional batch. Single-hash feed-range negatives live alongside the
//! positive feed-range tests in `cosmos_feed_ranges.rs`.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::models::{
    ContainerProperties, PartitionKeyKind, PatchInstructions, PatchOperation,
};
use azure_data_cosmos::{PartitionKey, Query, TransactionalBatch};
use framework::{TestClient, TestOptions, TestRunContext};
use futures::{StreamExt, TryStreamExt};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// An item keyed by a three-level hierarchical partition key
/// (`/country/state/city`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GeoItem {
    id: String,
    country: String,
    state: String,
    city: String,
    population: i64,
}

impl GeoItem {
    fn new(id: &str, country: &str, state: &str, city: &str, population: i64) -> Self {
        Self {
            id: id.to_string(),
            country: country.to_string(),
            state: state.to_string(),
            city: city.to_string(),
            population,
        }
    }

    /// The full three-level partition key for this item.
    fn partition_key(&self) -> PartitionKey {
        PartitionKey::from((&self.country, &self.state, &self.city))
    }
}

/// An item keyed by a two-level hierarchical partition key
/// (`/tenantId/userId`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct TenantItem {
    id: String,
    tenant_id: String,
    user_id: String,
    value: i64,
}

impl TenantItem {
    fn new(id: &str, tenant_id: &str, user_id: &str, value: i64) -> Self {
        Self {
            id: id.to_string(),
            tenant_id: tenant_id.to_string(),
            user_id: user_id.to_string(),
            value,
        }
    }

    fn partition_key(&self) -> PartitionKey {
        PartitionKey::from((&self.tenant_id, &self.user_id))
    }
}

/// The deterministic geo dataset seeded by [`seed_three_level`].
///
/// Prefix counts are unambiguous:
/// * `(USA,)` → 8, `(USA, CA)` → 5, `(USA, CA, LosAngeles)` → 3,
/// * `(CANADA,)` → 1, `(USA, TX)` → 0, whole container → 9.
fn geo_dataset() -> Vec<GeoItem> {
    vec![
        GeoItem::new("la-1", "USA", "CA", "LosAngeles", 100),
        GeoItem::new("la-2", "USA", "CA", "LosAngeles", 101),
        GeoItem::new("la-3", "USA", "CA", "LosAngeles", 102),
        GeoItem::new("sf-1", "USA", "CA", "SanFrancisco", 200),
        GeoItem::new("sf-2", "USA", "CA", "SanFrancisco", 201),
        GeoItem::new("se-1", "USA", "WA", "Seattle", 300),
        GeoItem::new("se-2", "USA", "WA", "Seattle", 301),
        GeoItem::new("ny-1", "USA", "NY", "NewYork", 400),
        GeoItem::new("to-1", "CANADA", "ON", "Toronto", 500),
    ]
}

/// Creates a 3-level `/country/state/city` container and seeds [`geo_dataset`].
async fn seed_three_level(
    run_context: &TestRunContext,
    db_client: &DatabaseClient,
) -> Result<ContainerClient, Box<dyn Error>> {
    let properties = ContainerProperties::new("HpkGeo", ("/country", "/state", "/city").into());
    let container = run_context
        .create_container(db_client, properties, None)
        .await?;

    for item in geo_dataset() {
        container
            .create_item(item.partition_key(), &item.id, &item, None)
            .await?;
    }

    Ok(container)
}

/// Creates an empty 2-level `/tenantId/userId` container.
async fn create_two_level(
    run_context: &TestRunContext,
    db_client: &DatabaseClient,
) -> Result<ContainerClient, Box<dyn Error>> {
    let properties = ContainerProperties::new("HpkTenant", ("/tenantId", "/userId").into());
    let container = run_context
        .create_container(db_client, properties, None)
        .await?;
    Ok(container)
}

/// Drains a query to completion via its page iterator, returning every item.
///
/// All emulator query tests in this suite use the page iterator (`into_pages`)
/// rather than the item-level `TryStream`, because cross-partition fan-out is
/// only driven correctly through the paged interface.
async fn collect_query<T>(
    container: &ContainerClient,
    query: impl Into<Query>,
    scope: FeedScope,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
    let mut pages = container
        .query_items::<T>(query, scope, None)
        .await?
        .into_pages();
    let mut items = Vec::new();
    while let Some(page) = pages.next().await {
        items.extend(page?.into_items());
    }
    Ok(items)
}

/// Runs `SELECT * FROM c` scoped to a (possibly partial) hierarchical key and
/// returns the matching items.
///
/// Prefix (partial-key) HPK queries are issued through
/// [`FeedScope::partition`] with fewer components than the container has paths.
/// Despite its doc comment recommending a full key, `FeedScope::partition`
/// builds a logical-partition feed range whose effective partition key is
/// computed from the supplied prefix, which the gateway expands to the matching
/// sub-range. Resolving the prefix through
/// [`ContainerClient::feed_range_from_partition_key`] instead would return the
/// *physical* partition range (the whole partition) and therefore would not
/// filter by the prefix.
async fn query_geo_prefix(
    container: &ContainerClient,
    prefix: PartitionKey,
) -> Result<Vec<GeoItem>, Box<dyn Error>> {
    collect_query::<GeoItem>(container, "SELECT * FROM c", FeedScope::partition(prefix)).await
}

fn sorted_ids<I: IntoIterator<Item = String>>(ids: I) -> Vec<String> {
    let mut v: Vec<String> = ids.into_iter().collect();
    v.sort();
    v
}

// ─── Group A — HPK point CRUD ────────────────────────────────────────────────

/// A1: create an item with a full 3-level key and read it back by the same key.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_create_and_read_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("HpkGeoCrud", ("/country", "/state", "/city").into());
            let container = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Confirm the container really is hierarchical.
            let created = container.read(None).await?.into_model()?;
            assert_eq!(created.partition_key.kind(), PartitionKeyKind::MultiHash);
            assert_eq!(created.partition_key.paths().len(), 3);

            let item = GeoItem::new("la-1", "USA", "CA", "LosAngeles", 100);
            let create = container
                .create_item(item.partition_key(), &item.id, &item, None)
                .await?;
            assert_eq!(create.status(), StatusCode::Created);

            let read = run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(read.status(), StatusCode::Ok);
            assert_eq!(read.into_model::<GeoItem>()?, item);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A2: replace an item addressed by a full 2-level key.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_replace_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let mut item = TenantItem::new("i1", "tenantA", "user1", 1);
            container
                .create_item(item.partition_key(), &item.id, &item, None)
                .await?;

            item.value = 99;
            let replace = container
                .replace_item(item.partition_key(), &item.id, &item, None)
                .await?;
            assert_eq!(replace.status(), StatusCode::Ok);

            let read = run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<TenantItem>()?.value, 99);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A3: upsert (insert then update) an item addressed by a full 2-level key.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_upsert_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let mut item = TenantItem::new("i1", "tenantA", "user1", 1);
            let insert = container
                .upsert_item(item.partition_key(), &item.id, &item, None)
                .await?;
            assert_eq!(insert.status(), StatusCode::Created);

            item.value = 42;
            let update = container
                .upsert_item(item.partition_key(), &item.id, &item, None)
                .await?;
            assert_eq!(update.status(), StatusCode::Ok);

            let read = run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<TenantItem>()?.value, 42);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A4: delete an item by full 2-level key, then confirm a follow-up read 404s.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_delete_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let item = TenantItem::new("i1", "tenantA", "user1", 1);
            container
                .create_item(item.partition_key(), &item.id, &item, None)
                .await?;
            // Ensure the item is visible before deleting.
            run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;

            let delete = container
                .delete_item(item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(delete.status(), StatusCode::NoContent);

            let err = container
                .read_item(item.partition_key(), &item.id, None)
                .await
                .expect_err("read after delete should fail");
            assert_eq!(err.status().status_code(), StatusCode::NotFound);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A5: patch a field on an item addressed by a full 2-level key.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_patch_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let item = TenantItem::new("i1", "tenantA", "user1", 1);
            container
                .create_item(item.partition_key(), &item.id, &item, None)
                .await?;

            let patch =
                PatchInstructions::from(vec![PatchOperation::set("/value", serde_json::json!(7))]);
            let patched = container
                .patch_item(item.partition_key(), &item.id, patch, None)
                .await?;
            assert_eq!(patched.status(), StatusCode::Ok);

            let read = run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<TenantItem>()?.value, 7);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A6 (discovery D-1): a point operation addressed by a partial (prefix) key on
/// a hierarchical container must fail — a point op requires the full logical key.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_partial_key_point_op_fails() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            // Address an existing item (USA, CA, LosAngeles) with only a 2-level prefix.
            let prefix = PartitionKey::from(("USA", "CA"));
            let err = container
                .read_item(prefix, "la-1", None)
                .await
                .expect_err("partial-key point read should fail");
            // The gateway rejects a partial logical key on a point operation
            // with 400 BadRequest (sub-status 1001 PartitionKeyMismatch).
            assert_eq!(
                err.status().status_code(),
                StatusCode::BadRequest,
                "partial-key point read should be rejected with 400 BadRequest"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// A7 (discovery D-2): a point operation with more components than the container
/// has paths must fail.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_too_many_components_point_op_fails() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            // Two-level container, three-component key.
            let too_many = PartitionKey::from(("tenantA", "user1", "extra"));
            let item = TenantItem::new("i1", "tenantA", "user1", 1);
            let err = container
                .create_item(too_many, &item.id, &item, None)
                .await
                .expect_err("too-many-component point op should fail");
            // A point op with more components than the container has paths is
            // rejected by the gateway with 400 BadRequest (1001 PartitionKeyMismatch).
            assert_eq!(
                err.status().status_code(),
                StatusCode::BadRequest,
                "too-many-component point op should be rejected with 400 BadRequest"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

// ─── Group B — HPK queries ───────────────────────────────────────────────────

/// B1: a query scoped to a full 3-level key returns exactly that partition.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_full_key_scope() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items: Vec<GeoItem> = container
                .query_items::<GeoItem>(
                    "SELECT * FROM c",
                    FeedScope::partition(("USA", "CA", "LosAngeles")),
                    None,
                )
                .await?
                .try_collect()
                .await?;

            assert_eq!(
                items.len(),
                3,
                "full-key scope should return the 3 LA items"
            );
            assert!(items.iter().all(|i| i.city == "LosAngeles"));
            assert_eq!(
                sorted_ids(items.into_iter().map(|i| i.id)),
                vec!["la-1", "la-2", "la-3"]
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B2: a level-1 prefix query returns every item under that prefix.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[ignore = "GAP (#4680): HPK prefix queries are not filtered server-side. FeedScope::partition(prefix) \
            builds a single-point EPK via EffectivePartitionKey::compute (not compute_range), so a \
            partial key scans the entire physical partition and returns every item. Re-enable once \
            the SDK exposes a filtered prefix-query path."]
pub async fn hpk_query_prefix_level1() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items = query_geo_prefix(&container, PartitionKey::from("USA")).await?;
            assert_eq!(items.len(), 8, "prefix (USA,) should return 8 items");
            assert!(items.iter().all(|i| i.country == "USA"));

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B3: a level-2 prefix query returns every item under that prefix.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[ignore = "GAP (#4680): HPK prefix queries are not filtered server-side (see hpk_query_prefix_level1)."]
pub async fn hpk_query_prefix_level2() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items = query_geo_prefix(&container, PartitionKey::from(("USA", "CA"))).await?;
            assert_eq!(items.len(), 5, "prefix (USA, CA) should return 5 items");
            assert!(items.iter().all(|i| i.state == "CA"));

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B4: a prefix that matches no items returns an empty result without error.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[ignore = "GAP (#4680): HPK prefix queries are not filtered server-side (see hpk_query_prefix_level1)."]
pub async fn hpk_query_prefix_no_match_returns_empty() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items = query_geo_prefix(&container, PartitionKey::from(("USA", "TX"))).await?;
            assert!(items.is_empty(), "prefix (USA, TX) should return 0 items");

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B5: a prefix query returns ONLY items under the prefix and excludes siblings
/// — an explicit anti-leak guard (cf. the .NET prefix-iterator test that is
/// currently `[Ignore]`d).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[ignore = "GAP (#4680): HPK prefix queries are not filtered server-side (see hpk_query_prefix_level1)."]
pub async fn hpk_query_prefix_correctness_guard() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items = query_geo_prefix(&container, PartitionKey::from(("USA", "CA"))).await?;

            assert_eq!(items.len(), 5);
            // Only CA items; nothing from WA, NY, or CANADA leaks in.
            assert!(items.iter().all(|i| i.country == "USA" && i.state == "CA"));
            assert!(items.iter().all(|i| i.state != "WA" && i.state != "NY"));
            assert!(items.iter().all(|i| i.country != "CANADA"));
            assert_eq!(
                sorted_ids(items.into_iter().map(|i| i.id)),
                vec!["la-1", "la-2", "la-3", "sf-1", "sf-2"]
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B6: a simple cross-partition query over a hierarchical container fans out and
/// merges results (no advanced query features involved).
///
/// Currently fails: a `full_container()` (cross-partition) query over a MultiHash
/// container is rejected by the gateway with 400 BadRequest
/// ("One of the input values is invalid", empty PartitionKeyRangeId). The same
/// query shape succeeds on single-hash containers, so the cross-partition fan-out
/// path does not build a valid partition target for hierarchical containers.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[ignore = "GAP (#4681): cross-partition (full_container) queries over hierarchical containers are \
            rejected with 400 BadRequest ('One of the input values is invalid'). The same query \
            works on single-hash containers. Re-enable once cross-partition fan-out targets HPK \
            containers correctly."]
pub async fn hpk_query_cross_partition_full_container() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items: Vec<String> = collect_query::<String>(
                &container,
                "SELECT VALUE c.city FROM c WHERE c.country = 'USA'",
                FeedScope::full_container(),
            )
            .await?;

            assert_eq!(
                items.len(),
                8,
                "cross-partition USA filter should match 8 items"
            );
            assert!(!items.contains(&"Toronto".to_string()));

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

// ─── Group D — HPK component value edge cases ────────────────────────────────

/// D-edge2: a hierarchical key whose components mix string, numeric, and boolean
/// values round-trips through create / read / full-key query.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_numeric_and_bool_components() -> Result<(), Box<dyn Error>> {
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct MixedItem {
        id: String,
        tenant: String,
        ordinal: i64,
        active: bool,
    }

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties =
                ContainerProperties::new("HpkMixed", ("/tenant", "/ordinal", "/active").into());
            let container = run_context
                .create_container(db_client, properties, None)
                .await?;

            let item = MixedItem {
                id: "m1".to_string(),
                tenant: "tenantA".to_string(),
                ordinal: 7,
                active: true,
            };
            let pk = PartitionKey::from((&item.tenant, item.ordinal, item.active));

            container
                .create_item(pk.clone(), &item.id, &item, None)
                .await?;

            let read = run_context
                .read_item(&container, pk.clone(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<MixedItem>()?, item);

            let queried: Vec<MixedItem> = container
                .query_items::<MixedItem>("SELECT * FROM c", FeedScope::partition(pk), None)
                .await?
                .try_collect()
                .await?;
            assert_eq!(queried, vec![item]);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// D-edge3: a hierarchical key with an empty-string component round-trips.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_empty_string_component() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let item = TenantItem::new("i1", "tenantA", "", 1);
            container
                .create_item(item.partition_key(), &item.id, &item, None)
                .await?;

            let read = run_context
                .read_item(&container, item.partition_key(), &item.id, None)
                .await?;
            assert_eq!(read.into_model::<TenantItem>()?, item);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

// ─── Group E — HPK transactional batch ───────────────────────────────────────

/// E1: a transactional batch scoped to a full 2-level key executes its operations.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_batch_create_and_read_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let pk = PartitionKey::from(("tenantA", "user1"));
            let item1 = TenantItem::new("i1", "tenantA", "user1", 1);
            let item2 = TenantItem::new("i2", "tenantA", "user1", 2);

            let batch = TransactionalBatch::new(pk)
                .create_item(&item1)?
                .create_item(&item2)?
                .read_item("i1", None);

            let response = container.execute_transactional_batch(batch, None).await?;
            assert_eq!(response.status(), StatusCode::Ok);

            let model = response.into_model()?;
            let codes: Vec<u16> = model.results().iter().map(|r| r.status_code()).collect();
            assert_eq!(codes, vec![201, 201, 200]);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// E2 (discovery D-4): a batch operation whose item belongs to a different
/// partition than the batch is rejected.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_batch_item_outside_partition_fails() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_two_level(run_context, db_client).await?;

            let pk = PartitionKey::from(("tenantA", "user1"));
            // Item body declares a different logical partition than the batch.
            let mismatched = TenantItem::new("i1", "tenantB", "user9", 1);

            let batch = TransactionalBatch::new(pk).create_item(&mismatched)?;
            let response = container
                .execute_transactional_batch(batch, None)
                .await
                .expect("a PK-mismatched batch item returns a 207 response, not a transport error");

            // The batch is accepted as HTTP 207 MultiStatus, but the offending
            // operation fails with 400 (sub-status 1001 PartitionKeyMismatch) and
            // the transaction does not commit.
            assert_eq!(response.status(), StatusCode::MultiStatus);
            let model = response.into_model()?;
            let codes: Vec<u16> = model.results().iter().map(|r| r.status_code()).collect();
            assert!(
                codes.contains(&400),
                "expected the PK-mismatched operation to fail with 400, got {codes:?}"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
