// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::models::PartitionKeyKind;
use crate::murmur_hash::{murmurhash3_128, murmurhash3_32};
use std::fmt::Write;

const MAX_STRING_BYTES_TO_APPEND: usize = 100;
const MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "";
const MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "FF";

/// A strongly-typed wrapper around the hex-encoded effective partition key string.
///
/// Use [`AsRef<str>`] to obtain the underlying string when passing to APIs
/// that accept `&str`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectivePartitionKey(String);

impl EffectivePartitionKey {
    /// Returns the underlying string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for EffectivePartitionKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Contains all allowed markers for component marker types.
mod component {
    pub const UNDEFINED: u8 = 0x00;
    pub const NULL: u8 = 0x01;
    pub const BOOL_FALSE: u8 = 0x02;
    pub const BOOL_TRUE: u8 = 0x03;
    pub const NUMBER: u8 = 0x05;
    pub const STRING: u8 = 0x08;
    pub const INFINITY: u8 = 0xFF;
}

#[derive(Clone, Debug, PartialEq)]
pub enum InnerPartitionKeyValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Infinity,
    Undefined,
}

// `f64` does not implement `Eq`, but in this domain partition key numbers are
// always finite, non-NaN values, so total equality holds. We implement `Eq`
// manually to express this invariant.
impl Eq for InnerPartitionKeyValue {}

impl InnerPartitionKeyValue {
    /// Common hashing writer core: writes type marker + payload (string suffix used by V2).
    fn write_for_hashing_core(&self, string_suffix: u8, writer: &mut Vec<u8>, truncate: bool) {
        match self {
            InnerPartitionKeyValue::Bool(true) => writer.push(component::BOOL_TRUE),
            InnerPartitionKeyValue::Bool(false) => writer.push(component::BOOL_FALSE),
            InnerPartitionKeyValue::Null => writer.push(component::NULL),
            InnerPartitionKeyValue::Number(n) => {
                writer.push(component::NUMBER); // Number marker
                let bytes = n.to_le_bytes();
                writer.extend_from_slice(&bytes);
            }
            InnerPartitionKeyValue::String(s) => {
                writer.push(component::STRING); // String marker
                let bytes = s.as_bytes();
                if truncate && bytes.len() > MAX_STRING_BYTES_TO_APPEND {
                    writer.extend_from_slice(&bytes[..MAX_STRING_BYTES_TO_APPEND]);
                } else {
                    writer.extend_from_slice(bytes);
                }
                writer.push(string_suffix);
            }
            InnerPartitionKeyValue::Undefined => writer.push(component::UNDEFINED),
            InnerPartitionKeyValue::Infinity => writer.push(component::INFINITY),
        }
    }

    /// V1 hashing wrapper (string suffix 0x00)
    pub fn write_for_hashing_v1(&self, writer: &mut Vec<u8>) {
        self.write_for_hashing_core(0x00u8, writer, true)
    }

    /// V2 hashing wrapper (string suffix 0xFF)
    pub fn write_for_hashing_v2(&self, writer: &mut Vec<u8>) {
        self.write_for_hashing_core(0xFFu8, writer, false)
    }

