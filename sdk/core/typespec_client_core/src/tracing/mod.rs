// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Distributed tracing trait definitions
//!
use crate::http::{Context, Request};
use crate::time::OffsetDateTime;
use std::{borrow::Cow, fmt::Debug, sync::Arc};

/// Overall architecture for distributed tracing in the SDK.
///
/// This module defines the traits that are used to implement distributed tracing functionality.
///
/// Notes: There are three major traits defined here:
/// - TracerProvider: This trait is responsible for providing tracers - this is the
///   entrypoint for distributed tracing in the SDK.
/// - Tracer: This trait is responsible for creating spans and managing the active span.
/// - Span: This trait represents a single unit of work in the distributed tracing system.
mod attributes;

pub use attributes::{Attribute, AttributeArray, AttributeValue};

/// The `TracerProvider` trait is the entrypoint for distributed tracing in the SDK.
///
/// It provides a method to get a tracer for a specific name and package version.
pub trait TracerProvider: Send + Sync + Debug {
    /// Returns a tracer for the given name.
    ///
    /// Arguments:
    /// - `namespace_name`: The namespace of the package for which the tracer is requested. See
    ///   [this page](https://learn.microsoft.com/azure/azure-resource-manager/management/azure-services-resource-providers)
    ///   for more information on namespace names.
    /// - `crate_name`: The name of the crate for which the tracer is requested.
    /// - `crate_version`: The version of the crate for which the tracer is requested.
    fn get_tracer(
        &self,
        namespace_name: Option<&'static str>,
        crate_name: &'static str,
        crate_version: Option<&'static str>,
    ) -> Arc<dyn Tracer>;
}

/// Options that customize how a span is started.
///
/// `SpanOptions` is passed to [`Tracer::start_span_with_options`] and
/// [`Tracer::start_span_with_parent_and_options`]. It implements [`Default`], so new options
/// can be added in the future without changing the [`Tracer`] method signatures. Construct it
/// with [`Default::default`] and struct update syntax:
///
/// ```
/// use typespec_client_core::tracing::SpanOptions;
/// use typespec_client_core::time::OffsetDateTime;
///
/// let options = SpanOptions {
///     start_time: Some(OffsetDateTime::now_utc()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct SpanOptions {
    /// An explicit start time to record for the span.
    ///
    /// When `None`, the span starts at the current time. When `Some`, the span is
    /// *backdated* to the given time, letting a caller reconstruct a span for an operation
    /// that has *already completed* — required for tail-based (late-bound) sampling, where
    /// the decision to emit a span is made after the operation finishes.
    pub start_time: Option<OffsetDateTime>,
}

/// The `Tracer` trait is responsible for creating spans and managing the active span in distributed tracing.
///
/// This trait defines methods for starting new spans, starting spans with a parent, and retrieving the namespace of the tracer.
pub trait Tracer: Send + Sync + Debug {
    /// Starts a new span with the given name and type.
    ///
    ///  The newly created span will have the "current" span as a parent.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    /// - `attributes`: A vector of attributes to associate with the span.
    ///
    /// # Returns
    /// An `Arc<dyn Span>` representing the started span.
    ///
    fn start_span(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<Attribute>,
    ) -> Arc<dyn Span>;

    /// Starts a new child with the given name, type, and parent span.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    /// - `attributes`: A vector of attributes to associate with the span.
    /// - `parent`: The parent span to use for the new span.
    ///
    /// # Returns
    /// An `Arc<dyn Span>` representing the started span
    ///
    /// Note: This method may panic if the parent span cannot be downcasted to the expected type.
    ///
    fn start_span_with_parent(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        parent: Arc<dyn Span>,
    ) -> Arc<dyn Span>;

    /// Starts a new span with the given name and type, customized by [`SpanOptions`].
    ///
    /// The newly created span will have the "current" span as a parent. [`SpanOptions`] lets
    /// the caller, for example, *backdate* the span via [`SpanOptions::start_time`] so it can
    /// reconstruct a span for an operation that has *already completed*. This is required for
    /// tail-based (late-bound) sampling, where the decision to emit a span is made after the
    /// operation finishes.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    /// - `attributes`: A vector of attributes to associate with the span.
    /// - `options`: Additional options, such as an explicit start time, for the span.
    ///
    /// # Returns
    /// An `Arc<dyn Span>` representing the started span.
    ///
    /// # Note
    /// The default implementation ignores `options` and delegates to [`Tracer::start_span`]
    /// so that existing implementations remain source-compatible. Implementations backed by a
    /// tracing system that supports these options (such as the OpenTelemetry bridge) override
    /// this to honor them.
    fn start_span_with_options(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        options: SpanOptions,
    ) -> Arc<dyn Span> {
        let _ = options;
        self.start_span(name, kind, attributes)
    }

