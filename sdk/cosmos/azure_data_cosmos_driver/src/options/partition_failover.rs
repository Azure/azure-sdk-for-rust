// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration for partition-level failover (PPAF) and the
//! per-partition circuit breaker (PPCB).

use std::time::Duration;

use super::env_parsing::{
    parse_duration_millis_from_env, parse_from_env, parse_optional_bool_from_env, ValidationBounds,
};

/// Configuration for partition-level failover and the per-partition circuit
/// breaker (PPCB).
///
/// These knobs are read **once** when the driver is constructed and are not
/// resolved per-operation. Use [`PartitionFailoverOptionsBuilder`] to build a
/// value, then attach it to
/// [`DriverOptions`](crate::options::DriverOptions) via
/// [`DriverOptionsBuilder::with_partition_failover_options`](crate::options::DriverOptionsBuilder::with_partition_failover_options).
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::PartitionFailoverOptions;
/// use std::time::Duration;
///
/// let options = PartitionFailoverOptions::builder()
///     .with_circuit_breaker_enabled(true)
///     .with_partition_unavailability_duration(Duration::from_secs(10))
///     .build()
///     .expect("valid options");
///
/// assert!(options.circuit_breaker_enabled());
/// assert_eq!(options.partition_unavailability_duration(), Duration::from_secs(10));
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct PartitionFailoverOptions {
    circuit_breaker_enabled: bool,
    circuit_breaker_enabled_override: Option<bool>,
    read_failure_threshold: u32,
    write_failure_threshold: u32,
    counter_reset_window: Duration,
    partition_unavailability_duration: Duration,
    failback_sweep_interval: Duration,
    consecutive_hedge_win_threshold: u32,
}

impl Default for PartitionFailoverOptions {
    fn default() -> Self {
        Self {
            circuit_breaker_enabled: true, // PPCB is enabled by default.
            circuit_breaker_enabled_override: None,
            read_failure_threshold: 10,
            write_failure_threshold: 5,
            counter_reset_window: Duration::from_millis(300_000),
            partition_unavailability_duration: Duration::from_millis(5_000),
            failback_sweep_interval: Duration::from_millis(300_000),
            consecutive_hedge_win_threshold: 5,
        }
    }
}

impl PartitionFailoverOptions {
    /// Creates a new builder for `PartitionFailoverOptions`.
    pub fn builder() -> PartitionFailoverOptionsBuilder {
        PartitionFailoverOptionsBuilder::new()
    }

    /// Returns whether the per-partition circuit breaker (PPCB) is enabled
    /// via driver options.
    ///
    /// The effective in-driver value is `enabled_via_options ||
    /// account_property_enable_per_partition_failover_behavior`, so PPCB still
    /// turns on when the account property is set even if this flag is `false`
    /// — unless the internal incident kill switch
    /// (`AZURE_COSMOS_PPCB_ENABLED_OVERRIDE`) is set, which wins over both.
    pub fn circuit_breaker_enabled(&self) -> bool {
        self.circuit_breaker_enabled
    }

    /// Returns the per-partition circuit breaker (PPCB) incident kill switch,
    /// if set via the `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` environment variable.
    ///
    /// When `Some(_)`, this value is **authoritative**: it overrides every
    /// other source of PPCB enablement — the base
    /// [`circuit_breaker_enabled`](Self::circuit_breaker_enabled) option **and**
    /// the account property `enable_per_partition_failover_behavior`. It exists
    /// as a process-wide operational safety valve so an operator can force PPCB
    /// on or off fleet-wide during a livesite incident without a code change or
    /// redeploy. `None` (the default) defers to the normal
    /// `option || account property` resolution.
    ///
    /// Read once when the driver runtime is built, not per request; flipping it
    /// mid-incident requires a process restart.
    ///
    /// Internal: the kill switch is operator-facing (env-only) and is not part
    /// of the public configuration surface, so this accessor is crate-private.
    pub(crate) fn circuit_breaker_enabled_override(&self) -> Option<bool> {
        self.circuit_breaker_enabled_override
    }

    /// Returns the read failure count threshold before the per-partition
    /// circuit breaker trips for a `(partition, region)` pair.
    pub fn read_failure_threshold(&self) -> u32 {
        self.read_failure_threshold
    }

