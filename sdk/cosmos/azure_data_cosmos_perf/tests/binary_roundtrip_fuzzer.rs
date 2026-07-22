// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end **round-trip fuzzer** for Cosmos binary JSON encoding.
//!
//! Generates random JSON documents with a seeded PRNG, stores and reads each one
//! back through a live Cosmos account under several binary-encoding
//! configurations, and asserts the value survives unchanged by comparing a
//! **Cosmos-compatible canonical form** of what was sent against what came back.
//!
//! See the design doc:
//! `azure_data_cosmos_driver/docs/BINARY_ENCODING_ROUNDTRIP_FUZZER.md`.
//!
//! # Running
//!
//! ```bash
//! # Smoke run against a local emulator:
//! AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
//! AZURE_COSMOS_ALLOW_INVALID_CERT=true \
//! RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!   cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer -- --nocapture
//!
//! # Multi-day soak (millions of docs), release build:
//! AZURE_COSMOS_CONNECTION_STRING='...' AZURE_COSMOS_FUZZ_ITERATIONS=5000000 \
//! RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!   cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer --release -- --nocapture
//!
//! # Reproduce a failure exactly:
//! AZURE_COSMOS_FUZZ_SEED=<seed printed by the failing run> ... cargo test ...
//! ```
//!
//! Every run prints its seed; a failing document is reproduced deterministically
//! by re-running with `AZURE_COSMOS_FUZZ_SEED=<seed>`.

#![allow(clippy::large_futures)]

use std::collections::BTreeMap;
use std::error::Error;
use std::hash::{Hash, Hasher};

