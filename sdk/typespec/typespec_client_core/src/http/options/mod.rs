// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Options and builders for clients, client methods, and policies.

mod retry;
mod transport;

pub use retry::*;
pub use transport::*;

use crate::http::{policies::Policy, Context};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

/// Controls what HTTP headers and query parameters are logged by default.
///
/// Headers and query parameters not in the allow list will be redacted.
///
/// This list is added to the default allow list of headers and query parameters.
///
#[derive(Clone, Debug, Default)]
pub struct LoggingOptions {
    /// The allowed header names to be logged.
    pub additional_allowed_header_names: Vec<Cow<'static, str>>,

    /// The allowed query parameters to be logged.
    pub additional_allowed_query_params: Vec<Cow<'static, str>>,
}

/// Client options allow customization of general client policies, retry options, and more.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub per_call_policies: Vec<Arc<dyn Policy>>,

    /// Policies called per try.
    pub per_try_policies: Vec<Arc<dyn Policy>>,

    /// Retry options.
    pub retry: Option<RetryOptions>,

    /// Transport options.
    pub transport: Option<TransportOptions>,

    pub logging: LoggingOptions,
}

/// Method options allow customization of client method calls.
#[derive(Clone, Debug, Default)]
pub struct ClientMethodOptions<'a> {
    /// The [`Context`] for this method call.
    pub context: Context<'a>,
}
