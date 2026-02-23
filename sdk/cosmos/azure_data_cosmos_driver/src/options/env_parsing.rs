// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

/// Validation bounds for parsed values.
pub(super) struct ValidationBounds<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

impl<T> ValidationBounds<T> {
    /// No validation bounds.
    pub const fn none() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    /// Create validation bounds with both min and max.
    pub const fn range(min: T, max: T) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }

    /// Create validation bounds with only a minimum value.
    pub const fn min(min: T) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }
}

/// Parses a value from an environment variable with proper error handling and optional validation.
///
/// Returns the value from the builder if present, otherwise attempts to parse from the environment variable.
/// Falls back to the default value if the environment variable is not set.
///
/// Optionally validates the final value against min/max bounds.
pub(super) fn parse_from_env<T>(
    builder_value: Option<T>,
    env_var_name: &str,
    default: T,
    bounds: ValidationBounds<T>,
) -> azure_core::Result<T>
where
    T: std::str::FromStr + PartialOrd + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let value = match builder_value {
        Some(v) => v,
        None => match std::env::var(env_var_name) {
            Ok(v) => v.parse().map_err(|e| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!(
                        "Failed to parse {} as {}: {} ({})",
                        env_var_name,
                        std::any::type_name::<T>(),
                        v,
                        e
                    ),
                )
            })?,
            Err(_) => default,
        },
    };

    validate_bounds(value, env_var_name, bounds)
}

/// Validates a value against optional min/max bounds.
fn validate_bounds<T>(
    value: T,
    env_var_name: &str,
    bounds: ValidationBounds<T>,
) -> azure_core::Result<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    if let Some(min) = bounds.min {
        if value < min {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "{} must be at least {:?}, got {:?}",
                    env_var_name
                        .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                        .unwrap_or(env_var_name)
                        .to_lowercase(),
                    min,
                    value
                ),
            ));
        }
    }

    if let Some(max) = bounds.max {
        if value > max {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "{} must be at most {:?}, got {:?}",
                    env_var_name
                        .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
                        .unwrap_or(env_var_name)
                        .to_lowercase(),
                    max,
                    value
                ),
            ));
        }
    }

    Ok(value)
}

/// Parses a duration from an environment variable (in milliseconds) with validation.
pub(super) fn parse_duration_millis_from_env(
    builder_value: Option<Duration>,
    env_var_name: &str,
    default_millis: u64,
    min_millis: u64,
    max_millis: u64,
) -> azure_core::Result<Duration> {
    let value = match builder_value {
        Some(v) => v,
        None => match std::env::var(env_var_name) {
            Ok(v) => {
                let millis = v.parse::<u64>().map_err(|e| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!(
                            "Failed to parse {} as u64 milliseconds: {} ({})",
                            env_var_name, v, e
                        ),
                    )
                })?;
                Duration::from_millis(millis)
            }
            Err(_) => Duration::from_millis(default_millis),
        },
    };

    validate_duration_bounds(value, env_var_name, min_millis, max_millis)?;
    Ok(value)
}

/// Validates a duration value against min/max bounds (in milliseconds).
///
/// Comparisons use `u128` to avoid silent truncation since
/// [`Duration::as_millis`] returns `u128`.
fn validate_duration_bounds(
    value: Duration,
    env_var_name: &str,
    min_millis: u64,
    max_millis: u64,
) -> azure_core::Result<()> {
    let value_millis = value.as_millis();
    let min = u128::from(min_millis);
    let max = u128::from(max_millis);
    let field_name = env_var_name
        .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
        .unwrap_or(env_var_name)
        .to_lowercase();

    if value_millis < min {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "{} must be at least {}ms, got {}ms",
                field_name, min_millis, value_millis
            ),
        ));
    }

    if value_millis > max {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "{} must be at most {}ms, got {}ms",
                field_name, max_millis, value_millis
            ),
        ));
    }

    Ok(())
}

