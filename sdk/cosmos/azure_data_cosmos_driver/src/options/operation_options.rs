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
        ExcludedRegions, PriorityLevel, ReadConsistencyStrategy,
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

    /// Throughput-control tuning for this request.
    #[option(nested)]
    pub throughput_control: Option<ThroughputControlOptions>,

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
    /// Master switch that enables or disables cross-region read hedging.
    ///
    /// **Default**: `None`, which the driver treats as **enabled** — eligible
    /// requests are hedged using the built-in default threshold of
    /// `min(1000ms, request_timeout / 2)` (falling back to `1000ms`).
    ///
    /// **Environment variable**: `AZURE_COSMOS_HEDGING_ENABLED`. When set, it is
    /// the **source of truth** and takes precedence over the programmatic
    /// [`Self::availability_strategy`] in both directions:
    /// - `Some(false)` turns hedging off even when an explicit
    ///   [`AvailabilityStrategy::Hedging`] is configured.
    /// - `Some(true)` turns hedging on even when an explicit
    ///   [`AvailabilityStrategy::Disabled`] is configured; a programmatic
    ///   `Hedging(..)` strategy still supplies its custom threshold, otherwise
    ///   the default threshold above applies.
    ///
    /// Leaving it unset (`None`) defers to the programmatic strategy.
    ///
    /// **Kill switch**: `AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE` takes
    /// precedence over **every** layer (including a programmatic per-request
    /// value and [`Self::availability_strategy`]). It is intended as a
    /// fleet-wide incident override and should normally be left unset.
    #[option(env = "AZURE_COSMOS_HEDGING_ENABLED", overridable)]
    pub hedging_enabled: Option<bool>,

    /// Cross-region availability strategy controlling whether eligible
    /// requests are hedged to additional regions when the primary is slow.
    ///
    /// **Default**: `None` — the driver applies the built-in default
    /// strategy. Setting
    /// `Some(AvailabilityStrategy::Disabled)` at any layer turns hedging
    /// off for that scope.
    ///
    /// **Note**: This strategy is overridden by [`Self::hedging_enabled`]
    /// whenever the latter resolves to `Some(_)` (for example via
    /// `AZURE_COSMOS_HEDGING_ENABLED`): `Some(false)` forces hedging off and
    /// `Some(true)` forces it on, regardless of the strategy configured here.
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

/// Throughput-control tuning for an individual request (or layer default).
///
/// Mirrors the [`ThrottlingRetryOptions`] pattern: three independently
/// layered knobs grouped under a single nested option group on
/// [`OperationOptions`]. None of these fields read from environment
/// variables — throughput control is a per-application policy.
///
/// # Resolution
///
/// Each inner field participates independently in the standard runtime →
/// account → operation layered resolution. Once resolved, the driver
/// computes the wire headers (`x-ms-cosmos-priority-level`,
/// `x-ms-cosmos-throughput-bucket`) using a two-step rule per field:
///
/// 1. If the layered value for the field is `Some`, use it directly.
/// 2. Else, if [`group_name`](Self::group_name) resolves to a group
///    registered on the driver via
///    [`DriverOptionsBuilder::register_throughput_control_group`](crate::options::DriverOptionsBuilder::register_throughput_control_group),
///    use the group's value for the field (if the group sets it).
/// 3. Else, the header is omitted.
///
/// The two fields resolve independently, so a layered
/// `throughput_bucket = Some(...)` does not suppress a
/// `priority_level` carried by the registered group, and vice versa.
///
/// # Why direct overrides exist
///
/// The direct [`throughput_bucket`](Self::throughput_bucket) /
/// [`priority_level`](Self::priority_level) overrides let callers set the
/// per-operation headers without having to register a
/// [`ThroughputControlGroupOptions`](super::ThroughputControlGroupOptions)
/// on the driver. Use a registered group when you want shared, mutable
/// values to apply to a family of operations; use the direct fields for
/// one-off overrides.
#[derive(CosmosOptions, Clone, Debug)]
#[options(layers(runtime, account, operation))]
#[non_exhaustive]
pub struct ThroughputControlOptions {
    /// Name of a previously-registered throughput-control group.
    ///
    /// Used as the fallback source for
    /// [`throughput_bucket`](Self::throughput_bucket) and
    /// [`priority_level`](Self::priority_level) when those fields are not
    /// set at any layer. A name that does not resolve to a registered group
    /// produces an error at request time.
    pub group_name: Option<ThroughputControlGroupName>,

    /// Direct override for the `x-ms-cosmos-throughput-bucket` header.
    ///
    /// Takes precedence over the bucket carried by the resolved
    /// [`group_name`](Self::group_name) (if any). `None` falls back to the
    /// resolved group's bucket, then to no header.
    pub throughput_bucket: Option<u32>,

