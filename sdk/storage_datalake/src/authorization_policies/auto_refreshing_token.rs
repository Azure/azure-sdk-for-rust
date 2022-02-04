use azure_core::{
    auth::{TokenCredential, TokenResponse},
    Context, Error, Policy, PolicyResult, Request, Response,
};
use chrono::{Duration, Utc};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use std::sync::Arc;
use std::sync::Mutex;

fn is_expired(token: TokenResponse) -> bool {
    token.expires_on < Utc::now() + Duration::seconds(20)
}

#[derive(Clone)]
pub struct AutoRefreshingTokenCredential {
    credential: Arc<dyn TokenCredential>,
    current_token: Arc<Mutex<Option<std::result::Result<TokenResponse, Error>>>>,
}

impl std::fmt::Debug for AutoRefreshingTokenCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AutoRefreshingTokenCredential")
            .field("credential", &"TokenCredential")
            .finish()
    }
}

impl AutoRefreshingTokenCredential {
    /// Create a new `AutoRefreshingTokenCredential` around the provided base provider.
    pub fn new(provider: Arc<dyn TokenCredential>) -> Self {
        Self {
            credential: provider,
            current_token: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for AutoRefreshingTokenCredential {
    async fn get_token(&self, resource: &str) -> std::result::Result<TokenResponse, Error> {
        loop {
            let mut guard = self.current_token.lock().unwrap();
            match guard.as_ref() {
                None => {
                    let res = self.credential.get_token(resource).await;
                    *guard = Some(res);
                }
                Some(Err(err)) => {
                    return Err(Error::HeaderNotFound("Box::new((*err))".to_string()));
                }
                Some(Ok(token)) => {
                    if is_expired(token.clone()) {
                        *guard = None;
                    } else {
                        return Ok(token.clone());
                    };
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct AuthorizationPolicy {
    credential: Arc<dyn TokenCredential>,
    resource: String,
}

impl std::fmt::Debug for AuthorizationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AuthorizationPolicy")
            .field("credential", &"TokenCredential")
            .field("resource", &self.resource)
            .finish()
    }
}

impl AuthorizationPolicy {
    pub(crate) fn new<T>(credential: Arc<dyn TokenCredential>, resource: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            credential,
            resource: resource.into(),
        }
    }
}

#[async_trait::async_trait]
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult<Response> {
        if next.is_empty() {
            return Err(Box::new(azure_core::PipelineError::InvalidTailPolicy(
                "Authorization policies cannot be the last policy of a pipeline".to_owned(),
            )));
        }

        let token = self.credential.get_token(&self.resource).await?;
        let auth_header_value = format!("Bearer {}", token.token.secret().clone());

        request
            .headers_mut()
            .append(AUTHORIZATION, HeaderValue::from_str(&auth_header_value)?);

        next[0].send(ctx, request, &next[1..]).await
    }
}
