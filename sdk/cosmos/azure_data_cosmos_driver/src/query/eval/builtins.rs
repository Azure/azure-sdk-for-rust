// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore STARTSWITH ENDSWITH LTRIM RTRIM TOSTRING multibyte

//! Built-in scalar function evaluation. Split out of val/mod.rs (#16) so the
//! ~200-line function dispatch table lives in its own file.

use super::EvalError;
use crate::query::value::CosmosValue;
pub(super) fn eval_function(name: &str, args: &[CosmosValue]) -> Result<CosmosValue, EvalError> {
    let upper = name.to_ascii_uppercase();
    match upper.as_str() {
        // Type checking
        "IS_DEFINED" => Ok(CosmosValue::Boolean(
            args.first().is_some_and(|v| !v.is_undefined()),
        )),
        "IS_NULL" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Null)
        ))),
        "IS_BOOL" | "IS_BOOLEAN" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Boolean(_))
        ))),
        "IS_NUMBER" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Number(_) | CosmosValue::Integer(_))
        ))),
        "IS_STRING" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::String(_))
        ))),
        "IS_ARRAY" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Array(_))
        ))),
        "IS_OBJECT" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Object(_))
        ))),

        // String functions
        "CONTAINS" => match args {
            [CosmosValue::String(s), CosmosValue::String(sub), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().contains(&sub.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.contains(sub.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "STARTSWITH" => match args {
            [CosmosValue::String(s), CosmosValue::String(prefix), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().starts_with(&prefix.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.starts_with(prefix.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "ENDSWITH" => match args {
            [CosmosValue::String(s), CosmosValue::String(suffix), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().ends_with(&suffix.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.ends_with(suffix.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "UPPER" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.to_uppercase())),
            _ => Ok(CosmosValue::Undefined),
        },
        "LOWER" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.to_lowercase())),
            _ => Ok(CosmosValue::Undefined),
        },
        "LENGTH" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::Integer(s.chars().count() as i64)),
            _ => Ok(CosmosValue::Undefined),
        },
        "LTRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim_start().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "RTRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim_end().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "TRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "CONCAT" => {
            // Cosmos SQL `CONCAT` requires every argument to be a string;
            // any non-string (including `Undefined`) yields `Undefined`. The
            // gateway-comparison test `gw_concat_*` pins this contract.
            let mut result = String::new();
            for arg in args {
                match arg {
                    CosmosValue::String(s) => result.push_str(s),
                    _ => return Ok(CosmosValue::Undefined),
                }
            }
            Ok(CosmosValue::String(result))
        }
        "SUBSTRING" => {
            // (#4) Negative `start` or `length` is not a valid Cosmos
            // SUBSTRING input; previously the code did `n as usize` and
            // wrapped to ~2^63, silently producing odd results. Reject
            // negatives (and non-numeric / non-finite arguments) by
            // returning `Undefined`.
            let s = match args.first() {
                Some(CosmosValue::String(s)) => s,
                _ => return Ok(CosmosValue::Undefined),
            };
            let Some(start) = nonneg_usize(args.get(1)) else {
                return Ok(CosmosValue::Undefined);
            };
            let Some(len) = nonneg_usize(args.get(2)) else {
                return Ok(CosmosValue::Undefined);
            };
            Ok(CosmosValue::String(
                s.chars().skip(start).take(len).collect(),
            ))
        }
        "REPLACE" => match args {
            [CosmosValue::String(s), CosmosValue::String(old), CosmosValue::String(new)] => {
                Ok(CosmosValue::String(s.replace(old.as_str(), new.as_str())))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        // (#4) `LEFT` / `RIGHT` reject negative lengths; previously
        // `n as usize` wrapped negative i64 to ~2^63 and `LEFT(s, -1)`
        // returned the entire string instead of `Undefined`.
        "LEFT" => match args {
            [CosmosValue::String(s), n_arg] => match nonneg_usize(Some(n_arg)) {
                Some(n) => Ok(CosmosValue::String(s.chars().take(n).collect())),
                None => Ok(CosmosValue::Undefined),
            },
            _ => Ok(CosmosValue::Undefined),
        },
        "RIGHT" => match args {
            [CosmosValue::String(s), n_arg] => match nonneg_usize(Some(n_arg)) {
                Some(n) => {
                    let chars: Vec<char> = s.chars().collect();
                    let start = chars.len().saturating_sub(n);
                    Ok(CosmosValue::String(chars[start..].iter().collect()))
                }
                None => Ok(CosmosValue::Undefined),
            },
            _ => Ok(CosmosValue::Undefined),
        },
        "TOSTRING" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.clone())),
            Some(CosmosValue::Integer(n)) => Ok(CosmosValue::String(format!("{n}"))),
            Some(CosmosValue::Number(n)) => Ok(CosmosValue::String(format!("{n}"))),
            Some(CosmosValue::Boolean(b)) => Ok(CosmosValue::String(
                if *b { "true" } else { "false" }.into(),
            )),
            Some(CosmosValue::Null) => Ok(CosmosValue::String("null".into())),
            _ => Ok(CosmosValue::Undefined),
        },

        // Math functions
        "ABS" => num_fn1(args, |n| n.abs()),
        "CEILING" => num_fn1(args, |n| n.ceil()),
        "FLOOR" => num_fn1(args, |n| n.floor()),
        "ROUND" => num_fn1(args, |n| n.round()),
        "POWER" => num_fn2(args, |a, b| a.powf(b)),
        "SQRT" => num_fn1(args, |n| n.sqrt()),
        "LOG" => num_fn1(args, |n| n.ln()),
        "LOG10" => num_fn1(args, |n| n.log10()),
        "EXP" => num_fn1(args, |n| n.exp()),
        "SIGN" => num_fn1(args, |n| {
            if n > 0.0 {
                1.0
            } else if n < 0.0 {
                -1.0
            } else {
                0.0
            }
        }),

        // Array functions
        "ARRAY_CONTAINS" => match args {
            [CosmosValue::Array(arr), search, ..] => {
                let found = arr
                    .iter()
                    .any(|item| matches!(item.cosmos_eq(search), CosmosValue::Boolean(true)));
                Ok(CosmosValue::Boolean(found))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "ARRAY_LENGTH" => match args.first() {
            Some(CosmosValue::Array(arr)) => Ok(CosmosValue::Integer(arr.len() as i64)),
            _ => Ok(CosmosValue::Undefined),
        },
        "ARRAY_SLICE" => match args {
            [CosmosValue::Array(arr), start, ..] => {
                // Negative `start` is meaningful for `ARRAY_SLICE` - it
                // indexes from the end, matching Cosmos semantics. The
                // `length` argument however must be non-negative; we treat
                // negatives as `Undefined`.
                let Some(start) = as_number(start).map(|value| value as i64) else {
                    return Ok(CosmosValue::Undefined);
                };
                let start = if start < 0 {
                    (arr.len() as i64 + start).max(0) as usize
                } else {
                    start as usize
                };
                let len = match args.get(2) {
                    Some(value) => match nonneg_usize(Some(value)) {
                        Some(n) => Some(n),
                        None => return Ok(CosmosValue::Undefined),
                    },
                    None => None,
                };
                let end = match len {
                    Some(l) => (start + l).min(arr.len()),
                    None => arr.len(),
                };
                if start >= arr.len() {
                    Ok(CosmosValue::Array(Vec::new()))
                } else {
                    Ok(CosmosValue::Array(arr[start..end].to_vec()))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },

        // Aggregate placeholders (return undefined — they need special handling)
        "COUNT" | "SUM" | "AVG" | "MIN" | "MAX" => Err(EvalError::Unsupported(format!(
            "aggregate function {upper}"
        ))),

        _ => Err(EvalError::UnknownFunction(name.to_string())),
    }
}

pub(super) fn num_fn1(args: &[CosmosValue], f: fn(f64) -> f64) -> Result<CosmosValue, EvalError> {
    Ok(match args.first().and_then(as_number) {
        Some(n) => CosmosValue::Number(f(n)),
        None => CosmosValue::Undefined,
    })
}

pub(super) fn num_fn2(
    args: &[CosmosValue],
    f: fn(f64, f64) -> f64,
) -> Result<CosmosValue, EvalError> {
    Ok(match args {
        [a, b] => match (as_number(a), as_number(b)) {
            (Some(a), Some(b)) => CosmosValue::Number(f(a, b)),
            _ => CosmosValue::Undefined,
        },
        _ => CosmosValue::Undefined,
    })
}

pub(super) fn as_number(value: &CosmosValue) -> Option<f64> {
    match value {
        CosmosValue::Number(n) => Some(*n),
        CosmosValue::Integer(n) => Some(*n as f64),
        _ => None,
    }
}

/// Coerce an argument to a non-negative `usize` for length / index parameters.
///
/// Returns `None` for missing, non-numeric, negative, or non-finite inputs.
/// Used by `SUBSTRING`, `LEFT`, `RIGHT`, and `ARRAY_SLICE` to avoid the
/// `as usize` wrap-around on negative values that previously produced silent
/// surprising behavior (`LEFT(s, -1)` returning the entire string).
fn nonneg_usize(arg: Option<&CosmosValue>) -> Option<usize> {
    let n = match arg? {
        CosmosValue::Integer(n) => *n as f64,
        CosmosValue::Number(n) => *n,
        _ => return None,
    };
    if !n.is_finite() || n < 0.0 {
        return None;
    }
    Some(n as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    // (#4) Regression: previously `SUBSTRING`/`LEFT`/`RIGHT`/`ARRAY_SLICE`
    // length cast negative i64 to usize via `as usize`, wrapping to ~2^63
    // and producing surprising results (e.g. `LEFT('abc', -1)` returned the
    // entire string). All four must now return `Undefined` on negative
    // numeric inputs.
    #[test]
    fn substring_negative_start_is_undefined() {
        let r = eval_function(
            "SUBSTRING",
            &[
                CosmosValue::String("hello".into()),
                CosmosValue::Integer(-1),
                CosmosValue::Integer(3),
            ],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn substring_negative_length_is_undefined() {
        let r = eval_function(
            "SUBSTRING",
            &[
                CosmosValue::String("hello".into()),
                CosmosValue::Integer(0),
                CosmosValue::Integer(-1),
            ],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn left_negative_length_is_undefined() {
        let r = eval_function(
            "LEFT",
            &[
                CosmosValue::String("hello".into()),
                CosmosValue::Integer(-1),
            ],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn right_negative_length_is_undefined() {
        let r = eval_function(
            "RIGHT",
            &[
                CosmosValue::String("hello".into()),
                CosmosValue::Integer(-1),
            ],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn array_slice_negative_length_is_undefined() {
        let arr = CosmosValue::Array(vec![
            CosmosValue::Integer(1),
            CosmosValue::Integer(2),
            CosmosValue::Integer(3),
        ]);
        let r = eval_function(
            "ARRAY_SLICE",
            &[arr, CosmosValue::Integer(0), CosmosValue::Integer(-1)],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    // (#10) CONCAT semantics pinned to match the Cosmos DB gateway.
    //
    // Per the Cosmos SQL reference for `CONCAT`:
    //   - All arguments must be string values.
    //   - Any non-string argument (including `Undefined`, numbers, booleans,
    //     arrays, objects, null) yields `Undefined`.
    //
    // Source: https://learn.microsoft.com/azure/cosmos-db/nosql/query/concat
    //
    // The earlier reviewer note suggested numeric/boolean coercion to match
    // ANSI SQL, but the gateway does NOT coerce - we keep the strict
    // contract here and document it. The gateway-comparison test
    // `gw_concat_plan_parses` ensures the plan-level shape matches.
    #[test]
    fn concat_all_strings_produces_concatenation() {
        let r = eval_function(
            "CONCAT",
            &[
                CosmosValue::String("a".into()),
                CosmosValue::String("b".into()),
                CosmosValue::String("c".into()),
            ],
        )
        .unwrap();
        assert_eq!(r, CosmosValue::String("abc".into()));
    }

    #[test]
    fn concat_with_number_argument_is_undefined() {
        let r = eval_function(
            "CONCAT",
            &[CosmosValue::String("a".into()), CosmosValue::Integer(1)],
        )
        .unwrap();
        assert!(
            matches!(r, CosmosValue::Undefined),
            "Cosmos CONCAT does NOT coerce numbers to strings - expected Undefined, got {r:?}"
        );
    }

    #[test]
    fn concat_with_boolean_argument_is_undefined() {
        let r = eval_function(
            "CONCAT",
            &[CosmosValue::String("a".into()), CosmosValue::Boolean(true)],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn concat_with_null_argument_is_undefined() {
        let r = eval_function(
            "CONCAT",
            &[CosmosValue::String("a".into()), CosmosValue::Null],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }

    #[test]
    fn concat_with_undefined_argument_is_undefined() {
        let r = eval_function(
            "CONCAT",
            &[CosmosValue::String("a".into()), CosmosValue::Undefined],
        )
        .unwrap();
        assert!(matches!(r, CosmosValue::Undefined));
    }
}
