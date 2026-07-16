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
//!   `azure.cosmosdb.client.operation.request_charge`,
//!   `db.client.response.returned_rows`,
//!   `azure.cosmosdb.client.active_instance.count`.
//!
//! # Operation-scope identity
//!
//! `db.operation.name`, `db.collection.name`, and `db.namespace` are not carried
//! on the driver's [`DiagnosticsContext`]; they are supplied by the SDK through a
//! [`CosmosOperationContext`] stored on the pipeline
//! [`Context`](azure_core::http::Context). The handler reads whichever fields are
//! present and omits the rest, so it degrades gracefully.

pub mod attributes;

mod handler;
mod instruments;
mod options;

pub use handler::CosmosMetricsHandler;
pub use options::MetricsOptions;

use std::borrow::Cow;

/// SDK-supplied, operation-scope identity for a single Cosmos operation.
///
/// The driver's [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)
/// captures what happened on the wire (status, duration, regions, request
/// charge) but not the caller-facing identity of the operation — the operation
/// name, database, and container are known only to the SDK. This type carries
/// that identity to [`CosmosMetricsHandler`] via the pipeline
/// [`Context`](azure_core::http::Context):
///
/// ```
/// use azure_core::http::Context;
/// use azure_data_cosmos::diagnostics::CosmosOperationContext;
///
/// let op = CosmosOperationContext::new()
///     .with_operation_name("read_item")
///     .with_database_name("my_db")
///     .with_container_name("my_container");
/// let cx = Context::new().with_value(op);
/// assert_eq!(
///     cx.value::<CosmosOperationContext>().and_then(|o| o.operation_name()),
///     Some("read_item"),
/// );
/// ```
///
/// Every field is optional; the handler emits an attribute only for the fields
/// that are set. All setters accept anything convertible into a
/// `Cow<'static, str>`, so canonical static operation names (e.g. `"read_item"`)
/// are stored without allocating.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CosmosOperationContext {
    operation_name: Option<Cow<'static, str>>,
    database_name: Option<Cow<'static, str>>,
    container_name: Option<Cow<'static, str>>,
    server_address: Option<Cow<'static, str>>,
    consistency_level: Option<Cow<'static, str>>,
    connection_mode: Option<Cow<'static, str>>,
    returned_item_count: Option<u64>,
}

impl CosmosOperationContext {
    /// Creates an empty operation context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the canonical operation name (`db.operation.name`), e.g. `read_item`.
    #[must_use]
    pub fn with_operation_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.operation_name = Some(name.into());
        self
    }

    /// Sets the database name (`db.namespace`).
    #[must_use]
    pub fn with_database_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.database_name = Some(name.into());
        self
    }

    /// Sets the container name (`db.collection.name`).
    #[must_use]
    pub fn with_container_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.container_name = Some(name.into());
        self
    }

    /// Sets the server address (`server.address`), overriding the value the
    /// handler would otherwise derive from the contacted endpoint.
    #[must_use]
    pub fn with_server_address(mut self, address: impl Into<Cow<'static, str>>) -> Self {
        self.server_address = Some(address.into());
        self
    }

    /// Sets the effective consistency level (development attribute).
    #[must_use]
    pub fn with_consistency_level(mut self, level: impl Into<Cow<'static, str>>) -> Self {
        self.consistency_level = Some(level.into());
        self
    }

    /// Sets the connection mode, e.g. `gateway` or `direct` (development attribute).
    #[must_use]
    pub fn with_connection_mode(mut self, mode: impl Into<Cow<'static, str>>) -> Self {
        self.connection_mode = Some(mode.into());
        self
    }

    /// Sets the number of items returned (feeds `db.client.response.returned_rows`).
    #[must_use]
    pub fn with_returned_item_count(mut self, count: u64) -> Self {
        self.returned_item_count = Some(count);
        self
    }

    /// The canonical operation name, if set.
    pub fn operation_name(&self) -> Option<&str> {
        self.operation_name.as_deref()
    }

    /// The database name, if set.
    pub fn database_name(&self) -> Option<&str> {
        self.database_name.as_deref()
    }

    /// The container name, if set.
    pub fn container_name(&self) -> Option<&str> {
        self.container_name.as_deref()
    }

    /// The server-address override, if set.
    pub fn server_address(&self) -> Option<&str> {
        self.server_address.as_deref()
    }

    /// The effective consistency level, if set.
    pub fn consistency_level(&self) -> Option<&str> {
        self.consistency_level.as_deref()
    }

    /// The connection mode, if set.
    pub fn connection_mode(&self) -> Option<&str> {
        self.connection_mode.as_deref()
    }

    /// The number of items returned, if set.
    pub fn returned_item_count(&self) -> Option<u64> {
        self.returned_item_count
    }
}
