mod retry_policies;
mod telemetry_policy;
mod transport;

use crate::{PipelineContext, Request, Response};
pub use retry_policies::*;
use std::error::Error;
use std::sync::Arc;
pub use telemetry_policy::*;
pub use transport::*;

pub type PolicyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// A pipeline policy.
///
/// Policies are expected to modify the request and then call the following policy.
/// Policies can then inspect the response, potentially signaling failure.
/// The only runtime enforced check is that the last policy must be a Transport policy. It's up to
/// the implementer to call the following policy.
#[async_trait::async_trait]
pub trait Policy<R: Send + Sync>: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: &mut PipelineContext<R>,
        request: &mut Request,
        next: &[Arc<dyn Policy<R>>],
    ) -> PolicyResult<Response>;
}
