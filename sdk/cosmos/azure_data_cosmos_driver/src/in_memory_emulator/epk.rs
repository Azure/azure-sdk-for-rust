// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Effective partition key (EPK) computation.
//!
//! Contains a self-contained implementation of MurmurHash3 (32-bit and 128-bit),
//! partition key component serialization, and EPK hash algorithms (V1, V2, MultiHash).
//!
//! This module is self-contained (not imported from `azure_data_cosmos`) to avoid
//! a cyclic crate dependency.

use crate::models::{PartitionKeyKind, PartitionKeyVersion};
use std::fmt;

/// Effective partition key — uppercase hex-encoded hash string.
///
/// Implements `Ord` via lexicographic comparison of the underlying hex string,
/// which preserves EPK ordering for `BTreeMap`-based range scans.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epk(String);

impl Epk {
    /// Minimum inclusive EPK (empty string).
    pub const MIN: Epk = Epk(String::new());

    /// Maximum exclusive EPK (`"FF"`).
    pub fn max() -> Epk {
        Epk("FF".to_string())
    }

    /// Creates an `Epk` from a raw hex string.
    pub(crate) fn new(s: String) -> Self {
        Epk(s)
    }

    /// Returns the underlying hex string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Epk {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Epk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Epk(\"{}\")", self.0)
    }
}

impl fmt::Display for Epk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A partition key component value used for EPK hash computation.
///
/// This mirrors `InnerPartitionKeyValue` from `azure_data_cosmos` but is
/// self-contained to avoid cyclic dependencies.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PartitionKeyComponent {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    #[allow(dead_code)]
    Undefined,
    Infinity,
}

impl Eq for PartitionKeyComponent {}

// --- Component type markers ---

const UNDEFINED: u8 = 0x00;
const NULL: u8 = 0x01;
const BOOL_FALSE: u8 = 0x02;
const BOOL_TRUE: u8 = 0x03;
const NUMBER: u8 = 0x05;
const STRING: u8 = 0x08;
const INFINITY: u8 = 0xFF;

const MAX_STRING_BYTES_TO_APPEND: usize = 100;

impl PartitionKeyComponent {
    /// Core hashing writer — writes type marker + payload.
    fn write_for_hashing_core(&self, string_suffix: u8, writer: &mut Vec<u8>, truncate: bool) {
        match self {
            PartitionKeyComponent::Bool(true) => writer.push(BOOL_TRUE),
            PartitionKeyComponent::Bool(false) => writer.push(BOOL_FALSE),
            PartitionKeyComponent::Null => writer.push(NULL),
            PartitionKeyComponent::Number(n) => {
                writer.push(NUMBER);
                writer.extend_from_slice(&n.to_le_bytes());
            }
            PartitionKeyComponent::String(s) => {
                writer.push(STRING);
                let bytes = s.as_bytes();
                if truncate && bytes.len() > MAX_STRING_BYTES_TO_APPEND {
                    writer.extend_from_slice(&bytes[..MAX_STRING_BYTES_TO_APPEND]);
                } else {
                    writer.extend_from_slice(bytes);
                }
                writer.push(string_suffix);
            }
            PartitionKeyComponent::Undefined => writer.push(UNDEFINED),
            PartitionKeyComponent::Infinity => writer.push(INFINITY),
        }
    }

    /// V1 hashing (string suffix 0x00, truncate at 100 bytes).
    fn write_for_hashing_v1(&self, writer: &mut Vec<u8>) {
        self.write_for_hashing_core(0x00, writer, true);
    }

    /// V2 hashing (string suffix 0xFF, no truncation).
    fn write_for_hashing_v2(&self, writer: &mut Vec<u8>) {
        self.write_for_hashing_core(0xFF, writer, false);
    }

