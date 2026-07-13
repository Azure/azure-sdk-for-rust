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
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

/// A newtype wrapping the raw effective partition key bytes.
///
/// An `EffectivePartitionKey` is the result of hashing a [`PartitionKey`](crate::models::PartitionKey)
/// into the binary EPK that determines which partition key range owns a given
/// item. These are the same bytes sent in the RNTBD `EffectivePartitionKey`
/// token (0x005A); the uppercase-hex encoding ([`to_hex`](Self::to_hex)) is the
/// canonical routing string and `x-ms-*-epk` HTTP header value. Using a newtype
/// ensures callers cannot accidentally pass an arbitrary value where an EPK is
/// expected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectivePartitionKey(Cow<'static, [u8]>);

impl EffectivePartitionKey {
    /// The minimum EPK (empty), representing the start of the EPK space.
    pub const MIN: Self = Self(Cow::Borrowed(&[]));

    /// The maximum exclusive EPK (`0xFF`), representing the upper bound of the EPK space.
    pub const MAX: Self = Self(Cow::Borrowed(&[0xFF]));

    /// Returns the raw EPK bytes (the binary form sent in the RNTBD
    /// `EffectivePartitionKey` token, 0x005A).
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the uppercase-hex encoding of the EPK — the canonical routing
    /// string and `x-ms-start-epk`/`x-ms-end-epk` HTTP header value.
    pub fn to_hex(&self) -> String {
        bytes_to_hex_upper(&self.0)
    }

    /// Constructs an EPK from its raw bytes.
    pub(crate) fn from_bytes(bytes: impl Into<Cow<'static, [u8]>>) -> Self {
        Self(bytes.into())
    }

    /// Computes the effective partition key from partition key values.
    ///
    /// This hashes the given values according to the partition key kind and version,
    /// producing the EPK that determines which partition key range owns a given item.
    pub(crate) fn compute(
        pk_values: &[PartitionKeyValue],
        kind: PartitionKeyKind,
        version: PartitionKeyVersion,
    ) -> Self {
        if pk_values.is_empty() {
            return Self::MIN;
        }
        if pk_values.len() == 1 && pk_values[0].is_infinity() {
            return Self::MAX;
        }

        let bytes = match kind {
            PartitionKeyKind::Hash => match version {
                PartitionKeyVersion::V1 => effective_partition_key_v1_binary(pk_values),
                PartitionKeyVersion::V2 => effective_partition_key_v2_binary(pk_values),
            },
            PartitionKeyKind::MultiHash => {
                // MultiHash is only supported with V2. Deserialization rejects
                // this invalid combination before it can reach routing.
                assert!(
                    version == PartitionKeyVersion::V2,
                    "MultiHash requires V2, got {:?}",
                    version
                );
                effective_partition_key_multi_hash_v2_binary(pk_values)
            }
            // Range partitioning is legacy; fall through to V2 as a reasonable default.
            _ => effective_partition_key_v2_binary(pk_values),
        };
        Self::from_bytes(bytes)
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
    pub(crate) fn compute_range(
        pk_values: &[PartitionKeyValue],
        pk_definition: &PartitionKeyDefinition,
    ) -> crate::error::Result<std::ops::Range<Self>> {
        if pk_values.is_empty() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_COMPUTE_RANGE_INVOKED_WITH_EMPTY_PARTITION_KEY)
                .with_message("compute_range called with empty pk_values")
                .build());
        }
        if pk_values.len() > pk_definition.paths().len() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_PARTITION_KEY_TOO_MANY_COMPONENTS)
                .with_message(format!(
                    "more partition key components ({}) than definition paths ({})",
                    pk_values.len(),
                    pk_definition.paths().len()
                ))
                .build());
        }

        let kind = pk_definition.kind();
        let version = pk_definition.version();
        let epk = Self::compute(pk_values, kind, version);

        let is_prefix =
            kind == PartitionKeyKind::MultiHash && pk_values.len() < pk_definition.paths().len();

        if kind != PartitionKeyKind::MultiHash && pk_values.len() != pk_definition.paths().len() {
            return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_NON_MULTIHASH_PARTITION_KEY_ARITY_MISMATCH).with_message(format!(
                    "non-MultiHash containers require exactly as many components ({}) as paths ({})",
                    pk_values.len(),
                    pk_definition.paths().len()
                )).build());
        }

        if is_prefix {
            // A trailing `0xFF` byte is a safe upper-bound sentinel because
            // `hash_v2_raw_bytes` masks byte 0 with 0x3F, so every EPK
            // component's first byte is in `[0x00, 0x3F]`. `0xFF` is greater
            // than any valid suffix byte.
            let mut max_bytes = epk.as_bytes().to_vec();
            max_bytes.push(0xFF);
            let max = Self::from_bytes(max_bytes);
            Ok(epk..max)
        } else {
            Ok(epk.clone()..epk)
        }
    }
}

