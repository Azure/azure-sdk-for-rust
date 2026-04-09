// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Effective partition key (EPK) computation.
//!
//! Hashes partition key values into a hex-encoded effective partition key string
//! that can be used to locate the target partition key range.

use crate::models::{
    murmur_hash::{murmurhash3_128, murmurhash3_32},
    partition_key::write_number_v1_binary,
    PartitionKeyDefinition, PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Write;

/// A newtype wrapping the hex-encoded effective partition key string.
///
/// An `EffectivePartitionKey` is the result of hashing a [`PartitionKey`](crate::models::PartitionKey)
/// into a hex string that determines which partition key range owns a given item.
/// Using a newtype ensures callers cannot accidentally pass an arbitrary string
/// where an EPK is expected.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct EffectivePartitionKey(String);

impl EffectivePartitionKey {
    /// Returns the minimum EPK (empty string), representing the start of the EPK space.
    pub fn min() -> Self {
        Self(String::new())
    }

    /// Returns the maximum exclusive EPK ("FF"), representing the upper bound of the EPK space.
    pub fn max() -> Self {
        Self("FF".to_string())
    }

    /// Returns a reference to the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Computes the effective partition key from partition key values.
    ///
    /// This hashes the given values according to the partition key kind and version,
    /// producing the EPK that determines which partition key range owns a given item.
    pub fn compute(
        pk_values: &[PartitionKeyValue],
        kind: PartitionKeyKind,
        version: PartitionKeyVersion,
    ) -> Self {
        if pk_values.is_empty() {
            return Self::min();
        }
        if pk_values.len() == 1 && pk_values[0].is_infinity() {
            return Self::max();
        }

        let hex = match kind {
            PartitionKeyKind::Hash => match version {
                PartitionKeyVersion::V1 => effective_partition_key_hash_v1(pk_values),
                PartitionKeyVersion::V2 => effective_partition_key_hash_v2(pk_values),
            },
            PartitionKeyKind::MultiHash => {
                // MultiHash is only supported with V2. All MultiHash container definitions
                // are created with version=2; V1 MultiHash does not exist in Cosmos DB.
                assert!(
                    version == PartitionKeyVersion::V2,
                    "MultiHash requires V2, got {:?}",
                    version
                );
                effective_partition_key_multi_hash_v2(pk_values)
            }
            // Range partitioning is legacy; fall through to V2 as a reasonable default.
            _ => effective_partition_key_hash_v2(pk_values),
        };
        Self::from(hex)
    }

    /// Computes an EPK range for the given partition key values and definition.
    ///
    /// For full partition keys (component count == definition path count), returns a
    /// point range where start == end. Note: in Rust a `Range` with `start == end` is
    /// technically empty; callers should check `start == end` to detect the point case
    /// rather than iterating the range.
    ///
    /// For prefix partition keys on MultiHash containers (fewer components than the
    /// definition), returns a range `[prefix_epk, prefix_epk + "FF")` covering all
    /// possible completions of the prefix across multiple physical partitions.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `pk_values` is empty.
    /// - `pk_values.len()` exceeds `pk_definition.paths().len()`.
    /// - For non-MultiHash containers, `pk_values.len()` does not equal
    ///   `pk_definition.paths().len()` (prefix keys are only valid for MultiHash).
    pub fn compute_range(
        pk_values: &[PartitionKeyValue],
        pk_definition: &PartitionKeyDefinition,
    ) -> azure_core::Result<std::ops::Range<Self>> {
        if pk_values.is_empty() {
            return Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                "compute_range called with empty pk_values",
            ));
        }
        if pk_values.len() > pk_definition.paths().len() {
            return Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                format!(
                    "more partition key components ({}) than definition paths ({})",
                    pk_values.len(),
                    pk_definition.paths().len()
                ),
            ));
        }

        let kind = pk_definition.kind();
        let version = pk_definition.version();
        let epk = Self::compute(pk_values, kind, version);

        let is_prefix =
            kind == PartitionKeyKind::MultiHash && pk_values.len() < pk_definition.paths().len();

        if kind != PartitionKeyKind::MultiHash && pk_values.len() != pk_definition.paths().len() {
            return Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                format!(
                    "non-MultiHash containers require exactly as many components ({}) as paths ({})",
                    pk_values.len(),
                    pk_definition.paths().len()
                ),
            ));
        }

        if is_prefix {
            // "FF" is safe as an upper-bound sentinel because hash_v2_to_epk masks
            // hash_bytes[0] with 0x3F, so every EPK component's first hex digit
            // is in [0-3]. "FF" is lexicographically greater than any valid suffix.
            let max_str = format!("{}FF", epk.as_str());
            let max = Self::from(max_str);
            Ok(epk..max)
        } else {
            Ok(epk.clone()..epk)
        }
    }
}

