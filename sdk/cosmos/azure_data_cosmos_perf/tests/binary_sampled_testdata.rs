// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live integration test that samples the bundled `testdata/*.json` corpus and
//! round-trips the sampled documents through Cosmos DB using **binary JSON**
//! encoding.
//!
//! The perf crate ships a large collection of representative JSON payloads in
//! `testdata/`. This test picks random documents out of that corpus, injects an
//! `id` and a `pk` (the container is partitioned on `/pk`), then creates each
//! document and reads it back — with the SDK's binary-encoding preview enabled —
//! asserting the fields we wrote survive the binary request/response round-trip.
//!
//! Binary encoding is enabled through the `AZURE_COSMOS_BINARY_ENCODING_ENABLED`
//! environment variable, which the SDK resolves **once at client-build time**.
//! This test therefore sets it before building the client.
//!
//! # Running
//!
//! Provide a live account connection string (which carries both the endpoint
//! and the account key) and select the `binary_encoding` test category:
//!
//! ```bash
//! AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
//!     RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!     cargo test -p azure_data_cosmos_perf --test binary_sampled_testdata
//! ```
//!
//! The test targets the `dkunda-be-db` database and `dkunda-be-ct` container
//! (partition key `/pk`), creating them if they do not already exist. To run
//! against a local emulator, also set `AZURE_COSMOS_ALLOW_INVALID_CERT=true`.

#![allow(clippy::large_futures)]

use std::error::Error;
use std::path::{Path, PathBuf};

use azure_core::http::StatusCode;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{
    ConnectionPoolOptions, ContentResponseOnWrite, ItemWriteOptions, OperationOptions, Region,
    ServerCertificateValidation,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use rand::RngExt;
use serde_json::{Map, Value};
use uuid::Uuid;

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const BINARY_ENV: &str = "AZURE_COSMOS_BINARY_ENCODING_ENABLED";
const ALLOW_INVALID_CERT_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";

const DATABASE_NAME: &str = "dkunda-be-db";
const CONTAINER_NAME: &str = "dkunda-be-ct";
const PARTITION_KEY_PATH: &str = "/pk";

/// Number of documents sampled from the corpus per test run.
const SAMPLE_COUNT: usize = 25;

/// Reads all bundled `testdata/*.json` files and flattens them into a pool of
/// candidate JSON **objects** (non-object top-level values and array elements
/// are skipped, since only objects can carry the injected `id`/`pk` fields).
fn load_sample_pool() -> Result<Vec<Map<String, Value>>, Box<dyn Error>> {
    let testdata_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata");
    let mut pool = Vec::new();

    for entry in std::fs::read_dir(&testdata_dir)? {
        let path = entry?.path();
        if path
            .extension()
            .and_then(|e| e.to_str())
            .map(str::to_ascii_lowercase)
            != Some("json".to_string())
        {
            continue;
        }
        collect_objects_from_file(&path, &mut pool);
    }

    if pool.is_empty() {
        return Err(format!(
            "no candidate JSON objects found under {}",
            testdata_dir.display()
        )
        .into());
    }
    Ok(pool)
}

/// Parses a single testdata file and appends any JSON objects it contains to
/// `pool`. A file may be a top-level object, an array of objects, or an object
/// whose values contain arrays of objects (e.g. GeoJSON `features`). Unparsable
/// files are skipped rather than failing the whole run.
fn collect_objects_from_file(path: &Path, pool: &mut Vec<Map<String, Value>>) {
    let Ok(bytes) = std::fs::read(path) else {
        return;
    };
    let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
        return;
    };
    collect_objects_from_value(value, pool);
}

/// Recursively harvests JSON objects from a value, descending into arrays and
/// into the array-valued fields of objects.
fn collect_objects_from_value(value: Value, pool: &mut Vec<Map<String, Value>>) {
    match value {
        Value::Array(items) => {
            for item in items {
                collect_objects_from_value(item, pool);
            }
        }
        Value::Object(map) => {
            // Descend into array-valued fields (e.g. GeoJSON `features`) so
            // container documents like `{ "features": [ {...}, {...} ] }`
            // contribute their inner objects too.
            for nested in map.values() {
                if let Value::Array(items) = nested {
                    for item in items {
                        if let Value::Object(obj) = item {
                            pool.push(obj.clone());
                        }
                    }
                }
            }
            pool.push(map);
        }
        _ => {}
    }
}

