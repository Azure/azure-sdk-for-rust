// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Single-group reduction of rewritten aggregate projection payloads.

use super::{
    order_by::{compare_json_values, OrderByNumber},
    query_plan::QueryInfo,
};
use crate::error::{status_codes, CosmosError, Result};
use serde_json::{Map, Number, Value};
use std::{cmp::Ordering, collections::BTreeMap};

/// Reduces partition partials; `None` represents Cosmos undefined, not JSON null.
pub(crate) struct SingleGroupAggregator {
    projection: Projection,
}

enum Projection {
    Value(Accumulator),
    List {
        slots: BTreeMap<String, Accumulator>,
        aliases: Vec<String>,
    },
}

enum Accumulator {
    Count(i64),
    Sum(Option<f64>),
    Average {
        sum: Option<f64>,
        count: i64,
    },
    MinMax {
        minimum: bool,
        value: Extremum,
    },
    Scalar {
        initialized: bool,
        value: Option<Value>,
    },
}

enum Extremum {
    Empty,
    Undefined,
    Value(Value),
}

impl SingleGroupAggregator {
    /// Creates the slots described by a query plan.
    pub(crate) fn new(info: &QueryInfo) -> Result<Self> {
        let projection = if info.has_select_value {
            Projection::Value(Accumulator::new(
                info.aggregates.first().map(String::as_str),
            )?)
        } else {
            let mut aliases = info.group_by_aliases.clone();
            if aliases.is_empty() {
                aliases.extend(info.group_by_alias_to_aggregate_type.keys().cloned());
                aliases.sort_unstable();
            }
            let mut slots = BTreeMap::new();
            for (alias, operator) in &info.group_by_alias_to_aggregate_type {
                let operator = match operator {
                    Value::Null => None,
                    Value::String(operator) if operator.is_empty() => None,
                    Value::String(operator) => Some(operator.as_str()),
                    _ => return Err(invalid("aggregate operator must be a string or null")),
                };
                slots.insert(alias.clone(), Accumulator::new(operator)?);
            }
            for alias in &aliases {
                slots
                    .entry(alias.clone())
                    .or_insert(Accumulator::new(None)?);
            }
            Projection::List { slots, aliases }
        };
        Ok(Self { projection })
    }

    /// Adds one rewritten projection, unwrapping aggregate cells but not scalars.
    pub(crate) fn add(&mut self, payload: Option<&Value>) -> Result<()> {
        match &mut self.projection {
            Projection::Value(slot) => slot.add(payload),
            Projection::List { slots, .. } => {
                let payload = payload
                    .and_then(Value::as_object)
                    .ok_or_else(|| invalid("SELECT-list aggregate payload must be an object"))?;
                for (alias, slot) in slots {
                    slot.add(payload.get(alias))?;
                }
                Ok(())
            }
        }
    }

    /// Returns the final value, omitting undefined SELECT-list aliases.
    pub(crate) fn result(&self) -> Result<Option<Value>> {
        match &self.projection {
            Projection::Value(slot) => slot.result(),
            Projection::List { slots, aliases } => {
                let mut result = Map::with_capacity(aliases.len());
                for alias in aliases {
                    let slot = slots
                        .get(alias)
                        .ok_or_else(|| invalid("aggregate output alias has no slot"))?;
                    if let Some(value) = slot.result()? {
                        result.insert(alias.clone(), value);
                    }
                }
                Ok(Some(Value::Object(result)))
            }
        }
    }
}

impl Accumulator {
    fn new(operator: Option<&str>) -> Result<Self> {
        Ok(match operator {
            None => Self::Scalar {
                initialized: false,
                value: None,
            },
            Some("Count") => Self::Count(0),
            Some("Sum") => Self::Sum(Some(0.0)),
            Some("Average") => Self::Average {
                sum: Some(0.0),
                count: 0,
            },
            Some("Min") => Self::MinMax {
                minimum: true,
                value: Extremum::Empty,
            },
            Some("Max") => Self::MinMax {
                minimum: false,
                value: Extremum::Empty,
            },
            Some("CountIf" | "MakeList" | "MakeSet") => {
                return Err(unsupported("aggregate operator is not supported"))
            }
            Some(_) => return Err(unsupported("unknown aggregate operator")),
        })
    }