    /// V1 binary encoding (subset required for test cases):
    /// * Bool -> marker (0x03 true / 0x02 false)
    /// * Number -> marker (0x05) + variable-length 64-bit ordering-preserving encoding
    /// * String -> marker (0x08) + each byte+1 (no 0xFF guard) up to 100 or 101 (if truncated) then 0x00 terminator if short
    /// * Undefined -> marker (0x00)
    /// * Null -> marker (0x01).
    pub fn write_for_binary_encoding_v1(&self, writer: &mut Vec<u8>) {
        match self {
            InnerPartitionKeyValue::Bool(true) => writer.push(component::BOOL_TRUE),
            InnerPartitionKeyValue::Bool(false) => writer.push(component::BOOL_FALSE),
            InnerPartitionKeyValue::Infinity => writer.push(component::INFINITY),
            InnerPartitionKeyValue::Number(n) => {
                writer.push(component::NUMBER);
                let mut payload = encode_double_as_uint64(*n);
                // First 8 bits
                writer.push((payload >> 56) as u8);
                payload <<= 8;
                let mut first = true;
                let mut byte_to_write: u8 = 0;
                while payload != 0 {
                    if !first {
                        writer.push(byte_to_write);
                    } else {
                        first = false;
                    }
                    byte_to_write = ((payload >> 56) as u8) | 0x01; // set continuation bit
                    payload <<= 7; // consume 7 bits (since we used 7 data bits + 1 flag)
                }
                writer.push(byte_to_write & 0xFE); // last byte with 0 flag
            }
            InnerPartitionKeyValue::String(s) => {
                writer.push(component::STRING);
                let utf8 = s.as_bytes();
                let short = utf8.len() <= MAX_STRING_BYTES_TO_APPEND;
                // Use std::cmp to determine truncated write length (include sentinel +1 when longer than max)
                let write_len = if short {
                    utf8.len()
                } else {
                    std::cmp::min(utf8.len(), MAX_STRING_BYTES_TO_APPEND + 1)
                };
                for item in utf8.iter().take(write_len) {
                    let b = item.wrapping_add(1); // unconditional +1
                    writer.push(b);
                }
                if short {
                    writer.push(0x00);
                }
            }
            InnerPartitionKeyValue::Undefined => writer.push(component::UNDEFINED),
            InnerPartitionKeyValue::Null => writer.push(component::NULL),
        }
    }

    /// Binary encoding used by `_to_hex_encoded_binary_string`.
    pub fn write_for_binary_encoding(&self, writer: &mut Vec<u8>) {
        match self {
            InnerPartitionKeyValue::Bool(true) => writer.push(component::BOOL_TRUE),
            InnerPartitionKeyValue::Bool(false) => writer.push(component::BOOL_FALSE),
            InnerPartitionKeyValue::Infinity => writer.push(component::INFINITY),
            InnerPartitionKeyValue::Number(n) => {
                writer.push(component::NUMBER);
                // use IEEE754 little-endian double representation
                writer.extend_from_slice(&n.to_le_bytes());
            }
            InnerPartitionKeyValue::String(s) => {
                writer.push(component::STRING);
                let utf8 = s.as_bytes();
                let size = std::cmp::min(utf8.len(), MAX_STRING_BYTES_TO_APPEND);
                let short_string: bool;
                let write_len = if size == MAX_STRING_BYTES_TO_APPEND {
                    short_string = false;
                    size + 1
                } else {
                    short_string = true;
                    size
                };
                for item in utf8.iter().take(write_len) {
                    let mut b = *item;
                    if b < 0xFF {
                        b = b.wrapping_add(1);
                    }
                    writer.push(b);
                }
                if short_string {
                    writer.push(0x00);
                }
            }
            InnerPartitionKeyValue::Undefined => writer.push(component::UNDEFINED),
            InnerPartitionKeyValue::Null => writer.push(component::NULL),
        }
    }
}

/// Returns an [`EffectivePartitionKey`] representing the hashed partition key.
pub fn get_hashed_partition_key_string(
    pk_value: &[&InnerPartitionKeyValue],
    kind: PartitionKeyKind,
    version: u8,
) -> EffectivePartitionKey {
    if pk_value.is_empty() {
        return EffectivePartitionKey(MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string());
    }
    if pk_value.len() == 1 && *pk_value[0] == InnerPartitionKeyValue::Infinity {
        return EffectivePartitionKey(MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string());
    }

    let raw = if kind.as_str() == PartitionKeyKind::HASH {
        match version {
            1 => get_effective_partition_key_for_hash_partitioning_v1(pk_value),
            2 => get_effective_partition_key_for_hash_partitioning_v2(pk_value),
            _ => {
                tracing::warn!(
                    "Hash partitioning version {} is not supported, falling back to binary encoding.",
                    version
                );
                to_hex_encoded_binary_string(pk_value)
            }
        }
    } else if kind.as_str() == PartitionKeyKind::MULTI_HASH {
        // MultiHash is not yet implemented; use the non-hashed binary encoding
        // as a deterministic fallback instead of panicking.
        tracing::warn!(
            "MultiHash partitioning is not yet supported, falling back to binary encoding."
        );
        to_hex_encoded_binary_string(pk_value)
    } else {
        tracing::warn!(
            "Unknown partition key kind '{}', falling back to binary encoding.",
            kind
        );
        to_hex_encoded_binary_string(pk_value)
    };
    EffectivePartitionKey(raw)
}

