// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    headers::{AsHeaders, HeaderName, HeaderValue},
    test::TestMode,
    Context, Policy, PolicyResult, Request,
};
use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};
use tracing::{debug_span, Instrument};

use crate::Skip;

#[derive(Debug, Default)]
pub struct RecordingPolicy {
    pub test_mode: TestMode,
    pub options: RwLock<RecordingOptions>,
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
        if let Ok(options) = self.options.read() {
            request.insert_headers(&*options)?;
        }

        next[0]
            .send(ctx, request, &next[1..])
            .instrument(span)
            .await
    }
}

#[derive(Debug, Default)]
pub struct RecordingOptions {
    pub skip: Option<Skip>,
}

impl AsHeaders for RecordingOptions {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        self.skip.as_headers()
    }
}
