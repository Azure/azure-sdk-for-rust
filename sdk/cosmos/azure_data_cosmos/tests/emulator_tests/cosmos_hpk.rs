// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end emulator coverage for Hierarchical Partition Keys (HPK).
//!
//! These tests exercise the public `azure_data_cosmos` surface against
//! MultiHash (hierarchical) containers: point CRUD with full keys, partial-key
//! and too-many-component rejection, full-key and prefix queries, cross-partition
//! queries, component value edge cases (numeric / bool / empty string),
//! and transactional batch. HPK feed-range negative validation lives alongside
//! the positive feed-range tests in `cosmos_feed_ranges.rs`.

// Use the shared test framework declared in `tests/emulator_tests/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::feed::FeedScope;
use azure_data_cosmos::models::{
    ContainerProperties, PartitionKeyKind, PatchInstructions, PatchOperation,
};
use azure_data_cosmos::{CosmosStatus, PartitionKey, Query, SubStatusCode, TransactionalBatch};
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
/// The cross-partition fan-out cases require the page iterator (`into_pages`),
/// because item-level `TryStream` draining currently 400s on a `full_container`
/// scope over an HPK container (see `hpk_query_cross_partition_full_container`).
/// Single-partition query tests may use either interface; this helper uses the
/// paged one uniformly so every query test shares one drain path.
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
/// Prefix (partial-key) HPK queries are issued through [`FeedScope::partition`]
/// with fewer components than the container has paths. Since #4729,
/// `FeedRange::for_partition` computes an EPK *range* (`compute_range`, not the
/// single-point `compute`) and the driver tags the request as an
/// `EffectivePartitionKeyRange`, so the backend filters results to the prefix
/// rather than scanning the whole physical partition.
///
/// The classic Cosmos emulator predates that support and rejects these requests
/// with 400 BadRequest, which is why the prefix tests below skip there — see
/// [`skip_prefix_query_on_classic_emulator`].
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