    fn add(&mut self, payload: Option<&Value>) -> Result<()> {
        if let Self::Scalar { initialized, value } = self {
            if !*initialized {
                *value = payload.cloned();
                *initialized = true;
            }
            return Ok(());
        }
        let partial = aggregate_item(payload)?;
        match self {
            Self::Count(total) => {
                *total = total
                    .checked_add(count(partial)?)
                    .ok_or_else(|| invalid("COUNT overflow"))?;
            }
            Self::Sum(total) => {
                if let Some(sum) = total {
                    *total = match partial {
                        None => None,
                        Some(value) => Some(checked_sum(*sum, numeric(Some(value))?)?),
                    };
                }
            }
            Self::Average { sum, count: total } => {
                let partial = partial
                    .and_then(Value::as_object)
                    .ok_or_else(|| invalid("AVG partial must be an object"))?;
                let partial_count = count(partial.get("count"))?;
                let partial_sum = partial.get("sum").map(|v| numeric(Some(v))).transpose()?;
                let new_count = total
                    .checked_add(partial_count)
                    .ok_or_else(|| invalid("AVG count overflow"))?;
                let new_sum = match (*sum, partial_sum) {
                    (Some(a), Some(b)) => Some(checked_sum(a, b)?),
                    _ => None,
                };
                *sum = new_sum;
                *total = new_count;
            }
            Self::MinMax { minimum, value } => {
                if matches!(value, Extremum::Undefined) {
                    return Ok(());
                }
                let mut partial = partial;
                if let Some(object) = partial.and_then(Value::as_object) {
                    if let Some(local_count) = object.get("count") {
                        if count(Some(local_count))? == 0 {
                            return Ok(());
                        }
                        // The .NET partial envelope prefers min even for MAX.
                        partial = object.get("min").or_else(|| object.get("max"));
                    }
                }
                let Some(partial) = partial else {
                    *value = Extremum::Undefined;
                    return Ok(());
                };
                match value {
                    Extremum::Empty => *value = Extremum::Value(partial.clone()),
                    Extremum::Value(current) => {
                        // .NET accepts a first non-primitive but poisons on any comparison.
                        if !primitive(partial) || !primitive(current) {
                            *value = Extremum::Undefined;
                        } else {
                            let order = compare_json_values(partial, current);
                            if order
                                == if *minimum {
                                    Ordering::Less
                                } else {
                                    Ordering::Greater
                                }
                            {
                                *current = partial.clone();
                            }
                        }
                    }
                    Extremum::Undefined => {}
                }
            }
            Self::Scalar { .. } => unreachable!(),
        }
        Ok(())
    }

    fn result(&self) -> Result<Option<Value>> {
        match self {
            Self::Count(total) => Ok(Some(Value::from(*total))),
            Self::Sum(sum) => sum.map(number).transpose(),
            Self::Average { sum, count } => {
                if *count <= 0 {
                    Ok(None)
                } else {
                    sum.map(|sum| number(sum / *count as f64)).transpose()
                }
            }
            Self::MinMax { value, .. } => Ok(match value {
                Extremum::Empty | Extremum::Undefined => None,
                Extremum::Value(value) => Some(value.clone()),
            }),
            Self::Scalar {
                initialized: true,
                value,
            } => Ok(value.clone()),
            Self::Scalar {
                initialized: false, ..
            } => Err(invalid("scalar aggregate slot has not received a value")),
        }
    }
}

fn aggregate_item(payload: Option<&Value>) -> Result<Option<&Value>> {
    let Some(payload) = payload else {
        return Ok(None);
    };
    let object = match payload {
        Value::Object(object) => Some(object),
        Value::Array(values) if values.len() == 1 => values[0].as_object(),
        _ => None,
    }
    .ok_or_else(|| invalid("aggregate cell must be an object or singleton object array"))?;
    Ok(object.get("item2").or_else(|| object.get("item")))
}

fn numeric(value: Option<&Value>) -> Result<f64> {
    value
        .and_then(Value::as_f64)
        .filter(|value| value.is_finite())
        .ok_or_else(|| invalid("aggregate partial must be a finite number"))
}

