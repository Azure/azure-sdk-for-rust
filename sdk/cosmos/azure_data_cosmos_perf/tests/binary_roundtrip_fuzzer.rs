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

use std::error::Error;

use arbitrary::{Arbitrary, Unstructured};
use arbitrary_json::ArbitraryValue;
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
use sha2::{Digest, Sha256};
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
const BREADTH_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_BREADTH";
const CALIBRATE_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_CALIBRATE";
const PRINT_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_PRINT";

const DEFAULT_DATABASE_NAME: &str = "binary-fuzz-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-fuzz-ct";
const PARTITION_KEY_PATH: &str = "/pk";

const DEFAULT_ITERATIONS: u64 = 200;
const DEFAULT_MAX_DEPTH: u32 = 6;
/// Default maximum number of child fields/elements generated at each container
/// level (the branching factor). Higher values produce larger, wider documents.
const DEFAULT_BREADTH: u32 = 6;

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
    breadth: u32,
    calibrate: bool,
    print_docs: bool,
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
            breadth: env_u64(BREADTH_ENV_VAR, DEFAULT_BREADTH as u64).max(1) as u32,
            calibrate: env_bool(CALIBRATE_ENV_VAR, false),
            print_docs: env_bool(PRINT_ENV_VAR, false),
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
// JSON generator (arbitrary-json, seeded from the PRNG)
// ─────────────────────────────────────────────────────────────────────────────

/// Generates a random JSON **object** suitable as a Cosmos item body.
///
/// Uses a **hybrid** strategy: a depth-controlled *skeleton* guarantees the
/// document actually reaches a target nesting depth (drawn from `[1, max_depth]`),
/// while every leaf and filler branch is irregular JSON — a mix of hand-rolled
/// typed scalars (integers, floats, alphabetic / alphanumeric / free-text /
/// non-ASCII strings, booleans, nulls, number arrays) and [`arbitrary_json`]
/// subtrees. This fixes the `arbitrary_iter` shallowness (it stops nesting
/// almost immediately regardless of byte budget), so `max_depth` now
/// meaningfully scales structure and `breadth` scales width. Everything is
/// driven by the [`SplitMix64`] seed stream, so the same `AZURE_COSMOS_FUZZ_SEED`
/// reproduces the same document.
///
/// Every document also carries a **sampler** subtree ([`gen_sampler`]) that
/// guarantees at least one value of each category appears, so a single run
/// exercises numeric, alphabetic, alphanumeric, free-text, and non-ASCII data
/// under multi-level nesting.
///
/// A [`bound_value`] pass applies the `wide_numbers` / `unicode` knobs: by
/// default numbers are clamped into the calibrated-safe envelope (design doc
/// §3.2) and strings to ASCII, unless explicitly widened. The hand-rolled
/// scalars already respect those knobs directly.
fn gen_object(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let max_depth = cfg.max_depth.max(1);
    // Target nesting depth for this document's spine, in [1, max_depth].
    let target_depth = 1 + rng.below(max_depth as u64) as u32;

    let mut map = Map::new();
    // A guaranteed sampler covering every value category (numeric, alphabetic,
    // alphanumeric, free text, non-ASCII, boolean, null, number array, nested).
    // Keyed distinctly from the caller-reserved `id`/`pk` so it is never
    // overwritten.
    map.insert("_sampler".to_string(), gen_sampler(rng, cfg));
    // A spread of irregular root fields (typed scalars + arbitrary-json subtrees).
    for _ in 0..rng.below(cfg.breadth as u64 + 1) {
        map.insert(gen_key(rng), gen_filler_value(rng, cfg));
    }
    // The spine field guarantees the target depth is reached. Its key avoids the
    // caller-reserved `id`/`pk`/`_sampler` (and empty) so nothing overwrites the
    // deep subtree.
    let mut spine_key = gen_key(rng);
    while spine_key.is_empty() || spine_key == "id" || spine_key == "pk" || spine_key == "_sampler"
    {
        spine_key.push('_');
    }
    map.insert(spine_key, gen_spine(rng, cfg, target_depth));

    map
}

/// Number of PRNG bytes fed to `arbitrary-json` for one filler subtree. A larger
/// budget lets `arbitrary-json` build bigger, deeper irregular subtrees.
const FILLER_BUDGET: usize = 256;

/// Refills `n` bytes deterministically from the PRNG.
fn fill_bytes(rng: &mut SplitMix64, n: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(n);
    while bytes.len() < n {
        bytes.extend_from_slice(&rng.next_u64().to_le_bytes());
    }
    bytes
}

/// A random object key from `arbitrary-json`'s string generator.
fn gen_key(rng: &mut SplitMix64) -> String {
    let bytes = fill_bytes(rng, 16);
    let mut u = Unstructured::new(&bytes);
    String::arbitrary(&mut u).unwrap_or_default()
}

// ─────────────────────────────────────────────────────────────────────────────
// Typed scalar generators (character classes + numbers), seeded from the PRNG.
// ─────────────────────────────────────────────────────────────────────────────

