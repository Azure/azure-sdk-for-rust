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
//!   cargo test -p azure_data_cosmos --test binary_roundtrip_fuzzer --features key_auth,fault_injection,control_plane -- --nocapture
//!
//! # Multi-day soak (millions of docs), release build:
//! AZURE_COSMOS_CONNECTION_STRING='...' AZURE_COSMOS_FUZZ_ITERATIONS=5000000 \
//! RUSTFLAGS='--cfg test_category="binary_encoding"' \
//!   cargo test -p azure_data_cosmos --test binary_roundtrip_fuzzer --features key_auth,fault_injection,control_plane --release -- --nocapture
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
use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::{
    BinaryEncodingOptions, ConnectionPoolOptions, ContentResponseOnWrite, ItemWriteOptions,
    OperationOptions, Region, ServerCertificateValidation,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, FeedScope, Query,
    RoutingStrategy, SubStatusCode,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
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
const SHAPE_RATIO_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_SHAPE_RATIO";
const SIZE_SCALE_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_SIZE_SCALE";
const CALIBRATE_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_CALIBRATE";
const PRINT_ENV_VAR: &str = "AZURE_COSMOS_FUZZ_PRINT";

const DEFAULT_DATABASE_NAME: &str = "binary-fuzz-db";
const DEFAULT_CONTAINER_NAME: &str = "binary-fuzz-ct";
const PARTITION_KEY_PATH: &str = "/pk";

const DEFAULT_ITERATIONS: u64 = 180;
const DEFAULT_MAX_DEPTH: u32 = 6;
/// Upper bound for `max_depth`: the generator recurses per level, so an
/// unbounded value would stack-overflow. 64 is safe on an ordinary stack.
const MAX_DEPTH_LIMIT: u32 = 64;
/// Default maximum number of child fields/elements generated at each container
/// level (the branching factor). Higher values produce larger, wider documents.
const DEFAULT_BREADTH: u32 = 6;
/// Upper bound for `breadth`: each level allocates up to this many children.
const BREADTH_LIMIT: u32 = 1024;
/// Default percent (0-100) of generated documents built in the shape of a real
/// corpus file (see `SHAPE_SAMPLERS`); the rest are free-form hybrid documents.
const DEFAULT_SHAPE_RATIO: u32 = 85;
/// Default multiplier applied to the internal array / collection sizes of the
/// corpus shape samplers. `1` keeps documents compact; larger values grow the
/// per-item payload (e.g. embedding-vector dimensions, nutrient / member /
/// keyword / similars arrays) toward corpus-scale sizes.
const DEFAULT_SIZE_SCALE: u32 = 1;
/// Upper bound for `size_scale`: multiplies collection sizes.
const SIZE_SCALE_LIMIT: u32 = 1024;

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
    shape_ratio: u32,
    size_scale: u32,
    calibrate: bool,
    print_docs: bool,
}

impl FuzzConfig {
    fn from_env() -> Self {
        let seed = match std::env::var(SEED_ENV_VAR) {
            // If set, it must parse — don't silently randomize (breaks reproduction).
            Ok(v) => v.trim().parse::<u64>().unwrap_or_else(|_| {
                panic!(
                    "{SEED_ENV_VAR} is set to {v:?} but is not a valid u64 seed; \
                     provide a decimal u64 (e.g. 12345) or unset it to use a random seed"
                )
            }),
            // Unset: random seed from the wall clock.
            Err(_) => std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x1234_5678_9ABC_DEF0),
        };
        Self {
            // A zero count would pass without exercising anything.
            iterations: {
                let n = env_u64(ITERATIONS_ENV_VAR, DEFAULT_ITERATIONS);
                assert!(n > 0, "{ITERATIONS_ENV_VAR} must be greater than 0");
                n
            },
            seed,
            max_depth: env_u32(MAX_DEPTH_ENV_VAR, DEFAULT_MAX_DEPTH, 1, MAX_DEPTH_LIMIT),
            wide_numbers: env_bool(WIDE_NUMBERS_ENV_VAR, false),
            unicode: env_bool(UNICODE_ENV_VAR, true),
            breadth: env_u32(BREADTH_ENV_VAR, DEFAULT_BREADTH, 1, BREADTH_LIMIT),
            shape_ratio: env_u64(SHAPE_RATIO_ENV_VAR, DEFAULT_SHAPE_RATIO as u64).min(100) as u32,
            size_scale: env_u32(SIZE_SCALE_ENV_VAR, DEFAULT_SIZE_SCALE, 1, SIZE_SCALE_LIMIT),
            calibrate: env_bool(CALIBRATE_ENV_VAR, false),
            print_docs: env_bool(PRINT_ENV_VAR, false),
        }
    }
}

/// Reads a `u64` knob, **panicking** on a malformed value: a typo in a CI
/// variable must fail the run, not silently change what the fuzzer covers.
fn env_u64(name: &str, default: u64) -> u64 {
    match std::env::var(name) {
        Ok(raw) => raw.trim().parse::<u64>().unwrap_or_else(|_| {
            panic!("{name} must be a non-negative integer, got {raw:?}");
        }),
        Err(_) => default,
    }
}

/// Reads a `u32` knob within `[min, max]`: below `min` is raised to `min`,
/// above `max` **panics** (a checked conversion, never a silent `as u32` wrap
/// such as a multiple of `2^32` truncating to `0`). Bounds recursion/allocation
/// so a typo'd CI value fails fast instead of stack-overflowing or OOMing.
fn env_u32(name: &str, default: u32, min: u32, max: u32) -> u32 {
    let raw = env_u64(name, u64::from(default));
    assert!(
        raw <= u64::from(max),
        "{name} must not exceed {max}, got {raw}"
    );
    raw.max(u64::from(min)) as u32
}

/// Reads a boolean knob, accepting `1`/`0`, `yes`/`no`, `on`/`off` alongside
/// `true`/`false`. **Panics** on anything else so a mistyped value cannot
/// silently disable coverage.
fn env_bool(name: &str, default: bool) -> bool {
    match std::env::var(name) {
        Ok(raw) => match raw.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => true,
            "false" | "0" | "no" | "off" => false,
            _ => panic!("{name} must be a boolean (true/false/1/0/yes/no/on/off), got {raw:?}"),
        },
        Err(_) => default,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// JSON generator (in-tree ArbitraryValue, seeded from the PRNG)
// ─────────────────────────────────────────────────────────────────────────────

/// Generates a random JSON **object** suitable as a Cosmos item body.
///
/// Uses a **hybrid** strategy: a depth-controlled *skeleton* guarantees the
/// document actually reaches a target nesting depth (drawn from `[1, max_depth]`),
/// while every leaf and filler branch is irregular JSON — a mix of hand-rolled
/// typed scalars (integers, floats, alphabetic / alphanumeric / free-text /
/// non-ASCII strings, booleans, nulls, number arrays) and [`ArbitraryValue`]
/// subtrees. This fixes the shallowness of the raw `arbitrary`-driven descent
/// (it stops nesting almost immediately regardless of byte budget), so
/// `max_depth` now
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
    // A configurable fraction of documents are built in the shape of a real
    // corpus file (see SHAPE_SAMPLERS), so a run resembles the service corpus.
    // The rest are the free-form hybrid documents below. The `_sampler` subtree
    // is attached in both cases so every document still covers all categories.
    let shaped = cfg.shape_ratio > 0 && rng.below(100) < cfg.shape_ratio as u64;
    if shaped {
        let mut map = gen_shaped_document(rng, cfg);
        // Ensure the all-category sampler is present without clobbering a
        // shape field of the same name (shapes never use `_sampler`).
        map.insert("_sampler".to_string(), gen_sampler(rng, cfg));
        return map;
    }

    let max_depth = cfg.max_depth.max(1);
    // Target nesting depth for this document's spine, in [1, max_depth].
    let target_depth = 1 + rng.below(max_depth as u64) as u32;

    let mut map = Map::new();
    // A guaranteed sampler covering every value category (numeric, alphabetic,
    // alphanumeric, free text, non-ASCII, boolean, null, number array, nested).
    // Keyed distinctly from the caller-reserved `id`/`pk` so it is never
    // overwritten.
    map.insert("_sampler".to_string(), gen_sampler(rng, cfg));
    // A spread of irregular root fields (typed scalars + ArbitraryValue subtrees).
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

/// Number of PRNG bytes fed to the value generator for one filler subtree. A
/// larger budget lets it build bigger, deeper irregular subtrees.
const FILLER_BUDGET: usize = 256;

/// Refills `n` bytes deterministically from the PRNG.
fn fill_bytes(rng: &mut SplitMix64, n: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(n);
    while bytes.len() < n {
        bytes.extend_from_slice(&rng.next_u64().to_le_bytes());
    }
    bytes
}

/// A random object key from an entropy-driven string generator.
fn gen_key(rng: &mut SplitMix64) -> String {
    let bytes = fill_bytes(rng, 16);
    let mut u = Unstructured::new(&bytes);
    String::arbitrary(&mut u).unwrap_or_default()
}

/// Builds a `serde_json::Value` from raw entropy.
///
/// Replaces the WTFPL-licensed `arbitrary-json` crate (not on the workspace
/// license allow-list, and `cargo deny` audits dev-dependencies of a published
/// crate). A depth-bounded recursive descent over the JSON kinds, driven by the
/// [`Unstructured`] byte stream, so it stays deterministic for a given seed.
struct ArbitraryValue(Value);

impl From<ArbitraryValue> for Value {
    fn from(v: ArbitraryValue) -> Self {
        v.0
    }
}

impl<'a> Arbitrary<'a> for ArbitraryValue {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(ArbitraryValue(arbitrary_value(u, 4)))
    }
}

