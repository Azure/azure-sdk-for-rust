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

const AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
const AZURE_TENANT_ID: &str = "AZURE_TENANT_ID";

/// WorkloadIdentityCredential supports Azure workload identity on Kubernetes. See
/// [Azure Kubernetes Service documentation](https://learn.microsoft.com/azure/aks/workload-identity-overview)
/// for more information.
#[derive(Debug)]
pub struct WorkloadIdentityCredential(ClientAssertionCredential<Token>);

/// Options for constructing a new [`WorkloadIdentityCredential`].
#[derive(Debug, Default)]
pub struct WorkloadIdentityCredentialOptions {
    /// Options for the [`ClientAssertionCredential`] used by the [`WorkloadIdentityCredential`].
    pub credential_options: ClientAssertionCredentialOptions,

    /// Client ID of the Entra identity. Defaults to the value of the environment variable AZURE_CLIENT_ID.
    pub client_id: Option<String>,

    /// Tenant ID of the Entra identity. Defaults to the value of the environment variable AZURE_TENANT_ID.
    pub tenant_id: Option<String>,

    /// Path of a file containing a Kubernetes service account token. Defaults to the value of the environment
    /// variable AZURE_FEDERATED_TOKEN_FILE.
    pub token_file_path: Option<String>,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`.
    pub fn new(
        options: Option<WorkloadIdentityCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        let options = options.unwrap_or_default();
        let env = options.credential_options.credential_options.env();
        let tenant_id = match options.tenant_id {
            Some(id) => id,
            None => env.var(AZURE_TENANT_ID).with_context(ErrorKind::Credential, || {
                "no tenant ID specified. Check pod configuration or set tenant_id in the options"
            })?
        };
        crate::validate_tenant_id(&tenant_id)?;
        let path = match options.token_file_path {
            Some(path) => path,
            None => env.var(AZURE_FEDERATED_TOKEN_FILE).with_context(ErrorKind::Credential, || {
                "no token file specified. Check pod configuration or set token_file_path in the options"
            })?
        };
        let client_id = match options.client_id {
            Some(id) => id,
            None => env.var(AZURE_CLIENT_ID).with_context(ErrorKind::Credential, || {
                "no client id specified. Check pod configuration or set client_id in the options"
            })?
        };
        Ok(Arc::new(Self(
            ClientAssertionCredential::<Token>::new_exclusive(
                tenant_id,
                client_id,
                Token::new(&path)?,
                Some(options.credential_options),
            )?,
        )))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::message(ErrorKind::Credential, "no scopes specified"));
        }
        self.0.get_token(scopes).await
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
            ..Default::default()
        }
    }
}

#[derive(Debug)]
struct Token {
    path: String,
    cache: Arc<RwLock<FileCache>>,
}

#[derive(Debug)]
struct FileCache {
    token: Secret,
    last_read: Instant,
}

impl Token {
    fn new(path: &str) -> azure_core::Result<Self> {
        let last_read = Instant::now();
        let token = std::fs::read_to_string(path).with_context(ErrorKind::Credential, || {
            format!("failed to read federated token from file {}", path)
        })?;

        Ok(Self {
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
        const TIMEOUT: Duration = Duration::from_secs(600);

        let now = Instant::now();
        let cache = self.cache.upgradable_read().await;
        if now - cache.last_read > TIMEOUT {
            // TODO: https://github.com/Azure/azure-sdk-for-rust/issues/2002
            let path = self.path.clone();
            let (tx, rx) = oneshot::channel();
            thread::spawn(move || {
                let token = fs::read_to_string(&path).with_context(ErrorKind::Credential, || {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, tests::*};
    use azure_core::{
        authority_hosts::AZURE_PUBLIC_CLOUD,
        http::{
            headers::{self, content_type, Headers},
            Body, Method, Request, Response, StatusCode,
        },
        Bytes,
    };
    use std::{
        collections::HashMap,
        env,
        fs::File,
        io::Write,
        time::{SystemTime, UNIX_EPOCH},
    };
    use url::form_urlencoded;

    const FAKE_ASSERTION: &str = "fake assertion";

    pub struct TempFile {
        pub path: String,
    }

    impl TempFile {
        pub fn new(content: &str) -> Self {
            let n = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            let path = env::temp_dir().join(format!("azure_identity_test_{}", n));
            File::create(&path)
                .expect("create temp file")
                .write_all(content.as_bytes())
                .expect("write temp file");

            Self {
                path: path.to_string_lossy().to_string(),
            }
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn is_valid_request() -> impl Fn(&Request) -> azure_core::Result<()> {
        let expected_url = format!(
            "{}{}/oauth2/v2.0/token",
            AZURE_PUBLIC_CLOUD.as_str(),
            FAKE_TENANT_ID
        );
        move |req: &Request| {
            assert_eq!(&Method::Post, req.method());
            assert_eq!(expected_url, req.url().to_string());
            assert_eq!(
                req.headers().get_str(&headers::CONTENT_TYPE).unwrap(),
                content_type::APPLICATION_X_WWW_FORM_URLENCODED.as_str()
            );
            let expected_params = [
                ("client_assertion", FAKE_ASSERTION),
                (
                    "client_assertion_type",
                    "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                ),
                ("client_id", FAKE_CLIENT_ID),
                ("grant_type", "client_credentials"),
                ("scope", &LIVE_TEST_SCOPES.join(" ")),
            ];
            let body = match req.body() {
                Body::Bytes(bytes) => str::from_utf8(bytes).unwrap(),
                _ => panic!("unexpected body type"),
            };
            let actual_params: HashMap<String, String> = form_urlencoded::parse(body.as_bytes())
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            for (key, value) in expected_params.iter() {
                assert_eq!(
                    *value,
                    actual_params
                        .get(*key)
                        .unwrap_or_else(|| panic!("no {} in request body", key))
                );
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn env_vars() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let mock = MockSts::new(
            vec![Response::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":3600,"ext_expires_in":3600,"token_type":"Bearer"}}"#,
                    FAKE_TOKEN
                )),
            )],
            Some(Arc::new(is_valid_request())),
        );
        let cred = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: TokenCredentialOptions {
                    env: Env::from(
                        &[
                            (AZURE_CLIENT_ID, FAKE_CLIENT_ID),
                            (AZURE_TENANT_ID, FAKE_TENANT_ID),
                            (AZURE_FEDERATED_TOKEN_FILE, temp_file.path.as_str()),
                        ][..],
                    ),
                    http_client: Arc::new(mock),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }))
        .expect("valid credential");

        let token = cred.get_token(LIVE_TEST_SCOPES).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
    }

    #[test]
    fn invalid_tenant_id() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some("not a valid tenant".to_string()),
            token_file_path: Some(temp_file.path.clone()),
            ..Default::default()
        }))
        .expect_err("invalid tenant ID");
    }

    #[test]
    fn missing_config() {
        WorkloadIdentityCredential::new(None).expect_err("missing config");
    }

    #[tokio::test]
    async fn no_scopes() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(temp_file.path.clone()),
            ..Default::default()
        }))
        .expect("valid credential")
        .get_token(&[])
        .await
        .expect_err("no scopes specified");
    }

    #[tokio::test]
    async fn options_override_env() {
        let right_file = TempFile::new(FAKE_ASSERTION);
        let wrong_file = TempFile::new("wrong assertion");
        let mock = MockSts::new(
            vec![Response::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":3600,"ext_expires_in":3600,"token_type":"Bearer"}}"#,
                    FAKE_TOKEN
                )),
            )],
            Some(Arc::new(is_valid_request())),
        );
        let cred = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(right_file.path.clone()),
            credential_options: ClientAssertionCredentialOptions {
                credential_options: TokenCredentialOptions {
                    env: Env::from(
                        &[
                            (AZURE_CLIENT_ID, "wrong-client-id"),
                            (AZURE_TENANT_ID, "wrong-tenant-id"),
                            (AZURE_FEDERATED_TOKEN_FILE, wrong_file.path.as_str()),
                        ][..],
                    ),
                    http_client: Arc::new(mock),
                    ..Default::default()
                },
                ..Default::default()
            },
        }))
        .expect("valid credential");

        let token = cred.get_token(LIVE_TEST_SCOPES).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
    }
}
