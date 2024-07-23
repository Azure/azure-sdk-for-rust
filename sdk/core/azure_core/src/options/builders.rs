#![allow(unused_mut, unused_variables)]

use super::{RetryOptions, TelemetryOptions, TransportOptions};
use crate::{Context, Policy};
use std::sync::Arc;

/// Methods to set general client options for client-specific [`ClientOptions`].
pub trait ClientOptionsBuilder {
    /// Sets policies for each call to a client method.
    /// These policies are called once per call: at most 1.
    fn with_per_call_policies<P>(mut self, per_call_policies: P) -> Self
    where
        P: Into<Vec<Arc<dyn Policy>>>,
        Self: Sized,
    {
        unimplemented!()
    }

    /// Sets policies for each attempt to call a client method.
    /// These policies are called once per attempt: at least 1, but as many as the number of tries allowed.
    fn with_per_try_policies<P>(mut self, per_try_policies: P) -> Self
    where
        P: Into<Vec<Arc<dyn Policy>>>,
        Self: Sized,
    {
        unimplemented!()
    }

    /// Set the default [`RetryOptions`] for every client method call.
    fn with_retry<P>(mut self, retry: P) -> Self
    where
        P: Into<RetryOptions>,
        Self: Sized,
    {
        unimplemented!()
    }

    /// Set [`TelemetryOptions`] used by the client.
    fn with_telemetry<P>(mut self, telemetry: P) -> Self
    where
        P: Into<TelemetryOptions>,
        Self: Sized,
    {
        unimplemented!()
    }

    /// Set the [`TransportOptions`] used by the client.
    fn with_transport<P>(mut self, transport: P) -> Self
    where
        P: Into<TransportOptions>,
        Self: Sized,
    {
        unimplemented!()
    }
}

/// Methods to set general method options for client-specific [`ClientMethodOptions`].
pub trait ClientMethodOptionsBuilder {
    /// Set optional [`Context`] for each client method call.
    fn with_context<P>(mut self, context: &Context<'_>) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
