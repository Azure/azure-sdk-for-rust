// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos binary JSON encoder ([`serde_json::Value`] → `binary`).
//!
//! [`encode`] produces a valid binary buffer for any [`serde_json::Value`],
//! using a subset of the wire forms:
//!
//! - `null` / `false` / `true` singletons,
//! - numbers as a literal int (`0`–`31`), `Int64`, `UInt64`, or `Double`,
//! - strings as an encoded-length string (≤ 63 bytes) or `StrL1`/`StrL2`/`StrL4`,
//! - arrays and objects as the length+count `ArrLC*` / `ObjLC*` forms.
//!
//! It does not emit the more compact forms (system/user strings,
//! reference-string dedup, compressed strings, the `Arr0`/`Arr1`/`Obj0`/`Obj1`
//! container forms, or uniform number arrays). The decoder accepts all of
//! those, so an encode/decode round-trip reproduces the original value.

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

/// The `LC1`/`LC2`/`LC4` length+count markers for **arrays**, passed to
/// [`encode_container`]. Shared with the native serde serializer.
pub(super) const ARRAY_LC_MARKERS: [u8; 3] = [ARR_LC1, ARR_LC2, ARR_LC4];

/// The `LC1`/`LC2`/`LC4` length+count markers for **objects**, passed to
/// [`encode_container`]. Shared with the native serde serializer.
pub(super) const OBJECT_LC_MARKERS: [u8; 3] = [OBJ_LC1, OBJ_LC2, OBJ_LC4];

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
            encode_container(ARRAY_LC_MARKERS, items.len(), &body, out);
        }
        Value::Object(map) => {
            let mut body = Vec::new();
            for (key, val) in map {
                encode_string(key, &mut body);
                encode_value(val, &mut body);
            }
            encode_container(OBJECT_LC_MARKERS, map.len(), &body, out);
        }
    }
}

/// Encodes a JSON number as a literal int (`0`–`31`), `Int64`, `UInt64`, or
/// `Double` — the minimal set that covers every [`serde_json::Number`].
fn encode_number(n: &serde_json::Number, out: &mut Vec<u8>) {
    if let Some(i) = n.as_i64() {
        encode_i64(i, out);
    } else if let Some(u) = n.as_u64() {
        // Only reached when the value exceeds `i64::MAX` (so `as_i64` is `None`).
        encode_u64(u, out);
    } else {
        // A `serde_json::Number` that is neither `i64` nor `u64` is an `f64`,
        // and JSON numbers are always finite, so `as_f64` yields a value here.
        let f = n
            .as_f64()
            .expect("serde_json::Number is i64, u64, or finite f64");
        encode_f64(f, out);
    }
}

/// Encodes a signed integer as a literal int (`0`–`31`) or `Int64`.
///
/// Shared by the [`Value`]-based [`encode`] path and the native serde
/// serializer so both emit identical bytes for the same integer.
pub(super) fn encode_i64(i: i64, out: &mut Vec<u8>) {
    if (0..i64::from(LITERAL_INT_MAX)).contains(&i) {
        // Literal int: the value is the marker.
        out.push(i as u8);
    } else {
        out.push(NUMBER_INT64);
        out.extend_from_slice(&i.to_le_bytes());
    }
}

/// Encodes an unsigned integer as a literal int (`0`–`31`), `Int64` (when it
/// still fits `i64`), or `UInt64`.
///
/// Shared by the [`Value`]-based [`encode`] path and the native serde
/// serializer.
pub(super) fn encode_u64(u: u64, out: &mut Vec<u8>) {
    if let Ok(i) = i64::try_from(u) {
        // Fits `i64`, so route through the signed path to keep literal-int and
        // `Int64` selection identical to the `Value` encoder.
        encode_i64(i, out);
    } else {
        out.push(NUMBER_UINT64);
        out.extend_from_slice(&u.to_le_bytes());
    }
}

/// Encodes a floating-point number as an IEEE-754 `Double`.
///
/// Shared by the [`Value`]-based [`encode`] path and the native serde
/// serializer.
pub(super) fn encode_f64(f: f64, out: &mut Vec<u8>) {
    out.push(NUMBER_DOUBLE);
    out.extend_from_slice(&f.to_le_bytes());
}

