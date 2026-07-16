// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-partition query feature coverage.
//!
//! These tests exercise the query language features that are *servable within a
//! single logical partition* — aggregates, `DISTINCT`, `TOP`, `OFFSET`/`LIMIT`,
//! and `GROUP BY`. Scoping every query to one partition (`FeedScope::partition`)
//! makes it a "trivial" operation that the backend serves directly, without the
//! client-side cross-partition merge that the SDK's gateway pager does not
//! implement for these features. This mirrors the aggregate/distinct/orderby/
//! groupby suites that the Python and .NET SDKs maintain.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::collections::HashMap;
use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    feed::FeedScope,
    models::ContainerProperties,
    Query,
};
use framework::{TestClient, TestOptions};
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// The single logical partition every record in these tests lives in. Keeping
/// all records in one partition is what makes the advanced query features
/// (aggregates, `GROUP BY`, ...) servable by the backend.
const PARTITION: &str = "tenant-1";

/// A record with a repeated grouping key (`category`) and a numeric measure
/// (`amount`), which lets us assert meaningful `GROUP BY`, `DISTINCT`, and
/// aggregate results.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct SalesRecord {
    id: String,
    partition_key: String,
    category: String,
    amount: i64,
}

/// The fixed data set used by every test in this module.
///
/// Category → amounts:
/// - `a`: 10, 20, 30  (count 3, sum 60)
/// - `b`: 5, 15       (count 2, sum 20)
/// - `c`: 100         (count 1, sum 100)
///
/// Totals: count 6, sum 180, min 5, max 100, avg 30.
fn sales_records() -> Vec<SalesRecord> {
    let rows: &[(&str, i64)] = &[
        ("a", 10),
        ("a", 20),
        ("a", 30),
        ("b", 5),
        ("b", 15),
        ("c", 100),
    ];
    rows.iter()
        .enumerate()
        .map(|(i, (category, amount))| SalesRecord {
            id: format!("item-{i}"),
            partition_key: PARTITION.to_string(),
            category: (*category).to_string(),
            amount: *amount,
        })
        .collect()
}

/// Creates a single-partition container and seeds it with [`sales_records`].
async fn seed_container(db: &DatabaseClient) -> azure_data_cosmos::Result<ContainerClient> {
    let properties = ContainerProperties::new("QueryFeaturesContainer", "/partitionKey".into());

    // Retry on 429 (throttling) and tolerate a pre-existing container.
    loop {
        match db.create_container(properties.clone(), None).await {
            Ok(_) => break,
            Err(e) if e.status().status_code() == StatusCode::TooManyRequests => {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            Err(e) if e.status().status_code() == StatusCode::Conflict => break,
            Err(e) => return Err(e),
        }
    }

    let container = db.container_client("QueryFeaturesContainer").await?;
    for record in sales_records() {
        container
            .create_item(record.partition_key.clone(), &record.id, &record, None)
            .await?;
    }
    Ok(container)
}

/// Runs a single-partition query and collects every result item.
async fn run_query<T>(
    container: &ContainerClient,
    query: impl Into<Query>,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
    let mut pages = container
        .query_items::<T>(query, FeedScope::partition(PARTITION), None)
        .await?
        .into_pages();

    let mut items = Vec::new();
    while let Some(page) = pages.next().await {
        items.extend(page?.into_items());
    }
    Ok(items)
}

/// Runs a query expected to yield exactly one scalar (`SELECT VALUE ...`) row.
async fn run_scalar<T>(
    container: &ContainerClient,
    query: impl Into<Query>,
) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
    let mut results = run_query::<T>(container, query).await?;
    assert_eq!(
        results.len(),
        1,
        "expected exactly one scalar result from an aggregate query"
    );
    Ok(results.pop().unwrap())
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_count_aggregate() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            let count: i64 = run_scalar(&container, "SELECT VALUE COUNT(1) FROM c").await?;
            assert_eq!(count, 6, "expected COUNT(1) to match seeded record count");
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
pub async fn single_partition_sum_min_max_aggregates() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;

            let sum: i64 = run_scalar(&container, "SELECT VALUE SUM(c.amount) FROM c").await?;
            assert_eq!(sum, 180, "unexpected SUM(c.amount)");

            let min: i64 = run_scalar(&container, "SELECT VALUE MIN(c.amount) FROM c").await?;
            assert_eq!(min, 5, "unexpected MIN(c.amount)");

            let max: i64 = run_scalar(&container, "SELECT VALUE MAX(c.amount) FROM c").await?;
            assert_eq!(max, 100, "unexpected MAX(c.amount)");

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
pub async fn single_partition_avg_aggregate() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            let avg: f64 = run_scalar(&container, "SELECT VALUE AVG(c.amount) FROM c").await?;
            assert!(
                (avg - 30.0).abs() < f64::EPSILON,
                "expected AVG(c.amount) == 30.0, got {avg}"
            );
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
pub async fn single_partition_distinct() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            let mut categories: Vec<String> =
                run_query(&container, "SELECT DISTINCT VALUE c.category FROM c").await?;
            categories.sort();
            assert_eq!(
                categories,
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "expected DISTINCT to collapse to the three seeded categories"
            );
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
pub async fn single_partition_top_with_order_by() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            let amounts: Vec<i64> = run_query(
                &container,
                "SELECT VALUE c.amount FROM c ORDER BY c.amount DESC OFFSET 0 LIMIT 2",
            )
            .await?;
            assert_eq!(
                amounts,
                vec![100, 30],
                "expected the two largest amounts in descending order"
            );

            // `TOP` keyword form should agree with the ORDER BY + LIMIT form.
            let top: Vec<i64> = run_query(
                &container,
                "SELECT TOP 2 VALUE c.amount FROM c ORDER BY c.amount DESC",
            )
            .await?;
            assert_eq!(top, vec![100, 30], "TOP N disagreed with LIMIT form");

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
pub async fn single_partition_offset_limit() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            // Sorted amounts: [5, 10, 15, 20, 30, 100]; skip 1, take 2 => [10, 15].
            let amounts: Vec<i64> = run_query(
                &container,
                "SELECT VALUE c.amount FROM c ORDER BY c.amount ASC OFFSET 1 LIMIT 2",
            )
            .await?;
            assert_eq!(amounts, vec![10, 15], "unexpected OFFSET/LIMIT window");
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CategoryRollup {
    category: String,
    count: i64,
    total: i64,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_group_by() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let container = seed_container(db_client).await?;
            let rollups: Vec<CategoryRollup> = run_query(
                &container,
                "SELECT c.category AS category, COUNT(1) AS count, SUM(c.amount) AS total \
                 FROM c GROUP BY c.category",
            )
            .await?;

            let by_category: HashMap<String, (i64, i64)> = rollups
                .into_iter()
                .map(|r| (r.category, (r.count, r.total)))
                .collect();

            assert_eq!(by_category.len(), 3, "expected three groups");
            assert_eq!(
                by_category["a"],
                (3, 60),
                "unexpected rollup for category a"
            );
            assert_eq!(
                by_category["b"],
                (2, 20),
                "unexpected rollup for category b"
            );
            assert_eq!(
                by_category["c"],
                (1, 100),
                "unexpected rollup for category c"
            );
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
