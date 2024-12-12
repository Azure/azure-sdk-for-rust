// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{test::TestMode, Context, Policy, PolicyResult, Request, TransportOptions};
use std::sync::Arc;
use tracing::{debug_span, Instrument};

/// Wraps the original [`TransportOptions`] and records or plays back session records for testing.
#[derive(Debug)]
pub struct ProxyTransportPolicy {
    pub(crate) inner: TransportOptions,
    pub(crate) mode: TestMode,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for ProxyTransportPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // There must be no other policies since we're encapsulating the original TransportPolicy.
        assert_eq!(0, next.len());

        let span = debug_span!(target: crate::SPAN_TARGET, "request", mode = ?self.mode);
        async move { { self.inner.send(ctx, request) }.await }
            .instrument(span)
            .await
    }
}
