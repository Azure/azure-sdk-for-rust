mod custom_headers_policy;
mod retry_policies;
mod telemetry_policy;
mod timeout_policy;
mod transport;

pub use custom_headers_policy::{CustomHeaders, CustomHeadersPolicy};
pub use retry_policies::*;
pub use telemetry_policy::*;
pub use timeout_policy::*;
pub use transport::*;

use crate::{Context, Request, Response};
use async_trait::async_trait;
use std::sync::Arc;

/// A specialized `Result` type for policies.
pub type PolicyResult = crate::error::Result<Response>;
// pub type PolicyResult = Result<Response, Box<dyn Error + Send + Sync>>;

/// A pipeline policy.
///
/// Policies are expected to modify the request and then call the following policy.
/// Policies can then inspect the response, potentially signaling failure.
/// The only runtime enforced check is that the last policy must be a Transport policy. It's up to
/// the implementer to call the following policy.
/// The `C` generic represents the *contents* of the AuthorizationPolicy specific of this pipeline.
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
