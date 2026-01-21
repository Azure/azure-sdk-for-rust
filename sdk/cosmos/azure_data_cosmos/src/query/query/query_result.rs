// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use std::fmt::Debug;

use crate::ErrorKind;

/// Holds an owned list of items retrieved from the backend
#[derive(Serialize, Deserialize)]
pub(crate) struct FeedResponse<T> {
    #[serde(rename = "Documents")]
    pub documents: Vec<T>,
}

/// Helper struct for ORDER BY query results
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderByResult {
    order_by_items: Vec<QueryClauseItem>,
    payload: Box<serde_json::value::RawValue>,
}

/// Describes the expected shape of the query result.
///
/// The results the gateway gives us can vary in shape depending on the type of query executed.
/// However, to properly move through the pipeline, we want a normalized representation of the results.
/// This enum describes the expected shape, and provides deserialization logic to convert from the raw gateway response into a list of normalized [`QueryResult`]s.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryResultShape {
    /// The result will be just the raw payload, with no additional metadata.
    RawPayload,

    /// The payload is the result of a `ORDER BY` query.
    OrderBy,

    /// The result is from a `SELECT VALUE [aggregate function](...)` query.
    ValueAggregate,

    /// The result it is from a hybrid search component query.
    HybridComponent,
}

impl QueryResultShape {
    pub fn results_from_slice(self, buffer: &[u8]) -> crate::Result<Vec<QueryResult>> {
        match self {
            QueryResultShape::RawPayload => {
                let results: FeedResponse<Box<serde_json::value::RawValue>> =
                    serde_json::from_slice(buffer)
                        .map_err(|e| ErrorKind::InvalidGatewayResponse.with_source(e))?;
                Ok(results
                    .documents
                    .into_iter()
                    .map(QueryResult::RawPayload)
                    .collect())
            }
            QueryResultShape::OrderBy => {
                let results: FeedResponse<OrderByResult> = serde_json::from_slice(buffer)
                    .map_err(|e| ErrorKind::InvalidGatewayResponse.with_source(e))?;
                Ok(results
                    .documents
                    .into_iter()
                    .map(|item| QueryResult::OrderBy {
                        order_by_items: item.order_by_items,
                        payload: item.payload,
                    })
                    .collect())
            }
            QueryResultShape::ValueAggregate => {
                let results: FeedResponse<Vec<QueryClauseItem>> = serde_json::from_slice(buffer)
                    .map_err(|e| ErrorKind::InvalidGatewayResponse.with_source(e))?;
                Ok(results
                    .documents
                    .into_iter()
                    .map(QueryResult::ValueAggregates)
                    .collect())
            }
            QueryResultShape::HybridComponent => todo!(),
        }
    }
}

/// Represents the result of a rewritten query.
///
/// When we generate a query plan, the gateway rewrites the query so that it can be properly executed against each partition.
/// For example, order by items are collected into a well-known property with a well-known format so that the pipeline can easily access them.
#[derive(Clone, Debug)]
pub enum QueryResult {
    /// The result is just the raw payload, with no additional metadata.
    RawPayload(Box<serde_json::value::RawValue>),

    /// The result is a payload annotated with order by metadata,
    OrderBy {
        /// The items used for ordering the results.
        order_by_items: Vec<QueryClauseItem>,

        /// The actual payload of the query result.
        payload: Box<serde_json::value::RawValue>,
    },

    /// The result is from a `SELECT VALUE [aggregate function](...)` query against a single partition.
    ValueAggregates(Vec<QueryClauseItem>),
}

impl QueryResult {
    /// Expects the result to be of the `RawPayload` variant and unwraps it, returning an error if it is not.
    pub fn as_raw_payload(&self) -> Option<&serde_json::value::RawValue> {
        match self {
            QueryResult::RawPayload(payload) => Some(payload),
            _ => None,
        }
    }

