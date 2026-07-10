// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::framework;

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::error::Error;

use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::feed::{FeedRange, FeedScope};
use azure_data_cosmos::models::{
    ContainerProperties, EffectivePartitionKey, PartitionKeyDefinition, ThroughputProperties,
};
use azure_data_cosmos::options::CreateContainerOptions;
use azure_data_cosmos::{PartitionKey, Query};
use base64::Engine;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use framework::{TestClient, TestOptions};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn read_feed_ranges_returns_physical_partitions() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeContainer", "/pk".into());

            // Use 11000 RU/s to ensure at least 2 physical partitions (10000 RU/s per partition).
            let throughput = ThroughputProperties::manual(11000);
            let options = CreateContainerOptions::default().with_throughput(throughput);

            let container_client = run_context
                .create_container(db_client, properties, Some(options))
                .await?;

            let ranges = container_client.read_feed_ranges(None).await?;

            // With 11000 RU/s the service should create at least 2 physical partitions.
            assert!(
                ranges.len() >= 2,
                "expected at least 2 feed ranges with 11000 RU/s, got {}",
                ranges.len()
            );

            // No two ranges should overlap (they partition the EPK space).
            for i in 0..ranges.len() {
                for j in (i + 1)..ranges.len() {
                    assert_ne!(
                        ranges[i], ranges[j],
                        "ranges {i} and {j} should be distinct"
                    );
                }
            }

            // Each range should be serializable via Display and parseable via FromStr.
            for range in &ranges {
                let serialized = range.to_string();
                // Verify the serialized string is valid base64-encoded JSON
                // with the expected cross-SDK structure.
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(&serialized)
                    .expect("feed range Display should produce valid base64");
                let json: serde_json::Value =
                    serde_json::from_slice(&decoded).expect("decoded base64 should be valid JSON");
                let inner = json.get("Range").expect("expected 'Range' key");
                assert!(inner.get("min").is_some(), "expected 'min' field");
                assert!(inner.get("max").is_some(), "expected 'max' field");
                assert!(
                    inner.get("isMinInclusive").unwrap().as_bool().unwrap(),
                    "isMinInclusive should be true"
                );
                assert!(
                    !inner.get("isMaxInclusive").unwrap().as_bool().unwrap(),
                    "isMaxInclusive should be false"
                );
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
pub async fn feed_range_from_partition_key_maps_correctly() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeFromPK", "/pk".into());

            // Use 11000 RU/s to ensure at least 2 physical partitions.
            let throughput = ThroughputProperties::manual(11000);
            let options = CreateContainerOptions::default().with_throughput(throughput);

            let container_client = run_context
                .create_container(db_client, properties, Some(options))
                .await?;

            // Get the physical partition ranges.
            let physical_ranges = container_client.read_feed_ranges(None).await?;

            // Get the feed range for a specific partition key.
            let pk_ranges = container_client
                .feed_range_from_partition_key("test_partition_key", None)
                .await?;

            // Full key should return exactly one feed range.
            assert_eq!(
                pk_ranges.len(),
                1,
                "full partition key should map to exactly one feed range"
            );
            let pk_range = &pk_ranges[0];

            // The returned range must match one of the physical partitions.
            let matches_physical = physical_ranges.iter().any(|pr| pr == pk_range);
            assert!(
                matches_physical,
                "feed_range_from_partition_key should return one of the physical partition ranges"
            );

            // The same partition key should always map to the same range (deterministic).
            let pk_ranges_again = container_client
                .feed_range_from_partition_key("test_partition_key", None)
                .await?;
            assert_eq!(
                pk_ranges, pk_ranges_again,
                "same PK should map to same range"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that `feed_range_from_partition_key` returns exactly one feed range
/// for a full hierarchical partition key (all components provided).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn feed_range_from_full_hpk_returns_single_range() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new(
                "FeedRangeHPKFull",
                ("/tenant", "/user", "/session").into(),
            );

            let container_client = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Full key: all 3 components provided.
            let pk = azure_data_cosmos::PartitionKey::from(("tenantA", "user1", "sess1"));
            let ranges = container_client
                .feed_range_from_partition_key(pk, None)
                .await?;

            assert_eq!(
                ranges.len(),
                1,
                "full HPK should map to exactly one feed range"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that `feed_range_from_partition_key` returns one or more feed ranges
/// for a prefix hierarchical partition key (fewer components than paths).
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn feed_range_from_prefix_hpk_returns_ranges() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new(
                "FeedRangeHPKPrefix",
                ("/tenant", "/user", "/session").into(),
            );

            let container_client = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Prefix key: only 1 of 3 components.
            let pk = azure_data_cosmos::PartitionKey::from("tenantA");
            let ranges = container_client
                .feed_range_from_partition_key(pk, None)
                .await?;

            // Should return at least one feed range.
            assert!(
                !ranges.is_empty(),
                "prefix HPK should return at least one feed range"
            );

            // All returned ranges should be distinct.
            for range in &ranges {
                assert_ne!(
                    range.to_string(),
                    "",
                    "feed range should serialize to a non-empty string"
                );
            }

            // No two returned ranges should be equal.
            for i in 0..ranges.len() {
                for j in (i + 1)..ranges.len() {
                    assert_ne!(
                        ranges[i], ranges[j],
                        "returned feed ranges should be distinct"
                    );
                }
            }

            // Prefix with 2 of 3 components.
            let pk2 = azure_data_cosmos::PartitionKey::from(("tenantA", "user1"));
            let ranges2 = container_client
                .feed_range_from_partition_key(pk2, None)
                .await?;

            assert!(
                !ranges2.is_empty(),
                "prefix HPK (2-of-3) should return at least one feed range"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that `feed_range_from_partition_key` works correctly for
/// a full key on a single-hash container.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn feed_range_from_partition_key_single_hash_full_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("FeedRangeSingleHash", "/pk".into());
            let container_client = run_context
                .create_container(db_client, properties, None)
                .await?;

            let result = container_client
                .feed_range_from_partition_key("valid_key", None)
                .await;
            assert!(result.is_ok(), "full key on single-hash should succeed");

            let ranges = result.unwrap();
            assert_eq!(
                ranges.len(),
                1,
                "full key should return exactly one feed range"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

const DOC_COUNT: usize = 40;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Doc {
    id: String,
    pk: String,
}

fn pk_definition() -> PartitionKeyDefinition {
    PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")])
}

/// Runs a feed-range-scoped `SELECT *` and returns the set of returned ids.
///
/// Drains the result set page-by-page (via [`into_pages`]) rather than
/// item-by-item; the paged future is small enough to satisfy
/// `clippy::large-futures`, which the item-level stream would otherwise trip.
async fn drain_ids(
    container: &ContainerClient,
    scope: FeedScope,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let mut pages = container
        .query_items::<Doc>(Query::from("SELECT * FROM c"), scope, None)
        .await?
        .into_pages();
    let mut got = BTreeSet::new();
    while let Some(page) = pages.next().await {
        for doc in page?.into_items() {
            got.insert(doc.id);
        }
    }
    Ok(got)
}

/// End-to-end regression test for the cross-partition query **scope bug**: the
/// planner used to ignore the caller's `FeedScope::range(..)` feed range.
///
/// Before the fix, `plan_fresh` built its request ranges purely from the query
/// plan (`query_plan.query_ranges`) and never intersected them with the
/// operation's scope feed range (`operation.target()`). For a plain `SELECT *`
/// the query plan reports the whole container, so a `FeedScope::range([X, Y))`
/// window was silently dropped and the query did a **full scan** — returning
/// documents outside the requested window.
///
/// This seeds one document per partition key, computes each document's
/// effective partition key locally, and issues cross-partition queries scoped
/// to interior EPK windows, asserting the result set is exactly the windowed
/// subset. Without the planner fix these fail with `SCOPE BUG`; with the fix —
/// plus the `x-ms-read-key-type: EffectivePartitionKeyRange` header correction
/// that lets the gateway accept the emitted interior EPK window — they pass.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn feed_range_scope_restricts_cross_partition_query() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("ScopeBugContainer", pk_definition());
            let container = run_context
                .create_container(db_client, properties, None)
                .await?;

            // Seed DOC_COUNT documents, each on its own partition key.
            let mut docs: Vec<Doc> = Vec::with_capacity(DOC_COUNT);
            for i in 0..DOC_COUNT {
                let id = format!("doc-{i:03}");
                let doc = Doc {
                    id: id.clone(),
                    pk: id.clone(),
                };
                container
                    .create_item(PartitionKey::from(doc.pk.clone()), &id, doc.clone(), None)
                    .await?;
                docs.push(doc);
            }

            // Compute each document's effective partition key locally (same hash
            // the service uses), then sort ascending by EPK.
            let pk_def = pk_definition();
            let mut points: Vec<(String, FeedRange)> = docs
                .iter()
                .map(|d| {
                    let fr = FeedRange::for_partition(PartitionKey::from(d.pk.clone()), &pk_def);
                    (d.id.clone(), fr)
                })
                .collect();
            points.sort_by(|a, b| a.1.min_inclusive().cmp(b.1.min_inclusive()));
            let k = points.len();

            // Control: a full-container scan must return every seeded document.
            let all: BTreeSet<String> = points.iter().map(|(id, _)| id.clone()).collect();
            let control = drain_ids(&container, FeedScope::full_container()).await?;
            assert_eq!(
                control, all,
                "control full-container scan should return all {DOC_COUNT} docs"
            );

            // Test B: a WIDE interior window [X1, X_{k-1}) must exclude the
            // globally smallest and largest EPKs => expect k-2 documents.
            let window_b = FeedRange::new(
                points[1].1.min_inclusive().clone(),
                points[k - 1].1.min_inclusive().clone(),
            )?;
            let expected_b: BTreeSet<String> =
                points[1..k - 1].iter().map(|(id, _)| id.clone()).collect();
            let got_b = drain_ids(&container, FeedScope::range(window_b)).await?;
            assert_eq!(
                got_b,
                expected_b,
                "SCOPE BUG (wide window): FeedScope::range was ignored and the \
                 query fell back to a full scan. got {} docs, expected {}. \
                 unexpected(outside window)={:?}",
                got_b.len(),
                expected_b.len(),
                got_b.difference(&expected_b).collect::<Vec<_>>()
            );

            // Test C: the TIGHTEST interior window [X_mid, X_{mid+1}) must
            // return exactly one document.
            let mid = k / 2;
            let window_c = FeedRange::new(
                points[mid].1.min_inclusive().clone(),
                points[mid + 1].1.min_inclusive().clone(),
            )?;
            let expected_c: BTreeSet<String> = std::iter::once(points[mid].0.clone()).collect();
            let got_c = drain_ids(&container, FeedScope::range(window_c)).await?;
            assert_eq!(
                got_c,
                expected_c,
                "SCOPE BUG (tight window): FeedScope::range was ignored. \
                 got {} docs, expected 1. unexpected(outside window)={:?}",
                got_c.len(),
                got_c.difference(&expected_c).collect::<Vec<_>>()
            );

            // Test D: a window that lies ENTIRELY in the gap between two adjacent
            // EPKs must return zero documents. Appending a whole byte (`80`) to
            // `X_mid`'s hex keeps `X_mid` as a byte prefix, so the result is
            // strictly greater than `X_mid` and — because adjacent single-hash
            // EPKs differ within their leading bytes — strictly less than
            // `X_{mid+1}`. So `[X_mid || 0x80, X_{mid+1})` contains no doc's EPK.
            // (A single hex nibble would be dropped by the byte-wise hex parser,
            // so a full byte is required.) This guards against an off-by-one
            // where the lower bound is treated as inclusive of `X_mid`.
            let gap_start = EffectivePartitionKey::from(format!(
                "{}80",
                points[mid].1.min_inclusive().to_hex()
            ));
            let window_d = FeedRange::new(gap_start, points[mid + 1].1.min_inclusive().clone())?;
            let got_d = drain_ids(&container, FeedScope::range(window_d)).await?;
            assert!(
                got_d.is_empty(),
                "SCOPE BUG (empty gap window): a window between adjacent EPKs \
                 returned {} docs, expected 0. unexpected={:?}",
                got_d.len(),
                got_d.iter().collect::<Vec<_>>()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
