mod retry_policies;
mod transport;
use crate::{Context, Request};
pub use retry_policies::*;
use std::error::Error;
pub use transport::*;

pub type PolicyResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Policies will take ownership of the request and give back either a request or a response. Right
/// now only transport policies are allowed to return a response. Other policies must be `Request ->
/// Request`.
#[async_trait::async_trait]
pub trait Policy<R>: Send + Sync + std::fmt::Debug
where
    R: Send + Sync,
{
    async fn send(&self, ctx: &mut Context, request: Request) -> PolicyResult<R>;
}
