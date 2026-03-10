// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Policies for recordings.

use async_trait::async_trait;
use azure_core::{
    http::{
        policies::{Policy, PolicyResult},
        AsyncRawResponse, Context, Request,
    },
    test::RecordingMode,
};
use std::sync::Arc;

/// Adds the `x-recording-mode` header to test responses.
#[derive(Debug)]
pub struct RecordingModePolicy {
    mode: RecordingMode,
}

impl RecordingModePolicy {
    /// Create a new `RecordingModePolicy`.
    pub fn new(mode: RecordingMode) -> Self {
        Self { mode }
    }
}

#[async_trait]
impl Policy for RecordingModePolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let resp = next[0].send(ctx, request, &next[1..]).await?;
        let (status, mut headers, body) = resp.deconstruct();
        headers.add(self.mode)?;

        Ok(AsyncRawResponse::new(status, headers, Box::pin(body)))
    }
}