    /// V1 binary encoding for EPK construction.
    fn write_for_binary_encoding_v1(&self, writer: &mut Vec<u8>) {
        match self {
            PartitionKeyComponent::Bool(true) => writer.push(BOOL_TRUE),
            PartitionKeyComponent::Bool(false) => writer.push(BOOL_FALSE),
            PartitionKeyComponent::Infinity => writer.push(INFINITY),
            PartitionKeyComponent::Number(n) => {
                writer.push(NUMBER);
                let mut payload = encode_double_as_uint64(*n);
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
                    byte_to_write = ((payload >> 56) as u8) | 0x01;
                    payload <<= 7;
                }
                writer.push(byte_to_write & 0xFE);
            }
            PartitionKeyComponent::String(s) => {
                writer.push(STRING);
                let utf8 = s.as_bytes();
                let short = utf8.len() <= MAX_STRING_BYTES_TO_APPEND;
                let write_len = if short {
                    utf8.len()
                } else {
                    std::cmp::min(utf8.len(), MAX_STRING_BYTES_TO_APPEND + 1)
                };
                for item in utf8.iter().take(write_len) {
                    writer.push(item.wrapping_add(1));
                }
                if short {
                    writer.push(0x00);
                }
            }
            PartitionKeyComponent::Undefined => writer.push(UNDEFINED),
            PartitionKeyComponent::Null => writer.push(NULL),
        }
    }
}

fn encode_double_as_uint64(value: f64) -> u64 {
    let raw = u64::from_le_bytes(value.to_le_bytes());
    let mask: u64 = 0x8000_0000_0000_0000;
    if raw < mask {
        raw ^ mask
    } else {
        (!raw).wrapping_add(1)
    }
}

// --- MurmurHash3 implementations ---

/// MurmurHash3 128-bit (x64 variant).
fn murmurhash3_128(data: &[u8], seed: u128) -> u128 {
    let c1: u64 = 0x87c37b91114253d5;
    let c2: u64 = 0x4cf5ad432745937f;

    let mut h1: u64 = seed as u64;
    let mut h2: u64 = (seed >> 64) as u64;

    let len = data.len();
    let mut position = 0usize;

    while position + 16 <= len {
        let k1 = u64::from_le_bytes(data[position..position + 8].try_into().unwrap());
        let k2 = u64::from_le_bytes(data[position + 8..position + 16].try_into().unwrap());

        let mut k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
        h1 = h1.rotate_left(27);
        h1 = h1.wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dce729);

        let mut k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
        h2 = h2.rotate_left(31);
        h2 = h2.wrapping_add(h1);
        h2 = h2.wrapping_mul(5).wrapping_add(0x38495ab5);

        position += 16;
    }

    let mut k1: u64 = 0;
    let mut k2: u64 = 0;
    let n = len & 15;

    if n >= 15 {
        k2 ^= (data[position + 14] as u64) << 48;
    }
    if n >= 14 {
        k2 ^= (data[position + 13] as u64) << 40;
    }
    if n >= 13 {
        k2 ^= (data[position + 12] as u64) << 32;
    }
    if n >= 12 {
        k2 ^= (data[position + 11] as u64) << 24;
    }
    if n >= 11 {
        k2 ^= (data[position + 10] as u64) << 16;
    }
    if n >= 10 {
        k2 ^= (data[position + 9] as u64) << 8;
    }
    if n >= 9 {
        k2 ^= data[position + 8] as u64;
    }
    if k2 != 0 {
        k2 = k2.wrapping_mul(c2);
        k2 = k2.rotate_left(33);
        k2 = k2.wrapping_mul(c1);
        h2 ^= k2;
    }

    if n >= 8 {
        k1 ^= (data[position + 7] as u64) << 56;
    }
    if n >= 7 {
        k1 ^= (data[position + 6] as u64) << 48;
    }
    if n >= 6 {
        k1 ^= (data[position + 5] as u64) << 40;
    }
    if n >= 5 {
        k1 ^= (data[position + 4] as u64) << 32;
    }
    if n >= 4 {
        k1 ^= (data[position + 3] as u64) << 24;
    }
    if n >= 3 {
        k1 ^= (data[position + 2] as u64) << 16;
    }
    if n >= 2 {
        k1 ^= (data[position + 1] as u64) << 8;
    }
    if n >= 1 {
        k1 ^= data[position] as u64;
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(31);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= len as u64;
    h2 ^= len as u64;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    ((h2 as u128) << 64) | (h1 as u128)
}

fn fmix64(mut value: u64) -> u64 {
    value ^= value >> 33;
    value = value.wrapping_mul(0xff51afd7ed558ccd);
    value ^= value >> 33;
    value = value.wrapping_mul(0xc4ceb9fe1a85ec53);
    value ^= value >> 33;
    value
}

