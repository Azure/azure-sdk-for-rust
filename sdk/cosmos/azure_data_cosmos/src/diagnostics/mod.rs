// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-operation diagnostics surfaced by the Cosmos DB SDK.
//!
//! A completed Cosmos operation can provide a [`DiagnosticsContext`] capturing
//! request tracking, retries, and regions contacted. On failure, retrieve it
//! from [`CosmosError::diagnostics()`](crate::CosmosError::diagnostics) when available.
//! On success, retrieve it from the
//! [`FeedPage`](crate::feed::FeedPage), [`ItemResponse`](crate::models::ItemResponse), and
//! similar response wrappers.
//!
//! The SDK also exposes an emission extension point on top of that context: a
//! [`DiagnosticsHandler`] receives each operation's completed
//! [`DiagnosticsContext`], and an ordered [`DiagnosticsHandlerChain`] invokes
//! registered handlers once per operation at completion. Register handlers via
//! [`CosmosClientBuilder::with_diagnostics_handler()`](crate::CosmosClientBuilder::with_diagnostics_handler).
//! An empty chain emits no telemetry.
//!
//! A built-in OpenTelemetry metrics handler,
//! [`CosmosMetricsHandler`], is available behind
//! the off-by-default `metrics` feature. It emits the stable
//! `db.client.operation.duration` histogram (and, opt-in, development-tier
//! metrics) from each completed context.
//!
//! Two more built-in handlers emit telemetry for failed operations or operations
//! that breach [`DiagnosticsThresholds`]:
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
pub use handler::{
    ClientLifetimeToken, CosmosClientInfo, DiagnosticsHandler, DiagnosticsHandlerChain,
};
pub use logging::{SamplingLogHandler, TracingLogHandler};
pub use operation_context::CosmosOperationContext;
pub use rate_limiter::RateLimiterConfig;
pub use region::{RequestedRegion, RequestedRegionReason};
#[cfg(feature = "distributed_tracing")]
pub use tracing::{
    CosmosTracingHandler, CosmosTracingHandlerBuilder, CosmosTracingHandlerWithTracer,
};

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
mod region;
// Count-per-interval rate limiter shared by the sampling handlers (logging and,
// when enabled, tracing) so they can bound emission under an error storm.
pub(crate) mod rate_limiter;

#[cfg(feature = "metrics")]
pub mod metrics;

#[cfg(feature = "distributed_tracing")]
mod tracing;
