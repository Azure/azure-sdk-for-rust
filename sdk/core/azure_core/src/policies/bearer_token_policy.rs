use crate::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, Result},
    headers::AUTHORIZATION,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use async_lock::RwLock;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Arc<RwLock<Option<AccessToken>>>,
}

/// Default timeout in seconds before refreshing a new token.
const DEFAULT_REFRESH_TIME: Duration = Duration::from_secs(120);

impl BearerTokenCredentialPolicy {
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            credential,
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            access_token: Arc::new(RwLock::new(None)),
        }
    }

    fn scopes(&self) -> Vec<&str> {
        self.scopes
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    async fn access_token(&self) -> Result<String> {
        let access_token = self.access_token.as_ref().read().await;
        if let Some(x) = &(*access_token) {
            Ok(x.token.secret().to_string())
        } else {
            Err(Error::with_message(ErrorKind::Credential, || {
                "Failed to get AccessToken credential."
            }))
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for BearerTokenCredentialPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let access_token = self.access_token.read().await;

        if let Some(token) = &(*access_token) {
            if token.is_expired(Some(DEFAULT_REFRESH_TIME)) {
                let mut access_token = self.access_token.write().await;
                *access_token = Some(self.credential.get_token(&self.scopes()).await?);
            }
        } else {
            let mut access_token = self.access_token.write().await;
            *access_token = Some(self.credential.get_token(&self.scopes()).await?);
        }

        request.insert_header(
            AUTHORIZATION,
            format!("Bearer {}", self.access_token().await?),
        );

        next[0].send(ctx, request, &next[1..]).await
    }
}
