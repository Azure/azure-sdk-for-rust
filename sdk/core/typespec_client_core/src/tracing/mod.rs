// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Distributed tracing trait definitions
//!
use crate::http::{Context, Request};
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
    fn set_attribute(&self, key: &'static str, value: attributes::AttributeValue);

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