impl fmt::Display for EffectivePartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<str> for EffectivePartitionKey {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for EffectivePartitionKey {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl From<String> for EffectivePartitionKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for EffectivePartitionKey {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for EffectivePartitionKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Length-aware ordering for effective partition keys.
///
/// For hierarchical partition key (HPK) containers, the backend may return
/// partition key ranges with mixed-length boundaries: a partial EPK (32 chars,
/// one hash component) and a fully specified EPK (64 chars, two hash components
/// zero-padded). For example:
///
///   - Partial:   `06AB34CFE4E482236BCACBBF50E234AB`
///   - Full:      `06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000`
///
/// These represent the same partition boundary. Plain lexicographic comparison
/// treats the shorter string as "less than" the longer one when it is a prefix,
/// causing incorrect overlap detection in routing maps.
///
/// This implementation treats two EPKs as equal when one is a prefix of the other
/// and the remainder consists entirely of `'0'` characters.
///
/// See: <https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5260>
impl Ord for EffectivePartitionKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.0.as_str();
        let b = other.0.as_str();
        let common = a.len().min(b.len());
        match a[..common].cmp(&b[..common]) {
            std::cmp::Ordering::Equal => {
                let tail = if a.len() > b.len() {
                    &a[common..]
                } else {
                    &b[common..]
                };
                if tail.bytes().all(|b| b == b'0') {
                    std::cmp::Ordering::Equal
                } else if a.len() > b.len() {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            other => other,
        }
    }
}

impl PartialOrd for EffectivePartitionKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// V2: MurmurHash3-128, then reverse bytes and clear top 2 bits.
fn effective_partition_key_hash_v2(pk_values: &[PartitionKeyValue]) -> String {
    let mut buf: Vec<u8> = Vec::new();
    for v in pk_values {
        v.write_for_hashing_v2(&mut buf);
    }
    hash_v2_to_epk(&buf)
}

/// MultiHash V2: per-component MurmurHash3-128, concatenated.
///
/// Each component is independently encoded, hashed, and hex-encoded.
/// The results are concatenated to produce an EPK of N×32 hex characters
/// where N is the number of partition key components.
fn effective_partition_key_multi_hash_v2(pk_values: &[PartitionKeyValue]) -> String {
    let mut result = String::with_capacity(pk_values.len() * 32);
    let mut buf = Vec::new();
    for v in pk_values {
        buf.clear();
        v.write_for_hashing_v2(&mut buf);
        result.push_str(&hash_v2_to_epk(&buf));
    }
    result
}

/// Shared V2 hash-to-EPK conversion: MurmurHash3-128, reverse bytes, mask top 2 bits, hex-encode.
///
/// Returns a 32-character uppercase hexadecimal string.
fn hash_v2_to_epk(data: &[u8]) -> String {
    let hash_128 = murmurhash3_128(data, 0);
    let mut hash_bytes = hash_128.to_le_bytes();
    hash_bytes.reverse();
    hash_bytes[0] &= 0x3F;
    bytes_to_hex_upper(&hash_bytes)
}

/// V1: MurmurHash3-32, cast to f64, then binary-encode [hash, ...truncated values].
fn effective_partition_key_hash_v1(pk_values: &[PartitionKeyValue]) -> String {
    let mut hashing_bytes: Vec<u8> = Vec::new();
    for v in pk_values {
        v.write_for_hashing_v1(&mut hashing_bytes);
    }

    let hash32 = murmurhash3_32(&hashing_bytes, 0u32);

    // Build the binary-encoded EPK: hash value as Number + truncated original components.
    let mut buffer: Vec<u8> = Vec::new();

    // Write the hash as a Number component using V1 binary encoding.
    // We need to encode f64(hash32) using the V1 number encoding.
    let hash_f64 = hash32 as f64;
    write_number_v1_binary(hash_f64, &mut buffer);

    // Truncate string components to MAX_STRING_BYTES_TO_APPEND before binary encoding,
    // matching the truncation applied during hashing.
    let truncated: Vec<PartitionKeyValue> = pk_values
        .iter()
        .map(|v| v.truncated_for_v1_encoding())
        .collect();

    // Append the truncated original components.
    for v in &truncated {
        v.write_for_binary_encoding_v1(&mut buffer);
    }

    bytes_to_hex_upper(&buffer)
}

fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pk_returns_min() {
        let result =
            EffectivePartitionKey::compute(&[], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, EffectivePartitionKey::min());
    }

