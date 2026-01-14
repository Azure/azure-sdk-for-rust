// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::{
    ops::{Add, AddAssign},
    str::FromStr,
};

use serde::Deserialize;

use crate::{query::QueryClauseItem, ErrorKind};

/// Helper type to try and keep numeric types as integers until necessary
#[derive(Debug, Clone, Copy)]
pub enum Sum {
    Empty,
    Int(i64),
    Float(f64),
}

impl Add for Sum {
    type Output = Sum;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Sum::Int(a), Sum::Int(b)) => Sum::Int(a.wrapping_add(b)),
            (Sum::Int(a), Sum::Float(b)) => Sum::Float((a as f64) + b),
            (Sum::Float(a), Sum::Int(b)) => Sum::Float(a + (b as f64)),
            (Sum::Float(a), Sum::Float(b)) => Sum::Float(a + b),
            (Sum::Empty, num) | (num, Sum::Empty) => num,
        }
    }
}

impl Add<&serde_json::Value> for Sum {
    type Output = Sum;

    fn add(self, rhs: &serde_json::Value) -> Self::Output {
        let rhs_num: Sum = rhs.into();
        self + rhs_num
    }
}

impl AddAssign<&serde_json::Value> for Sum {
    fn add_assign(&mut self, rhs: &serde_json::Value) {
        let rhs_num: Sum = rhs.into();
        *self = *self + rhs_num;
    }
}

impl From<&serde_json::Value> for Sum {
    fn from(num: &serde_json::Value) -> Self {
        if let Some(i) = num.as_i64() {
            Sum::Int(i)
        } else if let Some(f) = num.as_f64() {
            Sum::Float(f)
        } else {
            Sum::Empty
        }
    }
}

impl TryFrom<Sum> for serde_json::Number {
    type Error = crate::Error;

    fn try_from(value: Sum) -> crate::Result<Self> {
        match value {
            Sum::Int(i) => Ok(serde_json::Number::from(i)),
            Sum::Float(f) => serde_json::Number::from_f64(f).ok_or_else(|| {
                crate::ErrorKind::ArithmeticOverflow.with_message("aggregator has non-finite value")
            }),
            Sum::Empty => Ok(serde_json::Number::from(0)),
        }
    }
}

#[derive(Debug)]
pub enum Aggregator {
    Count { count: u64 },
    Sum { sum: Sum },
    Average { sum: f64, count: u64 },
    Min { min: Option<QueryClauseItem> },
    Max { max: Option<QueryClauseItem> },
}

impl FromStr for Aggregator {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        // A match statement seems like the right thing to do, but it means forcing the string to lowercase first.
        // This allows us to do the comparison in a case-insensitive way without having to allocate a new string.
        if s.eq_ignore_ascii_case("count") {
            Ok(Aggregator::Count { count: 0 })
        } else if s.eq_ignore_ascii_case("sum") {
            Ok(Aggregator::Sum { sum: Sum::Empty })
        } else if s.eq_ignore_ascii_case("average") {
            Ok(Aggregator::Average { sum: 0.0, count: 0 })
        } else if s.eq_ignore_ascii_case("min") {
            Ok(Aggregator::Min { min: None })
        } else if s.eq_ignore_ascii_case("max") {
            Ok(Aggregator::Max { max: None })
        } else {
            Err(ErrorKind::UnsupportedQueryPlan.with_message(format!("unknown aggregator: {}", s)))
        }
    }
}

impl Aggregator {
    pub fn into_value(self) -> crate::Result<Option<serde_json::Value>> {
        let value = match self {
            Aggregator::Count { count } => Some(serde_json::Value::Number(count.into())),
            Aggregator::Sum { sum } => Some(serde_json::Value::Number(sum.try_into()?)),
            Aggregator::Average { sum, count } => {
                if count == 0 {
                    None
                } else {
                    let avg = sum / (count as f64);
                    Some(serde_json::Value::Number(
                        serde_json::Number::from_f64(avg).ok_or_else(|| {
                            crate::ErrorKind::ArithmeticOverflow
                                .with_message("aggregator has non-finite value")
                        })?,
                    ))
                }
            }
            Aggregator::Min { min, .. } => min.and_then(|c| c.item),
            Aggregator::Max { max, .. } => max.and_then(|c| c.item),
        };
        Ok(value)
    }