    /// Expects the result to be of the `OrderBy` variant and unwraps it, returning an error if it is not.
    pub fn as_order_by(&self) -> Option<(&[QueryClauseItem], &serde_json::value::RawValue)> {
        match self {
            QueryResult::OrderBy {
                order_by_items,
                payload,
            } => Some((order_by_items, payload)),
            _ => None,
        }
    }

    /// Expects the result to be of the `ValueAggregates` variant and unwraps it, returning an error if it is not.
    pub fn as_value_aggregates(&self) -> Option<&[QueryClauseItem]> {
        match self {
            QueryResult::ValueAggregates(aggregates) => Some(aggregates),
            _ => None,
        }
    }

    /// Converts the `QueryResult` into its payload, if it has one.
    pub fn into_payload(self) -> Option<Box<serde_json::value::RawValue>> {
        match self {
            QueryResult::RawPayload(payload) => Some(payload),
            QueryResult::OrderBy { payload, .. } => Some(payload),
            QueryResult::ValueAggregates(_) => None,
        }
    }
}

impl Serialize for QueryResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            QueryResult::RawPayload(payload) => payload.serialize(serializer),
            QueryResult::OrderBy {
                order_by_items,
                payload,
            } => {
                let mut state = serializer.serialize_struct("OrderByItem", 2)?;
                state.serialize_field("orderByItems", order_by_items)?;
                state.serialize_field("payload", payload)?;
                state.end()
            }
            QueryResult::ValueAggregates(aggregates) => aggregates.serialize(serializer),
        }
    }
}

/// Many of the gateway-rewritten queries cause the backend to produce `{"item": <value>}` objects for order by and group by items.
/// This struct represents that shape, and provides comparison logic for ordering.
#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct QueryClauseItem {
    #[serde(default, deserialize_with = "deserialize_item")]
    pub item: Option<serde_json::Value>,

    /// The backend sometimes returns an alternate form of the item, such as a min/max value with added metadata about the number of items in the partition.
    #[serde(
        default,
        deserialize_with = "deserialize_item",
        skip_serializing_if = "Option::is_none"
    )]
    pub item2: Option<serde_json::Value>,
}

// Based on https://github.com/serde-rs/serde/issues/984#issuecomment-314143738
// This will deserialize a missing field to `None`, a present-but-null field to `Some(serde_json::Value::Null)` and a present-non-null field to `Some(value)`.
fn deserialize_item<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

impl QueryClauseItem {
    /// Creates a new [`QueryClauseItem`] from a [`serde_json::Value`].
    pub fn from_value(value: serde_json::Value) -> Self {
        Self {
            item: Some(value),
            item2: None,
        }
    }

    /// Creates a new [`QueryClauseItem`] from a [`serde_json::Value`].
    pub fn from_values(item: serde_json::Value, item2: serde_json::Value) -> Self {
        Self {
            item: Some(item),
            item2: Some(item2),
        }
    }

    /// Compares two [`QueryClauseItem`]s based on the ordering rules defined for Cosmos DB.
    ///
    /// We can't just implement [`PartialOrd`] here, because we need to be able to return an error.
    pub fn compare(&self, other: &Self) -> crate::Result<std::cmp::Ordering> {
        let left_ordinal = self.type_ordinal()?;
        let right_ordinal = other.type_ordinal()?;

        if left_ordinal != right_ordinal {
            return Ok(left_ordinal.cmp(&right_ordinal));
        }

        match (&self.item, &other.item) {
            (None, None) => Ok(std::cmp::Ordering::Equal),
            (Some(serde_json::Value::Null), Some(serde_json::Value::Null)) => {
                Ok(std::cmp::Ordering::Equal)
            }
            (Some(serde_json::Value::String(left)), Some(serde_json::Value::String(right))) => {
                Ok(left.cmp(right))
            }
            (Some(serde_json::Value::Bool(left)), Some(serde_json::Value::Bool(right))) => {
                Ok(left.cmp(right))
            }
            (Some(serde_json::Value::Number(left)), Some(serde_json::Value::Number(right))) => {
                // Try integer comparison first. This will fail if either value is not an integer.
                if let (Some(l_int), Some(r_int)) = (left.as_i64(), right.as_i64()) {
                    Ok(l_int.cmp(&r_int))
                } else {
                    // We need to compare as floats.
                    let l = left.as_f64().ok_or_else(|| {
                        ErrorKind::InvalidGatewayResponse.with_message("encountered NaN or Infinity while comparing floats")
                    })?;
                    let r = right.as_f64().ok_or_else(|| {
                        ErrorKind::InvalidGatewayResponse.with_message("encountered NaN or Infinity while comparing floats")
                    })?;
                    l.partial_cmp(&r).ok_or_else(|| {
                        ErrorKind::InvalidGatewayResponse.with_message("encountered NaN or Infinity while comparing floats")
                    })
                }
            }

            // Shouldn't be possible to get here, since we've already checked the type ordinal.
            _ => unreachable!("encountered different types after comparing type ordinal, this shouldn't be possible")
        }
    }