    #[test]
    fn infinity_pk_returns_max() {
        let inf = PartitionKeyValue::infinity();
        let result =
            EffectivePartitionKey::compute(&[inf], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, EffectivePartitionKey::max());
    }

    /// V2 test cases ported from Java SDK tests via the Rust SDK's hash.rs.
    #[test]
    fn effective_partition_key_hash_v2() {
        let thousand_a = "a".repeat(1024);

        let cases: Vec<(PartitionKeyValue, &str)> = vec![
            (
                PartitionKeyValue::from("".to_string()),
                "32E9366E637A71B4E710384B2F4970A0",
            ),
            (
                PartitionKeyValue::from("partitionKey".to_string()),
                "013AEFCF77FA271571CF665A58C933F1",
            ),
            (
                PartitionKeyValue::from(thousand_a),
                "332BDF5512AE49615F32C7D98C2DB86C",
            ),
            (
                PartitionKeyValue::from(None::<String>),
                "378867E4430E67857ACE5C908374FE16",
            ),
            (
                PartitionKeyValue::from(true),
                "0E711127C5B5A8E4726AC6DD306A3E59",
            ),
            (
                PartitionKeyValue::from(false),
                "2FE1BE91E90A3439635E0E9E37361EF2",
            ),
            (
                PartitionKeyValue::from(-128f64),
                "01DAEDABF913540367FE219B2AD06148",
            ),
            (
                PartitionKeyValue::from(127f64),
                "0C507ACAC853ECA7977BF4CEFB562A25",
            ),
            (
                PartitionKeyValue::from(i64::MIN as f64),
                "23D5C6395512BDFEAFADAD15328AD2BB",
            ),
            (
                PartitionKeyValue::from(i64::MAX as f64),
                "2EDB959178DFCCA18983F89384D1629B",
            ),
            (
                PartitionKeyValue::from(i32::MIN as f64),
                "0B1660D5233C3171725B30D4A5F4CC1F",
            ),
            (
                PartitionKeyValue::from(i32::MAX as f64),
                "2D9349D64712AEB5EB1406E2F0BE2725",
            ),
            (
                PartitionKeyValue::from(f64::from_bits(0x1)),
                "0E6CBA63A280927DE485DEF865800139",
            ),
            (
                PartitionKeyValue::from(f64::MAX),
                "31424D996457102634591FF245DBCC4D",
            ),
            (
                PartitionKeyValue::from(5.0f64),
                "19C08621B135968252FB34B4CF66F811",
            ),
            (
                PartitionKeyValue::from(5.123_124_190_509_124f64),
                "0EF2E2D82460884AF0F6440BE4F726A8",
            ),
            (
                PartitionKeyValue::from("redmond".to_string()),
                "22E342F38A486A088463DFF7838A5963",
            ),
        ];

        for (component, expected) in &cases {
            let actual = EffectivePartitionKey::compute(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V2,
            );
            assert_eq!(
                actual.as_str(),
                *expected,
                "V2 mismatch for {:?}",
                component
            );
        }
    }

