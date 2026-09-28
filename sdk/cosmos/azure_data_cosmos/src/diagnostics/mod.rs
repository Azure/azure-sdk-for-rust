// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-operation diagnostics surfaced by the Cosmos DB SDK.
//!
//! Every fallible Cosmos operation produces a [`DiagnosticsContext`] capturing
//! request tracking, retries, regions contacted, and other observability
//! signals from the request pipeline. The context is reachable from
//! [`CosmosError`](crate::CosmosError) on failure, and from the
//! [`FeedPage`](crate::feed::FeedPage), [`ItemResponse`](crate::models::ItemResponse), and
//! similar response wrappers on success.
//!
//! The SDK also exposes an emission extension point on top of that context: a
//! [`DiagnosticsHandler`] receives each operation's completed
//! [`DiagnosticsContext`], and an ordered [`DiagnosticsHandlerChain`] invokes
//! registered handlers once per operation at completion. Register handlers via
//! [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
//! With no handlers registered the chain does nothing beyond checking whether a
//! handler is present.
//!
//! A built-in OpenTelemetry metrics handler, `CosmosMetricsHandler`, is available
//! behind the off-by-default `preview_opentelemetry` feature. It emits the stable
//! `db.client.operation.duration` histogram (and, opt-in, development-tier
//! metrics) from each completed context.
//!
//! Two further built-in handlers layer telemetry on top of the chain, both driven by
//! tail-based sampling against [`DiagnosticsThresholds`] (emit only for failed or
//! threshold-breaching operations):
//!
//! - [`SamplingLogHandler`] — a composable wrapper that applies the sampling
//!   gate plus a per-window rate limit and delegates emission to an inner
//!   handler, defaulting to a [`TracingLogHandler`] that logs a compact
//!   diagnostics line through the [`tracing`](https://docs.rs/tracing) ecosystem.
//! - `CosmosTracingHandler` — emits a backdated OpenTelemetry span tree
//!   (behind the off-by-default `preview_opentelemetry` feature), rate-limited so
//!   an error storm can't overwhelm exporters.

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::diagnostics::{
    DiagnosticsContext, ThresholdBreach, TransportKind,
};
#[doc(inline)]
pub use azure_data_cosmos_driver::DiagnosticsThresholds;
pub use handler::{
    ClientLifetimeToken, CosmosClientInfo, DiagnosticsHandler, DiagnosticsHandlerChain,
};
pub use logging::{SamplingLogHandler, TracingLogHandler};
pub use operation_context::CosmosOperationContext;
pub use rate_limiter::RateLimiterConfig;
pub use region::{RequestedRegion, RequestedRegionReason};
#[cfg(feature = "preview_opentelemetry")]
pub use tracing::{
    CosmosTracingHandler, CosmosTracingHandlerBuilder, CosmosTracingHandlerWithTracer,
};

#[cfg(feature = "preview_opentelemetry")]
pub use metrics::CosmosMetricsHandler;
pub use metrics::MetricsOptions;

// =========================================================================
// Internal modules
// =========================================================================

// Shared semantic-convention attribute-name literals for metrics and tracing.
pub(crate) mod attributes;

mod handler;
mod logging;
mod operation_context;
mod reason;
mod region;
// Count-per-interval rate limiter shared by the sampling handlers (logging and,
// when enabled, tracing) so they can bound emission under an error storm.
pub(crate) mod rate_limiter;

pub mod metrics;

#[cfg(feature = "preview_opentelemetry")]
mod tracing;
