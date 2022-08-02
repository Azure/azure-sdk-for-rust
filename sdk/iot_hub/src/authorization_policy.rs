use crate::service::IoTHubCredentials;
use azure_core::error::{ErrorKind, ResultExt};
use azure_core::{
    headers::{self, *},
    Context, Policy, PolicyResult, Request,
};
use std::sync::Arc;

const IOTHUB_TOKEN_SCOPE: &str = "https://iothubs.azure.net";

#[derive(Debug, Clone)]
pub struct AuthorizationPolicy {
    credentials: IoTHubCredentials,
}

impl AuthorizationPolicy {
    pub(crate) fn new(credentials: IoTHubCredentials) -> Self {
        Self { credentials }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        assert!(
            !next.is_empty(),
            "Authorization policies cannot be the last policy of a pipeline"
        );
        let request = match &self.credentials {
            IoTHubCredentials::SASToken(sas_token) => {
                request.insert_header(headers::AUTHORIZATION, sas_token);
                request
            }
            IoTHubCredentials::BearerToken(token) => {
                request.insert_header(AUTHORIZATION, format!("Bearer {}", token));
                request
            }
            IoTHubCredentials::TokenCredential(token_credential) => {
                let bearer_token = token_credential
                    .get_token(IOTHUB_TOKEN_SCOPE)
                    .await
                    .context(ErrorKind::Credential, "failed to get bearer token")?;

                request.insert_header(
                    AUTHORIZATION,
                    format!("Bearer {}", bearer_token.token.secret()),
                );
                request
            }
        };

        next[0].send(ctx, request, &next[1..]).await
    }
}
