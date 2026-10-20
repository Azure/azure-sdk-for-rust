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
//! The client is built with binary encoding on via
//! `CosmosClientBuilder::with_binary_encoding_options`, which sets the
//! **default** for operations that do not specify one — that is what the seed
//! phase writes under. Each query then sets [`OperationOptions::binary_encoding`]
//! explicitly, which takes precedence over that default (see the SDK's
//! `resolve_binary_encoding`), so the two comparison arms genuinely differ.
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
use serde_json::{Map, Number, Value};
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

/// One of the encoding configurations a query shape is replayed under.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Encoding {
    /// Binary encoding off: text JSON in both directions. The baseline every
    /// other arm is compared against.
    Text,
    /// Binary encoding on, response left in binary.
    Binary,
    /// Binary encoding on, but the caller asked for a text payload. The wire
    /// stays binary and the driver transcodes each item back to text in
    /// `execute_plan`, so this is the only arm that exercises that transcode
    /// over a real multi-page query.
    BinaryTextResponse,
}

impl Encoding {
    /// The label used for this arm in the report.
    fn label(self) -> &'static str {
        match self {
            Encoding::Text => "text",
            Encoding::Binary => "binary",
            Encoding::BinaryTextResponse => "bin+txt_rsp",
        }
    }

    /// The per-operation options that select this arm.
    fn options(self) -> BinaryEncodingOptions {
        match self {
            Encoding::Text => BinaryEncodingOptions::new().with_enabled(false),
            Encoding::Binary => BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(false),
            Encoding::BinaryTextResponse => BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(true),
        }
    }
}

/// Drains `spec` across the whole container under a single encoding arm.
///
/// The encoding is set explicitly on every arm so none silently inherits the
/// client-level default, which would make the comparison meaningless.
async fn run_query(
    container: &ContainerClient,
    spec: &QuerySpec,
    run_id: &str,
    encoding: Encoding,
) -> Result<QueryOutcome, Box<dyn Error>> {
    let query = Query::from(spec.text).with_parameter("@run", run_id)?;

    let mut operation = OperationOptions::default();
    operation.binary_encoding = Some(encoding.options());
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

/// Compares two JSON values, treating the two spellings of a number the
/// service may return as equal without loosening the comparison itself.
fn json_equivalent(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => numbers_equivalent(x, y),
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

/// The largest magnitude at which every integer is exactly representable as an
/// IEEE-754 double. At or below this, text and binary must agree *exactly*.
const EXACT_INTEGER_LIMIT: u128 = 1 << 53;

/// Strict comparison for the **text-vs-binary** arms, where tolerance would
/// defeat the test.
///
/// [`json_equivalent`] reconciles `Number` variants for the *sent → stored*
/// check, which is legitimate there because Cosmos stores every number as a
/// double. It does not transfer here: both arms read the same stored documents,
/// so any variant difference came from our encoding path — exactly what this
/// test exists to catch.
///
/// The one carve-out: past 2^53 an integer is no longer exactly representable
/// as a double, so the text spelling and the value recoverable from a binary
/// token can legitimately disagree. Those fall back to numeric comparison.
fn json_identical(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => numbers_identical(x, y),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| json_identical(a, b))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|other| json_identical(v, other)))
        }
        _ => a == b,
    }
}

/// Requires the same [`Number`] variant *and* value, except for integers beyond
/// ±2^53 — see [`json_identical`] for why that carve-out exists and why it is
/// the only one.
fn numbers_identical(x: &Number, y: &Number) -> bool {
    if x == y {
        return true;
    }
    // The literals differ. Tolerate that only for integers too wide to survive
    // a double exactly, and only when they still denote the same value.
    match (integral_value(x), integral_value(y)) {
        (Some(a), Some(b)) => a == b && a.unsigned_abs() > EXACT_INTEGER_LIMIT,
        _ => false,
    }
}

/// Exact numeric identity across the variants the service may produce.
///
/// Cosmos stores every number as an IEEE-754 double, so one value can come back
/// as an integer literal in text but as a `Double` in binary. Those decode to
/// different [`Number`] variants that compare unequal despite denoting the same
/// value, so the variants have to be reconciled.
///
/// Reconciling them by comparing `as_f64()` would defeat the test: beyond 2^53
/// distinct integers collapse onto the same `f64`, so a corrupted wide integer
/// — precisely the class this corpus exists to catch — would compare equal to
/// the original. Integral values are therefore compared exactly as `i128`.
fn numbers_equivalent(x: &Number, y: &Number) -> bool {
    match (integral_value(x), integral_value(y)) {
        (Some(a), Some(b)) => a == b,
        // Both fractional (or `-0.0`). Compare bit patterns so a 1-ULP shift is
        // caught and `-0.0` stays distinct from `0.0`.
        (None, None) => match (x.as_f64(), y.as_f64()) {
            (Some(a), Some(b)) => a.to_bits() == b.to_bits(),
            // Unreachable unless `serde_json/arbitrary_precision` is enabled,
            // where a number need not be representable as an `f64`; fall back
            // to the literal comparison rather than silently passing.
            _ => x == y,
        },
        // One denotes an exact integer and the other does not, so they differ.
        _ => false,
    }
}