/// ASCII letters, for the alphabetic string class.
const ALPHA_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
/// ASCII letters + digits, for the alphanumeric string class.
const ALPHANUMERIC_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
/// A spread of non-ASCII scalars across scripts, symbols, and astral-plane
/// emoji — exercises multi-byte UTF-8 and surrogate-pair paths in the codec.
const NON_ASCII_CHARS: &[char] = &[
    'é', 'ñ', 'ü', 'ß', 'ç', 'å', 'ø', 'Ω', 'λ', 'π', 'µ', 'я', 'ж', 'д', 'α', 'β', '中', '文',
    '日', '本', '語', '한', '국', 'ع', 'ب', '€', '£', '¥', '©', '™', '—', '…', '→', '∑', '≈', '♠',
    '☃', '😀', '🚀', '🌍', '🎉', '𝄞', '𐍈',
];

/// A uniform integer in `[lo, hi]` (inclusive). `lo <= hi` required.
fn gen_int_in(rng: &mut SplitMix64, lo: i64, hi: i64) -> i64 {
    let span = (hi - lo) as u64 + 1;
    lo + rng.below(span) as i64
}

/// A random ASCII string drawn from `pool`, length in `[1, max_len]`.
fn gen_string_from(rng: &mut SplitMix64, pool: &[u8], max_len: usize) -> String {
    let len = 1 + rng.below(max_len as u64) as usize;
    (0..len)
        .map(|_| pool[rng.below(pool.len() as u64) as usize] as char)
        .collect()
}

/// A string mixing alphanumeric and non-ASCII scalars, length in `[1, max_len]`.
/// Falls back to alphanumeric-only when `unicode` generation is disabled so the
/// ASCII envelope contract (design doc §3.2) still holds.
fn gen_unicode_string(rng: &mut SplitMix64, cfg: &FuzzConfig, max_len: usize) -> String {
    if !cfg.unicode {
        return gen_string_from(rng, ALPHANUMERIC_CHARS, max_len);
    }
    let len = 1 + rng.below(max_len as u64) as usize;
    (0..len)
        .map(|_| {
            if rng.below(2) == 0 {
                ALPHANUMERIC_CHARS[rng.below(ALPHANUMERIC_CHARS.len() as u64) as usize] as char
            } else {
                NON_ASCII_CHARS[rng.below(NON_ASCII_CHARS.len() as u64) as usize]
            }
        })
        .collect()
}

/// An envelope-safe integer (`±1_000_000`).
fn gen_envelope_int(rng: &mut SplitMix64) -> Value {
    Value::Number(Number::from(gen_int_in(rng, -1_000_000, 1_000_000)))
}

/// An envelope-safe two-decimal float (`±100_000.00`).
fn gen_envelope_float(rng: &mut SplitMix64) -> Value {
    let cents = gen_int_in(rng, -10_000_000, 10_000_000);
    Number::from_f64(cents as f64 / 100.0)
        .map(Value::Number)
        .unwrap_or_else(|| Value::Number(Number::from(0)))
}

/// A single number: envelope-safe by default; when `wide_numbers` is set,
/// occasionally a wide value beyond `2^53` that drives the calibrated
/// string-token comparison path (design doc §3.1).
fn gen_number(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    if cfg.wide_numbers && rng.below(4) == 0 {
        return Value::Number(Number::from(rng.next_u64() as i64));
    }
    if rng.below(2) == 0 {
        gen_envelope_int(rng)
    } else {
        gen_envelope_float(rng)
    }
}

/// One rich scalar spanning the value taxonomy: integer, float, alphabetic,
/// alphanumeric, free text, non-ASCII, boolean, or null.
fn gen_scalar(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    match rng.below(8) {
        0 | 1 => gen_number(rng, cfg),
        2 => Value::String(gen_string_from(rng, ALPHA_CHARS, 24)),
        3 => Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 24)),
        4 => Value::String(gen_unicode_string(rng, cfg, 24)),
        5 => Value::String(gen_unicode_string(rng, cfg, 80)), // longer free text
        6 => Value::Bool(rng.below(2) == 0),
        _ => Value::Null,
    }
}

/// A sampler object guaranteeing every value category appears in the document at
/// least once: integer, float, alphabetic, alphanumeric, free text, non-ASCII,
/// boolean, null, a homogeneous number array, and a small nested object.
fn gen_sampler(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    let mut map = Map::new();
    map.insert("int".into(), gen_envelope_int(rng));
    map.insert("float".into(), gen_envelope_float(rng));
    map.insert(
        "alpha".into(),
        Value::String(gen_string_from(rng, ALPHA_CHARS, 24)),
    );
    map.insert(
        "alphanumeric".into(),
        Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 24)),
    );
    map.insert(
        "text".into(),
        Value::String(gen_unicode_string(rng, cfg, 64)),
    );
    map.insert(
        "unicode".into(),
        Value::String(gen_unicode_string(rng, cfg, 24)),
    );
    map.insert("flag".into(), Value::Bool(rng.below(2) == 0));
    map.insert("empty".into(), Value::Null);
    let count = 1 + rng.below(8);
    let numbers = (0..count).map(|_| gen_envelope_int(rng)).collect();
    map.insert("numbers".into(), Value::Array(numbers));
    // A small nested object so the sampler itself has a second level.
    let mut nested = Map::new();
    nested.insert("mixed".into(), gen_scalar(rng, cfg));
    nested.insert(
        "list".into(),
        Value::Array(
            (0..1 + rng.below(4))
                .map(|_| gen_scalar(rng, cfg))
                .collect(),
        ),
    );
    map.insert("nested".into(), Value::Object(nested));
    Value::Object(map)
}

