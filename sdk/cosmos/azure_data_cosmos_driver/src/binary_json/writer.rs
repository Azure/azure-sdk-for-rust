// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos binary JSON **encoder** ([`serde_json::Value`] → `binary`).
//!
//! The encoder is **minimal-but-valid**: it produces a *correct* (not
//! size-optimal) binary buffer that the service accepts, using the smallest set
//! of forms needed to represent any [`serde_json::Value`]:
//!
//! - `null` / `false` / `true` singletons,
//! - numbers as a literal int (`0`–`31`), `Int64`, `UInt64`, or `Double`,
//! - strings as an encoded-length string (≤ 63 bytes) or `StrL1`/`StrL2`/`StrL4`,
//! - arrays and objects as the length+count `ArrLC*` / `ObjLC*` forms.
//!
//! It deliberately **skips** the size optimizations a writer *may* apply —
//! system/user strings, reference-string dedup, compressed strings, the compact
//! `Arr0`/`Arr1`/`Obj0`/`Obj1` container forms, and uniform number arrays. The
//! decoder ([`decode`](super::decode)) handles all of those, so an encode →
//! decode round-trip reproduces the original value even though the encoder emits
//! only the verbose forms.
//!
//! Encoding is the trusted in-memory direction (the input is a value the SDK
//! constructed, not untrusted wire bytes), so it is infallible.

use serde_json::Value;

use super::markers::{
    ARR_LC1, ARR_LC2, ARR_LC4, ENCODED_STRING_LENGTH_MAX, ENCODED_STRING_LENGTH_MIN, FALSE,
    LITERAL_INT_MAX, NULL, NUMBER_DOUBLE, NUMBER_INT64, NUMBER_UINT64, OBJ_LC1, OBJ_LC2, OBJ_LC4,
    STR_L1, STR_L2, STR_L4, TRUE,
};
use super::PREAMBLE;

/// The number of distinct encoded-length string markers, i.e. the maximum
/// string length (in bytes) that fits the encoded-length form (`0`–`63`).
const ENCODED_STRING_LENGTH_SPAN: usize =
    (ENCODED_STRING_LENGTH_MAX - ENCODED_STRING_LENGTH_MIN) as usize;

/// Encodes a [`serde_json::Value`] into a complete Cosmos binary JSON buffer.
///
/// The returned buffer begins with the [`PREAMBLE`] byte
/// (`0x80`) and can be round-tripped back through [`decode`](super::decode).
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{decode, encode};
///
/// let value = serde_json::json!({ "id": "1", "count": 7 });
/// let bytes = encode(&value);
/// assert_eq!(decode(&bytes).unwrap(), value);
/// ```
pub fn encode(value: &Value) -> Vec<u8> {
    let mut out = vec![PREAMBLE];
    encode_value(value, &mut out);
    out
}

/// Appends the encoding of `value` (its type marker and payload) to `out`.
fn encode_value(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.push(NULL),
        Value::Bool(false) => out.push(FALSE),
        Value::Bool(true) => out.push(TRUE),
        Value::Number(n) => encode_number(n, out),
        Value::String(s) => encode_string(s, out),
        Value::Array(items) => {
            let mut body = Vec::new();
            for item in items {
                encode_value(item, &mut body);
            }
            encode_container([ARR_LC1, ARR_LC2, ARR_LC4], items.len(), &body, out);
        }
        Value::Object(map) => {
            let mut body = Vec::new();
            for (key, val) in map {
                encode_string(key, &mut body);
                encode_value(val, &mut body);
            }
            encode_container([OBJ_LC1, OBJ_LC2, OBJ_LC4], map.len(), &body, out);
        }
    }
}

/// Encodes a JSON number as a literal int (`0`–`31`), `Int64`, `UInt64`, or
/// `Double` — the minimal set that covers every [`serde_json::Number`].
fn encode_number(n: &serde_json::Number, out: &mut Vec<u8>) {
    if let Some(i) = n.as_i64() {
        if (0..i64::from(LITERAL_INT_MAX)).contains(&i) {
            // Literal int: the value is the marker.
            out.push(i as u8);
        } else {
            out.push(NUMBER_INT64);
            out.extend_from_slice(&i.to_le_bytes());
        }
    } else if let Some(u) = n.as_u64() {
        // Only reached when the value exceeds `i64::MAX` (so `as_i64` is `None`).
        out.push(NUMBER_UINT64);
        out.extend_from_slice(&u.to_le_bytes());
    } else {
        // A `serde_json::Number` that is neither `i64` nor `u64` is an `f64`,
        // and JSON numbers are always finite, so `as_f64` yields a value here.
        let f = n
            .as_f64()
            .expect("serde_json::Number is i64, u64, or finite f64");
        out.push(NUMBER_DOUBLE);
        out.extend_from_slice(&f.to_le_bytes());
    }
}

/// Encodes a string as an encoded-length string (≤ 63 bytes, length baked into
/// the marker) or a length-prefixed `StrL1`/`StrL2`/`StrL4`.
fn encode_string(s: &str, out: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len < ENCODED_STRING_LENGTH_SPAN {
        out.push(ENCODED_STRING_LENGTH_MIN | (len as u8));
    } else if len <= u8::MAX as usize {
        out.push(STR_L1);
        out.push(len as u8);
    } else if len <= u16::MAX as usize {
        out.push(STR_L2);
        out.extend_from_slice(&(len as u16).to_le_bytes());
    } else {
        // Cosmos caps request bodies far below `u32::MAX`, so a `u32` length is
        // always sufficient on the data plane.
        out.push(STR_L4);
        out.extend_from_slice(&(len as u32).to_le_bytes());
    }
    out.extend_from_slice(bytes);
}