/// The exact integer `number` denotes, or `None` when it is fractional, is
/// `-0.0`, or falls outside `i128` (where no exact comparison is available).
///
/// `-0.0` is integral but carries a sign no integer form can, so it is left to
/// the bit-exact float path.
fn integral_value(number: &Number) -> Option<i128> {
    if let Some(value) = number.as_u64() {
        return Some(i128::from(value));
    }
    if let Some(value) = number.as_i64() {
        return Some(i128::from(value));
    }

    let float = number.as_f64()?;
    if !float.is_finite() || float.fract() != 0.0 {
        return None;
    }
    if float == 0.0 {
        return if float.is_sign_negative() {
            None
        } else {
            Some(0)
        };
    }
    // `as` saturates rather than wrapping, so bound the cast to keep it exact.
    const I128_EXCLUSIVE_UPPER_BOUND: f64 = 170_141_183_460_469_231_731_687_303_715_884_105_728.0;
    (-I128_EXCLUSIVE_UPPER_BOUND..I128_EXCLUSIVE_UPPER_BOUND)
        .contains(&float)
        .then(|| float as i128)
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

/// Asserts the baseline text run of `spec` and one other encoding arm returned
/// equivalent results.
///
/// Both arms read the same stored documents, so the comparison is **strict**
/// (see [`json_identical`]): a number that comes back as a different
/// `serde_json::Number` variant in one arm than the other is a defect in the
/// encoding path, not something the service is entitled to vary.
fn assert_encodings_agree(
    spec: &QuerySpec,
    text: &QueryOutcome,
    other: &QueryOutcome,
    other_label: &str,
) {
    assert_eq!(
        text.items.len(),
        other.items.len(),
        "{}: text returned {} items but {other_label} returned {}",
        spec.name,
        text.items.len(),
        other.items.len(),
    );

    let (text_items, other_items) = match spec.compare {
        Compare::Count => return,
        Compare::Sequence => (
            text.items.iter().collect::<Vec<_>>(),
            other.items.iter().collect::<Vec<_>>(),
        ),
        Compare::Set => (sorted_by_id(&text.items), sorted_by_id(&other.items)),
    };

    for (i, (t, b)) in text_items.iter().zip(&other_items).enumerate() {
        assert!(
            json_identical(t, b),
            "{}: item #{i} (id={}) differs between text and {other_label}\n  text:  {}\n  {other_label}: {}",
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
    let container = db_client.container_client(&container_name, None).await?;

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
        "\n{:<38} {:>11} {:>7} {:>7} {:>10} {:>10}",
        "query", "encoding", "items", "pages", "RU", "RU/item",
    );

    for spec in &specs {
        let text = run_query(&container, spec, &run_id, Encoding::Text).await?;
        let binary = run_query(&container, spec, &run_id, Encoding::Binary).await?;
        // Binary on the wire, text handed back. Every item on this arm goes
        // through the driver's transcode, so agreement here is what proves the
        // transcode preserves values across a real multi-page query.
        let binary_text =
            run_query(&container, spec, &run_id, Encoding::BinaryTextResponse).await?;

        for (encoding, outcome) in [
            (Encoding::Text, &text),
            (Encoding::Binary, &binary),
            (Encoding::BinaryTextResponse, &binary_text),
        ] {
            let per_item = if outcome.items.is_empty() {
                0.0
            } else {
                outcome.request_charge / outcome.items.len() as f64
            };
            println!(
                "{:<38} {:>11} {:>7} {:>7} {:>10.2} {:>10.3}",
                spec.name,
                encoding.label(),
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
        assert_encodings_agree(spec, &text, &binary, Encoding::Binary.label());
        assert_encodings_agree(
            spec,
            &text,
            &binary_text,
            Encoding::BinaryTextResponse.label(),
        );

        // The encoding delta is the point of the run, so report it rather than
        // leaving it to be eyeballed off the rows above. Reported only: the
        // service picks page boundaries, so these are observations.
        for (encoding, outcome) in [
            (Encoding::Binary, &binary),
            (Encoding::BinaryTextResponse, &binary_text),
        ] {
            let ru_delta = outcome.request_charge - text.request_charge;
            let ru_pct = if text.request_charge > 0.0 {
                ru_delta / text.request_charge * 100.0
            } else {
                0.0
            };
            println!(
                "{:<38} {:>11} {:>7} {:>+7} {:>+10.2} {:>9.1}%",
                "",
                format!("{} delta", encoding.label()),
                "",
                outcome.pages as i64 - text.pages as i64,
                ru_delta,
                ru_pct,
            );
        }
    }

    println!(
        "\nAll three encodings agreed on all {} query shapes.",
        specs.len()
    );
    Ok(())
}

/// Offline checks on the comparators, pinning the property the live comparison
/// depends on: [`json_identical`] must reject a variant difference that
/// [`json_equivalent`] is allowed to accept.
mod comparators {
    use super::{json_equivalent, json_identical};
    use serde_json::{json, Number, Value};

    /// The same value as an integer literal and as a double — what the
    /// integral-double coercion exists to reconcile.
    fn integer_and_double(value: u64) -> (Value, Value) {
        (
            Value::Number(Number::from(value)),
            Value::Number(Number::from_f64(value as f64).expect("finite")),
        )
    }

    #[test]
    fn strict_comparison_rejects_an_integral_double_the_tolerant_one_accepts() {
        let (int, double) = integer_and_double(3);
        // Tolerant: this is the *sent → stored* check, and Cosmos really does
        // store `3` as a double.
        assert!(json_equivalent(&int, &double));
        // Strict must not. If this ever passes, deleting the integral-double
        // coercion no longer fails the live comparison.
        assert!(!json_identical(&int, &double));
    }

    #[test]
    fn strict_comparison_looks_inside_arrays_and_objects() {
        let (int, double) = integer_and_double(7);
        let nested_int = json!({ "a": [ { "b": int } ] });
        let nested_double = json!({ "a": [ { "b": double } ] });
        assert!(json_equivalent(&nested_int, &nested_double));
        assert!(!json_identical(&nested_int, &nested_double));
    }

    #[test]
    fn strict_comparison_accepts_matching_values() {
        let value = json!({ "id": "x", "n": 3, "f": 1.5, "a": [1, 2], "z": null });
        assert!(json_identical(&value, &value.clone()));
    }

    #[test]
    fn wide_integers_keep_the_documented_carve_out() {
        // Past 2^53 the text spelling and the value recoverable from a binary
        // token can legitimately disagree, so the variants are reconciled.
        let (int, double) = integer_and_double(1 << 60);
        assert!(json_identical(&int, &double));

        // But only just past it: 2^53 itself is exactly representable, so it
        // stays inside the strict range.
        let (int, double) = integer_and_double(1 << 53);
        assert!(!json_identical(&int, &double));
    }

    #[test]
    fn strict_comparison_rejects_a_changed_value() {
        assert!(!json_identical(&json!(3), &json!(4)));
        assert!(!json_identical(&json!(1.5), &json!(1.5000000000000002)));
        assert!(!json_identical(&json!("3"), &json!(3)));
    }
}

/// Pins that the live comparison actually *uses* the strict comparator.
///
/// The checks in [`comparators`] exercise [`json_identical`] in isolation, so
/// they stay green if the call inside [`assert_encodings_agree`] is switched
/// back to [`json_equivalent`] — the one-line edit that would make the strict
/// comparison vacuous. These drive the helper itself instead.
mod use_site {
    use super::{assert_encodings_agree, Compare, QueryOutcome, QuerySpec};
    use serde_json::{json, Number, Value};

    fn spec() -> QuerySpec {
        QuerySpec {
            name: "use-site guard",
            text: "SELECT * FROM c",
            compare: Compare::Sequence,
            expected_items: 1,
        }
    }

    fn outcome(items: Vec<Value>) -> QueryOutcome {
        QueryOutcome {
            items,
            pages: 1,
            request_charge: 0.0,
        }
    }

    /// One item whose `n` is the integer `3`, and the same item whose `n` is
    /// the double `3.0`: the same value in a different `Number` variant, which
    /// is exactly what a defect in the encoding path looks like.
    fn text_arm() -> QueryOutcome {
        outcome(vec![json!({ "id": "a", "n": 3 })])
    }

    fn binary_arm() -> QueryOutcome {
        let double = Value::Number(Number::from_f64(3.0).expect("finite"));
        outcome(vec![json!({ "id": "a", "n": double })])
    }

    #[test]
    #[should_panic(expected = "differs between text and binary")]
    fn a_variant_split_between_the_arms_fails_the_comparison() {
        assert_encodings_agree(&spec(), &text_arm(), &binary_arm(), "binary");
    }

    /// The companion: the panic above has to come from the variant split, not
    /// from item counts or some unrelated assertion inside the helper.
    #[test]
    fn arms_that_match_exactly_pass_the_comparison() {
        assert_encodings_agree(&spec(), &text_arm(), &text_arm(), "binary");
    }
}
