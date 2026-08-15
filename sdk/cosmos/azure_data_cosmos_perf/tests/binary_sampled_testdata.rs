// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live integration test that seeds the bundled `testdata/*.json` corpus into a
//! container and then replays one query set **twice** — once over text JSON and
//! once over **binary JSON** — to compare the two encodings apple-to-apple.
//!
//! The perf crate ships a large collection of representative JSON payloads in
//! `testdata/`. The test runs in two phases:
//!
//! 1. **Seed.** Sample documents out of the corpus, stamp each with a unique
//!    `id`, a `pk` (the container is partitioned on `/pk`) and a per-run
//!    `testRun` marker, then insert them all with binary encoding and a content
//!    response, asserting both the echoed document and a subsequent point read
//!    match what was sent. The `testRun` marker scopes every later query to this
//!    run, so the container can be reused across runs without result counts
//!    drifting.
//! 2. **Compare.** Run each query shape twice over the seeded set — once with
//!    binary encoding off, once with it on — assert the two agree, and report
//!    item count, page count and RU charge side by side.
//!
//! Both arms use a **single client** and toggle encoding per operation via
//! [`OperationOptions::binary_encoding`], so the two runs share a connection
//! pool, session and routing state. Only the wire encoding differs.
//!
//! The query shapes cover the three cross-partition pipelines the binary work
//! touches: the unordered passthrough drain, the streaming `ORDER BY` k-way
//! merge (which decodes and re-encodes every item), and the `SkipTake` window
//! behind `OFFSET`/`LIMIT`/`TOP`.
//!
//! # What the comparison does and does not prove
//!
//! Agreement between the arms proves the binary pipeline returns the same data
//! as the text pipeline. It does **not** prove the binary arm actually put
//! binary on the wire — that is asserted at the byte level by the driver and
//! in-memory-emulator tests, which can inspect the request. Treat the reported
//! page/RU numbers as observations, not assertions: page boundaries are chosen
//! by the service, so they are reported rather than asserted on.
//!
//! # Test data dependency
//!
//! This test reads the `testdata/*.json` corpus **at runtime** (via
//! [`load_sample_pool`], `std::fs::read_dir`), so the files must be present on
//! disk under `azure_data_cosmos_perf/testdata/` when the test runs. That corpus
//! (~500 MB) is intentionally **not tracked in the source repo** to keep the
//! repository small; it is kept as a local copy. If the directory is missing or
//! empty, [`load_sample_pool`] returns an error telling you to restore it.
//!
//! Nothing in the build or the CI gates depends on this corpus: the test is
//! gated behind `test_category = "binary_encoding"` (ignored otherwise) and
//! requires a live account, and the benchmarks generate their own synthetic
//! data — so removing the files from source control does not affect compilation
//! or any gate.
//!
//! Binary encoding is enabled explicitly on the client via
//! `CosmosClientBuilder::with_binary_encoding_options`, which the SDK resolves
//! **once at client-build time**. This test therefore sets it when building the
//! client.
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
//! The test targets the `binary-encoding-perf-db` database and
//! `binary-encoding-perf-ct` container (partition key `/pk`) by default,
//! creating them if they do not already exist. Override the names with
//! `AZURE_COSMOS_BINARY_TEST_DATABASE` / `AZURE_COSMOS_BINARY_TEST_CONTAINER`,
//! and the number of seeded documents with `AZURE_COSMOS_BINARY_TEST_SEED_COUNT`
//! (default 200). Seeding more documents makes multi-page results more likely,
//! which is what exercises binary pagination. To run against a local emulator,
//! also set `AZURE_COSMOS_ALLOW_INVALID_CERT=true`.

#![allow(clippy::large_futures)]

use std::error::Error;
use std::path::{Path, PathBuf};

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{
    BinaryEncodingOptions, ConnectionPoolOptions, ContentResponseOnWrite, ItemWriteOptions,
    OperationOptions, QueryOptions, Region, ServerCertificateValidation,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, FeedScope, Query,
    RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::TryStreamExt;
use rand::RngExt;
use serde_json::{Map, Value};
use uuid::Uuid;

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERT_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const DATABASE_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_DATABASE";
const CONTAINER_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_CONTAINER";
const SEED_COUNT_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_SEED_COUNT";

const DEFAULT_DATABASE_NAME: &str = "binary-encoding-perf-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-encoding-perf-ct";
const PARTITION_KEY_PATH: &str = "/pk";

/// Number of corpus documents seeded per test run.
const DEFAULT_SEED_COUNT: usize = 200;

/// Upper bound on a sampled document's serialized size. Cosmos rejects items
/// over 2 MB, and the service adds system properties on top of what we send, so
/// oversized corpus objects are skipped rather than failing the seed with a 413.
const MAX_DOCUMENT_BYTES: usize = 1_500_000;

/// Attempts to draw an acceptably-sized document before giving up, so a corpus
/// made entirely of oversized objects fails loudly instead of looping.
const MAX_SAMPLE_ATTEMPTS: usize = 50;

/// Number of documents seeded this run, from [`SEED_COUNT_ENV_VAR`].
fn seed_count() -> usize {
    std::env::var(SEED_COUNT_ENV_VAR)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_SEED_COUNT)
}