    /// Returns the write failure count threshold before the per-partition
    /// circuit breaker trips for a `(partition, region)` pair (multi-master
    /// accounts only).
    pub fn write_failure_threshold(&self) -> u32 {
        self.write_failure_threshold
    }

    /// Returns the window after which the per-partition failure counters
    /// reset for a `(partition, region)` pair.
    pub fn counter_reset_window(&self) -> Duration {
        self.counter_reset_window
    }

    /// Returns the minimum age a tripped circuit breaker entry must reach
    /// before the background failback sweep is allowed to transition it from
    /// `Unhealthy` to `ProbeCandidate`.
    pub fn partition_unavailability_duration(&self) -> Duration {
        self.partition_unavailability_duration
    }

    /// Returns the interval between iterations of the background failback
    /// sweep that promotes eligible `Unhealthy` entries to `ProbeCandidate`.
    pub fn failback_sweep_interval(&self) -> Duration {
        self.failback_sweep_interval
    }

    /// Returns the number of consecutive alternate-region hedge wins on the
    /// same `(partition, primary_region)` pair before the per-partition
    /// circuit breaker trips the partition away from that primary.
    pub fn consecutive_hedge_win_threshold(&self) -> u32 {
        self.consecutive_hedge_win_threshold
    }
}

/// Builder for [`PartitionFailoverOptions`].
///
/// Unset fields are populated from environment variables when available,
/// and otherwise fall back to compile-time defaults.
///
/// # Environment Variables
///
/// - `AZURE_COSMOS_PPCB_ENABLED`: Enables PPCB via driver options (default:
///   `true`).
/// - `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE`: Incident kill switch that wins over
///   **every** other source of PPCB enablement — the base option **and** the
///   account property `enable_per_partition_failover_behavior`. Inert unless
///   set; intended for fleet-wide livesite incident response without a code
///   change. Boolean values are parsed leniently (`true`/`false`, `1`/`0`,
///   `yes`/`no`, `on`/`off`, case-insensitive).
/// - `AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD`: Read failure count before
///   the breaker trips (default: `10`, min: `1`).
/// - `AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD`: Write failure count before
///   the breaker trips (default: `5`, min: `1`).
/// - `AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS`: Window after which the
///   per-`(partition, region)` failure counters reset (default: `300_000` ms,
///   min: `1_000` ms).
/// - `AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS`: Minimum time
///   a tripped entry must remain `Unhealthy` before failback is eligible to
///   probe (default: `5_000` ms, min: `1_000` ms).
/// - `AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS`: Interval between
///   background failback sweep iterations (default: `300_000` ms,
///   min: `1_000` ms).
/// - `AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD`: Consecutive
///   alternate-region hedge wins on the same `(partition, primary_region)`
///   pair before PPCB trips the partition (default: `5`, min: `1`).
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::PartitionFailoverOptions;
///
/// let options = PartitionFailoverOptions::builder()
///     .with_circuit_breaker_enabled(true)
///     .build()
///     .expect("valid options");
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct PartitionFailoverOptionsBuilder {
    circuit_breaker_enabled: Option<bool>,
    circuit_breaker_enabled_override: Option<bool>,
    read_failure_threshold: Option<u32>,
    write_failure_threshold: Option<u32>,
    counter_reset_window: Option<Duration>,
    partition_unavailability_duration: Option<Duration>,
    failback_sweep_interval: Option<Duration>,
    consecutive_hedge_win_threshold: Option<u32>,
}

