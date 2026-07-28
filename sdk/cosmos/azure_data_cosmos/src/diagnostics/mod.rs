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
//! A built-in OpenTelemetry metrics handler,
//! [`CosmosMetricsHandler`], is available behind
//! the off-by-default `metrics` feature. It emits the stable
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
//! - [`CosmosTracingHandler`] — emits a backdated OpenTelemetry span tree
//!   (behind the off-by-default `distributed_tracing` feature), rate-limited so
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
pub use handler::{DiagnosticsHandler, DiagnosticsHandlerChain};
pub use logging::{SamplingLogHandler, TracingLogHandler};
pub use operation_context::CosmosOperationContext;
pub use rate_limiter::RateLimiterConfig;
#[cfg(feature = "distributed_tracing")]
pub use tracing::CosmosTracingHandler;

#[cfg(feature = "metrics")]
pub use metrics::{CosmosMetricsHandler, MetricsOptions};

// =========================================================================
// Internal modules
// =========================================================================

// Shared semantic-convention attribute-name literals, used by the metrics and
// distributed-tracing handlers (single source of truth). Only needed when one of
// those feature-gated handlers is compiled.
#[cfg(any(feature = "metrics", feature = "distributed_tracing"))]
pub(crate) mod attributes;

mod handler;
mod logging;
mod operation_context;
mod reason;
// Count-per-interval rate limiter shared by the sampling handlers (logging and,
// when enabled, tracing) so they can bound emission under an error storm.
pub(crate) mod rate_limiter;

#[cfg(feature = "metrics")]
pub mod metrics;

#[cfg(feature = "distributed_tracing")]
mod tracing;
