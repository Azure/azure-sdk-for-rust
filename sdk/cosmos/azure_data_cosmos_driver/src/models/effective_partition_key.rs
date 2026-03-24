// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Effective partition key (EPK) computation.
//!
//! Hashes partition key values into a hex-encoded effective partition key string
//! that can be used to locate the target partition key range.

use crate::models::{
    murmur_hash::{murmurhash3_128, murmurhash3_32},
    partition_key::write_number_v1_binary,
    PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion,
};
use std::fmt::Write;

/// Minimum inclusive effective partition key (empty string).
pub(crate) const MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "";

/// Maximum exclusive effective partition key ("FF").
pub(crate) const MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "FF";

/// Computes the effective partition key string from partition key values.
///
/// This determines which partition key range a given partition key maps to.
pub(crate) fn compute_effective_partition_key(
    pk_values: &[PartitionKeyValue],
    kind: PartitionKeyKind,
    version: PartitionKeyVersion,
) -> String {
    if pk_values.is_empty() {
        return MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string();
    }
    if pk_values.len() == 1 && pk_values[0].is_infinity() {
        return MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string();
    }

    match kind {
        PartitionKeyKind::Hash => match version {
            PartitionKeyVersion::V1 => effective_partition_key_hash_v1(pk_values),
            PartitionKeyVersion::V2 => effective_partition_key_hash_v2(pk_values),
        },
        // Range partitioning is legacy; fall through to V2 as a reasonable default.
        _ => effective_partition_key_hash_v2(pk_values),
    }
}

/// V2: MurmurHash3-128, then reverse bytes and clear top 2 bits.
fn effective_partition_key_hash_v2(pk_values: &[PartitionKeyValue]) -> String {
    let mut buf: Vec<u8> = Vec::new();
    for v in pk_values {
        v.write_for_hashing_v2(&mut buf);
    }
    let hash_128 = murmurhash3_128(&buf, 0);
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

    // Append the original components (truncated strings).
    for v in pk_values {
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
            compute_effective_partition_key(&[], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY);
    }

    #[test]
    fn string_pk_hash_v2_matches_sdk() {
        // Known value from the SDK test suite.
        let pk = PartitionKeyValue::from("customer42".to_string());
        let result =
            compute_effective_partition_key(&[pk], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, "19819C94CE42A1654CCC8110539D9589");
    }

    #[test]
    fn string_pk_hash_v2_empty_string() {
        let pk = PartitionKeyValue::from("".to_string());
        let result =
            compute_effective_partition_key(&[pk], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, "32E9366E637A71B4E710384B2F4970A0");
    }

    #[test]
    fn string_pk_hash_v2_partition_key() {
        let pk = PartitionKeyValue::from("partitionKey".to_string());
        let result =
            compute_effective_partition_key(&[pk], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, "013AEFCF77FA271571CF665A58C933F1");
    }

    #[test]
    fn bool_true_hash_v2() {
        let pk = PartitionKeyValue::from(true);
        let result =
            compute_effective_partition_key(&[pk], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, "0E711127C5B5A8E4726AC6DD306A3E59");
    }

    #[test]
    fn bool_false_hash_v2() {
        let pk = PartitionKeyValue::from(false);
        let result =
            compute_effective_partition_key(&[pk], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, "2FE1BE91E90A3439635E0E9E37361EF2");
    }
}
