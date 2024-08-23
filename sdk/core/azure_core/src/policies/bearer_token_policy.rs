use crate::{
    auth::{AccessToken, TokenCredential},
    headers::AUTHORIZATION,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Arc<Mutex<Option<AccessToken>>>,
}

/// Default timeout in seconds before refreshing a new token.
const DEFAULT_REFRESH_TIME: Duration = Duration::from_secs(10);

impl BearerTokenCredentialPolicy {
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            credential,
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            access_token: Arc::new(Mutex::new(None)),
        }
    }

    fn scopes(&self) -> Vec<&str> {
        self.scopes
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    fn refresh_token(&self, new_access_token: &AccessToken) {
        let mut access_token = self.access_token.lock().unwrap();
        *access_token = Some(new_access_token.clone());
    }

    fn is_token_expired(&self) -> bool {
        let access_token = self.access_token.lock().unwrap().clone();
        match access_token {
            Some(access_token) => access_token.is_expired(Some(DEFAULT_REFRESH_TIME)),
            None => true,
        }
    }

    fn access_token(&self) -> Result<String, &'static str> {
        let access_token = self.access_token.lock().unwrap().clone();
        match access_token {
            Some(access_token) => Ok(String::from(access_token.token.secret())),
            None => Err("access_token is None."),
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
        if self.is_token_expired() {
            let access_token = self.credential.get_token(&self.scopes()).await?;
            self.refresh_token(&access_token);
        }
        request.insert_header(
            AUTHORIZATION,
            format!(
                "Bearer {}",
                self.access_token()
                    .expect("Fetching access_token unexpectedly failed.")
            ),
        );

        next[0].send(ctx, request, &next[1..]).await
    }
}