/// Encodes a string as an encoded-length string (≤ 63 bytes, length baked into
/// the marker) or a length-prefixed `StrL1`/`StrL2`/`StrL4`.
///
/// Shared by the [`Value`]-based [`encode`] path and the native serde
/// serializer.
pub(super) fn encode_string(s: &str, out: &mut Vec<u8>) {
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
///
/// Shared by the [`Value`]-based [`encode`] path and the native serde
/// serializer, which buffers each container's body in a scratch `Vec` and then
/// calls this to frame it.
pub(super) fn encode_container(lc_markers: [u8; 3], count: usize, body: &[u8], out: &mut Vec<u8>) {
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
        // Cosmos caps request bodies far below u32::MAX, so the widest LC4
        // markers always suffice; guard against silently truncating a larger
        // in-memory value into an invalid buffer.
        debug_assert!(
            len <= u32::MAX as usize && count <= u32::MAX as usize,
            "container length/count exceeds u32::MAX"
        );
        out.push(lc4);
        out.extend_from_slice(&(len as u32).to_le_bytes());
        out.extend_from_slice(&(count as u32).to_le_bytes());
    }
    out.extend_from_slice(body);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::markers;
    use serde_json::json;

    /// Builds the expected buffer: preamble, then `head`, then `payload`.
    fn buf(head: &[u8], payload: &[u8]) -> Vec<u8> {
        let mut v = vec![PREAMBLE];
        v.extend_from_slice(head);
        v.extend_from_slice(payload);
        v
    }

    #[test]
    fn encodes_null_and_booleans() {
        assert_eq!(encode(&Value::Null), vec![PREAMBLE, markers::NULL]);
        assert_eq!(encode(&json!(true)), vec![PREAMBLE, markers::TRUE]);
        assert_eq!(encode(&json!(false)), vec![PREAMBLE, markers::FALSE]);
    }

    #[test]
    fn encodes_literal_int_in_marker() {
        // Small non-negative integers are encoded as just the literal-int marker.
        assert_eq!(encode(&json!(0)), vec![PREAMBLE, 0]);
        assert_eq!(encode(&json!(7)), vec![PREAMBLE, 7]);
        assert_eq!(encode(&json!(31)), vec![PREAMBLE, 31]);
    }

    #[test]
    fn encodes_int64_and_uint64() {
        // 32 no longer fits the literal-int range, so it becomes an Int64.
        assert_eq!(
            encode(&json!(32)),
            vec![PREAMBLE, markers::NUMBER_INT64, 32, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            encode(&json!(i64::MAX)),
            vec![
                PREAMBLE,
                markers::NUMBER_INT64,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0x7F
            ],
        );
        // Values beyond i64::MAX use the UInt64 form.
        assert_eq!(
            encode(&json!(u64::MAX)),
            vec![
                PREAMBLE,
                markers::NUMBER_UINT64,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF
            ],
        );
    }

    #[test]
    fn encodes_double() {
        // 1.5 as an IEEE-754 little-endian Double.
        assert_eq!(
            encode(&json!(1.5)),
            vec![
                PREAMBLE,
                markers::NUMBER_DOUBLE,
                0,
                0,
                0,
                0,
                0,
                0,
                0xF8,
                0x3F
            ],
        );
    }

    #[test]
    fn encodes_empty_string_as_encoded_length_marker() {
        // Empty string -> a single encoded-length marker (0x80 | 0).
        assert_eq!(
            encode(&json!("")),
            vec![PREAMBLE, markers::ENCODED_STRING_LENGTH_MIN],
        );
    }

    #[test]
    fn encodes_strings_at_width_boundaries() {
        // 63 bytes: the maximum encoded-length form (0x80 | 63 == 0xBF).
        assert_eq!(encode(&json!("x".repeat(63))), buf(&[0xBF], &[b'x'; 63]),);
        // 64 bytes: first StrL1 (marker 0xC0, 1-byte length 64).
        assert_eq!(
            encode(&json!("y".repeat(64))),
            buf(&[markers::STR_L1, 64], &[b'y'; 64]),
        );
        // 256 bytes: first StrL2 (marker 0xC1, 2-byte little-endian length).
        assert_eq!(
            encode(&json!("p".repeat(256))),
            buf(&[markers::STR_L2, 0x00, 0x01], &[b'p'; 256]),
        );
    }

    #[test]
    fn encodes_empty_array_as_length_count_form() {
        // Empty array -> ArrLC1 with length 0 and count 0.
        assert_eq!(encode(&json!([])), vec![PREAMBLE, markers::ARR_LC1, 0, 0]);
    }

    #[test]
    fn encodes_mixed_array() {
        // [null, true, "x", 3.5] -> ArrLC1, len 13, count 4, then the elements.
        assert_eq!(
            encode(&json!([null, true, "x", 3.5])),
            vec![
                PREAMBLE,
                markers::ARR_LC1,
                0x0D, // payload length
                0x04, // element count
                markers::NULL,
                markers::TRUE,
                0x81,
                b'x', // encoded-length "x"
                markers::NUMBER_DOUBLE,
                0,
                0,
                0,
                0,
                0,
                0,
                0x0C,
                0x40, // 3.5
            ],
        );
    }

    #[test]
    fn encodes_empty_object_as_length_count_form() {
        assert_eq!(encode(&json!({})), vec![PREAMBLE, markers::OBJ_LC1, 0, 0]);
    }

    #[test]
    fn encodes_object_with_nested_array() {
        // { "a": 1, "b": [2, 3] } -> ObjLC1, len 10, count 2, then the members.
        assert_eq!(
            encode(&json!({ "a": 1, "b": [2, 3] })),
            vec![
                PREAMBLE,
                markers::OBJ_LC1,
                0x0A, // payload length
                0x02, // member count
                0x81,
                b'a', // key "a"
                0x01, // value 1 (literal int)
                0x81,
                b'b', // key "b"
                markers::ARR_LC1,
                0x02,
                0x02,
                0x02,
                0x03, // [2, 3]
            ],
        );
    }

    #[test]
    fn encodes_large_array_with_two_byte_container() {
        // 300 single-byte elements push the container past the 1-byte length and
        // count fields, so ArrLC2 (2-byte length + 2-byte count) is used.
        let value = Value::Array((0..300).map(|_| json!(0)).collect());
        let encoded = encode(&value);
        assert_eq!(encoded[0], PREAMBLE);
        assert_eq!(encoded[1], markers::ARR_LC2);
        // 2-byte little-endian length (300) then count (300).
        assert_eq!(&encoded[2..6], &[0x2C, 0x01, 0x2C, 0x01]);
        assert_eq!(encoded.len(), 6 + 300);
    }
}
