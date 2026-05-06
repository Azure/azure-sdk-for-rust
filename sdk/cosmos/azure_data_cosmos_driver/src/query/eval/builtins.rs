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
            let mut result = String::new();
            for arg in args {
                match arg {
                    CosmosValue::String(s) => result.push_str(s),
                    CosmosValue::Undefined => return Ok(CosmosValue::Undefined),
                    _ => return Ok(CosmosValue::Undefined),
                }
            }
            Ok(CosmosValue::String(result))
        }
        "SUBSTRING" => {
            let s = match args.first() {
                Some(CosmosValue::String(s)) => s,
                _ => return Ok(CosmosValue::Undefined),
            };
            let start = match args.get(1) {
                Some(CosmosValue::Number(n)) => *n as usize,
                Some(CosmosValue::Integer(n)) => *n as usize,
                _ => return Ok(CosmosValue::Undefined),
            };
            let len = match args.get(2) {
                Some(CosmosValue::Number(n)) => *n as usize,
                Some(CosmosValue::Integer(n)) => *n as usize,
                _ => return Ok(CosmosValue::Undefined),
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
        "LEFT" => match args {
            [CosmosValue::String(s), CosmosValue::Number(n)] => {
                let n = *n as usize;
                Ok(CosmosValue::String(s.chars().take(n).collect()))
            }
            [CosmosValue::String(s), CosmosValue::Integer(n)] => {
                let n = *n as usize;
                Ok(CosmosValue::String(s.chars().take(n).collect()))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "RIGHT" => match args {
            [CosmosValue::String(s), CosmosValue::Number(n)] => {
                let n = *n as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Ok(CosmosValue::String(chars[start..].iter().collect()))
            }
            [CosmosValue::String(s), CosmosValue::Integer(n)] => {
                let n = *n as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Ok(CosmosValue::String(chars[start..].iter().collect()))
            }
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
                let Some(start) = as_number(start).map(|value| value as i64) else {
                    return Ok(CosmosValue::Undefined);
                };
                let start = if start < 0 {
                    (arr.len() as i64 + start).max(0) as usize
                } else {
                    start as usize
                };
                let len = match args.get(2) {
                    Some(value) => as_number(value).map(|n| n as usize),
                    _ => None,
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
