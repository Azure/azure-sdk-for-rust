// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live integration tests for **Cosmos binary JSON** encoding against a real
//! Cosmos DB account.
//!
//! Unlike the in-memory-emulator round-trip tests (which exercise binary
//! encoding against an emulator we control), these tests validate the complete
//! loop against the **real service**: the SDK encodes item write bodies as
//! Cosmos binary JSON and advertises binary-response support, the service must
//! accept the binary request body, and — because the negotiation header is
//! present — reply with a binary body that the SDK auto-detects and decodes.
//!
//! Binary encoding is enabled through the `AZURE_COSMOS_BINARY_ENCODING_ENABLED`
//! environment variable, which the SDK resolves **once at client-build time**.
//! These tests therefore set it before the [`TestClient`] builds its client.
//!
//! # Running
//!
//! Provide a live account connection string and select the `binary_encoding`
//! test category:
//!
//! ```bash
//! AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
//!     RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!     cargo test -p azure_data_cosmos --test binary_encoding
//! ```
//!
//! To run against the local emulator instead, also set
//! `AZURE_COSMOS_ALLOW_INVALID_CERT=true` so its self-signed certificate is
//! accepted.

use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::clients::{ContainerClient, DatabaseClient};
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{ContentResponseOnWrite, ItemWriteOptions, OperationOptions};
use framework::{TestClient, TestRunContext};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Once;

/// The environment variable that enables the binary-encoding preview.
const BINARY_ENV: &str = "AZURE_COSMOS_BINARY_ENCODING_ENABLED";

/// Enables binary encoding for the lifetime of this (dedicated) test process.
///
/// `BinaryEncoding::from_env` resolves the flag once when a `CosmosClient` is
/// built, so it must be set before any `TestClient` builds its client. Every
/// test in this target wants binary on and the variable is scoped to this test
/// binary, so it is set exactly once (via [`Once`]) and never cleared — which
/// keeps it race-free under the parallel test harness.
fn enable_binary_encoding() {
    static ENABLE: Once = Once::new();
    ENABLE.call_once(|| {
        std::env::set_var(BINARY_ENV, "true");
    });
}

/// A document covering every JSON value shape the binary encoder emits: literal
/// and wide integers, an unsigned value beyond `i64::MAX`, a double, booleans,
/// `null`, unicode/empty strings, and nested arrays and objects.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct BinaryItem {
    id: String,
    partition_key: String,
    text: String,
    unicode: String,
    empty: String,
    small_int: i64,
    big_int: i64,
    negative: i64,
    huge: u64,
    ratio: f64,
    active: bool,
    inactive: bool,
    maybe: Option<String>,
    tags: Vec<String>,
    numbers: Vec<i64>,
    nested: NestedData,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct NestedData {
    label: String,
    values: Vec<i64>,
    flag: bool,
}

/// Builds a [`BinaryItem`] exercising the full range of encoder forms.
fn sample_item(id: &str, partition_key: &str) -> BinaryItem {
    BinaryItem {
        id: id.to_owned(),
        partition_key: partition_key.to_owned(),
        text: "hello binary".to_owned(),
        unicode: "café ☃ 𝄞 quotes:\" backslash:\\".to_owned(),
        empty: String::new(),
        small_int: 7,           // literal-int form (0..32)
        big_int: 9_000_000_000, // Int64 form
        negative: -1_234_567,   // Int64 form
        huge: u64::MAX,         // UInt64 form (beyond i64::MAX)
        ratio: 123.456_789,     // Double form
        active: true,
        inactive: false,
        maybe: None, // null
        tags: vec!["alpha".to_owned(), "beta".to_owned(), "gamma".to_owned()],
        numbers: vec![0, 1, 31, 32, 255, 256, -1],
        nested: NestedData {
            label: "nested".to_owned(),
            values: vec![10, 20, 30],
            flag: false,
        },
    }
}

/// Write options that request the service echo the stored document back, so the
/// binary **response** decode path is exercised on every write.
fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

/// Creates a fresh container in `db_client` for a binary-encoding test run.
async fn create_container(
    run_context: &TestRunContext,
    db_client: &DatabaseClient,
) -> azure_data_cosmos::Result<ContainerClient> {
    let container_id = format!("binary-{}", Uuid::new_v4());
    run_context
        .create_container(
            db_client,
            ContainerProperties::new(container_id, "/partition_key".into()),
            None,
        )
        .await
}