fn count(value: Option<&Value>) -> Result<i64> {
    let value = value
        .and_then(Value::as_number)
        .ok_or_else(|| invalid("aggregate count must be a number"))?;
    match OrderByNumber::from_json_number(value) {
        OrderByNumber::I64(value) => Ok(value),
        OrderByNumber::U64(value) => {
            i64::try_from(value).map_err(|_| invalid("aggregate count exceeds i64"))
        }
        OrderByNumber::F64(value) => {
            // Number64.ToLong truncates doubles; range-check before Rust's saturating cast.
            const LIMIT: f64 = 9_223_372_036_854_775_808.0;
            if value.is_finite() && (-LIMIT..LIMIT).contains(&value) {
                Ok(value as i64)
            } else {
                Err(invalid("aggregate count exceeds i64"))
            }
        }
    }
}

fn checked_sum(a: f64, b: f64) -> Result<f64> {
    let sum = a + b;
    if sum.is_finite() {
        Ok(sum)
    } else {
        Err(invalid("aggregate sum is non-finite"))
    }
}

fn number(value: f64) -> Result<Value> {
    // Keep integral results consumable as integers in both text and binary responses.
    const I64_LIMIT: f64 = 9_223_372_036_854_775_808.0;
    const U64_LIMIT: f64 = 18_446_744_073_709_551_616.0;
    if value.fract() == 0.0 {
        if (-I64_LIMIT..I64_LIMIT).contains(&value) {
            return Ok(Value::from(value as i64));
        }
        if (0.0..U64_LIMIT).contains(&value) {
            return Ok(Value::from(value as u64));
        }
    }
    Number::from_f64(value)
        .map(Value::Number)
        .ok_or_else(|| invalid("aggregate result is non-finite"))
}

fn primitive(value: &Value) -> bool {
    !matches!(value, Value::Array(_) | Value::Object(_))
}

fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
        .with_message(message)
        .build()
}

fn unsupported(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(status_codes::CROSS_PARTITION_QUERY_NOT_SERVABLE)
        .with_message(message)
        .build()
}

#[cfg(test)]
mod tests {
    use super::{status_codes, QueryInfo, SingleGroupAggregator, Value};
    use serde_json::json;

    fn aggregator(operator: &str) -> SingleGroupAggregator {
        SingleGroupAggregator::new(&QueryInfo {
            has_select_value: true,
            aggregates: vec![operator.into()],
            ..Default::default()
        })
        .unwrap()
    }

    fn reduce(operator: &str, values: &[Value]) -> Option<Value> {
        let mut aggregator = aggregator(operator);
        for value in values {
            aggregator.add(Some(value)).unwrap();
        }
        aggregator.result().unwrap()
    }

    #[test]
    fn empty_aggregate_results() {
        for (operator, expected) in [
            ("Count", Some(json!(0))),
            ("Sum", Some(json!(0))),
            ("Average", None),
            ("Min", None),
            ("Max", None),
        ] {
            assert_eq!(reduce(operator, &[]), expected, "{operator}");
        }
    }

    #[test]
    fn count_sums_partial_counts_instead_of_counting_rows() {
        assert_eq!(
            reduce("Count", &[json!({"item": 17}), json!({"item": 28})]),
            Some(json!(45))
        );
        assert_eq!(reduce("Count", &[json!({"item": 2.9})]), Some(json!(2)));
    }

    #[test]
    fn sum_adds_and_undefined_is_sticky() {
        assert_eq!(
            reduce("Sum", &[json!({"item": 2}), json!({"item": -0.5})]),
            Some(json!(1.5))
        );
        assert_eq!(
            reduce("Sum", &[json!({"item": 2}), json!({}), json!({"item": 3})]),
            None
        );
        let mut sum = aggregator("Sum");
        sum.add(None).unwrap();
        sum.add(Some(&json!({"item": null}))).unwrap();
        assert_eq!(sum.result().unwrap(), None);
    }