/// A small, irregular filler value. Draws from typed scalars, mixed
/// arrays/objects of typed scalars, homogeneous number arrays (to exercise the
/// uniform-number wire forms), and `arbitrary-json` subtrees — so filler is both
/// varied and non-trivial in size. Already respects the `wide_numbers`/`unicode`
/// knobs.
fn gen_filler_value(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    match rng.below(10) {
        // Homogeneous number array (uniform-number wire forms).
        0 => {
            let len = rng.below(8);
            let arr = (0..len).map(|_| gen_envelope_int(rng)).collect();
            Value::Array(arr)
        }
        // Typed scalars across the character/number classes.
        1..=3 => gen_scalar(rng, cfg),
        // A short mixed-type array.
        4 => {
            let len = 1 + rng.below(cfg.breadth as u64 + 1);
            Value::Array((0..len).map(|_| gen_scalar(rng, cfg)).collect())
        }
        // A small object of typed scalars.
        5 => {
            let mut map = Map::new();
            for _ in 0..1 + rng.below(cfg.breadth as u64) {
                map.insert(gen_key(rng), gen_scalar(rng, cfg));
            }
            Value::Object(map)
        }
        // An `arbitrary-json` subtree (bigger byte budget), envelope-bounded.
        _ => {
            let bytes = fill_bytes(rng, FILLER_BUDGET);
            let mut u = Unstructured::new(&bytes);
            let mut v: Value = ArbitraryValue::arbitrary(&mut u)
                .map(Into::into)
                .unwrap_or(Value::Null);
            bound_value(&mut v, cfg);
            v
        }
    }
}

/// Builds a nested container chain `depth` levels deep, with several irregular
/// filler siblings at each level, guaranteeing the document reaches `depth`.
/// Each level is randomly an object or an array; exactly one child continues the
/// spine deeper. The sibling count scales with `breadth`, so deeper documents
/// are also wider.
fn gen_spine(rng: &mut SplitMix64, cfg: &FuzzConfig, depth: u32) -> Value {
    if depth == 0 {
        return gen_filler_value(rng, cfg);
    }
    if rng.below(2) == 0 {
        // Object: filler fields + one spine field going deeper.
        let mut map = Map::new();
        for _ in 0..rng.below(cfg.breadth as u64 + 1) {
            map.insert(gen_key(rng), gen_filler_value(rng, cfg));
        }
        let mut key = gen_key(rng);
        while key.is_empty() {
            key.push('_');
        }
        map.insert(key, gen_spine(rng, cfg, depth - 1));
        Value::Object(map)
    } else {
        // Array: filler elements + one spine element going deeper.
        let mut arr = Vec::new();
        for _ in 0..rng.below(cfg.breadth as u64 + 1) {
            arr.push(gen_filler_value(rng, cfg));
        }
        arr.push(gen_spine(rng, cfg, depth - 1));
        Value::Array(arr)
    }
}

/// Recursively applies the generation bounds to a value: clamps numbers into the
/// calibrated-safe envelope unless `wide_numbers`, and drops non-ASCII from
/// strings unless `unicode`. Leaves structure otherwise untouched.
fn bound_value(value: &mut Value, cfg: &FuzzConfig) {
    match value {
        Value::Number(n) => {
            if !cfg.wide_numbers {
                *value = clamp_number_to_envelope(n);
            }
        }
        Value::String(s) => {
            if !cfg.unicode && !s.is_ascii() {
                *s = s.chars().filter(char::is_ascii).collect();
            }
        }
        Value::Array(items) => {
            for item in items.iter_mut() {
                bound_value(item, cfg);
            }
        }
        Value::Object(map) => {
            for v in map.values_mut() {
                bound_value(v, cfg);
            }
        }
        _ => {}
    }
}

