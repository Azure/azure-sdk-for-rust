// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
    http::{
        headers::AUTHORIZATION,
        policies::{Policy, PolicyResult},
    },
};
use async_lock::RwLock;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use typespec_client_core::date::OffsetDateTime;
use typespec_client_core::http::{Context, Request};

/// Authentication policy for a bearer token.
#[derive(Debug, Clone)]
pub struct BearerTokenCredentialPolicy {
    credential: Arc<dyn TokenCredential>,
    scopes: Vec<String>,
    access_token: Arc<RwLock<Option<AccessToken>>>,
}

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

    async fn access_token(&self) -> Option<String> {
        let access_token = self.access_token.read().await;
        access_token.as_ref().map(|s| s.token.secret().to_string())
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

        match access_token.as_ref() {
            None => {
                // cache is empty. Upgrade the lock and acquire a token, provided another thread hasn't already done so
                drop(access_token);
                let mut access_token = self.access_token.write().await;
                if access_token.is_none() {
                    *access_token = Some(self.credential.get_token(&self.scopes()).await?);
                }
            }
            Some(token) if should_refresh(&token.expires_on) => {
                // token is expired or within its refresh window. Upgrade the lock and
                // acquire a new token, provided another thread hasn't already done so
                let expires_on = token.expires_on;
                drop(access_token);
                let mut access_token = self.access_token.write().await;
                // access_token shouldn't be None here, but check anyway to guarantee unwrap won't panic
                if access_token.is_none() || access_token.as_ref().unwrap().expires_on == expires_on
                {
                    match self.credential.get_token(&self.scopes()).await {
                        Ok(new_token) => {
                            *access_token = Some(new_token);
                        }
                        Err(e)
                            if access_token.is_none()
                                || expires_on <= OffsetDateTime::now_utc() =>
                        {
                            // propagate this error because we can't proceed without a new token
                            return Err(e);
                        }
                        Err(_) => {
                            // ignore this error because the cached token is still valid
                        }
                    }
                }
            }
            Some(_) => {
                // do nothing; cached token is valid and not within its refresh window
            }
        }

        let access_token = self.access_token().await.ok_or_else(|| {
            Error::message(
                ErrorKind::Credential,
                "The request failed due to an error while fetching the access token.",
            )
        })?;
        request.insert_header(AUTHORIZATION, format!("Bearer {}", access_token));

        next[0].send(ctx, request, &next[1..]).await
    }
}

fn should_refresh(expires_on: &OffsetDateTime) -> bool {
    *expires_on <= OffsetDateTime::now_utc() + Duration::from_secs(300)
}