    /// Gets the "Type Ordinal" for a given item.
    ///
    /// The Type Ordinal is used to order items of differing types.
    /// If the Type Ordinal is the same, the items are compared using their underlying values.
    ///
    /// Returns an error if a non-primitive value is encountered.
    fn type_ordinal(&self) -> crate::Result<usize> {
        match &self.item {
            None => Ok(0),
            Some(serde_json::Value::Null) => Ok(1),
            Some(serde_json::Value::Bool(_)) => Ok(2),
            // 3 is skipped in the current implementation for both Python and JS.
            Some(serde_json::Value::Number(_)) => Ok(4),
            Some(serde_json::Value::String(_)) => Ok(5),
            _ => Err(ErrorKind::InvalidGatewayResponse
                .with_message("cannot compare non-primitive values")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    fn json_to_query_result(shape: QueryResultShape, json: &str) -> QueryResult {
        let result: Vec<QueryResult> = shape.results_from_slice(json.as_bytes()).unwrap();
        assert_eq!(1, result.len());
        result.into_iter().next().unwrap()
    }

    #[test]
    pub fn query_result_deserializes_raw_payload_shape() {
        const JSON: &str = r#"{"Documents":[{"a":1}]}"#;
        let result: QueryResult = json_to_query_result(QueryResultShape::RawPayload, JSON);
        match result {
            QueryResult::RawPayload(payload) => {
                assert_eq!(payload.get(), r#"{"a":1}"#);
            }
            _ => panic!("expected RawPayload variant"),
        }
    }

    #[test]
    pub fn query_result_deserializes_order_by_shape() {
        const JSON: &str = r#"{"Documents":[{"orderByItems":[{"item":1}], "payload": {"a":1}}]}"#;
        let result: QueryResult = json_to_query_result(QueryResultShape::OrderBy, JSON);
        match result {
            QueryResult::OrderBy {
                order_by_items,
                payload,
            } => {
                assert_eq!(
                    order_by_items,
                    vec![QueryClauseItem::from_value(serde_json::json!(1))]
                );
                assert_eq!(payload.get(), r#"{"a":1}"#);
            }
            _ => panic!("expected OrderBy variant"),
        }
    }

    #[test]
    pub fn query_result_deserializes_value_aggregate_shape() {
        const JSON: &str = r#"{"Documents":[[{"item":42}]]}"#;
        let result: QueryResult = json_to_query_result(QueryResultShape::ValueAggregate, JSON);
        match result {
            QueryResult::ValueAggregates(aggregates) => {
                assert_eq!(
                    aggregates,
                    vec![QueryClauseItem::from_value(serde_json::json!(42))]
                );
            }
            _ => panic!("expected ValueAggregates variant"),
        }
    }

    #[test]
    pub fn query_result_serializes_raw_payload() {
        let payload = serde_json::value::RawValue::from_string(r#"{"a":1}"#.to_string()).unwrap();
        let result = QueryResult::RawPayload(payload);
        let serialized = serde_json::to_string(&result).unwrap();
        assert_eq!(serialized, r#"{"a":1}"#);
    }

    #[test]
    pub fn query_result_serializes_order_by() {
        let payload = serde_json::value::RawValue::from_string(r#"{"a":1}"#.to_string()).unwrap();
        let result = QueryResult::OrderBy {
            order_by_items: vec![QueryClauseItem::from_value(serde_json::json!(1))],
            payload,
        };
        let serialized = serde_json::to_string(&result).unwrap();
        assert_eq!(
            serialized,
            r#"{"orderByItems":[{"item":1}],"payload":{"a":1}}"#
        );
    }

    #[test]
    pub fn query_result_serializes_value_aggregates() {
        let result =
            QueryResult::ValueAggregates(vec![QueryClauseItem::from_value(serde_json::json!(42))]);
        let serialized = serde_json::to_string(&result).unwrap();
        assert_eq!(serialized, r#"[{"item":42}]"#);
    }

    macro_rules! ordering_tests {
        (
            $(
                $name:ident {
                    $($left:tt, $right:tt => $expected:pat,)*
                }
            )+
        ) => {
            $(
                #[test]
                #[allow(clippy::redundant_pattern_matching)] // Clippy doesn't like that sometimes we match on Err(_) instead of calling .is_err
                pub fn $name() {
                    $(
                        let left = serde_json::json!($left);
                        let right = serde_json::json!($right);
                        let left: QueryClauseItem = serde_json::from_value(left).unwrap();
                        let right: QueryClauseItem  = serde_json::from_value(right).unwrap();
                        let result = left.compare(&right);

                        assert!(matches!(result, $expected), "comparing {:?} and {:?}, expected: {}, but got {:?}", left, right, stringify!($expected), result);
                    )*
                }
            )+
        };
    }

    ordering_tests! {
        compare_numbers {
            {"item": 1}, {"item": 1} => Ok(Ordering::Equal),
            {"item": 1}, {"item": 2} => Ok(Ordering::Less),
            {"item": 2}, {"item": 1} => Ok(Ordering::Greater),
            {"item": 1.0}, {"item": 1.0} => Ok(Ordering::Equal),
            {"item": 1.0}, {"item": 1.1} => Ok(Ordering::Less),
            {"item": 1.1}, {"item": 1.0} => Ok(Ordering::Greater),
            {"item": -1}, {"item": -1} => Ok(Ordering::Equal),
            {"item": -1}, {"item": 1} => Ok(Ordering::Less),
            {"item": 1}, {"item": -1} => Ok(Ordering::Greater),
        }

        compare_bools {
            {"item": true}, {"item": false} => Ok(Ordering::Greater),
            {"item": false}, {"item": true} => Ok(Ordering::Less),
            {"item": true}, {"item": true} => Ok(Ordering::Equal),
            {"item": false}, {"item": false} => Ok(Ordering::Equal),
        }

        compare_strings {
            {"item": "aaa"}, {"item": "aab"} => Ok(Ordering::Less),
            {"item": "aab"}, {"item": "aaa"} => Ok(Ordering::Greater),
            {"item": "aaa"}, {"item": "aaa"} => Ok(Ordering::Equal),
        }

        compare_nulls_and_undefined {
            {}, {} => Ok(Ordering::Equal),
            {"item": null}, {"item": null} => Ok(Ordering::Equal),
            {}, {"item": null} => Ok(Ordering::Less),
        }

        compare_mixed_types {
            {}, {"item": null} => Ok(Ordering::Less),
            {"item": null}, {"item": true} => Ok(Ordering::Less),
            {"item": true}, {"item": 1} => Ok(Ordering::Less),
            {"item": 1}, {"item": "a"} => Ok(Ordering::Less),
        }

        cannot_compare_non_primitives {
            {"item": {"a": 1}}, {"item": {"a": 2}} => Err(_),
            {"item": [1, 2]}, {"item": [3, 4]} => Err(_),
            {"item": {"a": 1}}, {} => Err(_),
        }
    }
}
