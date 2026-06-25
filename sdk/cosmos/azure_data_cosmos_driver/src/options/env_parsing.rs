// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resolution and validation helpers for option groups.
//!
//! Most environment variables are read and parsed by the `CosmosOptions`
//! derive macro's generated `from_env()`/`from_env_vars()`; the `resolve_*`
//! helpers here only resolve a final value from
//! `builder override → pre-read env value → default` and validate it against
//! bounds. A few driver-level helpers (`parse_duration_millis_from_env`,
//! `parse_from_env`, `parse_optional_bool_from_env`) additionally look up
//! environment variables for option builders that are not macro-generated.
//! Rather than calling `std::env::var` directly, they take a
//! `get_env: &dyn Fn(&str) -> Option<String>` accessor (production passes
//! `|k| std::env::var(k).ok()`; tests inject a deterministic map so option
//! resolution can be exercised without racing on process-wide environment).
//! Like the macro, they log and ignore a present-but-unparseable value
//! (fail-soft) rather than erroring.

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

/// Resolves a value from a builder override and a pre-read environment value,
/// falling back to a default, then validates against optional bounds.
///
/// The environment value is read and parsed by the `CosmosOptions`-generated
/// `from_env()` (the single env-reading mechanism); this helper only performs
/// the `builder → env → default` resolution and bounds validation.
pub(super) fn resolve_from_env<T>(
    builder_value: Option<T>,
    env_value: Option<T>,
    env_var_name: &str,
    default: T,
    bounds: ValidationBounds<T>,
) -> crate::error::Result<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    let value = builder_value.or(env_value).unwrap_or(default);
    validate_bounds(value, env_var_name, bounds)
}

/// Resolves an optional value from a builder override and a pre-read
/// environment value, validating against optional bounds when present.
pub(super) fn resolve_optional_from_env<T>(
    builder_value: Option<T>,
    env_value: Option<T>,
    env_var_name: &str,
    bounds: ValidationBounds<T>,
) -> crate::error::Result<Option<T>>
where
    T: PartialOrd + std::fmt::Debug,
{
    match builder_value.or(env_value) {
        Some(value) => validate_bounds(value, env_var_name, bounds).map(Some),
        None => Ok(None),
    }
}

/// Validates a value against optional min/max bounds.
fn validate_bounds<T>(
    value: T,
    env_var_name: &str,
    bounds: ValidationBounds<T>,
) -> crate::error::Result<T>
where
    T: PartialOrd + std::fmt::Debug,
{
    if let Some(min) = bounds.min {
        if value < min {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "{} must be at least {:?}, got {:?}",
                    short_field_name(env_var_name),
                    min,
                    value
                ))
                .build());
        }
    }

    if let Some(max) = bounds.max {
        if value > max {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "{} must be at most {:?}, got {:?}",
                    short_field_name(env_var_name),
                    max,
                    value
                ))
                .build());
        }
    }

    Ok(value)
}

/// Strips well-known `AZURE_COSMOS_*_` group prefixes from `env_var_name` and
/// lowercases the remainder, producing a short field-style name suitable for
/// inclusion in user-facing validation error messages.
fn short_field_name(env_var_name: &str) -> String {
    env_var_name
        .strip_prefix("AZURE_COSMOS_CONNECTION_POOL_")
        .or_else(|| env_var_name.strip_prefix("AZURE_COSMOS_PPCB_"))
        .unwrap_or(env_var_name)
        .to_lowercase()
}

/// Resolves a duration (in milliseconds) from a builder override and a
/// pre-read environment value, falling back to a default, then validates
/// against millisecond bounds.
pub(crate) fn resolve_duration_ms(
    builder_value: Option<Duration>,
    env_millis: Option<u64>,
    env_var_name: &str,
    default_millis: u64,
    min_millis: u64,
    max_millis: u64,
) -> crate::error::Result<Duration> {
    let value = builder_value
        .or_else(|| env_millis.map(Duration::from_millis))
        .unwrap_or_else(|| Duration::from_millis(default_millis));

    validate_duration_bounds(value, env_var_name, min_millis, max_millis)?;
    Ok(value)
}