impl PartitionFailoverOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables the per-partition circuit breaker (PPCB) via
    /// driver options. Defaults to `true`.
    ///
    /// PPCB still turns on when the account property
    /// `enable_per_partition_failover_behavior` is set on the server, even
    /// if this value is `false`.
    pub fn with_circuit_breaker_enabled(mut self, value: bool) -> Self {
        self.circuit_breaker_enabled = Some(value);
        self
    }

    /// Test-only setter for the PPCB incident kill switch.
    ///
    /// Not part of the public API: the kill switch is operator-facing and is set
    /// exclusively via the `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` environment
    /// variable. This helper exists only so in-crate tests can exercise the
    /// authoritative-override resolution without mutating process-wide env.
    #[cfg(test)]
    pub(crate) fn with_circuit_breaker_enabled_override(mut self, value: bool) -> Self {
        self.circuit_breaker_enabled_override = Some(value);
        self
    }

    /// Sets the read failure count threshold before the per-partition
    /// circuit breaker trips for a `(partition, region)` pair.
    ///
    /// Default: `10`. Counted within
    /// [`with_counter_reset_window`](Self::with_counter_reset_window).
    pub fn with_read_failure_threshold(mut self, value: u32) -> Self {
        self.read_failure_threshold = Some(value);
        self
    }

    /// Sets the write failure count threshold before the per-partition
    /// circuit breaker trips for a `(partition, region)` pair (multi-master
    /// accounts only).
    ///
    /// Default: `5`. Counted within
    /// [`with_counter_reset_window`](Self::with_counter_reset_window).
    pub fn with_write_failure_threshold(mut self, value: u32) -> Self {
        self.write_failure_threshold = Some(value);
        self
    }

    /// Sets the window after which the per-partition failure counters reset
    /// for a `(partition, region)` pair.
    ///
    /// Default: 5 minutes. Failures older than this window do not contribute
    /// to the trip threshold.
    pub fn with_counter_reset_window(mut self, value: Duration) -> Self {
        self.counter_reset_window = Some(value);
        self
    }

    /// Sets the minimum age a tripped circuit breaker entry must reach
    /// before the background failback sweep is allowed to transition it
    /// from `Unhealthy` to `ProbeCandidate` (and thereby attempt failback
    /// to the original region).
    ///
    /// Default: 5 seconds.
    pub fn with_partition_unavailability_duration(mut self, value: Duration) -> Self {
        self.partition_unavailability_duration = Some(value);
        self
    }

    /// Sets the interval between iterations of the background failback
    /// sweep that promotes eligible `Unhealthy` entries to `ProbeCandidate`.
    ///
    /// Default: 5 minutes.
    pub fn with_failback_sweep_interval(mut self, value: Duration) -> Self {
        self.failback_sweep_interval = Some(value);
        self
    }

    /// Sets the number of consecutive alternate-region hedge wins on the
    /// same `(partition, primary_region)` pair before the per-partition
    /// circuit breaker trips the partition away from that primary.
    ///
    /// Default: `5` (matches the .NET v3 SDK convention).
    pub fn with_consecutive_hedge_win_threshold(mut self, value: u32) -> Self {
        self.consecutive_hedge_win_threshold = Some(value);
        self
    }

    /// Builds the [`PartitionFailoverOptions`] with configured values.
    ///
    /// Unset values are populated from environment variables or fall back to
    /// sensible defaults. See the [`PartitionFailoverOptionsBuilder`] docs
    /// for the full list of recognized environment variables and bounds.
    ///
    /// # Errors
    ///
    /// Returns an error if any environment variable cannot be parsed or any
    /// supplied value falls outside its validation bounds.
    pub fn build(self) -> crate::error::Result<PartitionFailoverOptions> {
        self.build_from_env(&|k| std::env::var(k).ok())
    }

    /// Builds the options, resolving unset fields through the supplied
    /// environment accessor instead of the process environment directly.
    ///
    /// `build()` delegates here with `|k| std::env::var(k).ok()`. The seam
    /// exists so the driver-options layer can share one env lookup across all
    /// option groups, and so tests can exercise every `builder × env` override
    /// combination deterministically without mutating process-wide environment
    /// (which would race across parallel tests).
    pub(crate) fn build_from_env(
        self,
        get_env: &dyn Fn(&str) -> Option<String>,
    ) -> crate::error::Result<PartitionFailoverOptions> {
        let defaults = PartitionFailoverOptions::default();

        let circuit_breaker_enabled = parse_from_env(
            self.circuit_breaker_enabled,
            "AZURE_COSMOS_PPCB_ENABLED",
            defaults.circuit_breaker_enabled,
            ValidationBounds::none(),
            get_env,
        )?;

        // Incident kill switch — lenient bool, builder wins over env, and an
        // unrecognized env value is ignored (treated as unset) so a typo can't
        // silently flip the switch the wrong way. Authoritative over both the
        // base option and the account property; applied in `LocationStateStore`.
        let circuit_breaker_enabled_override = parse_optional_bool_from_env(
            self.circuit_breaker_enabled_override,
            "AZURE_COSMOS_PPCB_ENABLED_OVERRIDE",
            get_env,
        );

        let read_failure_threshold = parse_from_env(
            self.read_failure_threshold,
            "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
            defaults.read_failure_threshold,
            ValidationBounds::min(1),
            get_env,
        )?;

        let write_failure_threshold = parse_from_env(
            self.write_failure_threshold,
            "AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD",
            defaults.write_failure_threshold,
            ValidationBounds::min(1),
            get_env,
        )?;

        let counter_reset_window = parse_duration_millis_from_env(
            self.counter_reset_window,
            "AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS",
            defaults.counter_reset_window.as_millis() as u64,
            1_000,
            u64::MAX,
            get_env,
        )?;

        let partition_unavailability_duration = parse_duration_millis_from_env(
            self.partition_unavailability_duration,
            "AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS",
            defaults.partition_unavailability_duration.as_millis() as u64,
            1_000,
            u64::MAX,
            get_env,
        )?;

        let failback_sweep_interval = parse_duration_millis_from_env(
            self.failback_sweep_interval,
            "AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS",
            defaults.failback_sweep_interval.as_millis() as u64,
            1_000,
            u64::MAX,
            get_env,
        )?;

        let consecutive_hedge_win_threshold = parse_from_env(
            self.consecutive_hedge_win_threshold,
            "AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD",
            defaults.consecutive_hedge_win_threshold,
            ValidationBounds::min(1),
            get_env,
        )?;

        Ok(PartitionFailoverOptions {
            circuit_breaker_enabled,
            circuit_breaker_enabled_override,
            read_failure_threshold,
            write_failure_threshold,
            counter_reset_window,
            partition_unavailability_duration,
            failback_sweep_interval,
            consecutive_hedge_win_threshold,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults_match_documented_values() {
        // Resolve against an explicitly empty environment so the documented
        // compile-time defaults are asserted deterministically, regardless of
        // any ambient `AZURE_COSMOS_PPCB_*` value. (Real-environment resolution
        // is covered by the `real_env_tests` module below.)
        let options = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&|_| None)
            .unwrap();

        assert!(options.circuit_breaker_enabled());
        assert_eq!(options.read_failure_threshold(), 10);
        assert_eq!(options.write_failure_threshold(), 5);
        assert_eq!(options.counter_reset_window(), Duration::from_secs(5 * 60));
        assert_eq!(
            options.partition_unavailability_duration(),
            Duration::from_secs(5)
        );
        assert_eq!(options.failback_sweep_interval(), Duration::from_secs(300));
        assert_eq!(options.consecutive_hedge_win_threshold(), 5);
    }

    #[test]
    fn builder_round_trips_custom_values() {
        // Resolve against an empty environment so the builder values are the
        // only inputs (and the test can't race the env-mutating
        // `real_env_tests`).
        let options = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled(true)
            .with_read_failure_threshold(20)
            .with_write_failure_threshold(7)
            .with_counter_reset_window(Duration::from_secs(60))
            .with_partition_unavailability_duration(Duration::from_secs(30))
            .with_failback_sweep_interval(Duration::from_secs(120))
            .with_consecutive_hedge_win_threshold(3)
            .build_from_env(&|_| None)
            .unwrap();

        assert!(options.circuit_breaker_enabled());
        assert_eq!(options.read_failure_threshold(), 20);
        assert_eq!(options.write_failure_threshold(), 7);
        assert_eq!(options.counter_reset_window(), Duration::from_secs(60));
        assert_eq!(
            options.partition_unavailability_duration(),
            Duration::from_secs(30)
        );
        assert_eq!(options.failback_sweep_interval(), Duration::from_secs(120));
        assert_eq!(options.consecutive_hedge_win_threshold(), 3);
    }

    #[test]
    fn circuit_breaker_override_unset_by_default() {
        // Empty environment → the kill switch is unset. (Real-environment
        // resolution of the override is covered by `real_env_tests`.)
        let options = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&|_| None)
            .unwrap();
        assert_eq!(options.circuit_breaker_enabled_override(), None);
    }

    #[test]
    fn circuit_breaker_override_builder_value_round_trips() {
        // Empty env so the *other* (unset) PPCB vars can't leak in from a
        // concurrently-running env-mutating test and fail bounds validation.
        let off = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled_override(false)
            .build_from_env(&|_| None)
            .unwrap();
        assert_eq!(off.circuit_breaker_enabled_override(), Some(false));

        let on = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled_override(true)
            .build_from_env(&|_| None)
            .unwrap();
        assert_eq!(on.circuit_breaker_enabled_override(), Some(true));
    }

    #[test]
    fn read_failure_threshold_zero_rejected() {
        let err = PartitionFailoverOptionsBuilder::new()
            .with_read_failure_threshold(0)
            .build_from_env(&|_| None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("read_failure_threshold must be at least 1"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn counter_reset_window_below_min_rejected() {
        let err = PartitionFailoverOptionsBuilder::new()
            .with_counter_reset_window(Duration::from_millis(500))
            .build_from_env(&|_| None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("counter_reset_window_ms must be at least 1000ms"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn partition_unavailability_duration_below_min_rejected() {
        let err = PartitionFailoverOptionsBuilder::new()
            .with_partition_unavailability_duration(Duration::from_millis(50))
            .build_from_env(&|_| None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("partition_unavailability_duration_ms must be at least 1000ms"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn failback_sweep_interval_below_min_rejected() {
        let err = PartitionFailoverOptionsBuilder::new()
            .with_failback_sweep_interval(Duration::from_millis(100))
            .build_from_env(&|_| None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("failback_sweep_interval_ms must be at least 1000ms"),
            "unexpected error: {err}",
        );
    }
}

/// Exhaustive `builder × environment` resolution matrix for
/// [`PartitionFailoverOptionsBuilder::build_from_env`].
///
/// Each case drives the builder through a deterministic, injected environment
/// map (no process-wide `std::env` mutation, so the cases are parallel-safe)
/// and asserts the resolved value for every field. The goal is to pin down the
/// precedence contract — **builder value > environment value > compile-time
/// default** — and the parsing quirks (strict vs. lenient booleans, bounds
/// validation, fail-soft on garbage) in all the "weird" combinations a fuzzing
/// consumer might throw at it.
#[cfg(test)]
mod env_matrix_tests {
    use super::*;
    use std::collections::HashMap;

    /// Builds an env accessor from a slice of `(name, value)` pairs.
    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        move |k: &str| map.get(k).cloned()
    }

    /// An env accessor that always returns `None` (nothing set).
    fn empty_env() -> impl Fn(&str) -> Option<String> {
        |_: &str| None
    }

    // ── Master switch: AZURE_COSMOS_PPCB_ENABLED ────────────────────────────

    #[test]
    fn enabled_defaults_true_when_nothing_set() {
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&empty_env())
            .unwrap();
        assert!(o.circuit_breaker_enabled());
    }

    #[test]
    fn enabled_env_false_honored_when_builder_unset() {
        // The customer's exact scenario: env says false, caller never set it.
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED", "false")]))
            .unwrap();
        assert!(!o.circuit_breaker_enabled());
    }

    #[test]
    fn enabled_builder_true_wins_over_env_false() {
        let o = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled(true)
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED", "false")]))
            .unwrap();
        assert!(o.circuit_breaker_enabled());
    }

    #[test]
    fn enabled_builder_false_wins_over_env_true() {
        let o = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled(false)
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED", "true")]))
            .unwrap();
        assert!(!o.circuit_breaker_enabled());
    }

    #[test]
    fn enabled_env_uses_strict_bool_parsing() {
        // `AZURE_COSMOS_PPCB_ENABLED` parses via `bool::from_str`, which only
        // accepts exactly "true"/"false" (case-sensitive, no trimming). Every
        // other spelling is fail-soft ignored and falls back to the default
        // (true). The *kill switch* is the lenient one — verified separately.
        for garbage in ["1", "yes", "on", "TRUE", "False", " false ", "enabled", ""] {
            let o = PartitionFailoverOptionsBuilder::new()
                .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED", garbage)]))
                .unwrap();
            assert!(
                o.circuit_breaker_enabled(),
                "{garbage:?} should be ignored and fall back to default true",
            );
        }
    }

    // ── Kill switch: AZURE_COSMOS_PPCB_ENABLED_OVERRIDE ─────────────────────

    #[test]
    fn override_unset_by_default() {
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&empty_env())
            .unwrap();
        assert_eq!(o.circuit_breaker_enabled_override(), None);
    }

    #[test]
    fn override_env_lenient_true_spellings() {
        for raw in ["true", "1", "yes", "on", "TRUE", "On", "  yes  "] {
            let o = PartitionFailoverOptionsBuilder::new()
                .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", raw)]))
                .unwrap();
            assert_eq!(
                o.circuit_breaker_enabled_override(),
                Some(true),
                "{raw:?} should parse as Some(true)",
            );
        }
    }

    #[test]
    fn override_env_lenient_false_spellings() {
        for raw in ["false", "0", "no", "off", "FALSE", "Off", "  no  "] {
            let o = PartitionFailoverOptionsBuilder::new()
                .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", raw)]))
                .unwrap();
            assert_eq!(
                o.circuit_breaker_enabled_override(),
                Some(false),
                "{raw:?} should parse as Some(false)",
            );
        }
    }

    #[test]
    fn override_env_garbage_ignored() {
        // An unrecognized override value is treated as unset so an operator
        // typo can't silently flip the incident kill switch the wrong way.
        for raw in ["maybe", "2", "", "tru", "noo"] {
            let o = PartitionFailoverOptionsBuilder::new()
                .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", raw)]))
                .unwrap();
            assert_eq!(
                o.circuit_breaker_enabled_override(),
                None,
                "{raw:?} should be ignored (treated as unset)",
            );
        }
    }

    #[test]
    fn override_is_independent_of_base_enable() {
        // The base flag and the override resolve from independent variables;
        // setting one must not perturb the other. Here base=false (env) while
        // override=true (env) — both observed verbatim. The *effective* PPCB
        // value (override wins) is resolved later in the routing layer, which
        // is covered by `partition_endpoint_state` tests.
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[
                ("AZURE_COSMOS_PPCB_ENABLED", "false"),
                ("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", "true"),
            ]))
            .unwrap();
        assert!(!o.circuit_breaker_enabled());
        assert_eq!(o.circuit_breaker_enabled_override(), Some(true));
    }

    #[test]
    fn override_builder_wins_over_env() {
        let o = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled_override(false)
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", "true")]))
            .unwrap();
        assert_eq!(o.circuit_breaker_enabled_override(), Some(false));
    }

    // ── Numeric thresholds: builder > env > default, with bounds ────────────

    #[test]
    fn thresholds_env_values_honored() {
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[
                ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "20"),
                ("AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD", "8"),
                ("AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD", "3"),
            ]))
            .unwrap();
        assert_eq!(o.read_failure_threshold(), 20);
        assert_eq!(o.write_failure_threshold(), 8);
        assert_eq!(o.consecutive_hedge_win_threshold(), 3);
    }

    #[test]
    fn thresholds_builder_wins_over_env() {
        let o = PartitionFailoverOptionsBuilder::new()
            .with_read_failure_threshold(99)
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
                "20",
            )]))
            .unwrap();
        assert_eq!(o.read_failure_threshold(), 99);
    }

    #[test]
    fn thresholds_default_when_env_unparseable() {
        // Garbage numeric env is fail-soft: logged and ignored, falls back to
        // the compile-time default rather than erroring.
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
                "not-a-number",
            )]))
            .unwrap();
        assert_eq!(o.read_failure_threshold(), 10);
    }

    #[test]
    fn threshold_env_zero_is_out_of_bounds_error() {
        // A *parseable* but out-of-bounds env value is a hard error (unlike
        // unparseable garbage, which is ignored).
        let err = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
                "0",
            )]))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("read_failure_threshold must be at least 1"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn threshold_builder_zero_is_out_of_bounds_even_with_valid_env() {
        // The builder value bypasses the env but is still bounds-validated.
        let err = PartitionFailoverOptionsBuilder::new()
            .with_write_failure_threshold(0)
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD",
                "5",
            )]))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("write_failure_threshold must be at least 1"),
            "unexpected error: {err}",
        );
    }

    // ── Duration knobs (millis): builder > env > default, with min bound ────

    #[test]
    fn durations_env_values_honored() {
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[
                ("AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS", "60000"),
                (
                    "AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS",
                    "30000",
                ),
                ("AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS", "120000"),
            ]))
            .unwrap();
        assert_eq!(o.counter_reset_window(), Duration::from_millis(60_000));
        assert_eq!(
            o.partition_unavailability_duration(),
            Duration::from_millis(30_000)
        );
        assert_eq!(o.failback_sweep_interval(), Duration::from_millis(120_000));
    }

    #[test]
    fn duration_builder_wins_over_env() {
        let o = PartitionFailoverOptionsBuilder::new()
            .with_counter_reset_window(Duration::from_secs(90))
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS",
                "60000",
            )]))
            .unwrap();
        assert_eq!(o.counter_reset_window(), Duration::from_secs(90));
    }

    #[test]
    fn duration_env_below_min_is_error() {
        let err = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS",
                "500",
            )]))
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("counter_reset_window_ms must be at least 1000ms"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn duration_env_unparseable_falls_back_to_default() {
        let o = PartitionFailoverOptionsBuilder::new()
            .build_from_env(&env_of(&[(
                "AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS",
                "soon",
            )]))
            .unwrap();
        assert_eq!(o.failback_sweep_interval(), Duration::from_millis(300_000));
    }

    // ── Everything at once, in the weirdest mixed combination ───────────────

    #[test]
    fn kitchen_sink_mixed_builder_env_and_defaults() {
        // builder: read threshold + override(false)
        // env:     enabled=false, write threshold, garbage hedge-win (ignored),
        //          counter-reset duration, unparseable unavailability (ignored)
        // default: failback sweep, consecutive hedge wins, unavailability dur.
        let o = PartitionFailoverOptionsBuilder::new()
            .with_read_failure_threshold(33)
            .with_circuit_breaker_enabled_override(false)
            .build_from_env(&env_of(&[
                ("AZURE_COSMOS_PPCB_ENABLED", "false"),
                ("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", "true"), // builder wins
                ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "11"), // builder wins
                ("AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD", "9"),
                ("AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD", "xx"), // ignored
                ("AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS", "45000"),
                (
                    "AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS",
                    "later",
                ), // ignored
            ]))
            .unwrap();

        assert!(!o.circuit_breaker_enabled()); // env false, builder unset
        assert_eq!(o.circuit_breaker_enabled_override(), Some(false)); // builder wins
        assert_eq!(o.read_failure_threshold(), 33); // builder wins over env 11
        assert_eq!(o.write_failure_threshold(), 9); // env
        assert_eq!(o.consecutive_hedge_win_threshold(), 5); // env garbage -> default
        assert_eq!(o.counter_reset_window(), Duration::from_millis(45_000)); // env
        assert_eq!(
            o.partition_unavailability_duration(),
            Duration::from_secs(5)
        ); // env garbage -> default
        assert_eq!(o.failback_sweep_interval(), Duration::from_secs(300)); // default
    }
}