/// MurmurHash3 32-bit.
fn murmurhash3_32(data: &[u8], seed: u32) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let length = data.len() as u32;
    let mut h1: u32 = seed;
    let rounded_end = (length & 0xfffffffc) as usize;

    let mut i = 0usize;
    while i < rounded_end {
        let k1 = (data[i] as u32)
            | ((data[i + 1] as u32) << 8)
            | ((data[i + 2] as u32) << 16)
            | ((data[i + 3] as u32) << 24);
        i += 4;

        let mut k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let mut k1_tail: u32 = 0;
    let tail = (length & 0x03) as usize;
    if tail == 3 {
        k1_tail ^= (data[rounded_end + 2] as u32) << 16;
    }
    if tail >= 2 {
        k1_tail ^= (data[rounded_end + 1] as u32) << 8;
    }
    if tail >= 1 {
        k1_tail ^= data[rounded_end] as u32;
        k1_tail = k1_tail.wrapping_mul(c1);
        k1_tail = k1_tail.rotate_left(15);
        k1_tail = k1_tail.wrapping_mul(c2);
        h1 ^= k1_tail;
    }

    h1 ^= length;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;

    h1
}

// --- EPK computation ---

fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    s
}

/// Computes the effective partition key for a set of components using the specified
/// partition key kind and version.
pub(crate) fn compute_epk(
    components: &[PartitionKeyComponent],
    kind: PartitionKeyKind,
    version: PartitionKeyVersion,
) -> Epk {
    if components.is_empty() {
        return Epk::MIN;
    }
    if components.len() == 1 && components[0] == PartitionKeyComponent::Infinity {
        return Epk::max();
    }

    let hex = match kind {
        PartitionKeyKind::Hash => match version {
            PartitionKeyVersion::V1 => compute_v1(components),
            PartitionKeyVersion::V2 => compute_v2(components),
        },
        PartitionKeyKind::MultiHash => compute_multi_hash(components),
        PartitionKeyKind::Range => compute_v2(components), // fallback
    };
    Epk::new(hex)
}

/// V2: concatenate hashing bytes → MurmurHash3-128 → reverse → clear top 2 bits → hex.
fn compute_v2(components: &[PartitionKeyComponent]) -> String {
    let mut buf: Vec<u8> = Vec::new();
    for c in components {
        c.write_for_hashing_v2(&mut buf);
    }
    let hash = murmurhash3_128(&buf, 0);
    let mut hash_bytes = hash.to_le_bytes();
    hash_bytes.reverse();
    hash_bytes[0] &= 0x3F;
    bytes_to_hex_upper(&hash_bytes)
}

/// V1: MurmurHash3-32 → f64 cast → binary encode [hash_number] + truncated components.
fn compute_v1(components: &[PartitionKeyComponent]) -> String {
    let mut hashing_bytes: Vec<u8> = Vec::new();
    for c in components {
        c.write_for_hashing_v1(&mut hashing_bytes);
    }

    let hash32 = murmurhash3_32(&hashing_bytes, 0);
    let hash_f64 = hash32 as f64;

    let hash_component = PartitionKeyComponent::Number(hash_f64);
    let truncated: Vec<PartitionKeyComponent> = components
        .iter()
        .map(|c| match c {
            PartitionKeyComponent::String(s) if s.len() > MAX_STRING_BYTES_TO_APPEND => {
                PartitionKeyComponent::String(s[..MAX_STRING_BYTES_TO_APPEND].to_string())
            }
            other => other.clone(),
        })
        .collect();

    let mut buffer: Vec<u8> = Vec::new();
    hash_component.write_for_binary_encoding_v1(&mut buffer);
    for c in &truncated {
        c.write_for_binary_encoding_v1(&mut buffer);
    }
    bytes_to_hex_upper(&buffer)
}

/// MultiHash: per-component V2 hash → concatenated hex.
fn compute_multi_hash(components: &[PartitionKeyComponent]) -> String {
    let mut result = String::new();
    for c in components {
        let mut buf: Vec<u8> = Vec::new();
        c.write_for_hashing_v2(&mut buf);
        let hash = murmurhash3_128(&buf, 0);
        let mut hash_bytes = hash.to_le_bytes();
        hash_bytes.reverse();
        hash_bytes[0] &= 0x3F;
        result.push_str(&bytes_to_hex_upper(&hash_bytes));
    }
    result
}

