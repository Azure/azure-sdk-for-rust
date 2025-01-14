// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{Context, Policy, PolicyResult, Request};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StorageHeadersPolicy {}

impl StorageHeadersPolicy {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for StorageHeadersPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // TODO: Check if the header is already set (that means cx set), so don't set if so
        let request_id = Uuid::new_v4().to_string();
        request.insert_header("x-ms-client-request-id", request_id);

        next[0].send(ctx, request, &next[1..]).await
    }
}