/// Reads all bundled `testdata/*.json` files and flattens them into a pool of
/// candidate JSON **objects** (non-object top-level values and array elements
/// are skipped, since only objects can carry the injected `id`/`pk` fields).
///
/// The `testdata/` corpus is a local copy that is intentionally not tracked in
/// the source repo (see the module docs). If the directory is missing or holds
/// no usable objects, this returns an error explaining how to restore it.
fn load_sample_pool() -> Result<Vec<Map<String, Value>>, Box<dyn Error>> {
    let testdata_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata");

    let entries = std::fs::read_dir(&testdata_dir).map_err(|e| {
        format!(
            "failed to read the test-data corpus under {} ({e}). This corpus is a \
             local copy (not tracked in source control); restore the \
             `testdata/*.json` files before running this test.",
            testdata_dir.display()
        )
    })?;

    let mut pool = Vec::new();
    for entry in entries {
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
            "no candidate JSON objects found under {} — the local test-data corpus \
             is missing or empty; restore the `testdata/*.json` files.",
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
    let connection_string = std::env::var(CONNECTION_STRING_ENV_VAR).map_err(|_| {
        format!("{CONNECTION_STRING_ENV_VAR} must be set to a Cosmos DB connection string")
    })?;
    let connection_string: ConnectionString = connection_string.parse()?;

    let endpoint: AccountEndpoint = connection_string.account_endpoint().parse()?;
    let account = AccountReference::with_authentication_key(
        endpoint,
        connection_string.account_key().clone(),
    );

    // Enable binary encoding explicitly via the client option rather than the
    // process environment, so the setting is scoped to this client.
    let mut builder = CosmosClient::builder()
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(true));

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
///
/// Comparison is numerically tolerant (see [`json_equivalent`]) because Cosmos
/// stores every number as a double, so a wide integer we sent can legitimately
/// come back as the same value in a different `serde_json::Number` variant.
fn assert_sent_fields_round_tripped(sent: &Map<String, Value>, got: &Value, context: &str) {
    let got = got
        .as_object()
        .unwrap_or_else(|| panic!("{context}: response body was not a JSON object: {got}"));
    for (key, expected) in sent {
        let actual = got
            .get(key)
            .unwrap_or_else(|| panic!("{context}: response missing field {key:?}"));
        assert!(
            json_equivalent(actual, expected),
            "{context}: field {key:?} did not round-trip\n  sent: {}\n  got:  {}",
            render_capped(expected),
            render_capped(actual),
        );
    }
}

/// How a query shape's results may be compared across the two encodings.
#[derive(Clone, Copy)]
enum Compare {
    /// The query pins a total order, so the runs must agree element by element.
    Sequence,
    /// Order is unspecified across partitions, so compare as sets keyed by `id`.
    Set,
    /// The shape does not pin *which* documents come back (`TOP` without
    /// `ORDER BY` may return any window), so only cardinality is comparable.
    Count,
}

/// One query shape, run once per encoding.
struct QuerySpec {
    name: &'static str,
    text: &'static str,
    compare: Compare,
    expected_items: usize,
}

/// A drained query, with the paging and RU facts used to compare encodings.
struct QueryOutcome {
    items: Vec<Value>,
    pages: usize,
    request_charge: f64,
}

/// Drains `spec` across the whole container with binary encoding forced on or
/// off for this operation only.
///
/// The encoding is set explicitly on both arms so neither silently inherits the
/// client-level default, which would make the comparison meaningless.
async fn run_query(
    container: &ContainerClient,
    spec: &QuerySpec,
    run_id: &str,
    binary: bool,
) -> Result<QueryOutcome, Box<dyn Error>> {
    let query = Query::from(spec.text).with_parameter("@run", run_id)?;

    let mut operation = OperationOptions::default();
    operation.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(binary));
    let options = QueryOptions::default().with_operation_options(operation);

    let mut pages = container
        .query_items::<Value>(query, FeedScope::full_container(), Some(options))
        .await?
        .into_pages();

    let mut outcome = QueryOutcome {
        items: Vec::new(),
        pages: 0,
        request_charge: 0.0,
    };
    while let Some(page) = pages.try_next().await? {
        outcome.pages += 1;
        outcome.request_charge += page.headers().request_charge().map_or(0.0, |c| c.value());
        outcome.items.extend(page.into_items());
    }
    Ok(outcome)
}

