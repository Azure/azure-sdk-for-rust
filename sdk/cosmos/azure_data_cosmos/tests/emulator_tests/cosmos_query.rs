// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::{constants, Query};
use framework::{test_data, MockItem, TestClient};
use futures::TryStreamExt;

fn collect_matching_items(
    items: &[MockItem],
    predicate: impl Fn(&MockItem) -> bool,
) -> Vec<MockItem> {
    items.iter().filter(|p| predicate(p)).cloned().collect()
}

#[tokio::test]
pub async fn single_partition_query() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let result_items: Vec<MockItem> = run_context.query_items_infinite_retries(
                &container_client,
                "select * from docs c",
                "partition0",
            ).await?;
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
            let result_items: Vec<MockItem> = run_context.query_items_infinite_retries(
                &container_client,
                query,
                "partition1",
            ).await?;
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
pub async fn single_partition_query_with_projection() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let result_items: Vec<String> = run_context.query_items_infinite_retries(
                &container_client,
                "select value c.id from c",
                "partition1").await?;
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
pub async fn cross_partition_query_with_projection_and_filter() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;


            let result_items: Vec<String> = run_context.query_items_infinite_retries(
                &container_client,
                "select value c.id from c where c.mergeOrder between 40 and 60",
                (),
            ).await?;

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
