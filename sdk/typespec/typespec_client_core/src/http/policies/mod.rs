// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP pipeline policies.

use crate::http::{Context, BufResponse, Request};
use async_trait::async_trait;
use std::sync::Arc;

mod custom_headers;
mod logging;
mod retry;
mod transport;

pub use custom_headers::*;
pub use logging::*;
pub use retry::*;
pub use transport::*;

/// A specialized `Result` type for policies.
pub type PolicyResult = typespec::error::Result<BufResponse>;

/// A pipeline policy.
///
/// Policies are expected to modify the request and then call the subsequent policy.
/// Policies can then inspect the response, potentially signaling failure.
/// The only runtime enforced check is that the last policy must be a Transport policy. It's up to
/// the implementer to call the subsequent policy.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Policy: Send + Sync + std::fmt::Debug {
    /// Send the request through this policy and the subsequent policies in the pipeline.
    ///
    /// # Arguments
    /// * `ctx` - The context for the request.
    /// * `request` - The mutable reference to the request to be sent.
    /// * `next` - The slice of subsequent policies to call after this one.
    ///
    /// # Returns
    /// A `PolicyResult` containing either the `RawResponse` or an error if the request failed at any point in the pipeline.
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult;
}
