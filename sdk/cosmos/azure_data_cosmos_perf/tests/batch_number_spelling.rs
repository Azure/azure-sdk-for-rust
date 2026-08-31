// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live probe that establishes **how a real Cosmos account spells the numbers
//! it authors itself** in a transactional batch response envelope.
//!
//! `binary_number_fidelity` established the service's spelling for *stored
//! document* values: a stored `3.0` reads back as `3` in text mode. The
//! in-memory emulator reproduces that. But a batch response envelope also
//! carries numbers the **service** authored — `requestCharge`, `statusCode`,
//! `subStatusCode` — and no measurement covers those. Extending the stored-value
//! result to them is an inference about the service's JSON writer, not an
//! observation.
//!
//! This matters because the emulator must choose. Normalizing the whole envelope
//! rewrites `requestCharge: 1.0` to `1`; normalizing nothing leaves a stored
//! `resourceBody` unnormalized. Only one of those matches the service, and the
//! existing live-comparison harness cannot say which: `dtx_live_comparison`
//! replaces `requestCharge` with the string `"<non-negative>"` before diffing,
//! discarding the spelling.
//!
//! So this prints the **raw response bytes, before deserialization**, and the
//! `serde_json` number variant each envelope field parses into. It asserts
//! nothing about `requestCharge` — the point is to find out, not to encode a
//! guess.
//!
//! # Running
//!
//! ```bash
//! AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
//!     RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!     cargo test -p azure_data_cosmos_perf --test batch_number_spelling -- --nocapture
//! ```
//!
//! Pass `--nocapture`: the report is the entire purpose of the test.
//!
//! Targets `binary-encoding-perf-db` / `binary-encoding-perf-ct` (partition key
//! `/pk`) by default, creating them if absent. Override with
//! `AZURE_COSMOS_BINARY_TEST_DATABASE` / `AZURE_COSMOS_BINARY_TEST_CONTAINER`.
//! Set `AZURE_COSMOS_ALLOW_INVALID_CERT=true` to run against a local emulator.

#![allow(clippy::large_futures)]

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::models::{ContainerProperties, TransactionalBatch};
use azure_data_cosmos::options::{ConnectionPoolOptions, Region, ServerCertificateValidation};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use serde_json::Value;
use uuid::Uuid;

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERT_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const DATABASE_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_DATABASE";
const CONTAINER_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_CONTAINER";

const DEFAULT_DATABASE_NAME: &str = "binary-encoding-perf-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-encoding-perf-ct";
const PARTITION_KEY_PATH: &str = "/pk";

/// Names the `serde_json::Number` variant a value landed in.
///
/// The variant is the observation: `1` parses as `u64`, `1.0` as `f64`, and
/// `Number`'s `PartialEq` discriminates between them.
fn variant_of(value: &Value) -> String {
    match value {
        Value::Number(number) if number.is_u64() => format!("u64({number})"),
        Value::Number(number) if number.is_i64() => format!("i64({number})"),
        Value::Number(number) => format!("f64({number})"),
        other => format!("{other}"),
    }
}

/// Builds a client. Binary encoding is left off: the batch path is text-only,
/// and the question is about the service's text rendering.
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

/// Reports the raw spelling of every number the service authored in a batch
/// response envelope.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
async fn batch_envelope_number_spelling() -> Result<(), Box<dyn Error>> {
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
    let pk = format!("batch-pk-{run_id}");

    // Two operations, so the envelope carries more than one `requestCharge` and
    // a per-operation charge is visible alongside any aggregate. `stored` is
    // written as a JSON float: if the echoed `resourceBody` comes back as `4`,
    // the stored-value normalization the emulator applies is confirmed on this
    // path too.
    let first = serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "pk": pk,
        "stored": 4.0,
    });
    let second = serde_json::json!({
        "id": Uuid::new_v4().to_string(),
        "pk": pk,
        "stored": 2.5,
    });

    let batch = TransactionalBatch::new(pk.clone())
        .create_item(first)?
        .create_item(second)?;

    let response = container.execute_transactional_batch(batch, None).await?;
    let status = response.status();

    // The raw buffers, before any deserialization. This is the measurement:
    // `into_model()` would parse the numbers and erase their spelling.
    let buffers = response.into_body().items()?;

    println!("\n=== Batch response, status {status:?} ===");
    println!("(raw bytes, before deserialization)");
    for (index, buffer) in buffers.iter().enumerate() {
        match std::str::from_utf8(buffer) {
            Ok(text) => println!("  [{index}] {text}"),
            Err(_) => println!("  [{index}] <{} non-UTF-8 bytes>", buffer.len()),
        }
    }

    // Re-parse and name the variant of every number, so the report says the
    // same thing an equality check would see.
    println!("\n=== Number variants, per envelope field ===");
    for (index, buffer) in buffers.iter().enumerate() {
        let parsed: Value = match serde_json::from_slice(buffer) {
            Ok(parsed) => parsed,
            Err(error) => {
                println!("  [{index}] did not parse as JSON: {error}");
                continue;
            }
        };
        report_numbers(&format!("[{index}]"), &parsed);
    }

    println!(
        "\nThe question this answers: does `requestCharge` arrive as `1` or \
         `1.0` when the charge is integral? If the account never produces an \
         integral charge, the emulator's choice is moot and it should stub a \
         fractional one."
    );

    Ok(())
}

/// Walks a value, printing the variant of every number under its JSON path.
fn report_numbers(path: &str, value: &Value) {
    match value {
        Value::Number(_) => println!("  {path:<44} {}", variant_of(value)),
        Value::Array(values) => {
            for (index, element) in values.iter().enumerate() {
                report_numbers(&format!("{path}[{index}]"), element);
            }
        }
        Value::Object(values) => {
            for (key, element) in values {
                report_numbers(&format!("{path}.{key}"), element);
            }
        }
        _ => {}
    }
}