/// Writes a length+count container: the marker, the payload byte length, the
/// item/member count, then the pre-encoded `body`. The narrowest of the three
/// `LC1`/`LC2`/`LC4` markers whose length and count fields both fit is used.
fn encode_container(lc_markers: [u8; 3], count: usize, body: &[u8], out: &mut Vec<u8>) {
    let [lc1, lc2, lc4] = lc_markers;
    let len = body.len();
    if len <= u8::MAX as usize && count <= u8::MAX as usize {
        out.push(lc1);
        out.push(len as u8);
        out.push(count as u8);
    } else if len <= u16::MAX as usize && count <= u16::MAX as usize {
        out.push(lc2);
        out.extend_from_slice(&(len as u16).to_le_bytes());
        out.extend_from_slice(&(count as u16).to_le_bytes());
    } else {
        out.push(lc4);
        out.extend_from_slice(&(len as u32).to_le_bytes());
        out.extend_from_slice(&(count as u32).to_le_bytes());
    }
    out.extend_from_slice(body);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::vectors::SCALAR_VECTORS;
    use crate::binary_json::{decode, is_binary, markers};
    use serde_json::json;

    /// Encodes `value`, asserts the buffer is recognized as binary, decodes it
    /// back, and asserts the round-trip reproduces the original value.
    fn round_trip(value: Value) {
        let encoded = encode(&value);
        assert!(
            is_binary(&encoded),
            "encoded buffer must start with the preamble: {value:?}",
        );
        let decoded =
            decode(&encoded).unwrap_or_else(|e| panic!("decode failed for {value:?}: {e}"));
        assert_eq!(decoded, value);
    }

    #[test]
    fn round_trips_null_and_booleans() {
        round_trip(Value::Null);
        round_trip(json!(true));
        round_trip(json!(false));
    }

    #[test]
    fn round_trips_integers() {
        for n in [0i64, 1, 30, 31, 32, 100, -1, -1000, i64::MIN, i64::MAX] {
            round_trip(json!(n));
        }
        // Unsigned values beyond i64::MAX use the UInt64 form.
        round_trip(json!(i64::MAX as u64 + 1));
        round_trip(json!(u64::MAX));
    }

    #[test]
    fn round_trips_floats() {
        for f in [0.0f64, 1.5, -2.25, 123.456_789, f64::MIN, f64::MAX] {
            round_trip(json!(f));
        }
    }

    #[test]
    fn round_trips_strings() {
        round_trip(json!(""));
        round_trip(json!("a"));
        round_trip(json!("id"));
        round_trip(json!("x".repeat(63))); // max encoded-length form
        round_trip(json!("y".repeat(64))); // first StrL1
        round_trip(json!("z".repeat(255))); // max StrL1
        round_trip(json!("p".repeat(256))); // first StrL2
        round_trip(json!("q".repeat(70_000))); // StrL4
        round_trip(json!("unicode: café ☃ 𝄞 \" \\ \n"));
    }

    #[test]
    fn round_trips_arrays() {
        round_trip(json!([]));
        round_trip(json!([1, 2, 3]));
        round_trip(json!([null, true, "x", 3.5]));
        round_trip(json!([[1, 2], [3, [4, 5]]]));
    }

    #[test]
    fn round_trips_objects() {
        round_trip(json!({}));
        round_trip(json!({ "id": "7" }));
        round_trip(json!({ "a": 1, "b": [2, 3], "c": { "d": true } }));
    }

    #[test]
    fn round_trips_nested_document() {
        round_trip(json!({
            "id": "doc-1",
            "_rid": "abc==",
            "count": 42,
            "ratio": 0.125,
            "active": true,
            "tags": ["x", "y", "z"],
            "nested": { "deep": { "value": [1, 2, { "k": null }] } },
        }));
    }

    #[test]
    fn round_trips_large_array_for_two_byte_container() {
        // 300 single-byte elements push the container past the 1-byte length and
        // count fields, exercising the ArrLC2 width selection.
        let value = Value::Array((0..300).map(|_| json!(0)).collect());
        round_trip(value);
    }

    #[test]
    fn round_trips_scalar_corpus_values() {
        // Encoding each golden vector's JSON then decoding reproduces the value
        // (the minimal encoder need not reproduce the exact golden bytes).
        for vector in SCALAR_VECTORS {
            let value: Value = serde_json::from_str(vector.json).unwrap();
            round_trip(value);
        }
    }

    #[test]
    fn encodes_literal_int_in_marker() {
        // Small non-negative integers are encoded as just the literal-int marker.
        assert_eq!(encode(&json!(0)), vec![PREAMBLE, 0]);
        assert_eq!(encode(&json!(7)), vec![PREAMBLE, 7]);
        assert_eq!(encode(&json!(31)), vec![PREAMBLE, 31]);
    }

    #[test]
    fn encodes_empty_array_as_length_count_form() {
        // Empty array -> ArrLC1 with length 0 and count 0.
        assert_eq!(encode(&json!([])), vec![PREAMBLE, markers::ARR_LC1, 0, 0],);
    }

    #[test]
    fn encodes_empty_object_as_length_count_form() {
        assert_eq!(encode(&json!({})), vec![PREAMBLE, markers::OBJ_LC1, 0, 0],);
    }

    #[test]
    fn encodes_empty_string_as_encoded_length_marker() {
        // Empty string -> a single encoded-length marker (0x80 | 0).
        assert_eq!(
            encode(&json!("")),
            vec![PREAMBLE, markers::ENCODED_STRING_LENGTH_MIN],
        );
    }
}