/// Builds a Cosmos client from the connection string, enabling binary encoding.
async fn build_client() -> Result<CosmosClient, Box<dyn Error>> {
    // Must be set before the client is built: the SDK resolves the binary
    // encoding flag once at build time.
    std::env::set_var(BINARY_ENV, "true");

    let connection_string = std::env::var(CONNECTION_STRING_ENV_VAR).map_err(|_| {
        format!("{CONNECTION_STRING_ENV_VAR} must be set to a Cosmos DB connection string")
    })?;
    let connection_string: ConnectionString = connection_string.parse()?;

    let endpoint: AccountEndpoint = connection_string.account_endpoint().parse()?;
    let account = AccountReference::with_authentication_key(
        endpoint,
        connection_string.account_key().clone(),
    );

    let mut builder = CosmosClient::builder();

    let allow_invalid_cert = std::env::var(ALLOW_INVALID_CERT_ENV_VAR)
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);
    if allow_invalid_cert {
        let runtime = CosmosRuntime::builder()
            .with_connection_pool(
                ConnectionPoolOptions::builder()
                    .with_server_certificate_validation(
                        ServerCertificateValidation::RequiredUnlessEmulator,
                    )
                    .build()?,
            )
            .build()
            .await?;
        builder = builder.with_runtime(runtime);
    }

    let client = builder
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;
    Ok(client)
}

/// Maps a 409 Conflict result to `Ok(())` so "resource already exists" is not
/// treated as a failure when ensuring the database/container exist.
fn ignore_conflict<T>(result: azure_data_cosmos::Result<T>) -> Result<(), Box<dyn Error>> {
    match result {
        Ok(_) => Ok(()),
        Err(e) if e.status().status_code() == StatusCode::Conflict => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Write options that request the service echo the stored document back, so the
/// binary **response** decode path is exercised on every write.
fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

/// Asserts that every field we sent in `sent` is present and equal in the
/// service-returned `got`. The service adds system fields (`_rid`, `_etag`,
/// `_ts`, ...) to stored documents, so a full-object equality would spuriously
/// fail — we only verify the fields we control round-tripped intact.
fn assert_sent_fields_round_tripped(sent: &Map<String, Value>, got: &Value, context: &str) {
    let got = got
        .as_object()
        .unwrap_or_else(|| panic!("{context}: response body was not a JSON object: {got}"));
    for (key, expected) in sent {
        let actual = got
            .get(key)
            .unwrap_or_else(|| panic!("{context}: response missing field {key:?}"));
        assert_eq!(
            actual, expected,
            "{context}: field {key:?} did not round-trip",
        );
    }
}

/// Samples documents from the bundled corpus and round-trips each one through
/// create + read using binary encoding, asserting the sent fields survive.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
async fn binary_round_trips_sampled_testdata() -> Result<(), Box<dyn Error>> {
    let pool = load_sample_pool()?;
    println!("Loaded {} candidate documents from testdata/", pool.len());

    let client = build_client().await?;

    // Ensure the target database and container (partitioned on /pk) exist,
    // treating a 409 Conflict as "already created".
    ignore_conflict(client.create_database(DATABASE_NAME, None).await)?;
    let db_client = client.database_client(DATABASE_NAME);
    ignore_conflict(
        db_client
            .create_container(
                ContainerProperties::new(CONTAINER_NAME.to_string(), PARTITION_KEY_PATH.into()),
                None,
            )
            .await,
    )?;
    let container = db_client.container_client(CONTAINER_NAME).await?;

    let mut rng = rand::rng();
    let sampled: Vec<Map<String, Value>> = (0..SAMPLE_COUNT)
        .map(|_| pool[rng.random_range(0..pool.len())].clone())
        .collect();

    for (i, base) in sampled.into_iter().enumerate() {
        // Inject the id and partition key. The corpus documents may already
        // carry an `id`/`pk`, which we overwrite to guarantee uniqueness and a
        // valid single-value partition key.
        let id = Uuid::new_v4().to_string();
        let pk = format!("pk-{}", rng.random_range(0..16));

        let mut doc = base;
        doc.insert("id".to_string(), Value::String(id.clone()));
        doc.insert("pk".to_string(), Value::String(pk.clone()));

        let context = format!("sample #{i} (id={id})");

        // CREATE with content response: the service echoes the stored document
        // back through the binary path.
        let created = container
            .create_item(&pk, &id, &doc, Some(write_options_with_content()))
            .await?;
        assert_eq!(created.status(), StatusCode::Created, "{context}: create");
        let created_doc: Value = created.into_model()?;
        assert_sent_fields_round_tripped(&doc, &created_doc, &format!("{context}: create echo"));

        // READ back through the binary path and verify the stored values.
        let read = container.read_item(&pk, &id, None).await?;
        assert_eq!(read.status(), StatusCode::Ok, "{context}: read");
        let read_doc: Value = read.into_model()?;
        assert_sent_fields_round_tripped(&doc, &read_doc, &format!("{context}: read"));
    }

    println!("Round-tripped {SAMPLE_COUNT} sampled documents through binary encoding.");
    Ok(())
}
