// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{TelemetryOptions, TelemetryPolicy};
use std::{ops::Deref, sync::Arc};
use typespec_client_core::http::{self, Policy};

/// Execution pipeline.
///
/// A pipeline follows a precise flow:
///
/// 1. Client library-specified per-call policies are executed. Per-call policies can fail and bail out of the pipeline
///    immediately.
/// 2. User-specified per-call policies are executed.
/// 3. Telemetry policy.
/// 4. Retry policy. It allows to re-execute the following policies.
/// 5. Client library-specified per-retry policies. Per-retry polices are always executed at least once but are re-executed
///    in case of retries.
/// 6. User-specified per-retry policies are executed.
/// 7. Authorization policy. Authorization can depend on the HTTP headers and/or the request body so it
///    must be executed right before sending the request to the transport. Also, the authorization
///    can depend on the current time so it must be executed at every retry.
/// 8. Transport policy. Transport policy is always the last policy and is the policy that
///    actually constructs the `Response` to be passed up the pipeline.
///
/// A pipeline is immutable. In other words a policy can either succeed and call the following
/// policy of fail and return to the calling policy. Arbitrary policy "skip" must be avoided (but
/// cannot be enforced by code). All policies except Transport policy can assume there is another following policy (so
/// `self.pipeline[0]` is always valid).
#[derive(Debug, Clone)]
pub struct Pipeline(http::Pipeline);

impl Pipeline {
    /// Creates a new pipeline given the client library crate name and version,
    /// alone with user-specified and client library-specified policies.
    ///
    /// Crates can simply pass `option_env!("CARGO_PKG_NAME")` and `option_env!("CARGO_PKG_VERSION")` for the
    /// `crate_name` and `crate_version` arguments respectively.
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: http::ClientOptions,
        per_call_policies: Vec<Arc<dyn Policy>>,
        per_retry_policies: Vec<Arc<dyn Policy>>,
    ) -> Self {
        let mut per_call_policies = per_call_policies.clone();

        let telemetry_policy = TelemetryPolicy::new(
            crate_name,
            crate_version,
            // TODO: &options.telemetry.unwrap_or_default(),
            &TelemetryOptions::default(),
        );
        per_call_policies.insert(0, Arc::new(telemetry_policy));

        Self(http::Pipeline::new(
            options,
            per_call_policies,
            per_retry_policies,
        ))
    }
}

// TODO: Should we instead use the newtype pattern?
impl Deref for Pipeline {
    type Target = http::Pipeline;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
