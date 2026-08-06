// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Effective partition key (EPK) helpers for the in-memory emulator.
//!
//! Thin wrappers around the production EPK implementation in
//! [`crate::models::effective_partition_key`]. The emulator must never reimplement
//! hashing — doing so would let emulator and production drift. This module only
//! adds two pieces of glue not exposed publicly by the product code:
//!
//! - [`parse_partition_key_header`]: parses the JSON-encoded
//!   `x-ms-documentdb-partitionkey` request header into a vector of
//!   [`PartitionKeyValue`].
//! - [`extract_pk_from_body`]: walks a document body using the container's
//!   partition key paths to pull out the partition key components.
//!
//! Both helpers validate their inputs the way the real Cosmos DB gateway does
//! (reject NaN/Infinity, reject non-scalar values, reject malformed JSON) so that
//! buggy callers fail with 400 BadRequest in the emulator just like they would
//! against a real account.

use crate::models::{PartitionKeyKind, PartitionKeyValue, PartitionKeyVersion};

/// Re-export of the production [`EffectivePartitionKey`](crate::models::effective_partition_key::EffectivePartitionKey).
///
/// Kept under the `Epk` alias so existing emulator code reads naturally; all
/// hashing, ordering, and serialization is handled by the product type.
pub use crate::models::effective_partition_key::EffectivePartitionKey as Epk;

/// Re-export of [`PartitionKeyValue`] under the alias used inside the emulator.
pub(crate) type PartitionKeyComponent = PartitionKeyValue;

/// Computes the effective partition key for a set of components.
///
/// Delegates entirely to [`Epk::compute`] so that the emulator and production
/// share a single, tested implementation.
pub(crate) fn compute_epk(
    components: &[PartitionKeyValue],
    kind: PartitionKeyKind,
    version: PartitionKeyVersion,
) -> Epk {
    Epk::compute(components, kind, version)
}

/// Parses a JSON-encoded partition key header value (e.g., `["pk1"]`, `[42]`,
/// `[null]`, `["tenant1","user1"]`) into a list of [`PartitionKeyValue`] components.
///
/// Mirrors the gateway's validation:
/// - Empty strings and `[]` parse to an empty vector (cross-partition).
/// - Malformed JSON returns `BadRequest` (HTTP 400).
/// - NaN / +Inf / -Inf numbers return `BadRequest` (HTTP 400).
/// - Object / array components return `BadRequest` (HTTP 400).
pub(crate) fn parse_partition_key_header(
    header: &str,
) -> crate::error::Result<Vec<PartitionKeyValue>> {
    let trimmed = header.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return Ok(Vec::new());
    }

    let value: serde_json::Value = serde_json::from_str(trimmed).map_err(|e| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message(format!("invalid partition key header: {e}"))
            .build()
    })?;

    let arr = value.as_array().ok_or_else(|| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("partition key header must be a JSON array")
            .build()
    })?;

    arr.iter().map(json_to_pk_component).collect()
}

/// Extracts partition key value(s) from a document body using the container's
/// partition key path definitions.
///
/// Returns `BadRequest` if the body is not a JSON object, if any path
/// traversal encounters a non-object intermediate, or if any extracted
/// component is non-scalar or non-finite. Missing properties (where every
/// intermediate is an object but the leaf is absent) are mapped to
/// [`PartitionKeyValue::undefined`] to match the real service's "missing
/// partition key property" handling.
pub(crate) fn extract_pk_from_body(
    body: &serde_json::Value,
    pk_paths: &[impl AsRef<str>],
) -> crate::error::Result<Vec<PartitionKeyValue>> {
    if !body.is_object() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("document body must be a JSON object to extract a partition key")
            .build());
    }
    pk_paths
        .iter()
        .map(|path| extract_pk_at_path(body, path.as_ref()))
        .collect()
}

