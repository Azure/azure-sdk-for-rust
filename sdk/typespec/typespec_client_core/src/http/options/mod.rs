// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options and builders for clients, client methods, and policies.

pub mod builders;
mod retry;
mod transport;

pub use retry::*;
pub use transport::*;

use crate::http::{policies::Policy, Context};
use std::fmt::Debug;
use std::sync::Arc;

/// Client options allow customization of general client policies, retry options, and more.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub per_call_policies: Vec<Arc<dyn Policy>>,

    /// Policies called per try.
    pub per_try_policies: Vec<Arc<dyn Policy>>,

    /// Retry options.
    pub retry: Option<RetryOptions>,

    // /// Telemetry options.
    // pub telemetry: Option<TelemetryOptions>,
    /// Transport options.
    pub transport: Option<TransportOptions>,
}

/// Method options allow customization of client method calls.
#[derive(Clone, Debug, Default)]
pub struct ClientMethodOptions<'a> {
    /// The [`Context`] for this method call.
    pub context: Context<'a>,
}
