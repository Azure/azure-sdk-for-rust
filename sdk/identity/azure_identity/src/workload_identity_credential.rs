// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    custom_token_proxy::{CustomTokenProxyConfig, TokenProxyClient},
    env::Env,
};
use async_lock::{RwLock, RwLockUpgradableReadGuard};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::ClientMethodOptions,
    Error,
};
use futures::channel::oneshot;
use std::{
    any::type_name,
    fmt, fs,
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
pub struct WorkloadIdentityCredential(ClientAssertionCredential<Token>);

impl fmt::Debug for WorkloadIdentityCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>()).finish_non_exhaustive()
    }
}

/// Options for constructing a new [`WorkloadIdentityCredential`].
#[derive(Default)]
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

    /// Enables Azure Kubernetes Service (AKS) identity binding proxy support.
    ///
    /// When `true`, the credential reads the proxy endpoint from `AZURE_KUBERNETES_TOKEN_PROXY` and optional
    /// TLS configuration from `AZURE_KUBERNETES_SNI_NAME`, `AZURE_KUBERNETES_CA_FILE`, and
    /// `AZURE_KUBERNETES_CA_DATA`. When `false` (the default), the credential ignores those variables and
    /// requests tokens directly from Microsoft Entra ID. See the
    /// [AKS identity bindings documentation](https://learn.microsoft.com/azure/aks/identity-bindings-concepts)
    /// for guidance about enabling this option.
    pub enable_proxy: bool,

    /// HTTP client factory for the AKS identity binding token proxy.
    ///
    /// When unset, the built-in client enabled by the `reqwest` feature is used.
    pub proxy_client: Option<Arc<dyn TokenProxyClient>>,

    #[cfg(test)]
    pub(crate) env: Env,
}

impl fmt::Debug for WorkloadIdentityCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("tenant_id", &self.tenant_id)
            .field("client_id", &self.client_id)
            .field("enable_proxy", &self.enable_proxy)
            .field("proxy_client", &self.proxy_client)
            .finish_non_exhaustive()
    }
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
        let mut credential_options = options.credential_options;
        if options.enable_proxy {
            let proxy = CustomTokenProxyConfig::from_env(&env)?;
            proxy.configure(&mut credential_options.client_options, options.proxy_client)?;
        }
        Ok(Arc::new(Self(
            ClientAssertionCredential::<Token>::new_exclusive(
                tenant_id,
                client_id,
                Token::new(path)?,
                stringify!(WorkloadIdentityCredential),
                Some(credential_options),
            )?,
        )))
    }
}

#[async_trait::async_trait]
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

#[async_trait::async_trait]
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
            headers::{HeaderName, Headers},
            policies::{Policy, PolicyResult},
            AsyncRawResponse, ClientOptions, Context, HttpClient, Method, RawResponse, Request,
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

    #[derive(Debug)]
    struct AddHeaderPolicy;

    #[derive(Debug)]
    struct MockTokenProxyClient(Arc<dyn HttpClient>);

    impl TokenProxyClient for MockTokenProxyClient {
        fn create(
            &self,
            _options: crate::TokenProxyClientOptions<'_>,
        ) -> azure_core::Result<Arc<dyn HttpClient>> {
            Ok(self.0.clone())
        }
    }

    #[async_trait::async_trait]
    impl Policy for AddHeaderPolicy {
        async fn send(
            &self,
            ctx: &Context,
            request: &mut Request,
            next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            request.headers_mut().insert("x-test-policy", "applied");
            next[0].send(ctx, request, &next[1..]).await
        }
    }

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

    #[tokio::test]
    async fn disabled_proxy_ignores_invalid_configuration() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let mock = MockSts::new(
            vec![token_response()],
            Some(Arc::new(is_valid_request(
                FAKE_PUBLIC_CLOUD_AUTHORITY.to_string(),
                Some(FAKE_ASSERTION.to_string()),
            ))),
        );
        let credential = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(temp_file.path.clone()),
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
            },
            env: Env::from(
                &[(
                    "AZURE_KUBERNETES_TOKEN_PROXY",
                    "http://insecure.example.com",
                )][..],
            ),
            ..Default::default()
        }))
        .expect("disabled proxy should ignore its environment variables");
        credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("direct Entra transport should remain configured");
    }

    #[tokio::test]
    async fn enabled_proxy_redirects_token_request_after_caller_policies() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let validate_request = is_valid_request(
            format!("https://proxy.example.com/base/{FAKE_TENANT_ID}"),
            Some(FAKE_ASSERTION.to_string()),
        );
        let mock: Arc<dyn HttpClient> = Arc::new(MockSts::new(
            vec![token_response()],
            Some(Arc::new(move |request| {
                validate_request(request)?;
                assert_eq!(
                    request
                        .headers()
                        .get_str(&HeaderName::from_static("x-test-policy"))
                        .expect("policy header"),
                    "applied"
                );
                Ok(())
            })),
        ));
        let credential = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(temp_file.path.clone()),
            enable_proxy: true,
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    per_call_policies: vec![Arc::new(AddHeaderPolicy)],
                    ..Default::default()
                },
            },
            proxy_client: Some(Arc::new(MockTokenProxyClient(mock))),
            env: Env::from(
                &[(
                    "AZURE_KUBERNETES_TOKEN_PROXY",
                    "https://proxy.example.com/base",
                )][..],
            ),
        }))
        .expect("valid proxy credential");

        let token = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .expect("proxy token response");
        assert_eq!(token.token.secret(), FAKE_TOKEN);
    }

    #[test]
    fn enabled_proxy_validates_configuration() {
        let temp_file = TempFile::new(FAKE_ASSERTION);
        let error = WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
            client_id: Some(FAKE_CLIENT_ID.to_string()),
            tenant_id: Some(FAKE_TENANT_ID.to_string()),
            token_file_path: Some(temp_file.path.clone()),
            enable_proxy: true,
            env: Env::from(
                &[(
                    "AZURE_KUBERNETES_TOKEN_PROXY",
                    "http://insecure.example.com",
                )][..],
            ),
            ..Default::default()
        }))
        .expect_err("enabled proxy should validate its environment variables");
        assert!(error.to_string().contains("AZURE_KUBERNETES_TOKEN_PROXY"));
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
        let client = azure_core::http::new_http_client(None);
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
            enable_proxy: false,
            ..Default::default()
        }))
        .expect("valid credential");

        let token = cred.get_token(LIVE_TEST_SCOPES, None).await.expect("token");
        assert_eq!(FAKE_TOKEN, token.token.secret());
        assert!(token.expires_on > SystemTime::now());
    }
}