    /// Starts a new child span with the given name, type, and parent span, customized by [`SpanOptions`].
    ///
    /// This is the [`SpanOptions`] variant of [`Tracer::start_span_with_parent`]. It lets a
    /// caller reconstruct a child span (for example, a single retry attempt) under an explicit
    /// parent, optionally *backdated* via [`SpanOptions::start_time`].
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    /// - `attributes`: A vector of attributes to associate with the span.
    /// - `parent`: The parent span to use for the new span.
    /// - `options`: Additional options, such as an explicit start time, for the span.
    ///
    /// # Returns
    /// An `Arc<dyn Span>` representing the started span.
    ///
    /// # Note
    /// The default implementation ignores `options` and delegates to
    /// [`Tracer::start_span_with_parent`] so that existing implementations remain
    /// source-compatible. Implementations backed by a tracing system that supports these
    /// options (such as the OpenTelemetry bridge) override this to honor them.
    ///
    /// Note: This method may panic if the parent span cannot be downcasted to the expected type.
    fn start_span_with_parent_and_options(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        parent: Arc<dyn Span>,
        options: SpanOptions,
    ) -> Arc<dyn Span> {
        let _ = options;
        self.start_span_with_parent(name, kind, attributes, parent)
    }

    /// Returns the namespace the tracer was configured with (if any).
    ///
    /// # Returns
    /// An `Option<&'static str>` representing the namespace of the tracer,
    fn namespace(&self) -> Option<&'static str>;
}

/// The status of a span.
///
/// This enum represents the possible statuses of a span in distributed tracing.
/// It can be either `Unset`, indicating that the span has not been set to any specific status,
/// or `Error`, which contains a description of the error that occurred during the span's execution
///
/// Note that OpenTelemetry defines an `Ok` status but that status is reserved for application and service developers,
/// so libraries should never set it.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum SpanStatus {
    /// The span has not been set to any specific status.
    Unset,
    /// The span has encountered an error, with a description of the error.
    Error {
        /// A description of the error that occurred during the span's execution.
        description: String,
    },
}

/// The kind of a span in distributed tracing.
///
/// This enum represents the different types of spans that can be created in distributed tracing, including internal operations, client requests, server requests, message production, and message consumption.
#[derive(Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpanKind {
    /// The default span kind, representing an internal operation within the library.
    #[default]
    Internal,
    /// The span represents a client request.
    Client,
    /// The span represents a server request.
    Server,
    /// The span represents a message being produced. This is typically used for messaging systems where a message is sent to a queue or topic.
    Producer,
    /// The span represents a message being consumed. This is typically used for messaging systems where a message is received from a queue or topic.
    Consumer,
}

/// A guard that ends a span when dropped.
pub trait SpanGuard {
    /// Ends the span when dropped.
    fn end(self);
}

/// A trait that represents a span in distributed tracing.
///
/// This trait defines the methods that a span must implement to be used in distributed tracing.
/// It includes methods for setting attributes, recording errors, and managing the span's lifecycle.
pub trait Span: AsAny + Send + Sync {
    /// Returns `true` if an application is listening for events on the span.
    fn is_recording(&self) -> bool;

    /// The 8 byte value which identifies the span.
    fn span_id(&self) -> [u8; 8];

    /// Ends the current span.
    fn end(&self);

    /// Ends the current span at an explicit end time.
    ///
    /// This is the backdating variant of [`Span::end`]. It lets a caller close a
    /// reconstructed span at the timestamp the operation actually finished, rather than
    /// "now" — the counterpart to [`SpanOptions::start_time`] for late-bound (tail-sampled)
    /// spans.
    ///
    /// # Arguments
    /// - `end_time`: The explicit end time to record for the span.
    ///
    /// # Note
    /// The default implementation ignores `end_time` and delegates to [`Span::end`] so that
    /// existing implementations remain source-compatible. Implementations backed by a
    /// tracing system that supports explicit timestamps (such as the OpenTelemetry bridge)
    /// override this to honor `end_time`.
    fn end_at(&self, end_time: OffsetDateTime) {
        let _ = end_time;
        self.end();
    }

    /// Sets the status of the current span.
    /// # Arguments
    /// - `status`: The status to set for the current span.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    ///
    fn set_status(&self, status: SpanStatus);

    /// Sets an attribute on the current span.
    ///
    /// # Arguments
    /// - `key`: The key of the attribute to set.
    /// - `value`: The value of the attribute to set.
    ///
    fn set_attribute(&self, key: &'static str, value: AttributeValue);

    /// Records a Rust standard error on the current span.
    ///
    /// # Arguments
    /// - `error`: A reference to the error to be recorded.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    ///
    fn record_error(&self, error: &dyn std::error::Error);

    /// Temporarily sets the span as the current active span in the context.
    ///
    /// # Arguments
    /// - `context`: The context in which to set the current span.
    ///
    /// # Returns
    /// A `SpanGuard` that will end the span when dropped.
    ///
    /// This method allows the span to be set as the current span in the context,
    /// enabling it to be used for tracing operations within that context.
    ///
    fn set_current(&self, context: &Context) -> Box<dyn SpanGuard>;

    /// Adds telemetry headers to the request for distributed tracing.
    ///
    /// # Arguments
    /// - `request`: A mutable reference to the request to which headers will be added.
    ///
    /// This method should be called before sending the request to ensure that the tracing information
    /// is included in the request headers. It typically adds the [W3C Distributed Tracing](https://www.w3.org/TR/trace-context/)
    /// headers to the request.
    ///
    fn propagate_headers(&self, request: &mut Request);
}

/// A trait that allows an object to be downcast to a reference of type `Any`.
pub trait AsAny {
    /// Returns a reference to the current object as a trait object.
    fn as_any(&self) -> &dyn std::any::Any;
}
