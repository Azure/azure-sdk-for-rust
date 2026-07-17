// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! OpenTelemetry metrics for Cosmos DB operations (feature `otel_metrics`).
//!
//! This module provides [`CosmosMetricsHandler`], a
//! [`DiagnosticsHandler`](crate::diagnostics::DiagnosticsHandler) that maps each
//! completed operation's [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)
//! to OpenTelemetry metrics following the database-client semantic conventions.
//!
//! The whole module is compiled only when the `otel_metrics` Cargo feature is
//! enabled, so with the feature off there is literally no metrics code — zero
//! cost. With the feature on but no meter provider registered, OpenTelemetry's
//! global meter is a no-op, so recording is still effectively free.
//!
//! There is no `Meter` abstraction in `azure_core` yet (design decision **D2**),
//! so metrics are emitted through the raw [`opentelemetry`] metrics API. The
//! handler is Cosmos-local today and a candidate for promotion once a shared
//! `azure_core` metrics surface exists.
//!
//! # Emitted metrics
//!
//! - **Stable (always on):** `db.client.operation.duration` (histogram, seconds).
//! - **Development (opt-in via [`MetricsOptions`]):**
//!   `azure.cosmosdb.client.operation.request_charge` and
//!   `db.client.response.returned_rows`.
//!
//! # Operation-scope identity
//!
//! `db.operation.name`, `db.collection.name`, and `db.namespace` are not carried
//! on the driver's [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext);
//! they are supplied by the SDK through a
//! [`CosmosOperationContext`](crate::diagnostics::CosmosOperationContext) stored on the pipeline
//! [`Context`](azure_core::http::Context). The handler reads whichever fields are
//! present and omits the rest, so it degrades gracefully.

pub mod attributes;

mod handler;
mod instruments;
mod options;

pub use handler::CosmosMetricsHandler;
pub use options::MetricsOptions;
