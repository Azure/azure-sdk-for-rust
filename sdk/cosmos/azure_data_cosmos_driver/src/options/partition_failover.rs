// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration for partition-level failover (PPAF) and the
//! per-partition circuit breaker (PPCB).

use std::time::Duration;

use super::env_parsing::{parse_duration_millis_from_env, parse_from_env, ValidationBounds};

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
    /// turns on when the account property is set even if this flag is `false`.
    pub fn circuit_breaker_enabled(&self) -> bool {
        self.circuit_breaker_enabled
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
        let defaults = PartitionFailoverOptions::default();

        let circuit_breaker_enabled = parse_from_env(
            self.circuit_breaker_enabled,
            "AZURE_COSMOS_PPCB_ENABLED",
            defaults.circuit_breaker_enabled,
            ValidationBounds::none(),
        )?;

        let read_failure_threshold = parse_from_env(
            self.read_failure_threshold,
            "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
            defaults.read_failure_threshold,
            ValidationBounds::min(1),
        )?;

        let write_failure_threshold = parse_from_env(
            self.write_failure_threshold,
            "AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD",
            defaults.write_failure_threshold,
            ValidationBounds::min(1),
        )?;

        let counter_reset_window = parse_duration_millis_from_env(
            self.counter_reset_window,
            "AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS",
            defaults.counter_reset_window.as_millis() as u64,
            1_000,
            u64::MAX,
        )?;

        let partition_unavailability_duration = parse_duration_millis_from_env(
            self.partition_unavailability_duration,
            "AZURE_COSMOS_PPCB_PARTITION_UNAVAILABILITY_DURATION_MS",
            defaults.partition_unavailability_duration.as_millis() as u64,
            1_000,
            u64::MAX,
        )?;

        let failback_sweep_interval = parse_duration_millis_from_env(
            self.failback_sweep_interval,
            "AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS",
            defaults.failback_sweep_interval.as_millis() as u64,
            1_000,
            u64::MAX,
        )?;

        let consecutive_hedge_win_threshold = parse_from_env(
            self.consecutive_hedge_win_threshold,
            "AZURE_COSMOS_PPCB_CONSECUTIVE_HEDGE_WIN_THRESHOLD",
            defaults.consecutive_hedge_win_threshold,
            ValidationBounds::min(1),
        )?;

        Ok(PartitionFailoverOptions {
            circuit_breaker_enabled,
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
        let options = PartitionFailoverOptionsBuilder::new().build().unwrap();

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
        let options = PartitionFailoverOptionsBuilder::new()
            .with_circuit_breaker_enabled(true)
            .with_read_failure_threshold(20)
            .with_write_failure_threshold(7)
            .with_counter_reset_window(Duration::from_secs(60))
            .with_partition_unavailability_duration(Duration::from_secs(30))
            .with_failback_sweep_interval(Duration::from_secs(120))
            .with_consecutive_hedge_win_threshold(3)
            .build()
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
    fn read_failure_threshold_zero_rejected() {
        let err = PartitionFailoverOptionsBuilder::new()
            .with_read_failure_threshold(0)
            .build()
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
            .build()
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
            .build()
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
            .build()
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("failback_sweep_interval_ms must be at least 1000ms"),
            "unexpected error: {err}",
        );
    }
}