    #[test]
    fn average_weights_by_partition_counts_and_omitted_sum_poisons() {
        assert_eq!(
            reduce(
                "Average",
                &[
                    json!({"item": {"sum": 10, "count": 1}}),
                    json!({"item": {"sum": 90, "count": 9}}),
                ]
            ),
            Some(json!(10))
        );
        assert_eq!(
            reduce(
                "Average",
                &[
                    json!({"item": {"sum": 0, "count": 0}}),
                    json!({"item": {"sum": 6, "count": 2}}),
                ]
            ),
            Some(json!(3))
        );
        assert_eq!(
            reduce(
                "Average",
                &[
                    json!({"item": {"count": 0}}),
                    json!({"item": {"sum": 6, "count": 2}}),
                ]
            ),
            None
        );
        assert_eq!(
            reduce("Average", &[json!({"item": {"sum": 0, "count": 0}})]),
            None
        );
    }

    #[test]
    fn extrema_order_mixed_primitives_and_preserve_null() {
        let values = [
            json!({"item": "z"}),
            json!({"item": 10}),
            json!({"item": false}),
            json!({"item": null}),
        ];
        assert_eq!(reduce("Min", &values), Some(Value::Null));
        assert_eq!(reduce("Max", &values), Some(json!("z")));
        assert_eq!(
            reduce("Min", &[json!({"item": 2}), json!({}), json!({"item": 1})]),
            None
        );
        assert_eq!(
            reduce("Max", &[json!({"item": 2}), json!({}), json!({"item": 3})]),
            None
        );
    }

    #[test]
    fn extrema_partial_counts_and_min_precedence_match_dotnet() {
        for operator in ["Min", "Max"] {
            assert_eq!(
                reduce(operator, &[json!({"item": {"count": 0, "min": null}})]),
                None
            );
            assert_eq!(
                reduce(
                    operator,
                    &[
                        json!({"item": {"count": 0}}),
                        json!({"item": {"count": 3, "min": 2, "max": 7}}),
                    ]
                ),
                Some(json!(2))
            );
            assert_eq!(
                reduce(operator, &[json!({"item": {"count": 1, "max": 7}})]),
                Some(json!(7))
            );
            assert_eq!(
                reduce(
                    operator,
                    &[json!({"item": {"count": 1}}), json!({"item": 7})]
                ),
                None
            );
        }
    }

    #[test]
    fn extrema_non_primitive_first_value_and_subsequent_poison() {
        for operator in ["Min", "Max"] {
            for value in [json!([1, 2]), json!({"x": 1})] {
                assert_eq!(
                    reduce(operator, &[json!({"item": value.clone()})]),
                    Some(value.clone())
                );
                assert_eq!(
                    reduce(operator, &[json!({"item": value}), json!({"item": 1})]),
                    None
                );
                assert_eq!(
                    reduce(operator, &[json!({"item": 1}), json!({"item": [1, 2]})]),
                    None
                );
            }
        }
    }

    #[test]
    fn aggregate_item2_precedence_and_singleton_array() {
        assert_eq!(
            reduce(
                "Count",
                &[json!({"item": 999, "item2": 3}), json!([{"item": 5}])]
            ),
            Some(json!(8))
        );
        assert_eq!(
            reduce("Min", &[json!({"item": 1, "item2": null})]),
            Some(Value::Null)
        );
        assert!(aggregator("Count")
            .add(Some(&json!({"item": 1, "item2": null})))
            .is_err());
    }

    #[test]
    fn select_list_multiple_aggregates_and_raw_scalar_slots() {
        let info: QueryInfo = serde_json::from_value(json!({
            "groupByAliases": ["count", "sum", "avg", "min", "max", "key", "empty", "missing"],
            "groupByAliasToAggregateType": {
                "count": "Count", "sum": "Sum", "avg": "Average",
                "min": "Min", "max": "Max", "key": null, "empty": "", "missing": null
            }
        }))
        .unwrap();
        let mut aggregator = SingleGroupAggregator::new(&info).unwrap();
        aggregator
            .add(Some(&json!({
                "count": {"item": 2}, "sum": {"item": 8},
                "avg": {"item": {"sum": 8, "count": 2}},
                "min": {"item": 2}, "max": {"item": 6},
                "key": {"item": "raw"}, "empty": null
            })))
            .unwrap();
        aggregator
            .add(Some(&json!({
                "count": {"item": 1}, "sum": {},
                "avg": {"item": {"sum": 10, "count": 1}},
                "min": {"item": 10}, "max": {"item": 10},
                "key": "ignored", "empty": "ignored", "missing": "ignored"
            })))
            .unwrap();
        assert_eq!(
            aggregator.result().unwrap(),
            Some(json!({
                "count": 3, "avg": 6, "min": 2, "max": 10,
                "key": {"item": "raw"}, "empty": null
            }))
        );
    }

