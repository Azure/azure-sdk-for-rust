// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{test::TestMode, Context, Policy, PolicyResult, Request};
use std::sync::Arc;
use tracing::{debug_span, Instrument};

#[derive(Debug)]
pub struct RecordingPolicy {
    pub test_mode: TestMode,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for RecordingPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let span = debug_span!(target: crate::SPAN_TARGET, "request", mode = ?self.test_mode);
        next[0]
            .send(ctx, request, &next[1..])
            .instrument(span)
            .await
    }
}