impl fmt::Display for EffectivePartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl PartialEq<str> for EffectivePartitionKey {
    fn eq(&self, other: &str) -> bool {
        self.to_hex() == other
    }
}

impl PartialEq<&str> for EffectivePartitionKey {
    fn eq(&self, other: &&str) -> bool {
        self.to_hex() == *other
    }
}

impl From<String> for EffectivePartitionKey {
    /// Parses an upper- or lower-hex EPK string into raw bytes.
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&str> for EffectivePartitionKey {
    /// Parses an upper- or lower-hex EPK string into raw bytes.
    ///
    /// EPK strings always originate as even-length hex (service pkrange/query
    /// plan bounds, `compute` output, and the `"…FF"` prefix
    /// sentinel), so a malformed value is a caller bug; `hex_to_bytes` stops at
    /// the first byte that fails to parse rather than panicking on the wire path.
    fn from(s: &str) -> Self {
        Self(Cow::Owned(hex_to_bytes(s)))
    }
}

impl Serialize for EffectivePartitionKey {
    /// Serializes as the canonical uppercase-hex string for wire compatibility
    /// (e.g. `PartitionKeyRange` `minInclusive`/`maxExclusive`).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for EffectivePartitionKey {
    /// Deserializes from a hex string (the form the service emits for routing
    /// boundaries).
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from(s))
    }
}

/// Length-aware ordering for effective partition keys.
///
/// For hierarchical partition key (HPK) containers, the backend may return
/// partition key ranges with mixed-length boundaries: a partial EPK (16 bytes,
/// one hash component) and a fully specified EPK (32 bytes, two hash components
/// zero-padded). For example, as hex:
///
///   - Partial:   `06AB34CFE4E482236BCACBBF50E234AB`
///   - Full:      `06AB34CFE4E482236BCACBBF50E234AB00000000000000000000000000000000`
///
/// These represent the same partition boundary. Plain lexicographic comparison
/// treats the shorter value as "less than" the longer one when it is a prefix,
/// causing incorrect overlap detection in routing maps.
///
/// This implementation treats two EPKs as equal when one is a prefix of the
/// other and the remaining bytes are all zero.
///
/// See: <https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5260>
impl Ord for EffectivePartitionKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.as_bytes();
        let b = other.as_bytes();
        let common = a.len().min(b.len());
        match a[..common].cmp(&b[..common]) {
            std::cmp::Ordering::Equal => {
                let tail = if a.len() > b.len() {
                    &a[common..]
                } else {
                    &b[common..]
                };
                if tail.iter().all(|&byte| byte == 0) {
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

/// V2 EPK raw bytes (16 bytes): MurmurHash3-128, reversed, top 2 bits of byte 0 cleared.
///
/// This is the binary form sent in the RNTBD `EffectivePartitionKey`
/// token (0x005A) for V2 hash-partitioned collections.
pub(crate) fn hash_v2_raw_bytes(data: &[u8]) -> [u8; 16] {
    let hash_128 = murmurhash3_128(data, 0);
    let mut hash_bytes = hash_128.to_le_bytes();
    hash_bytes.reverse();
    hash_bytes[0] &= 0x3F;
    hash_bytes
}

/// V2 EPK raw bytes for hash-partitioned collections.
pub(crate) fn effective_partition_key_v2_binary(pk_values: &[PartitionKeyValue]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for v in pk_values {
        v.write_for_hashing_v2(&mut buf);
    }
    hash_v2_raw_bytes(&buf).to_vec()
}

/// MultiHash V2 EPK raw bytes: per-component MurmurHash3-128, concatenated.
///
/// Each component is independently encoded and hashed, and the 16-byte hashes
/// are concatenated to produce `16 * N` bytes for `N` components. This is the
/// binary form the proxy expects for `MultiHash` containers in the RNTBD
/// `EffectivePartitionKey` token (0x005A).
pub(crate) fn effective_partition_key_multi_hash_v2_binary(
    pk_values: &[PartitionKeyValue],
) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(pk_values.len() * 16);
    let mut buf: Vec<u8> = Vec::new();
    for v in pk_values {
        buf.clear();
        v.write_for_hashing_v2(&mut buf);
        out.extend_from_slice(&hash_v2_raw_bytes(&buf));
    }
    out
}

/// Builds the V1-binary-encoded effective partition key bytes for hash partitions.
///
/// This is the raw byte form sent in the RNTBD `EffectivePartitionKey`
/// token (0x005A). It's the binary tuple `Number(hash) + truncated_components`,
/// not the hex-encoded routing string. Empty-component and infinity cases are
/// handled by the caller (see [`EffectivePartitionKey::compute`]).
pub(crate) fn effective_partition_key_v1_binary(pk_values: &[PartitionKeyValue]) -> Vec<u8> {
    let mut hashing_bytes: Vec<u8> = Vec::new();
    for v in pk_values {
        v.write_for_hashing_v1(&mut hashing_bytes);
    }

    let hash32 = murmurhash3_32(&hashing_bytes, 0u32);

    let mut buffer: Vec<u8> = Vec::new();
    write_number_v1_binary(hash32 as f64, &mut buffer);

    // Truncate string components to MAX_STRING_BYTES_TO_APPEND, matching the
    // truncation applied during hashing.
    for v in pk_values {
        v.truncated_for_v1_encoding()
            .write_for_binary_encoding_v1(&mut buffer);
    }

    buffer
}

fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut s, "{:02X}", b).unwrap();
    }
    s
}

