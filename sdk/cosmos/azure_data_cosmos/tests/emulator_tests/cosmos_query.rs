// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::collections::HashMap;
use std::error::Error;

use azure_core::http::headers::HeaderValue;
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    constants,
    options::{OperationOptions, QueryOptions},
    Query,
};
use framework::{test_data, MockItem, TestClient};
use futures::{StreamExt, TryStreamExt};

fn collect_matching_items(
    items: &[MockItem],
    predicate: impl Fn(&MockItem) -> bool,
) -> Vec<MockItem> {
    items.iter().filter(|p| predicate(p)).cloned().collect()
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn single_partition_query_simple() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let result_items: Vec<MockItem> = run_context
                .query_items(&container_client, "select * from docs c", "partition0")
                .await?;
            assert_eq!(
                collect_matching_items(&items, |p| p.partition_key == "partition0"),
                result_items
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn single_partition_query_with_parameters() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            // Find a merge order value in partition1's items
            let merge_order = items
                .iter()
                .find(|p| p.partition_key == "partition1")
                .expect("No items in partition1")
                .merge_order;

            // Query for items with that merge order
            let query = Query::from("select * from c where c.mergeOrder = @some_value")
                .with_parameter("@some_value", merge_order)?;
            let result_items: Vec<MockItem> = run_context
                .query_items(&container_client, query, "partition1")
                .await?;
            assert_eq!(
                collect_matching_items(&items, |p| p.merge_order == merge_order),
                result_items
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn single_partition_query_with_projection() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let result_items: Vec<String> = run_context
                .query_items(&container_client, "select value c.id from c", "partition1")
                .await?;
            assert_eq!(
                items
                    .iter()
                    .filter(|p| p.partition_key == "partition1")
                    .map(|p| p.id.to_string())
                    .collect::<Vec<_>>(),
                result_items
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn cross_partition_query_with_projection_and_filter() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 2);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let result_items: Vec<String> = run_context
                .query_items(
                    &container_client,
                    "select value c.id from c where c.mergeOrder between 40 and 60",
                    (),
                )
                .await?;

            assert_eq!(
                items
                    .iter()
                    .filter(|p| p.merge_order >= 40 && p.merge_order <= 60)
                    .map(|p| p.id.to_string())
                    .collect::<Vec<_>>(),
                result_items
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn cross_partition_query_with_order_by_fails_without_query_engine(
) -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let mut pager = container_client.query_items::<String>(
                "select value c.id from c order by c.mergeOrder",
                (),
                None,
            )?;
            let result = pager.try_next().await;

            let Err(err) = result else {
                panic!("expected an error but got a successful result");
            };
            assert_eq!(Some(StatusCode::BadRequest), err.http_status());

            let response =
                if let azure_core::error::ErrorKind::HttpResponse { raw_response, .. } = err.kind()
                {
                    raw_response.as_ref().unwrap().clone()
                } else {
                    panic!("expected an HTTP response error");
                };
            let sub_status = response.headers().get_optional_str(&constants::SUB_STATUS);

            // 1004 = CrossPartitionQueryNotServable
            assert_eq!(Some("1004"), sub_status);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn query_returns_index_and_query_metrics() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(5, 1);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            // Enable both index metrics and query metrics via custom headers
            let mut custom_headers = HashMap::new();
            custom_headers.insert(
                constants::COSMOS_POPULATEINDEXMETRICS,
                HeaderValue::from("true"),
            );
            custom_headers.insert(
                constants::DOCUMENTDB_POPULATEQUERYMETRICS,
                HeaderValue::from("true"),
            );
            let operation = OperationOptions::default().with_custom_headers(custom_headers);
            let options = QueryOptions::default().with_operation_options(operation);

            let mut pages = container_client
                .query_items::<MockItem>("select * from c", "partition0", Some(options))?
                .into_pages();

            // Get the first page and check metrics headers
            let page = pages
                .next()
                .await
                .expect("expected at least one page")?;

            assert!(!page.items().is_empty(), "expected items in first page");

            // Query metrics should be populated (semicolon-delimited key=value pairs)
            let query_metrics = page.query_metrics();
            assert!(
                query_metrics.is_some(),
                "expected query metrics to be present when x-ms-documentdb-populatequerymetrics is set"
            );
            assert!(
                query_metrics.unwrap().contains("totalExecutionTimeInMs"),
                "expected query metrics to contain totalExecutionTimeInMs"
            );

            // Index metrics should be populated (base64-decoded JSON from service)
            let index_metrics = page.index_metrics();
            assert!(
                index_metrics.is_some(),
                "expected index metrics to be present when x-ms-cosmos-populateindexmetrics is set"
            );

            // Verify common response metadata is also available on QueryFeedPage
            assert!(
                page.request_charge().is_some(),
                "expected request charge on feed page"
            );
            assert!(
                page.diagnostics().activity_id().is_some(),
                "expected activity ID on feed page"
            );
            assert!(
                page.diagnostics().server_duration_ms().is_some(),
                "expected server_duration_ms on feed page"
            );
            assert!(
                page.session_token().is_some(),
                "expected session token on feed page"
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn single_partition_query_pagination() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(1, 5);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let expected_items =
                collect_matching_items(&items, |p| p.partition_key == "partition0");
            assert!(
                expected_items.len() > 1,
                "need multiple items to test pagination"
            );

            // Force 1 item per page to exercise continuation token pagination
            let mut custom_headers = HashMap::new();
            custom_headers.insert(constants::MAX_ITEM_COUNT, HeaderValue::from_static("1"));
            let operation = OperationOptions::default().with_custom_headers(custom_headers);
            let options = QueryOptions::default().with_operation_options(operation);

            let mut pages = container_client
                .query_items::<MockItem>("select * from c", "partition0", Some(options))?
                .into_pages();

            let mut all_items = Vec::new();
            let mut page_count = 0;

            while let Some(page) = pages.next().await {
                let page = page?;
                assert!(
                    page.items().len() <= 1,
                    "expected at most 1 item per page, got {}",
                    page.items().len()
                );
                all_items.extend(page.into_items());
                page_count += 1;
            }

            assert!(
                page_count >= expected_items.len(),
                "expected at least {} pages with max-item-count=1, got {}",
                expected_items.len(),
                page_count
            );
            assert_eq!(expected_items, all_items);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn cross_partition_query_pagination() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(3, 3);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            // Force 1 item per page for cross-partition query
            let mut custom_headers = HashMap::new();
            custom_headers.insert(constants::MAX_ITEM_COUNT, HeaderValue::from_static("1"));
            let operation = OperationOptions::default().with_custom_headers(custom_headers);
            let options = QueryOptions::default().with_operation_options(operation);

            let mut pages = container_client
                .query_items::<MockItem>("select * from c", (), Some(options))?
                .into_pages();

            let mut all_items = Vec::new();
            let mut page_count = 0;

            while let Some(page) = pages.next().await {
                let page = page?;
                assert!(
                    page.items().len() <= 1,
                    "expected at most 1 item per page, got {}",
                    page.items().len()
                );
                all_items.extend(page.into_items());
                page_count += 1;
            }

            assert!(
                page_count > 1,
                "expected multiple pages with max-item-count=1, got {}",
                page_count
            );
            // Cross-partition ordering is not guaranteed, so just check count
            assert_eq!(
                items.len(),
                all_items.len(),
                "expected all items to be returned across pages"
            );

            Ok(())
        },
        None,
    )
    .await
}
