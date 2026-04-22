// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    policies::{Policy, PolicyResult, RetryHeaders},
    Context, Request,
};
use std::sync::Arc;

/// Retry policy that does not retry.
///
/// Use this policy as a stub to disable retry policies altogether.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct NoRetryPolicy {
    retry_headers: RetryHeaders,
}

impl NoRetryPolicy {
    /// Create a new `NoRetryPolicy`.
    pub fn new(retry_headers: RetryHeaders) -> Self {
        Self { retry_headers }
    }
}

#[async_trait::async_trait]
impl Policy for NoRetryPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // just call the following policies and bubble up the error.
        // Note that we do *not* modify the error to add HTTP error information,
        // that is the responsibility of the service clients.
        Ok(next[0].send(ctx, request, &next[1..]).await?)
    }
}