    /// Aggregates the current value with the provided value, updating it in place.
    pub fn aggregate(&mut self, clause_item: &QueryClauseItem) -> crate::Result<()> {
        match self {
            Aggregator::Count { count } => {
                let value = require_non_null_value(clause_item, "count")?;
                let int_value = value.as_u64().ok_or_else(|| {
                    crate::ErrorKind::InvalidGatewayResponse
                        .with_message("count aggregator expects an integer value")
                })?;
                *count += int_value;
            }
            Aggregator::Sum { sum } => {
                let value = require_non_null_value(clause_item, "sum")?;
                *sum += value;
            }
            Aggregator::Average { sum, count } => {
                let value = require_non_null_value(clause_item, "average")?;
                #[derive(Debug, Deserialize)]
                struct AverageItem {
                    sum: f64,
                    count: u64,
                }
                let item: AverageItem = serde_json::from_value(value.clone()).map_err(|e| {
                    crate::ErrorKind::InvalidGatewayResponse.with_message(format!(
                        "average aggregator expects object with 'sum' and 'count' properties: {}",
                        e
                    ))
                })?;
                *sum += item.sum;
                *count += item.count;
            }
            Aggregator::Min { min } => {
                if let Some(new) =
                    better_minmax_candidate(min, clause_item, std::cmp::Ordering::Less)?
                {
                    *min = Some(new);
                }
            }
            Aggregator::Max { max } => {
                if let Some(new) =
                    better_minmax_candidate(max, &clause_item, std::cmp::Ordering::Greater)?
                {
                    *max = Some(new);
                }
            }
        }
        Ok(())
    }
}

/// Helper function to extract a non-null value from a QueryClauseItem.
fn require_non_null_value<'a>(
    clause_item: &'a QueryClauseItem,
    aggregator_name: &str,
) -> crate::Result<&'a serde_json::Value> {
    clause_item.item.as_ref().ok_or_else(|| {
        crate::ErrorKind::InvalidGatewayResponse.with_message(format!(
            "{} aggregator expects a non-null value",
            aggregator_name
        ))
    })
}

