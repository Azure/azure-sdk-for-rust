// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB value type with type-aware comparison semantics.
//!
//! Cosmos DB has a specific type ordering for comparisons:
//! `null < boolean < number < string < array < object`
//!
//! Cross-type comparisons (except equality which returns false) produce `Undefined`.
//! `undefined` compared with anything is `Undefined`.

use std::cmp::Ordering;

/// A runtime value used during query evaluation, with Cosmos DB comparison semantics.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub(crate) enum CosmosValue {
    Null,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    String(String),
    Array(Vec<CosmosValue>),
    Object(Vec<(String, CosmosValue)>),
    Undefined,
}

/// Type order for Cosmos DB comparison semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TypeOrder {
    Null = 0,
    Boolean = 1,
    Number = 2,
    String = 3,
    Array = 4,
    Object = 5,
}

impl CosmosValue {
    fn type_order(&self) -> Option<TypeOrder> {
        match self {
            Self::Null => Some(TypeOrder::Null),
            Self::Boolean(_) => Some(TypeOrder::Boolean),
            Self::Number(_) | Self::Integer(_) => Some(TypeOrder::Number),
            Self::String(_) => Some(TypeOrder::String),
            Self::Array(_) => Some(TypeOrder::Array),
            Self::Object(_) => Some(TypeOrder::Object),
            Self::Undefined => None,
        }
    }

    /// Cosmos DB equality: returns `Undefined` for cross-type, `true`/`false` for same-type.
    pub(crate) fn cosmos_eq(&self, other: &Self) -> CosmosValue {
        match (self, other) {
            (Self::Undefined, _) | (_, Self::Undefined) => Self::Undefined,
            (Self::Null, Self::Null) => Self::Boolean(true),
            (Self::Boolean(a), Self::Boolean(b)) => Self::Boolean(a == b),
            (Self::Number(a), Self::Number(b)) => Self::Boolean(float_eq(*a, *b)),
            (Self::Integer(a), Self::Integer(b)) => Self::Boolean(a == b),
            (Self::Number(a), Self::Integer(b)) => Self::Boolean(float_eq(*a, *b as f64)),
            (Self::Integer(a), Self::Number(b)) => Self::Boolean(float_eq(*a as f64, *b)),
            (Self::String(a), Self::String(b)) => Self::Boolean(a == b),
            _ => {
                // Cross-type comparison
                if self.type_order() == other.type_order() {
                    // Same type but complex (array/object) — structural comparison
                    Self::Boolean(self.structural_eq(other))
                } else {
                    Self::Undefined
                }
            }
        }
    }

    /// Cosmos DB ordering comparison. Returns None for cross-type or undefined.
    pub(crate) fn cosmos_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Undefined, _) | (_, Self::Undefined) => None,
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Boolean(a), Self::Boolean(b)) => Some(a.cmp(b)),
            (Self::Number(a), Self::Number(b)) => float_cmp(*a, *b),
            (Self::Integer(a), Self::Integer(b)) => Some(a.cmp(b)),
            (Self::Number(a), Self::Integer(b)) => float_cmp(*a, *b as f64),
            (Self::Integer(a), Self::Number(b)) => float_cmp(*a as f64, *b),
            (Self::String(a), Self::String(b)) => Some(a.cmp(b)),
            _ => {
                if self.type_order() == other.type_order() {
                    // Same complex type — not comparable by ordering in general
                    None
                } else {
                    // Cross-type → undefined
                    None
                }
            }
        }
    }

    /// Deep structural equality for arrays and objects.
    fn structural_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Array(a), Self::Array(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| matches!(x.cosmos_eq(y), CosmosValue::Boolean(true)))
            }
            (Self::Object(a), Self::Object(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (key, val) in a {
                    let found = b.iter().find(|(k, _)| k == key);
                    match found {
                        Some((_, other_val)) => {
                            if !matches!(val.cosmos_eq(other_val), CosmosValue::Boolean(true)) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// Convert from `serde_json::Value`.
    pub(crate) fn from_json(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Boolean(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Self::Integer(i)
                } else {
                    Self::Number(n.as_f64().unwrap_or(0.0))
                }
            }
            serde_json::Value::String(s) => Self::String(s.clone()),
            serde_json::Value::Array(arr) => Self::Array(arr.iter().map(Self::from_json).collect()),
            serde_json::Value::Object(obj) => Self::Object(
                obj.iter()
                    .map(|(k, v)| (k.clone(), Self::from_json(v)))
                    .collect(),
            ),
        }
    }

    /// Convert to `serde_json::Value`.
    ///
    /// Top-level `Undefined` is rendered as `Value::Null` for callers that
    /// require a concrete JSON value; container positions (object properties
    /// and array elements) elide `Undefined` per Cosmos SQL semantics.
    pub(crate) fn to_json(&self) -> serde_json::Value {
        self.to_json_opt().unwrap_or(serde_json::Value::Null)
    }

    /// Convert to a `serde_json::Value`, returning `None` for `Undefined`.
    ///
    /// Cosmos SQL semantics: in object property positions and array element
    /// positions, `Undefined` is omitted entirely. Callers that need a
    /// top-level representation should fall back to `Value::Null`.
    fn to_json_opt(&self) -> Option<serde_json::Value> {
        match self {
            Self::Undefined => None,
            Self::Null => Some(serde_json::Value::Null),
            Self::Boolean(b) => Some(serde_json::Value::Bool(*b)),
            Self::Integer(n) => Some(serde_json::Value::Number((*n).into())),
            // Non-finite numbers (NaN / +Inf / -Inf) cannot be represented in
            // JSON. Treat them as `Undefined` so they are elided from arrays
            // and objects (matching Cosmos SQL's projection of an undefined
            // value), instead of silently coercing to `null` which would
            // collide with explicit `null` properties.
            Self::Number(n) => serde_json::Number::from_f64(*n).map(serde_json::Value::Number),
            Self::String(s) => Some(serde_json::Value::String(s.clone())),
            Self::Array(arr) => Some(serde_json::Value::Array(
                arr.iter().filter_map(|v| v.to_json_opt()).collect(),
            )),
            Self::Object(props) => {
                let map: serde_json::Map<String, serde_json::Value> = props
                    .iter()
                    .filter_map(|(k, v)| v.to_json_opt().map(|jv| (k.clone(), jv)))
                    .collect();
                Some(serde_json::Value::Object(map))
            }
        }
    }

    /// Check if this value is undefined.
    pub(crate) fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }
}

impl PartialEq for CosmosValue {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cosmos_eq(other), CosmosValue::Boolean(true))
    }
}

