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
//! With no handlers registered the chain is a zero-overhead no-op.
//!
//! A built-in OpenTelemetry metrics handler,
//! [`CosmosMetricsHandler`](metrics::CosmosMetricsHandler), is available behind
//! the off-by-default `otel_metrics` feature. It emits the stable
//! `db.client.operation.duration` histogram (and, opt-in, development-tier
//! metrics) from each completed context.
//!
//! Two further built-in handlers layer telemetry on top of the chain, both driven by
//! tail-based sampling against [`DiagnosticsThresholds`] (emit only for failed or
//! threshold-breaching operations):
//!
//! - [`SamplingLogHandler`] — logs a compact, rate-limited diagnostics line
//!   through the [`tracing`](https://docs.rs/tracing) ecosystem.
//! - [`CosmosTracingHandler`] — emits a backdated OpenTelemetry span tree
//!   (behind the off-by-default `otel_tracing` feature).

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, TransportKind};
#[doc(inline)]
pub use azure_data_cosmos_driver::DiagnosticsThresholds;
pub use handler::{DiagnosticsHandler, DiagnosticsHandlerChain};
pub use logging::SamplingLogHandler;
#[cfg(feature = "otel_tracing")]
pub use tracing::CosmosTracingHandler;

#[cfg(feature = "otel_metrics")]
pub use metrics::{CosmosMetricsHandler, CosmosOperationContext, MetricsOptions};

// =========================================================================
// Internal modules
// =========================================================================

mod handler;
mod logging;

#[cfg(feature = "otel_metrics")]
pub mod metrics;

#[cfg(feature = "otel_tracing")]
mod tracing;
