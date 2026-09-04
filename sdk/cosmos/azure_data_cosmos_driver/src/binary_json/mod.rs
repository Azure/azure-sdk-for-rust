// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos **binary JSON** codec.
//!
//! Cosmos binary JSON is a tagged byte stream that the service can persist and
//! transmit in place of UTF-8 text JSON. A buffer begins with the preamble byte
//! [`PREAMBLE`] (`0x80`); because no valid UTF-8 text JSON document starts with
//! `0x80`, the first byte unambiguously distinguishes binary from text. Each
//! value is introduced by a single **type-marker** byte (see [`markers`]) that
//! selects how the following bytes are interpreted.
//!
//! This module is schema-agnostic: it operates purely on bytes and either
//! [`serde_json::Value`] (via [`decode`] / [`encode`]) or `serde` types
//! directly (via [`from_slice`] / [`to_vec`]).
//!
//! # Reading and writing
//!
//! - Read binary bytes into a typed value with [`from_slice`], or into a
//!   [`serde_json::Value`] with [`decode`].
//! - Write a typed value to binary bytes with [`to_vec`], or a
//!   [`serde_json::Value`] with [`encode`].
//!
//! The decoder accepts every wire form the service can emit. The encoder emits
//! a valid subset of those forms rather than the most compact encoding; because
//! the service accepts the verbose form, an encode/decode round-trip preserves
//! the original value.
//!
//! The wire constants in [`markers`] match the service byte-for-byte.

pub mod de;
pub mod error;
pub mod markers;
pub mod reader;
pub mod ser;
pub mod system_strings;
pub mod writer;

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod vectors;

pub use de::from_slice;
pub use error::{BinaryError, Result};
pub use reader::decode;
pub use ser::to_vec;
pub use writer::encode;

/// The Cosmos binary JSON preamble byte.
///
/// Every binary JSON buffer starts with this byte. It is the basis for
/// first-byte auto-detection ([`is_binary`]): no UTF-8 text JSON document can
/// begin with `0x80` (it is a continuation byte), so its presence reliably
/// distinguishes a binary buffer from a text one.
pub const PREAMBLE: u8 = 0x80;

/// Returns `true` if `buffer` appears to be Cosmos binary JSON.
///
/// Detection is the single-byte test described in the spec: a buffer is binary
/// iff its first byte is the [`PREAMBLE`]. An empty buffer is not binary.
///
/// This is intentionally independent of any HTTP content negotiation so the
/// response path can decode binary bodies even when headers are absent or
/// unexpected.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{is_binary, PREAMBLE};
///
/// assert!(is_binary(&[PREAMBLE, 0xD2])); // binary `true`
/// assert!(!is_binary(b"{\"id\":\"1\"}")); // text JSON
/// assert!(!is_binary(&[])); // empty
/// ```
pub fn is_binary(buffer: &[u8]) -> bool {
    buffer.first() == Some(&PREAMBLE)
}

/// Transcodes a Cosmos binary JSON buffer to UTF-8 **text** JSON.
///
/// This is the driver-side conversion used when an upstream SDK/app wants to
/// deal only with text JSON while still keeping the wire binary (efficient RUs
/// and network bandwidth): the request and the service response stay binary,
/// and the driver converts the binary response to text before handing it back.
///
/// Behavior:
///
/// - If `buffer` is Cosmos binary JSON (begins with the [`PREAMBLE`]), it is
///   decoded to a [`serde_json::Value`] and re-serialized as compact UTF-8 text
///   JSON (matching `serde_json::to_vec`).
/// - If `buffer` is already text JSON (or empty), it is returned **unchanged**
///   so the conversion is safe to apply unconditionally on a response whose
///   format was negotiated but not guaranteed.
///
/// Integral `Double`s render with integer syntax (`3`, not `3.0`), matching the
/// service's text mode, so a value read from a binary page matches the same
/// value read from a text page.
///
/// # Errors
///
/// Returns a [`BinaryError`] if `buffer` is binary but malformed, or if the
/// decoded value cannot be re-serialized as JSON.
pub fn transcode_to_text(buffer: &[u8]) -> Result<Vec<u8>> {
    if !is_binary(buffer) {
        // Already text (or empty): nothing to convert.
        return Ok(buffer.to_vec());
    }
    let mut value = decode(buffer)?;
    normalize_integral_floats(&mut value);
    serde_json::to_vec(&value)
        .map_err(|e| BinaryError::Custom(format!("failed to re-serialize decoded value: {e}")))
}