/// Maximum children drawn for a single array or object node.
const ARBITRARY_FANOUT: u8 = 8;

fn arbitrary_value(u: &mut Unstructured<'_>, depth: u32) -> Value {
    // Once depth or entropy is exhausted, only leaf kinds are produced.
    let leaf = depth == 0 || u.is_empty();
    let kinds = if leaf { 4 } else { 6 };
    match u.arbitrary::<u8>().unwrap_or(0) % kinds {
        0 => Value::Null,
        1 => Value::Bool(u.arbitrary().unwrap_or(false)),
        2 => match u.arbitrary::<u8>().unwrap_or(0) % 3 {
            0 => Value::from(u.arbitrary::<i64>().unwrap_or(0)),
            1 => Value::from(u.arbitrary::<u64>().unwrap_or(0)),
            _ => {
                let f = u.arbitrary::<f64>().unwrap_or(0.0);
                serde_json::Number::from_f64(if f.is_finite() { f } else { 0.0 })
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
        },
        3 => Value::String(String::arbitrary(u).unwrap_or_default()),
        4 => {
            let len = u.arbitrary::<u8>().unwrap_or(0) % ARBITRARY_FANOUT;
            let mut arr = Vec::with_capacity(len as usize);
            for _ in 0..len {
                arr.push(arbitrary_value(u, depth - 1));
            }
            Value::Array(arr)
        }
        _ => {
            let len = u.arbitrary::<u8>().unwrap_or(0) % ARBITRARY_FANOUT;
            let mut map = Map::new();
            for _ in 0..len {
                map.insert(
                    String::arbitrary(u).unwrap_or_default(),
                    arbitrary_value(u, depth - 1),
                );
            }
            Value::Object(map)
        }
    }
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
/// string-token comparison path (design doc §3.1). The wide branch spans signed
/// i64, true u64 above `i64::MAX`, and a wide non-integral float — all of which
/// the backend stores as lossy doubles, so [`normalize_number`] tokenizes each
/// via its rounded `f64`.
fn gen_number(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Value {
    if cfg.wide_numbers && rng.below(4) == 0 {
        return match rng.below(3) {
            // Signed wide integer (> 2^53) → lossy double-token path.
            0 => Value::Number(Number::from(rng.next_u64() as i64)),
            // True u64 above i64::MAX → double-token path.
            1 => Value::Number(Number::from((i64::MAX as u64) + 1 + (rng.next_u64() >> 1))),
            // Wide non-integral float → double-token path.
            _ => {
                let scaled = (rng.next_u64() >> 8) as f64 * 1.000_000_1;
                Number::from_f64(scaled)
                    .map(Value::Number)
                    .unwrap_or_else(|| Value::Number(Number::from(rng.next_u64() as i64)))
            }
        };
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

// ─────────────────────────────────────────────────────────────────────────────
// Domain-flavored scalar helpers (used by the corpus shape samplers below).
// ─────────────────────────────────────────────────────────────────────────────

/// Lowercase hex string of `len` nibbles (e.g. hashes, GUIDs-as-hex).
fn gen_hex(rng: &mut SplitMix64, len: usize) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    (0..len)
        .map(|_| HEX[rng.below(16) as usize] as char)
        .collect()
}

/// Uppercase-hex string of `len` nibbles (e.g. GUID-like ids without dashes).
fn gen_hex_upper(rng: &mut SplitMix64, len: usize) -> String {
    gen_hex(rng, len).to_ascii_uppercase()
}

/// A canonical `8-4-4-4-12` UUID string (lowercase hex).
fn gen_uuid(rng: &mut SplitMix64) -> String {
    format!(
        "{}-{}-{}-{}-{}",
        gen_hex(rng, 8),
        gen_hex(rng, 4),
        gen_hex(rng, 4),
        gen_hex(rng, 4),
        gen_hex(rng, 12)
    )
}

/// An ISO-8601-ish timestamp string, optionally with fractional seconds and a
/// UTC `Z` suffix. Deterministic from the PRNG (not the wall clock).
fn gen_iso_datetime(rng: &mut SplitMix64) -> String {
    let year = gen_int_in(rng, 1990, 2024);
    let month = gen_int_in(rng, 1, 12);
    let day = gen_int_in(rng, 1, 28);
    let hour = gen_int_in(rng, 0, 23);
    let min = gen_int_in(rng, 0, 59);
    let sec = gen_int_in(rng, 0, 59);
    match rng.below(3) {
        0 => format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}"),
        1 => format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}.{:07}Z",
            gen_int_in(rng, 0, 9_999_999)
        ),
        _ => format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}:{sec:02}"),
    }
}

/// A short uppercase alphabetic code (e.g. airport / entity codes).
fn gen_code(rng: &mut SplitMix64, len: usize) -> String {
    const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (0..len)
        .map(|_| UPPER[rng.below(26) as usize] as char)
        .collect()
}

/// A signed geographic coordinate (`±180`) with high precision, as an `f64`.
fn gen_coordinate(rng: &mut SplitMix64) -> Value {
    let scaled = gen_int_in(rng, -180_000_000, 180_000_000);
    Number::from_f64(scaled as f64 / 1_000_000.0)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

/// A tiny normalized floating value in `[-1, 1]` (e.g. embedding components,
/// similarity scores) with many significant digits.
fn gen_unit_float(rng: &mut SplitMix64) -> Value {
    let scaled = gen_int_in(rng, -100_000_000, 100_000_000);
    Number::from_f64(scaled as f64 / 100_000_000.0)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}

/// A short human-ish name from a small fixed pool (keeps documents readable and
/// deterministic without a names dictionary).
fn gen_name(rng: &mut SplitMix64) -> String {
    const NAMES: &[&str] = &[
        "Casual",
        "Joe Flacco",
        "Emmanuel",
        "Aruba",
        "Gary Stevens",
        "Aachen",
        "Atlanta",
        "Chenault",
        "Millett",
        "Coffee",
        "Volcano",
        "Xpert",
        "Reddit",
        "Bitcoin",
    ];
    NAMES[rng.below(NAMES.len() as u64) as usize].to_string()
}

/// A GeoJSON linear-ring of `n` `[lon, lat]` coordinate pairs.
fn gen_coord_ring(rng: &mut SplitMix64, n: usize) -> Value {
    Value::Array(
        (0..n)
            .map(|_| Value::Array(vec![gen_coordinate(rng), gen_coordinate(rng)]))
            .collect(),
    )
}

/// `n` items produced by `f`, as a JSON array. `n` is computed **before** the
/// call to avoid double-borrowing `rng`.
fn gen_array_of<F>(rng: &mut SplitMix64, n: usize, mut f: F) -> Value
where
    F: FnMut(&mut SplitMix64) -> Value,
{
    Value::Array((0..n).map(|_| f(rng)).collect())
}

/// A random, **size-scaled** count in `[min, min+span)` × `cfg.size_scale` — a
/// small helper so array builders can pick their length in a `let` binding
/// (keeping the `gen_array_of` call from borrowing `rng` twice in one
/// expression). The `size_scale` knob multiplies every corpus-shape collection
/// length, so a run can grow per-item payloads toward corpus-scale sizes.
fn count(rng: &mut SplitMix64, cfg: &FuzzConfig, min: usize, span: u64) -> usize {
    let scale = cfg.size_scale.max(1) as usize;
    (min + rng.below(span) as usize) * scale
}

// ─────────────────────────────────────────────────────────────────────────────
// Corpus shape samplers.
//
// Each `shape_*` produces a single document (a `Map`) matching the structural
// "shape" of one of the local `testdata/*.json` corpus files, populated with
// randomized alphanumeric / numeric / float / non-ASCII / boolean / null /
// datetime / GUID / coordinate values. The generator can emit any of these
// shapes at random (see [`gen_shaped_document`]), so a run resembles the real
// service corpus while staying fully synthetic and seed-reproducible.
//
// These reproduce the *shape*, not verbatim data — no corpus bytes are embedded.
// ─────────────────────────────────────────────────────────────────────────────

/// `airline-delays-2003-2016.json`: nested Airport/Time/Statistics record.
fn shape_airline_delays(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut airport = Map::new();
    airport.insert("Code".into(), Value::String(gen_code(rng, 3)));
    airport.insert(
        "Name".into(),
        Value::String(gen_unicode_string(rng, cfg, 40)),
    );
    let mut time = Map::new();
    time.insert(
        "Label".into(),
        Value::String(format!(
            "{:04}/{:02}",
            gen_int_in(rng, 2003, 2016),
            gen_int_in(rng, 1, 12)
        )),
    );
    time.insert(
        "Month".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 12))),
    );
    time.insert(
        "Year".into(),
        Value::Number(Number::from(gen_int_in(rng, 2003, 2016))),
    );
    let mut stats = Map::new();
    stats.insert(
        "Delayed".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 100_000))),
    );
    stats.insert("OnTime".into(), gen_envelope_float(rng));
    let mut m = Map::new();
    m.insert("Airport".into(), Value::Object(airport));
    m.insert("Time".into(), Value::Object(time));
    m.insert("Statistics".into(), Value::Object(stats));
    m
}

