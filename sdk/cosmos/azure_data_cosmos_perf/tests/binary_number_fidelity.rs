// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live test that establishes **how a real Cosmos account spells numbers** and
//! whether the binary and text paths agree on it.
//!
//! The driver renders an integral `Double` (`3.0`) read off a binary payload as
//! an integer (`3`), on the premise that this is what text mode returns. No
//! other test observes that: the fuzzer compares two *binary* decoders, the
//! emulator re-serializes what the test wrote, and `binary_sampled_testdata`
//! reconciles number variants through `numbers_equivalent` and so tolerates
//! this difference by design.
//!
//! Reads the same document five ways — point read and query (passthrough and
//! `ORDER BY`, which take different driver paths) over both encodings — and
//! requires all five to agree with text mode. Comparison is strict
//! `serde_json::Value` equality: `Number`'s `PartialEq` is variant-sensitive, so
//! `PosInt(3) != Float(3.0)`, and any normalizing helper would erase the signal.
//! `-0.0` is **reported, not asserted**: the service folds its sign at storage,
//! so all paths return `0` and the driver's carve-out never applies to
//! service-originated data.
//!
//! # Running
//!
//! ```bash
//! AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
//!     RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!     cargo test -p azure_data_cosmos_perf --test binary_number_fidelity -- --nocapture
//! ```
//!
//! Pass `--nocapture`: the observation report is printed even when everything
//! passes, and is the point of the test.
//!
//! Targets `binary-encoding-perf-db` / `binary-encoding-perf-ct` (partition key
//! `/pk`) by default, creating them if absent. Override with
//! `AZURE_COSMOS_BINARY_TEST_DATABASE` / `AZURE_COSMOS_BINARY_TEST_CONTAINER`.
//! Set `AZURE_COSMOS_ALLOW_INVALID_CERT=true` to run against a local emulator.

#![allow(clippy::large_futures)]

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{
    BinaryEncodingOptions, ConnectionPoolOptions, ContentResponseOnWrite, ItemReadOptions,
    ItemWriteOptions, OperationOptions, QueryOptions, Region, ServerCertificateValidation,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, FeedScope, Query,
    RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::TryStreamExt;
use serde_json::{Map, Value};
use uuid::Uuid;

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERT_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const DATABASE_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_DATABASE";
const CONTAINER_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_CONTAINER";

const DEFAULT_DATABASE_NAME: &str = "binary-encoding-perf-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-encoding-perf-ct";
const PARTITION_KEY_PATH: &str = "/pk";

/// The encoding arm a read is performed under.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Encoding {
    /// Binary off: text JSON in both directions. The oracle.
    Text,
    /// Binary on, response left binary — the passthrough decode path.
    Binary,
}

impl Encoding {
    fn label(self) -> &'static str {
        match self {
            Encoding::Text => "text",
            Encoding::Binary => "binary",
        }
    }

    fn options(self) -> BinaryEncodingOptions {
        match self {
            Encoding::Text => BinaryEncodingOptions::new().with_enabled(false),
            Encoding::Binary => BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(false),
        }
    }
}

/// Names the `serde_json::Number` variant a value landed in, for the report.
///
/// The variant *is* the observation: it is what `PartialEq` discriminates on,
/// so naming it makes the report say the same thing the assertions check.
fn variant_of(value: &Value) -> String {
    match value {
        Value::Number(number) if number.is_u64() => format!("u64({number})"),
        Value::Number(number) if number.is_i64() => format!("i64({number})"),
        Value::Number(number) => format!("f64({number})"),
        other => format!("{other}"),
    }
}

/// Builds a client. Binary encoding is left **off** at the client level so every
/// read below must opt in explicitly and no arm inherits a default.
async fn build_client() -> Result<CosmosClient, Box<dyn Error>> {
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

    Ok(builder
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?)
}