/// Parses a JSON-encoded partition key header value (e.g., `["pk1"]`, `[42]`, `[null]`)
/// into a list of `PartitionKeyComponent` values.
pub(crate) fn parse_partition_key_header(header: &str) -> Vec<PartitionKeyComponent> {
    let trimmed = header.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return vec![];
    }

    // Parse as JSON array
    let value: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let arr = match value.as_array() {
        Some(a) => a,
        None => return vec![],
    };

    arr.iter()
        .map(|v| match v {
            serde_json::Value::Null => PartitionKeyComponent::Null,
            serde_json::Value::Bool(b) => PartitionKeyComponent::Bool(*b),
            serde_json::Value::Number(n) => {
                PartitionKeyComponent::Number(n.as_f64().unwrap_or(0.0))
            }
            serde_json::Value::String(s) => PartitionKeyComponent::String(s.clone()),
            _ => PartitionKeyComponent::Null,
        })
        .collect()
}

/// Extracts partition key value(s) from a document body using the container's
/// partition key path definitions.
pub(crate) fn extract_pk_from_body(
    body: &serde_json::Value,
    pk_paths: &[impl AsRef<str>],
) -> Vec<PartitionKeyComponent> {
    pk_paths
        .iter()
        .map(|path| {
            let path_str = path.as_ref().trim_start_matches('/');
            let val = path_str
                .split('/')
                .try_fold(body, |curr, segment| curr.get(segment));
            match val {
                Some(serde_json::Value::Null) | None => PartitionKeyComponent::Null,
                Some(serde_json::Value::Bool(b)) => PartitionKeyComponent::Bool(*b),
                Some(serde_json::Value::Number(n)) => {
                    PartitionKeyComponent::Number(n.as_f64().unwrap_or(0.0))
                }
                Some(serde_json::Value::String(s)) => PartitionKeyComponent::String(s.clone()),
                Some(_) => PartitionKeyComponent::Null,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epk_min_is_empty() {
        assert_eq!(Epk::MIN.as_str(), "");
    }

    #[test]
    fn epk_max_is_ff() {
        assert_eq!(Epk::max().as_str(), "FF");
    }

    #[test]
    fn epk_ordering() {
        let a = Epk::new("0A".to_string());
        let b = Epk::new("0B".to_string());
        let c = Epk::new("FF".to_string());
        assert!(Epk::MIN < a);
        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn v2_single_string() {
        let epk = compute_epk(
            &[PartitionKeyComponent::String("customer42".to_string())],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(epk.as_str(), "19819C94CE42A1654CCC8110539D9589");
    }

    #[test]
    fn v2_known_vectors() {
        let cases: Vec<(PartitionKeyComponent, &str)> = vec![
            (
                PartitionKeyComponent::String(String::new()),
                "32E9366E637A71B4E710384B2F4970A0",
            ),
            (
                PartitionKeyComponent::String("partitionKey".to_string()),
                "013AEFCF77FA271571CF665A58C933F1",
            ),
            (
                PartitionKeyComponent::String("a".repeat(1024)),
                "332BDF5512AE49615F32C7D98C2DB86C",
            ),
            (
                PartitionKeyComponent::Null,
                "378867E4430E67857ACE5C908374FE16",
            ),
            (
                PartitionKeyComponent::Undefined,
                "11622DAA78F835834610ABE56EFF5CB5",
            ),
            (
                PartitionKeyComponent::Bool(true),
                "0E711127C5B5A8E4726AC6DD306A3E59",
            ),
            (
                PartitionKeyComponent::Bool(false),
                "2FE1BE91E90A3439635E0E9E37361EF2",
            ),
            (
                PartitionKeyComponent::Number(-128.0),
                "01DAEDABF913540367FE219B2AD06148",
            ),
            (
                PartitionKeyComponent::Number(127.0),
                "0C507ACAC853ECA7977BF4CEFB562A25",
            ),
            (
                PartitionKeyComponent::Number(5.0),
                "19C08621B135968252FB34B4CF66F811",
            ),
            (
                PartitionKeyComponent::Number(5.123_124_190_509_124),
                "0EF2E2D82460884AF0F6440BE4F726A8",
            ),
            (
                PartitionKeyComponent::String("redmond".to_string()),
                "22E342F38A486A088463DFF7838A5963",
            ),
        ];

        for (component, expected) in cases {
            let epk = compute_epk(
                &[component.clone()],
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V2,
            );
            assert_eq!(epk.as_str(), expected, "V2 mismatch for {:?}", component);
        }
    }

    #[test]
    fn v2_multiple_components() {
        let components = vec![
            PartitionKeyComponent::Number(5.0),
            PartitionKeyComponent::String("redmond".to_string()),
            PartitionKeyComponent::Bool(true),
            PartitionKeyComponent::Null,
        ];
        let epk = compute_epk(&components, PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(epk.as_str(), "3032DECBE2AB1768D8E0AEDEA35881DF");
    }

    #[test]
    fn v1_known_vectors() {
        let cases: Vec<(PartitionKeyComponent, &str)> = vec![
            (
                PartitionKeyComponent::String(String::new()),
                "05C1CF33970FF80800",
            ),
            (
                PartitionKeyComponent::String("partitionKey".to_string()),
                "05C1E1B3D9CD2608716273756A756A706F4C667A00",
            ),
            (PartitionKeyComponent::Null, "05C1ED45D7475601"),
            (PartitionKeyComponent::Undefined, "05C1D529E345DC00"),
            (PartitionKeyComponent::Bool(true), "05C1D7C5A903D803"),
            (PartitionKeyComponent::Bool(false), "05C1DB857D857C02"),
            (
                PartitionKeyComponent::Number(-128.0),
                "05C1D73349F54C053FA0",
            ),
            (
                PartitionKeyComponent::Number(127.0),
                "05C1DD539DDFCC05C05FE0",
            ),
        ];

        for (component, expected) in cases {
            let epk = compute_epk(
                &[component.clone()],
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V1,
            );
            assert_eq!(epk.as_str(), expected, "V1 mismatch for {:?}", component);
        }
    }

    #[test]
    fn empty_pk_returns_min() {
        let epk = compute_epk(&[], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(epk, Epk::MIN);
    }

    #[test]
    fn infinity_returns_max() {
        let epk = compute_epk(
            &[PartitionKeyComponent::Infinity],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(epk, Epk::max());
    }

    #[test]
    fn multi_hash_per_component() {
        // MultiHash should produce concatenated per-component V2 hashes
        let c1 = PartitionKeyComponent::String("tenant1".to_string());
        let c2 = PartitionKeyComponent::String("user1".to_string());

        let epk_c1 = compute_epk(
            &[c1.clone()],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        let epk_c2 = compute_epk(
            &[c2.clone()],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );

        let multi = compute_epk(
            &[c1, c2],
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );

        let expected = format!("{}{}", epk_c1.as_str(), epk_c2.as_str());
        assert_eq!(multi.as_str(), expected);
    }

    #[test]
    fn parse_pk_header_string() {
        let components = parse_partition_key_header(r#"["hello"]"#);
        assert_eq!(
            components,
            vec![PartitionKeyComponent::String("hello".to_string())]
        );
    }

    #[test]
    fn parse_pk_header_number() {
        let components = parse_partition_key_header("[42]");
        assert_eq!(components, vec![PartitionKeyComponent::Number(42.0)]);
    }

    #[test]
    fn parse_pk_header_null() {
        let components = parse_partition_key_header("[null]");
        assert_eq!(components, vec![PartitionKeyComponent::Null]);
    }

    #[test]
    fn parse_pk_header_hierarchical() {
        let components = parse_partition_key_header(r#"["tenant1", "user1"]"#);
        assert_eq!(
            components,
            vec![
                PartitionKeyComponent::String("tenant1".to_string()),
                PartitionKeyComponent::String("user1".to_string()),
            ]
        );
    }

    #[test]
    fn extract_pk_from_json_body() {
        let body = serde_json::json!({"id": "doc1", "pk": "value1", "nested": {"key": 42}});
        let components = extract_pk_from_body(&body, &["/pk"]);
        assert_eq!(
            components,
            vec![PartitionKeyComponent::String("value1".to_string())]
        );
    }

    #[test]
    fn extract_pk_nested() {
        let body = serde_json::json!({"id": "doc1", "nested": {"key": 42}});
        let components = extract_pk_from_body(&body, &["/nested/key"]);
        assert_eq!(components, vec![PartitionKeyComponent::Number(42.0)]);
    }
}
