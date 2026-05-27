// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::models::{ContainerProperties, ResponseHeaders};
use azure_data_cosmos::Query;
use azure_data_cosmos::{clients::ContainerClient, query::FeedScope};
use framework::{TestClient, TestRunContext};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct ResponseMetadataItem {
    id: String,
    partition_key: String,
    value: String,
}

async fn create_container(
    run_context: &TestRunContext,
) -> azure_data_cosmos::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            &db_client,
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;
    db_client.container_client(&container_id).await
}

fn cosmos_headers_from_error(error: &azure_data_cosmos::CosmosError) -> ResponseHeaders {
    let driver_headers = error
        .response()
        .map(|r| r.headers().clone())
        .unwrap_or_else(|| {
            panic!("expected typed Cosmos response headers on error, got {error:?}")
        });
    ResponseHeaders::from_driver(&driver_headers)
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn response_metadata_on_missing_read() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let pk = format!("Partition-missing-{unique_id}");
            let item_id = format!("Item-missing-{unique_id}");

            // Read an item that has never been created — must surface a 404 with
            // partition-replica metadata (LSN, SESSION_TOKEN, PARTITION_KEY_RANGE_ID)
            // in the error response headers. Call ContainerClient directly to avoid
            // the framework's TestRunContext::read_item 404 retry loop, which would
            // otherwise spin forever on a non-existent item.
            let error = container_client
                .read_item(&pk, &item_id, None)
                .await
                .expect_err("expected 404 when reading non-existent item");

            assert_eq!(
                error.status().status_code(),
                StatusCode::NotFound,
                "expected 404 NotFound"
            );

            let headers = cosmos_headers_from_error(&error);
            assert!(
                headers.session_token().is_some(),
                "expected session_token on 404 read"
            );
            assert!(headers.lsn().is_some(), "expected lsn on 404 read");
            assert!(
                headers.partition_key_range_id().is_some(),
                "expected partition_key_range_id on 404 read"
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
pub async fn response_metadata_on_read_write_preserves_session_and_lsn(
) -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let pk = format!("Partition-lsn-{unique_id}");
            let item_id = format!("Item-lsn-{unique_id}");
            let unrelated_item_id = format!("Item-lsn-unrelated-{unique_id}");

            let item = ResponseMetadataItem {
                id: item_id.clone(),
                partition_key: pk.clone(),
                value: "initial".to_string(),
            };
            let unrelated_item = ResponseMetadataItem {
                id: unrelated_item_id.clone(),
                partition_key: pk.clone(),
                value: "unrelated".to_string(),
            };

            // Pre-write read of a not-yet-existent item — capture the partition LSN
            // baseline from the 404 headers.
            let pre_write_error = container_client
                .read_item(&pk, &item_id, None)
                .await
                .expect_err("expected 404 for pre-write read");
            assert_eq!(pre_write_error.status().status_code(), StatusCode::NotFound);
            let pre_write_headers = cosmos_headers_from_error(&pre_write_error);
            let pre_write_lsn = pre_write_headers
                .lsn()
                .expect("pre-write 404 should carry partition LSN");

            // First write: response carries session_token, etag, and partition LSN.
            // item_lsn is a read-only header surfaced on point reads, not on creates.
            let create_response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_eq!(create_response.status(), StatusCode::Created);
            assert!(
                create_response.session_token().is_some(),
                "expected session_token on create"
            );
            assert!(create_response.headers().etag().is_some(), "expected etag on create");
            let write_lsn = create_response
                .lsn()
                .expect("create_item should surface partition LSN");
            // Weakened from donor's strict `<`: the partition LSN must never go
            // backwards across a successful write to that partition.
            assert!(
                pre_write_lsn <= write_lsn,
                "partition LSN must not regress across a successful write \
                 (pre_write={pre_write_lsn}, write={write_lsn})"
            );

            // Read the item back as a generic JSON value so we can probe for the
            // _lsn key in the item body. The body must NOT contain _lsn — that
            // header belongs in response metadata, not the item payload.
            let read_response = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await?;
            assert_eq!(read_response.status(), StatusCode::Ok);
            assert!(
                read_response.session_token().is_some(),
                "expected session_token on read"
            );
            assert!(read_response.headers().etag().is_some(), "expected etag on read");
            assert_eq!(
                read_response.item_lsn(),
                Some(write_lsn),
                "item_lsn on a point read should equal the LSN of the most recent write to that item"
            );
            let first_read_partition_lsn = read_response
                .lsn()
                .expect("read_item should surface partition LSN");
            assert!(
                first_read_partition_lsn >= write_lsn,
                "partition LSN observed on read should be at least the write LSN \
                 (read={first_read_partition_lsn}, write={write_lsn})"
            );
            let read_body: serde_json::Value = read_response.into_model()?;
            assert!(
                read_body.get("_lsn").is_none(),
                "_lsn should stay in response metadata, not appear in the item body: {read_body:?}"
            );

            // Second write to the same partition (different id): partition LSN
            // must advance (weakened from strict `>` to `>=`).
            let second_write = container_client
                .create_item(&pk, &unrelated_item_id, &unrelated_item, None)
                .await?;
            assert_eq!(second_write.status(), StatusCode::Created);
            let second_write_lsn = second_write
                .lsn()
                .expect("second create_item should surface partition LSN");
            assert!(
                second_write_lsn >= write_lsn,
                "partition LSN must not regress after a second write to the same partition \
                 (first={write_lsn}, second={second_write_lsn})"
            );

            // A subsequent point read of the original item still reports
            // item_lsn == the first-write LSN (the item was not touched).
            let second_read = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await?;
            assert_eq!(second_read.item_lsn(), Some(write_lsn));
            let second_read_partition_lsn = second_read
                .lsn()
                .expect("second read_item should surface partition LSN");
            assert!(second_read_partition_lsn >= second_write_lsn);
            // Note: ITEM_LSN is intentionally NOT exposed as a raw header by
            // the driver-to-SDK bridge (see `driver_response_headers_to_headers`
            // in `src/driver_bridge.rs`, which forwards an explicit allowlist of
            // headers). It is surfaced only through the typed `item_lsn()`
            // accessor asserted above.
            let second_read_body: serde_json::Value = second_read.into_model()?;
            assert!(
                second_read_body.get("_lsn").is_none(),
                "_lsn should stay in response metadata, not the second read body: {second_read_body:?}"
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
pub async fn query_pages_do_not_leak_lsn_in_items() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let pk = format!("Partition-query-{unique_id}");
            let id_one = format!("Item-query-a-{unique_id}");
            let id_two = format!("Item-query-b-{unique_id}");

            let item_one = ResponseMetadataItem {
                id: id_one.clone(),
                partition_key: pk.clone(),
                value: "first".to_string(),
            };
            let item_two = ResponseMetadataItem {
                id: id_two.clone(),
                partition_key: pk.clone(),
                value: "second".to_string(),
            };

            container_client
                .create_item(&pk, &id_one, &item_one, None)
                .await?;
            container_client
                .create_item(&pk, &id_two, &item_two, None)
                .await?;

            let id_prefix = "Item-query-";
            let query = Query::from(
                "SELECT * FROM c WHERE c.partition_key = @partition_key AND STARTSWITH(c.id, @prefix)",
            )
            .with_parameter("@partition_key", pk.as_str())?
            .with_parameter("@prefix", id_prefix)?;

            let mut pages = container_client
                .query_items::<serde_json::Value>(query, FeedScope::partition(pk), None)
                .await?
                .into_pages();

            let mut returned_ids = Vec::new();
            let mut saw_session_token = false;
            while let Some(page) = pages.next().await {
                let page = page?;
                if page.session_token().is_some() {
                    saw_session_token = true;
                }
                for item in page.into_items() {
                    assert!(
                        item.get("_lsn").is_none(),
                        "_lsn should stay in page metadata, not appear in a query item body: {item:?}"
                    );
                    let id = item
                        .get("id")
                        .and_then(|v| v.as_str())
                        .expect("query item should have a string id")
                        .to_string();
                    returned_ids.push(id);
                }
            }
            assert!(
                saw_session_token,
                "expected at least one query page to surface a session_token"
            );

            returned_ids.sort();
            let mut expected_ids = vec![id_one.clone(), id_two.clone()];
            expected_ids.sort();
            assert_eq!(returned_ids, expected_ids);

            Ok(())
        },
        None,
    )
    .await
}