/// `bitcoin_transactions.json`: flat transaction with hash + numeric fields.
fn shape_bitcoin(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("hash".into(), Value::String(gen_hex(rng, 64)));
    m.insert(
        "ver".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 2))),
    );
    m.insert(
        "vin_sz".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 10))),
    );
    m.insert(
        "vout_sz".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 10))),
    );
    m.insert(
        "size".into(),
        Value::Number(Number::from(gen_int_in(rng, 100, 5000))),
    );
    m.insert(
        "fee".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 100_000))),
    );
    m.insert("relayed_by".into(), Value::String("0.0.0.0".into()));
    m
}

/// `CombinedBingDocs.json`: blog post with a structured `postTime`.
fn shape_bing_docs(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut post_time = Map::new();
    post_time.insert(
        "Month".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 12))),
    );
    post_time.insert(
        "Day".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 28))),
    );
    post_time.insert(
        "Year".into(),
        Value::Number(Number::from(gen_int_in(rng, 1600, 2024))),
    );
    post_time.insert(
        "Hour".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 23))),
    );
    post_time.insert(
        "Minute".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 59))),
    );
    let mut m = Map::new();
    m.insert("blogId".into(), Value::String(gen_code(rng, 9)));
    m.insert(
        "blogName".into(),
        Value::String(gen_string_from(rng, ALPHA_CHARS, 12)),
    );
    m.insert("postId".into(), Value::String(gen_code(rng, 9)));
    m.insert("postTitle".into(), Value::String(gen_code(rng, 32)));
    m.insert("authorName".into(), Value::String(gen_name(rng)));
    m.insert("postTime".into(), Value::Object(post_time));
    m.insert(
        "body".into(),
        Value::String(gen_unicode_string(rng, cfg, 120)),
    );
    m
}

/// `CombinedScriptsData.Json`: entity with a `from` object + `actions` array.
fn shape_scripts_data(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut from = Map::new();
    from.insert("name".into(), Value::String(gen_name(rng)));
    from.insert(
        "id".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 1000))),
    );
    let n = count(rng, cfg, 1, 4);
    let actions = gen_array_of(rng, n, |_r| {
        let mut a = Map::new();
        a.insert("name".into(), Value::String("Comment".into()));
        a.insert(
            "link".into(),
            Value::String("http://www.facebook.com/X999/posts/Y999".into()),
        );
        Value::Object(a)
    });
    let mut m = Map::new();
    m.insert("entityId".into(), Value::String(gen_code(rng, 9)));
    m.insert("from".into(), Value::Object(from));
    m.insert(
        "message".into(),
        Value::String(gen_unicode_string(rng, cfg, 60)),
    );
    m.insert("actions".into(), actions);
    m
}

/// `countries.json` / `Volcanoes.json`: a GeoJSON `Feature` with geometry.
fn shape_geojson_feature(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut props = Map::new();
    props.insert("ADMIN".into(), Value::String(gen_name(rng)));
    props.insert("ISO_A3".into(), Value::String(gen_code(rng, 3)));
    props.insert(
        "POP".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 1_000_000))),
    );
    props.insert(
        "NOTE".into(),
        Value::String(gen_unicode_string(rng, cfg, 30)),
    );
    let mut geometry = Map::new();
    if rng.below(2) == 0 {
        geometry.insert("type".into(), Value::String("Point".into()));
        geometry.insert(
            "coordinates".into(),
            Value::Array(vec![gen_coordinate(rng), gen_coordinate(rng)]),
        );
    } else {
        geometry.insert("type".into(), Value::String("Polygon".into()));
        let ring_len = count(rng, cfg, 3, 5);
        geometry.insert(
            "coordinates".into(),
            Value::Array(vec![gen_coord_ring(rng, ring_len)]),
        );
    }
    let mut m = Map::new();
    m.insert("type".into(), Value::String("Feature".into()));
    m.insert("properties".into(), Value::Object(props));
    m.insert("geometry".into(), Value::Object(geometry));
    m
}

/// `devtestcoll.json` / `runsCollection.json`: Cosmos-run metadata record with
/// `id`, state strings, and ISO timestamps. (The corpus files also carry a
/// `_self` resource link, but that is a Cosmos-reserved system property the
/// service owns; we model it as a non-reserved `resourceLink` string instead so
/// the document round-trips cleanly.)
fn shape_cosmos_run(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    const STATES: &[&str] = &["InProgress", "Completed", "Failed", "Queued"];
    let mut m = Map::new();
    m.insert("id".into(), Value::String(gen_hex(rng, 7)));
    m.insert(
        "resourceLink".into(),
        Value::String(format!(
            "dbs/{}==/colls/{}=/docs/{}==/",
            gen_code(rng, 6),
            gen_code(rng, 9),
            gen_code(rng, 20)
        )),
    );
    m.insert(
        "RunState".into(),
        Value::String(STATES[rng.below(STATES.len() as u64) as usize].into()),
    );
    m.insert("RunResult".into(), Value::String("Failed".into()));
    m.insert(
        "FederationName".into(),
        Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 24)),
    );
    m.insert("StartTime".into(), Value::String(gen_iso_datetime(rng)));
    m.insert("CompletedTime".into(), Value::String(gen_iso_datetime(rng)));
    m
}

/// `earth-meteorite-landings.json`: flat record with **numbers-as-strings** and
/// a nested `geolocation`.
fn shape_meteorite(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let lat = gen_int_in(rng, -90_000_000, 90_000_000) as f64 / 1_000_000.0;
    let lon = gen_int_in(rng, -180_000_000, 180_000_000) as f64 / 1_000_000.0;
    let mut geo = Map::new();
    geo.insert("type".into(), Value::String("Point".into()));
    geo.insert(
        "coordinates".into(),
        Value::Array(vec![gen_coordinate(rng), gen_coordinate(rng)]),
    );
    let mut m = Map::new();
    m.insert("name".into(), Value::String(gen_name(rng)));
    m.insert(
        "id".into(),
        Value::String(gen_int_in(rng, 1, 100_000).to_string()),
    );
    m.insert("nametype".into(), Value::String("Valid".into()));
    m.insert("recclass".into(), Value::String(gen_code(rng, 2)));
    m.insert(
        "mass".into(),
        Value::String(gen_int_in(rng, 1, 100_000).to_string()),
    );
    m.insert("year".into(), Value::String(gen_iso_datetime(rng)));
    m.insert("reclat".into(), Value::String(format!("{lat:.6}")));
    m.insert("reclong".into(), Value::String(format!("{lon:.6}")));
    m.insert("geolocation".into(), Value::Object(geo));
    m
}

/// `Employee-Data-Skewed*.json`: user record with GUIDs, names, a cloud SID.
fn shape_employee(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("objectType".into(), Value::String("User".into()));
    m.insert("id".into(), Value::String(gen_uuid(rng)));
    m.insert("objectId".into(), Value::String(gen_uuid(rng)));
    m.insert("tenantId".into(), Value::String(gen_uuid(rng)));
    m.insert(
        "firstName".into(),
        Value::String(gen_string_from(rng, ALPHA_CHARS, 8)),
    );
    m.insert(
        "lastName".into(),
        Value::String(gen_unicode_string(rng, cfg, 12)),
    );
    m.insert(
        "MailNickname".into(),
        Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 10)),
    );
    m.insert(
        "cloudSid".into(),
        Value::String(format!(
            "S-1-12-1-{}-{}",
            gen_int_in(rng, 1, 4_000_000_000i64),
            gen_int_in(rng, 1, 4_000_000_000i64)
        )),
    );
    m.insert("isActive".into(), Value::Bool(rng.below(2) == 0));
    m
}

/// `FuzzingStrings.json`: `{ "string": <adversarial string> }`. Draws from a
/// pool of edge-case strings plus random unicode.
fn shape_fuzzing_string(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    const EDGE: &[&str] = &[
        "",
        "undefined",
        "null",
        "NULL",
        "#",
        "\t",
        "\n",
        "true",
        "false",
        "0",
        "-0",
        "NaN",
        "\\",
        "\"",
        "{}",
        "[]",
        "\u{0000}",
        "😀",
        "中文",
        "\u{feff}",
    ];
    let s = if rng.below(2) == 0 {
        let edge = EDGE[rng.below(EDGE.len() as u64) as usize];
        if cfg.unicode {
            edge.to_string()
        } else {
            // Honor AZURE_COSMOS_FUZZ_UNICODE=false: drop non-ASCII edge cases
            // (😀, 中文, BOM) so ASCII-only runs stay ASCII.
            edge.chars().filter(char::is_ascii).collect()
        }
    } else {
        gen_unicode_string(rng, cfg, 40)
    };
    let mut m = Map::new();
    m.insert("string".into(), Value::String(s));
    m
}

/// `lastfm.json` / `MillionSong1KDocuments.json`: artist track with a `similars`
/// array of `[trackId, score]` heterogeneous pairs.
fn shape_lastfm(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 1, 6);
    let similars = gen_array_of(rng, n, |r| {
        Value::Array(vec![
            Value::String(format!("TR{}", gen_hex_upper(r, 16))),
            gen_unit_float(r),
        ])
    });
    let mut m = Map::new();
    m.insert(
        "id".into(),
        Value::String(format!("item {:03}", gen_int_in(rng, 0, 999))),
    );
    m.insert("artist".into(), Value::String(gen_name(rng)));
    m.insert("timestamp".into(), Value::String(gen_iso_datetime(rng)));
    m.insert("similars".into(), similars);
    m
}

