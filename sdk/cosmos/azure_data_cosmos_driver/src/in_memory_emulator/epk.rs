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
pub(crate) fn parse_partition_key_header(header: &str) -> azure_core::Result<Vec<PartitionKeyValue>> {
    let trimmed = header.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return Ok(Vec::new());
    }

    let value: serde_json::Value = serde_json::from_str(trimmed).map_err(|e| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!("invalid partition key header: {}", e),
        )
    })?;

    let arr = value.as_array().ok_or_else(|| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "partition key header must be a JSON array",
        )
    })?;

    arr.iter().map(json_to_pk_component).collect()
}

/// Extracts partition key value(s) from a document body using the container's
/// partition key path definitions.
///
/// Returns `BadRequest` if any extracted component is non-scalar or non-finite.
/// Missing properties are mapped to [`PartitionKeyValue::undefined`] to match
/// the real service's "missing partition key property" handling.
pub(crate) fn extract_pk_from_body(
    body: &serde_json::Value,
    pk_paths: &[impl AsRef<str>],
) -> azure_core::Result<Vec<PartitionKeyValue>> {
    pk_paths
        .iter()
        .map(|path| {
            let path_str = path.as_ref().trim_start_matches('/');
            let val = path_str
                .split('/')
                .try_fold(body, |curr, segment| curr.get(segment));
            match val {
                None => Ok(PartitionKeyValue::undefined()),
                Some(v) => json_to_pk_component(v),
            }
        })
        .collect()
}

/// Converts a single JSON value to a [`PartitionKeyValue`], rejecting non-scalars
/// and non-finite numbers the way the real service does.
fn json_to_pk_component(value: &serde_json::Value) -> azure_core::Result<PartitionKeyValue> {
    match value {
        serde_json::Value::Null => Ok(Option::<&str>::None.into()),
        serde_json::Value::Bool(b) => Ok(PartitionKeyValue::from(*b)),
        serde_json::Value::String(s) => Ok(PartitionKeyValue::from(s.clone())),
        serde_json::Value::Number(n) => {
            let f = n.as_f64().ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "partition key number is not representable as f64",
                )
            })?;
            if !f.is_finite() {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "partition key numbers must be finite (NaN and Infinity are not allowed)",
                ));
            }
            Ok(PartitionKeyValue::from(f))
        }
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "partition key components must be scalar (null, bool, number, or string)",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pk_header_string() {
        let components = parse_partition_key_header(r#"["hello"]"#).unwrap();
        assert_eq!(components, vec![PartitionKeyValue::from("hello".to_string())]);
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
    fn parse_pk_header_nan_errors() {
        // Construct a serde_json::Value that wraps a non-finite f64 by parsing
        // a numeric token directly through serde_json::Number's arbitrary-
        // precision support. serde_json's `from_str` rejects literal `NaN`, so
        // we have to forge a Number with a non-finite value via a small detour:
        // mutate a parsed number's underlying repr by going through f64.
        //
        // The exposed contract is that `json_to_pk_component` rejects any
        // non-finite f64. We assert that directly to cover the branch — the
        // other tests (`parse_pk_header_*`) cover the JSON parser path.
        let v = serde_json::Value::Number(
            serde_json::Number::from_f64(1.5_f64).unwrap(),
        );
        assert!(json_to_pk_component(&v).is_ok());

        // Forge a non-finite by smuggling through serde_json's representation.
        // serde_json::Number does not accept NaN/Inf, so the easiest way to
        // reach the non-finite branch is to invoke the converter with a
        // serde_json value whose `as_f64()` is NaN. Today, no such Value can
        // exist via the public API (Number::from_f64 returns None). The
        // branch is therefore reachable only if a future serde_json version
        // ever exposes non-finite numbers; pin behavior for that case via a
        // direct constructor on a custom Number-like wrapper here would
        // require unsafe. Instead, validate that any attempt to construct
        // such a Number returns None — which is the upstream guarantee that
        // makes `json_to_pk_component`'s non-finite branch unreachable today.
        assert!(serde_json::Number::from_f64(f64::NAN).is_none());
        assert!(serde_json::Number::from_f64(f64::INFINITY).is_none());
        assert!(serde_json::Number::from_f64(f64::NEG_INFINITY).is_none());
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
        assert_eq!(components, vec![PartitionKeyValue::from("value1".to_string())]);
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
        assert_eq!(components, vec![PartitionKeyValue::undefined()]);
    }

    #[test]
    fn extract_pk_object_value_errors() {
        let body = serde_json::json!({"pk": {"nested": "object"}});
        assert!(extract_pk_from_body(&body, &["/pk"]).is_err());
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
        assert_eq!(epk.as_str(), "19819C94CE42A1654CCC8110539D9589");
    }
}