/// Maps a 409 Conflict to `Ok(())` when ensuring the database/container exist.
fn ignore_conflict<T>(result: azure_data_cosmos::Result<T>) -> Result<(), Box<dyn Error>> {
    match result {
        Ok(_) => Ok(()),
        Err(e) if e.status().status_code() == StatusCode::Conflict => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Point-reads the document under one encoding arm.
async fn read_item_as(
    container: &ContainerClient,
    pk: &str,
    id: &str,
    encoding: Encoding,
) -> Result<Value, Box<dyn Error>> {
    let mut operation = OperationOptions::default();
    operation.binary_encoding = Some(encoding.options());
    let options = ItemReadOptions::default().with_operation_options(operation);

    let response = container
        .read_item(pk.to_string(), id, Some(options))
        .await?;
    assert_eq!(response.status(), StatusCode::Ok, "read_item status");
    Ok(response.into_model()?)
}

/// Drains a single-document query under one encoding arm.
async fn query_one(
    container: &ContainerClient,
    query_text: &str,
    run_id: &str,
    encoding: Encoding,
) -> Result<Value, Box<dyn Error>> {
    let query = Query::from(query_text).with_parameter("@run", run_id)?;

    let mut operation = OperationOptions::default();
    operation.binary_encoding = Some(encoding.options());
    let options = QueryOptions::default().with_operation_options(operation);

    let mut pages = container
        .query_items::<Value>(query, FeedScope::full_container(), Some(options))
        .await?
        .into_pages();

    let mut items = Vec::new();
    while let Some(page) = pages.try_next().await? {
        items.extend(page.into_items());
    }
    assert_eq!(
        items.len(),
        1,
        "query {query_text:?} under {} must match exactly the seeded document",
        encoding.label(),
    );
    Ok(items.into_iter().next().expect("length asserted above"))
}

/// Compares only the fields this test wrote, ignoring the system properties the
/// service adds (`_rid`, `_etag`, `_ts`, ...) and whose spelling is not at issue.
fn project(value: &Value, fields: &[&str]) -> Map<String, Value> {
    let object = value
        .as_object()
        .unwrap_or_else(|| panic!("response body was not a JSON object: {value}"));
    fields
        .iter()
        .map(|field| {
            let found = object
                .get(*field)
                .unwrap_or_else(|| panic!("response missing field {field:?}"));
            ((*field).to_string(), found.clone())
        })
        .collect()
}

/// Establishes the service's number spelling and asserts the binary paths match it.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
async fn binary_and_text_agree_on_number_spelling() -> Result<(), Box<dyn Error>> {
    /// Fields under test, in report order. `id`/`pk`/`testRun` are excluded:
    /// they are strings and carry no numeric signal.
    const FIELDS: &[&str] = &[
        "small",
        "negative",
        "fractional",
        "negativeZero",
        "wide",
        "large",
    ];

    let client = build_client().await?;

    let database_name =
        std::env::var(DATABASE_NAME_ENV_VAR).unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
    let container_name = std::env::var(CONTAINER_NAME_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_CONTAINER_NAME.to_string());

    ignore_conflict(client.create_database(&database_name, None).await)?;
    let db_client = client.database_client(&database_name);
    ignore_conflict(
        db_client
            .create_container(
                ContainerProperties::new(container_name.clone(), PARTITION_KEY_PATH.into()),
                None,
            )
            .await,
    )?;
    let container = db_client.container_client(&container_name).await?;

    let run_id = Uuid::new_v4().to_string();
    let id = Uuid::new_v4().to_string();
    let pk = format!("pk-{run_id}");

    // Every literal is written as a JSON *float*, so the service stores each as
    // a `Double` — the case in question. `wide` is past 2^53, where doubles stop
    // representing consecutive integers; `large` is beyond `u64::MAX`, where the
    // coercion rule declines and the value must stay floating point everywhere.
    let document = serde_json::json!({
        "id": id,
        "pk": pk,
        "testRun": run_id,
        "small": 3.0,
        "negative": -7.0,
        "fractional": 2.5,
        "negativeZero": -0.0,
        "wide": 9_007_199_254_740_993_f64,
        "large": 1.0e20_f64,
    });

    // Seed over **text**, so the document reaching the service is not shaped by
    // the binary encoder — the write path is not what is being measured.
    let mut write_operation = OperationOptions::default();
    write_operation.binary_encoding = Some(Encoding::Text.options());
    write_operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    let write_options = ItemWriteOptions::default().with_operation_options(write_operation);

    let created = container
        .create_item(&pk, &id, &document, Some(write_options))
        .await?;
    assert_eq!(created.status(), StatusCode::Created, "seed create");

    // ---- Observation 1: what does text mode actually return? ----
    let text_read = read_item_as(&container, &pk, &id, Encoding::Text).await?;

    println!("\n=== Service number spelling, as read back in TEXT mode ===");
    println!("(this is the ground truth the binary coercion rule is built on)");
    for field in FIELDS {
        println!(
            "  {field:<13} sent {:<28} read back as {}",
            variant_of(&document[*field]),
            variant_of(&text_read[*field]),
        );
    }

    // ---- Observation 2: the other three read paths ----
    let binary_read = read_item_as(&container, &pk, &id, Encoding::Binary).await?;

    // A passthrough query deserializes the binary page directly; `ORDER BY`
    // transcodes it to text first. Different code paths, same required result.
    let passthrough = "SELECT * FROM c WHERE c.testRun = @run";
    let ordered = "SELECT * FROM c WHERE c.testRun = @run ORDER BY c.id";

    let text_passthrough = query_one(&container, passthrough, &run_id, Encoding::Text).await?;
    let binary_passthrough = query_one(&container, passthrough, &run_id, Encoding::Binary).await?;
    let binary_ordered = query_one(&container, ordered, &run_id, Encoding::Binary).await?;

    let observed = [
        ("point read / text", &text_read),
        ("point read / binary", &binary_read),
        ("query passthrough / text", &text_passthrough),
        ("query passthrough / binary", &binary_passthrough),
        ("query ORDER BY / binary", &binary_ordered),
    ];

    println!("\n=== Per-path number variants ===");
    for field in FIELDS {
        println!("  {field}:");
        for (label, value) in &observed {
            println!("    {label:<28} {}", variant_of(&value[*field]));
        }
    }

    // `-0.0` is reported rather than asserted. Observed: the service folds the
    // sign at storage and returns `0` on every path, so the carve-out in
    // `integral_double` only ever applies to client-side encoding.
    println!(
        "\nnegativeZero: sent f64(-0.0), text mode returned {} \
         (sign preserved: {})",
        variant_of(&text_read["negativeZero"]),
        text_read["negativeZero"]
            .as_f64()
            .is_some_and(|f| f.is_sign_negative()),
    );
    println!();

    // Strict `Value` equality on the fields written. `Number`'s `PartialEq` is
    // variant-sensitive, which is the property being tested — a normalizing
    // comparison would pass regardless and prove nothing.
    let expected = project(&text_read, FIELDS);
    for (label, value) in &observed {
        assert_eq!(
            project(value, FIELDS),
            expected,
            "{label} disagreed with text mode on number spelling",
        );
    }

    Ok(())
}