/// Reads, resolves, and validates a millisecond-duration environment variable
/// in a single call.
///
/// Looks up `env_var_name` through the supplied `get_env` accessor (parsing it
/// as `u64` milliseconds), then applies the shared `builder → env → default`
/// resolution and bounds validation. `get_env` is normally
/// `|k| std::env::var(k).ok()`; tests inject a deterministic map so they don't
/// race on process-wide environment. This is the single duration env-reading
/// path shared by the driver-level option builders (e.g.
/// [`PartitionFailoverOptions`](crate::options::PartitionFailoverOptions))
/// and the runtime CPU-refresh interval.
pub(crate) fn parse_duration_millis_from_env(
    builder_value: Option<Duration>,
    env_var_name: &str,
    default_millis: u64,
    min_millis: u64,
    max_millis: u64,
    get_env: &dyn Fn(&str) -> Option<String>,
) -> crate::error::Result<Duration> {
    let env_millis = get_env(env_var_name).and_then(|raw| {
        raw.parse::<u64>().ok().or_else(|| {
            // Fail-soft: a present-but-unparseable value is logged and ignored
            // (falls back to the default), matching the macro's lenient
            // env-parsing behavior.
            tracing::warn!(
                env_var = env_var_name,
                value = %raw,
                "failed to parse millisecond duration from environment variable; ignoring",
            );
            None
        })
    });

    resolve_duration_ms(
        builder_value,
        env_millis,
        env_var_name,
        default_millis,
        min_millis,
        max_millis,
    )
}

/// Compatibility wrapper for call sites that still use the legacy helper name.
pub(super) fn parse_from_env<T>(
    builder_value: Option<T>,
    env_var_name: &str,
    default: T,
    bounds: ValidationBounds<T>,
    get_env: &dyn Fn(&str) -> Option<String>,
) -> crate::error::Result<T>
where
    T: PartialOrd + std::fmt::Debug + std::str::FromStr,
{
    let env_value = get_env(env_var_name).and_then(|raw| {
        raw.parse::<T>().ok().or_else(|| {
            // Fail-soft: a present-but-unparseable value is logged and ignored
            // (falls back to the default), matching the macro's lenient
            // env-parsing behavior.
            tracing::warn!(
                env_var = env_var_name,
                value = %raw,
                "failed to parse environment variable; ignoring",
            );
            None
        })
    });
    resolve_from_env(builder_value, env_value, env_var_name, default, bounds)
}

/// Parses an *optional* boolean from `env_var_name` using the same lenient
/// spellings the `CosmosOptions` derive uses for kill-switch booleans
/// (`true`/`false`, `1`/`0`, `yes`/`no`, `on`/`off`, case-insensitive). A
/// builder value, when present, wins over the environment; an unrecognized
/// env value is logged and ignored (treated as unset) so an operator typo on
/// an incident kill switch does not silently flip the switch the wrong way.
///
/// Returns `None` when neither a builder value nor a recognized env value is
/// present, letting the caller fall through to its own default.
pub(super) fn parse_optional_bool_from_env(
    builder_value: Option<bool>,
    env_var_name: &str,
    get_env: &dyn Fn(&str) -> Option<String>,
) -> Option<bool> {
    if let Some(value) = builder_value {
        return Some(value);
    }

    let raw = get_env(env_var_name)?;
    match raw.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Some(true),
        "false" | "0" | "no" | "off" => Some(false),
        _ => {
            tracing::warn!(
                env_var = env_var_name,
                value = %raw,
                "failed to parse boolean environment variable; ignoring",
            );
            None
        }
    }
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
) -> crate::error::Result<()> {
    let value_millis = value.as_millis();
    let min = u128::from(min_millis);
    let max = u128::from(max_millis);
    let field_name = short_field_name(env_var_name);

    if value_millis < min {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message(format!(
                "{} must be at least {}ms, got {}ms",
                field_name, min_millis, value_millis
            ))
            .build());
    }

    if value_millis > max {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message(format!(
                "{} must be at most {}ms, got {}ms",
                field_name, max_millis, value_millis
            ))
            .build());
    }

    Ok(())
}

