// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-supplied, operation-scope identity carried to diagnostics handlers.
//!
//! The driver's [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)
//! records what happened on the wire, but the caller-facing identity of an
//! operation — its name, database, and container — is known only to the SDK.
//! [`CosmosOperationContext`] carries that identity to the handler chain through
//! the pipeline [`Context`](azure_core::http::Context), so both the metrics and
//! tracing handlers can emit correct operation-scope attributes.
//!
//! This type is always compiled (independent of the `otel_metrics` /
//! `otel_tracing` features) because the SDK populates it on every completed
//! operation; the feature-gated handlers simply read whichever fields are set.

use std::borrow::Cow;

use azure_core::fmt::SafeDebug;

/// SDK-supplied, operation-scope identity for a single Cosmos operation.
///
/// The driver's [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)
/// captures what happened on the wire (status, duration, regions, request
/// charge) but not the caller-facing identity of the operation — the operation
/// name, database, and container are known only to the SDK. This type carries
/// that identity to the diagnostics handlers via the pipeline
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
/// Every field is optional; a handler emits an attribute only for the fields
/// that are set. All setters accept anything convertible into a
/// `Cow<'static, str>`, so canonical static operation names (e.g. `"read_item"`)
/// are stored without allocating.
#[derive(Clone, Default, SafeDebug, PartialEq, Eq)]
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