/// Smoke tests that the production [`PartitionFailoverOptionsBuilder::build`]
/// path is wired to the real process environment.
///
/// The exhaustive `builder × env` precedence matrix lives in `env_matrix_tests`
/// (which injects a closure for determinism and parallel-safety). These two
/// cases exist only to prove that the public `build()` actually reads
/// `std::env::var` — a gap the injected tests cannot cover — by setting a real
/// `AZURE_COSMOS_PPCB_*` variable and observing it flow through. They run inside
/// [`with_scoped_env`] (shared lock + clear/restore) so they stay hermetic.
#[cfg(test)]
mod real_env_tests {
    use super::*;
    use crate::options::env_parsing::test_env::{with_scoped_env, PPCB_ENV_VARS};

    #[test]
    fn real_env_enable_false_disables_ppcb() {
        // A real `AZURE_COSMOS_PPCB_ENABLED=false` flows through `build()`.
        with_scoped_env(
            PPCB_ENV_VARS,
            &[("AZURE_COSMOS_PPCB_ENABLED", "false")],
            || {
                let o = PartitionFailoverOptionsBuilder::new().build().unwrap();
                assert!(!o.circuit_breaker_enabled());
            },
        );
    }

    #[test]
    fn real_env_empty_uses_default_enabled() {
        // With no PPCB variables set, `build()` yields the documented defaults.
        with_scoped_env(PPCB_ENV_VARS, &[], || {
            let o = PartitionFailoverOptionsBuilder::new().build().unwrap();
            assert!(o.circuit_breaker_enabled());
            assert_eq!(o.circuit_breaker_enabled_override(), None);
        });
    }
}
