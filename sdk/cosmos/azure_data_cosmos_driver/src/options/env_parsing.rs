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

    /// Create validation bounds with only min.
    pub const fn min(min: T) -> Self {
        Self {
            min: Some(min),
            max: None,
        }
    }

    /// Create validation bounds with only max.
    pub const fn max(max: T) -> Self {
        Self {
            min: None,
            max: Some(max),
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
