use crate::policies::Policy;
use crate::Context;
use std::fmt::Debug;
use std::sync::Arc;

/// Builders for client and method options in this crate.
pub mod builders;

mod retry;
pub use retry::*;

mod telemetry;
pub use telemetry::*;

mod transport;
pub use transport::*;

/// Client options allow customization of general client policies, retry options, and more.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub(crate) per_call_policies: Vec<Arc<dyn Policy>>,
    /// Policies called per try.
    pub(crate) per_try_policies: Vec<Arc<dyn Policy>>,
    /// Retry options.
    pub(crate) retry: Option<RetryOptions>,
    /// Telemetry options.
    pub(crate) telemetry: Option<TelemetryOptions>,
    /// Transport options.
    pub(crate) transport: Option<TransportOptions>,
}

impl ClientOptions {
    /// Create `ClientOptions` with the given [`TransportOptions`].
    #[cfg(test)]
    pub(crate) fn new(transport: TransportOptions) -> Self {
        Self {
            transport: Some(transport),
            ..Default::default()
        }
    }

    /// Sets policies for each call to a client method.
    /// These policies are called once per call: at most 1.
    pub fn set_per_call_policies<P>(&mut self, per_call_policies: P)
    where
        P: Into<Vec<Arc<dyn Policy>>>,
    {
        self.per_call_policies = per_call_policies.into();
    }

    /// Sets policies for each attempt to call a client method.
    /// These policies are called once per attempt: at least 1, but as many as the number of tries allowed.
    pub fn set_per_try_policies<P>(&mut self, per_try_policies: P)
    where
        P: Into<Vec<Arc<dyn Policy>>>,
    {
        self.per_try_policies = per_try_policies.into()
    }

    /// Set the default [`RetryOptions`] for every client method call.
    pub fn set_retry<P>(&mut self, retry: P)
    where
        P: Into<RetryOptions>,
    {
        self.retry = Some(retry.into());
    }

    /// Set [`TelemetryOptions`] used by the client.
    pub fn set_telemetry<P>(&mut self, telemetry: P)
    where
        P: Into<TelemetryOptions>,
    {
        self.telemetry = Some(telemetry.into());
    }

    /// Set the [`TransportOptions`] used by the client.
    pub fn set_transport<P>(&mut self, transport: P)
    where
        P: Into<TransportOptions>,
    {
        self.transport = Some(transport.into());
    }
}

/// Method options allow customization of client method calls.
#[derive(Clone, Debug, Default)]
pub struct ClientMethodOptions<'a> {
    pub(crate) context: Context<'a>,
}

impl<'a> ClientMethodOptions<'a> {
    /// Set optional [`Context`] for each client method call.
    pub fn set_context(&mut self, context: &Context<'a>) {
        self.context = context.clone();
    }
}
