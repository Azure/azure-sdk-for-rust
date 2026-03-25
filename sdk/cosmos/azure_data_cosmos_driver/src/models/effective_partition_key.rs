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
            compute_effective_partition_key(&[], PartitionKeyKind::Hash, PartitionKeyVersion::V2);
        assert_eq!(result, MIN_INCLUSIVE_EFFECTIVE_PARTITION_KEY);
    }

    #[test]
    fn infinity_pk_returns_max() {
        let inf = PartitionKeyValue::infinity();
        let result = compute_effective_partition_key(
            &[inf],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(result, MAX_EXCLUSIVE_EFFECTIVE_PARTITION_KEY);
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
            let actual = compute_effective_partition_key(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V2,
            );
            assert_eq!(&actual, expected, "V2 mismatch for {:?}", component);
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

        let actual = compute_effective_partition_key(
            &components,
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(actual, expected);
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
            let actual = compute_effective_partition_key(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V1,
            );
            assert_eq!(&actual, expected, "V1 mismatch for {:?}", component);
        }
    }
}

/// Cross-SDK baseline tests using the same XML datasets as the Go SDK.
///
/// These tests validate the core hash computation and encoding building blocks
/// against a shared baseline, ensuring cross-language consistency.
///
/// See: <https://github.com/Azure/azure-sdk-for-go/blob/main/sdk/data/azcosmos/internal/epk/epk_test.go>
#[cfg(test)]
mod baseline_tests {
    use crate::models::murmur_hash::{murmurhash3_128, murmurhash3_32};
    use std::fmt::Write;

    // Embed XML test data within the test module so it's absent from product binaries.
    const SINGLETONS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Singletons.xml");
    const NUMBERS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Numbers.xml");
    const STRINGS_XML: &str =
        include_str!("../../testdata/PartitionKeyHashBaselineTest.Strings.xml");
    const LISTS_XML: &str = include_str!("../../testdata/PartitionKeyHashBaselineTest.Lists.xml");

    // -- Value representation (matches Go's `parseValues`) --

    enum TestValue {
        Undefined,
        Null,
        Bool(bool),
        Number(f64),
        String(String),
    }