fn better_minmax_candidate(
    current: &Option<QueryClauseItem>,
    candidate: &QueryClauseItem,
    preferred_ordering: std::cmp::Ordering,
) -> crate::Result<Option<QueryClauseItem>> {
    let candidate_value = match (&candidate.item, &candidate.item2) {
        // Prefer the higher-precision "item2" value
        (_, Some(serde_json::Value::Object(o))) => {
            let count = o.get("count").and_then(|v| v.as_u64()).ok_or_else(|| {
                crate::ErrorKind::InvalidGatewayResponse
                    .with_message("max aggregator expects 'item2' to have a 'count' property")
            })?;

            if count == 0 {
                tracing::trace!("ignoring min/max candidate with zero count");
                // Ignore aggregation if count is zero
                return Ok(None);
            }

            let value = o.get("max").or_else(|| o.get("min")).ok_or_else(|| {
                crate::ErrorKind::InvalidGatewayResponse.with_message(
                    "max aggregator expects 'item2' to have a 'max' or 'min' property",
                )
            })?;
            QueryClauseItem::from_value(value.clone())
        }
        (Some(i), _) => QueryClauseItem::from_value(i.clone()),
        _ => {
            return Err(crate::ErrorKind::InvalidGatewayResponse
                .with_message("min/max aggregator expects either 'item' or 'item2' to be present"))
        }
    };

    Ok(match current {
        None => Some(candidate_value),
        Some(existing) if candidate_value.compare(existing)? == preferred_ordering => {
            Some(candidate_value)
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn count() -> crate::Result<()> {
        let mut aggregator = Aggregator::Count { count: 0 };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(5)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(3)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(7)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(15)));

        Ok(())
    }

    #[test]
    fn count_zero_values() -> crate::Result<()> {
        let mut aggregator = Aggregator::Count { count: 0 };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(0)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(0)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(0)));

        Ok(())
    }

    #[test]
    fn count_empty() -> crate::Result<()> {
        let aggregator = Aggregator::Count { count: 0 };

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(0)));

        Ok(())
    }

    #[test]
    fn sum() -> crate::Result<()> {
        let mut aggregator = Aggregator::Sum { sum: Sum::Empty };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(10.5)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(20)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(-5.5)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(25.0)));

        Ok(())
    }

    #[test]
    fn sum_empty() -> crate::Result<()> {
        let aggregator = Aggregator::Sum { sum: Sum::Empty };

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(0)));

        Ok(())
    }

    #[test]
    fn sum_all_integers() -> crate::Result<()> {
        let mut aggregator = Aggregator::Sum { sum: Sum::Empty };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(10)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(20)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(-5)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(25)));

        Ok(())
    }

    #[test]
    fn sum_mixed_types() -> crate::Result<()> {
        let mut aggregator = Aggregator::Sum { sum: Sum::Empty };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(10)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(20.5)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(-5)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(25.5)));

        Ok(())
    }

    #[test]
    fn average() -> crate::Result<()> {
        let mut aggregator = Aggregator::Average { sum: 0.0, count: 0 };

        aggregator.aggregate(&QueryClauseItem::from_value(
            json!({"sum": 9.0, "count": 2}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_value(
            json!({"sum": 4.0, "count": 3}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_value(
            json!({"sum": 5.0, "count": 1}),
        ))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(3.0)));

        Ok(())
    }

    #[test]
    fn average_empty() -> crate::Result<()> {
        let aggregator = Aggregator::Average { sum: 0.0, count: 0 };

        let result = aggregator.into_value()?;
        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn min_with_objects() -> crate::Result<()> {
        let mut aggregator = Aggregator::Min { min: None };

        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(10),
            json!({"min": 10, "count": 1}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(5),
            json!({"min": 5, "count": 2}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(15),
            json!({"min": 15, "count": 1}),
        ))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(5)));

        Ok(())
    }

    #[test]
    fn min_with_direct_values() -> crate::Result<()> {
        let mut aggregator = Aggregator::Min { min: None };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(10)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(5)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(15)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(5)));

        Ok(())
    }

    #[test]
    fn min_ignore_zero_count() -> crate::Result<()> {
        let mut aggregator = Aggregator::Min { min: None };

        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(10),
            json!({"min": 10, "count": 1}),
        ))?;
        // Zero count values are ignored because they come from empty partitions
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(1),
            json!({"min": 1, "count": 0}),
        ))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(10)));

        Ok(())
    }

    #[test]
    fn min_empty() -> crate::Result<()> {
        let aggregator = Aggregator::Min { min: None };

        let result = aggregator.into_value()?;
        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn max_with_objects() -> crate::Result<()> {
        let mut aggregator = Aggregator::Max { max: None };

        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(10),
            json!({"max": 10, "count": 1}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(5),
            json!({"max": 5, "count": 2}),
        ))?;
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(15),
            json!({"max": 15, "count": 1}),
        ))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(15)));

        Ok(())
    }

    #[test]
    fn max_with_direct_values() -> crate::Result<()> {
        let mut aggregator = Aggregator::Max { max: None };

        aggregator.aggregate(&QueryClauseItem::from_value(json!(10)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(5)))?;
        aggregator.aggregate(&QueryClauseItem::from_value(json!(15)))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(15)));

        Ok(())
    }

    #[test]
    fn max_ignore_zero_count() -> crate::Result<()> {
        let mut aggregator = Aggregator::Max { max: None };

        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(10),
            json!({"max": 10, "count": 1}),
        ))?;
        // Zero count values are ignored because they come from empty partitions
        aggregator.aggregate(&QueryClauseItem::from_values(
            json!(100),
            json!({"max": 100, "count": 0}),
        ))?;

        let result = aggregator.into_value()?;
        assert_eq!(result, Some(json!(10)));

        Ok(())
    }

    #[test]
    fn max_empty() -> crate::Result<()> {
        let aggregator = Aggregator::Max { max: None };

        let result = aggregator.into_value()?;
        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn min_max_with_strings() -> crate::Result<()> {
        let mut min_aggregator = Aggregator::Min { min: None };
        let mut max_aggregator = Aggregator::Max { max: None };

        min_aggregator.aggregate(&QueryClauseItem::from_value(json!("banana")))?;
        min_aggregator.aggregate(&QueryClauseItem::from_value(json!("apple")))?;
        min_aggregator.aggregate(&QueryClauseItem::from_value(json!("cherry")))?;

        max_aggregator.aggregate(&QueryClauseItem::from_value(json!("banana")))?;
        max_aggregator.aggregate(&QueryClauseItem::from_value(json!("apple")))?;
        max_aggregator.aggregate(&QueryClauseItem::from_value(json!("cherry")))?;

        let min_result = min_aggregator.into_value()?;
        let max_result = max_aggregator.into_value()?;
        assert_eq!(min_result, Some(json!("apple")));
        assert_eq!(max_result, Some(json!("cherry")));

        Ok(())
    }
}
