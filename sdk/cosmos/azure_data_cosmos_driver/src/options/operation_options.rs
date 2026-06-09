// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation options that participate in runtime/account/operation resolution.

use std::collections::HashMap;
use std::time::Duration;

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_data_cosmos_macros::CosmosOptions;

use crate::{
    models::ThroughputControlGroupName,
    options::{
        AvailabilityStrategy, ContentResponseOnWrite, EndToEndOperationLatencyPolicy,
        ExcludedRegions, ReadConsistencyStrategy,
    },
};

/// Options that apply to individual Cosmos DB requests.
///
/// `OperationOptions` controls cross-cutting concerns such as consistency, routing,
/// retry behavior, and custom headers. These settings can be specified at multiple
/// levels — each per-operation options type (e.g., `ItemReadOptions`)
/// has an `operation` field of this type.
///
/// # Layered Resolution
///
/// When the same option is set at multiple levels, the most specific value wins:
///
/// 1. **Operation** — set on the per-request options (highest priority)
/// 2. **Account** — set on `CosmosClientOptions` when building the client
/// 3. **Runtime** — application-wide defaults
/// 4. **Environment** — loaded from `AZURE_COSMOS_*` environment variables (lowest priority)
///
/// A field set to `None` means "inherit from a lower-priority level."
/// A field set to `Some(value)` overrides all lower levels.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct OperationOptions {
    /// Read consistency strategy for this request.
    ///
    /// Controls the consistency guarantee for read operations. Set to `None` to
    /// inherit the account or runtime default.
    #[option(env = "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY")]
    pub read_consistency_strategy: Option<ReadConsistencyStrategy>,

    /// Regions to exclude from request routing.
    ///
    /// When set, the SDK will not route this request to the specified regions.
    /// Set to `Some(empty)` to clear exclusions; `None` inherits from a lower level.
    pub excluded_regions: Option<ExcludedRegions>,

    /// Whether write responses include the resource body.
    ///
    /// [`ContentResponseOnWrite::Enabled`] returns the written resource in the response.
    /// [`ContentResponseOnWrite::Disabled`] suppresses the body to reduce bandwidth.
    /// `None` inherits from a lower level (default: disabled).
    #[option(env = "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE")]
    pub content_response_on_write: Option<ContentResponseOnWrite>,

    /// Throughput control group name for this request.
    ///
    /// References a group registered at runtime via
    /// [`CosmosDriverRuntimeBuilder::register_throughput_control_group()`](crate::driver::CosmosDriverRuntimeBuilder::register_throughput_control_group).
    ///
    /// `None` inherits from a lower-priority level or falls back to the
    /// container's default group.
    pub throughput_control_group: Option<ThroughputControlGroupName>,

    /// End-to-end timeout policy for this request.
    pub end_to_end_latency_policy: Option<EndToEndOperationLatencyPolicy>,

    /// Maximum number of region failover retries.
    #[option(env = "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT")]
    pub max_failover_retry_count: Option<u32>,

    /// How long an endpoint is considered unavailable after a failure.
    pub endpoint_unavailability_ttl: Option<Duration>,

    /// Disables automatic session token management.
    ///
    /// When `None` or `Some(false)`, session tokens are captured from responses
    /// and sent on subsequent requests for session consistency.
    /// Set to `Some(true)` to disable this behavior.
    pub session_capturing_disabled: Option<bool>,

    /// Maximum number of session-consistency retries on 404/1002 errors.
    #[option(env = "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT")]
    pub max_session_retry_count: Option<u32>,

    /// Retry behavior for requests throttled by the service (HTTP 429,
    /// rate-limited).
    ///
    /// Groups the throttle-retry knobs into a single option group, mirroring
    /// the .NET SDK's `ThrottlingRetryOptions` and the Java SDK's
    /// `ThrottlingRetryOptions`. See [`ThrottlingRetryOptions`] for the
    /// individual settings ([`max_retry_count`](ThrottlingRetryOptions::max_retry_count)
    /// and [`max_retry_wait_time`](ThrottlingRetryOptions::max_retry_wait_time)).
    ///
    /// Each inner setting resolves independently across the runtime → account
    /// → operation → environment layers. To bound the **total** time an
    /// operation can spend on retries (across throttling, failover, hedging,
    /// etc.), configure [`end_to_end_latency_policy`](Self::end_to_end_latency_policy).
    #[option(nested)]
    pub throttling_retry_options: Option<ThrottlingRetryOptions>,

    /// Read failure count threshold before the per-partition circuit breaker
    /// trips for a `(partition, region)` pair.
    ///
    /// **Default**: `10`. Counted within the
    /// `circuit_breaker_timeout_counter_reset_window_in_minutes` window.
    ///
    /// **Tuning**: Lower values trip the breaker faster, isolating bad regions
    /// sooner but at the cost of false positives during transient blips and
    /// more cross-region traffic. Higher values are more tolerant of
    /// short-lived issues but delay isolation of a genuinely unhealthy region
    /// (more user-visible failed reads before failover engages).
    #[option(env = "AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS")]
    pub circuit_breaker_failure_count_for_reads: Option<u32>,

    /// Write failure count threshold before the per-partition circuit breaker
    /// trips for a `(partition, region)` pair (multi-master accounts only).
    ///
    /// **Default**: `5`. Lower than the read threshold because writes are not
    /// retried as aggressively across regions and a stuck write region has a
    /// larger user-visible blast radius.
    ///
    /// **Tuning**: same trade-offs as
    /// `circuit_breaker_failure_count_for_reads`; only applies on accounts
    /// where multiple write locations are enabled.
    #[option(env = "AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES")]
    pub circuit_breaker_failure_count_for_writes: Option<u32>,

    /// Window (in minutes) after which the per-partition failure counters
    /// reset for a `(partition, region)` pair.
    ///
    /// **Default**: `5` minutes. Failures older than this window do not
    /// contribute to the trip threshold.
    ///
    /// **Tuning**: Shorter windows make the breaker more forgiving of
    /// occasional failures (less likely to trip from sparse, intermittent
    /// errors); longer windows accumulate evidence of chronic regional
    /// degradation that does not happen all at once.
    #[option(env = "AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES")]
    pub circuit_breaker_timeout_counter_reset_window_in_minutes: Option<u32>,

    /// Minimum age (in seconds) a tripped circuit breaker entry must reach
    /// before the background failback sweep is allowed to transition it from
    /// `Unhealthy` to `ProbeCandidate` (and thereby attempt failback to the
    /// original region).
    ///
    /// **Default**: `5` seconds (also the minimum permitted value).
    ///
    /// **Tuning**: Larger values keep traffic on the alternate region for
    /// longer once a failover has happened, reducing flapping when a region
    /// is recovering unevenly. Smaller values bring traffic back to the
    /// preferred region sooner but risk repeatedly probing a not-yet-healed
    /// region.
    #[option(env = "AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS")]
    pub allowed_partition_unavailability_duration_in_seconds: Option<u32>,

    /// Interval (in seconds) between iterations of the background failback
    /// sweep that promotes eligible `Unhealthy` entries to `ProbeCandidate`.
    ///
    /// **Default**: `300` seconds (5 minutes).
    ///
    /// **Tuning**: This is purely a polling interval; it places an upper
    /// bound on how long after `allowed_partition_unavailability_duration_in_seconds`
    /// a tripped entry has to wait before being eligible to probe back to its
    /// original region. Smaller values reduce that latency but raise the
    /// background scan rate; larger values do the opposite.
    #[option(env = "AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS")]
    pub ppcb_stale_partition_unavailability_refresh_interval_in_seconds: Option<u32>,

    /// Whether the per-partition circuit breaker (PPCB) is enabled.
    ///
    /// **Default**: `false`. PPCB tracks failures per
    /// `(partition_key_range_id, region)` and routes traffic to a healthy
    /// alternate region once the threshold is exceeded, then probes the
    /// original region for recovery via the failback sweep.
    ///
    /// **Tuning**: Enable to opt into partition-level circuit breaking on
    /// reads (any account) and writes (multi-master accounts). When disabled,
    /// the driver falls back to account-level endpoint marking, which is
    /// coarser-grained.
    #[option(env = "AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED")]
    pub per_partition_circuit_breaker_enabled: Option<bool>,

    /// Consecutive alternate-region hedge wins on the same
    /// `(partition, primary_region)` pair before the per-partition circuit
    /// breaker (PPCB) trips the partition away from that primary.
    ///
    /// **Default**: `5` (matches the .NET v3 SDK convention).
    ///
    /// **Tuning**: Lower values trip the partition faster when the primary
    /// region is chronically slow but the alternate is healthy — useful
    /// when operators want hedging to drive failover aggressively. Higher
    /// values are more tolerant of occasional latency spikes that the
    /// hedge happens to win, avoiding spurious failovers when both regions
    /// are healthy and the primary just happened to lose the race. Set
    /// well above `max_failover_retries` to effectively disable hedge-win
    /// driven trips while keeping the hedge race itself active.
    #[option(env = "AZURE_COSMOS_CONSECUTIVE_HEDGE_WIN_THRESHOLD")]
    pub consecutive_hedge_win_threshold: Option<u32>,

    /// Cross-region availability strategy controlling whether eligible
    /// requests are hedged to additional regions when the primary is slow.
    ///
    /// **Default**: `None` — the driver applies the built-in default
    /// strategy. Setting
    /// `Some(AvailabilityStrategy::Disabled)` at any layer turns hedging
    /// off for that scope.
    pub availability_strategy: Option<AvailabilityStrategy>,

    // Additional headers beyond those natively supported by the driver.
    // May be removed in the future as we analyze exactly what options are needed.
    pub custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
}