/// `MsnCollection.json`: food item with a deeply nested `Contents` of
/// unit-tagged numbers, some with scientific-notation-scale floats.
fn shape_msn_food(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let mut calories = Map::new();
    calories.insert(
        "InCalories".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 2000))),
    );
    let mut carbs = Map::new();
    // Small scientific-scale magnitude, kept envelope-safe as a plain float.
    carbs.insert("InKg".into(), gen_unit_float(rng));
    carbs.insert(
        "PreferredUnit".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 5))),
    );
    let mut contents = Map::new();
    contents.insert("TotalCalories".into(), Value::Object(calories));
    contents.insert("Carbohydrates".into(), Value::Object(carbs));
    let mut m = Map::new();
    m.insert(
        "FoodId".into(),
        Value::String(gen_int_in(rng, 1, 99999).to_string()),
    );
    m.insert("FoodName".into(), Value::String(gen_name(rng)));
    m.insert(
        "ServingSize".into(),
        Value::String("1 mug (8 fl oz)".into()),
    );
    m.insert(
        "NumberOfServings".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 10))),
    );
    m.insert("Contents".into(), Value::Object(contents));
    m
}

/// `NutritionData.json`: food doc with `tags` and `nutrients` object arrays.
fn shape_nutrition(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n_tags = count(rng, cfg, 1, 5);
    let tags = gen_array_of(rng, n_tags, |r| {
        let mut t = Map::new();
        t.insert(
            "name".into(),
            Value::String(gen_string_from(r, ALPHA_CHARS, 10)),
        );
        Value::Object(t)
    });
    let n_nutrients = count(rng, cfg, 1, 6);
    let nutrients = gen_array_of(rng, n_nutrients, |r| {
        let mut n = Map::new();
        n.insert(
            "id".into(),
            Value::String(gen_int_in(r, 1, 999).to_string()),
        );
        n.insert(
            "description".into(),
            Value::String(gen_string_from(r, ALPHANUMERIC_CHARS, 8)),
        );
        n.insert("nutritionValue".into(), gen_envelope_float(r));
        Value::Object(n)
    });
    let mut m = Map::new();
    m.insert(
        "id".into(),
        Value::String(format!("{:05}", gen_int_in(rng, 0, 99999))),
    );
    m.insert(
        "description".into(),
        Value::String(gen_unicode_string(rng, cfg, 50)),
    );
    m.insert("tags".into(), tags);
    m.insert(
        "version".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 5))),
    );
    m.insert("foodGroup".into(), Value::String(gen_name(rng)));
    m.insert("nutrients".into(), nutrients);
    m
}

/// `OpenAI_3072dim.json`: `{ "vector": [ <many unit floats> ] }`.
fn shape_embedding_vector(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    // Base 16-80 dims; `size_scale` grows it toward the corpus's 3072-dim
    // vectors (e.g. scale=40 → ~640-3200 dims).
    let dims = count(rng, cfg, 16, 64);
    let mut m = Map::new();
    m.insert("vector".into(), gen_array_of(rng, dims, gen_unit_float));
    m
}

/// `store01C.json`: shop record with id arrays, a status, and **non-ASCII**
/// (CJK) name/description fields plus a `null`.
fn shape_store(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 0, 4);
    let cat_ids = gen_array_of(rng, n, |r| {
        Value::String(gen_string_from(r, ALPHANUMERIC_CHARS, 22))
    });
    let mut m = Map::new();
    m.insert(
        "ShopId".into(),
        Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 22)),
    );
    m.insert("CategoryIds".into(), cat_ids);
    m.insert("CollectionIds".into(), Value::Array(Vec::new()));
    m.insert("IsActive".into(), Value::Bool(rng.below(2) == 0));
    m.insert(
        "Status".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 100))),
    );
    m.insert(
        "Name".into(),
        Value::String(gen_unicode_string(rng, cfg, 20)),
    );
    m.insert(
        "Summary".into(),
        Value::String(gen_unicode_string(rng, cfg, 20)),
    );
    m.insert("Description".into(), Value::Null);
    m
}

/// `TicinoErrorBuckets.json`: error-bucket with a multiline stack-trace string
/// (embedded `\n`), a hash, and a hit count.
fn shape_error_bucket(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let stack = format!(
        "Error: spawn REG ENOENT\n    at exports._errnoException (util.js:{}:11)\n    at Process.ChildProcess._handle.onexit (child_process.js:{}:32)",
        gen_int_in(rng, 100, 999),
        gen_int_in(rng, 1000, 2000)
    );
    let mut m = Map::new();
    m.insert("BucketId".into(), Value::String(stack));
    m.insert("BucketIdHash".into(), Value::String(gen_hex(rng, 32)));
    m.insert(
        "Hits".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 100_000))),
    );
    m
}

/// `XpertEvents.json`: telemetry event with nested `ingest`, ISO timestamps, a
/// GUID-bearing `userId`, and mixed numeric quality.
fn shape_xpert_event(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let mut ingest = Map::new();
    ingest.insert("time".into(), Value::String(gen_iso_datetime(rng)));
    ingest.insert("uploadTime".into(), Value::String(gen_iso_datetime(rng)));
    ingest.insert(
        "clientIp".into(),
        Value::String(format!(
            "{}.{}.{}.{}",
            gen_int_in(rng, 0, 255),
            gen_int_in(rng, 0, 255),
            gen_int_in(rng, 0, 255),
            gen_int_in(rng, 0, 255)
        )),
    );
    ingest.insert(
        "quality".into(),
        Value::Number(Number::from(gen_int_in(rng, 0, 5))),
    );
    let mut m = Map::new();
    m.insert("ingest".into(), Value::Object(ingest));
    m.insert("time".into(), Value::String(gen_iso_datetime(rng)));
    m.insert(
        "userId".into(),
        Value::String(format!("w:{{{}}}", gen_uuid(rng).to_ascii_uppercase())),
    );
    m.insert("appId".into(), Value::String(gen_hex(rng, 40)));
    m
}

/// `ups1.json`: personalization payload with an opaque high-entropy `Vector`
/// string, an ANID GUID, and a small nested `payload`.
fn shape_ups(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let mut payload = Map::new();
    payload.insert("ANID".into(), Value::String(gen_uuid(rng)));
    payload.insert(
        "MUID".into(),
        Value::Array(vec![Value::String("muid".into())]),
    );
    payload.insert("AppDomain".into(), Value::String("prime".into()));
    payload.insert("Algo".into(), Value::String("lda".into()));
    payload.insert("Culture".into(), Value::String("en-us".into()));
    payload.insert("Version".into(), Value::Number(Number::from(1)));
    payload.insert(
        "Vector".into(),
        Value::String(gen_string_from(rng, ALPHANUMERIC_CHARS, 64)),
    );
    let mut m = Map::new();
    m.insert("domain".into(), Value::String("Personalization".into()));
    m.insert("lid".into(), Value::String("lda-prime-en-us-1".into()));
    m.insert("payload".into(), Value::Object(payload));
    m
}

/// `states_committees.json`: committee with a `members` array of role records.
fn shape_committee(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 1, 6);
    let members = gen_array_of(rng, n, |r| {
        let mut mem = Map::new();
        mem.insert(
            "leg_id".into(),
            Value::String(format!("{}{:06}", gen_code(r, 3), gen_int_in(r, 0, 999999))),
        );
        mem.insert("role".into(), Value::String("member".into()));
        mem.insert(
            "name".into(),
            Value::String(format!("Representative {}", gen_name(r))),
        );
        Value::Object(mem)
    });
    let mut m = Map::new();
    m.insert("members".into(), members);
    m
}

/// `states_legislators.json`: legislator with `sources`, `old_roles` (object
/// keyed by term with role arrays containing `null`s), and name fields.
fn shape_legislator(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 1, 2);
    let sources = gen_array_of(rng, n, |r| {
        let mut s = Map::new();
        s.insert(
            "url".into(),
            Value::String(format!(
                "http://example.gov/legislator.php?id={}",
                gen_string_from(r, ALPHA_CHARS, 4)
            )),
        );
        Value::Object(s)
    });
    let n = count(rng, cfg, 1, 2);
    let roles = gen_array_of(rng, n, |r| {
        let mut role = Map::new();
        role.insert(
            "term".into(),
            Value::String(gen_int_in(r, 20, 30).to_string()),
        );
        role.insert("end_date".into(), Value::Null);
        role.insert("district".into(), Value::String(gen_code(r, 1)));
        Value::Object(role)
    });
    let mut old_roles = Map::new();
    old_roles.insert(gen_int_in(rng, 20, 30).to_string(), roles);
    let mut m = Map::new();
    m.insert("last_name".into(), Value::String(gen_name(rng)));
    m.insert("updated_at".into(), Value::String(gen_iso_datetime(rng)));
    m.insert("sources".into(), sources);
    m.insert("full_name".into(), Value::String(gen_name(rng)));
    m.insert("old_roles".into(), Value::Object(old_roles));
    m
}

