// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP pipeline policies.

use crate::http::{Context, RawResponse, Request};
use async_trait::async_trait;
use std::sync::Arc;

mod custom_headers;
mod retry;
mod transport;

pub use custom_headers::*;
pub use retry::*;
pub use transport::*;

/// A specialized `Result` type for policies.
pub type PolicyResult = typespec::error::Result<RawResponse>;

/// A pipeline policy.
///
/// Policies are expected to modify the request and then call the subsequent policy.
/// Policies can then inspect the response, potentially signaling failure.
/// The only runtime enforced check is that the last policy must be a Transport policy. It's up to
/// the implementer to call the subsequent policy.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult;
}
