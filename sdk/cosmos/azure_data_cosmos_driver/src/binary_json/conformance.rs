// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Encode-direction conformance tests for the Cosmos binary JSON codec.
//!
//! These tests implement the encoder conformance requirements of the
//! wire-format spec (`docs/BINARY_ENCODING_RFC.md` §7 "Canonical encoding" and
//! Appendix A "Golden test vectors"). Decoder conformance (§8) is covered by the
//! per-form tests in [`reader`](super::reader) and [`de`](super::de); this
//! module fills the previously-missing encode side.
//!
//! Two guarantees are asserted:
//!
//! 1. **Round-trip validity** — for every golden-corpus value, `decode(encode(v))
//!    == v`. The encoder MUST emit a valid buffer that decodes back to the input
//!    (RFC §7: the encoder emits a conformant *subset* of the wire forms; the
//!    decoder accepts all of them).
//! 2. **Canonical output snapshots** — the encoder is deterministic, so its exact
//!    bytes for representative values are pinned as regression snapshots. This
//!    documents the Rust encoder's actual canonical form, which is a valid
//!    subset that does **not** use the most compact forms (system strings,
//!    `Arr0`/`Arr1`, narrowest `Number*`, etc.) — see the notes below.
//!
//! The module is compiled only under `cfg(test)`.

use super::{decode, encode, is_binary, PREAMBLE};
use serde_json::{json, Value};

/// Parses a spaced-hex string (e.g. `"80 D0"`) into bytes, matching the corpus
/// notation used throughout the RFC.
fn hex(s: &str) -> Vec<u8> {
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).expect("valid hex byte"))
        .collect()
}

/// RFC §7 (round-trip validity): every value in the shared golden corpus MUST
/// re-encode to a buffer that decodes back to the identical value. This is the
/// encode-direction counterpart to `reader::decodes_golden_corpus`.
#[test]
fn encode_round_trips_golden_corpus() {
    for vector in super::vectors::golden_vectors() {
        let expected: Value =
            serde_json::from_str(&vector.json).expect("corpus json is valid JSON");
        let encoded = encode(&expected);
        assert!(
            is_binary(&encoded),
            "{}: encoder output missing preamble",
            vector.name
        );
        let decoded = decode(&encoded)
            .unwrap_or_else(|e| panic!("{}: re-encoded buffer failed to decode: {e}", vector.name));
        assert_eq!(
            decoded, expected,
            "{}: encode→decode did not round-trip",
            vector.name
        );
    }
}

/// RFC §7 (deterministic canonical output): the encoder emits exactly these
/// bytes for representative values. These snapshots are the regression bar for
/// the Rust encoder's canonical form.
///
/// Note the encoder deliberately emits a **valid but non-minimal subset** of the
/// wire forms (RFC §7): integers outside `[0,31]` use `Int64`/`UInt64` (never the
/// narrower `NumberUInt8`/`Int16`/`Int32`), strings use the encoded-length or
/// `StrL*` forms (never system/user/compressed strings), and containers always
/// use the `LC*` length+count framing (never `Arr0`/`Arr1`/`Obj0`/`Obj1`). The
/// decoder accepts the compact forms the service may emit; the encoder need not
/// produce them.
#[test]
fn encode_produces_expected_canonical_bytes() {
    let cases: &[(Value, &str)] = &[
        // Singletons.
        (json!(null), "80 D0"),
        (json!(false), "80 D1"),
        (json!(true), "80 D2"),
        // Literal small integers (value == marker), 0..=31.
        (json!(0), "80 00"),
        (json!(31), "80 1F"),
        // Integers outside [0,31] use Int64 (not the narrower Number* forms).
        (json!(32), "80 CB 20 00 00 00 00 00 00 00"),
        (json!(200), "80 CB C8 00 00 00 00 00 00 00"),
        (json!(-5), "80 CB FB FF FF FF FF FF FF FF"),
        // Values above i64::MAX use UInt64.
        (
            json!(18446744073709551614u64),
            "80 C7 FE FF FF FF FF FF FF FF",
        ),
        // Non-integral numbers use NumberDouble.
        (json!(3.5), "80 CC 00 00 00 00 00 00 0C 40"),
        // Strings ≤ 63 bytes use the encoded-length form (length baked into the
        // marker), including "hello" — the corpus stores it as StrL1, but the
        // encoder's canonical form is encoded-length.
        (json!(""), "80 80"),
        (json!("hi"), "80 82 68 69"),
        (json!("hello"), "80 85 68 65 6C 6C 6F"),
        // Containers always use LC* framing (byte-length + count).
        (json!([]), "80 E5 00 00"),
        (json!([true]), "80 E5 01 01 D2"),
        (json!({}), "80 ED 00 00"),
        (json!({"id": 0}), "80 ED 04 01 82 69 64 00"),
    ];

    for (value, expected_hex) in cases {
        let encoded = encode(value);
        let expected = hex(expected_hex);
        assert_eq!(
            encoded, expected,
            "encoder output for {value} did not match the canonical snapshot\n  expected: {expected_hex}\n  actual:   {}",
            spaced_hex(&encoded),
        );
    }
}

/// RFC §7 (valid subset): where the golden corpus stores a **compact** wire form
/// the encoder does not emit (system strings, `Arr0`, `NumberUInt8`, …), the
/// encoder's own output differs byte-wise but still decodes to the same value.
/// This pins the intentional asymmetry so a future "make the encoder compact"
/// change is a conscious decision rather than a silent regression.
#[test]
fn encoder_emits_valid_subset_for_compact_corpus_forms() {
    // (value, the corpus's compact encoding) — the encoder produces a *different*
    // buffer, but both decode to `value`.
    let compact_cases: &[(Value, &str)] = &[
        (json!(200), "80 C8 C8"),    // corpus: NumberUInt8
        (json!("id"), "80 2C"),      // corpus: system string
        (json!([]), "80 E0"),        // corpus: Arr0
        (json!({}), "80 E8"),        // corpus: Obj0
        (json!([true]), "80 E1 D2"), // corpus: Arr1
        (
            json!([1, 2, 3]),
            "80 F0 DA 03 01 00 00 00 02 00 00 00 03 00 00 00",
        ), // uniform array
    ];

    for (value, compact_hex) in compact_cases {
        let compact = hex(compact_hex);
        // The compact form is valid and decodes to `value` ...
        assert_eq!(
            decode(&compact).unwrap(),
            *value,
            "compact corpus form {compact_hex} did not decode to {value}",
        );
        // ... but the encoder emits a different (verbose) buffer.
        let encoded = encode(value);
        assert_ne!(
            encoded, compact,
            "encoder unexpectedly produced the compact form for {value}; update this test if the encoder was made compact",
        );
        // ... which still decodes to the same value.
        assert_eq!(
            decode(&encoded).unwrap(),
            *value,
            "encoder's verbose form for {value} did not round-trip",
        );
    }
}

/// RFC §3.1: a complete buffer begins with the preamble and the encoder always
/// emits it.
#[test]
fn encoder_always_emits_preamble() {
    for value in [
        json!(null),
        json!(1),
        json!("x"),
        json!([1]),
        json!({"a": 1}),
    ] {
        let encoded = encode(&value);
        assert_eq!(
            encoded.first(),
            Some(&PREAMBLE),
            "missing preamble for {value}"
        );
    }
}

/// Formats bytes as spaced uppercase hex for assertion messages.
fn spaced_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}
