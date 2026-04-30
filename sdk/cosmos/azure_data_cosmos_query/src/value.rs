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

    /// Returns true if this value is "truthy" in Cosmos DB semantics.
    /// `null`, `undefined`, `false`, `0`, and `""` are falsy; everything else is truthy.
    /// However, Cosmos DB WHERE clauses only accept boolean true as "matches".
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Null | Self::Undefined => false,
            Self::Number(n) => *n != 0.0,
            Self::Integer(n) => *n != 0,
            Self::String(s) => !s.is_empty(),
            Self::Array(_) | Self::Object(_) => true,
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
    pub(crate) fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Null | Self::Undefined => serde_json::Value::Null,
            Self::Boolean(b) => serde_json::Value::Bool(*b),
            Self::Integer(n) => serde_json::Value::Number((*n).into()),
            Self::Number(n) => serde_json::Number::from_f64(*n)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Self::String(s) => serde_json::Value::String(s.clone()),
            Self::Array(arr) => serde_json::Value::Array(arr.iter().map(|v| v.to_json()).collect()),
            Self::Object(props) => {
                let map: serde_json::Map<String, serde_json::Value> = props
                    .iter()
                    .map(|(k, v)| (k.clone(), v.to_json()))
                    .collect();
                serde_json::Value::Object(map)
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
    if a.is_nan() && b.is_nan() {
        return true;
    }
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
    fn truthy_values() {
        assert!(CosmosValue::Boolean(true).is_truthy());
        assert!(!CosmosValue::Boolean(false).is_truthy());
        assert!(!CosmosValue::Null.is_truthy());
        assert!(!CosmosValue::Undefined.is_truthy());
        assert!(CosmosValue::Number(1.0).is_truthy());
        assert!(!CosmosValue::Number(0.0).is_truthy());
    }
}