    /// V2 multi-component partition key test.
    #[test]
    fn effective_partition_key_hash_v2_multiple_keys() {
        let components = vec![
            PartitionKeyValue::from(5.0f64),
            PartitionKeyValue::from("redmond".to_string()),
            PartitionKeyValue::from(true),
            PartitionKeyValue::from(None::<String>),
        ];
        let expected = "3032DECBE2AB1768D8E0AEDEA35881DF";

        let actual = EffectivePartitionKey::compute(
            &components,
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(actual.as_str(), expected);
    }

    /// V1 test cases ported from Java SDK tests via the Rust SDK's hash.rs.
    #[test]
    fn effective_partition_key_hash_v1() {
        let thousand_a = "a".repeat(1024);

        let cases: Vec<(PartitionKeyValue, &str)> = vec![
            (
                PartitionKeyValue::from("".to_string()),
                "05C1CF33970FF80800",
            ),
            (
                PartitionKeyValue::from("partitionKey".to_string()),
                "05C1E1B3D9CD2608716273756A756A706F4C667A00",
            ),
            (
                PartitionKeyValue::from(thousand_a),
                "05C1EB5921F706086262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626200",
            ),
            (
                PartitionKeyValue::from(None::<String>),
                "05C1ED45D7475601",
            ),
            (
                PartitionKeyValue::from(true),
                "05C1D7C5A903D803",
            ),
            (
                PartitionKeyValue::from(false),
                "05C1DB857D857C02",
            ),
            (
                PartitionKeyValue::from(-128f64),
                "05C1D73349F54C053FA0",
            ),
            (
                PartitionKeyValue::from(127f64),
                "05C1DD539DDFCC05C05FE0",
            ),
            (
                PartitionKeyValue::from(i64::MIN as f64),
                "05C1DB35F33D1C053C20",
            ),
            (
                PartitionKeyValue::from(i64::MAX as f64),
                "05C1B799AB2DD005C3E0",
            ),
            (
                PartitionKeyValue::from(i32::MIN as f64),
                "05C1DFBF252BCC053E20",
            ),
            (
                PartitionKeyValue::from(i32::MAX as f64),
                "05C1E1F503DFB205C1DFFFFFFFFC",
            ),
            (
                PartitionKeyValue::from(f64::from_bits(0x1)),
                "05C1E5C91F4D3005800101010101010102",
            ),
            (
                PartitionKeyValue::from(f64::MAX),
                "05C1CBE367C53005FFEFFFFFFFFFFFFFFE",
            ),
        ];

        for (component, expected) in &cases {
            let actual = EffectivePartitionKey::compute(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V1,
            );
            assert_eq!(
                actual.as_str(),
                *expected,
                "V1 mismatch for {:?}",
                component
            );
        }
    }

    /// A single-component MultiHash EPK should produce the same 32-char hex
    /// as V2 single-hash, since both hash one component identically.
    #[test]
    fn multi_hash_single_component_matches_v2() {
        let pk = vec![PartitionKeyValue::from("redmond".to_string())];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        let single =
            EffectivePartitionKey::compute(&pk, PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(multi.as_str(), single.as_str());
    }

    /// Two-component MultiHash EPK should be 64 hex chars (2 × 32).
    /// Each component is hashed independently, so the result is NOT the same
    /// as a single V2 hash of both components concatenated.
    #[test]
    fn multi_hash_two_components() {
        let pk = vec![
            PartitionKeyValue::from("redmond".to_string()),
            PartitionKeyValue::from(5.0f64),
        ];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(multi.as_str().len(), 64);

        // Expected values from the effective_partition_key_hash_v2 test cases above,
        // verified against cross-SDK baselines (.NET, Go, Java).
        // First 32 chars should match the single-component hash of "redmond"
        assert_eq!(&multi.as_str()[..32], "22E342F38A486A088463DFF7838A5963");
        // Second 32 chars should match the single-component hash of 5.0
        assert_eq!(&multi.as_str()[32..], "19C08621B135968252FB34B4CF66F811");
    }

    /// Three-component MultiHash EPK should be 96 hex chars (3 × 32).
    #[test]
    fn multi_hash_three_components() {
        let pk = vec![
            PartitionKeyValue::from("redmond".to_string()),
            PartitionKeyValue::from(true),
            PartitionKeyValue::from(None::<String>),
        ];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(multi.as_str().len(), 96);

        // Expected values from the effective_partition_key_hash_v2 test cases above,
        // verified against cross-SDK baselines (.NET, Go, Java).
        assert_eq!(&multi.as_str()[..32], "22E342F38A486A088463DFF7838A5963");
        assert_eq!(&multi.as_str()[32..64], "0E711127C5B5A8E4726AC6DD306A3E59");
        assert_eq!(&multi.as_str()[64..], "378867E4430E67857ACE5C908374FE16");
    }

    /// MultiHash with an Undefined component (used for partial HPK).
    #[test]
    fn multi_hash_with_undefined() {
        let pk = vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::undefined(),
        ];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(multi.as_str().len(), 64);

        // First segment: hash of "tenant1"
        let single_tenant = EffectivePartitionKey::compute(
            &[PartitionKeyValue::from("tenant1".to_string())],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(&multi.as_str()[..32], single_tenant.as_str());

        // Second segment: hash of Undefined (0x00 byte)
        let single_undef = EffectivePartitionKey::compute(
            &[PartitionKeyValue::undefined()],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(&multi.as_str()[32..], single_undef.as_str());
    }

    /// MultiHash should NOT produce the same result as single-hash V2 for
    /// multi-component keys, since single-hash concatenates then hashes once.
    #[test]
    fn multi_hash_differs_from_single_hash() {
        let pk = vec![
            PartitionKeyValue::from(5.0f64),
            PartitionKeyValue::from("redmond".to_string()),
            PartitionKeyValue::from(true),
            PartitionKeyValue::from(None::<String>),
        ];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        let single =
            EffectivePartitionKey::compute(&pk, PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        // Single hash produces 32 chars, multi hash produces 128 chars (4 × 32)
        assert_eq!(single.as_str().len(), 32);
        assert_eq!(multi.as_str().len(), 128);
        assert_ne!(multi.as_str(), single.as_str());
    }

    /// compute_range returns a point range for a full MultiHash key.
    #[test]
    fn compute_range_full_key_returns_point() {
        let pk = vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::from("user1".to_string()),
            PartitionKeyValue::from("session1".to_string()),
        ];
        let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let range = EffectivePartitionKey::compute_range(&pk, &pk_def).unwrap();

        assert_eq!(
            range.start, range.end,
            "Full key should produce a point range"
        );
        assert_eq!(range.start.as_str().len(), 96);
    }

    /// compute_range returns an EPK range for a prefix (2 of 3 components).
    #[test]
    fn compute_range_prefix_two_of_three() {
        let pk = vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::from("user1".to_string()),
        ];
        let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let range = EffectivePartitionKey::compute_range(&pk, &pk_def).unwrap();

        assert_ne!(range.start, range.end, "Prefix key should produce a range");
        // min EPK = hash(tenant1) + hash(user1) = 64 chars
        assert_eq!(range.start.as_str().len(), 64);
        // max EPK = min + "FF" = 66 chars
        assert_eq!(range.end.as_str().len(), 66);
        assert!(range.end.as_str().ends_with("FF"));
        assert!(range.end.as_str().starts_with(range.start.as_str()));
    }

    /// compute_range returns an EPK range for a prefix (1 of 3 components).
    #[test]
    fn compute_range_prefix_one_of_three() {
        let pk = vec![PartitionKeyValue::from("tenant1".to_string())];
        let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let range = EffectivePartitionKey::compute_range(&pk, &pk_def).unwrap();

        assert_ne!(range.start, range.end);
        assert_eq!(range.start.as_str().len(), 32);
        assert_eq!(range.end.as_str().len(), 34);
        assert!(range.end.as_str().ends_with("FF"));
    }

    /// compute_range returns a point for single-hash (non-MultiHash) keys
    /// when the component count matches the definition path count.
    #[test]
    fn compute_range_single_hash_always_point() {
        let pk = vec![PartitionKeyValue::from("tenant1".to_string())];
        let pk_def = PartitionKeyDefinition::from("/tenantId");
        let range = EffectivePartitionKey::compute_range(&pk, &pk_def).unwrap();

        assert_eq!(
            range.start, range.end,
            "Single-hash should always be a point"
        );
    }
}
///
/// These tests operate at two levels:
///
/// 1. **Full production pipeline** ([`EffectivePartitionKey::compute`]):
///    For values representable as [`PartitionKeyValue`] (no edge-cases like
///    NaN, ±Infinity, −0.0), the test calls `EffectivePartitionKey::compute`
///    to exercise the complete PK → EPK pipeline.  For single-hash keys this
///    covers encoding, hashing, V2 masking, and V1 binary encoding.  For
///    multi-hash (hierarchical PK) keys, it exercises per-component V2
///    hashing with per-component masking (V1 MultiHash does not exist).
///    V2 results are compared against the Go baseline hash with top-2-bit
///    masking applied.  V1 results cannot be directly compared to Go's V1
///    hash format (which is a zero-padded 32-bit hash, not the full V1 EPK),
///    so the V1 pipeline is exercised for correctness and separately checked
///    by the `effective_partition_key_hash_v1` unit test.
///
/// 2. **Raw MurmurHash baseline** (encoding + hash, no masking/truncation):
///    Verifies that the byte encoding of each value type and the MurmurHash
///    implementation match the cross-SDK baselines.  These use the canonical
///    byte encoding (without V1 100-byte string truncation or V2 top-2-bit
///    masking) so that the raw hash outputs are comparable across SDKs.
///    Edge-case values that [`PartitionKeyValue`] cannot represent (Undefined,
///    NaN, ±Infinity, −0.0) are encoded directly at this level only.
///
/// See: <https://github.com/Azure/azure-sdk-for-go/blob/main/sdk/data/azcosmos/internal/epk/epk_test.go>
#[cfg(test)]
mod baseline_tests {
    use crate::models::murmur_hash::{murmurhash3_128, murmurhash3_32};
    use crate::models::{PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion};
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::fmt::Write;

    use super::EffectivePartitionKey;

    // Embed XML test data within the test module so it's absent from product binaries.
    const SINGLETONS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Singletons.xml");
    const NUMBERS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Numbers.xml");
    const STRINGS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Strings.xml");
    const LISTS_XML: &str = include_str!("../../testdata/PartitionKeyHashBaselineTest.Lists.xml");

    // -- Parsed value: delegates to real PartitionKeyValue where possible --

    /// A parsed baseline test value.
    ///
    /// Normal values are stored as real `PartitionKeyValue` and use the production
    /// encoding path.  Edge-case values that `PartitionKeyValue` cannot represent
    /// (Undefined, NaN, ±Infinity, -0.0) are handled with minimal inline encoding.
    enum ParsedValue {
        /// A value representable as a real `PartitionKeyValue`.
        Value(PartitionKeyValue),
        /// The Undefined sentinel (byte `0x00`).  Not a valid partition key value
        /// in the production pipeline.
        Undefined,
        /// A number that `PartitionKeyValue` would normalize or reject:
        /// NaN, ±Infinity, and -0.0.  Encoded as raw float bytes.
        RawNumber(f64),
    }

    /// Parse the XML `PartitionKeyValue` field into test values, matching Go's `parseValues`.
    fn parse_values(raw: &str) -> Vec<ParsedValue> {
        if raw == "UNDEFINED" {
            return vec![ParsedValue::Undefined];
        }
        if raw.starts_with('[') && raw.ends_with(']') {
            let inner = &raw[1..raw.len() - 1];
            return inner
                .split(',')
                .map(|s| parse_single_json_value(s.trim()))
                .collect();
        }
        vec![parse_single_json_value(raw)]
    }

    fn parse_single_json_value(raw: &str) -> ParsedValue {
        match raw {
            "null" => ParsedValue::Value(PartitionKeyValue::from(None::<String>)),
            "true" => ParsedValue::Value(PartitionKeyValue::from(true)),
            "false" => ParsedValue::Value(PartitionKeyValue::from(false)),
            s if s.starts_with('"') && s.ends_with('"') => {
                let inner = &s[1..s.len() - 1];
                match inner {
                    // Non-finite floats: PartitionKeyValue requires FiniteF64,
                    // so encode them directly.  Use .NET's NaN bit pattern so the
                    // hash matches the baseline.
                    "NaN" => ParsedValue::RawNumber(f64::from_bits(0xFFF8000000000000)),
                    "-Infinity" => ParsedValue::RawNumber(f64::NEG_INFINITY),
                    "Infinity" => ParsedValue::RawNumber(f64::INFINITY),
                    _ => ParsedValue::Value(PartitionKeyValue::from(inner.to_string())),
                }
            }
            _ => {
                let n: f64 = raw
                    .parse()
                    .unwrap_or_else(|e| panic!("failed to parse number '{raw}': {e}"));
                // FiniteF64 normalizes -0.0 to +0.0 which changes the hash,
                // so route negative zero through RawNumber.
                if n == 0.0 && n.is_sign_negative() {
                    ParsedValue::RawNumber(n)
                } else {
                    ParsedValue::Value(PartitionKeyValue::from(n))
                }
            }
        }
    }

    // -- Encoding: delegates to real PartitionKeyValue methods --

    /// Encode a value for the V1 raw hash baseline (canonical byte encoding).
    ///
    /// For [`ParsedValue::Value`], this delegates to the production
    /// [`PartitionKeyValue::write_for_hashing_v2`] and fixes up the string
    /// suffix byte from `0xFF` to `0x00`.  We cannot use the production
    /// [`PartitionKeyValue::write_for_hashing_v1`] here because it truncates
    /// strings at 100 bytes, whereas the cross-SDK baselines use the full
    /// canonical encoding (no truncation) so that the raw hash outputs are
    /// comparable.  Non-string types produce identical bytes for V1 and V2.
    ///
    /// The production V1 pipeline (with truncation) is tested separately via
    /// [`EffectivePartitionKey::compute`] in `run_baseline`.
    fn encode_v1(pv: &ParsedValue, buf: &mut Vec<u8>) {
        match pv {
            ParsedValue::Value(v) => {
                let start = buf.len();
                v.write_for_hashing_v2(buf);
                // String encoding: [0x08] [bytes…] [0xFF].
                // V1 baseline uses suffix 0x00 instead of 0xFF.
                if buf.len() > start + 1 && buf[start] == 0x08 {
                    *buf.last_mut().unwrap() = 0x00;
                }
            }
            ParsedValue::Undefined => buf.push(0x00),
            ParsedValue::RawNumber(f) => {
                buf.push(0x05); // NUMBER marker
                buf.extend_from_slice(&f.to_le_bytes());
            }
        }
    }

    /// Encode a value for the V2 raw hash baseline.
    ///
    /// Normal values use the production [`PartitionKeyValue::write_for_hashing_v2`]
    /// method; edge-cases are encoded inline.
    fn encode_v2(pv: &ParsedValue, buf: &mut Vec<u8>) {
        match pv {
            ParsedValue::Value(v) => v.write_for_hashing_v2(buf),
            ParsedValue::Undefined => buf.push(0x00),
            ParsedValue::RawNumber(f) => {
                buf.push(0x05); // NUMBER marker
                buf.extend_from_slice(&f.to_le_bytes());
            }
        }
    }

    // -- XML parsing using quick_xml --

    struct BaselineResult {
        description: String,
        partition_key_value: String,
        v1_hash: String,
        v2_hash: String,
    }

    fn parse_baseline_xml(xml: &str) -> Vec<BaselineResult> {
        let mut reader = Reader::from_str(xml);
        let mut results = Vec::new();
        let mut current_tag = String::new();
        let mut desc = String::new();
        let mut pk_val = String::new();
        let mut v1 = String::new();
        let mut v2 = String::new();
        let mut in_result = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let qname = e.name();
                    let name = std::str::from_utf8(qname.as_ref()).unwrap();
                    match name {
                        "Result" => {
                            in_result = true;
                            desc.clear();
                            pk_val.clear();
                            v1.clear();
                            v2.clear();
                        }
                        _ if in_result => {
                            current_tag = name.to_string();
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(ref e)) if in_result => {
                    let text = String::from_utf8(e.to_vec()).unwrap();
                    match current_tag.as_str() {
                        "Description" => desc.push_str(&text),
                        "PartitionKeyValue" => pk_val.push_str(&text),
                        "PartitionKeyHashV1" => v1.push_str(&text),
                        "PartitionKeyHashV2" => v2.push_str(&text),
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let qname = e.name();
                    let name = std::str::from_utf8(qname.as_ref()).unwrap();
                    if name == "Result" {
                        in_result = false;
                        results.push(BaselineResult {
                            description: desc.clone(),
                            partition_key_value: pk_val.clone(),
                            v1_hash: v1.clone(),
                            v2_hash: v2.clone(),
                        });
                    }
                    current_tag.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!(
                    "XML parse error at position {}: {e}",
                    reader.error_position()
                ),
                _ => {}
            }
        }

        results
    }

    // -- Hash computation --

    /// Compute V1 hash matching Go's `ComputeV1`:
    /// per-component MurmurHash3-32, formatted as 24 zero hex chars + 8 hex hash chars.
    fn compute_v1_baseline(values: &[ParsedValue]) -> String {
        let mut result = String::new();
        for v in values {
            let mut buf = Vec::new();
            encode_v1(v, &mut buf);
            let hash = murmurhash3_32(&buf, 0);
            write!(&mut result, "000000000000000000000000{hash:08X}").unwrap();
        }
        result
    }

    /// Compute V2 single-hash matching Go's `ComputeV2Hash`:
    /// all components concatenated, MurmurHash3-128, reversed bytes, uppercase hex.
    fn compute_v2_hash_baseline(values: &[ParsedValue]) -> String {
        let mut buf = Vec::new();
        for v in values {
            encode_v2(v, &mut buf);
        }
        hash128_to_epk(&buf)
    }

    /// Compute V2 multi-hash matching Go's `ComputeV2MultiHash`:
    /// per-component MurmurHash3-128, concatenated.
    fn compute_v2_multi_hash_baseline(values: &[ParsedValue]) -> String {
        let mut result = String::new();
        for v in values {
            let mut buf = Vec::new();
            encode_v2(v, &mut buf);
            result.push_str(&hash128_to_epk(&buf));
        }
        result
    }

    fn hash128_to_epk(data: &[u8]) -> String {
        let h128 = murmurhash3_128(data, 0);
        let mut bytes = h128.to_le_bytes();
        bytes.reverse();
        bytes_to_hex_upper(&bytes)
    }

    fn bytes_to_hex_upper(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for b in bytes {
            write!(&mut s, "{b:02X}").unwrap();
        }
        s
    }

    // -- Test runner --

    /// Derives the expected V2 EPK from a Go baseline V2 hash.
    ///
    /// The Go baseline V2 hash is the raw reversed MurmurHash3-128.  The Rust
    /// EPK pipeline clears the top two bits: `byte[0] &= 0x3F`.
    fn apply_v2_masking(raw_v2_hash: &str) -> String {
        let first_byte = u8::from_str_radix(&raw_v2_hash[..2], 16).unwrap();
        let masked = first_byte & 0x3F;
        format!("{masked:02X}{}", &raw_v2_hash[2..])
    }

    /// Applies V2 masking to each 32-char component of a multi-hash EPK.
    fn apply_v2_masking_per_component(raw_v2_hash: &str) -> String {
        let mut result = String::with_capacity(raw_v2_hash.len());
        for chunk in raw_v2_hash.as_bytes().chunks(32) {
            result.push_str(&apply_v2_masking(std::str::from_utf8(chunk).unwrap()));
        }
        result
    }

    fn run_baseline(xml: &str, multi_hash: bool) {
        let cases = parse_baseline_xml(xml);
        assert!(!cases.is_empty(), "no test cases parsed from XML");

        for tc in &cases {
            let values = parse_values(&tc.partition_key_value);

            // --- Full production pipeline (EffectivePartitionKey::compute) ---
            //
            // For values representable as PartitionKeyValue (no edge-cases like
            // NaN, ±Infinity, -0.0), run the complete PK → EPK pipeline and
            // compare against the Go baseline with V2 masking applied.
            if values.iter().all(|v| matches!(v, ParsedValue::Value(_))) {
                let pk_values: Vec<PartitionKeyValue> = values
                    .iter()
                    .map(|v| match v {
                        ParsedValue::Value(v) => v.clone(),
                        _ => unreachable!(),
                    })
                    .collect();

                if multi_hash {
                    // MultiHash V2: per-component hashing, each component masked independently.
                    let v2_epk = EffectivePartitionKey::compute(
                        &pk_values,
                        PartitionKeyKind::MultiHash,
                        PartitionKeyVersion::V2,
                    );
                    let expected_v2 = apply_v2_masking_per_component(&tc.v2_hash);
                    assert_eq!(
                        v2_epk.as_str(),
                        expected_v2,
                        "V2 MultiHash full pipeline mismatch for {} (value: {})",
                        tc.description,
                        tc.partition_key_value,
                    );
                    // V1 MultiHash does not exist in Cosmos DB; skip V1 pipeline.
                } else {
                    // Single-hash V2: one hash of all components, masked once.
                    let v2_epk = EffectivePartitionKey::compute(
                        &pk_values,
                        PartitionKeyKind::Hash,
                        PartitionKeyVersion::V2,
                    );
                    let expected_v2 = apply_v2_masking(&tc.v2_hash);
                    assert_eq!(
                        v2_epk.as_str(),
                        expected_v2,
                        "V2 full pipeline mismatch for {} (value: {})",
                        tc.description,
                        tc.partition_key_value,
                    );

                    // V1: Exercise the production V1 pipeline (with 100-byte string
                    // truncation and binary EPK encoding).  The V1 EPK format
                    // differs from Go's V1 hash format, so we verify the pipeline
                    // completes and produces a non-empty hex string.
                    let v1_epk = EffectivePartitionKey::compute(
                        &pk_values,
                        PartitionKeyKind::Hash,
                        PartitionKeyVersion::V1,
                    );
                    assert!(
                        !v1_epk.as_str().is_empty(),
                        "V1 full pipeline produced empty EPK for {} (value: {})",
                        tc.description,
                        tc.partition_key_value,
                    );
                }
            }

            // --- Cross-SDK raw hash baseline ---
            //
            // Verifies byte encoding + MurmurHash correctness against the same
            // baselines used by Go, .NET, Java, and Python SDKs.  Uses canonical
            // encoding (no V1 truncation, no V2 masking) so raw hashes match.
            let actual_v1 = compute_v1_baseline(&values);
            assert_eq!(
                actual_v1, tc.v1_hash,
                "V1 hash mismatch for {} (value: {})",
                tc.description, tc.partition_key_value,
            );

            let actual_v2 = if multi_hash {
                compute_v2_multi_hash_baseline(&values)
            } else {
                compute_v2_hash_baseline(&values)
            };
            assert_eq!(
                actual_v2, tc.v2_hash,
                "V2 hash mismatch for {} (value: {})",
                tc.description, tc.partition_key_value,
            );
        }
    }

    #[test]
    fn baseline_singletons() {
        run_baseline(SINGLETONS_XML, false);
    }

    #[test]
    fn baseline_numbers() {
        run_baseline(NUMBERS_XML, false);
    }

    #[test]
    fn baseline_strings() {
        run_baseline(STRINGS_XML, false);
    }

    #[test]
    fn baseline_lists() {
        run_baseline(LISTS_XML, true);
    }
}