/// Decodes an even-length hex string (upper or lower case) into bytes.
///
/// Returns the bytes decoded so far if the input is malformed. EPK strings are
/// always well-formed, even-length hex in practice, so this lenient handling
/// only guards against caller bugs without panicking on the wire path.
fn hex_to_bytes(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        match (hex_nibble(pair[0]), hex_nibble(pair[1])) {
            (Some(hi), Some(lo)) => out.push((hi << 4) | lo),
            _ => break,
        }
    }
    out
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pk_returns_min() {
        let result =
            EffectivePartitionKey::compute(&[], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, EffectivePartitionKey::MIN.clone());
    }

    #[test]
    fn infinity_pk_returns_max() {
        let inf = PartitionKeyValue::INFINITY;
        let result =
            EffectivePartitionKey::compute(&[inf], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, EffectivePartitionKey::MAX.clone());
    }

    /// V2 hash test cases.
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
                actual.to_hex(),
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
        assert_eq!(actual.to_hex(), expected);
    }

    /// V1 hash test cases.
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
                actual.to_hex(),
                *expected,
                "V1 mismatch for {:?}",
                component
            );
        }
    }

    /// The definition's `version` must drive the EPK algorithm end-to-end:
    /// a version-less definition (deserialized as V1) must produce the V1
    /// binary encoding, while a V2 definition produces the V2 encoding for the
    /// same partition key value. This locks down the version→algorithm wiring
    /// that Gateway 2.0 point-op routing depends on.
    #[test]
    fn definition_version_selects_epk_algorithm() {
        let pk_values = [PartitionKeyValue::from("tenant1".to_string())];

        // A version-less definition read back from the service is a legacy V1
        // container (see `default_pk_version`).
        let v1_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        assert_eq!(v1_def.version(), PartitionKeyVersion::V1);
        let v2_def: PartitionKeyDefinition =
            serde_json::from_str(r#"{"paths":["/pk"],"version":2}"#).unwrap();
        assert_eq!(v2_def.version(), PartitionKeyVersion::V2);

        let v1_epk = EffectivePartitionKey::compute(&pk_values, v1_def.kind(), v1_def.version());
        let v2_epk = EffectivePartitionKey::compute(&pk_values, v2_def.kind(), v2_def.version());

        assert_eq!(
            v1_epk.as_bytes(),
            effective_partition_key_v1_binary(&pk_values).as_slice(),
            "version-less (V1) definition must produce the V1 EPK encoding"
        );
        assert_eq!(
            v2_epk.as_bytes(),
            effective_partition_key_v2_binary(&pk_values).as_slice(),
            "V2 definition must produce the V2 EPK encoding"
        );
        assert_ne!(
            v1_epk.as_bytes(),
            v2_epk.as_bytes(),
            "V1 and V2 EPK encodings must differ for the same partition key value"
        );
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
        assert_eq!(multi.to_hex(), single.to_hex());
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
        assert_eq!(multi.to_hex().len(), 64);

        // Expected values from the effective_partition_key_hash_v2 test cases above,
        // verified against cross-SDK baselines.
        // First 32 chars should match the single-component hash of "redmond"
        assert_eq!(&multi.to_hex()[..32], "22E342F38A486A088463DFF7838A5963");
        // Second 32 chars should match the single-component hash of 5.0
        assert_eq!(&multi.to_hex()[32..], "19C08621B135968252FB34B4CF66F811");
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
        assert_eq!(multi.to_hex().len(), 96);

        // Expected values from the effective_partition_key_hash_v2 test cases above,
        // verified against cross-SDK baselines.
        assert_eq!(&multi.to_hex()[..32], "22E342F38A486A088463DFF7838A5963");
        assert_eq!(&multi.to_hex()[32..64], "0E711127C5B5A8E4726AC6DD306A3E59");
        assert_eq!(&multi.to_hex()[64..], "378867E4430E67857ACE5C908374FE16");
    }

    /// MultiHash with an Undefined component (used for partial HPK).
    #[test]
    fn multi_hash_with_undefined() {
        let pk = vec![
            PartitionKeyValue::from("tenant1".to_string()),
            PartitionKeyValue::UNDEFINED,
        ];
        let multi = EffectivePartitionKey::compute(
            &pk,
            PartitionKeyKind::MultiHash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(multi.to_hex().len(), 64);

        // First segment: hash of "tenant1"
        let single_tenant = EffectivePartitionKey::compute(
            &[PartitionKeyValue::from("tenant1".to_string())],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(&multi.to_hex()[..32], single_tenant.to_hex());

        // Second segment: hash of Undefined (0x00 byte)
        let single_undef = EffectivePartitionKey::compute(
            &[PartitionKeyValue::UNDEFINED],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(&multi.to_hex()[32..], single_undef.to_hex());
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
        assert_eq!(single.to_hex().len(), 32);
        assert_eq!(multi.to_hex().len(), 128);
        assert_ne!(multi.to_hex(), single.to_hex());
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
        assert_eq!(range.start.to_hex().len(), 96);
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
        assert_eq!(range.start.to_hex().len(), 64);
        // max EPK = min + "FF" = 66 chars
        assert_eq!(range.end.to_hex().len(), 66);
        assert!(range.end.to_hex().ends_with("FF"));
        assert!(range.end.to_hex().starts_with(&range.start.to_hex()));
    }

    /// compute_range returns an EPK range for a prefix (1 of 3 components).
    #[test]
    fn compute_range_prefix_one_of_three() {
        let pk = vec![PartitionKeyValue::from("tenant1".to_string())];
        let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        let range = EffectivePartitionKey::compute_range(&pk, &pk_def).unwrap();

        assert_ne!(range.start, range.end);
        assert_eq!(range.start.to_hex().len(), 32);
        assert_eq!(range.end.to_hex().len(), 34);
        assert!(range.end.to_hex().ends_with("FF"));
    }

    /// The `[epk, epk+"FF")` prefix range relies on every hash component's first
    /// hex digit being masked into `[0-3]` (`hash_bytes[0] & 0x3F`), so the "FF"
    /// sentinel is always lexicographically greater than any real completion of
    /// the prefix. Guard that coupling so `compute_range` and the planner's
    /// prefix clip cannot silently break.
    #[test]
    fn compute_range_first_nibble_masked_keeps_ff_sentinel_valid() {
        let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
        for values in [
            vec![PartitionKeyValue::from("tenant1".to_string())],
            vec![
                PartitionKeyValue::from("tenant1".to_string()),
                PartitionKeyValue::from("user1".to_string()),
            ],
        ] {
            let range = EffectivePartitionKey::compute_range(&values, &pk_def).unwrap();
            let start = range.start.to_hex();
            // Each hash component is 32 hex chars; the first digit of every
            // component must be in [0-3] for the "FF" upper bound to dominate.
            for component_start in (0..start.len()).step_by(32) {
                let first = start.as_bytes()[component_start];
                assert!(
                    (b'0'..=b'3').contains(&first),
                    "component at {component_start} starts with '{}', expected [0-3]",
                    first as char
                );
            }
            assert!(
                range.end.to_hex() > start,
                "the FF sentinel must exceed the prefix EPK"
            );
        }
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
                        v2_epk.to_hex(),
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
                        v2_epk.to_hex(),
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
                        !v1_epk.to_hex().is_empty(),
                        "V1 full pipeline produced empty EPK for {} (value: {})",
                        tc.description,
                        tc.partition_key_value,
                    );
                }
            }

            // --- Cross-SDK raw hash baseline ---
            //
            // Verifies byte encoding + MurmurHash correctness against the same
            // cross-SDK baselines. Uses canonical
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
