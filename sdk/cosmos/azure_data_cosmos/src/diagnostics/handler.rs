// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`DiagnosticsHandler`] emission extension point and its
//! [`DiagnosticsHandlerChain`].
//!
//! The driver produces a canonical [`DiagnosticsContext`] for every operation
//! and always materializes it. This module adds the SDK-side seam that decides
//! *what to emit* from that context: applications register one or more
//! [`DiagnosticsHandler`]s, and the SDK invokes them — in registration order —
//! exactly once per operation at completion.
//!
//! The surface is deliberately small, additive, and swappable (Cosmos-local for
//! now; a candidate for promotion into `azure_core` later). Built-in handlers
//! (metrics, tracing, sampled logging) are layered on top in separate modules.

use std::fmt;
use std::sync::Arc;

use azure_core::http::Context;

use crate::diagnostics::DiagnosticsContext;

/// A sink that consumes a completed [`DiagnosticsContext`] for a single Cosmos
/// operation.
///
/// Handlers are the SDK's emission extension point: the driver produces the
/// context and the handler decides what telemetry (metrics, spans, logs, …) to
/// emit from it. [`handle`](DiagnosticsHandler::handle) is called once per
/// operation, after the operation has completed, with the finalized context.
///
/// Implementations must be cheap and non-blocking — they run on the operation's
/// completion path. Handlers should never panic; a panicking handler will
/// propagate out of the invoking operation.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::{
///     DiagnosticsContext, DiagnosticsHandler, DiagnosticsHandlerChain,
/// };
/// use azure_core::http::Context;
///
/// struct LoggingHandler;
///
/// impl DiagnosticsHandler for LoggingHandler {
///     fn handle(&self, diagnostics: &DiagnosticsContext, _cx: &Context) {
///         tracing::debug!(
///             duration_ms = diagnostics.duration().as_millis() as u64,
///             "cosmos operation completed"
///         );
///     }
/// }
///
/// let chain = DiagnosticsHandlerChain::new().with_handler(Arc::new(LoggingHandler));
/// assert_eq!(chain.len(), 1);
/// ```
pub trait DiagnosticsHandler: Send + Sync {
    /// Consumes the completed diagnostics for one operation.
    ///
    /// * `diagnostics` - The finalized context for the just-completed operation.
    /// * `cx` - The pipeline/trace [`Context`] associated with the operation,
    ///   available so handlers can correlate emitted telemetry with the caller's
    ///   trace context.
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>);
}

/// An ordered, cheaply cloneable chain of [`DiagnosticsHandler`]s.
///
/// The chain is the unit the SDK invokes at operation completion. Handlers run
/// in registration order, so ordering is deterministic. An empty chain — the
/// default when no handler is registered — is a zero-overhead no-op.
///
/// The chain is backed by a shared, reference-counted slice, so cloning it (for
/// example when a [`CosmosClient`](crate::CosmosClient) hands state down to a
/// [`DatabaseClient`](crate::clients::DatabaseClient) or
/// [`ContainerClient`](crate::clients::ContainerClient)) is a single atomic
/// increment rather than a deep copy.
///
/// Register handlers via
/// [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
#[derive(Clone)]
pub struct DiagnosticsHandlerChain {
    handlers: Arc<[Arc<dyn DiagnosticsHandler>]>,
}

