// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env::Env;
use async_lock::{RwLock, RwLockUpgradableReadGuard};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::ClientMethodOptions,
    Error,
};
use futures::channel::oneshot;
use std::{
    fs,
    path::PathBuf,
    str,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use super::{ClientAssertion, ClientAssertionCredential, ClientAssertionCredentialOptions};

const AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
const AZURE_FEDERATED_TOKEN_FILE: &str = "AZURE_FEDERATED_TOKEN_FILE";
const AZURE_TENANT_ID: &str = "AZURE_TENANT_ID";

/// Authenticates an [Entra Workload Identity on Kubernetes](https://learn.microsoft.com/azure/aks/workload-identity-overview).
#[derive(Debug)]
pub struct WorkloadIdentityCredential(ClientAssertionCredential<Token>);

/// Options for constructing a new [`WorkloadIdentityCredential`].
#[derive(Debug, Default)]
pub struct WorkloadIdentityCredentialOptions {
    /// Options for the [`ClientAssertionCredential`] used by the [`WorkloadIdentityCredential`].
    pub credential_options: ClientAssertionCredentialOptions,

    /// Client ID of the Entra identity. Defaults to the value of the environment variable `AZURE_CLIENT_ID`.
    pub client_id: Option<String>,

    /// Tenant ID of the Entra identity. Defaults to the value of the environment variable `AZURE_TENANT_ID`.
    pub tenant_id: Option<String>,

    /// Path of a file containing a Kubernetes service account token. Defaults to the value of the environment
    /// variable `AZURE_FEDERATED_TOKEN_FILE`.
    pub token_file_path: Option<PathBuf>,

    #[cfg(test)]
    pub(crate) env: Env,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`.
    pub fn new(
        options: Option<WorkloadIdentityCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        let options = options.unwrap_or_default();
        #[cfg(test)]
        let env = options.env;
        #[cfg(not(test))]
        let env = Env::default();
        let tenant_id = match options.tenant_id {
            Some(id) => id,
            None => env.var(AZURE_TENANT_ID).with_context_fn(ErrorKind::Credential, || {
                "no tenant ID specified. Check pod configuration or set tenant_id in the options"
            })?
        };
        crate::validate_tenant_id(&tenant_id)?;
        let path = match options.token_file_path {
            Some(path) => path,
            None => env.var(AZURE_FEDERATED_TOKEN_FILE).map(PathBuf::from).with_context_fn(ErrorKind::Credential, || {
                "no token file specified. Check pod configuration or set token_file_path in the options"
            })?
        };
        let client_id = match options.client_id {
            Some(id) => id,
            None => env.var(AZURE_CLIENT_ID).with_context_fn(ErrorKind::Credential, || {
                "no client id specified. Check pod configuration or set client_id in the options"
            })?
        };
        Ok(Arc::new(Self(
            ClientAssertionCredential::<Token>::new_exclusive(
                tenant_id,
                client_id,
                Token::new(path)?,
                stringify!(WorkloadIdentityCredential),
                Some(options.credential_options),
            )?,
        )))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        if scopes.is_empty() {
            return Err(Error::with_message(
                ErrorKind::Credential,
                "no scopes specified",
            ));
        }
        self.0.get_token(scopes, options).await
    }
}

#[derive(Debug)]
struct Token {
    path: PathBuf,
    cache: Arc<RwLock<FileCache>>,
}

#[derive(Debug)]
struct FileCache {
    token: Secret,
    last_read: Instant,
}

impl Token {
    fn new(path: PathBuf) -> azure_core::Result<Self> {
        let last_read = Instant::now();
        let token =
            std::fs::read_to_string(&path).with_context_fn(ErrorKind::Credential, || {
                format!(
                    "failed to read federated token from file {}",
                    path.display()
                )
            })?;

        Ok(Self {
            path,
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
    async fn secret(&self, _: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
        const TIMEOUT: Duration = Duration::from_secs(600);

        let now = Instant::now();
        let cache = self.cache.upgradable_read().await;
        if now - cache.last_read > TIMEOUT {
            // TODO: https://github.com/Azure/azure-sdk-for-rust/issues/2002
            let path = self.path.clone();
            let (tx, rx) = oneshot::channel();
            thread::spawn(move || {
                let token =
                    fs::read_to_string(&path).with_context_fn(ErrorKind::Credential, || {
                        format!(
                            "failed to read federated token from file {}",
                            path.display()
                        )
                    });
                tx.send(token)
            });

            let mut write_cache = RwLockUpgradableReadGuard::upgrade(cache).await;
            let token = rx.await.map_err(|err| {
                azure_core::Error::with_error(ErrorKind::Io, err, "canceled reading certificate")
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
    use crate::{
        client_assertion_credential::tests::{is_valid_request, FAKE_ASSERTION},
        env::Env,
        tests::*,
    };
    use azure_core::{
        http::{
            headers::Headers, AsyncRawResponse, ClientOptions, Method, RawResponse, Request,
            StatusCode, Transport, Url,
        },
        Bytes,
    };
    use azure_core_test::recorded;
    use std::{
        env,
        fs::File,
        io::Write,
        sync::atomic::{AtomicUsize, Ordering},
        time::SystemTime,
    };

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    pub struct TempFile {
        pub path: PathBuf,
    }

    impl TempFile {
        pub fn new(content: &str) -> Self {
            let n = TEMP_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = env::temp_dir().join(format!("azure_identity_test_{}", n));
            File::create(&path)
                .expect("create temp file")
                .write_all(content.as_bytes())
                .expect("write temp file");

            Self { path }
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    #[tokio::test]
    async fn env_vars() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let mock = MockSts::new(
            vec![AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":3600,"ext_expires_in":3600,"token_type":"Bearer"}}"#,
                    FAKE_TOKEN
                )),
            )],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
        );
        let cred = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
            },
            env: Env::from(
                &[
                    (AZURE_CLIENT_ID, FAKE_CLIENT_ID),
                    (AZURE_TENANT_ID, FAKE_TENANT_ID),
                    (AZURE_FEDERATED_TOKEN_FILE, temp_file.path.to_str().unwrap()),
                ][..],
            ),
            ..Default::default()
        }))
        .expect("valid credential");

        let token = cred.get_token(LIVE_TEST_SCOPES, None).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
    }

    #[tokio::test]
    async fn get_token_error() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let expected_status = StatusCode::Forbidden;
        let body = r#"{"error":"invalid_request","error_description":"invalid assertion"}"#;
        let mut headers = Headers::default();
        headers.insert("key", "value");
        let expected_response = RawResponse::from_bytes(expected_status, headers.clone(), body);
        let mock = MockSts::new(
            vec![AsyncRawResponse::from_bytes(
                expected_status,
                headers.clone(),
                Bytes::from(body),
            )],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
        );
        let cred = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
            },
            env: Env::from(
                &[
                    (AZURE_CLIENT_ID, FAKE_CLIENT_ID),
                    (AZURE_TENANT_ID, FAKE_TENANT_ID),
                    (AZURE_FEDERATED_TOKEN_FILE, temp_file.path.to_str().unwrap()),
                ][..],
            ),
            ..Default::default()
        }))
        .expect("valid credential");

        let err = cred
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect_err("expected error");

        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert_eq!(
            "WorkloadIdentityCredential authentication failed. invalid assertion\nTo troubleshoot, visit https://aka.ms/azsdk/rust/identity/troubleshoot#workload",
             err.to_string(),
        );
        match err
            .downcast_ref::<azure_core::Error>()
            .expect("returned error should wrap an azure_core::Error")
            .kind()
        {
            ErrorKind::HttpResponse {
                error_code: None,
                raw_response: Some(response),
                status,
                ..
            } => {
                assert_eq!(&expected_response, response.as_ref());
                assert_eq!(expected_status, *status);
            }
            kind => panic!("unexpected ErrorKind {:?}", kind),
        };
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

    #[recorded::test(live)]
    async fn live() -> azure_core::Result<()> {
        if env::var("CI_HAS_DEPLOYED_RESOURCES").is_err() {
            println!("Skipped: workload identity live tests require deployed resources");
            return Ok(());
        }
        let ip = env::var("IDENTITY_AKS_IP").expect("IDENTITY_AKS_IP");
        let storage_name = env::var("IDENTITY_STORAGE_NAME_USER_ASSIGNED")
            .expect("IDENTITY_STORAGE_NAME_USER_ASSIGNED");

        let url =
            format!("http://{ip}:8080/api?test=workload-identity&storage-name={storage_name}");
        let u = Url::parse(&url).expect("valid URL");
        let client = azure_core::http::new_http_client();
        let req = Request::new(u, Method::Get);

        let res = client.execute_request(&req).await.expect("response");
        let status = res.status();
        let body = res
            .into_body()
            .collect_string()
            .await
            .expect("body content");

        assert_eq!(StatusCode::Ok, status, "Test app responded with '{body}'");

        Ok(())
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
        .get_token(&[], None)
        .await
        .expect_err("no scopes specified");
    }

    #[tokio::test]
    async fn options_override_env() {
        let right_file = TempFile::new(FAKE_ASSERTION);
        let wrong_file = TempFile::new("wrong assertion");
        let mock = MockSts::new(
            vec![AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::default(),
                Bytes::from(format!(
                    r#"{{"access_token":"{}","expires_in":3600,"ext_expires_in":3600,"token_type":"Bearer"}}"#,
                    FAKE_TOKEN
                )),
            )],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
        );
        let cred = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(right_file.path.clone()),
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
            },
            env: Env::from(
                &[
                    (AZURE_CLIENT_ID, "wrong-client-id"),
                    (AZURE_TENANT_ID, "wrong-tenant-id"),
                    (
                        AZURE_FEDERATED_TOKEN_FILE,
                        wrong_file.path.to_str().unwrap(),
                    ),
                ][..],
            ),
        }))
        .expect("valid credential");

        let token = cred.get_token(LIVE_TEST_SCOPES, None).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
    }
}