/// Compares two JSON values treating numerically-equal numbers as equal.
///
/// Cosmos stores every number as an IEEE-754 double, so a wide integer can come
/// back as an integer literal in text but as a `Double` in binary. Those decode
/// to different `serde_json::Number` variants that compare unequal despite
/// being the same value, so numbers are compared as `f64`.
fn json_equivalent(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_f64(), y.as_f64()) {
            (Some(x), Some(y)) => x == y,
            _ => x == y,
        },
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| json_equivalent(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|other| json_equivalent(v, other)))
        }
        _ => a == b,
    }
}

fn id_of(value: &Value) -> &str {
    value.get("id").and_then(Value::as_str).unwrap_or("<no id>")
}

fn sorted_by_id(items: &[Value]) -> Vec<&Value> {
    let mut refs: Vec<&Value> = items.iter().collect();
    refs.sort_by(|a, b| id_of(a).cmp(id_of(b)));
    refs
}

/// Renders a value for an assertion message, capped so a multi-MB corpus
/// document does not flood the log.
fn render_capped(value: &Value) -> String {
    let mut rendered = value.to_string();
    if let Some((idx, _)) = rendered.char_indices().nth(512) {
        rendered.truncate(idx);
        rendered.push_str("... (truncated)");
    }
    rendered
}

/// Asserts the text and binary runs of `spec` returned equivalent results.
fn assert_encodings_agree(spec: &QuerySpec, text: &QueryOutcome, binary: &QueryOutcome) {
    assert_eq!(
        text.items.len(),
        binary.items.len(),
        "{}: text returned {} items but binary returned {}",
        spec.name,
        text.items.len(),
        binary.items.len(),
    );

    let (text_items, binary_items) = match spec.compare {
        Compare::Count => return,
        Compare::Sequence => (
            text.items.iter().collect::<Vec<_>>(),
            binary.items.iter().collect::<Vec<_>>(),
        ),
        Compare::Set => (sorted_by_id(&text.items), sorted_by_id(&binary.items)),
    };

    for (i, (t, b)) in text_items.iter().zip(&binary_items).enumerate() {
        assert!(
            json_equivalent(t, b),
            "{}: item #{i} (id={}) differs between encodings\n  text:   {}\n  binary: {}",
            spec.name,
            id_of(t),
            render_capped(t),
            render_capped(b),
        );
    }
}

/// Draws a corpus document small enough to store, retrying past oversized ones.
///
/// Top-level `_`-prefixed keys are dropped: much of the corpus was exported
/// from Cosmos and still carries system properties (`_rid`, `_etag`, `_ts`,
/// ...) that the service owns and rewrites on write, so they can never
/// round-trip.
fn sample_document(
    pool: &[Map<String, Value>],
    rng: &mut impl rand::Rng,
) -> Result<Map<String, Value>, Box<dyn Error>> {
    for _ in 0..MAX_SAMPLE_ATTEMPTS {
        let candidate: Map<String, Value> = pool[rng.random_range(0..pool.len())]
            .iter()
            .filter(|(k, _)| !k.starts_with('_'))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let size = serde_json::to_vec(&candidate).map(|v| v.len()).unwrap_or(0);
        if size > 0 && size <= MAX_DOCUMENT_BYTES {
            return Ok(candidate);
        }
    }
    Err(format!(
        "no corpus document under {MAX_DOCUMENT_BYTES} bytes found in {MAX_SAMPLE_ATTEMPTS} attempts"
    )
    .into())
}

