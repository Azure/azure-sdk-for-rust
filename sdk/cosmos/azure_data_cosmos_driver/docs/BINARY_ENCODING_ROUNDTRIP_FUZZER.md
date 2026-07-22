# Binary-Encoding Round-Trip Fuzzer — Design

**Status:** Draft · **Companion harness:** `azure_data_cosmos_perf/tests/binary_roundtrip_fuzzer.rs`

## 1. Goal

Validate that **arbitrary JSON survives a full Cosmos round-trip unchanged**,
across binary-encoding configurations, at high volume. Where the deterministic
golden vectors ([`binary_json_vectors.json`](../testdata/binary_json_vectors.json))
prove specific byte layouts and the in-tree fuzz suite hammers the *decoder* with
malformed input, this harness exercises the **end-to-end path**:

```
generate JSON → Rust encode → wire → backend store+rewrite → wire → Rust decode → compare
```

A single machine can validate **millions of distinct JSON structures over a few
days**, which no hand-written test set can approach.

This directly implements the reviewer request (FabianMeiswinkel, PR #4671):

> a tool that can produce random json structures and does e2e validation (via
> canonicalization + hash) — this would allow us to test millions of differently
> structured json objects and increase confidence level.

## 2. Core idea: canonicalize + compare

For each generated document `D`:

1. `H0 = hash(canonicalize(D))`
2. For each config `C` (binary on/off, text-response on/off, …):
   - store `D`, read back `R`
   - `Hc = hash(canonicalize(project(R, keys(D))))`
   - **assert `Hc == H0`** — otherwise dump the seed + both canonical forms.

`project(R, keys(D))` strips the service-added system fields (`_rid`, `_etag`,
`_ts`, `_self`, `_attachments`) so only the fields we control are compared.

The harness compares **canonical strings directly** (strongest signal — it can
print the exact diff) *and* logs a 64-bit hash so the "store `H0` once, compare
later" workflow is available for a persistent corpus. The hash is
`std::hash::DefaultHasher` (SipHash): we are detecting *differences*, not
defending against adversarial collisions, so a cryptographic digest is
unnecessary. Swap in SHA-256 if a durable cross-run corpus is later desired.

## 3. Canonicalization (the hard part)

Two JSON texts are "the same value" if they canonicalize identically. Rules:

| Aspect      | Rule |
| ----------- | ---- |
| Whitespace  | removed entirely |
| Object keys | sorted lexicographically (by UTF-8 code unit) |
| Strings     | minimally JSON-escaped (via `serde_json`) |
| Arrays      | order preserved |
| **Numbers** | **Cosmos-compatible normalization — see §3.1** |

### 3.1 Number normalization (the tuning surface)

The subtlety the reviewer flagged: **the backend rewrites numbers on store**, and
its rewrite is *not* identical to a strict canonicalizer like
[JCS / RFC 8785](https://datatracker.ietf.org/doc/html/rfc8785). If we
canonicalized with JCS but the backend renders `1.0` as `1`, a faithful
round-trip would *falsely* report a mismatch.

The harness therefore uses a **Cosmos-compatible** number canonicalizer, not JCS:

- **Integers** (fit `i64`/`u64`): emit plain decimal, no decimal point, no
  exponent, no leading zeros. `-0` → `0`.
- **Integral-valued floats** (e.g. `1.0`, `2.0e1`): normalized to their integer
  form (`1`, `20`) — this mirrors the observed backend rewrite where a trailing
  `.0` is dropped.
- **Non-integral floats**: shortest round-trippable decimal (Rust's `ryu`, via
  `serde_json`'s `f64` formatting).

> **This rule set is a starting point and MUST be calibrated against a real
> account.** Run the harness in calibration mode
> (`AZURE_COSMOS_FUZZ_CALIBRATE=true`, see §6) to have it store a fixed spread of
> numeric edge cases through the binary path, read them back, and print a table
> comparing how `canonicalize_number` renders each value against the backend's
> actual returned form. Every `DIFF` row is a form the canonicalizer does not yet
> model — tune `canonicalize_number` until the table is all `MATCH`. The probe
> set (`NUMBER_PROBES` in the harness) covers: integral floats (`1.0`, `2e1`),
> repeating/high-precision floats (`0.1`, `0.1 + 0.2`, π), large/small exponents
> (`1e20`, `1e-20`), integers near `2^63`/`2^64`, negative zero, and trailing
> zeros (`1.2300`). Calibration is a **diagnostic** — it prints the table and
> does not assert, since a `DIFF` on the first run is the expected signal to
> tune, not a failure.

### 3.2 Generator stays inside the calibrated envelope

To avoid false positives from *un-calibrated* number forms, the generator emits
numbers in **backend-safe ranges by default** (bounded integers, bounded-precision
floats). A `--wide-numbers` flag widens the range once the canonicalizer is
calibrated for those forms — this is how you progressively expand coverage.

## 4. Generator

A seeded PRNG (`StdRng`, seed logged for exact reproduction) produces a random
JSON **object** (Cosmos items are objects) with:

- bounded nesting depth (default 5),
- mixed-type and uniform-type arrays,
- nested sub-objects and arrays-of-objects,
- strings of varied length, optionally including Unicode (BMP + astral),
- numbers across the calibrated envelope (§3.2),
- `null`, `true`, `false`.

Every run prints its seed; a failing run is reproduced exactly with
`AZURE_COSMOS_FUZZ_SEED=<seed>`.

## 5. Configurations exercised

Each config is a separate `CosmosClient` (binary encoding is resolved once at
build time):

| Config | `enabled` | `request_text_response` |
| ------ | --------- | ----------------------- |
| control (text) | false | — |
| binary | true | false |
| binary + text response | true | true |

Extend with two accounts (dictionary encoding on/off) by pointing
`AZURE_COSMOS_FUZZ_CONNECTION_STRING_2` at a second account — the harness runs
every generated doc through both.

## 6. Running the harness

```bash
# One-shot smoke run (a few hundred docs), local emulator:
AZURE_COSMOS_CONNECTION_STRING='AccountEndpoint=...;AccountKey=...;' \
AZURE_COSMOS_ALLOW_INVALID_CERT=true \
RUSTFLAGS='--cfg test_category="binary_encoding"' \
  cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer -- --nocapture

# Multi-day soak (millions of docs):
AZURE_COSMOS_CONNECTION_STRING='...' \
AZURE_COSMOS_FUZZ_ITERATIONS=5000000 \
AZURE_COSMOS_FUZZ_MAX_DEPTH=6 \
RUSTFLAGS='--cfg test_category="binary_encoding"' \
  cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer --release -- --nocapture

# Reproduce a failure:
AZURE_COSMOS_FUZZ_SEED=12345678901234567890 ... cargo test ...

# Calibrate number canonicalization against the account (prints a table, no assert):
AZURE_COSMOS_CONNECTION_STRING='...' AZURE_COSMOS_FUZZ_CALIBRATE=true \
RUSTFLAGS='--cfg test_category="binary_encoding"' \
  cargo test -p azure_data_cosmos_perf --test binary_roundtrip_fuzzer -- --nocapture
```

### Environment knobs

| Variable | Default | Meaning |
| -------- | ------- | ------- |
| `AZURE_COSMOS_CONNECTION_STRING` | — (required) | live account (endpoint + key) |
| `AZURE_COSMOS_ALLOW_INVALID_CERT` | false | accept emulator cert |
| `AZURE_COSMOS_FUZZ_ITERATIONS` | 200 | number of generated docs |
| `AZURE_COSMOS_FUZZ_SEED` | random | PRNG seed (for reproduction) |
| `AZURE_COSMOS_FUZZ_MAX_DEPTH` | 5 | max JSON nesting depth |
| `AZURE_COSMOS_FUZZ_WIDE_NUMBERS` | false | widen numeric range (post-calibration) |
| `AZURE_COSMOS_FUZZ_UNICODE` | true | include Unicode strings |
| `AZURE_COSMOS_FUZZ_CALIBRATE` | false | number-calibration mode (§3.1) |
| `AZURE_COSMOS_BINARY_TEST_DATABASE` / `_CONTAINER` | `binary-fuzz-*` | target names |

## 7. What a failure tells you

A mismatch is one of:

1. **A real codec bug** — Rust encoded or decoded a value wrong. (The golden
   vectors + in-tree fuzz should also then be extended with the reduced case.)
2. **A canonicalization gap** — the backend rewrote a number/string in a form the
   canonicalizer doesn't yet model. Fix `canonicalize_number` (§3.1) and, if the
   form is legitimately out of scope, narrow the generator.
3. **A backend rewrite difference** — genuinely different value after store; this
   is the highest-value finding and should be escalated.

The harness prints the seed, the config, and both canonical forms so the case is
immediately reproducible and reducible.

## 8. Relationship to the other test layers

The normative wire format is defined by
[`BINARY_ENCODING_RFC.md`](BINARY_ENCODING_RFC.md); this harness is one of the
mechanisms that **validates Rust against that spec** — specifically the RFC's §7
round-trip invariant, exercised end-to-end at scale (see the RFC's §1.4 diagram
for how all the artifacts relate).

| Layer | Input | Checks | Location |
| ----- | ----- | ------ | -------- |
| RFC | — | normative wire spec (source of truth) | `docs/BINARY_ENCODING_RFC.md` |
| Golden vectors | fixed corpus | exact byte layout, decode parity | `binary_json/vectors.rs` |
| Encoder conformance | fixed corpus | encode byte-exactness, canonical form | `binary_json/conformance.rs` |
| In-tree fuzz | random/truncated buffers | decoder never panics; `decode` ≡ `from_slice` | `binary_json/fuzz_tests.rs` |
| **Round-trip fuzzer** | **random JSON** | **end-to-end value fidelity across configs** | **this harness** |

These are complementary: golden vectors + conformance pin *the format*, in-tree
fuzz hardens *the decoder*, and this harness validates *the whole pipeline
against the live service* — and its number-canonicalization calibration (§3.1)
feeds the backend's observed rewrite rules back into the RFC's §7.
