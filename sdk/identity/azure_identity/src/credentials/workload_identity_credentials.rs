// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_lock::{RwLock, RwLockUpgradableReadGuard};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential},
    error::{ErrorKind, ResultExt},
    Error,
};
use futures::channel::oneshot;
use std::{
    fs, str,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use super::{
    client_assertion_credentials::{ClientAssertion, ClientAssertionCredential},
    ClientAssertionCredentialOptions, TokenCredentialOptions,
};

const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
const AZURE_FEDERATED_TOKEN: &str = "AZURE_FEDERATED_TOKEN";

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://learn.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>
#[derive(Debug)]
pub struct WorkloadIdentityCredential(ClientAssertionCredential<Token>);

/// Options for constructing a new [`WorkloadIdentityCredential`].
#[derive(Debug, Default)]
pub struct WorkloadIdentityCredentialOptions {
    /// Options for the [`ClientAssertionCredential`] used by the [`WorkloadIdentityCredential`].
    pub credential_options: ClientAssertionCredentialOptions,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`.
    pub fn new<T>(
        tenant_id: String,
        client_id: String,
        token: T,
        options: Option<WorkloadIdentityCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>>
    where
        T: Into<Secret>,
    {
        let options = options.unwrap_or_default();
        Ok(Arc::new(Self(
            ClientAssertionCredential::<Token>::new_exclusive(
                tenant_id,
                client_id,
                Token::Value(token.into()),
                Some(options.credential_options),
            )?,
        )))
    }

    /// Create a new `WorkloadIdentityCredential` from environment variables.
    ///
    /// # Variables
    ///
    /// * `AZURE_TENANT_ID`
    /// * `AZURE_CLIENT_ID`
    /// * `AZURE_FEDERATED_TOKEN` or `AZURE_FEDERATED_TOKEN_FILE`
    pub fn from_env(
        options: Option<WorkloadIdentityCredentialOptions>,
    ) -> azure_core::Result<Arc<WorkloadIdentityCredential>> {
        let options = options.unwrap_or_default();
        let env = options.credential_options.credential_options.env();
        if let Ok(token) = env
            .var(AZURE_FEDERATED_TOKEN)
            .map_kind(ErrorKind::Credential)
        {
            return Ok(Arc::new(Self(
                ClientAssertionCredential::from_env_exclusive(
                    Token::Value(token.into()),
                    Some(options.credential_options),
                )?,
            )));
        }

        if let Ok(token_file) = env
            .var(AZURE_FEDERATED_TOKEN_FILE)
            .map_kind(ErrorKind::Credential)
        {
            return Ok(Arc::new(Self(
                ClientAssertionCredential::from_env_exclusive(
                    Token::with_file(token_file.as_ref())?,
                    Some(options.credential_options),
                )?,
            )));
        }

        Err(Error::with_message(ErrorKind::Credential, || {
            format!("working identity credential requires {AZURE_FEDERATED_TOKEN} or {AZURE_FEDERATED_TOKEN_FILE} environment variables")
        }))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.0.get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.0.clear_cache().await
    }
}

// TODO: Should probably remove this once we consolidate and unify credentials.
impl From<TokenCredentialOptions> for WorkloadIdentityCredentialOptions {
    fn from(value: TokenCredentialOptions) -> Self {
        Self {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: value,
                ..Default::default()
            },
        }
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
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl ClientAssertion for Token {
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
