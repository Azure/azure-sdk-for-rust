mod retry_policies;
mod transport;
use crate::{Context, Request, Response};
pub use retry_policies::*;
use std::error::Error;
use std::sync::Arc;
pub use transport::*;

pub type PolicyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Policies are expected to modify the request and then call the following policy.
/// Policies can then inspect the response, potentially signaling failure.
/// The only runtime enforced check is that the last policy must be a Transport policy.
#[async_trait::async_trait]
pub trait Policy: Send + Sync + std::fmt::Debug {
    async fn send(
        &self,
        ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response>;
}
