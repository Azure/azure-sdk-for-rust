// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{
    headers::Headers,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use std::sync::Arc;
use tracing::trace;

/// Custom headers to add to a request.
#[derive(Debug, Clone)]
pub struct CustomHeaders(Headers);

impl From<Headers> for CustomHeaders {
    fn from(h: Headers) -> Self {
        Self(h)
    }
}

/// [`Policy`] to add [`CustomHeaders`] to a request.
#[derive(Clone, Debug, Default)]
pub struct CustomHeadersPolicy {}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for CustomHeadersPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if let Some(CustomHeaders(custom_headers)) = ctx.value::<CustomHeaders>() {
            custom_headers
                .iter()
                .for_each(|(header_name, header_value)| {
                    trace!(
                        "injecting custom context header {:?} with value {:?}",
                        header_name,
                        header_value
                    );
                    request.insert_header(header_name.clone(), header_value.clone());
                });
        }

        next[0].send(ctx, request, &next[1..]).await
    }
}