/// Prefix HPK queries (`FeedScope::partition` with a *partial* key) are servable
/// against a live account (validated by #4729) and the service-accurate
/// `vnext-preview` emulator, but the **classic** Cosmos emulator rejects them
/// with 400 BadRequest — it predates service-accurate EPK-range HPK query
/// support.
///
/// Returns `true` (and logs) when the current target is the classic emulator, so
/// a prefix test can skip cleanly there while still running against live accounts
/// and the vnext emulator. The #4680 filtering fix is additionally covered
/// deterministically in CI by `in_memory_emulator_tests::hpk`.
fn skip_prefix_query_on_classic_emulator() -> bool {
    // vnext is also a local emulator but *does* serve prefix EPK-range queries,
    // so only the non-vnext local emulator (classic) is skipped.
    let is_classic_emulator =
        framework::targets_emulator() && !cfg!(test_category = "emulator_vnext");
    if is_classic_emulator {
        eprintln!(
            "skipping prefix HPK query test: not servable on the classic Cosmos \
             emulator (400 BadRequest). Covered in CI by \
             in_memory_emulator_tests::hpk; runs here against live accounts and \
             the vnext emulator."
        );
    }
    is_classic_emulator
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
#[cfg(feature = "preview_patch")]
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: 400 BadRequest is returned for a partial-key point \
              read, but without the x-ms-substatus 1001 PartitionKeyMismatch header (behavioral \
              divergence)"
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
            assert_eq!(
                err.status().sub_status(),
                Some(SubStatusCode::PARTITION_KEY_MISMATCH),
                "partial-key point read should carry sub-status 1001 PartitionKeyMismatch"
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
            // rejected by the gateway with a bare 400 BadRequest. Unlike the
            // partial-key case (A6), the emulator returns no sub-status here, so
            // only the status code is asserted.
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

/// A8: a point read for an existing id addressed to a *different* (but valid,
/// full) logical partition must return 404 — the item does not live in the
/// partition being read, so it is not found there.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_item_wrong_partition_read_not_found() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            // `la-1` really lives in (USA, CA, LosAngeles); confirm it is readable there.
            run_context
                .read_item(
                    &container,
                    PartitionKey::from(("USA", "CA", "LosAngeles")),
                    "la-1",
                    None,
                )
                .await?;

            // Reading the same id from a different (valid, full) partition 404s.
            let err = container
                .read_item(PartitionKey::from(("USA", "WA", "Seattle")), "la-1", None)
                .await
                .expect_err("reading an id from the wrong partition should fail");
            assert_eq!(
                err.status().status_code(),
                StatusCode::NotFound,
                "reading an existing id from the wrong logical partition should 404"
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
///
/// Prefix filtering was fixed by #4729: the driver now emits
/// `x-ms-read-key-type: EffectivePartitionKeyRange` for EPK range-scoped
/// requests, so `FeedScope::partition(prefix)` filters to the prefix instead of
/// scanning the entire physical partition.
///
/// Runs against **live accounts** and the service-accurate `vnext-preview`
/// emulator. Skipped at runtime on the classic Cosmos emulator, which rejects
/// the prefix EPK-range query with 400 BadRequest; the #4680 fix is covered
/// deterministically in CI by `in_memory_emulator_tests::hpk`.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_prefix_level1() -> Result<(), Box<dyn Error>> {
    if skip_prefix_query_on_classic_emulator() {
        return Ok(());
    }
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
///
/// Runs against live accounts and the vnext emulator; skipped on the classic
/// emulator (see `hpk_query_prefix_level1`).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_prefix_level2() -> Result<(), Box<dyn Error>> {
    if skip_prefix_query_on_classic_emulator() {
        return Ok(());
    }
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
///
/// Runs against live accounts and the vnext emulator; skipped on the classic
/// emulator (see `hpk_query_prefix_level1`).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_prefix_no_match_returns_empty() -> Result<(), Box<dyn Error>> {
    if skip_prefix_query_on_classic_emulator() {
        return Ok(());
    }
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
///
/// Runs against live accounts and the vnext emulator; skipped on the classic
/// emulator (see `hpk_query_prefix_level1`).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_prefix_correctness_guard() -> Result<(), Box<dyn Error>> {
    if skip_prefix_query_on_classic_emulator() {
        return Ok(());
    }
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
/// Fixed by #4729: cross-partition (`full_container`) queries over a MultiHash
/// container previously failed with 400 BadRequest ("One of the input values is
/// invalid", empty PartitionKeyRangeId) because the driver hardcoded the point
/// key-type on EPK range-scoped requests. It now emits the range key-type and
/// the gateway accepts the fan-out.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

// ─── Group B (cont.) — query-surface semantics ───────────────────────────────

/// B7: a SQL filter on a *non-leading* partition-key path, scoped to a fully
/// routed (single logical partition) 3-level key, is served by the backend.
///
/// This deliberately scopes to a **full** key rather than a prefix so the case
/// under test is the SQL predicate itself, not prefix routing: prefix scoping is
/// exercised separately by the `hpk_query_prefix_*` tests. Here we prove that
/// within a routed partition the server accepts and applies a SQL predicate that
/// references a non-leading key path (`/city`).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_secondary_path_filter_servable() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            // Route to the full (USA, CA, SanFrancisco) key and have the server
            // apply a SQL predicate on the non-leading `/city` path.
            let items: Vec<GeoItem> = collect_query::<GeoItem>(
                &container,
                "SELECT * FROM c WHERE c.city = 'SanFrancisco'",
                FeedScope::partition(("USA", "CA", "SanFrancisco")),
            )
            .await?;

            assert_eq!(
                items.len(),
                2,
                "secondary-path filter should match 2 SF items"
            );
            assert!(items.iter().all(|i| i.city == "SanFrancisco"));
            assert_eq!(
                sorted_ids(items.into_iter().map(|i| i.id)),
                vec!["sf-1", "sf-2"]
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B8: an `ORDER BY` scoped to a single logical partition is servable — the
/// backend serves the sort within one partition without the client-side
/// cross-partition pipeline.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_single_partition_order_by_servable() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items: Vec<GeoItem> = collect_query::<GeoItem>(
                &container,
                "SELECT * FROM c ORDER BY c.population DESC",
                FeedScope::partition(("USA", "CA", "LosAngeles")),
            )
            .await?;

            // The 3 LA items, sorted by population descending.
            assert_eq!(
                items.iter().map(|i| i.id.clone()).collect::<Vec<_>>(),
                vec!["la-3", "la-2", "la-1"],
                "single-partition ORDER BY should return the LA items population-descending"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B9: cross-partition advanced query operators over a hierarchical container.
///
/// The SDK advertises only the features it has a client-side pipeline for (see
/// `query::SUPPORTED_QUERY_FEATURES`), and the service rejects any query needing
/// one it did not advertise with 400 BadRequest / 1004
/// CrossPartitionQueryNotServable. HPK fan-out itself already works (see B6),
/// so these cases turn on operator support alone.
///
/// `DISTINCT` and `OFFSET`/`LIMIT`/`TOP` are servable now that their stages
/// landed; aggregates and `GROUP BY` are still pipeline-less and remain
/// rejected.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn hpk_query_cross_partition_advanced_not_servable() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            // Servable: DISTINCT has a client-side stage, and it must
            // deduplicate correctly across the container's physical partitions.
            let mut countries = collect_query::<serde_json::Value>(
                &container,
                "SELECT DISTINCT VALUE c.country FROM c",
                FeedScope::full_container(),
            )
            .await?
            .into_iter()
            .map(|v| v.as_str().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
            countries.sort();
            assert_eq!(
                countries,
                vec!["CANADA".to_string(), "USA".to_string()],
                "cross-partition DISTINCT over an HPK container must return each country once"
            );

            // Still rejected: no client-side pipeline for these yet.
            let advanced = [
                "SELECT VALUE COUNT(1) FROM c",
                "SELECT c.state, COUNT(1) AS n FROM c GROUP BY c.state",
            ];

            for query in advanced {
                let err = collect_query::<serde_json::Value>(
                    &container,
                    query,
                    FeedScope::full_container(),
                )
                .await
                .expect_err(&format!(
                    "cross-partition query should not be servable on an HPK container: {query}"
                ));

                let status = err
                    .downcast_ref::<azure_data_cosmos::CosmosError>()
                    .map(|e| e.status())
                    .unwrap_or_else(|| panic!("expected a CosmosError for {query}, got: {err}"));
                assert_eq!(
                    status,
                    CosmosStatus::CROSS_PARTITION_QUERY_NOT_SERVABLE,
                    "expected 400 / 1004 CrossPartitionQueryNotServable for {query}"
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// B9b: cross-partition `OFFSET/LIMIT` and `TOP` over a hierarchical container
/// are servable via the #4750 skip/take pipeline.
///
/// The dataset has 9 distinct documents spread across several physical
/// partitions. #4750 deliberately excludes `ORDER BY`, so the global row order
/// is unspecified; the stable contract is the window cardinality, exact-once
/// delivery, and membership in the seeded id set.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn hpk_query_cross_partition_offset_limit_top_servable() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let universe: Vec<String> = geo_dataset().into_iter().map(|i| i.id).collect();
            let total = universe.len();
            assert_eq!(total, 9, "seed dataset should have 9 documents");

            // (query, expected window cardinality) for a `total`-document container.
            let cases = [
                ("SELECT * FROM c OFFSET 1 LIMIT 2", 2),
                ("SELECT * FROM c OFFSET 8 LIMIT 5", total - 8),
                ("SELECT * FROM c OFFSET 20 LIMIT 5", 0),
                ("SELECT * FROM c OFFSET 3 LIMIT 0", 0),
                ("SELECT TOP 4 * FROM c", 4),
                ("SELECT TOP 0 * FROM c", 0),
                ("SELECT TOP 25 * FROM c", total),
            ];

            for (query, expected) in cases {
                let items: Vec<GeoItem> =
                    collect_query::<GeoItem>(&container, query, FeedScope::full_container())
                        .await?;

                assert_eq!(
                    items.len(),
                    expected,
                    "cross-partition query should return {expected} items: {query}"
                );

                let ids: Vec<String> = items.into_iter().map(|i| i.id).collect();
                let mut seen = std::collections::HashSet::new();
                for id in &ids {
                    assert!(
                        seen.insert(id.clone()),
                        "duplicate id {id} for query: {query}"
                    );
                    assert!(
                        universe.contains(id),
                        "result id {id} is not a seeded document for query: {query}"
                    );
                }
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// the single owning partition (closed point) and returns the matching docs.
///
/// The equality/`IN` collapse-to-point fix landed in #4638 (issue #4574), which
/// normalizes the closed point range `[X, X]` to `[X, normalized_successor(X))`
/// and routes it as an EPK window, so this predicate shape is now servable.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn hpk_query_full_key_equality_cross_partition() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = seed_three_level(run_context, db_client).await?;

            let items: Vec<GeoItem> = collect_query::<GeoItem>(
                &container,
                "SELECT * FROM c WHERE c.country = 'USA' AND c.state = 'CA' AND c.city = 'LosAngeles'",
                FeedScope::full_container(),
            )
            .await?;

            assert_eq!(items.len(), 3, "full-key equality should route to and return the 3 LA items");
            assert!(items.iter().all(|i| i.city == "LosAngeles"));

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