    /// Parse the XML `PartitionKeyValue` field into test values, matching Go's `parseValues`.
    fn parse_values(raw: &str) -> Vec<TestValue> {
        if raw == "UNDEFINED" {
            return vec![TestValue::Undefined];
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

    fn parse_single_json_value(raw: &str) -> TestValue {
        match raw {
            "null" => TestValue::Null,
            "true" => TestValue::Bool(true),
            "false" => TestValue::Bool(false),
            s if s.starts_with('"') && s.ends_with('"') => {
                let inner = &s[1..s.len() - 1];
                match inner {
                    // Use .NET's NaN bit pattern so the hash matches the baseline.
                    "NaN" => TestValue::Number(f64::from_bits(0xFFF8000000000000)),
                    "-Infinity" => TestValue::Number(f64::NEG_INFINITY),
                    "Infinity" => TestValue::Number(f64::INFINITY),
                    _ => TestValue::String(inner.to_string()),
                }
            }
            _ => TestValue::Number(
                raw.parse()
                    .unwrap_or_else(|e| panic!("failed to parse number '{raw}': {e}")),
            ),
        }
    }

    // -- XML parsing --

    struct BaselineResult {
        description: String,
        partition_key_value: String,
        v1_hash: String,
        v2_hash: String,
    }

    fn parse_baseline_xml(xml: &str) -> Vec<BaselineResult> {
        let mut results = Vec::new();
        for block in xml.split("<Result>").skip(1) {
            let end = block
                .find("</Result>")
                .expect("malformed XML: missing </Result>");
            let block = &block[..end];

            let desc = extract_tag(block, "Description").expect("missing <Description>");
            let pk_val =
                extract_tag(block, "PartitionKeyValue").expect("missing <PartitionKeyValue>");
            let v1 =
                extract_tag(block, "PartitionKeyHashV1").expect("missing <PartitionKeyHashV1>");
            let v2 =
                extract_tag(block, "PartitionKeyHashV2").expect("missing <PartitionKeyHashV2>");

            results.push(BaselineResult {
                description: desc.to_string(),
                partition_key_value: pk_val.to_string(),
                v1_hash: v1.to_string(),
                v2_hash: v2.to_string(),
            });
        }
        results
    }

    fn extract_tag<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = xml.find(&open)? + open.len();
        let end = xml[start..].find(&close)? + start;
        Some(&xml[start..end])
    }

    // -- Hash computation matching Go SDK functions --

    /// Byte markers for partition key value encoding (same as Go constants).
    mod component {
        pub const UNDEFINED: u8 = 0x00;
        pub const NULL: u8 = 0x01;
        pub const BOOL_FALSE: u8 = 0x02;
        pub const BOOL_TRUE: u8 = 0x03;
        pub const NUMBER: u8 = 0x05;
        pub const STRING: u8 = 0x08;
    }

    /// Encode a value for V1 hashing (string suffix = 0x00, no truncation).
    /// Matches Go's `writeForHashing`.
    fn encode_for_v1(value: &TestValue, buf: &mut Vec<u8>) {
        encode_core(value, 0x00, buf);
    }

    /// Encode a value for V2 hashing (string suffix = 0xFF, no truncation).
    /// Matches Go's `writeForHashingV2`.
    fn encode_for_v2(value: &TestValue, buf: &mut Vec<u8>) {
        encode_core(value, 0xFF, buf);
    }

    fn encode_core(value: &TestValue, string_suffix: u8, buf: &mut Vec<u8>) {
        match value {
            TestValue::Undefined => buf.push(component::UNDEFINED),
            TestValue::Null => buf.push(component::NULL),
            TestValue::Bool(false) => buf.push(component::BOOL_FALSE),
            TestValue::Bool(true) => buf.push(component::BOOL_TRUE),
            TestValue::Number(f) => {
                buf.push(component::NUMBER);
                buf.extend_from_slice(&f.to_le_bytes());
            }
            TestValue::String(s) => {
                buf.push(component::STRING);
                buf.extend_from_slice(s.as_bytes());
                buf.push(string_suffix);
            }
        }
    }

    /// Compute V1 hash matching Go's `ComputeV1`:
    /// per-component MurmurHash3-32, formatted as 24 zero hex chars + 8 hex hash chars.
    fn compute_v1_baseline(values: &[TestValue]) -> String {
        let mut result = String::new();
        for v in values {
            let mut buf = Vec::new();
            encode_for_v1(v, &mut buf);
            let hash = murmurhash3_32(&buf, 0);
            write!(&mut result, "000000000000000000000000{hash:08X}").unwrap();
        }
        result
    }

    /// Compute V2 single-hash matching Go's `ComputeV2Hash`:
    /// all components concatenated, MurmurHash3-128, reversed bytes, uppercase hex.
    fn compute_v2_hash_baseline(values: &[TestValue]) -> String {
        let mut buf = Vec::new();
        for v in values {
            encode_for_v2(v, &mut buf);
        }
        hash128_to_epk(&buf)
    }

    /// Compute V2 multi-hash matching Go's `ComputeV2MultiHash`:
    /// per-component MurmurHash3-128, concatenated.
    fn compute_v2_multi_hash_baseline(values: &[TestValue]) -> String {
        let mut result = String::new();
        for v in values {
            let mut buf = Vec::new();
            encode_for_v2(v, &mut buf);
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

    fn run_baseline(xml: &str, multi_hash: bool) {
        let cases = parse_baseline_xml(xml);
        assert!(!cases.is_empty(), "no test cases parsed from XML");

        for tc in &cases {
            let values = parse_values(&tc.partition_key_value);

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