/// Walks `body` along `path` (slash-separated, leading `/` stripped),
/// rejecting non-object intermediates and converting the leaf to a
/// [`PartitionKeyValue`]. A leaf-absent traversal returns
/// [`PartitionKeyValue::undefined`].
fn extract_pk_at_path(
    body: &serde_json::Value,
    path: &str,
) -> crate::error::Result<PartitionKeyValue> {
    let path_str = path.trim_start_matches('/');
    if path_str.is_empty() {
        return json_to_pk_component(body);
    }
    let segments: Vec<&str> = path_str.split('/').collect();
    let last_idx = segments.len() - 1;
    let mut current = body;
    for (i, segment) in segments.iter().enumerate() {
        let obj = current.as_object().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "partition key path component '{segment}' encountered a non-object intermediate"
                ))
                .build()
        })?;
        match obj.get(*segment) {
            Some(next) if i == last_idx => return json_to_pk_component(next),
            Some(next) => current = next,
            None => return Ok(PartitionKeyValue::UNDEFINED),
        }
    }
    // Unreachable: loop returns or assigns on every iteration.
    Ok(PartitionKeyValue::UNDEFINED)
}

/// Converts a single JSON value to a [`PartitionKeyValue`], rejecting non-scalars
/// and non-finite numbers the way the real service does.
fn json_to_pk_component(value: &serde_json::Value) -> crate::error::Result<PartitionKeyValue> {
    match value {
        serde_json::Value::Null => Ok(Option::<&str>::None.into()),
        serde_json::Value::Bool(b) => Ok(PartitionKeyValue::from(*b)),
        serde_json::Value::String(s) => Ok(PartitionKeyValue::from(s.clone())),
        serde_json::Value::Number(n) => {
            let f = n.as_f64().ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("partition key number is not representable as f64")
                    .build()
            })?;
            if !f.is_finite() {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message(
                        "partition key numbers must be finite (NaN and Infinity are not allowed)",
                    )
                    .build());
            }
            Ok(PartitionKeyValue::from(f))
        }
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(
                    "partition key components must be scalar (null, bool, number, or string)",
                )
                .build())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pk_header_string() {
        let components = parse_partition_key_header(r#"["hello"]"#).unwrap();
        assert_eq!(
            components,
            vec![PartitionKeyValue::from("hello".to_string())]
        );
    }

    #[test]
    fn parse_pk_header_number() {
        let components = parse_partition_key_header("[42]").unwrap();
        assert_eq!(components, vec![PartitionKeyValue::from(42.0_f64)]);
    }

    #[test]
    fn parse_pk_header_null() {
        let components = parse_partition_key_header("[null]").unwrap();
        let expected: PartitionKeyValue = Option::<&str>::None.into();
        assert_eq!(components, vec![expected]);
    }

    #[test]
    fn parse_pk_header_hierarchical() {
        let components = parse_partition_key_header(r#"["tenant1", "user1"]"#).unwrap();
        assert_eq!(
            components,
            vec![
                PartitionKeyValue::from("tenant1".to_string()),
                PartitionKeyValue::from("user1".to_string()),
            ]
        );
    }

    #[test]
    fn parse_pk_header_empty_is_cross_partition() {
        assert!(parse_partition_key_header("[]").unwrap().is_empty());
        assert!(parse_partition_key_header("").unwrap().is_empty());
    }

    #[test]
    fn parse_pk_header_invalid_json_errors() {
        assert!(parse_partition_key_header("not json").is_err());
        assert!(parse_partition_key_header("{\"x\":1}").is_err());
    }

    #[test]
    fn parse_pk_header_object_errors() {
        let err = parse_partition_key_header(r#"[{"x":1}]"#).unwrap_err();
        assert!(err.to_string().contains("scalar"));
    }

    #[test]
    fn extract_pk_from_json_body() {
        let body = serde_json::json!({"id": "doc1", "pk": "value1", "nested": {"key": 42}});
        let components = extract_pk_from_body(&body, &["/pk"]).unwrap();
        assert_eq!(
            components,
            vec![PartitionKeyValue::from("value1".to_string())]
        );
    }

    #[test]
    fn extract_pk_nested() {
        let body = serde_json::json!({"id": "doc1", "nested": {"key": 42}});
        let components = extract_pk_from_body(&body, &["/nested/key"]).unwrap();
        assert_eq!(components, vec![PartitionKeyValue::from(42.0_f64)]);
    }

    #[test]
    fn extract_pk_missing_path_is_undefined() {
        let body = serde_json::json!({"id": "doc1"});
        let components = extract_pk_from_body(&body, &["/missing"]).unwrap();
        assert_eq!(components, vec![PartitionKeyValue::UNDEFINED]);
    }

    #[test]
    fn extract_pk_object_value_errors() {
        let body = serde_json::json!({"pk": {"nested": "object"}});
        assert!(extract_pk_from_body(&body, &["/pk"]).is_err());
    }

    #[test]
    fn extract_pk_non_object_body_errors() {
        let body = serde_json::json!([1, 2, 3]);
        assert!(extract_pk_from_body(&body, &["/pk"]).is_err());
        let body = serde_json::json!("string");
        assert!(extract_pk_from_body(&body, &["/pk"]).is_err());
    }

    #[test]
    fn extract_pk_non_object_intermediate_errors() {
        // Real Cosmos rejects PK paths that traverse a scalar intermediate.
        let body = serde_json::json!({"a": 42});
        assert!(extract_pk_from_body(&body, &["/a/b"]).is_err());
        let body = serde_json::json!({"a": [1, 2, 3]});
        assert!(extract_pk_from_body(&body, &["/a/b"]).is_err());
    }

    #[test]
    fn compute_epk_uses_product_v2_vector() {
        // Spot-check one well-known V2 vector. Full vector coverage lives in the
        // product crate's `effective_partition_key::tests`.
        let epk = compute_epk(
            &[PartitionKeyValue::from("customer42".to_string())],
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V2,
        );
        assert_eq!(epk.to_hex(), "19819C94CE42A1654CCC8110539D9589");
    }

    /// Header-vs-body parity: a partition key value extracted from an item
    /// body MUST hash to the same EPK as the wire-format header value the
    /// SDK computes for the matching point operation. Without this, a
    /// document inserted as `{"pk": 42}` would land in a different
    /// partition than a read carrying `x-ms-documentdb-partitionkey: [42]`,
    /// so the read would 404 even though the doc is in the container.
    ///
    /// Covers the numeric paths most likely to drift: small integers,
    /// fractional doubles, large doubles past 2^53, and the boundary
    /// between number-as-int and number-as-f64 representations.
    #[test]
    fn header_and_body_extraction_produce_identical_epk() {
        let cases: Vec<(&str, serde_json::Value, &str)> = vec![
            ("integer 42", serde_json::json!({"pk": 42}), "[42]"),
            ("integer 0", serde_json::json!({"pk": 0}), "[0]"),
            ("negative integer", serde_json::json!({"pk": -7}), "[-7]"),
            (
                "integer-as-f64 42.0",
                serde_json::json!({"pk": 42.0}),
                "[42.0]",
            ),
            ("fractional 1.5", serde_json::json!({"pk": 1.5}), "[1.5]"),
            ("large 1e10", serde_json::json!({"pk": 1e10}), "[1e10]"),
            (
                "u64 boundary 2^53",
                serde_json::json!({"pk": 9_007_199_254_740_992_u64}),
                "[9007199254740992]",
            ),
            ("string", serde_json::json!({"pk": "abc"}), r#"["abc"]"#),
            ("bool true", serde_json::json!({"pk": true}), "[true]"),
            ("bool false", serde_json::json!({"pk": false}), "[false]"),
            ("null", serde_json::json!({"pk": null}), "[null]"),
        ];

        for (label, body, header) in cases {
            let from_body = extract_pk_from_body(&body, &["/pk"]).unwrap_or_else(|e| {
                panic!("body extraction failed for {}: {}", label, e);
            });
            let from_header = parse_partition_key_header(header).unwrap_or_else(|e| {
                panic!("header parsing failed for {}: {}", label, e);
            });
            assert_eq!(
                from_body, from_header,
                "components diverge for {}: body={:?} header={:?}",
                label, from_body, from_header
            );

            let epk_body = compute_epk(&from_body, PartitionKeyKind::Hash, PartitionKeyVersion::V2);
            let epk_header = compute_epk(
                &from_header,
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V2,
            );
            assert_eq!(
                epk_body,
                epk_header,
                "EPK diverges for {}: body={} header={}",
                label,
                epk_body.to_hex(),
                epk_header.to_hex(),
            );

            // V1 too — hierarchical-PK V1 containers exist and the same
            // body/header parity must hold there.
            let epk_body_v1 =
                compute_epk(&from_body, PartitionKeyKind::Hash, PartitionKeyVersion::V1);
            let epk_header_v1 = compute_epk(
                &from_header,
                PartitionKeyKind::Hash,
                PartitionKeyVersion::V1,
            );
            assert_eq!(
                epk_body_v1,
                epk_header_v1,
                "V1 EPK diverges for {}: body={} header={}",
                label,
                epk_body_v1.to_hex(),
                epk_header_v1.to_hex(),
            );
        }
    }
}