/// V2: encode components with `_write_for_hashing_v2`, hash the concatenated bytes,
fn get_effective_partition_key_for_hash_partitioning_v2(
    pk_value: &[&InnerPartitionKeyValue],
) -> String {
    let mut ms: Vec<u8> = Vec::new();
    for comp in pk_value {
        comp.write_for_hashing_v2(&mut ms);
    }
    let hash_128 = murmurhash3_128(&ms, 0);
    let mut hash_bytes = hash_128.to_le_bytes();
    hash_bytes.reverse();
    // Reset 2 most significant bits of first byte
    hash_bytes[0] &= 0x3F;
    bytes_to_hex_upper(&hash_bytes)
}

/// V1: compute 32-bit murmur hash over concatenated component encodings (suffix 0x00 for strings),
/// convert hash (u32) to f64 (possible precision loss is intentional to mirror other sdks), then binary-encode
/// [hash_value_as_number] + truncated original components using V1 binary rules.
fn get_effective_partition_key_for_hash_partitioning_v1(
    pk_value: &[&InnerPartitionKeyValue],
) -> String {
    // Build hashing buffer using V1 hashing encoding (truncation is handled by write_for_hashing_v1)
    let mut hashing_bytes: Vec<u8> = Vec::new();
    for v in pk_value {
        v.write_for_hashing_v1(&mut hashing_bytes);
    }

    let hash32 = murmurhash3_32(&hashing_bytes, 0u32);
    let hash_value_f64 = hash32 as f64; // casts UInt32 -> float (lossy above 2^24)

    // For the binary encoding step, strings must also be truncated to match
    // the truncation applied during hashing.
    let hash_component = InnerPartitionKeyValue::Number(hash_value_f64);
    let truncated_values: Vec<InnerPartitionKeyValue> = pk_value
        .iter()
        .map(|v| match v {
            InnerPartitionKeyValue::String(s) if s.len() > MAX_STRING_BYTES_TO_APPEND => {
                InnerPartitionKeyValue::String(s[..MAX_STRING_BYTES_TO_APPEND].to_string())
            }
            other => (*other).clone(),
        })
        .collect();

    let mut components: Vec<&InnerPartitionKeyValue> =
        Vec::with_capacity(truncated_values.len() + 1);
    components.push(&hash_component);
    components.extend(truncated_values.iter());

    to_hex_encoded_binary_string_v1(&components)
}

/// Encode multiple components into a binary buffer using V1 rules and return uppercase hex string.
fn to_hex_encoded_binary_string_v1(components: &[&InnerPartitionKeyValue]) -> String {
    let mut buffer: Vec<u8> = Vec::new();
    for comp in components {
        comp.write_for_binary_encoding_v1(&mut buffer);
    }
    bytes_to_hex_upper(&buffer)
}

fn encode_double_as_uint64(value: f64) -> u64 {
    let value_in_uint64 = u64::from_le_bytes(value.to_le_bytes());
    let mask: u64 = 0x8000_0000_0000_0000;
    if value_in_uint64 < mask {
        value_in_uint64 ^ mask
    } else {
        (!value_in_uint64).wrapping_add(1)
    }
}

/// Encode multiple components into a binary buffer and return lowercase hex string.
/// This corresponds to `_to_hex_encoded_binary_string` + `_write_for_binary_encoding`.
fn to_hex_encoded_binary_string(components: &[&InnerPartitionKeyValue]) -> String {
    let mut buffer: Vec<u8> = Vec::new();
    for comp in components {
        comp.write_for_binary_encoding(&mut buffer);
    }
    bytes_to_hex_lower(&buffer)
}

fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    s
}

fn bytes_to_hex_lower(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_pk() {
        let result =
            get_hashed_partition_key_string(&[], PartitionKeyKind::new(PartitionKeyKind::HASH), 0);
        assert_eq!(result.as_str(), MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY);
    }

    #[test]
    fn test_infinity_pk() {
        let inf = InnerPartitionKeyValue::Infinity;
        let result = get_hashed_partition_key_string(
            &[&inf],
            PartitionKeyKind::new(PartitionKeyKind::HASH),
            0,
        );
        assert_eq!(result.as_str(), MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY);
    }

    #[test]
    fn test_single_string_hash_v2() {
        let comp = InnerPartitionKeyValue::String("customer42".to_string());
        let result = get_hashed_partition_key_string(
            &[&comp],
            PartitionKeyKind::new(PartitionKeyKind::HASH),
            2,
        );
        // result should be a hex string of length 32 (16 bytes * 2 chars)
        assert_eq!(result.as_str().len(), 32);
        assert_eq!(
            result.as_str(),
            "19819C94CE42A1654CCC8110539D9589",
            "Mismatch for component hash"
        )
    }

    #[test]
    fn test_effective_partition_key_hash_v2() {
        // Each entry represents a single-component partition key and the expected
        // effective partition key hash (uppercase hex) for V2 hash partitioning.
        let thousand_a = "a".repeat(1024);

        // Expected values taken from Java SDK tests.
        let cases: Vec<(InnerPartitionKeyValue, &str)> = vec![
            (
                InnerPartitionKeyValue::String(String::from("")),
                "32E9366E637A71B4E710384B2F4970A0",
            ),
            (
                InnerPartitionKeyValue::String(String::from("partitionKey")),
                "013AEFCF77FA271571CF665A58C933F1",
            ),
            (
                InnerPartitionKeyValue::String(thousand_a),
                "332BDF5512AE49615F32C7D98C2DB86C",
            ),
            (
                InnerPartitionKeyValue::Null,
                "378867E4430E67857ACE5C908374FE16",
            ),
            (
                InnerPartitionKeyValue::Undefined,
                "11622DAA78F835834610ABE56EFF5CB5",
            ),
            (
                InnerPartitionKeyValue::Bool(true),
                "0E711127C5B5A8E4726AC6DD306A3E59",
            ),
            (
                InnerPartitionKeyValue::Bool(false),
                "2FE1BE91E90A3439635E0E9E37361EF2",
            ),
            (
                InnerPartitionKeyValue::Number(-128f64),
                "01DAEDABF913540367FE219B2AD06148",
            ), // Java Byte.MIN_VALUE
            (
                InnerPartitionKeyValue::Number(127f64),
                "0C507ACAC853ECA7977BF4CEFB562A25",
            ), // Java Byte.MAX_VALUE
            (
                InnerPartitionKeyValue::Number(i64::MIN as f64),
                "23D5C6395512BDFEAFADAD15328AD2BB",
            ),
            (
                InnerPartitionKeyValue::Number(i64::MAX as f64),
                "2EDB959178DFCCA18983F89384D1629B",
            ),
            (
                InnerPartitionKeyValue::Number(i32::MIN as f64),
                "0B1660D5233C3171725B30D4A5F4CC1F",
            ),
            (
                InnerPartitionKeyValue::Number(i32::MAX as f64),
                "2D9349D64712AEB5EB1406E2F0BE2725",
            ),
            (
                InnerPartitionKeyValue::Number(f64::from_bits(0x1)),
                "0E6CBA63A280927DE485DEF865800139",
            ), // Java Double.MIN_VALUE
            (
                InnerPartitionKeyValue::Number(f64::MAX),
                "31424D996457102634591FF245DBCC4D",
            ),
            (
                InnerPartitionKeyValue::Number(5.0),
                "19C08621B135968252FB34B4CF66F811",
            ),
            (
                InnerPartitionKeyValue::Number(5.123_124_190_509_124),
                "0EF2E2D82460884AF0F6440BE4F726A8",
            ),
            (
                InnerPartitionKeyValue::String(String::from("redmond")),
                "22E342F38A486A088463DFF7838A5963",
            ),
        ];

        for (component, expected) in &cases {
            let actual = get_hashed_partition_key_string(
                &[component],
                PartitionKeyKind::new(PartitionKeyKind::HASH),
                2,
            );
            assert_eq!(actual.as_str(), *expected, "Mismatch for component hash");
        }
    }

    #[test]
    fn test_effective_partition_key_hash_v2_multiple_keys() {
        let component: Vec<InnerPartitionKeyValue> = vec![
            InnerPartitionKeyValue::Number(5.0),
            InnerPartitionKeyValue::String(String::from("redmond")),
            InnerPartitionKeyValue::Bool(true),
            InnerPartitionKeyValue::Null,
        ];
        let expected = "3032DECBE2AB1768D8E0AEDEA35881DF";

        let refs: Vec<&InnerPartitionKeyValue> = component.iter().collect();
        let actual = get_hashed_partition_key_string(
            &refs,
            PartitionKeyKind::new(PartitionKeyKind::HASH),
            2,
        );
        assert_eq!(actual.as_str(), expected, "Mismatch for component hash");
    }

    #[test]
    fn test_effective_partition_key_hash_v1() {
        // Expected strings are the direct V1 effective partition key representations (uppercase hex).
        let thousand_a = "a".repeat(1024);

        // Expected values taken from Java SDK tests.
        let cases: Vec<(InnerPartitionKeyValue, &str)> = vec![
            (InnerPartitionKeyValue::String(String::from("")), "05C1CF33970FF80800"),
            (InnerPartitionKeyValue::String(String::from("partitionKey")), "05C1E1B3D9CD2608716273756A756A706F4C667A00"),
            (InnerPartitionKeyValue::String(thousand_a), "05C1EB5921F706086262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626200"),
            (InnerPartitionKeyValue::Null, "05C1ED45D7475601"),
            (InnerPartitionKeyValue::Undefined, "05C1D529E345DC00"),
            (InnerPartitionKeyValue::Bool(true), "05C1D7C5A903D803"),
            (InnerPartitionKeyValue::Bool(false), "05C1DB857D857C02"),
            (InnerPartitionKeyValue::Number(-128f64), "05C1D73349F54C053FA0"),
            (InnerPartitionKeyValue::Number(127f64), "05C1DD539DDFCC05C05FE0"),
            (InnerPartitionKeyValue::Number(i64::MIN as f64), "05C1DB35F33D1C053C20"),
            (InnerPartitionKeyValue::Number(i64::MAX as f64), "05C1B799AB2DD005C3E0"),
            (InnerPartitionKeyValue::Number(i32::MIN as f64), "05C1DFBF252BCC053E20"),
            (InnerPartitionKeyValue::Number(i32::MAX as f64), "05C1E1F503DFB205C1DFFFFFFFFC"),
            (InnerPartitionKeyValue::Number(f64::from_bits(0x1)), "05C1E5C91F4D3005800101010101010102"), // Java Double.MIN_VALUE
            (InnerPartitionKeyValue::Number(f64::MAX), "05C1CBE367C53005FFEFFFFFFFFFFFFFFE"),
        ];

        for (component, expected) in &cases {
            let actual = get_hashed_partition_key_string(
                &[component],
                PartitionKeyKind::new(PartitionKeyKind::HASH),
                1,
            );
            assert_eq!(
                actual.as_str(),
                *expected,
                "Mismatch for V1 component hash (enable test after implementation)"
            );
            // unspecified version defaults to V1
            let actual = get_hashed_partition_key_string(
                &[component],
                PartitionKeyKind::new(PartitionKeyKind::HASH),
                1,
            );
            assert_eq!(
                actual.as_str(),
                *expected,
                "Mismatch for V1 component hash (enable test after implementation)"
            );
        }
    }
}