/// Seeds sampled corpus documents, then replays each query shape over both
/// encodings and asserts they agree.
#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
async fn binary_and_text_queries_agree_over_seeded_corpus() -> Result<(), Box<dyn Error>> {
    let pool = load_sample_pool()?;
    let seed_count = seed_count();
    println!(
        "Loaded {} candidate documents from testdata/; seeding {seed_count}.",
        pool.len(),
    );

    let client = build_client().await?;

    // Target database/container names default to generic values but can be
    // overridden via env vars so the test can run against caller-owned
    // resources.
    let database_name =
        std::env::var(DATABASE_NAME_ENV_VAR).unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
    let container_name = std::env::var(CONTAINER_NAME_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_CONTAINER_NAME.to_string());

    // Ensure the target database and container (partitioned on /pk) exist,
    // treating a 409 Conflict as "already created".
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

    // Scopes every query below to this run, so a reused container does not let
    // earlier runs' documents perturb the expected counts.
    let run_id = Uuid::new_v4().to_string();
    let mut rng = rand::rng();

    // ---- Phase 1: seed every document up front ----
    for i in 0..seed_count {
        let id = Uuid::new_v4().to_string();
        let pk = format!("pk-{}", rng.random_range(0..16));

        // Corpus documents may already carry `id`/`pk`, which we overwrite to
        // guarantee uniqueness and a valid single-value partition key.
        let mut doc = sample_document(&pool, &mut rng)?;
        doc.insert("id".to_string(), Value::String(id.clone()));
        doc.insert("pk".to_string(), Value::String(pk.clone()));
        doc.insert("testRun".to_string(), Value::String(run_id.clone()));

        let context = format!("seed #{i} (id={id})");

        // Content response echoes the stored document back, so the binary
        // response decode path is exercised on every write.
        let created = container
            .create_item(&pk, &id, &doc, Some(write_options_with_content()))
            .await?;
        assert_eq!(created.status(), StatusCode::Created, "{context}: create");
        let created_doc: Value = created.into_model()?;
        assert_sent_fields_round_tripped(&doc, &created_doc, &format!("{context}: create echo"));

        // Read back through the binary path, which decodes a stored document
        // rather than the service's echo of the one just sent.
        let read = container.read_item(&pk, &id, None).await?;
        assert_eq!(read.status(), StatusCode::Ok, "{context}: read");
        let read_doc: Value = read.into_model()?;
        assert_sent_fields_round_tripped(&doc, &read_doc, &format!("{context}: read"));
    }
    println!("Seeded {seed_count} documents (testRun={run_id}).");

    // ---- Phase 2: replay each shape over both encodings ----
    // `OFFSET`/`LIMIT` and `TOP` bounds are fixed, so the expected counts are
    // clamped against the seeded total.
    const OFFSET: usize = 5;
    const LIMIT: usize = 25;
    const TOP: usize = 10;

    let specs = [
        QuerySpec {
            name: "passthrough (unordered fan-out)",
            text: "SELECT * FROM c WHERE c.testRun = @run",
            compare: Compare::Set,
            expected_items: seed_count,
        },
        QuerySpec {
            name: "ORDER BY (streaming merge)",
            text: "SELECT * FROM c WHERE c.testRun = @run ORDER BY c.id",
            compare: Compare::Sequence,
            expected_items: seed_count,
        },
        QuerySpec {
            name: "ORDER BY + OFFSET/LIMIT (SkipTake)",
            text: "SELECT * FROM c WHERE c.testRun = @run ORDER BY c.id OFFSET 5 LIMIT 25",
            compare: Compare::Sequence,
            expected_items: LIMIT.min(seed_count.saturating_sub(OFFSET)),
        },
        QuerySpec {
            name: "TOP (SkipTake over passthrough)",
            text: "SELECT TOP 10 * FROM c WHERE c.testRun = @run",
            compare: Compare::Count,
            expected_items: TOP.min(seed_count),
        },
    ];

    println!(
        "\n{:<38} {:>8} {:>7} {:>7} {:>10} {:>10}",
        "query", "encoding", "items", "pages", "RU", "RU/item",
    );

    for spec in &specs {
        let text = run_query(&container, spec, &run_id, false).await?;
        let binary = run_query(&container, spec, &run_id, true).await?;

        for (label, outcome) in [("text", &text), ("binary", &binary)] {
            let per_item = if outcome.items.is_empty() {
                0.0
            } else {
                outcome.request_charge / outcome.items.len() as f64
            };
            println!(
                "{:<38} {:>8} {:>7} {:>7} {:>10.2} {:>10.3}",
                spec.name,
                label,
                outcome.items.len(),
                outcome.pages,
                outcome.request_charge,
                per_item,
            );
        }

        assert_eq!(
            text.items.len(),
            spec.expected_items,
            "{}: expected {} items from the seeded set, got {}",
            spec.name,
            spec.expected_items,
            text.items.len(),
        );
        assert_encodings_agree(spec, &text, &binary);

        // The encoding delta is the point of the run, so report it rather than
        // leaving it to be eyeballed off the two rows above. Reported only:
        // the service picks page boundaries, so these are observations.
        let ru_delta = binary.request_charge - text.request_charge;
        let ru_pct = if text.request_charge > 0.0 {
            ru_delta / text.request_charge * 100.0
        } else {
            0.0
        };
        println!(
            "{:<38} {:>8} {:>7} {:>+7} {:>+10.2} {:>9.1}%",
            "",
            "delta",
            "",
            binary.pages as i64 - text.pages as i64,
            ru_delta,
            ru_pct,
        );
    }

    println!(
        "\nText and binary agreed on all {} query shapes.",
        specs.len()
    );
    Ok(())
}
