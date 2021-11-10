mod custom_headers_injector_policy;
#[cfg(feature = "mock_transport_framework")]
mod mock_transport_player_policy;
#[cfg(feature = "mock_transport_framework")]
mod mock_transport_recorder_policy;
mod retry_policies;
mod telemetry_policy;
mod transport;

use crate::{PipelineContext, Request, Response};
pub use custom_headers_injector_policy::{CustomHeaders, CustomHeadersInjectorPolicy};
#[cfg(feature = "mock_transport_framework")]
pub use mock_transport_player_policy::MockTransportPlayerPolicy;
#[cfg(feature = "mock_transport_framework")]
pub use mock_transport_recorder_policy::MockTransportRecorderPolicy;
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
/// The `C` generic represents the *contents* of the AuthorizationPolicy specific of this pipeline.
#[async_trait::async_trait]
pub trait Policy<C>: Send + Sync + std::fmt::Debug
where
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response>;
}
