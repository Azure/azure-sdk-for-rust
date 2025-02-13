// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{credentials::cache::TokenCache, federated_credentials_flow, TokenCredentialOptions};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
use azure_core::{
    auth::{AccessToken, Secret, TokenCredential},
    error::{ErrorKind, ResultExt},
    Error, HttpClient,
};
use futures::channel::oneshot;
use std::{
    fs, str,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use time::OffsetDateTime;
use url::Url;

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
const AZURE_FEDERATED_TOKEN: &str = "AZURE_FEDERATED_TOKEN";

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://learn.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>
#[derive(Debug)]
pub struct WorkloadIdentityCredential {
    http_client: Arc<dyn HttpClient>,
    authority_host: Url,
    tenant_id: String,
    client_id: String,
    token: Token,
    cache: TokenCache,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`
    pub fn new<T>(
        http_client: Arc<dyn HttpClient>,
        authority_host: Url,
        tenant_id: String,
        client_id: String,
        token: T,
    ) -> Self
    where
        T: Into<Secret>,
    {
        Self {
            http_client,
            authority_host,
            tenant_id,
            client_id,
            token: Token::Value(token.into()),
            cache: TokenCache::new(),
        }
    }

    pub fn create(
        options: impl Into<TokenCredentialOptions>,
    ) -> azure_core::Result<WorkloadIdentityCredential> {
        let options = options.into();
        let http_client = options.http_client();
        let authority_host = options.authority_host()?;
        let env = options.env();
        let tenant_id =
            env.var(AZURE_TENANT_ID_ENV_KEY)
                .with_context(ErrorKind::Credential, || {
                    format!(
                        "working identity credential requires {} environment variable",
                        AZURE_TENANT_ID_ENV_KEY
                    )
                })?;
        let client_id =
            env.var(AZURE_CLIENT_ID_ENV_KEY)
                .with_context(ErrorKind::Credential, || {
                    format!(
                        "working identity credential requires {} environment variable",
                        AZURE_CLIENT_ID_ENV_KEY
                    )
                })?;

        if let Ok(token) = env
            .var(AZURE_FEDERATED_TOKEN)
            .map_kind(ErrorKind::Credential)
        {
            return Ok(WorkloadIdentityCredential::new(
                http_client,
                authority_host,
                tenant_id,
                client_id,
                token,
            ));
        }

        if let Ok(token_file) = env
            .var(AZURE_FEDERATED_TOKEN_FILE)
            .map_kind(ErrorKind::Credential)
        {
            return Ok(Arc::new(Self {
                http_client,
                authority_host,
                tenant_id,
                client_id,
                token: Token::with_file(token_file.as_ref())?,
                cache: TokenCache::new(),
            }));
        }

        Err(Error::with_message(ErrorKind::Credential, || {
            format!("working identity credential requires {AZURE_FEDERATED_TOKEN} or {AZURE_FEDERATED_TOKEN_FILE} environment variables")
        }))
    }

    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let token = self.token.secret().await?;
        let res: AccessToken = federated_credentials_flow::authorize(
            self.http_client.clone(),
            &self.client_id,
            &token,
            scopes,
            &self.tenant_id,
            &self.authority_host,
        )
        .await
        .map(|r| {
            AccessToken::new(
                r.access_token().clone(),
                OffsetDateTime::now_utc() + Duration::from_secs(r.expires_in),
            )
        })
        .context(ErrorKind::Credential, "request token error")?;
        Ok(res)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.cache.clear().await
    }
}

#[derive(Debug)]
enum Token {
    Value(Secret),
    File {
        path: String,
        cache: Arc<RwLock<FileCache>>,
    },
}

#[derive(Debug)]
struct FileCache {
    token: Secret,
    last_read: Instant,
}

impl Token {
    fn with_file(path: &str) -> azure_core::Result<Self> {
        let last_read = Instant::now();
        let token = std::fs::read_to_string(path).with_context(ErrorKind::Credential, || {
            format!("failed to read federated token from file {}", path)
        })?;

        Ok(Self::File {
            path: path.into(),
            cache: Arc::new(RwLock::new(FileCache {
                token: Secret::new(token),
                last_read,
            })),
        })
    }

    async fn secret(&self) -> azure_core::Result<String> {
        match self {
            Self::Value(secret) => Ok(secret.secret().into()),
            Self::File { path, cache } => {
                const TIMEOUT: Duration = Duration::from_secs(600);

                let now = Instant::now();
                let cache = cache.upgradable_read().await;
                if now - cache.last_read > TIMEOUT {
                    // TODO: https://github.com/Azure/azure-sdk-for-rust/issues/2002
                    let path = path.clone();
                    let (tx, rx) = oneshot::channel();
                    thread::spawn(move || {
                        let token = fs::read_to_string(&path)
                            .with_context(ErrorKind::Credential, || {
                                format!("failed to read federated token from file {}", &path)
                            });
                        tx.send(token)
                    });

                    let mut write_cache = RwLockUpgradableReadGuard::upgrade(cache).await;
                    let token = rx.await.map_err(|err| {
                        azure_core::Error::full(ErrorKind::Io, err, "canceled reading certificate")
                    })??;

                    write_cache.token = Secret::new(token);
                    write_cache.last_read = now;

                    return Ok(write_cache.token.secret().into());
                }

                Ok(cache.token.secret().into())
            }
        }
    }
}