    #[test]
    fn alias_map_only_projection_and_missing_map_scalar() {
        let info: QueryInfo = serde_json::from_value(json!({
            "groupByAliasToAggregateType": {"n": "Count", "key": null}
        }))
        .unwrap();
        let mut aggregator = SingleGroupAggregator::new(&info).unwrap();
        aggregator
            .add(Some(&json!({"n": {"item": 4}, "key": "a"})))
            .unwrap();
        assert_eq!(
            aggregator.result().unwrap(),
            Some(json!({"n": 4, "key": "a"}))
        );
        let mut scalar = SingleGroupAggregator::new(&QueryInfo {
            group_by_aliases: vec!["key".into()],
            ..Default::default()
        })
        .unwrap();
        assert!(scalar.result().is_err());
        scalar.add(Some(&json!({"key": [1, 2]}))).unwrap();
        assert_eq!(scalar.result().unwrap(), Some(json!({"key": [1, 2]})));
    }

    #[test]
    fn alias_map_slots_are_all_validated_but_only_ordered_aliases_are_emitted() {
        let mut info: QueryInfo = serde_json::from_value(json!({
            "groupByAliases": ["n"],
            "groupByAliasToAggregateType": {"n": "Count", "hidden": "Sum"}
        }))
        .unwrap();
        let mut aggregator = SingleGroupAggregator::new(&info).unwrap();
        aggregator
            .add(Some(&json!({"n": {"item": 3}, "hidden": {"item": 8}})))
            .unwrap();
        assert_eq!(aggregator.result().unwrap(), Some(json!({"n": 3})));
        info.group_by_alias_to_aggregate_type
            .insert("hidden".into(), json!("MakeList"));
        assert!(SingleGroupAggregator::new(&info).is_err());
    }

    #[test]
    fn value_scalar_retains_first_value_including_undefined() {
        let info = QueryInfo {
            has_select_value: true,
            ..Default::default()
        };
        let mut scalar = SingleGroupAggregator::new(&info).unwrap();
        assert!(scalar.result().is_err());
        scalar.add(None).unwrap();
        scalar.add(Some(&json!(1))).unwrap();
        assert_eq!(scalar.result().unwrap(), None);
        let mut scalar = SingleGroupAggregator::new(&info).unwrap();
        scalar.add(Some(&Value::Null)).unwrap();
        assert_eq!(scalar.result().unwrap(), Some(Value::Null));
    }

    #[test]
    fn extrema_compare_large_signed_unsigned_and_float_without_precision_loss() {
        let values = [
            json!({"item": 9_007_199_254_740_993_u64}),
            json!({"item": 9_007_199_254_740_992.0}),
        ];
        assert_eq!(reduce("Min", &values), Some(json!(9_007_199_254_740_992.0)));
        assert_eq!(
            reduce("Max", &values),
            Some(json!(9_007_199_254_740_993_u64))
        );
        let values = [
            json!({"item": u64::MAX}),
            json!({"item": i64::MAX}),
            json!({"item": i64::MIN}),
        ];
        assert_eq!(reduce("Min", &values), Some(json!(i64::MIN)));
        assert_eq!(reduce("Max", &values), Some(json!(u64::MAX)));
    }

    #[test]
    fn overflow_and_out_of_range_counts_are_typed_errors() {
        let mut count = aggregator("Count");
        count.add(Some(&json!({"item": i64::MAX}))).unwrap();
        assert!(count.add(Some(&json!({"item": 1}))).is_err());
        for value in [json!(u64::MAX), json!(9_223_372_036_854_775_808.0)] {
            assert!(aggregator("Count")
                .add(Some(&json!({"item": value})))
                .is_err());
        }
        for operator in ["Sum", "Average"] {
            let mut aggregator = aggregator(operator);
            let partial = if operator == "Sum" {
                json!({"item": f64::MAX})
            } else {
                json!({"item": {"sum": f64::MAX, "count": 1}})
            };
            aggregator.add(Some(&partial)).unwrap();
            assert!(aggregator.add(Some(&partial)).is_err());
        }
        let mut avg = aggregator("Average");
        avg.add(Some(&json!({"item": {"sum": 0, "count": i64::MAX}})))
            .unwrap();
        assert!(avg
            .add(Some(&json!({"item": {"sum": 0, "count": 1}})))
            .is_err());
    }