    /// Direct override for the `x-ms-cosmos-priority-level` header.
    ///
    /// Takes precedence over the priority carried by the resolved
    /// [`group_name`](Self::group_name) (if any). `None` falls back to the
    /// resolved group's priority level, then to no header.
    pub priority_level: Option<PriorityLevel>,
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
        assert!(options.throughput_control.is_none());
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

    /// Rule 2 + Rule 3 (RCS resolution):
    /// An explicit per-request `Default` overrides a client-level non-`Default`,
    /// resulting in no RCS being emitted on the wire.
    #[test]
    fn view_request_level_default_overrides_client_level_non_default() {
        use std::sync::Arc;

        let client = Arc::new(OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::LatestCommitted),
            ..Default::default()
        });

        let operation = OperationOptions {
            read_consistency_strategy: Some(ReadConsistencyStrategy::Default),
            ..Default::default()
        };

        let view = OperationOptionsView::new(None, Some(client), None, Some(&operation));

        assert_eq!(
            view.read_consistency_strategy(),
            Some(&ReadConsistencyStrategy::Default),
            "explicit request-level Default must override client-level non-Default"
        );
    }

    #[test]
    fn from_env_vars_parses_known_vars() {
        let options = OperationOptions::from_env_vars(|key| match key {
            "AZURE_COSMOS_READ_CONSISTENCY_STRATEGY" => Ok("Session".to_string()),
            "AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE" => Ok("true".to_string()),
            "AZURE_COSMOS_MAX_FAILOVER_RETRY_COUNT" => Ok("7".to_string()),
            "AZURE_COSMOS_MAX_SESSION_RETRY_COUNT" => Ok("3".to_string()),
            "AZURE_COSMOS_HEDGING_ENABLED" => Ok("false".to_string()),
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
        assert_eq!(options.hedging_enabled, Some(false));
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
        assert!(options.hedging_enabled.is_none());
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
    /// to the request-class compile-time defaults selected by the operation
    /// pipeline (metadata: `METADATA_MAX_THROTTLE_ATTEMPTS` /
    /// `METADATA_MAX_THROTTLE_WAIT`; data-plane: `DATA_PLANE_MAX_THROTTLE_ATTEMPTS`
    /// / `DATA_PLANE_MAX_THROTTLE_WAIT`).
    #[test]
    fn nested_throttling_retry_options_view_is_none_when_unset_at_every_layer() {
        let op = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op));
        let throttling = view.throttling_retry_options();

        assert!(throttling.max_retry_count().is_none());
        assert!(throttling.max_retry_wait_time().is_none());
    }

    /// The `env_override` kill-switch layer must win over the operation layer
    /// for an `overridable` field — this is the whole point of the
    /// `{ENV}_OVERRIDE` variant: a fleet-wide incident override that beats a
    /// hard-coded per-request value.
    #[test]
    fn env_override_layer_wins_over_operation_for_hedging_enabled() {
        use std::sync::Arc;

        // Override layer disables hedging.
        let env_override = Arc::new(OperationOptions {
            hedging_enabled: Some(false),
            ..Default::default()
        });

        // Operation layer tries to enable hedging.
        let operation = OperationOptions {
            hedging_enabled: Some(true),
            ..Default::default()
        };

        let view = OperationOptionsView::new_with_override(
            Some(env_override),
            None,
            None,
            None,
            Some(&operation),
        );

        assert_eq!(
            view.hedging_enabled(),
            Some(&false),
            "env_override must beat the operation layer for hedging_enabled",
        );
    }

    /// When the `env_override` layer leaves a field unset, resolution falls
    /// through to the normal layer chain (operation → … → env), so the
    /// kill switch is inert unless the `{ENV}_OVERRIDE` variant is set.
    #[test]
    fn env_override_unset_falls_through_to_operation() {
        let operation = OperationOptions {
            hedging_enabled: Some(true),
            ..Default::default()
        };

        // Override layer present but the field is None — must not mask the
        // operation value.
        let env_override = std::sync::Arc::new(OperationOptions::default());

        let view = OperationOptionsView::new_with_override(
            Some(env_override),
            None,
            None,
            None,
            Some(&operation),
        );

        assert_eq!(view.hedging_enabled(), Some(&true));
    }

    /// `from_env_override_vars` populates only the `overridable` fields from
    /// their `{ENV}_OVERRIDE` variants and leaves every other env field
    /// `None` (the base `from_env_vars` path is unaffected).
    #[test]
    fn from_env_override_vars_reads_only_override_variants() {
        let options = OperationOptions::from_env_override_vars(|key| match key {
            "AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE" => Ok("false".to_string()),
            // A non-override env var must be ignored by the override path.
            "AZURE_COSMOS_HEDGING_ENABLED" => Ok("true".to_string()),
            _ => Err(std::env::VarError::NotPresent),
        });

        assert_eq!(options.hedging_enabled, Some(false));
        // A non-overridable env field must stay None on the override layer.
        assert!(options.availability_strategy.is_none());
    }

    /// With nothing set, the override constructor produces an all-`None`
    /// instance.
    #[test]
    fn from_env_override_vars_returns_none_when_unset() {
        let options =
            OperationOptions::from_env_override_vars(|_| Err(std::env::VarError::NotPresent));
        assert!(options.hedging_enabled.is_none());
        assert!(options.availability_strategy.is_none());
    }

    /// Each inner field on the nested [`ThroughputControlOptions`] group must
    /// participate independently in the standard runtime → account →
    /// operation layered resolution. Mirrors the throttle equivalent so a
    /// later macro change can't silently regress this layering.
    #[test]
    fn nested_throughput_control_resolves_across_layers() {
        use std::sync::Arc;

        let runtime = Arc::new(OperationOptions {
            throughput_control: Some(ThroughputControlOptions {
                group_name: Some(ThroughputControlGroupName::new("runtime-group")),
                throughput_bucket: Some(7),
                priority_level: Some(PriorityLevel::Low),
            }),
            ..Default::default()
        });

        let operation = OperationOptions {
            throughput_control: Some(ThroughputControlOptions {
                group_name: None,
                throughput_bucket: Some(99),
                priority_level: None,
            }),
            ..Default::default()
        };

        let view = OperationOptionsView::new(None, Some(runtime), None, Some(&operation));
        let throughput = view.throughput_control();

        assert_eq!(
            throughput.group_name(),
            Some(&ThroughputControlGroupName::new("runtime-group")),
            "missing inner field on the operation layer must fall through to runtime",
        );
        assert_eq!(
            throughput.throughput_bucket(),
            Some(&99),
            "operation-layer override must win over runtime for `throughput_bucket`",
        );
        assert_eq!(
            throughput.priority_level(),
            Some(&PriorityLevel::Low),
            "missing inner field on the operation layer must fall through to runtime",
        );
    }

    /// When no layer sets `throughput_control`, the view's inner-field
    /// accessors must return `None` so the driver-side resolver knows to
    /// omit the wire headers.
    #[test]
    fn nested_throughput_control_view_is_none_when_unset_at_every_layer() {
        let op = OperationOptions::default();
        let view = OperationOptionsView::new(None, None, None, Some(&op));
        let throughput = view.throughput_control();

        assert!(throughput.group_name().is_none());
        assert!(throughput.throughput_bucket().is_none());
        assert!(throughput.priority_level().is_none());
    }
}