use azure_core::http::StatusCode;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{
    BinaryEncodingOptions, ConnectionPoolOptions, ContentResponseOnWrite, ItemWriteOptions,
    OperationOptions, Region, ServerCertificateValidation,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use serde_json::{Map, Number, Value};
use uuid::Uuid;

const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const ALLOW_INVALID_CERT_ENV_VAR: &str = "AZURE_COSMOS_ALLOW_INVALID_CERT";
const DATABASE_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_DATABASE";
const CONTAINER_NAME_ENV_VAR: &str = "AZURE_COSMOS_BINARY_TEST_CONTAINER";
const ITERATIONS_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_ITERATIONS";
const SEED_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_SEED";
const MAX_DEPTH_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_MAX_DEPTH";
const WIDE_NUMBERS_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_WIDE_NUMBERS";
const UNICODE_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_UNICODE";

const DEFAULT_DATABASE_NAME: &str = "binary-fuzz-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-fuzz-ct";
const PARTITION_KEY_PATH: &str = "/pk";

const DEFAULT_ITERATIONS: u64 = 200;
const DEFAULT_MAX_DEPTH: u32 = 5;

// ─────────────────────────────────────────────────────────────────────────────
// Seeded PRNG (SplitMix64) — deterministic and dependency-feature-free, matching
// the codebase's in-tree fuzz convention so a failure reproduces from its seed.
// ─────────────────────────────────────────────────────────────────────────────

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform integer in `[0, n)`. `n` must be non-zero.
    fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }

    /// `true` with probability `num/den`.
    fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }

    fn f64_unit(&mut self) -> f64 {
        // 53-bit mantissa → [0, 1).
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Configuration
// ─────────────────────────────────────────────────────────────────────────────

struct FuzzConfig {
    iterations: u64,
    seed: u64,
    max_depth: u32,
    wide_numbers: bool,
    unicode: bool,
}

impl FuzzConfig {
    fn from_env() -> Self {
        let seed = std::env::var(SEED_ENV_VAR)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or_else(|| {
                // Non-deterministic default seed derived from the wall clock.
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos() as u64)
                    .unwrap_or(0x1234_5678_9ABC_DEF0)
            });
        Self {
            iterations: env_u64(ITERATIONS_ENV_VAR, DEFAULT_ITERATIONS),
            seed,
            max_depth: env_u64(MAX_DEPTH_ENV_VAR, DEFAULT_MAX_DEPTH as u64) as u32,
            wide_numbers: env_bool(WIDE_NUMBERS_ENV_VAR, false),
            unicode: env_bool(UNICODE_ENV_VAR, true),
        }
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(default)
}

// ─────────────────────────────────────────────────────────────────────────────
// JSON generator
// ─────────────────────────────────────────────────────────────────────────────

/// Generates a random JSON **object** suitable as a Cosmos item body.
fn gen_object(rng: &mut SplitMix64, cfg: &FuzzConfig, depth: u32) -> Map<String, Value> {
    let field_count = rng.below(6); // 0..=5 fields
    let mut map = Map::new();
    for _ in 0..field_count {
        let key = gen_key(rng, cfg);
        map.insert(key, gen_value(rng, cfg, depth + 1));
    }
    map
}

/// Generates a random JSON value, biased toward scalars as depth increases.
fn gen_value(rng: &mut SplitMix64, cfg: &FuzzConfig, depth: u32) -> Value {
    // Past max depth, only scalars.
    let allow_containers = depth < cfg.max_depth;
    let pick = if allow_containers {
        rng.below(9)
    } else {
        rng.below(6)
    };
    match pick {
        0 => Value::Null,
        1 => Value::Bool(rng.chance(1, 2)),
        2 | 3 => gen_number(rng, cfg),
        4 | 5 => Value::String(gen_string(rng, cfg)),
        6 => Value::Array(gen_array(rng, cfg, depth)),
        7 => Value::Array(gen_uniform_number_array(rng, cfg)),
        _ => Value::Object(gen_object(rng, cfg, depth)),
    }
}

fn gen_array(rng: &mut SplitMix64, cfg: &FuzzConfig, depth: u32) -> Vec<Value> {
    let len = rng.below(6);
    (0..len).map(|_| gen_value(rng, cfg, depth + 1)).collect()
}

/// A homogeneous array of numbers, to exercise the uniform-number-array wire
/// forms (`ArrNumC*`).
fn gen_uniform_number_array(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Vec<Value> {
    let len = rng.below(8);
    (0..len).map(|_| gen_number(rng, cfg)).collect()
}

/// Object keys: a mix of common Cosmos-ish names (to hit the system-string
/// dictionary) and random short identifiers.
fn gen_key(rng: &mut SplitMix64, cfg: &FuzzConfig) -> String {
    const COMMON: &[&str] = &[
        "id", "type", "name", "value", "data", "items", "count", "tags", "meta", "nested",
    ];
    if rng.chance(1, 2) {
        COMMON[rng.below(COMMON.len() as u64) as usize].to_string()
    } else {
        gen_string(rng, cfg)
    }
}

fn gen_string(rng: &mut SplitMix64, cfg: &FuzzConfig) -> String {
    let len = rng.below(24); // 0..=23 chars
    let mut s = String::new();
    for _ in 0..len {
        let ch = if cfg.unicode && rng.chance(1, 8) {
            // Occasionally emit a non-ASCII BMP or astral code point.
            if rng.chance(1, 3) {
                // Astral (emoji-ish) range.
                char::from_u32(0x1_F300 + rng.below(0x300) as u32).unwrap_or('*')
            } else {
                // BMP above ASCII, skipping surrogates.
                char::from_u32(0x00A1 + rng.below(0x2000) as u32).unwrap_or('*')
            }
        } else {
            // Printable ASCII, including characters that need JSON escaping.
            let ascii = 0x20 + rng.below(0x5F) as u8;
            ascii as char
        };
        s.push(ch);
    }
    s
}

/// Generates a number inside the **backend-safe** envelope by default (see the
/// design doc §3.2). `wide_numbers` widens it once the canonicalizer has been
/// calibrated for the extra forms.
fn gen_number(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    match rng.below(4) {
        // Small integer (literal / narrow markers).
        0 => Value::Number(Number::from(rng.below(64) as i64)),
        // Signed integer across the i64 range (narrowed unless wide).
        1 => {
            let magnitude = if cfg.wide_numbers {
                rng.next_u64() as i64
            } else {
                (rng.below(2_000_000) as i64) - 1_000_000
            };
            Value::Number(Number::from(magnitude))
        }
        // Large unsigned (exercises NumberUInt64) — only when wide.
        2 if cfg.wide_numbers => Value::Number(Number::from(rng.next_u64())),
        // Non-integral float with bounded precision.
        _ => {
            let f = if cfg.wide_numbers {
                (rng.f64_unit() - 0.5) * 1e6
            } else {
                // Two decimal places keeps it within the calibrated envelope.
                (rng.below(200_000) as f64 - 100_000.0) / 100.0
            };
            Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cosmos-compatible canonicalization (design doc §3)
// ─────────────────────────────────────────────────────────────────────────────

/// Produces a canonical string for a JSON value: whitespace removed, object keys
/// sorted, numbers normalized to a Cosmos-compatible form. Two values with the
/// same canonical string are considered equal after a round-trip.
fn canonicalize(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => canonicalize_number(n, out),
        Value::String(s) => {
            // Reuse serde_json's minimal escaping for exact string semantics.
            out.push_str(&serde_json::to_string(s).expect("string always serializes"));
        }
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                canonicalize(item, out);
            }
            out.push(']');
        }
        Value::Object(map) => {
            // Sort keys lexicographically by UTF-8 code unit.
            let sorted: BTreeMap<&String, &Value> = map.iter().collect();
            out.push('{');
            for (i, (k, v)) in sorted.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                out.push_str(&serde_json::to_string(k).expect("key always serializes"));
                out.push(':');
                canonicalize(v, out);
            }
            out.push('}');
        }
    }
}

/// Cosmos-compatible number normalization. **This is the tuning surface** — see
/// the design doc §3.1. Calibrate against a real account before widening the
/// generator's numeric envelope.
fn canonicalize_number(n: &Number, out: &mut String) {
    if let Some(i) = n.as_i64() {
        out.push_str(&i.to_string());
    } else if let Some(u) = n.as_u64() {
        out.push_str(&u.to_string());
    } else if let Some(f) = n.as_f64() {
        // Integral-valued floats (e.g. 1.0) → integer form, mirroring the
        // backend's observed rewrite of dropping a trailing ".0".
        if f.is_finite() && f.fract() == 0.0 && f.abs() < 9.007_199_254_740_992e15 {
            out.push_str(&(f as i64).to_string());
        } else {
            // Format from the `f64` via Rust's shortest round-trippable Display.
            // This is idempotent under serialize→parse: `serde_json::to_string`
            // of a `from_f64` value can emit a 17-digit form that reparses to a
            // neighboring `f64` with a shorter shortest-form, so formatting the
            // decoded `f64` directly keeps the sent and round-tripped values
            // comparable.
            out.push_str(&format!("{f}"));
        }
    } else {
        out.push_str("null");
    }
}

/// Canonicalizes and returns `(canonical_string, 64-bit hash)`.
fn canonical_hash(value: &Value) -> (String, u64) {
    let mut s = String::new();
    canonicalize(value, &mut s);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    (s, hasher.finish())
}

/// Normalizes a value by one JSON serialize→parse pass. Any Cosmos round-trip
/// (through text or binary, plus the backend's own store rewrite) puts the value
/// through at least one serialize→parse, which can shift a `from_f64` float to a
/// neighboring value with a shorter shortest-form. Computing the **sent**
/// canonical form from the normalized value puts sent and round-tripped
/// documents on equal footing. Normalization is idempotent (a parsed value's
/// shortest serialization round-trips back to itself).
fn normalize(value: &Value) -> Value {
    let text = serde_json::to_string(value).expect("value always serializes");
    serde_json::from_str(&text).expect("serialized value always parses")
}

/// Projects a returned document to only the keys present in `sent`, so
/// service-added system fields (`_rid`, `_etag`, `_ts`, ...) don't affect the
/// comparison.
fn project_to_sent_keys(sent: &Map<String, Value>, got: &Value) -> Value {
    let got_obj = match got.as_object() {
        Some(o) => o,
        None => return got.clone(),
    };
    let mut out = Map::new();
    for key in sent.keys() {
        if let Some(v) = got_obj.get(key) {
            out.insert(key.clone(), v.clone());
        }
    }
    Value::Object(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// Client / account setup
// ─────────────────────────────────────────────────────────────────────────────

/// One binary-encoding configuration exercised per generated document.
struct RunConfig {
    label: &'static str,
    binary: Option<BinaryEncodingOptions>,
}

fn run_configs() -> Vec<RunConfig> {
    vec![
        RunConfig {
            label: "text-control",
            binary: None,
        },
        RunConfig {
            label: "binary",
            binary: Some(BinaryEncodingOptions::new().with_enabled(true)),
        },
        RunConfig {
            label: "binary+text-response",
            binary: Some(
                BinaryEncodingOptions::new()
                    .with_enabled(true)
                    .with_request_text_response(true),
            ),
        },
    ]
}

/// Builds a Cosmos client for a given binary-encoding configuration.
async fn build_client(
    binary: &Option<BinaryEncodingOptions>,
) -> Result<CosmosClient, Box<dyn Error>> {
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
    if let Some(options) = binary {
        builder = builder.with_binary_encoding_options(options.clone());
    }

    let allow_invalid_cert = env_bool(ALLOW_INVALID_CERT_ENV_VAR, false);
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

fn ignore_conflict<T>(result: azure_data_cosmos::Result<T>) -> Result<(), Box<dyn Error>> {
    match result {
        Ok(_) => Ok(()),
        Err(e) if e.status().status_code() == StatusCode::Conflict => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

// ─────────────────────────────────────────────────────────────────────────────
// The fuzzer test
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::test]
#[cfg_attr(
    not(test_category = "binary_encoding"),
    ignore = "requires test_category 'binary_encoding' and a live account connection string"
)]
async fn binary_encoding_roundtrip_fuzz() -> Result<(), Box<dyn Error>> {
    let cfg = FuzzConfig::from_env();
    println!(
        "binary_roundtrip_fuzzer: seed={} iterations={} max_depth={} wide_numbers={} unicode={}",
        cfg.seed, cfg.iterations, cfg.max_depth, cfg.wide_numbers, cfg.unicode
    );
    println!("Reproduce this run with {SEED_ENV_VAR}={}", cfg.seed);

    let configs = run_configs();

    // One client per config (binary encoding is resolved at build time).
    let mut clients = Vec::new();
    for rc in &configs {
        clients.push((rc.label, build_client(&rc.binary).await?));
    }

    // Ensure the target database + container exist on each client's account
    // (they share the same account, so the first client suffices).
    let database_name =
        std::env::var(DATABASE_NAME_ENV_VAR).unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
    let container_name = std::env::var(CONTAINER_NAME_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_CONTAINER_NAME.to_string());

    let setup_client = &clients[0].1;
    ignore_conflict(setup_client.create_database(&database_name, None).await)?;
    let setup_db = setup_client.database_client(&database_name);
    ignore_conflict(
        setup_db
            .create_container(
                ContainerProperties::new(container_name.clone(), PARTITION_KEY_PATH.into()),
                None,
            )
            .await,
    )?;

    let mut rng = SplitMix64::new(cfg.seed);
    let mut checked: u64 = 0;

    for iter in 0..cfg.iterations {
        // Generate a document and stamp id + pk.
        let mut doc = gen_object(&mut rng, &cfg, 0);
        let id = Uuid::new_v4().to_string();
        let pk = format!("pk-{}", rng.below(16));
        doc.insert("id".to_string(), Value::String(id.clone()));
        doc.insert("pk".to_string(), Value::String(pk.clone()));

        let sent_value = Value::Object(doc.clone());
        // Compute the sent canonical form from a normalized copy so it matches
        // documents that have been through the backend's serialize→parse.
        let (sent_canon, sent_hash) = canonical_hash(&normalize(&sent_value));

        for (label, client) in &clients {
            let container = client
                .database_client(&database_name)
                .container_client(&container_name)
                .await?;

            let context = format!("iter={iter} config={label} id={id} seed={}", cfg.seed);

            // CREATE with content response (exercises the response decode path).
            let created = container
                .create_item(&pk, &id, &doc, Some(write_options_with_content()))
                .await
                .map_err(|e| format!("{context}: create failed: {e}"))?;
            let created_doc: Value = created
                .into_model()
                .map_err(|e| format!("{context}: create response decode failed: {e}"))?;
            assert_roundtrip(
                &doc,
                &created_doc,
                &sent_canon,
                sent_hash,
                &context,
                "create",
            );

            // READ back.
            let read = container
                .read_item(&pk, &id, None)
                .await
                .map_err(|e| format!("{context}: read failed: {e}"))?;
            let read_doc: Value = read
                .into_model()
                .map_err(|e| format!("{context}: read response decode failed: {e}"))?;
            assert_roundtrip(&doc, &read_doc, &sent_canon, sent_hash, &context, "read");

            checked += 1;
        }

        if (iter + 1) % 100 == 0 {
            println!("... {} iterations, {checked} round-trips OK", iter + 1);
        }
    }

    println!(
        "binary_roundtrip_fuzzer: DONE — {} documents × {} configs = {checked} round-trips, all canonical-equal (seed={})",
        cfg.iterations,
        configs.len(),
        cfg.seed
    );
    Ok(())
}

/// Asserts the returned document, projected to the sent keys, canonicalizes to
/// the same form (and hash) as what was sent. On mismatch, prints both canonical
/// forms and the reproduction seed.
fn assert_roundtrip(
    sent: &Map<String, Value>,
    got: &Value,
    sent_canon: &str,
    sent_hash: u64,
    context: &str,
    phase: &str,
) {
    let projected = project_to_sent_keys(sent, got);
    let (got_canon, got_hash) = canonical_hash(&projected);
    if got_hash != sent_hash || got_canon != sent_canon {
        panic!(
            "{context}: {phase} round-trip MISMATCH\n  sent  (hash {sent_hash:016x}): {sent_canon}\n  got   (hash {got_hash:016x}): {got_canon}\n  reproduce with {SEED_ENV_VAR} from the context above",
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Offline unit tests — validate the canonicalizer and generator without a live
// account. These run under a normal `cargo test -p azure_data_cosmos_perf`.
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn canon(value: &Value) -> String {
        let mut s = String::new();
        canonicalize(value, &mut s);
        s
    }

    #[test]
    fn canonicalize_sorts_object_keys() {
        let a = serde_json::json!({ "b": 1, "a": 2, "c": 3 });
        let b = serde_json::json!({ "c": 3, "a": 2, "b": 1 });
        assert_eq!(canon(&a), canon(&b));
        assert_eq!(canon(&a), r#"{"a":2,"b":1,"c":3}"#);
    }

    #[test]
    fn canonicalize_drops_whitespace_and_preserves_array_order() {
        let v: Value = serde_json::from_str("  [ 1 ,2,  3 ] ").unwrap();
        assert_eq!(canon(&v), "[1,2,3]");
    }

    #[test]
    fn canonicalize_normalizes_integral_floats_to_integers() {
        // 1.0 and 1 must canonicalize identically (mirrors the backend rewrite).
        assert_eq!(canon(&serde_json::json!(1.0)), "1");
        assert_eq!(canon(&serde_json::json!(1)), "1");
        assert_eq!(canon(&serde_json::json!(20.0)), "20");
        assert_eq!(canon(&serde_json::json!(-0.0)), "0");
    }

    #[test]
    fn canonicalize_keeps_non_integral_floats() {
        assert_eq!(canon(&serde_json::json!(3.5)), "3.5");
        assert_eq!(canon(&serde_json::json!(-2.25)), "-2.25");
    }

    #[test]
    fn canonicalize_large_unsigned_integer() {
        let v: Value = serde_json::from_str("18446744073709551614").unwrap();
        assert_eq!(canon(&v), "18446744073709551614");
    }

    #[test]
    fn project_strips_service_fields() {
        let sent: Map<String, Value> = serde_json::from_value(serde_json::json!({
            "id": "x", "pk": "p", "value": 1
        }))
        .unwrap();
        let got = serde_json::json!({
            "id": "x", "pk": "p", "value": 1,
            "_rid": "abc", "_etag": "\"y\"", "_ts": 123
        });
        let projected = project_to_sent_keys(&sent, &got);
        assert_eq!(canon(&projected), canon(&Value::Object(sent)));
    }

    #[test]
    fn generator_is_deterministic_for_a_seed() {
        let cfg = FuzzConfig {
            iterations: 0,
            seed: 42,
            max_depth: 4,
            wide_numbers: false,
            unicode: true,
        };
        let mut a = SplitMix64::new(cfg.seed);
        let mut b = SplitMix64::new(cfg.seed);
        let doc_a = Value::Object(gen_object(&mut a, &cfg, 0));
        let doc_b = Value::Object(gen_object(&mut b, &cfg, 0));
        assert_eq!(canon(&doc_a), canon(&doc_b));
    }

    #[test]
    fn generated_documents_normalize_idempotently() {
        // Sanity: after one serialize→parse normalization, a generated doc is
        // stable — a second round-trip does not change its canonical form. This
        // is the invariant the fuzzer relies on to compare the (normalized) sent
        // doc against a backend-round-tripped one.
        let cfg = FuzzConfig {
            iterations: 0,
            seed: 7,
            max_depth: 5,
            wide_numbers: true,
            unicode: true,
        };
        let mut rng = SplitMix64::new(cfg.seed);
        for _ in 0..500 {
            let doc = Value::Object(gen_object(&mut rng, &cfg, 0));
            let once = normalize(&doc);
            let twice = normalize(&once);
            assert_eq!(
                canon(&once),
                canon(&twice),
                "normalization not idempotent for doc: {}",
                serde_json::to_string(&doc).unwrap()
            );
        }
    }
}
