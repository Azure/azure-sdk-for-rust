#[cfg(not(target_arch = "wasm32"))]
use crate::policies::{Policy, PolicyResult};
#[allow(unused_imports)]
use crate::TransportOptions;
#[allow(unused_imports)]
use crate::{Context, HttpClient, Request, Response};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TransportPolicy {
    pub(crate) http_client: Arc<dyn HttpClient>,
}

impl TransportPolicy {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(options: &TransportOptions) -> Self {
        Self {
            http_client: options.http_client.clone(),
        }
    }
}

#[async_trait::async_trait]
#[cfg(not(target_arch = "wasm32"))]
impl Policy for TransportPolicy {
    async fn send(
        &self,
        _ctx: &mut Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        // there must be no more policies
        assert_eq!(0, next.len());

        let response = { self.http_client.execute_request2(request) };

        Ok(response.await?.into())
    }
}