fn float_eq(a: f64, b: f64) -> bool {
    // IEEE 754 / Cosmos SQL semantics: NaN is not equal to anything,
    // including itself. Do not special-case NaN here.
    a == b
}

fn float_cmp(a: f64, b: f64) -> Option<Ordering> {
    a.partial_cmp(&b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_equals_null() {
        assert_eq!(
            CosmosValue::Null.cosmos_eq(&CosmosValue::Null),
            CosmosValue::Boolean(true)
        );
    }

    #[test]
    fn cross_type_is_undefined() {
        let result = CosmosValue::Number(42.0).cosmos_eq(&CosmosValue::String("42".into()));
        assert!(result.is_undefined());
    }

    #[test]
    fn undefined_eq_is_undefined() {
        let result = CosmosValue::Undefined.cosmos_eq(&CosmosValue::Undefined);
        assert!(result.is_undefined());
    }

    #[test]
    fn number_comparison() {
        assert_eq!(
            CosmosValue::Number(1.0).cosmos_cmp(&CosmosValue::Number(2.0)),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn string_comparison() {
        assert_eq!(
            CosmosValue::String("a".into()).cosmos_cmp(&CosmosValue::String("b".into())),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn from_json_roundtrip() {
        let json = serde_json::json!({"name": "Alice", "age": 30, "active": true});
        let cv = CosmosValue::from_json(&json);
        let back = cv.to_json();
        assert_eq!(json, back);
    }

    #[test]
    fn nan_is_not_equal_to_nan() {
        // Cosmos SQL semantics, IEEE 754, every other JSON stack: NaN != NaN.
        let nan = f64::NAN;
        assert!(!float_eq(nan, nan));
        assert_eq!(
            CosmosValue::Number(nan).cosmos_eq(&CosmosValue::Number(nan)),
            CosmosValue::Boolean(false)
        );
    }

    #[test]
    fn to_json_object_elides_undefined_properties() {
        let obj = CosmosValue::Object(vec![
            ("present".to_string(), CosmosValue::Integer(1)),
            ("missing".to_string(), CosmosValue::Undefined),
            ("explicit_null".to_string(), CosmosValue::Null),
        ]);
        let json = obj.to_json();
        let expected = serde_json::json!({
            "present": 1,
            "explicit_null": null,
        });
        assert_eq!(
            json, expected,
            "Undefined properties must be omitted; explicit Null preserved"
        );
    }

    #[test]
    fn to_json_array_elides_undefined_elements() {
        let arr = CosmosValue::Array(vec![
            CosmosValue::Integer(1),
            CosmosValue::Undefined,
            CosmosValue::Null,
            CosmosValue::Integer(2),
        ]);
        let json = arr.to_json();
        let expected = serde_json::json!([1, null, 2]);
        assert_eq!(json, expected, "Undefined elements omitted; Null preserved");
    }

    #[test]
    fn to_json_top_level_undefined_falls_back_to_null() {
        assert_eq!(CosmosValue::Undefined.to_json(), serde_json::Value::Null);
    }

    // (#3) Regression: non-finite f64 values used to coerce to `Value::Null`
    // in `to_json`, which silently collided with explicit `null` properties
    // and could be produced from `c.x / 0` or `c.x % 0`. They must instead be
    // elided from containers (matching how Cosmos SQL projects `Undefined`).
    #[test]
    fn to_json_object_elides_non_finite_number_properties() {
        let obj = CosmosValue::Object(vec![
            ("nan".to_string(), CosmosValue::Number(f64::NAN)),
            ("pos_inf".to_string(), CosmosValue::Number(f64::INFINITY)),
            (
                "neg_inf".to_string(),
                CosmosValue::Number(f64::NEG_INFINITY),
            ),
            ("explicit_null".to_string(), CosmosValue::Null),
            ("finite".to_string(), CosmosValue::Number(1.5)),
        ]);
        let json = obj.to_json();
        let expected = serde_json::json!({
            "explicit_null": null,
            "finite": 1.5,
        });
        assert_eq!(
            json, expected,
            "non-finite f64 properties must be elided like Undefined; \
             explicit Null preserved"
        );
    }

    #[test]
    fn to_json_top_level_non_finite_falls_back_to_null() {
        assert_eq!(
            CosmosValue::Number(f64::NAN).to_json(),
            serde_json::Value::Null
        );
        assert_eq!(
            CosmosValue::Number(f64::INFINITY).to_json(),
            serde_json::Value::Null
        );
    }
}