/// `LogData.json`: impression log with GUID ids, a boolean, and an `Events`
/// array of nested event/page/requestInfo objects.
fn shape_logdata(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 1, 3);
    let events = gen_array_of(rng, n, |r| {
        let mut page = Map::new();
        page.insert("Name".into(), Value::String("API.Qsml".into()));
        let mut req = Map::new();
        req.insert("AFORM".into(), Value::String("MSNH2".into()));
        req.insert(
            "Bytes".into(),
            Value::Number(Number::from(gen_int_in(r, 0, 10000))),
        );
        let mut ev = Map::new();
        ev.insert("T".into(), Value::String("Event.Impression".into()));
        ev.insert("EventId".into(), Value::String(gen_hex_upper(r, 32)));
        ev.insert("Page".into(), Value::Object(page));
        ev.insert("RequestInfo".into(), Value::Object(req));
        Value::Object(ev)
    });
    let mut m = Map::new();
    m.insert("AppNS".into(), Value::String("API".into()));
    m.insert("ClientId".into(), Value::String(gen_hex_upper(rng, 32)));
    m.insert(
        "ImpressionGuid".into(),
        Value::String(gen_hex_upper(rng, 32)),
    );
    m.insert("ProvClientId".into(), Value::Bool(rng.below(2) == 0));
    m.insert("Events".into(), events);
    m
}

/// `sampleWorkload.json`: record with a `header` carrying nested `schema`.
fn shape_workload(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let mut schema = Map::new();
    schema.insert("name".into(), Value::String("_xdm.context.profile".into()));
    schema.insert("version".into(), Value::String("1.0".into()));
    let mut header = Map::new();
    header.insert("recordType".into(), Value::String("keyvalue".into()));
    header.insert(
        "tag".into(),
        Value::String(format!("batchId-{}", gen_hex_upper(rng, 26))),
    );
    header.insert("packetVersion".into(), Value::String("1.0".into()));
    header.insert(
        "component".into(),
        Value::String(format!("{:03}", gen_int_in(rng, 0, 999))),
    );
    header.insert("schema".into(), Value::Object(schema));
    let mut m = Map::new();
    m.insert("header".into(), Value::Object(header));
    m.insert(
        "payload".into(),
        Value::String(gen_unicode_string(rng, cfg, 40)),
    );
    m
}

/// `DefaultHybridRowSchema.json`: schema-policy document with booleans and a
/// nested `tableSchema.schemas` array (options flags).
fn shape_hybrid_schema(rng: &mut SplitMix64, _cfg: &FuzzConfig) -> Map<String, Value> {
    let mut options = Map::new();
    options.insert(
        "disallowUnschematized".into(),
        Value::Bool(rng.below(2) == 0),
    );
    options.insert(
        "enablePropertyLevelTimestamp".into(),
        Value::Bool(rng.below(2) == 0),
    );
    options.insert("disableSystemPrefix".into(), Value::Bool(rng.below(2) == 0));
    let schema = {
        let mut s = Map::new();
        s.insert("version".into(), Value::String("v1".into()));
        s.insert("name".into(), Value::String("Row".into()));
        s.insert("id".into(), Value::Number(Number::from(-1)));
        s.insert("type".into(), Value::String("schema".into()));
        s.insert("options".into(), Value::Object(options));
        Value::Object(s)
    };
    let mut table_schema = Map::new();
    table_schema.insert("version".into(), Value::String("v1".into()));
    table_schema.insert("name".into(), Value::String("tableSchema".into()));
    table_schema.insert("schemas".into(), Value::Array(vec![schema]));
    let mut policy = Map::new();
    policy.insert("tableSchema".into(), Value::Object(table_schema));
    let mut m = Map::new();
    m.insert("schemaPolicy".into(), Value::Object(policy));
    m
}

/// `reddit_all.json` (single-record variant): a Listing with a nested `data`
/// carrying a `children` array of `t3` post objects and many `null`s.
fn shape_reddit(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 1, 4);
    let children = gen_array_of(rng, n, |r| {
        let mut data = Map::new();
        data.insert("approved_at_utc".into(), Value::Null);
        data.insert(
            "title".into(),
            Value::String(gen_unicode_string_for(r, cfg, 40)),
        );
        data.insert(
            "ups".into(),
            Value::Number(Number::from(gen_int_in(r, 0, 100_000))),
        );
        data.insert("over_18".into(), Value::Bool(r.below(2) == 0));
        data.insert("score".into(), gen_envelope_float(r));
        let mut child = Map::new();
        child.insert("kind".into(), Value::String("t3".into()));
        child.insert("data".into(), Value::Object(data));
        Value::Object(child)
    });
    let mut data = Map::new();
    data.insert(
        "after".into(),
        Value::String(format!(
            "t3_{}",
            gen_string_from(rng, ALPHANUMERIC_CHARS, 7)
        )),
    );
    data.insert(
        "dist".into(),
        Value::Number(Number::from(gen_int_in(rng, 1, 100))),
    );
    data.insert("modhash".into(), Value::String(String::new()));
    data.insert("geo_filter".into(), Value::Null);
    data.insert("children".into(), children);
    let mut m = Map::new();
    m.insert("kind".into(), Value::String("Listing".into()));
    m.insert("data".into(), Value::Object(data));
    m
}

/// `open-food-facts.json` (single-record variant): a product wrapper with a
/// `_keywords` string array and a nested `product`.
fn shape_open_food(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let n = count(rng, cfg, 3, 6);
    let keywords = gen_array_of(rng, n, |r| {
        Value::String(gen_string_from(r, ALPHA_CHARS, 10))
    });
    let mut product = Map::new();
    product.insert(
        "_id".into(),
        Value::String(gen_int_in(rng, 1, i64::MAX / 2).to_string()),
    );
    product.insert("_keywords".into(), keywords);
    product.insert("nutriments".into(), {
        let mut n = Map::new();
        n.insert("energy".into(), gen_envelope_float(rng));
        n.insert("fat".into(), gen_envelope_float(rng));
        Value::Object(n)
    });
    let mut m = Map::new();
    m.insert(
        "code".into(),
        Value::String(gen_int_in(rng, 1, i64::MAX / 2).to_string()),
    );
    m.insert("product".into(), Value::Object(product));
    m
}

/// Convenience: unicode string honoring the `unicode` knob (thin wrapper so the
/// closure-based array builders can call it with a `cfg`).
fn gen_unicode_string_for(rng: &mut SplitMix64, cfg: &FuzzConfig, max_len: usize) -> String {
    gen_unicode_string(rng, cfg, max_len)
}

/// The set of corpus-shape samplers. Each entry is `(name, fn)` where `name`
/// mirrors the originating `testdata/*.json` file and `fn` builds one document
/// in that shape. [`gen_shaped_document`] picks one uniformly at random.
type ShapeSampler = fn(&mut SplitMix64, &FuzzConfig) -> Map<String, Value>;

const SHAPE_SAMPLERS: &[(&str, ShapeSampler)] = &[
    ("airline-delays", shape_airline_delays),
    ("bitcoin", shape_bitcoin),
    ("bing-docs", shape_bing_docs),
    ("scripts-data", shape_scripts_data),
    ("geojson-feature", shape_geojson_feature),
    ("cosmos-run", shape_cosmos_run),
    ("meteorite", shape_meteorite),
    ("employee", shape_employee),
    ("fuzzing-string", shape_fuzzing_string),
    ("lastfm", shape_lastfm),
    ("msn-food", shape_msn_food),
    ("nutrition", shape_nutrition),
    ("embedding-vector", shape_embedding_vector),
    ("store", shape_store),
    ("error-bucket", shape_error_bucket),
    ("xpert-event", shape_xpert_event),
    ("ups", shape_ups),
    ("committee", shape_committee),
    ("legislator", shape_legislator),
    ("logdata", shape_logdata),
    ("workload", shape_workload),
    ("hybrid-schema", shape_hybrid_schema),
    ("reddit", shape_reddit),
    ("open-food", shape_open_food),
];

/// Builds one document matching a **randomly chosen** corpus shape (see
/// [`SHAPE_SAMPLERS`]). The returned map does not yet carry the caller-reserved
/// `id`/`pk` fields — the run loop inserts those.
fn gen_shaped_document(rng: &mut SplitMix64, cfg: &FuzzConfig) -> Map<String, Value> {
    let idx = rng.below(SHAPE_SAMPLERS.len() as u64) as usize;
    SHAPE_SAMPLERS[idx].1(rng, cfg)
}