/// The integer form of an integral `Double`, or `None` when the value must stay
/// floating point.
///
/// The signed/unsigned split is not cosmetic: it selects which visitor a value
/// is fed to, and a `u64` above `i64::MAX` is rejected by signed targets.
pub(crate) enum IntegralDouble {
    Unsigned(u64),
    Signed(i64),
}

/// Classifies `float` as an integer when the service's text mode would render
/// it with integer syntax (`3`, not `3.0`).
///
/// This is the single rule behind both [`normalize_integral_floats`] (which
/// rewrites a decoded [`serde_json::Value`]) and the untyped decode path in
/// [`de`](crate::binary_json::de). Keeping one rule is what makes a binary page
/// decode to the same number types no matter which query pipeline carried it —
/// see the `integral_doubles_decode_identically_*` tests.
///
/// Bounds are exclusive at `2^64` so a cast cannot saturate; out-of-range
/// magnitudes stay floating point. Non-finite values return `None` because
/// `NaN.fract()` is `NaN`.
pub(crate) fn integral_double(float: f64) -> Option<IntegralDouble> {
    const U64_EXCLUSIVE_UPPER_BOUND: f64 = 18_446_744_073_709_551_616.0;

    if float.fract() != 0.0 {
        return None;
    }
    // `-0.0` is integral and compares equal to `0.0`, so it would fall into the
    // unsigned branch and coerce to `0`, losing the sign a locally-encoded
    // payload must round-trip byte-for-byte. (The service itself folds `-0.0`
    // to `0` at storage, so this only ever applies to client-side encoding.)
    if float == 0.0 && float.is_sign_negative() {
        return None;
    }
    if (0.0..U64_EXCLUSIVE_UPPER_BOUND).contains(&float) {
        Some(IntegralDouble::Unsigned(float as u64))
    } else if float < 0.0 && float >= i64::MIN as f64 {
        Some(IntegralDouble::Signed(float as i64))
    } else {
        None
    }
}

/// Rewrites every integral `f64` in `value` to an integer `Number`, matching
/// the service's text rendering of a stored `Double`.
pub(crate) fn normalize_integral_floats(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) if number.is_f64() => {
            let Some(float) = number.as_f64() else {
                return;
            };
            match integral_double(float) {
                Some(IntegralDouble::Unsigned(unsigned)) => {
                    *number = serde_json::Number::from(unsigned);
                }
                Some(IntegralDouble::Signed(signed)) => {
                    *number = serde_json::Number::from(signed);
                }
                None => {}
            }
        }
        serde_json::Value::Array(values) => {
            values.iter_mut().for_each(normalize_integral_floats);
        }
        serde_json::Value::Object(values) => {
            values.values_mut().for_each(normalize_integral_floats);
        }
        _ => {}
    }
}

/// Transcodes a UTF-8 **text** JSON buffer to Cosmos **binary** JSON.
///
/// This is the mirror of [`transcode_to_text`] for the **request** path: when a
/// schema-agnostic caller (for example an FFI host) deals only in text JSON but
/// the driver negotiates a binary wire, the driver converts the text request
/// body to binary before sending it. No item schema is required — the buffer is
/// parsed to a [`serde_json::Value`] and re-encoded.
///
/// Behavior:
///
/// - If `buffer` is already Cosmos binary JSON (begins with the [`PREAMBLE`]),
///   it is returned **unchanged** so the conversion is safe to apply
///   unconditionally (a caller that already encoded binary is not re-encoded).
/// - An empty buffer is returned unchanged (no body to encode).
/// - Otherwise `buffer` is parsed as text JSON and encoded to binary.
///
/// # Errors
///
/// Returns a [`BinaryError`] if `buffer` is neither binary nor valid text JSON,
/// or if it nests containers more than 256 deep — the same limit [`decode`]
/// accepts. Exceeding it yields [`BinaryError::DepthLimitExceeded`], whose
/// `limit` field reports the bound.
pub fn transcode_to_binary(buffer: &[u8]) -> Result<Vec<u8>> {
    if buffer.is_empty() || is_binary(buffer) {
        // Empty, or already binary: nothing to convert.
        return Ok(buffer.to_vec());
    }
    let value = parse_text_json(buffer)?;
    Ok(encode(&value))
}

