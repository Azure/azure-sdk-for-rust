use crate::{
    auth::{AccessToken, TokenCredential},
    headers::AUTHORIZATION,
    policies::{Policy, PolicyResult},
    Context, Request,
};
use async_trait::async_trait;
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Option<AccessToken>,
    last_refresh_time: i64,
}

/// Default timeout in seconds before refreshing a new token.
const DEFAULT_REFRESH_TIME: i64 = 120;

impl BearerTokenCredentialPolicy {
    pub fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        Self {
            credential,
            scopes: scopes.into_iter().map(|s| s.into()).collect(),
            access_token: None,
            last_refresh_time: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }

    fn scopes(&self) -> Vec<&str> {
        self.scopes
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    async fn need_new_token(&mut self) {
        self.last_refresh_time = OffsetDateTime::now_utc().unix_timestamp();
        let access_token = self.credential.get_token(&self.scopes()).await;
        self.access_token = Some(access_token.unwrap());
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
        if OffsetDateTime::now_utc().unix_timestamp() - self.last_refresh_time
            > DEFAULT_REFRESH_TIME
        {
            self.need_new_token().await;
        }
        let token = self.access_token.clone().unwrap();
        let token = token.token;
        let token = token.secret();
        let token = String::from(token);
        request.insert_header(AUTHORIZATION, format!("Bearer {token}"));

        next[0].send(ctx, request, &next[1..]).await
    }
}

//TODO:
// Figure out the flow of Some(), Ok(), etc. and make it work with access_token
// Figure out how to get the string from the token that we can use
