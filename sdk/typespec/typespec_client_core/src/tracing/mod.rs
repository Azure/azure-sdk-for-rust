// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Distributed tracing trait definitions
//!
use std::sync::Arc;

use crate::http::Context;

/// Overall architecture for distributed tracing in the SDK.
///
/// This module defines the traits that are used to implement distributed tracing functionality.
///
/// Notes: There are three major traits defined here:
/// - TracerProvider: This trait is responsible for providing tracers - this is the
///   entrypoint for distributed tracing in the SDK.
/// - Tracer: This trait is responsible for creating spans and managing the active span.
/// - Span: This trait represents a single unit of work in the distributed tracing system.
///
pub mod attributes;
pub mod with_context;
pub use with_context::{FutureExt, WithContext};

/// The TracerProvider trait is the entrypoint for distributed tracing in the SDK.
///
/// It provides a method to get a tracer for a specific name and package version.
pub trait TracerProvider {
    /// Returns a tracer for the given name.
    ///
    /// Arguments:
    /// - `package_name`: The name of the package for which the tracer is requested.
    /// - `package_version`: The version of the package for which the tracer is requested.
    fn get_tracer(
        &self,
        package_name: &'static str,
        package_version: &'static str,
    ) -> Box<dyn Tracer + Send + Sync>;
}

pub trait Tracer {
    /// Starts a new span with the given name and type.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    ///
    /// # Returns
    /// An `Arc<dyn Span + Send + Sync>` representing the started span.
    ///
    fn start_span(&self, name: &'static str, kind: SpanKind) -> Arc<dyn Span + Send + Sync>;

    /// Starts a new span with the given type, using the current span as the parent span.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    ///
    /// # Returns
    /// An `Arc<dyn Span + Send + Sync>` representing the started span.
    ///
    fn start_span_with_current(
        &self,
        name: &'static str,
        kind: SpanKind,
    ) -> Arc<dyn Span + Send + Sync>;

    /// Starts a new child with the given name, type, and parent span.
    ///
    /// # Arguments
    /// - `name`: The name of the span to start.
    /// - `kind`: The type of the span to start.
    /// - `parent`: The parent span to use for the new span.
    ///
    /// # Returns
    /// An `Arc<dyn Span + Send + Sync>` representing the started span
    ///
    fn start_span_with_parent(
        &self,
        name: &'static str,
        kind: SpanKind,
        parent: Arc<dyn Span + Send + Sync>,
    ) -> Arc<dyn Span + Send + Sync>;
}
pub enum SpanStatus {
    Unset,
    Ok,
    Error { description: String },
}

#[derive(Debug, Default)]
pub enum SpanKind {
    #[default]
    Internal,
    Client,
    Server,
    Producer,
    Consumer,
}

pub trait SpanGuard {
    /// Ends the span when dropped.
    fn end(self) -> crate::Result<()>;
}

pub trait Span: AsAny {
    /// The 8 byte value which identifies the span.
    fn span_id(&self) -> [u8; 8];

    /// Ends the current span.
    fn end(&self) -> crate::Result<()>;

    /// Adds an event to the current span.
    ///
    /// # Arguments
    /// - `name`: The name of the event to add.
    /// - `attributes`: Optional attributes to associate with the event.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    ///
    fn add_event(
        &self,
        name: &'static str,
        attributes: Option<Vec<attributes::KeyValue>>,
    ) -> crate::Result<()>;

    /// Sets the status of the current span.
    /// # Arguments
    /// - `status`: The status to set for the current span.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    ///
    fn set_status(&self, status: SpanStatus) -> crate::Result<()>;

    /// Sets an attribute on the current span.
    fn set_attribute(
        &self,
        key: &'static str,
        value: attributes::AttributeValue,
    ) -> crate::Result<()>;

    /// Records a Rust standard error on the current span.
    ///
    /// # Arguments
    /// - `error`: A reference to the error to be recorded.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    ///
    fn record_error(&self, error: &dyn std::error::Error) -> crate::Result<()>;

    /// Temporarily sets the span as the current active span in the context.
    /// # Arguments
    /// - `context`: The context in which to set the current span.
    ///
    /// # Returns
    /// A `SpanGuard` that will end the span when dropped.
    ///
    /// This method allows the span to be set as the current span in the context,
    /// enabling it to be used for tracing operations within that context.
    ///
    fn set_current(&self, context: &Context) -> crate::Result<Box<dyn SpanGuard>>;
}

/// A trait that allows an object to be downcast to a reference of type `Any`.
pub trait AsAny {
    /// Returns a reference to the current object as a trait object.
    fn as_any(&self) -> &dyn std::any::Any;
}