/// Resolves an optional duration (in milliseconds) from a builder override
/// and a pre-read environment value, validating against millisecond bounds
/// when a value is present.
pub(super) fn resolve_optional_duration_ms(
    builder_value: Option<Duration>,
    env_millis: Option<u64>,
    env_var_name: &str,
    min_millis: u64,
    max_millis: u64,
) -> crate::error::Result<Option<Duration>> {
    match builder_value.or_else(|| env_millis.map(Duration::from_millis)) {
        Some(value) => {
            validate_duration_bounds(value, env_var_name, min_millis, max_millis)?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

/// Shared helpers for tests that exercise the *real* process environment.
///
/// Mutating `std::env` is process-global, so every test that does so — in any
/// module — must serialize on one lock and restore what it changed, otherwise
/// tests racing in parallel corrupt each other's view of the environment. This
/// module provides that single lock plus a scoping helper, so the partition-
/// failover and driver-options test modules can validate that the production
/// `build()` path (which reads `std::env::var`) honors the `AZURE_COSMOS_PPCB_*`
/// variables.
#[cfg(test)]
pub(crate) mod test_env {
    use std::sync::Mutex;

    /// Process-wide lock serializing every test that mutates real environment
    /// variables through [`with_scoped_env`].
    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Every `AZURE_COSMOS_PPCB_*` environment variable read by
    /// [`PartitionFailoverOptionsBuilder::build`](crate::options::PartitionFailoverOptionsBuilder::build).
    /// Tests clear all of these before applying their own subset so a value left
    /// in the ambient / CI environment cannot leak into an assertion.
    pub(crate) const PPCB_ENV_VARS: &[&str] = &[
        "AZURE_COSMOS_PPCB_ENABLED",
        "AZURE_COSMOS_PPCB_ENABLED_OVERRIDE",
        "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
        "AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD",
        "AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS",
        "AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS",
        "AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS",
        "AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD",
    ];

    /// Runs `body` with a hermetic view of the named environment variables.
    ///
    /// Holds the shared [`ENV_TEST_LOCK`] for the whole call (so parallel tests
    /// don't race), snapshots and **removes** every key in `clear` (so the test
    /// starts from a known-empty state regardless of ambient environment), then
    /// applies each `(key, value)` in `set`. The original values of all `clear`
    /// keys are restored when the call returns — including on panic, via a
    /// `Drop` guard — and the restore happens before the lock is released.
    ///
    /// `std::env::set_var` / `remove_var` are safe to call here on the crate's
    /// edition; the serialization above is what makes that sound under parallel
    /// test execution.
    pub(crate) fn with_scoped_env<R>(
        clear: &[&str],
        set: &[(&str, &str)],
        body: impl FnOnce() -> R,
    ) -> R {
        // Recover a poisoned lock: a prior test panicking while holding it left
        // no shared state behind (the env is restored by the `Drop` guard), so
        // the lock is still usable.
        let _guard = ENV_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        let saved: Vec<(String, Option<String>)> = clear
            .iter()
            .map(|&k| (k.to_string(), std::env::var(k).ok()))
            .collect();

        // Restore-on-drop runs before `_guard` drops (reverse declaration
        // order), so the environment is repaired while the lock is still held,
        // even if `body` panics.
        struct Restore(Vec<(String, Option<String>)>);
        impl Drop for Restore {
            fn drop(&mut self) {
                for (k, original) in &self.0 {
                    match original {
                        Some(v) => std::env::set_var(k, v),
                        None => std::env::remove_var(k),
                    }
                }
            }
        }
        let _restore = Restore(saved);

        for &k in clear {
            std::env::remove_var(k);
        }
        for &(k, v) in set {
            std::env::set_var(k, v);
        }

        body()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_from_env_prefers_builder_value() {
        let value = resolve_from_env(
            Some(7_u32),
            Some(42_u32),
            "AZURE_COSMOS_TEST_INT",
            1_u32,
            ValidationBounds::none(),
        )
        .unwrap();
        assert_eq!(value, 7);
    }

    #[test]
    fn resolve_from_env_uses_env_when_no_builder_value() {
        let value = resolve_from_env(
            None::<u32>,
            Some(42_u32),
            "AZURE_COSMOS_TEST_INT",
            1_u32,
            ValidationBounds::none(),
        )
        .unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn resolve_from_env_uses_default_when_builder_and_env_missing() {
        let value = resolve_from_env(
            None::<u32>,
            None,
            "AZURE_COSMOS_TEST_DEFAULT",
            99_u32,
            ValidationBounds::none(),
        )
        .unwrap();
        assert_eq!(value, 99);
    }

    #[test]
    fn resolve_from_env_validates_min_and_max_bounds() {
        let below_min = resolve_from_env(
            Some(4_u32),
            None,
            "AZURE_COSMOS_CONNECTION_POOL_TEST_LIMIT",
            0_u32,
            ValidationBounds::range(5_u32, 10_u32),
        )
        .unwrap_err()
        .to_string();
        assert!(below_min.contains("test_limit must be at least 5"));

        let above_max = resolve_from_env(
            None,
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
    fn resolve_optional_from_env_none_when_unset() {
        let value = resolve_optional_from_env(
            None::<u32>,
            None,
            "AZURE_COSMOS_TEST_OPTIONAL",
            ValidationBounds::none(),
        )
        .unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn resolve_optional_from_env_validates_present_value() {
        let err = resolve_optional_from_env(
            None,
            Some(99_u32),
            "AZURE_COSMOS_CONNECTION_POOL_TEST_LIMIT",
            ValidationBounds::range(1_u32, 10_u32),
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("test_limit must be at most 10"));
    }

    #[test]
    fn resolve_duration_ms_prefers_builder_then_env_then_default() {
        // Builder wins.
        assert_eq!(
            resolve_duration_ms(
                Some(Duration::from_millis(7)),
                Some(42),
                "AZURE_COSMOS_TEST_DURATION",
                100,
                1,
                500
            )
            .unwrap(),
            Duration::from_millis(7)
        );
        // Env used when no builder value.
        assert_eq!(
            resolve_duration_ms(None, Some(250), "AZURE_COSMOS_TEST_DURATION", 100, 1, 500)
                .unwrap(),
            Duration::from_millis(250)
        );
        // Default used when neither present.
        assert_eq!(
            resolve_duration_ms(None, None, "AZURE_COSMOS_TEST_DURATION", 123, 1, 500).unwrap(),
            Duration::from_millis(123)
        );
    }

    #[test]
    fn resolve_duration_ms_validates_bounds() {
        let err = resolve_duration_ms(
            None,
            Some(50),
            "AZURE_COSMOS_CONNECTION_POOL_MIN_CONNECT_TIMEOUT_MS",
            100,
            100,
            6_000,
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("min_connect_timeout_ms must be at least 100ms"));
    }

    #[test]
    fn resolve_optional_duration_ms_none_when_unset() {
        let value = resolve_optional_duration_ms(
            None,
            None,
            "AZURE_COSMOS_TEST_OPTIONAL_DURATION",
            10,
            1_000,
        )
        .unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn resolve_optional_duration_ms_uses_env_and_validates() {
        let value = resolve_optional_duration_ms(
            None,
            Some(450),
            "AZURE_COSMOS_TEST_OPTIONAL_DURATION_SET",
            100,
            500,
        )
        .unwrap();
        assert_eq!(value, Some(Duration::from_millis(450)));
    }
}