    #[test]
    fn sum_and_average_materialize_integral_results_without_saturating_casts() {
        let cases = [
            (json!(-0.0), json!(0)),
            (json!(12.0), json!(12)),
            (json!(-9_223_372_036_854_775_808.0), json!(i64::MIN)),
            (
                json!(9_223_372_036_854_775_808.0),
                json!(9_223_372_036_854_775_808_u64),
            ),
            (
                json!(18_446_744_073_709_549_568.0),
                json!(18_446_744_073_709_549_568_u64),
            ),
            (
                json!(18_446_744_073_709_551_616.0),
                json!(18_446_744_073_709_551_616.0),
            ),
            (json!(1.5), json!(1.5)),
        ];
        for (partial, expected) in cases {
            for operator in ["Sum", "Average"] {
                let cell = if operator == "Sum" {
                    json!({"item": partial})
                } else {
                    json!({"item": {"sum": partial, "count": 1}})
                };
                assert_eq!(
                    reduce(operator, &[cell]),
                    Some(expected.clone()),
                    "{operator}"
                );
            }
        }
        let value = reduce("Sum", &[json!({"item": 12.0})]).unwrap();
        assert_eq!(serde_json::from_value::<i64>(value).unwrap(), 12);
    }

    #[test]
    fn malformed_cells_and_nonnumeric_partials_are_rejected() {
        for operator in ["Count", "Sum", "Average", "Min", "Max"] {
            for payload in [
                Value::Null,
                json!(3),
                json!([]),
                json!([{}, {}]),
                json!([null]),
            ] {
                assert!(
                    aggregator(operator).add(Some(&payload)).is_err(),
                    "{operator}"
                );
            }
        }
        for operator in ["Count", "Sum"] {
            for value in [Value::Null, json!(true), json!("1"), json!([]), json!({})] {
                let error = aggregator(operator)
                    .add(Some(&json!({"item": value})))
                    .unwrap_err();
                assert_eq!(
                    error.status(),
                    status_codes::SERIALIZATION_RESPONSE_BODY_INVALID
                );
            }
        }
        for partial in [
            json!({}),
            json!({"sum": 1}),
            json!({"count": null}),
            json!({"sum": null, "count": 1}),
            json!({"sum": "1", "count": 1}),
            json!({"sum": 1, "count": "1"}),
            json!(3),
        ] {
            assert!(aggregator("Average")
                .add(Some(&json!({"item": partial})))
                .is_err());
        }
        for operator in ["Min", "Max"] {
            assert!(aggregator(operator)
                .add(Some(&json!({"item": {"count": "1", "min": 2}})))
                .is_err());
        }
    }

    #[test]
    fn unsupported_operators_and_invalid_alias_plans_are_rejected() {
        for operator in ["CountIf", "MakeList", "MakeSet", "Unknown"] {
            let error = SingleGroupAggregator::new(&QueryInfo {
                has_select_value: true,
                aggregates: vec![operator.into()],
                ..Default::default()
            })
            .err()
            .unwrap();
            assert_eq!(
                error.status(),
                status_codes::CROSS_PARTITION_QUERY_NOT_SERVABLE
            );
        }
        let info: QueryInfo = serde_json::from_value(json!({
            "groupByAliasToAggregateType": {"bad": 1}
        }))
        .unwrap();
        assert!(SingleGroupAggregator::new(&info).is_err());
        let mut aggregator = SingleGroupAggregator::new(&QueryInfo {
            group_by_aliases: vec!["x".into()],
            ..Default::default()
        })
        .unwrap();
        assert!(aggregator.add(None).is_err());
        assert!(aggregator.add(Some(&json!([]))).is_err());
    }
}
