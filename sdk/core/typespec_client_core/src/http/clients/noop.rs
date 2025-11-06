// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    error::Result,
    http::{AsyncRawResponse, Request},
};
use async_trait::async_trait;

#[derive(Debug)]
struct NoopClient;

pub(crate) fn new_noop_client() -> std::sync::Arc<dyn super::HttpClient> {
    std::sync::Arc::new(NoopClient)
}

// TODO: We probably don't want to limit this to wasm32 since there will be wasm environments with threads. This should be a feature flag.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl super::HttpClient for NoopClient {
    #[allow(clippy::diverging_sub_expression)]
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse> {
        panic!(
            "A request was called on the default http client `NoopClient`.\
	This client does nothing but panic. Make sure to enable an http\
	 client that can actually perform requests. You can do this by ensuring that the `reqwest` feature is enabled.\n\
     Request:\n{request:?}"
        );
    }
}