/// Smoke tests that the production `OperationOptions::from_env` path is wired
/// to the real process environment.
///
/// The exhaustive per-field matrix lives in the injected-closure tests above.
/// These two cases exist only to prove that the real production constructor
/// actually reads `std::env::var` — a gap the injected tests cannot cover — by
/// setting a real `AZURE_COSMOS_*` variable and observing it flow through. They
/// run inside [`with_scoped_env`] (shared lock + clear/restore) so they stay
/// hermetic.
#[cfg(test)]
mod real_env_tests {
    use super::*;
    use crate::options::env_parsing::test_env::{with_scoped_env, OPERATION_ENV_VARS};

    #[test]
    fn real_env_empty_yields_all_none() {
        // With no variables set, the env layer contributes nothing.
        with_scoped_env(OPERATION_ENV_VARS, &[], || {
            let o = OperationOptions::from_env();
            assert!(o.read_consistency_strategy.is_none());
            assert!(o.content_response_on_write.is_none());
            assert!(o.max_failover_retry_count.is_none());
            assert!(o.max_session_retry_count.is_none());
            assert!(o.hedging_enabled.is_none());
        });
    }

    #[test]
    fn real_env_value_flows_through() {
        // A real `AZURE_COSMOS_READ_CONSISTENCY_STRATEGY` flows through
        // `from_env`, proving the production accessor is connected.
        with_scoped_env(
            OPERATION_ENV_VARS,
            &[("AZURE_COSMOS_READ_CONSISTENCY_STRATEGY", "Session")],
            || {
                let o = OperationOptions::from_env();
                assert_eq!(
                    o.read_consistency_strategy,
                    Some(ReadConsistencyStrategy::Session)
                );
            },
        );
    }
}