/// Parses an optional duration from an environment variable (in milliseconds) with validation.
pub(super) fn parse_optional_duration_millis_from_env(
    builder_value: Option<Duration>,
    env_var_name: &str,
    min_millis: u64,
    max_millis: u64,
) -> azure_core::Result<Option<Duration>> {
    match builder_value {
        Some(timeout) => {
            validate_duration_bounds(timeout, env_var_name, min_millis, max_millis)?;
            Ok(Some(timeout))
        }
        None => match std::env::var(env_var_name) {
            Ok(v) => {
                let timeout = v.parse::<u64>().map(Duration::from_millis).map_err(|e| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!(
                            "Failed to parse {} as milliseconds: {} ({})",
                            env_var_name, v, e
                        ),
                    )
                })?;
                validate_duration_bounds(timeout, env_var_name, min_millis, max_millis)?;
                Ok(Some(timeout))
            }
            Err(_) => Ok(None),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn with_env_var<R>(name: &str, value: Option<&str>, f: impl FnOnce() -> R) -> R {
        let _guard = env_lock().lock().expect("env lock poisoned");
        let previous = std::env::var(name).ok();

        match value {
            Some(v) => std::env::set_var(name, v),
            None => std::env::remove_var(name),
        }

        let result = f();

        match previous {
            Some(v) => std::env::set_var(name, v),
            None => std::env::remove_var(name),
        }

        result
    }

    #[test]
    fn parse_from_env_prefers_builder_value() {
        with_env_var("AZURE_COSMOS_TEST_INT", Some("42"), || {
            let value = parse_from_env(
                Some(7_u32),
                "AZURE_COSMOS_TEST_INT",
                1_u32,
                ValidationBounds::none(),
            )
            .unwrap();

            assert_eq!(value, 7);
        });
    }

    #[test]
    fn parse_from_env_uses_default_when_env_missing() {
        with_env_var("AZURE_COSMOS_TEST_DEFAULT", None, || {
            let value = parse_from_env(
                None::<u32>,
                "AZURE_COSMOS_TEST_DEFAULT",
                99_u32,
                ValidationBounds::none(),
            )
            .unwrap();

            assert_eq!(value, 99);
        });
    }

    #[test]
    fn parse_from_env_reports_parse_error() {
        with_env_var("AZURE_COSMOS_TEST_PARSE_ERR", Some("not-a-number"), || {
            let err = parse_from_env(
                None::<u32>,
                "AZURE_COSMOS_TEST_PARSE_ERR",
                5_u32,
                ValidationBounds::none(),
            )
            .unwrap_err();

            let message = err.to_string();
            assert!(message.contains("AZURE_COSMOS_TEST_PARSE_ERR"));
            assert!(message.contains("Failed to parse"));
        });
    }

    #[test]
    fn parse_from_env_validates_min_and_max_bounds() {
        let below_min = parse_from_env(
            Some(4_u32),
            "AZURE_COSMOS_CONNECTION_POOL_TEST_LIMIT",
            0_u32,
            ValidationBounds::range(5_u32, 10_u32),
        )
        .unwrap_err()
        .to_string();
        assert!(below_min.contains("test_limit must be at least 5"));

        let above_max = parse_from_env(
            Some(11_u32),
            "AZURE_COSMOS_CONNECTION_POOL_TEST_LIMIT",
            0_u32,
            ValidationBounds::range(5_u32, 10_u32),
        )
        .unwrap_err()
        .to_string();
        assert!(above_max.contains("test_limit must be at most 10"));
    }

    #[test]
    fn parse_duration_millis_from_env_parses_and_validates() {
        with_env_var("AZURE_COSMOS_TEST_DURATION", Some("250"), || {
            let value =
                parse_duration_millis_from_env(None, "AZURE_COSMOS_TEST_DURATION", 100, 50, 500)
                    .unwrap();

            assert_eq!(value, Duration::from_millis(250));
        });
    }

    #[test]
    fn parse_duration_millis_from_env_uses_default_when_missing() {
        with_env_var("AZURE_COSMOS_TEST_DURATION_DEFAULT", None, || {
            let value = parse_duration_millis_from_env(
                None,
                "AZURE_COSMOS_TEST_DURATION_DEFAULT",
                123,
                50,
                500,
            )
            .unwrap();

            assert_eq!(value, Duration::from_millis(123));
        });
    }

    #[test]
    fn parse_optional_duration_millis_from_env_none_when_missing() {
        with_env_var("AZURE_COSMOS_TEST_OPTIONAL_DURATION", None, || {
            let value = parse_optional_duration_millis_from_env(
                None,
                "AZURE_COSMOS_TEST_OPTIONAL_DURATION",
                10,
                1000,
            )
            .unwrap();

            assert_eq!(value, None);
        });
    }

    #[test]
    fn parse_optional_duration_millis_from_env_parses_and_validates() {
        with_env_var(
            "AZURE_COSMOS_TEST_OPTIONAL_DURATION_SET",
            Some("450"),
            || {
                let value = parse_optional_duration_millis_from_env(
                    None,
                    "AZURE_COSMOS_TEST_OPTIONAL_DURATION_SET",
                    100,
                    500,
                )
                .unwrap();

                assert_eq!(value, Some(Duration::from_millis(450)));
            },
        );
    }
}