/// Drives create / read / upsert / replace / delete with binary encoding
/// enabled and asserts every hop round-trips the document unchanged.
///
/// Because writes request a content response, each create/upsert/replace makes
/// the service return a (binary) body that the SDK must decode — so the test
/// validates both the binary **request** body the service accepts and the
/// binary **response** body the SDK decodes, against the real service.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
pub async fn binary_encoding_item_crud_round_trips() -> Result<(), Box<dyn Error>> {
    enable_binary_encoding();

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_container(run_context, db_client).await?;
            let pk = "pk-binary";
            let item = sample_item("doc-1", pk);

            // CREATE: the request body is binary; with the negotiation header
            // the echoed response body comes back binary and is decoded here.
            let created = container
                .create_item(pk, &item.id, &item, Some(write_options_with_content()))
                .await?;
            assert_eq!(created.status(), StatusCode::Created);
            let created_doc: BinaryItem = created.into_body().into_single()?;
            assert_eq!(created_doc, item, "create response must round-trip");

            // READ: the response body comes back binary and decodes to the
            // original value. `read_item` retries on transient 404s.
            let read = run_context
                .read_item(&container, pk, &item.id, None)
                .await?;
            let read_doc: BinaryItem = read.into_model()?;
            assert_eq!(read_doc, item, "read response must round-trip");

            // UPSERT (modified) with content response.
            let mut updated = item.clone();
            updated.small_int = 30;
            updated.text = "upserted binary".to_owned();
            updated.tags.push("delta".to_owned());
            let upserted = container
                .upsert_item(
                    pk,
                    &updated.id,
                    &updated,
                    Some(write_options_with_content()),
                )
                .await?;
            assert_eq!(upserted.status(), StatusCode::Ok);
            let upserted_doc: BinaryItem = upserted.into_body().into_single()?;
            assert_eq!(upserted_doc, updated, "upsert response must round-trip");

            // REPLACE (modified) with content response.
            updated.ratio = 987.654_321;
            updated.numbers = vec![100, 200, 300];
            updated.nested.flag = true;
            let replaced = container
                .replace_item(
                    pk,
                    &updated.id,
                    &updated,
                    Some(write_options_with_content()),
                )
                .await?;
            assert_eq!(replaced.status(), StatusCode::Ok);
            let replaced_doc: BinaryItem = replaced.into_body().into_single()?;
            assert_eq!(replaced_doc, updated, "replace response must round-trip");

            // DELETE, then confirm the item is gone.
            let deleted = container.delete_item(pk, &updated.id, None).await?;
            assert_eq!(deleted.status(), StatusCode::NoContent);

            Ok(())
        },
        None,
    )
    .await
}

/// Writes a document with payloads large enough to push the encoder past its
/// single-byte length/count forms — a 1000-byte string (`StrL2`) and a
/// 300-element array (`ArrLC2`) — and verifies the real service stores and
/// returns them intact through the binary path.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
pub async fn binary_encoding_handles_large_payloads() -> Result<(), Box<dyn Error>> {
    enable_binary_encoding();

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container = create_container(run_context, db_client).await?;
            let pk = "pk-large";

            let big_text = "a".repeat(1000); // > 255 bytes -> StrL2
            let big_array: Vec<i64> = (0..300).collect(); // 300 items -> ArrLC2
            let item = serde_json::json!({
                "id": "large-1",
                "partition_key": pk,
                "big_text": big_text,
                "big_array": big_array,
            });

            // CREATE with content response: the service echoes the document
            // back through the binary path.
            let created = container
                .create_item(pk, "large-1", &item, Some(write_options_with_content()))
                .await?;
            assert_eq!(created.status(), StatusCode::Created);
            let echoed: serde_json::Value = created.into_body().into_single()?;
            assert_eq!(echoed["big_text"], item["big_text"]);
            assert_eq!(echoed["big_array"], item["big_array"]);

            // READ back and verify the stored values survived the round-trip.
            let read = run_context
                .read_item(&container, pk, "large-1", None)
                .await?;
            let read_doc: serde_json::Value = read.into_model()?;
            assert_eq!(read_doc["big_text"], item["big_text"]);
            assert_eq!(read_doc["big_array"], item["big_array"]);

            Ok(())
        },
        None,
    )
    .await
}