impl DiagnosticsHandlerChain {
    /// Creates an empty chain.
    ///
    /// An empty chain performs no work when invoked.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new().into(),
        }
    }

    /// Creates a chain from an ordered list of handlers.
    ///
    /// Handlers are invoked in the order supplied.
    pub fn from_handlers(handlers: Vec<Arc<dyn DiagnosticsHandler>>) -> Self {
        Self {
            handlers: handlers.into(),
        }
    }

    /// Returns a new chain with `handler` appended to the end.
    ///
    /// This is additive: existing handlers keep their relative order and the new
    /// handler runs last.
    #[must_use]
    pub fn with_handler(&self, handler: Arc<dyn DiagnosticsHandler>) -> Self {
        let mut handlers: Vec<Arc<dyn DiagnosticsHandler>> = self.handlers.to_vec();
        handlers.push(handler);
        Self {
            handlers: handlers.into(),
        }
    }

    /// Returns the registered handlers in invocation order.
    pub fn handlers(&self) -> &[Arc<dyn DiagnosticsHandler>] {
        &self.handlers
    }

    /// Returns `true` when no handlers are registered.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }

    /// Returns the number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }

    /// Invokes every handler, in order, with the completed diagnostics.
    ///
    /// A no-op when the chain is empty.
    pub(crate) fn dispatch(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        for handler in self.handlers.iter() {
            handler.handle(diagnostics, cx);
        }
    }
}

impl Default for DiagnosticsHandlerChain {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for DiagnosticsHandlerChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handlers are `dyn` trait objects without a `Debug` bound, so surface
        // only the count to keep the trait surface minimal.
        f.debug_struct("DiagnosticsHandlerChain")
            .field("handlers", &self.handlers.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::models::ActivityId;
    use std::sync::Mutex;

    /// A test handler that appends `(label, activity_id)` to a shared log every
    /// time it is invoked, so tests can assert both per-operation receipt and
    /// cross-handler ordering.
    struct RecordingHandler {
        label: &'static str,
        log: Arc<Mutex<Vec<(&'static str, ActivityId)>>>,
    }

    impl DiagnosticsHandler for RecordingHandler {
        fn handle(&self, diagnostics: &DiagnosticsContext, _cx: &Context<'_>) {
            self.log
                .lock()
                .unwrap()
                .push((self.label, diagnostics.activity_id().clone()));
        }
    }

    fn recording(
        label: &'static str,
        log: &Arc<Mutex<Vec<(&'static str, ActivityId)>>>,
    ) -> Arc<dyn DiagnosticsHandler> {
        Arc::new(RecordingHandler {
            label,
            log: Arc::clone(log),
        })
    }

    #[test]
    fn empty_chain_is_noop() {
        let chain = DiagnosticsHandlerChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);

        // Dispatching against an empty chain must not panic and must do nothing.
        let ctx = DiagnosticsContext::for_testing(ActivityId::new_uuid());
        chain.dispatch(&ctx, &Context::new());
    }

    #[test]
    fn handlers_receive_completed_context_in_registration_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let chain = DiagnosticsHandlerChain::from_handlers(vec![
            recording("a", &log),
            recording("b", &log),
            recording("c", &log),
        ]);
        assert_eq!(chain.len(), 3);

        // Two distinct "operations", each with its own completed context.
        let op1 = ActivityId::new_uuid();
        let op2 = ActivityId::new_uuid();
        let cx = Context::new();
        chain.dispatch(&DiagnosticsContext::for_testing(op1.clone()), &cx);
        chain.dispatch(&DiagnosticsContext::for_testing(op2.clone()), &cx);

        let recorded = log.lock().unwrap().clone();
        // Every handler saw each operation's own completed context, and the
        // handlers ran in deterministic registration order (a, b, c) per op.
        assert_eq!(
            recorded,
            vec![
                ("a", op1.clone()),
                ("b", op1.clone()),
                ("c", op1),
                ("a", op2.clone()),
                ("b", op2.clone()),
                ("c", op2),
            ]
        );
    }

    #[test]
    fn with_handler_appends_and_is_additive() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let base = DiagnosticsHandlerChain::new().with_handler(recording("first", &log));
        let extended = base.with_handler(recording("second", &log));

        // `with_handler` returns a new chain and leaves the original untouched.
        assert_eq!(base.len(), 1);
        assert_eq!(extended.len(), 2);

        let op = ActivityId::new_uuid();
        extended.dispatch(
            &DiagnosticsContext::for_testing(op.clone()),
            &Context::new(),
        );

        let recorded = log.lock().unwrap().clone();
        assert_eq!(recorded, vec![("first", op.clone()), ("second", op)]);
    }
}