/// A small, irregular filler value. Draws from typed scalars, mixed
/// arrays/objects of typed scalars, homogeneous number arrays (to exercise the
/// uniform-number wire forms), and `ArbitraryValue` subtrees — so filler is both
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
        // An irregular entropy-driven subtree (bigger byte budget),
        // envelope-bounded.
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
            // When Unicode is disabled, object *keys* must be ASCII-filtered
            // too — property names go through the same binary string-encoding
            // path as values, so leaving non-ASCII keys would not isolate the
            // ASCII codec path. `ArbitraryValue` can emit non-ASCII keys.
            if !cfg.unicode && map.keys().any(|k| !k.is_ascii()) {
                let rebuilt: Map<String, Value> = std::mem::take(map)
                    .into_iter()
                    .map(|(k, mut v)| {
                        bound_value(&mut v, cfg);
                        let key = if k.is_ascii() {
                            k
                        } else {
                            k.chars().filter(char::is_ascii).collect()
                        };
                        (key, v)
                    })
                    .collect();
                *map = rebuilt;
            } else {
                for v in map.values_mut() {
                    bound_value(v, cfg);
                }
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
/// - integers with magnitude `>= 2^53` (whether `i64` or `u64`) → **tagged
///   wide-number value** ([`cosmos_wide_number_value`]) holding the `f64` token.
///   The backend stores *every* JSON number as an
///   IEEE-754 double, so an integer beyond `2^53` is not preserved exactly (e.g.
///   `28423844363879210` is echoed back as `28423844363879208`); tokenizing the
///   rounded double makes the sent and returned values compare equal;
/// - integral-valued floats below `2^53` (e.g. `1.0`) → integer form (the
///   backend drops the trailing `.0`);
/// - integral-valued floats `>= 2^53` → tagged wide-number value (matches the
///   lossy double case above);
/// - other finite floats → kept as `f64` (JCS-safe);
/// - non-finite (`NaN` / `±∞`) → `null`.
///
/// The wide-number tokens are only ever compared for equality, so a tagged token
/// is sound: any two values Cosmos round-trips to each other produce the same
/// token, and the tag keeps them out of the plain-string domain (see
/// [`WIDE_NUMBER_TAG`]).
fn normalize_number(n: &Number) -> Value {
    if let Some(i) = n.as_i64() {
        if (i.unsigned_abs() as f64) < JCS_SAFE_INT_LIMIT {
            Value::Number(Number::from(i))
        } else {
            // i >= 2^53: the backend stores it as a lossy double, so tokenize the
            // rounded f64 (not the exact decimal) — otherwise the returned,
            // double-rounded value would mismatch.
            cosmos_wide_number_value(i as f64)
        }
    } else if let Some(u) = n.as_u64() {
        // u > i64::MAX: Cosmos stores it as a lossy double; token from the double.
        cosmos_wide_number_value(u as f64)
    } else if let Some(f) = n.as_f64() {
        if !f.is_finite() {
            Value::Null
        } else if f.fract() == 0.0 && f.abs() < JCS_SAFE_INT_LIMIT {
            Value::Number(Number::from(f as i64))
        } else if f.fract() == 0.0 {
            // Integral but out of the JCS-safe range → double token.
            cosmos_wide_number_value(f)
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

/// The object key that tags a normalized wide-number token. Wide numbers are
/// canonicalized to `{ WIDE_NUMBER_TAG: "<double token>" }` rather than a bare
/// [`Value::String`], keeping the token in a distinct type domain so a
/// number-to-string codec bug cannot canonicalize equal and pass silently.
///
/// Known blind spot: the tag lives in the user JSON domain, so it is not fully
/// type-injective — a genuine user object with this exact key and a matching
/// value canonicalizes identically to the wide number. The sentinel is
/// deliberately obscure to make that astronomically unlikely; a fully injective
/// fix would tag normalized kinds out-of-band rather than as a user-visible key.
const WIDE_NUMBER_TAG: &str = "$__cosmos_wide_number__";

/// Wraps a Cosmos-stored double's decimal token in the [`WIDE_NUMBER_TAG`]
/// envelope. The inner value stays a `String`, so [`normalize_numbers`] is
/// idempotent over the result.
fn cosmos_wide_number_value(f: f64) -> Value {
    let mut map = serde_json::Map::new();
    map.insert(
        WIDE_NUMBER_TAG.to_string(),
        Value::String(cosmos_double_token(f)),
    );
    Value::Object(map)
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

/// Cosmos-reserved system properties: the service assigns and owns these, so a
/// document we author must never carry them (a random value would round-trip
/// back as the service's own value and cause a false mismatch).
const RESERVED_SYSTEM_KEYS: &[&str] = &["_rid", "_self", "_etag", "_ts", "_attachments"];

/// Removes any Cosmos-reserved system properties from `doc` in place. Applied to
/// every generated document before it is sent, so neither the corpus shapes nor
/// the free-form value generator can emit a reserved key the service
/// would overwrite.
fn strip_reserved_fields(doc: &mut Map<String, Value>) {
    for key in RESERVED_SYSTEM_KEYS {
        doc.remove(*key);
    }
}

/// Strips the service-assigned system properties from a returned document so
/// they don't affect the comparison.
///
/// Removes **only the reserved keys** rather than projecting down to the sent
/// keys: projecting would silently discard any *extra* key the round-trip
/// introduced (a mis-parsed length prefix can invent a field) — exactly the
/// codec bug this fuzzer exists to catch.
fn project_to_sent_keys(sent: &Map<String, Value>, got: &Value) -> Value {
    let got_obj = match got.as_object() {
        Some(o) => o,
        None => return got.clone(),
    };
    let mut out = got_obj.clone();
    for key in RESERVED_SYSTEM_KEYS {
        // Keep a reserved key only if we actually sent it (safety valve).
        if !sent.contains_key(*key) {
            out.remove(*key);
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
        // Explicitly disabled, not unset: binary encoding is enabled by
        // default, so an unset option would make this control a binary run.
        RunConfig {
            label: "text-control",
            binary: Some(BinaryEncodingOptions::new().with_enabled(false)),
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

/// A typed document with integer fields, used to exercise the native binary
/// deserializer's integer path live (the `Value`-based round-trips only reach
/// `deserialize_any`).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct IntProbe {
    id: String,
    pk: String,
    normal: u64,
    signed: i64,
    /// Wide value the service stores as a lossy double; reading it back drives
    /// the integral-`Double`→integer coercion. `u64::MAX` saturates back to
    /// `u64::MAX`, so the comparison stays exact.
    wide: u64,
}

/// Round-trips an [`IntProbe`] so `deserialize_integer` (the production change)
/// is exercised live, then asserts the typed values survived. Only meaningful on
/// the pure-binary config (see the call site).
async fn assert_typed_integer_probe(
    container: &ContainerClient,
    pk: &str,
    iter: u64,
    seed: u64,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let id = format!("int-probe-{seed:016x}-{iter}");
    // Own `pk` locally so the operation futures don't borrow the `&str` param.
    let pk = pk.to_string();
    let sent = IntProbe {
        id: id.clone(),
        pk: pk.clone(),
        normal: 42,
        signed: -7,
        wide: u64::MAX,
    };
    with_transient_retry("int-probe-upsert", context, || {
        container.upsert_item(&pk, &id, &sent, Some(write_options_with_content()))
    })
    .await?;
    let read = with_transient_retry("int-probe-read", context, || {
        container.read_item(&pk, &id, None)
    })
    .await?;
    // The decode is the coverage: a regressed `deserialize_integer` would fail
    // here with `invalid type: floating point, expected u64`.
    let got: IntProbe = read
        .into_model()
        .map_err(|e| format!("{context}: typed integer probe decode failed: {e}"))?;
    assert_eq!(
        got, sent,
        "{context}: typed integer probe round-trip changed"
    );

    // Also decode the same wide-integer probe through a full-container binary
    // ORDER BY query, driving the streaming-merge envelope decode (`build_page`
    // re-encodes to binary so `deserialize_integer` runs). A merge that emitted
    // text would fail here with `invalid type: floating point, expected u64`.
    //
    // This container has default throughput (one physical partition), so the
    // merge runs with a single child — multi-child interleave is covered by the
    // 3-partition emulator tests.
    let order_by = with_transient_retry("int-probe-order-by", context, || async {
        let query = Query::from("SELECT * FROM c WHERE c.id = @id ORDER BY c.id")
            .with_parameter("@id", id.as_str())?;
        let iter = container
            .query_items::<IntProbe>(query, FeedScope::full_container(), None)
            .await?;
        iter.try_collect::<Vec<_>>().await
    })
    .await?;
    assert_eq!(
        order_by.len(),
        1,
        "{context}: typed integer ORDER BY probe expected exactly one item, got {}",
        order_by.len(),
    );
    assert_eq!(
        order_by[0], sent,
        "{context}: typed integer probe changed through the binary ORDER BY merge"
    );
    Ok(())
}

/// Maximum attempts for a single point operation before the run gives up.
const MAX_OP_ATTEMPTS: u32 = 6;

/// Transient transport/service status codes worth retrying (429/408/503/...).
/// A response-body serialization failure (`500 / SERIALIZATION_RESPONSE_BODY_INVALID`)
/// is excluded: it is the decode corruption this fuzzer exists to catch, so it
/// must surface immediately rather than be retried into a masking 409.
fn is_transient(err: &azure_data_cosmos::CosmosError) -> bool {
    let status = err.status();
    if status.sub_status() == Some(SubStatusCode::SERIALIZATION_RESPONSE_BODY_INVALID) {
        return false;
    }
    matches!(
        u16::from(status.status_code()),
        408 | 429 | 500 | 502 | 503 | 504
    )
}

/// Runs a point operation, retrying transient failures with exponential backoff
/// so a long soak survives network blips. Non-transient errors (notably a decode
/// failure — what this fuzzer exists to catch) return immediately.
async fn with_transient_retry<T, F, Fut>(
    op_name: &str,
    context: &str,
    mut op: F,
) -> Result<T, Box<dyn Error>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = azure_data_cosmos::Result<T>>,
{
    let mut attempt = 0;
    loop {
        attempt += 1;
        match op().await {
            Ok(value) => return Ok(value),
            Err(e) if is_transient(&e) && attempt < MAX_OP_ATTEMPTS => {
                let backoff =
                    std::time::Duration::from_millis(200u64 * (1u64 << (attempt - 1)).min(16));
                eprintln!(
                    "{context}: {op_name} transient failure (attempt {attempt}/{MAX_OP_ATTEMPTS}), \
                     retrying in {backoff:?}: {e}"
                );
                tokio::time::sleep(backoff).await;
            }
            Err(e) => return Err(format!("{context}: {op_name} failed: {e}").into()),
        }
    }
}

/// Queries the just-written item back and asserts it round-trips, covering the
/// query binary-response decode path (which point ops do not exercise). Runs a
/// single-partition query (the passthrough decode path) and a full-container
/// streaming `ORDER BY` query, whose per-page envelope decode is the binary path
/// added for query support.
///
/// Both shapes run on **every** config, including `text-control`: a control that
/// exercises a smaller surface than the experiment cannot localize a failure to
/// the encoding.
///
/// The live container has default throughput (one physical partition), so the
/// merge runs with a single child — multi-child interleave is covered by the
/// 3-partition emulator tests.
///
/// Both filter on the unique `id`, so each returns exactly this item.
async fn assert_query_roundtrip(
    container: &ContainerClient,
    pk: &str,
    id: &str,
    sent_canon: &str,
    sent_hash: &[u8; 32],
    doc: &Map<String, Value>,
    context: &str,
) -> Result<usize, Box<dyn Error>> {
    let single_partition = with_transient_retry("query", context, || async {
        let query = Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", id)?;
        let iter = container
            .query_items::<Value>(query, FeedScope::partition(pk.to_string()), None)
            .await?;
        iter.try_collect::<Vec<_>>().await
    })
    .await?;
    assert_query_hit(
        &single_partition,
        doc,
        sent_canon,
        sent_hash,
        context,
        "query",
    );

    let order_by = with_transient_retry("query-order-by", context, || async {
        let query = Query::from("SELECT * FROM c WHERE c.id = @id ORDER BY c.id")
            .with_parameter("@id", id)?;
        let iter = container
            .query_items::<Value>(query, FeedScope::full_container(), None)
            .await?;
        iter.try_collect::<Vec<_>>().await
    })
    .await?;
    assert_query_hit(
        &order_by,
        doc,
        sent_canon,
        sent_hash,
        context,
        "query-order-by",
    );

    Ok(2)
}

/// Asserts a query returned exactly the one expected item and that it
/// round-trips against the sent canonical form.
fn assert_query_hit(
    results: &[Value],
    doc: &Map<String, Value>,
    sent_canon: &str,
    sent_hash: &[u8; 32],
    context: &str,
    phase: &str,
) {
    assert_eq!(
        results.len(),
        1,
        "{context}: {phase} expected exactly one item, got {}",
        results.len()
    );
    assert_roundtrip(doc, &results[0], sent_canon, sent_hash, context, phase);
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

        for (config_idx, (label, client)) in clients.iter().enumerate() {
            let container = client
                .database_client(&database_name)
                .container_client(&container_name, None)
                .await?;

            // Deterministic id per (seed, iteration, config) — derived without
            // touching the document RNG stream so a rerun with the same
            // AZURE_COSMOS_FUZZ_SEED reproduces the exact document *and* its
            // canonical form (a random `Uuid` here would defeat that promise).
            let id = format!("fuzz-{:016x}-{iter}-{config_idx}", cfg.seed);
            let mut doc = base_doc.clone();
            doc.insert("id".to_string(), Value::String(id.clone()));
            doc.insert("pk".to_string(), Value::String(pk.clone()));
            // Never send Cosmos-reserved system properties (`_rid`, `_self`,
            // `_etag`, `_ts`, `_attachments`): the service **owns** these and
            // overwrites/assigns them, so a random value we send would come
            // back different and cause a false round-trip mismatch. The corpus
            // shapes (and free-form value generation) can incidentally emit them.
            strip_reserved_fields(&mut doc);

            // Compute the sent canonical form from a normalized copy so it
            // matches documents that have been through the backend's
            // serialize→parse.
            let (sent_canon, sent_hash) = canonical_hash(&normalize(&Value::Object(doc.clone())));

            let context = format!("iter={iter} config={label} id={id} seed={}", cfg.seed);

            // CREATE with content response (exercises the response decode
            // path). Retry transient failures. A 409 Conflict means a replayed
            // seed collided with an item an earlier run committed — possibly one
            // whose create-response decode then failed. Reading it back would
            // validate a read response and silently mask that decode failure on
            // replay, so instead delete the stale item and retry create.
            let created_doc: Value = {
                let mut attempt = 0;
                loop {
                    attempt += 1;
                    match container
                        .create_item(&pk, &id, &doc, Some(write_options_with_content()))
                        .await
                    {
                        Ok(resp) => {
                            break resp.into_model().map_err(|e| {
                                format!("{context}: create response decode failed: {e}")
                            })?;
                        }
                        Err(e)
                            if e.status().status_code() == StatusCode::Conflict
                                && attempt < MAX_OP_ATTEMPTS =>
                        {
                            // Delete the stale item, then loop to retry create.
                            // A concurrent delete (404) is fine.
                            match container.delete_item(&pk, &id, None).await {
                                Ok(_) => {}
                                Err(e) if e.status().status_code() == StatusCode::NotFound => {}
                                Err(e) => {
                                    return Err(format!(
                                        "{context}: create-conflict cleanup delete failed: {e}"
                                    )
                                    .into())
                                }
                            }
                        }
                        Err(e) if is_transient(&e) && attempt < MAX_OP_ATTEMPTS => {
                            let backoff = std::time::Duration::from_millis(
                                200u64 * (1u64 << (attempt - 1)).min(16),
                            );
                            eprintln!(
                                "{context}: create transient failure (attempt {attempt}/{MAX_OP_ATTEMPTS}), retrying in {backoff:?}: {e}"
                            );
                            tokio::time::sleep(backoff).await;
                        }
                        Err(e) => return Err(format!("{context}: create failed: {e}").into()),
                    }
                }
            };
            assert_roundtrip(
                &doc,
                &created_doc,
                &sent_canon,
                &sent_hash,
                &context,
                "create",
            );

            // READ back.
            let read =
                with_transient_retry("read", &context, || container.read_item(&pk, &id, None))
                    .await?;
            let read_doc: Value = read
                .into_model()
                .map_err(|e| format!("{context}: read response decode failed: {e}"))?;
            assert_roundtrip(&doc, &read_doc, &sent_canon, &sent_hash, &context, "read");

            // REPLACE the item with the same value (exercises the replace point
            // op's request encode + response decode). Binary encoding is honored
            // for replace, so this drives the encoder/decoder just like create.
            let replaced = with_transient_retry("replace", &context, || {
                container.replace_item(&pk, &id, &doc, Some(write_options_with_content()))
            })
            .await?;
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
            let upserted = with_transient_retry("upsert", &context, || {
                container.upsert_item(&pk, &id, &doc, Some(write_options_with_content()))
            })
            .await?;
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

            // QUERY the item back. Both query shapes run on every config so a
            // failure localizes to the encoding, not the config (#4976). Note
            // `binary+text-response` is still binary on the wire, so its ORDER BY
            // merge runs over binary pages.
            let queries_checked = assert_query_roundtrip(
                &container,
                &pk,
                &id,
                &sent_canon,
                &sent_hash,
                &doc,
                &context,
            )
            .await?;
            checked += queries_checked as u64;

            // The four ops above decode into `serde_json::Value` (→
            // `deserialize_any`), so they do NOT cover the native typed-integer
            // path (`deserialize_integer`) this PR ships. A typed probe covers it
            // live on the pure-binary config — the only mode that returns binary
            // for an integer field (text modes return text, which `serde_json`
            // rejects into an integer).
            if *label == "binary" {
                assert_typed_integer_probe(&container, &pk, iter, cfg.seed, &context).await?;
                // Three round-trips, not one: upsert, point read, ORDER BY.
                // `checked` is reported as a round-trip count, and the CI
                // budget in sdk/cosmos/ci.yml is sized off it.
                checked += 3;
            }
        }

        if (iter + 1) % 100 == 0 {
            println!("... {} iterations, {checked} round-trips OK", iter + 1);
        }
    }

    println!(
        "binary_roundtrip_fuzzer: DONE — {} documents × {} configs (4 point ops each + 2 queries: single-partition and full-container ORDER BY, plus a 3-call typed integer probe on the pure-binary config) = {checked} round-trips, all canonical-equal (seed={})",
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
    let container = db.container_client(&container_name, None).await?;

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
// account. These run under a normal `cargo test -p azure_data_cosmos`.
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
        // The backend stores integers above 2^53 as doubles, so they must
        // canonicalize to the same string token as the rounded double form.
        let sent_u64: Value = serde_json::from_str("18446744073709551614").unwrap();
        let backend_double: Value = serde_json::from_str("1.8446744073709552e+19").unwrap();
        assert_eq!(canon(&sent_u64), canon(&backend_double));

        // 2^63 (just above i64::MAX).
        let sent_2p63: Value = serde_json::from_str("9223372036854775808").unwrap();
        let backend_2p63: Value = serde_json::from_str("9.223372036854776e+18").unwrap();
        assert_eq!(canon(&sent_2p63), canon(&backend_2p63));

        // Signed i64 above 2^53 is also stored lossily (regression: the i64
        // branch used to emit the exact decimal). Two real live-leg cases:
        let sent_a: Value = serde_json::from_str("28423844363879210").unwrap();
        let backend_a: Value = serde_json::from_str("28423844363879208").unwrap();
        assert_eq!(canon(&sent_a), canon(&backend_a));
        let sent_b: Value = serde_json::from_str("39207287747660610").unwrap();
        let backend_b: Value = serde_json::from_str("39207287747660608").unwrap();
        assert_eq!(canon(&sent_b), canon(&backend_b));

        // i64::MAX exceeds 2^53 → lossy double token (2^63), not exact decimal.
        let i64_max: Value = serde_json::from_str("9223372036854775807").unwrap();
        assert_eq!(canon(&i64_max), canon(&sent_2p63));

        // A JCS-safe integer stays a bare number.
        assert_eq!(canon(&serde_json::json!(1_000_000)), "1000000");
    }

    #[test]
    fn wide_number_token_cannot_collide_with_a_plain_string() {
        // Oracle type-safety: a wide number lives in its own tagged domain, so a
        // codec bug that turns it into a JSON string with the same decimal text
        // must NOT canonicalize equal — otherwise the corruption passes silently.
        let wide_number: Value = serde_json::from_str("18446744073709551614").unwrap();
        let normalized = normalize_numbers(&wide_number);
        let token = normalized[WIDE_NUMBER_TAG]
            .as_str()
            .expect("wide number normalizes to a tagged token object")
            .to_string();

        // The bare decimal string the backend would echo for a number→string bug.
        let corrupted_string = Value::String(token);
        assert_ne!(canon(&wide_number), canon(&corrupted_string));
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
            shape_ratio: DEFAULT_SHAPE_RATIO,
            size_scale: DEFAULT_SIZE_SCALE,
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
        // grow with `max_depth` (the old free-form-only generator was flat
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
                // Force the free-form hybrid-skeleton generator (shape_ratio=0):
                // this test asserts that *its* depth scales with `max_depth`.
                // Corpus shape samplers have fixed, sampler-defined depth
                // independent of the knob, so they would dilute the signal at
                // the production `DEFAULT_SHAPE_RATIO`.
                shape_ratio: 0,
                size_scale: DEFAULT_SIZE_SCALE,
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
            shape_ratio: DEFAULT_SHAPE_RATIO,
            size_scale: DEFAULT_SIZE_SCALE,
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
    fn unicode_off_strips_non_ascii_from_object_keys_and_values() {
        // AZURE_COSMOS_FUZZ_UNICODE=false must isolate the ASCII codec path:
        // non-ASCII must be dropped from object *keys* as well as values
        // (property names go through the same binary string encoding).
        let cfg = FuzzConfig {
            iterations: 0,
            seed: 1,
            max_depth: 3,
            wide_numbers: false,
            unicode: false,
            breadth: DEFAULT_BREADTH,
            shape_ratio: DEFAULT_SHAPE_RATIO,
            size_scale: DEFAULT_SIZE_SCALE,
            calibrate: false,
            print_docs: false,
        };
        let mut doc = serde_json::json!({
            "kéy1": { "kéy2": "vàl", "ok": 1 },
            "plain": ["a", "bé"],
        });
        bound_value(&mut doc, &cfg);
        fn assert_ascii(v: &Value) {
            match v {
                Value::String(s) => assert!(s.is_ascii(), "non-ASCII value survived: {s:?}"),
                Value::Array(items) => items.iter().for_each(assert_ascii),
                Value::Object(map) => {
                    for (k, v) in map {
                        assert!(k.is_ascii(), "non-ASCII key survived: {k:?}");
                        assert_ascii(v);
                    }
                }
                _ => {}
            }
        }
        assert_ascii(&doc);
    }

    #[test]
    fn breadth_env_clamps_multiple_of_2_pow_32_to_a_nonzero_u32() {
        // `env_u32` must reject an out-of-range value (a checked conversion),
        // never silently `as u32`-wrap it. Mirror the helper's logic here so the
        // test does not mutate the process environment (racy under the parallel
        // harness).
        let checked = |raw: u64, min: u32, max: u32| -> std::result::Result<u32, ()> {
            if raw > u64::from(max) {
                return Err(()); // would panic in `env_u32`
            }
            Ok(raw.max(u64::from(min)) as u32)
        };
        // 2^32 would truncate to 0 under a bare `as u32`; the checked path rejects it.
        assert!(checked(4_294_967_296, 1, BREADTH_LIMIT).is_err());
        // Above the sane limit is rejected rather than clamped.
        assert!(checked(u64::from(BREADTH_LIMIT) + 1, 1, BREADTH_LIMIT).is_err());
        // Below the minimum is raised to the minimum.
        assert_eq!(checked(0, 1, BREADTH_LIMIT), Ok(1));
        assert_eq!(checked(6, 1, BREADTH_LIMIT), Ok(6));
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
            shape_ratio: DEFAULT_SHAPE_RATIO,
            size_scale: DEFAULT_SIZE_SCALE,
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

    #[test]
    fn every_corpus_shape_produces_a_valid_object() {
        // Each corpus shape sampler must produce a non-empty JSON object that
        // serializes and normalizes cleanly (the invariant the round-trip
        // comparison relies on). Exercised across several seeds and both the
        // wide-numbers / unicode knob settings.
        for &wide in &[false, true] {
            for &uni in &[false, true] {
                let cfg = FuzzConfig {
                    iterations: 0,
                    seed: 0x5EED_1234,
                    max_depth: 6,
                    wide_numbers: wide,
                    unicode: uni,
                    breadth: DEFAULT_BREADTH,
                    shape_ratio: DEFAULT_SHAPE_RATIO,
                    size_scale: DEFAULT_SIZE_SCALE,
                    calibrate: false,
                    print_docs: false,
                };
                let mut rng = SplitMix64::new(cfg.seed);
                for (name, sampler) in SHAPE_SAMPLERS {
                    let doc = sampler(&mut rng, &cfg);
                    assert!(!doc.is_empty(), "shape {name} produced an empty object");
                    let value = Value::Object(doc);
                    // Normalization is idempotent for a well-formed value.
                    let once = normalize(&value);
                    let twice = normalize(&once);
                    assert_eq!(
                        canon(&once),
                        canon(&twice),
                        "shape {name} not normalization-stable"
                    );
                }
            }
        }
    }

    #[test]
    fn no_shape_emits_reserved_system_fields() {
        // Cosmos owns `_rid`/`_self`/`_etag`/`_ts`/`_attachments`; a generated
        // document must never author them (the service overwrites them, causing
        // a false round-trip mismatch). Assert every shape sampler is clean, and
        // that the strip helper removes them if present.
        let cfg = FuzzConfig {
            iterations: 0,
            seed: 0xBADF00D,
            max_depth: 6,
            wide_numbers: true,
            unicode: true,
            breadth: DEFAULT_BREADTH,
            shape_ratio: DEFAULT_SHAPE_RATIO,
            size_scale: DEFAULT_SIZE_SCALE,
            calibrate: false,
            print_docs: false,
        };
        let mut rng = SplitMix64::new(cfg.seed);
        for (name, sampler) in SHAPE_SAMPLERS {
            let doc = sampler(&mut rng, &cfg);
            for reserved in RESERVED_SYSTEM_KEYS {
                assert!(
                    !doc.contains_key(*reserved),
                    "shape {name} emitted reserved system field {reserved}"
                );
            }
        }
        // The strip helper removes reserved keys wherever they appear.
        let mut with_reserved = Map::new();
        with_reserved.insert("_self".into(), Value::String("x".into()));
        with_reserved.insert("_ts".into(), Value::Number(123.into()));
        with_reserved.insert("keep".into(), Value::Bool(true));
        strip_reserved_fields(&mut with_reserved);
        assert_eq!(with_reserved.len(), 1);
        assert!(with_reserved.contains_key("keep"));
    }

    #[test]
    fn shaped_documents_are_emitted_when_ratio_is_full() {
        // With shape_ratio = 100 every document is a corpus shape; assert that
        // over many draws we see several distinct shapes (the id/pk-free base is
        // shaped, then the run loop adds id/pk). Detect a shape by a signature
        // key unique to a sampler.
        let cfg = FuzzConfig {
            iterations: 0,
            seed: 0xA11CE,
            max_depth: 6,
            wide_numbers: false,
            unicode: true,
            breadth: DEFAULT_BREADTH,
            shape_ratio: 100,
            size_scale: DEFAULT_SIZE_SCALE,
            calibrate: false,
            print_docs: false,
        };
        let mut rng = SplitMix64::new(cfg.seed);
        let mut seen_signature_keys = std::collections::BTreeSet::new();
        // Signature keys that appear only in specific shapes.
        let signatures = [
            "hash",         // bitcoin
            "blogId",       // bing-docs
            "similars",     // lastfm
            "vector",       // embedding
            "BucketId",     // error-bucket
            "schemaPolicy", // hybrid-schema
            "ShopId",       // store
            "cloudSid",     // employee
        ];
        for _ in 0..400 {
            let doc = gen_object(&mut rng, &cfg);
            // Every shaped document still carries the all-category sampler.
            assert!(doc.contains_key("_sampler"), "shaped doc missing _sampler");
            for sig in &signatures {
                if doc.contains_key(*sig) {
                    seen_signature_keys.insert(*sig);
                }
            }
        }
        assert!(
            seen_signature_keys.len() >= 4,
            "expected several distinct corpus shapes, saw signatures: {seen_signature_keys:?}"
        );
    }

    /// Prints a few sample generated documents as pretty JSON so the generator
    /// output can be eyeballed **offline** (no Cosmos account). Ignored by
    /// default; run explicitly with `--ignored --nocapture`:
    ///
    /// ```bash
    /// cargo test -p azure_data_cosmos --test binary_roundtrip_fuzzer --features key_auth,fault_injection,control_plane \
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