/// Retry behavior for requests throttled by the service (HTTP 429,
/// rate-limited).
///
/// Mirrors the .NET and Java SDKs' `ThrottlingRetryOptions`, grouping the two
/// throttle-retry knobs into a single option group instead of exposing them as
/// flat fields. Each setting participates independently in the standard
/// runtime → account → operation → environment layered resolution.
///
/// These limits bound the transport-level 429 retry loop, which honors the
/// service `x-ms-retry-after-ms` header (or an exponential-backoff fallback
/// when the header is absent).
///
/// # Scope
///
/// Both budgets apply *per transport-pipeline invocation*, not per logical
/// operation. An operation that performs cross-region failover or hedging can
/// call into the transport pipeline multiple times — each invocation starts
/// with a fresh throttle-retry budget. To bound the **total** time an
/// operation can spend on retries, configure
/// [`OperationOptions::end_to_end_latency_policy`].
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct ThrottlingRetryOptions {
    /// Maximum number of retries when a request is throttled by the service
    /// (HTTP 429, rate-limited).
    ///
    /// This is the analog of the .NET SDK's
    /// `MaxRetryAttemptsOnRateLimitedRequests` (and Java's
    /// `maxRetryAttemptsOnThrottledRequests`).
    ///
    /// **Default**: `9`. A value of `0` disables retrying throttled requests
    /// (the first 429 is surfaced to the caller).
    ///
    /// **Wire-request semantics**: a configured `max_retry_count = N`
    /// produces up to `N + 1` total HTTP requests on the wire (1 initial
    /// + up to N retries). The driver's one-shot forced-final-retry
    /// safety net is suppressed once the count budget is exhausted, so the
    /// count is the hard cap — matching .NET / Java parity. (The
    /// forced-final retry only fires when the cumulative-wait budget
    /// — see [`max_retry_wait_time`](Self::max_retry_wait_time) — is the
    /// limiter rather than the count.)
    #[option(env = "AZURE_COSMOS_MAX_THROTTLE_RETRY_COUNT")]
    pub max_retry_count: Option<u32>,

    /// Maximum cumulative time to spend waiting across throttle (HTTP 429)
    /// retries before giving up and surfacing the 429 to the caller.
    ///
    /// This is the analog of the .NET SDK's
    /// `MaxRetryWaitTimeOnRateLimitedRequests` (and Java's `maxRetryWaitTime`).
    /// Once the accumulated retry delay would exceed this budget, no further
    /// throttle retry is attempted.
    ///
    /// **Default**: 30 seconds.
    pub max_retry_wait_time: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_operation_options() {
        let options = OperationOptions::default();
        assert!(options.read_consistency_strategy.is_none());
        assert!(options.excluded_regions.is_none());
        assert!(options.content_response_on_write.is_none());
        assert!(options.throughput_control_group.is_none());
        assert!(options.max_failover_retry_count.is_none());
        assert!(options.max_session_retry_count.is_none());
    }

    #[test]
    fn builder_creates_options() {
        let throttling = ThrottlingRetryOptionsBuilder::new()
            .with_max_retry_count(4)
            .with_max_retry_wait_time(Duration::from_secs(12))
            .build();
        let options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .with_read_consistency_strategy(ReadConsistencyStrategy::Session)
            .with_max_failover_retry_count(5)
            .with_max_session_retry_count(3)
            .with_throttling_retry_options(throttling)
            .build();

        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(options.max_failover_retry_count, Some(5));
        assert_eq!(options.max_session_retry_count, Some(3));
        let throttling = options
            .throttling_retry_options
            .expect("throttling group should be set");
        assert_eq!(throttling.max_retry_count, Some(4));
        assert_eq!(
            throttling.max_retry_wait_time,
            Some(Duration::from_secs(12))
        );
    }

    #[test]
    fn view_resolves_across_layers() {
        use std::sync::Arc;

        let env = Arc::new(OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Eventual),
            max_failover_retry_count: Some(3),
            ..Default::default()
        });

        let runtime = Arc::new(OperationOptions {
            content_response_on_write: Some(ContentResponseOnWrite::Enabled),
            ..Default::default()
        });

        let account = Arc::new(OperationOptions {
            max_failover_retry_count: Some(5),
            content_response_on_write: Some(ContentResponseOnWrite::Disabled),
            ..Default::default()
        });

        let operation = OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Session),
            ..Default::default()
        };

        let view =
            OperationOptionsView::new(Some(env), Some(runtime), Some(account), Some(&operation));

        // Operation overrides env
        assert_eq!(
            view.read_consistency_strategy(),
            Some(&ReadConsistencyStrategy::Session)
        );
        // Account overrides runtime
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Disabled)
        );
        // Account overrides env
        assert_eq!(view.max_failover_retry_count(), Some(&5));
        // Not set anywhere
        assert!(view.excluded_regions().is_none());
        assert!(view.max_session_retry_count().is_none());
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = OperationOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY" => Ok("Session".to_string()),
            "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE" => Ok("true".to_string()),
            "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT" => Ok("7".to_string()),
            "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT" => Ok("3".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(
            options.read_consistency_strategy,
            Some(ReadConsistencyStrategy::Session)
        );
        assert_eq!(
            options.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
        assert_eq!(options.max_failover_retry_count, Some(7));
        assert_eq!(options.max_session_retry_count, Some(3));
        // Fields without env annotation remain None
        assert!(options.excluded_regions.is_none());
        // Nested option groups are not populated by the parent's `from_env`;
        // they are loaded separately at construction sites (see
        // `CosmosDriverRuntimeBuilder::build` and the
        // `throttling_retry_options_from_env` test below).
        assert!(options.throttling_retry_options.is_none());
    }

    #[test]
    fn throttling_retry_options_from_env() {
        let throttling = ThrottlingRetryOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_MAX_THROTTLE_RETRY_COUNT" => Ok("4".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(throttling.max_retry_count, Some(4));
        // `max_retry_wait_time` has no env var, so it stays None.
        assert!(throttling.max_retry_wait_time.is_none());
    }

    #[test]
    fn from_env_vars_returns_none_for_missing_vars() {
        let options = OperationOptions::from_env_vars(|_| Err(std::env::VarError::NotPresent));

        assert!(options.read_consistency_strategy.is_none());
        assert!(options.content_response_on_write.is_none());
        assert!(options.excluded_regions.is_none());
        assert!(options.max_failover_retry_count.is_none());
        assert!(options.max_session_retry_count.is_none());
        assert!(options.availability_strategy.is_none());
    }

    #[test]
    fn builder_round_trips_availability_strategy() {
        use crate::options::{HedgeThreshold, HedgingStrategy};
        use std::time::Duration;

        let strategy = AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(500)).unwrap(),
        ));

        let options = OperationOptionsBuilder::new()
            .with_availability_strategy(strategy)
            .build();

        assert_eq!(options.availability_strategy, Some(strategy));
    }

    #[test]
    fn builder_round_trips_disabled_availability_strategy() {
        let options = OperationOptionsBuilder::new()
            .with_availability_strategy(AvailabilityStrategy::Disabled)
            .build();

        assert_eq!(
            options.availability_strategy,
            Some(AvailabilityStrategy::Disabled)
        );
    }

    #[test]
    fn availability_strategy_resolves_via_view() {
        use crate::options::{HedgeThreshold, HedgingStrategy};
        use std::sync::Arc;
        use std::time::Duration;

        let account_strategy = AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(800)).unwrap(),
        ));
        let operation_strategy = AvailabilityStrategy::Disabled;

        let account = Arc::new(OperationOptions {
            availability_strategy: Some(account_strategy),
            ..Default::default()
        });

        let operation = OperationOptions {
            availability_strategy: Some(operation_strategy),
            ..Default::default()
        };

        let view_op_overrides =
            OperationOptionsView::new(None, None, Some(account.clone()), Some(&operation));
        assert_eq!(
            view_op_overrides.availability_strategy(),
            Some(&operation_strategy)
        );

        let empty_operation = OperationOptions::default();
        let view_account_wins =
            OperationOptionsView::new(None, None, Some(account), Some(&empty_operation));
        assert_eq!(
            view_account_wins.availability_strategy(),
            Some(&account_strategy)
        );
    }

    /// The nested [`ThrottlingRetryOptions`] group must participate in the
    /// standard runtime → account → operation → environment layered
    /// resolution on a *per-inner-field* basis. A finer-grained per-field
    /// guard than [`view_resolves_across_layers`] (which only covers flat
    /// fields), this test pins the contract that the
    /// [`OperationOptionsView::throttling_retry_options`] view walks every
    /// layer for each inner field independently.
    ///
    /// Regression guard: if the `#[option(nested)]` macro ever stopped
    /// recursing through layers for inner-field lookup, per-operation
    /// throttle overrides would silently inherit the runtime layer's value
    /// — visible end-to-end but invisible to the existing unit test suite.
    #[test]
    fn nested_throttling_retry_options_resolves_across_layers() {
        use std::sync::Arc;
        use std::time::Duration;

        // Runtime layer: both inner fields set.
        let runtime = Arc::new(OperationOptions {
            throttling_retry_options: Some(ThrottlingRetryOptions {
                max_retry_count: Some(9),
                max_retry_wait_time: Some(Duration::from_secs(15)),
            }),
            ..Default::default()
        });

        // Operation layer: only `max_retry_count` overridden; the inner
        // `max_retry_wait_time` is left `None` so the view should fall
        // through to the runtime layer for that one field.
        let operation = OperationOptions {
            throttling_retry_options: Some(ThrottlingRetryOptions {
                max_retry_count: Some(0),
                max_retry_wait_time: None,
            }),
            ..Default::default()
        };

        let view = OperationOptionsView::new(None, Some(runtime), None, Some(&operation));
        let throttling = view.throttling_retry_options();

        assert_eq!(
            throttling.max_retry_count(),
            Some(&0),
            "operation-layer override must win over runtime layer for `max_retry_count`",
        );
        assert_eq!(
            throttling.max_retry_wait_time(),
            Some(&Duration::from_secs(15)),
            "missing inner field on the operation layer must fall through to runtime",
        );
    }

    /// When *no* layer sets `throttling_retry_options`, the view's
    /// inner-field accessors must return `None` so the consumer falls back
    /// to the compile-time defaults (`DEFAULT_MAX_THROTTLE_ATTEMPTS` /
    /// `DEFAULT_MAX_THROTTLE_WAIT`).
    #[test]
    fn nested_throttling_retry_options_view_is_none_when_unset_at_every_layer() {
        let op = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op));
        let throttling = view.throttling_retry_options();

        assert!(throttling.max_retry_count().is_none());
        assert!(throttling.max_retry_wait_time().is_none());
    }
}