/// Clamps a number into the **backend-safe** envelope (design doc §3.2): bounded
/// integers and two-decimal floats, matching what the calibrated
/// [`normalize_number`] models without `--wide-numbers`.
fn clamp_number_to_envelope(n: &Number) -> Value {
    if let Some(i) = n.as_i64() {
        Value::Number(Number::from(i.rem_euclid(2_000_001) - 1_000_000))
    } else if let Some(u) = n.as_u64() {
        Value::Number(Number::from((u % 2_000_001) as i64 - 1_000_000))
    } else if let Some(f) = n.as_f64() {
        // Two decimal places within ±100_000 keeps it inside the calibrated
        // envelope; a non-finite arbitrary float collapses to 0.
        let bounded = if f.is_finite() {
            ((f % 100_000.0) * 100.0).round() / 100.0
        } else {
            0.0
        };
        Number::from_f64(bounded)
            .map(Value::Number)
            .unwrap_or_else(|| Value::Number(Number::from(0)))
    } else {
        Value::Number(Number::from(0))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cosmos-compatible canonicalization (design doc §3)
// ─────────────────────────────────────────────────────────────────────────────

/// Produces the canonical string for a JSON value: the calibrated Cosmos number
/// rewrite ([`normalize_numbers`]) followed by RFC 8785 (JCS) structural
/// canonicalization via [`json_canon`] — whitespace removed, object keys sorted,
/// strings minimally escaped. Two values with the same canonical string are
/// considered equal after a round-trip.
///
/// Numbers are normalized **first** so the JCS serializer's own number
/// formatting no longer affects the comparison; the only Cosmos-specific step is
/// [`normalize_numbers`] (design doc §3.1).
fn canonicalize(value: &Value) -> String {
    let normalized = normalize_numbers(value);
    json_canon::to_string(&normalized).expect("normalized value always canonicalizes")
}

/// Rewrites a single JSON number to its **Cosmos-calibrated** canonical
/// [`Value`]. **This is the tuning surface** — see the design doc §3.1. It is
/// the one number-specific step that must stay under our control (RFC 8785 / JCS
/// number formatting is *not* the backend's store-time rewrite); the structural
/// canonicalization around it can be delegated to a standard JCS serializer.
///
/// Rules (calibrated against a live account, design doc §3.1):
/// - integers with magnitude `< 2^53` → exact integer (JCS-safe);
/// - integers with magnitude `>= 2^53` that fit `i64` → exact **string token**
///   (Cosmos preserves them exactly, but RFC 8785 / JCS refuses to emit integers
///   beyond the safe range, so they are compared as a stable decimal token);
/// - integers above `i64::MAX` → **string token** of the `f64` form (the backend
///   stores them as IEEE-754 doubles), so a sent `u64` and its returned double
///   map to the same token;
/// - integral-valued floats below `2^53` (e.g. `1.0`) → integer form (the
///   backend drops the trailing `.0`);
/// - integral-valued floats `>= 2^53` → `f64` string token (matches the lossy
///   double case above);
/// - other finite floats → kept as `f64` (JCS-safe);
/// - non-finite (`NaN` / `±∞`) → `null`.
///
/// The string tokens are only ever compared for equality (never parsed back), so
/// representing an out-of-JCS-range number as a token is sound: any two values
/// Cosmos would round-trip to each other produce the identical token.
fn normalize_number(n: &Number) -> Value {
    if let Some(i) = n.as_i64() {
        if (i.unsigned_abs() as f64) < JCS_SAFE_INT_LIMIT {
            Value::Number(Number::from(i))
        } else {
            Value::String(i.to_string())
        }
    } else if let Some(u) = n.as_u64() {
        // u > i64::MAX: Cosmos stores it as a lossy double; token from the double.
        Value::String(cosmos_double_token(u as f64))
    } else if let Some(f) = n.as_f64() {
        if !f.is_finite() {
            Value::Null
        } else if f.fract() == 0.0 && f.abs() < JCS_SAFE_INT_LIMIT {
            Value::Number(Number::from(f as i64))
        } else if f.fract() == 0.0 {
            // Integral but out of the JCS-safe range → double token.
            Value::String(cosmos_double_token(f))
        } else {
            // Non-integral finite float is JCS-safe as a number.
            Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
    } else {
        Value::Null
    }
}

/// `2^53`: the largest magnitude RFC 8785 (JCS) will emit as an integer. At or
/// beyond this, `json-canon` refuses integer output and Cosmos stores `u64`
/// above `i64::MAX` lossily as doubles, so such numbers are canonicalized as
/// string tokens (see [`normalize_number`]).
const JCS_SAFE_INT_LIMIT: f64 = 9_007_199_254_740_992.0;

/// A stable decimal token for a Cosmos-stored double, from the `f64` value.
/// Both a sent `u64` and its returned scientific-notation double parse to the
/// same `f64`, so they produce the same token.
fn cosmos_double_token(f: f64) -> String {
    format!("{f}")
}

/// Recursively rewrites every number in `value` to its Cosmos-calibrated form
/// (see [`normalize_number`]), leaving all other value kinds unchanged. The
/// result is a `Value` ready for a standard (JCS) structural canonicalization
/// pass — the number rewrite has already been applied, so the structural
/// serializer's own number formatting no longer changes the comparison.
fn normalize_numbers(value: &Value) -> Value {
    match value {
        Value::Number(n) => normalize_number(n),
        Value::Array(items) => Value::Array(items.iter().map(normalize_numbers).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), normalize_numbers(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Canonicalizes and returns `(canonical_string, SHA-256 digest)`.
///
/// The digest is a cryptographic hash of the canonical string, so it is stable
/// across runs and platforms — suitable for a durable corpus of expected `H0`
/// values ("store the hash once, compare later").
fn canonical_hash(value: &Value) -> (String, [u8; 32]) {
    let s = canonicalize(value);
    let digest: [u8; 32] = Sha256::digest(s.as_bytes()).into();
    (s, digest)
}

/// Formats a 32-byte digest as lowercase hex, for mismatch reporting.
fn hex(digest: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for b in digest {
        out.push_str(&format!("{b:02x}"));
    }
    out
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

    if cfg.calibrate {
        return run_calibration().await;
    }

    println!(
        "binary_roundtrip_fuzzer: seed={} iterations={} max_depth={} breadth={} wide_numbers={} unicode={}",
        cfg.seed, cfg.iterations, cfg.max_depth, cfg.breadth, cfg.wide_numbers, cfg.unicode
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
        // Generate the document body once per iteration so all three configs
        // test the *same value* three ways. Each config gets a distinct `id`
        // below — the same document stored under multiple configs would
        // otherwise collide on the `(pk, id)` key and fail with 409 Conflict.
        let base_doc = gen_object(&mut rng, &cfg);
        let pk = format!("pk-{}", rng.below(16));

        // Optionally print the generated document (pretty JSON) so a run can be
        // eyeballed. Enable with `AZURE_COSMOS_FUZZ_PRINT=true`.
        if cfg.print_docs {
            println!(
                "--- iter {iter} (seed={}) ---\n{}",
                cfg.seed,
                serde_json::to_string_pretty(&Value::Object(base_doc.clone()))
                    .unwrap_or_else(|_| "<unserializable>".to_string())
            );
        }

        for (label, client) in &clients {
            let container = client
                .database_client(&database_name)
                .container_client(&container_name)
                .await?;

            // Distinct id per config so the three stores don't conflict.
            let id = Uuid::new_v4().to_string();
            let mut doc = base_doc.clone();
            doc.insert("id".to_string(), Value::String(id.clone()));
            doc.insert("pk".to_string(), Value::String(pk.clone()));

            // Compute the sent canonical form from a normalized copy so it
            // matches documents that have been through the backend's
            // serialize→parse.
            let (sent_canon, sent_hash) = canonical_hash(&normalize(&Value::Object(doc.clone())));

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
                &sent_hash,
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
            assert_roundtrip(&doc, &read_doc, &sent_canon, &sent_hash, &context, "read");

            // REPLACE the item with the same value (exercises the replace point
            // op's request encode + response decode). Binary encoding is honored
            // for replace, so this drives the encoder/decoder just like create.
            let replaced = container
                .replace_item(&pk, &id, &doc, Some(write_options_with_content()))
                .await
                .map_err(|e| format!("{context}: replace failed: {e}"))?;
            let replaced_doc: Value = replaced
                .into_model()
                .map_err(|e| format!("{context}: replace response decode failed: {e}"))?;
            assert_roundtrip(
                &doc,
                &replaced_doc,
                &sent_canon,
                &sent_hash,
                &context,
                "replace",
            );

            // UPSERT the same value (upsert is a point op that also carries a
            // body; here it updates the existing item). Covers the upsert
            // request-encode + response-decode path.
            let upserted = container
                .upsert_item(&pk, &id, &doc, Some(write_options_with_content()))
                .await
                .map_err(|e| format!("{context}: upsert failed: {e}"))?;
            let upserted_doc: Value = upserted
                .into_model()
                .map_err(|e| format!("{context}: upsert response decode failed: {e}"))?;
            assert_roundtrip(
                &doc,
                &upserted_doc,
                &sent_canon,
                &sent_hash,
                &context,
                "upsert",
            );

            // Four point-op round-trips this config: create, read, replace, upsert.
            checked += 4;
        }

        if (iter + 1) % 100 == 0 {
            println!("... {} iterations, {checked} round-trips OK", iter + 1);
        }
    }

    println!(
        "binary_roundtrip_fuzzer: DONE — {} documents × {} configs × 4 point ops = {checked} round-trips, all canonical-equal (seed={})",
        cfg.iterations,
        configs.len(),
        cfg.seed
    );
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Calibration mode
// ─────────────────────────────────────────────────────────────────────────────

/// A numeric edge case: a human label and the **exact JSON literal** to store.
/// The literal is parsed with `serde_json` so its precise form is preserved.
struct NumberProbe {
    label: &'static str,
    literal: &'static str,
}

/// The spread of numeric forms whose backend rewrite we want to learn. These are
/// the cases the design doc (§3.1) flags as `[CALIBRATE]`.
const NUMBER_PROBES: &[NumberProbe] = &[
    NumberProbe {
        label: "integer_zero",
        literal: "0",
    },
    NumberProbe {
        label: "negative_zero",
        literal: "-0",
    },
    NumberProbe {
        label: "integral_float_1.0",
        literal: "1.0",
    },
    NumberProbe {
        label: "integral_float_20.0",
        literal: "20.0",
    },
    NumberProbe {
        label: "integral_float_exp_2e1",
        literal: "2e1",
    },
    NumberProbe {
        label: "small_fraction_0.5",
        literal: "0.5",
    },
    NumberProbe {
        label: "repeating_0.1",
        literal: "0.1",
    },
    NumberProbe {
        label: "sum_0.1_plus_0.2",
        literal: "0.30000000000000004",
    },
    NumberProbe {
        label: "high_precision_pi",
        literal: "3.141592653589793",
    },
    NumberProbe {
        label: "large_exponent",
        literal: "1e20",
    },
    NumberProbe {
        label: "small_exponent",
        literal: "1e-20",
    },
    NumberProbe {
        label: "negative_large_exp",
        literal: "-1.5e18",
    },
    NumberProbe {
        label: "i64_max",
        literal: "9223372036854775807",
    },
    NumberProbe {
        label: "i64_min",
        literal: "-9223372036854775808",
    },
    NumberProbe {
        label: "u64_max_minus_1",
        literal: "18446744073709551614",
    },
    NumberProbe {
        label: "just_above_i64",
        literal: "9223372036854775808",
    },
    NumberProbe {
        label: "trailing_zeros_1.2300",
        literal: "1.2300",
    },
    NumberProbe {
        label: "leading_int_0e0",
        literal: "0e0",
    },
];

/// **Calibration mode** (design doc §3.1): stores each numeric probe through the
/// binary path, reads it back, and prints how the backend rewrote it alongside
/// how `canonicalize_number` currently renders it. Any `DIFF` row is a number
/// form the canonicalizer does not yet model — tune `canonicalize_number` (or
/// narrow the generator) until the calibration table is all `MATCH`.
///
/// This is a **diagnostic** that prints a table; it does not assert (a `DIFF` is
/// expected the first time and is the signal to tune, not a test failure). Run
/// it with `AZURE_COSMOS_FUZZ_CALIBRATE=true` against a live account.
async fn run_calibration() -> Result<(), Box<dyn Error>> {
    println!("binary_roundtrip_fuzzer: CALIBRATION MODE — learning the backend's number rewrite");
    println!("(store each probe via binary encoding, read back, compare canonical forms)\n");

    // Use the binary config so the full encode→store→decode path is exercised.
    let client = build_client(&Some(BinaryEncodingOptions::new().with_enabled(true))).await?;

    let database_name =
        std::env::var(DATABASE_NAME_ENV_VAR).unwrap_or_else(|_| DEFAULT_DATABASE_NAME.to_string());
    let container_name = std::env::var(CONTAINER_NAME_ENV_VAR)
        .unwrap_or_else(|_| DEFAULT_CONTAINER_NAME.to_string());

    ignore_conflict(client.create_database(&database_name, None).await)?;
    let db = client.database_client(&database_name);
    ignore_conflict(
        db.create_container(
            ContainerProperties::new(container_name.clone(), PARTITION_KEY_PATH.into()),
            None,
        )
        .await,
    )?;
    let container = db.container_client(&container_name).await?;

    println!(
        "{:<26} {:<24} {:<24} {:<24} {}",
        "probe", "sent-literal", "our-canonical", "backend-returned", "status"
    );
    println!("{}", "-".repeat(120));

    let mut diffs = 0u32;
    for probe in NUMBER_PROBES {
        // Parse the exact literal (skip probes serde_json cannot represent).
        let Ok(number_value) = serde_json::from_str::<Value>(probe.literal) else {
            println!(
                "{:<26} {:<24} (serde_json cannot parse this literal)",
                probe.label, probe.literal
            );
            continue;
        };

        let id = Uuid::new_v4().to_string();
        let pk = "calibration".to_string();
        let doc = serde_json::json!({ "id": id, "pk": pk, "n": number_value });

        container
            .create_item(&pk, &id, &doc, Some(write_options_with_content()))
            .await
            .map_err(|e| format!("{}: create failed: {e}", probe.label))?;
        let read = container
            .read_item(&pk, &id, None)
            .await
            .map_err(|e| format!("{}: read failed: {e}", probe.label))?;
        let read_doc: Value = read
            .into_model()
            .map_err(|e| format!("{}: read decode failed: {e}", probe.label))?;

        let returned_n = read_doc.get("n").cloned().unwrap_or(Value::Null);
        // The backend's raw JSON text rendering of the number.
        let backend_returned = serde_json::to_string(&returned_n).unwrap_or_default();
        // How our canonicalizer renders the sent value vs the returned value.
        let (our_canonical, _) = canonical_hash(&number_value);
        let (returned_canonical, _) = canonical_hash(&returned_n);

        let status = if our_canonical == returned_canonical {
            "MATCH"
        } else {
            diffs += 1;
            "DIFF  <-- tune normalize_number"
        };

        println!(
            "{:<26} {:<24} {:<24} {:<24} {}",
            probe.label, probe.literal, our_canonical, backend_returned, status
        );
    }

    println!("{}", "-".repeat(120));
    if diffs == 0 {
        println!("CALIBRATION: all probes MATCH — normalize_number models the backend rewrite.");
    } else {
        println!(
            "CALIBRATION: {diffs} probe(s) DIFF — update `normalize_number` to match the backend-returned column above."
        );
    }
    Ok(())
}

/// Asserts the returned document, projected to the sent keys, canonicalizes to
/// the same form (and hash) as what was sent. On mismatch, prints both canonical
/// forms and the reproduction seed.
fn assert_roundtrip(
    sent: &Map<String, Value>,
    got: &Value,
    sent_canon: &str,
    sent_hash: &[u8; 32],
    context: &str,
    phase: &str,
) {
    let projected = project_to_sent_keys(sent, got);
    let (got_canon, got_hash) = canonical_hash(&projected);
    if &got_hash != sent_hash || got_canon != sent_canon {
        panic!(
            "{context}: {phase} round-trip MISMATCH\n  sent  (sha256 {}): {sent_canon}\n  got   (sha256 {}): {got_canon}\n  reproduce with {SEED_ENV_VAR} from the context above",
            hex(sent_hash),
            hex(&got_hash),
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
        canonicalize(value)
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
    fn canonicalize_large_unsigned_integer_matches_backend_double() {
        // Calibrated (§3.1): the backend stores integers above i64::MAX as
        // doubles and returns them in scientific notation, so the canonicalizer
        // models that — a large u64 canonicalizes identically to the double form
        // the backend returns. Because RFC 8785 (JCS) refuses to emit integers
        // beyond 2^53, these are compared as stable string tokens (of the f64),
        // which keeps sent and round-tripped values comparable.
        let sent_u64: Value = serde_json::from_str("18446744073709551614").unwrap();
        let backend_double: Value = serde_json::from_str("1.8446744073709552e+19").unwrap();
        assert_eq!(canon(&sent_u64), canon(&backend_double));

        // 2^63 (just above i64::MAX) behaves the same way.
        let sent_2p63: Value = serde_json::from_str("9223372036854775808").unwrap();
        let backend_2p63: Value = serde_json::from_str("9.223372036854776e+18").unwrap();
        assert_eq!(canon(&sent_2p63), canon(&backend_2p63));

        // i64::MAX exceeds the JCS-safe integer range, so it canonicalizes to
        // an exact decimal string token (quoted by JCS), not a bare number.
        let i64_max: Value = serde_json::from_str("9223372036854775807").unwrap();
        assert_eq!(canon(&i64_max), r#""9223372036854775807""#);

        // A JCS-safe integer stays a bare number.
        assert_eq!(canon(&serde_json::json!(1_000_000)), "1000000");
    }

    #[test]
    fn normalize_numbers_rewrites_every_number_in_the_tree() {
        // The Cosmos-calibrated number rewrite applies recursively through
        // arrays and nested objects, leaving non-number values untouched.
        let input = serde_json::json!({
            "int": 5,
            "integral_float": 1.0,
            "fraction": 2.5,
            "arr": [1.0, 2.0, 3.5],
            "nested": { "big": 18446744073709551614u64, "s": "x", "b": true, "n": null }
        });
        let out = normalize_numbers(&input);

        // Integral floats collapse to integers; fractions stay; big u64 → f64.
        assert_eq!(out["int"], serde_json::json!(5));
        assert_eq!(out["integral_float"], serde_json::json!(1));
        assert_eq!(out["fraction"], serde_json::json!(2.5));
        assert_eq!(out["arr"], serde_json::json!([1, 2, 3.5]));
        assert_eq!(
            out["nested"]["big"],
            normalize_number(&serde_json::from_str::<Number>("18446744073709551614").unwrap())
        );
        // Non-number leaves pass through unchanged.
        assert_eq!(out["nested"]["s"], serde_json::json!("x"));
        assert_eq!(out["nested"]["b"], serde_json::json!(true));
        assert_eq!(out["nested"]["n"], Value::Null);
    }

    #[test]
    fn normalize_numbers_is_idempotent() {
        // Applying the rewrite twice yields the same tree — a prerequisite for
        // comparing a normalized sent doc against a normalized returned doc.
        let input = serde_json::json!({
            "a": 1.0, "b": [2.0, 3.5, 18446744073709551614u64], "c": { "d": 9223372036854775807i64 }
        });
        let once = normalize_numbers(&input);
        let twice = normalize_numbers(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn canonical_hash_is_stable_and_matches_json_canon() {
        // The digest is a deterministic function of the canonical string, and
        // the canonical string is the JCS form of the number-normalized value.
        let v = serde_json::json!({ "b": 1.0, "a": [2.0, 3.5], "c": "x" });
        let (s1, h1) = canonical_hash(&v);
        let (s2, h2) = canonical_hash(&v);
        assert_eq!(s1, s2);
        assert_eq!(h1, h2);

        // Structural equivalence (key order / whitespace / integral floats) maps
        // to the same digest.
        let equiv = serde_json::from_str::<Value>(r#" { "c":"x", "a":[2,3.5], "b":1 } "#).unwrap();
        let (_, h_equiv) = canonical_hash(&equiv);
        assert_eq!(h1, h_equiv);
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
            breadth: DEFAULT_BREADTH,
            calibrate: false,
            print_docs: false,
        };
        let mut a = SplitMix64::new(cfg.seed);
        let mut b = SplitMix64::new(cfg.seed);
        let doc_a = Value::Object(gen_object(&mut a, &cfg));
        let doc_b = Value::Object(gen_object(&mut b, &cfg));
        assert_eq!(canon(&doc_a), canon(&doc_b));
    }

    /// Nesting depth of a JSON value (scalars are depth 0).
    fn depth_of(v: &Value) -> u32 {
        match v {
            Value::Array(items) => 1 + items.iter().map(depth_of).max().unwrap_or(0),
            Value::Object(map) => 1 + map.values().map(depth_of).max().unwrap_or(0),
            _ => 0,
        }
    }

    #[test]
    fn generator_depth_scales_with_max_depth() {
        // Guards the hybrid-skeleton generator: the average nesting depth must
        // grow with `max_depth` (the old arbitrary-json-only generator was flat
        // at ~1.3 regardless of the knob). We assert a conservative lower bound
        // on the average and that the deepest doc reaches near the target.
        fn avg_and_max_depth(max_depth: u32) -> (f64, u32) {
            let cfg = FuzzConfig {
                iterations: 0,
                seed: 1784944014111583800,
                max_depth,
                wide_numbers: false,
                unicode: true,
                breadth: DEFAULT_BREADTH,
                calibrate: false,
                print_docs: false,
            };
            let mut rng = SplitMix64::new(cfg.seed);
            let n = 1000u32;
            let mut sum = 0u64;
            let mut max_seen = 0u32;
            for _ in 0..n {
                let d = depth_of(&Value::Object(gen_object(&mut rng, &cfg)));
                sum += d as u64;
                max_seen = max_seen.max(d);
            }
            (sum as f64 / n as f64, max_seen)
        }

        let (avg3, max3) = avg_and_max_depth(3);
        let (avg8, max8) = avg_and_max_depth(8);

        // Depth clearly scales with the knob (not flat like the old generator).
        assert!(
            avg8 > avg3 + 1.0,
            "avg depth should grow with max_depth: avg@3={avg3:.2}, avg@8={avg8:.2}"
        );
        // The deepest documents actually approach the requested depth.
        assert!(
            max3 >= 3,
            "max depth @3 should reach the target, got {max3}"
        );
        assert!(
            max8 >= 8,
            "max depth @8 should reach the target, got {max8}"
        );
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
            breadth: DEFAULT_BREADTH,
            calibrate: false,
            print_docs: false,
        };
        let mut rng = SplitMix64::new(cfg.seed);
        for _ in 0..500 {
            let doc = Value::Object(gen_object(&mut rng, &cfg));
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

    #[test]
    fn generated_documents_cover_all_value_categories() {
        // The sampler subtree guarantees every value category appears in each
        // document: integer, float, alphabetic, alphanumeric, non-ASCII string,
        // boolean, null, a number array, and multi-level nesting. This asserts
        // the "really complex JSON" contract holds for a spread of seeds.
        fn walk(v: &Value, seen: &mut Categories, max_depth: &mut u32, depth: u32) {
            *max_depth = (*max_depth).max(depth);
            match v {
                Value::Null => seen.null = true,
                Value::Bool(_) => seen.boolean = true,
                Value::Number(n) => {
                    if n.is_f64() {
                        seen.float = true;
                    } else {
                        seen.integer = true;
                    }
                }
                Value::String(s) => {
                    if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic()) {
                        seen.alphabetic = true;
                    }
                    if !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric()) {
                        seen.alphanumeric = true;
                    }
                    if !s.is_ascii() {
                        seen.non_ascii = true;
                    }
                }
                Value::Array(items) => {
                    seen.array = true;
                    for item in items {
                        walk(item, seen, max_depth, depth + 1);
                    }
                }
                Value::Object(map) => {
                    seen.object = true;
                    for child in map.values() {
                        walk(child, seen, max_depth, depth + 1);
                    }
                }
            }
        }

        #[derive(Default)]
        struct Categories {
            integer: bool,
            float: bool,
            alphabetic: bool,
            alphanumeric: bool,
            non_ascii: bool,
            boolean: bool,
            null: bool,
            array: bool,
            object: bool,
        }

        let cfg = FuzzConfig {
            iterations: 0,
            seed: 0xC0FFEE,
            max_depth: 6,
            wide_numbers: false,
            unicode: true,
            breadth: DEFAULT_BREADTH,
            calibrate: false,
            print_docs: false,
        };
        let mut rng = SplitMix64::new(cfg.seed);

        let mut all = Categories::default();
        let mut deepest = 0u32;
        for _ in 0..50 {
            let doc = Value::Object(gen_object(&mut rng, &cfg));
            walk(&doc, &mut all, &mut deepest, 0);
        }

        assert!(all.integer, "no integer produced");
        assert!(all.float, "no float produced");
        assert!(all.alphabetic, "no alphabetic string produced");
        assert!(all.alphanumeric, "no alphanumeric string produced");
        assert!(all.non_ascii, "no non-ASCII string produced");
        assert!(all.boolean, "no boolean produced");
        assert!(all.null, "no null produced");
        assert!(all.array, "no array produced");
        assert!(all.object, "no nested object produced");
        // Multi-level nesting: the guaranteed sampler alone reaches depth ≥ 3,
        // and the spine pushes documents deeper.
        assert!(
            deepest >= 4,
            "documents should reach multi-level nesting, deepest={deepest}"
        );
    }

    /// Prints a few sample generated documents as pretty JSON so the generator
    /// output can be eyeballed **offline** (no Cosmos account). Ignored by
    /// default; run explicitly with `--ignored --nocapture`:
    ///
    /// ```bash
    /// cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer \
    ///   print_sample_documents -- --ignored --nocapture
    /// ```
    ///
    /// Control shape/size via the same env vars as a live run, e.g.
    /// `AZURE_COSMOS_FUZZ_SEED`, `AZURE_COSMOS_FUZZ_MAX_DEPTH`,
    /// `AZURE_COSMOS_FUZZ_BREADTH`, `AZURE_COSMOS_FUZZ_WIDE_NUMBERS`; the count
    /// defaults to 3 (override with `AZURE_COSMOS_FUZZ_PRINT_COUNT`).
    #[test]
    #[ignore = "prints sample JSON on demand; run with --ignored --nocapture"]
    fn print_sample_documents() {
        let cfg = FuzzConfig::from_env();
        let count = std::env::var("AZURE_COSMOS_FUZZ_PRINT_COUNT")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(3);
        println!(
            "print_sample_documents: seed={} max_depth={} breadth={} wide_numbers={} unicode={}",
            cfg.seed, cfg.max_depth, cfg.breadth, cfg.wide_numbers, cfg.unicode
        );
        let mut rng = SplitMix64::new(cfg.seed);
        for i in 0..count {
            let doc = Value::Object(gen_object(&mut rng, &cfg));
            println!(
                "--- sample {i} ---\n{}",
                serde_json::to_string_pretty(&doc).unwrap()
            );
        }
    }

    #[test]
    fn calibration_probes_are_valid_and_unique() {
        // Every calibration probe literal must parse as a JSON number, and the
        // labels must be unique (they key the printed calibration table).
        let mut labels: Vec<&str> = Vec::new();
        for probe in NUMBER_PROBES {
            let value: Value = serde_json::from_str(probe.literal).unwrap_or_else(|e| {
                panic!(
                    "probe {} literal {:?} invalid: {e}",
                    probe.label, probe.literal
                )
            });
            assert!(
                value.is_number(),
                "probe {} literal {:?} is not a JSON number",
                probe.label,
                probe.literal
            );
            labels.push(probe.label);
        }
        labels.sort_unstable();
        let count = labels.len();
        labels.dedup();
        assert_eq!(labels.len(), count, "duplicate calibration probe label");
    }
}