/// Parses text JSON to a [`serde_json::Value`], accepting the same nesting
/// depth the binary decoder accepts.
///
/// `serde_json`'s built-in recursion guard stops at 128, but [`decode`] admits
/// [`reader::MAX_DEPTH`] (256, the service's own limit). Leaving the two
/// mismatched makes the driver accept a document on the way in that it cannot
/// re-encode on the way out, so a legal service document fails mid-pipeline.
///
/// Depth is therefore bounded by scanning the raw bytes *before* parsing, and
/// only then is the built-in guard disabled. The order matters: disabling it
/// first would let adversarial input recurse without limit and exhaust the
/// stack.
fn parse_text_json(buffer: &[u8]) -> Result<serde_json::Value> {
    check_text_json_depth(buffer)?;

    let mut deserializer = serde_json::Deserializer::from_slice(buffer);
    // Safe only because `check_text_json_depth` already bounded nesting.
    deserializer.disable_recursion_limit();
    let value = serde::Deserialize::deserialize(&mut deserializer)
        .map_err(|e| BinaryError::Custom(format!("failed to parse text JSON: {e}")))?;
    deserializer
        .end()
        .map_err(|e| BinaryError::Custom(format!("failed to parse text JSON: {e}")))?;
    Ok(value)
}

/// Rejects text JSON nesting containers deeper than [`reader::MAX_DEPTH`].
///
/// A byte scan rather than a parse, so it cannot itself recurse. Brackets
/// inside string literals are not structural and must not count, so the scan
/// tracks string state and skips the character after a backslash.
fn check_text_json_depth(buffer: &[u8]) -> Result<()> {
    let mut depth: usize = 0;
    let mut in_string = false;
    let mut escaped = false;

    for &byte in buffer {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'[' | b'{' => {
                depth += 1;
                if depth > reader::MAX_DEPTH {
                    return Err(BinaryError::DepthLimitExceeded {
                        limit: reader::MAX_DEPTH,
                    });
                }
            }
            // Saturating because a malformed buffer can close more containers
            // than it opened; `serde_json` reports the syntax error itself.
            b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preamble_is_0x80() {
        assert_eq!(PREAMBLE, 0x80);
        // The preamble shares its value with the start of the encoded-length
        // string range; that range begins at the same byte by design.
        assert_eq!(PREAMBLE, markers::ENCODED_STRING_LENGTH_MIN);
    }

    #[test]
    fn detects_binary_by_preamble() {
        assert!(is_binary(&[PREAMBLE]));
        assert!(is_binary(&[PREAMBLE, markers::TRUE]));
    }

    #[test]
    fn rejects_text_and_empty() {
        assert!(!is_binary(b"{}"));
        assert!(!is_binary(b"[1,2,3]"));
        assert!(!is_binary(b"\"hello\""));
        assert!(!is_binary(&[]));
        // A different leading byte is not binary even if 0x80 appears later.
        assert!(!is_binary(&[0x00, PREAMBLE]));
    }

    /// Builds `depth` nested arrays around a scalar without parsing, so the
    /// helper itself is not bound by any parser's recursion limit.
    #[cfg(test)]
    fn nested_arrays(depth: usize) -> serde_json::Value {
        let mut value = serde_json::Value::Number(1.into());
        for _ in 0..depth {
            value = serde_json::Value::Array(vec![value]);
        }
        value
    }

    /// The binary decoder admits [`reader::MAX_DEPTH`] nesting, so the text
    /// encoder must admit exactly the same. `serde_json`'s built-in guard stops
    /// at 128 — well inside what the service can legally send — and a document
    /// between the two limits would decode from a binary page and then fail to
    /// re-encode, taking the whole query page with it.
    #[test]
    fn transcodes_text_nested_to_the_binary_decoder_limit() {
        let deep = serde_json::to_vec(&nested_arrays(reader::MAX_DEPTH - 1)).unwrap();

        let binary = transcode_to_binary(&deep).expect("depth within MAX_DEPTH must re-encode");
        assert!(is_binary(&binary));
        // Round-trips through the decoder, proving the two limits agree.
        let text = transcode_to_text(&binary).expect("decoder must accept what the encoder emits");
        assert_eq!(text, deep);
    }

    #[test]
    fn rejects_text_nested_past_the_binary_decoder_limit() {
        let too_deep = serde_json::to_vec(&nested_arrays(reader::MAX_DEPTH + 1)).unwrap();

        let err = transcode_to_binary(&too_deep).expect_err("past MAX_DEPTH must be rejected");
        assert!(
            matches!(err, BinaryError::DepthLimitExceeded { limit } if limit == reader::MAX_DEPTH),
            "expected a depth-limit error, got {err:?}",
        );
    }

    /// Brackets inside a string literal are text, not structure. Counting them
    /// would reject an ordinary document carrying, say, a serialized JSON blob
    /// in a field.
    #[test]
    fn depth_scan_ignores_brackets_inside_strings() {
        let brackets = "[".repeat(reader::MAX_DEPTH * 2);
        let escaped_quote = format!(r#"{{"note":"{}\" {}"}}"#, brackets, brackets);

        let binary = transcode_to_binary(escaped_quote.as_bytes())
            .expect("brackets inside a string are not nesting");
        let decoded = decode(&binary).unwrap();
        assert_eq!(
            decoded["note"].as_str().unwrap(),
            format!("{}\" {}", brackets, brackets),
        );
    }

    /// A buffer closing more containers than it opens must not underflow the
    /// depth counter and mask a genuinely too-deep prefix.
    #[test]
    fn depth_scan_survives_unbalanced_closers() {
        let unbalanced = format!("{}1{}", "[".repeat(4), "]".repeat(40));
        // Malformed, so it fails to parse — but as a syntax error, not a panic.
        let err = transcode_to_binary(unbalanced.as_bytes()).expect_err("trailing closers");
        assert!(
            !matches!(err, BinaryError::DepthLimitExceeded { .. }),
            "unbalanced closers must not be reported as a depth error: {err:?}",
        );
    }

    #[test]
    fn transcode_round_trip_preserves_double_bits_exactly() {
        // The merged ORDER BY path re-encodes payloads by transcoding binary to
        // text and back, so that round trip must not perturb a double by even
        // one ULP. `96.182417728091792` is a live corpus value that serde_json's
        // default (non-`float_roundtrip`) parser decodes one ULP high, so this
        // guards the `float_roundtrip` feature staying enabled.
        let original = 96.182417728091792_f64;
        let source_text = serde_json::to_vec(&serde_json::json!({ "Lon": original })).unwrap();

        let binary = transcode_to_binary(&source_text).unwrap();
        let text = transcode_to_text(&binary).unwrap();
        let decoded = decode(&transcode_to_binary(&text).unwrap()).unwrap();

        let round_tripped = decoded["Lon"].as_f64().unwrap();
        assert_eq!(
            round_tripped.to_bits(),
            original.to_bits(),
            "double changed across the transcode round trip: {round_tripped} vs {original}",
        );
    }

    #[test]
    fn transcode_binary_to_text_produces_equivalent_json() {
        // A binary buffer transcodes to the same JSON serde_json would emit.
        let value = serde_json::json!({
            "id": "doc-1",
            "n": 42,
            "flag": true,
            "nested": { "arr": [1, 2, 3], "s": "café" },
        });
        let binary = encode(&value);
        assert!(is_binary(&binary));

        let text = transcode_to_text(&binary).unwrap();
        assert!(!is_binary(&text), "transcoded output must be text");

        // Bytes match serde_json::to_vec of the same value, and re-parse equal.
        assert_eq!(text, serde_json::to_vec(&value).unwrap());
        let reparsed: serde_json::Value = serde_json::from_slice(&text).unwrap();
        assert_eq!(reparsed, value);
    }

    #[test]
    fn transcode_renders_integral_double_as_integer() {
        // The service renders a stored integral `Double` as `3`.
        let binary = encode(&serde_json::json!({ "n": 3.0 }));
        assert_eq!(transcode_to_text(&binary).unwrap(), br#"{"n":3}"#);
    }

    #[test]
    fn transcode_keeps_fractional_double_unchanged() {
        let binary = encode(&serde_json::json!({ "n": 3.5 }));
        assert_eq!(transcode_to_text(&binary).unwrap(), br#"{"n":3.5}"#);
    }

    /// A binary page reaches an untyped caller by two different routes: the
    /// **passthrough** pipeline hands the envelope straight to `from_slice`,
    /// while the **merge** and **SkipTake** pipelines make a binary→text hop
    /// through [`transcode_to_text`] first. Both must produce the same
    /// `serde_json::Value`, and both must match what a text-mode response
    /// yields — `serde_json::Number`'s `PartialEq` is variant-sensitive, so
    /// `PosInt(3) != Float(3.0)` and adding `ORDER BY` to a query would
    /// otherwise silently change the JSON number type of every integral field.
    #[test]
    fn integral_doubles_decode_identically_on_every_pipeline() {
        // Every number here is stored as a `Double` on the wire.
        let stored = serde_json::json!({
            "small": 3.0,
            "negative": -7.0,
            "wide": 9_007_199_254_740_993_f64,
            "fractional": 3.5,
            "negative_zero": -0.0,
            "nested": { "arr": [1.0, 2.5, -3.0] },
        });
        let binary = encode(&stored);

        // Route 1 — passthrough: binary bytes deserialized directly.
        let passthrough: serde_json::Value = from_slice(&binary).unwrap();
        // Route 2 — merge / SkipTake: binary transcoded to text, then parsed.
        let via_text: serde_json::Value =
            serde_json::from_slice(&transcode_to_text(&binary).unwrap()).unwrap();

        assert_eq!(
            passthrough, via_text,
            "passthrough and the binary->text hop must agree on number types",
        );

        // Both must agree with the service's text rendering of the same values.
        let text_mode: serde_json::Value =
            serde_json::from_slice(br#"{"small":3,"negative":-7,"wide":9007199254740992,"fractional":3.5,"negative_zero":-0.0,"nested":{"arr":[1,2.5,-3]}}"#)
                .unwrap();
        assert_eq!(passthrough, text_mode);

        // Spot-check the variants directly: `assert_eq!` above relies on
        // `Number`'s variant sensitivity, so pin the intent explicitly.
        assert!(passthrough["small"].is_u64(), "integral double -> integer");
        assert!(passthrough["negative"].is_i64(), "negative integral -> i64");
        assert!(passthrough["fractional"].is_f64(), "fraction stays f64");
        assert!(
            passthrough["negative_zero"].is_f64(),
            "-0.0 must keep its sign rather than folding to 0",
        );
    }

    /// A float target still receives a float when the wire value is integral:
    /// serde's float visitors accept an integer visit, so coercing on the
    /// untyped path must not break typed `f64` fields.
    #[test]
    fn integral_double_still_deserializes_into_a_float_target() {
        #[derive(serde::Deserialize)]
        struct Doc {
            ratio: f64,
        }

        let binary = encode(&serde_json::json!({ "ratio": 4.0 }));
        let doc: Doc = from_slice(&binary).unwrap();
        assert_eq!(doc.ratio, 4.0);
    }

    /// `-0.0` is integral and `== 0.0`, so the unsigned coercion would fold it
    /// to `0` and drop the sign. A text response preserves `-0.0`, so binary
    /// must too or the two paths disagree.
    #[test]
    fn transcode_preserves_negative_zero() {
        let binary = encode(&serde_json::json!({ "n": -0.0 }));
        assert_eq!(transcode_to_text(&binary).unwrap(), br#"{"n":-0.0}"#);
    }

    /// Positive zero still normalizes, matching the service's text rendering.
    #[test]
    fn transcode_renders_positive_zero_as_integer() {
        let binary = encode(&serde_json::json!({ "n": 0.0 }));
        assert_eq!(transcode_to_text(&binary).unwrap(), br#"{"n":0}"#);
    }

    #[test]
    fn transcode_normalizes_integral_doubles_nested_in_containers() {
        let binary = encode(&serde_json::json!({ "a": [1.0, 2.5], "o": { "b": -4.0 } }));
        let text = transcode_to_text(&binary).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&text).unwrap();

        assert_eq!(value["a"][0].as_u64(), Some(1));
        assert!(value["a"][1].is_f64());
        assert_eq!(value["o"]["b"].as_i64(), Some(-4));
    }

    /// Out-of-range magnitudes stay floating point rather than saturating.
    #[test]
    fn transcode_leaves_out_of_range_integral_doubles_as_floats() {
        let binary = encode(&serde_json::json!({
            "i64_min": i64::MIN as f64,
            "too_big": 18_446_744_073_709_551_616.0_f64,
            "too_small": -18_446_744_073_709_551_616.0_f64,
        }));
        let text = transcode_to_text(&binary).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&text).unwrap();

        assert_eq!(value["i64_min"].as_i64(), Some(i64::MIN));
        assert!(value["too_big"].is_f64());
        assert!(value["too_small"].is_f64());
    }

    #[test]
    fn transcode_passes_text_through_unchanged() {
        // A text buffer is returned byte-for-byte unchanged.
        let text = br#"{"id":"1","n":7}"#;
        assert_eq!(transcode_to_text(text).unwrap(), text);
    }

    #[test]
    fn transcode_passes_empty_through_unchanged() {
        assert_eq!(transcode_to_text(&[]).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn transcode_errors_on_malformed_binary() {
        // A lone preamble is not a complete value.
        assert!(transcode_to_text(&[PREAMBLE]).is_err());
    }

    #[test]
    fn transcode_text_to_binary_produces_equivalent_binary() {
        // A text buffer transcodes to binary that decodes back to the same value.
        let value = serde_json::json!({
            "id": "doc-1",
            "n": 42,
            "flag": true,
            "nested": { "arr": [1, 2, 3], "s": "café" },
        });
        let text = serde_json::to_vec(&value).unwrap();
        assert!(!is_binary(&text));

        let binary = transcode_to_binary(&text).unwrap();
        assert!(is_binary(&binary), "transcoded output must be binary");

        // Bytes match the encoder oracle, and decode back to the same value.
        assert_eq!(binary, encode(&value));
        assert_eq!(decode(&binary).unwrap(), value);
    }

    #[test]
    fn transcode_to_binary_passes_binary_through_unchanged() {
        // A buffer that is already binary is returned byte-for-byte unchanged.
        let value = serde_json::json!({ "id": "1", "n": 7 });
        let binary = encode(&value);
        assert_eq!(transcode_to_binary(&binary).unwrap(), binary);
    }

    #[test]
    fn transcode_to_binary_passes_empty_through_unchanged() {
        assert_eq!(transcode_to_binary(&[]).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn transcode_to_binary_errors_on_invalid_text() {
        // Not binary and not valid JSON.
        assert!(transcode_to_binary(b"{not json").is_err());
    }

    #[test]
    fn transcode_round_trips_text_binary_text() {
        // text → binary → text is identity for a well-formed document.
        let value = serde_json::json!({ "a": 1, "b": ["x", "y"], "c": null });
        let text = serde_json::to_vec(&value).unwrap();
        let binary = transcode_to_binary(&text).unwrap();
        let back = transcode_to_text(&binary).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&back).unwrap(),
            value
        );
    }
}
