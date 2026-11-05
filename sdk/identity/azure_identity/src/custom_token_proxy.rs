// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env::Env;
use async_lock::{RwLock, RwLockUpgradableReadGuard};

use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    http::{AsyncRawResponse, ClientOptions, HttpClient, Request, Transport, Url},
};
use std::{
    fmt::Debug,
    fs,
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
    sync::Arc,
};

const AZURE_KUBERNETES_CA_DATA: &str = "AZURE_KUBERNETES_CA_DATA";
const AZURE_KUBERNETES_CA_FILE: &str = "AZURE_KUBERNETES_CA_FILE";
const AZURE_KUBERNETES_SNI_NAME: &str = "AZURE_KUBERNETES_SNI_NAME";
const AZURE_KUBERNETES_TOKEN_PROXY: &str = "AZURE_KUBERNETES_TOKEN_PROXY";

#[derive(Debug)]
pub(crate) struct CustomTokenProxyConfig {
    pub ca: Option<CertificateAuthority>,
    pub proxy_url: Option<Url>,
    pub sni_name: Option<String>,
}

#[derive(Debug)]
pub(crate) enum CertificateAuthority {
    Data(String),
    File(PathBuf),
}

impl CustomTokenProxyConfig {
    pub(crate) fn from_env(env: &Env) -> azure_core::Result<Self> {
        let proxy = optional_env(env, AZURE_KUBERNETES_TOKEN_PROXY);
        let sni_name = optional_env(env, AZURE_KUBERNETES_SNI_NAME);
        let ca_file = optional_env(env, AZURE_KUBERNETES_CA_FILE);
        let ca_data = optional_env(env, AZURE_KUBERNETES_CA_DATA);

        let Some(proxy) = proxy else {
            for (name, value) in [
                (AZURE_KUBERNETES_SNI_NAME, sni_name.as_ref()),
                (AZURE_KUBERNETES_CA_FILE, ca_file.as_ref()),
                (AZURE_KUBERNETES_CA_DATA, ca_data.as_ref()),
            ] {
                if value.is_some() {
                    return Err(invalid_configuration(
                        name,
                        format!("{AZURE_KUBERNETES_TOKEN_PROXY} is not set"),
                    ));
                }
            }

            return Ok(Self {
                ca: None,
                proxy_url: None,
                sni_name: None,
            });
        };

        if ca_file.is_some() && ca_data.is_some() {
            return Err(invalid_configuration(
                AZURE_KUBERNETES_CA_FILE,
                format!("cannot be set with {AZURE_KUBERNETES_CA_DATA}"),
            ));
        }

        let proxy_url = Url::parse(&proxy)
            .map_err(|err| invalid_configuration(AZURE_KUBERNETES_TOKEN_PROXY, err.to_string()))?;
        if proxy_url.scheme() != "https" {
            return Err(invalid_configuration(
                AZURE_KUBERNETES_TOKEN_PROXY,
                "must use HTTPS",
            ));
        }
        if proxy_url.host_str().is_none() {
            return Err(invalid_configuration(
                AZURE_KUBERNETES_TOKEN_PROXY,
                "must contain a host",
            ));
        }
        if !proxy_url.username().is_empty() || proxy_url.password().is_some() {
            return Err(invalid_configuration(
                AZURE_KUBERNETES_TOKEN_PROXY,
                "must not contain user information",
            ));
        }

        let ca = match (ca_file, ca_data) {
            (Some(path), None) => Some(CertificateAuthority::File(path.into())),
            (None, Some(data)) => Some(CertificateAuthority::Data(data)),
            (None, None) => None,
            (Some(_), Some(_)) => unreachable!("mutually exclusive CA sources checked above"),
        };

        Ok(Self {
            ca,
            proxy_url: Some(proxy_url),
            sni_name,
        })
    }

    pub(crate) fn configure(
        self,
        options: &mut ClientOptions,
        client: Option<Arc<dyn TokenProxyClient>>,
    ) -> azure_core::Result<()> {
        let Some(proxy_url) = self.proxy_url else {
            return Ok(());
        };
        let client = match client {
            Some(client) => client,
            None => default_token_proxy_client()?,
        };
        let transport = CustomTokenProxy::new(proxy_url, self.sni_name, self.ca, client)?;
        options.transport = Some(Transport::new(Arc::new(transport)));
        Ok(())
    }
}

/// Options for constructing an HTTP client used by the AKS identity binding token proxy.
#[derive(Debug)]
pub struct TokenProxyClientOptions<'a> {
    /// The exclusive PEM-encoded certificate authority roots, if configured.
    pub certificate_authority: Option<&'a [u8]>,

    /// The TLS server name used to connect to the proxy.
    pub server_name: &'a str,

    /// Addresses to use instead of resolving `server_name`, if configured.
    pub resolved_addresses: &'a [SocketAddr],
}

/// Constructs HTTP clients for the AKS identity binding token proxy.
///
/// Implement this trait to use an HTTP stack other than the built-in `reqwest` client.
/// The built-in client requires the `reqwest_rustls` feature, which is enabled by default.
pub trait TokenProxyClient: Debug + Send + Sync {
    /// Creates an HTTP client with the supplied TLS and name resolution options.
    fn create(
        &self,
        options: TokenProxyClientOptions<'_>,
    ) -> azure_core::Result<Arc<dyn HttpClient>>;
}

#[derive(Debug)]
struct CustomTokenProxy {
    proxy_url: Url,
    request_url: Url,
    host_header: Option<String>,
    resolved_addrs: Vec<SocketAddr>,
    ca_file: Option<PathBuf>,
    client_factory: Arc<dyn TokenProxyClient>,
    cache: RwLock<ClientCache>,
}

#[derive(Debug)]
struct ClientCache {
    client: Arc<dyn HttpClient>,
    ca_data: Option<Vec<u8>>,
}

impl CustomTokenProxy {
    fn new(
        proxy_url: Url,
        sni_name: Option<String>,
        ca: Option<CertificateAuthority>,
        client_factory: Arc<dyn TokenProxyClient>,
    ) -> azure_core::Result<Self> {
        let (request_url, host_header, resolved_addrs) =
            prepare_sni_target(&proxy_url, sni_name.as_deref())?;
        let (ca_file, ca_data) = match ca {
            Some(CertificateAuthority::Data(data)) => (None, Some(data.into_bytes())),
            Some(CertificateAuthority::File(path)) => {
                let data = read_ca_file(&path)?;
                (Some(path), Some(data))
            }
            None => (None, None),
        };
        let client = build_client(
            client_factory.as_ref(),
            &request_url,
            &resolved_addrs,
            ca_data.as_deref(),
            if ca_file.is_some() {
                Some(AZURE_KUBERNETES_CA_FILE)
            } else if ca_data.is_some() {
                Some(AZURE_KUBERNETES_CA_DATA)
            } else {
                None
            },
        )?;

        Ok(Self {
            proxy_url,
            request_url,
            host_header,
            resolved_addrs,
            ca_file,
            client_factory,
            cache: RwLock::new(ClientCache { client, ca_data }),
        })
    }

    async fn client(&self) -> azure_core::Result<Arc<dyn HttpClient>> {
        let cache = self.cache.upgradable_read().await;
        let Some(ca_file) = self.ca_file.as_deref() else {
            return Ok(cache.client.clone());
        };
        let data = fs::read(ca_file).with_context_fn(ErrorKind::Credential, || {
            format!(
                "failed to read {AZURE_KUBERNETES_CA_FILE} {}",
                ca_file.display()
            )
        })?;
        if data.is_empty() {
            return Ok(cache.client.clone());
        }
        if data != cache.ca_data.as_deref().unwrap_or_default() {
            let mut cache = RwLockUpgradableReadGuard::upgrade(cache).await;
            cache.client = build_client(
                self.client_factory.as_ref(),
                &self.request_url,
                &self.resolved_addrs,
                Some(&data),
                Some(AZURE_KUBERNETES_CA_FILE),
            )?;
            cache.ca_data = Some(data);
            return Ok(cache.client.clone());
        }
        Ok(cache.client.clone())
    }
}

#[async_trait::async_trait]
impl HttpClient for CustomTokenProxy {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<AsyncRawResponse> {
        let mut request = request.clone();
        *request.url_mut() = rewrite_proxy_url(&self.proxy_url, request.url())?;
        if let Some(host) = &self.host_header {
            request
                .url_mut()
                .set_host(self.request_url.host_str())
                .map_err(|_| {
                    invalid_configuration(AZURE_KUBERNETES_SNI_NAME, "is not a valid host name")
                })?;
            request.headers_mut().insert("host", host.clone());
        }
        self.client().await?.execute_request(&request).await
    }
}

fn build_client(
    factory: &dyn TokenProxyClient,
    request_url: &Url,
    resolved_addrs: &[SocketAddr],
    ca_data: Option<&[u8]>,
    ca_variable: Option<&'static str>,
) -> azure_core::Result<Arc<dyn HttpClient>> {
    let server_name = request_url.host_str().ok_or_else(|| {
        invalid_configuration(AZURE_KUBERNETES_SNI_NAME, "is not a valid host name")
    })?;
    factory
        .create(TokenProxyClientOptions {
            certificate_authority: ca_data,
            server_name,
            resolved_addresses: resolved_addrs,
        })
        .with_context_fn(ErrorKind::Credential, || {
            ca_variable
                .map(|name| format!("invalid {name}"))
                .unwrap_or_else(|| "failed to create token proxy client".to_string())
        })
}

#[cfg(feature = "reqwest_rustls")]
impl TokenProxyClient for reqwest::Client {
    fn create(
        &self,
        options: TokenProxyClientOptions<'_>,
    ) -> azure_core::Result<Arc<dyn HttpClient>> {
        let mut builder = reqwest::Client::builder()
            .tls_backend_rustls()
            .https_only(true)
            .redirect(reqwest::redirect::Policy::none());
        if let Some(data) = options.certificate_authority {
            let certificates = reqwest::tls::Certificate::from_pem_bundle(data).map_err(|err| {
                Error::with_error(
                    ErrorKind::Credential,
                    err,
                    "failed to parse token proxy certificate authority",
                )
            })?;
            if certificates.is_empty() {
                return Err(Error::with_message(
                    ErrorKind::Credential,
                    "token proxy certificate authority contains no certificates",
                ));
            }

            builder = builder.tls_certs_only(certificates);
        }
        if !options.resolved_addresses.is_empty() {
            builder = builder.resolve_to_addrs(options.server_name, options.resolved_addresses);
        }
        builder
            .build()
            .map(|client| Arc::new(client) as Arc<dyn HttpClient>)
            .with_context(
                ErrorKind::Credential,
                "failed to create AKS identity binding HTTP client",
            )
    }
}

#[cfg(feature = "reqwest_rustls")]
fn default_token_proxy_client() -> azure_core::Result<Arc<dyn TokenProxyClient>> {
    Ok(Arc::new(reqwest::Client::new()))
}

#[cfg(not(feature = "reqwest_rustls"))]
fn default_token_proxy_client() -> azure_core::Result<Arc<dyn TokenProxyClient>> {
    Err(Error::with_message(
        ErrorKind::Credential,
        "the `reqwest_rustls` feature is required to use the default token proxy client; enable `reqwest_rustls` or provide a custom `TokenProxyClient` implementation",
    ))
}

fn prepare_sni_target(
    proxy_url: &Url,
    sni_name: Option<&str>,
) -> azure_core::Result<(Url, Option<String>, Vec<SocketAddr>)> {
    let Some(sni_name) = sni_name else {
        return Ok((proxy_url.clone(), None, Vec::new()));
    };
    let host = proxy_url.host_str().ok_or_else(|| {
        invalid_configuration(AZURE_KUBERNETES_TOKEN_PROXY, "must contain a host")
    })?;
    let port = proxy_url.port_or_known_default().ok_or_else(|| {
        invalid_configuration(AZURE_KUBERNETES_TOKEN_PROXY, "must contain a valid port")
    })?;
    let resolved_addrs = (host, port)
        .to_socket_addrs()
        .with_context_fn(ErrorKind::Credential, || {
            format!("failed to resolve {AZURE_KUBERNETES_TOKEN_PROXY} host {host}")
        })?
        .collect::<Vec<_>>();
    if resolved_addrs.is_empty() {
        return Err(invalid_configuration(
            AZURE_KUBERNETES_TOKEN_PROXY,
            format!("host {host} resolved to no addresses"),
        ));
    }

    let mut request_url = proxy_url.clone();
    request_url.set_host(Some(sni_name)).map_err(|_| {
        invalid_configuration(AZURE_KUBERNETES_SNI_NAME, "is not a valid host name")
    })?;
    let host_header = proxy_url[url::Position::BeforeHost..url::Position::AfterPort].to_string();
    Ok((request_url, Some(host_header), resolved_addrs))
}

fn read_ca_file(path: &std::path::Path) -> azure_core::Result<Vec<u8>> {
    let data = fs::read(path).with_context_fn(ErrorKind::Credential, || {
        format!(
            "failed to read {AZURE_KUBERNETES_CA_FILE}: {}",
            path.display()
        )
    })?;
    if data.is_empty() {
        return Err(invalid_configuration(
            AZURE_KUBERNETES_CA_FILE,
            format!("{} is empty", path.display()),
        ));
    }
    Ok(data)
}

fn rewrite_proxy_url(proxy_url: &Url, request_url: &Url) -> azure_core::Result<Url> {
    let proxy_path = proxy_url.path().trim_end_matches('/');
    let request_path = request_url.path().trim_start_matches('/');
    let path = if request_path.is_empty() {
        format!("{proxy_path}/")
    } else {
        format!("{proxy_path}/{request_path}")
    };
    let mut value = proxy_url[url::Position::BeforeScheme..url::Position::AfterPort].to_string();
    value.push_str(&path);
    if let Some(query) = request_url.query() {
        value.push('?');
        value.push_str(query);
    }
    Url::parse(&value).with_context(
        ErrorKind::DataConversion,
        "failed to construct AKS identity binding proxy URL",
    )
}

#[inline(always)]
fn optional_env(env: &Env, name: &str) -> Option<String> {
    env.var(name).ok().filter(|value| !value.is_empty())
}

#[inline(always)]
fn invalid_configuration(name: &'static str, message: impl Into<String>) -> Error {
    Error::with_message(
        ErrorKind::Credential,
        format!("invalid {name}: {}", message.into()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "reqwest_rustls")]
    use crate::{
        tests::{token_response, MockSts, LIVE_TEST_SCOPES},
        WorkloadIdentityCredential, WorkloadIdentityCredentialOptions,
    };
    #[cfg(feature = "reqwest_rustls")]
    use azure_core::{
        credentials::TokenCredential,
        http::{headers::HeaderName, Method, StatusCode},
    };
    #[cfg(feature = "reqwest_rustls")]
    use futures::future::join_all;
    #[cfg(feature = "reqwest_rustls")]
    use std::{
        env,
        fs::File,
        io::Write,
        sync::atomic::{AtomicUsize, Ordering},
        time::{Duration, SystemTime},
    };

    #[cfg(feature = "reqwest_rustls")]
    #[derive(Debug)]
    struct MockTokenProxyClient {
        client: Arc<dyn HttpClient>,
        ca_data: Vec<u8>,
    }

    #[cfg(feature = "reqwest_rustls")]
    impl TokenProxyClient for MockTokenProxyClient {
        fn create(
            &self,
            options: TokenProxyClientOptions<'_>,
        ) -> azure_core::Result<Arc<dyn HttpClient>> {
            assert_eq!(options.certificate_authority, Some(self.ca_data.as_slice()));
            assert_eq!(options.server_name, "cluster.example.com");
            assert_eq!(
                options.resolved_addresses,
                &["127.0.0.1:8443".parse::<SocketAddr>().unwrap()]
            );
            Ok(self.client.clone())
        }
    }

    #[cfg(feature = "reqwest_rustls")]
    const MOCK_CA_DATA: &str = "mock CA data, not a certificate";

    #[cfg(feature = "reqwest_rustls")]
    const ROTATED_MOCK_CA_DATA: &str = "rotated mock CA data, not a certificate";

    #[cfg(feature = "reqwest_rustls")]
    #[derive(Debug, Default)]
    struct MockCaClient {
        calls: AtomicUsize,
    }

    #[cfg(feature = "reqwest_rustls")]
    impl TokenProxyClient for MockCaClient {
        fn create(
            &self,
            options: TokenProxyClientOptions<'_>,
        ) -> azure_core::Result<Arc<dyn HttpClient>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            match options.certificate_authority {
                Some(data)
                    if data == MOCK_CA_DATA.as_bytes()
                        || data == ROTATED_MOCK_CA_DATA.as_bytes() =>
                {
                    Ok(Arc::new(MockSts::new(Vec::new(), None)))
                }
                _ => Err(Error::with_message(
                    ErrorKind::Credential,
                    "mock client factory rejected CA data",
                )),
            }
        }
    }

    #[cfg(feature = "reqwest_rustls")]
    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[cfg(feature = "reqwest_rustls")]
    struct TempFile {
        path: PathBuf,
    }

    #[cfg(feature = "reqwest_rustls")]
    impl TempFile {
        fn new(content: &str) -> Self {
            let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let path = env::temp_dir().join(format!(
                "azure_identity_proxy_test_{}_{}",
                std::process::id(),
                id
            ));
            File::create(&path)
                .expect("create CA file")
                .write_all(content.as_bytes())
                .expect("write CA file");
            Self { path }
        }

        fn write(&self, content: &str) {
            File::create(&self.path)
                .expect("open CA file")
                .write_all(content.as_bytes())
                .expect("write CA file");
        }
    }

    #[cfg(feature = "reqwest_rustls")]
    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn config(pairs: &[(&str, &str)]) -> azure_core::Result<CustomTokenProxyConfig> {
        CustomTokenProxyConfig::from_env(&Env::from(pairs))
    }

    #[test]
    fn no_configuration() {
        let config = config(&[]).expect("empty configuration is valid");
        assert!(config.proxy_url.is_none());
        assert!(config.ca.is_none());
        assert!(config.sni_name.is_none());
    }

    #[test]
    fn minimal_configuration() {
        let config = config(&[(
            AZURE_KUBERNETES_TOKEN_PROXY,
            "https://kubernetes.default.svc/proxy",
        )])
        .expect("minimal configuration is valid");
        assert_eq!(
            config.proxy_url.as_ref().map(Url::as_str),
            Some("https://kubernetes.default.svc/proxy")
        );
        assert!(config.ca.is_none());
    }

    #[cfg(feature = "reqwest_rustls")]
    #[test]
    fn configures_default_client() {
        let mut options = ClientOptions::default();
        config(&[(AZURE_KUBERNETES_TOKEN_PROXY, "https://proxy.contoso.com")])
            .expect("valid proxy configuration")
            .configure(&mut options, None)
            .expect("default proxy client is available");
        assert!(options.transport.is_some());
    }

    #[cfg(not(feature = "reqwest_rustls"))]
    #[test]
    fn requires_rustls_for_default_client() {
        let mut options = ClientOptions::default();
        let error = config(&[(AZURE_KUBERNETES_TOKEN_PROXY, "https://proxy.contoso.com")])
            .expect("valid proxy configuration")
            .configure(&mut options, None)
            .expect_err("default proxy client requires rustls");
        assert_eq!(error.kind(), &ErrorKind::Credential);
        assert!(error.to_string().contains("enable `reqwest_rustls`"));
        assert!(error.to_string().contains("custom `TokenProxyClient`"));
        assert!(options.transport.is_none());
    }

    #[test]
    fn rejects_invalid_proxy_urls() {
        for value in [
            "not a URL",
            "http://kubernetes.default.svc",
            "https://user@kubernetes.default.svc",
        ] {
            let error = config(&[(AZURE_KUBERNETES_TOKEN_PROXY, value)])
                .expect_err("proxy URL should be invalid");
            assert!(error.to_string().contains(AZURE_KUBERNETES_TOKEN_PROXY));
        }
    }

    #[test]
    fn rejects_auxiliary_configuration_without_proxy() {
        for name in [
            AZURE_KUBERNETES_SNI_NAME,
            AZURE_KUBERNETES_CA_FILE,
            AZURE_KUBERNETES_CA_DATA,
        ] {
            let error =
                config(&[(name, "value")]).expect_err("auxiliary configuration requires a proxy");
            assert!(error.to_string().contains(name));
            assert!(error.to_string().contains(AZURE_KUBERNETES_TOKEN_PROXY));
        }
    }

    #[test]
    fn rejects_multiple_ca_sources() {
        let error = config(&[
            (AZURE_KUBERNETES_TOKEN_PROXY, "https://localhost"),
            (AZURE_KUBERNETES_CA_FILE, "/ca.pem"),
            (AZURE_KUBERNETES_CA_DATA, "certificate"),
        ])
        .expect_err("CA sources are mutually exclusive");
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_FILE));
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_DATA));
    }

    #[test]
    fn rewrites_request_url() {
        for (proxy, request, expected) in [
            (
                "https://proxy.contoso.com",
                "https://login.example.com/tenant/oauth2/v2.0/token?a=1&b=2",
                "https://proxy.contoso.com/tenant/oauth2/v2.0/token?a=1&b=2",
            ),
            (
                "https://proxy.contoso.com/base/",
                "https://login.example.com/a%20b?q=1",
                "https://proxy.contoso.com/base/a%20b?q=1",
            ),
            (
                "https://proxy.contoso.com/base",
                "https://login.example.com",
                "https://proxy.contoso.com/base/",
            ),
        ] {
            let actual = rewrite_proxy_url(
                &Url::parse(proxy).expect("proxy URL"),
                &Url::parse(request).expect("request URL"),
            )
            .expect("rewritten URL");
            assert_eq!(actual.as_str(), expected);
        }
    }

    #[cfg(feature = "reqwest_rustls")]
    #[test]
    fn rejects_missing_and_invalid_ca_files() {
        for path in ["/file/does/not/exist", file!()] {
            let error = CustomTokenProxy::new(
                Url::parse("https://proxy.contoso.com").expect("proxy URL"),
                None,
                Some(CertificateAuthority::File(path.into())),
                Arc::new(reqwest::Client::new()),
            )
            .expect_err("CA file should be invalid");
            assert!(error.to_string().contains(AZURE_KUBERNETES_CA_FILE));
        }
    }

    #[cfg(feature = "reqwest_rustls")]
    #[test]
    fn rejects_invalid_inline_ca_without_exposing_data() {
        let ca_data = "not a certificate";
        let error = CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::Data(ca_data.to_string())),
            Arc::new(reqwest::Client::new()),
        )
        .expect_err("CA data should be invalid");
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_DATA));
        assert!(!error.to_string().contains(ca_data));
    }

    #[cfg(feature = "reqwest_rustls")]
    #[test]
    #[ignore = "requires a real configured AKS certificate authority; see CONTRIBUTING.md"]
    fn accepts_inline_and_file_ca() -> azure_core::Result<()> {
        let config = CustomTokenProxyConfig::from_env(&Env::default())?;
        let ca_data = configured_ca_data(config.ca)?;
        CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::Data(ca_data.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid inline CA");

        let file = TempFile::new(&ca_data);
        CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid file CA");
        Ok(())
    }

    #[cfg(feature = "reqwest_rustls")]
    #[tokio::test]
    async fn reuses_and_rotates_file_ca_client() {
        let file = TempFile::new(MOCK_CA_DATA);
        let factory = Arc::new(MockCaClient::default());
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            factory.clone(),
        )
        .expect("valid file CA");
        let original = proxy.client().await.expect("cached client");
        assert!(Arc::ptr_eq(
            &original,
            &proxy.client().await.expect("reused client")
        ));

        file.write("");
        let retained = proxy.client().await.expect("last good client");
        assert!(Arc::ptr_eq(&original, &retained));

        file.write(ROTATED_MOCK_CA_DATA);
        let rotated = proxy.client().await.expect("rotated client");
        assert!(!Arc::ptr_eq(&original, &rotated));
        assert_eq!(factory.calls.load(Ordering::SeqCst), 2);
    }

    #[cfg(feature = "reqwest_rustls")]
    #[tokio::test]
    async fn serializes_concurrent_file_ca_refresh() {
        let file = TempFile::new(MOCK_CA_DATA);
        let factory = Arc::new(MockCaClient::default());
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            factory.clone(),
        )
        .expect("valid file CA");
        let original = proxy.client().await.expect("cached client");

        let clients = join_all((0..16).map(|_| proxy.client())).await;
        for client in clients {
            assert!(Arc::ptr_eq(&original, &client.expect("refreshed client")));
        }
        file.write(ROTATED_MOCK_CA_DATA);
        let clients = join_all((0..16).map(|_| proxy.client())).await;
        let rotated = proxy.client().await.expect("rotated client");
        assert!(!Arc::ptr_eq(&original, &rotated));
        for client in clients {
            assert!(Arc::ptr_eq(&rotated, &client.expect("rotated client")));
        }
        assert_eq!(factory.calls.load(Ordering::SeqCst), 2);
    }

    #[cfg(feature = "reqwest_rustls")]
    #[tokio::test]
    async fn surfaces_file_ca_refresh_errors_without_replacing_client() {
        let file = TempFile::new(MOCK_CA_DATA);
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.contoso.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(MockCaClient::default()),
        )
        .expect("valid file CA");
        let original = proxy.client().await.expect("cached client");

        file.write("rejected mock CA data");
        let error = proxy
            .client()
            .await
            .expect_err("factory rejected rotated CA");
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_FILE));
        assert!(Arc::ptr_eq(&original, &proxy.cache.read().await.client));

        fs::remove_file(&file.path).expect("remove CA file");
        let error = proxy.client().await.expect_err("missing rotated CA file");
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_FILE));
        assert!(Arc::ptr_eq(&original, &proxy.cache.read().await.client));
    }

    #[test]
    fn prepares_custom_sni_target() {
        let proxy_url = Url::parse("https://127.0.0.1:8443/base").expect("proxy URL");
        let (request_url, host_header, resolved_addrs) =
            prepare_sni_target(&proxy_url, Some("cluster.example.com")).expect("custom SNI target");
        assert_eq!(request_url.host_str(), Some("cluster.example.com"));
        assert_eq!(request_url.port(), Some(8443));
        assert_eq!(host_header.as_deref(), Some("127.0.0.1:8443"));
        assert_eq!(resolved_addrs, vec!["127.0.0.1:8443".parse().unwrap()]);
    }

    #[cfg(feature = "reqwest_rustls")]
    #[tokio::test]
    async fn sends_requests_with_custom_ca_and_sni_using_mock_client() {
        let file = TempFile::new(MOCK_CA_DATA);
        let request_count = Arc::new(AtomicUsize::new(0));
        let count = request_count.clone();
        let client = Arc::new(MockSts::new(
            vec![token_response(), token_response()],
            Some(Arc::new(move |request| {
                count.fetch_add(1, Ordering::SeqCst);
                assert_eq!(
                    request.url().as_str(),
                    "https://cluster.example.com:8443/base/tenant/token?query=value"
                );
                assert_eq!(
                    request
                        .headers()
                        .get_str(&HeaderName::from_static("host"))?,
                    "127.0.0.1:8443"
                );
                assert_eq!(request.method(), Method::Get);
                Ok(())
            })),
        ));
        let factory = Arc::new(MockTokenProxyClient {
            client,
            ca_data: MOCK_CA_DATA.as_bytes().to_vec(),
        });
        let request = Request::new(
            Url::parse("https://login.example.com/tenant/token?query=value").expect("request URL"),
            Method::Get,
        );
        for (name, value) in [
            (AZURE_KUBERNETES_CA_DATA, MOCK_CA_DATA),
            (
                AZURE_KUBERNETES_CA_FILE,
                file.path.to_str().expect("CA path"),
            ),
        ] {
            let config = config(&[
                (AZURE_KUBERNETES_TOKEN_PROXY, "https://127.0.0.1:8443/base"),
                (AZURE_KUBERNETES_SNI_NAME, "cluster.example.com"),
                (name, value),
            ])
            .expect("mock environment configuration");
            let proxy = CustomTokenProxy::new(
                config.proxy_url.expect("proxy URL"),
                config.sni_name,
                config.ca,
                factory.clone(),
            )
            .expect("proxy transport");
            let response = proxy
                .execute_request(&request)
                .await
                .expect("mock response");
            assert_eq!(response.status(), StatusCode::Ok);
        }
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
        assert_eq!(
            request.url().as_str(),
            "https://login.example.com/tenant/token?query=value"
        );
        assert!(request
            .headers()
            .get_str(&HeaderName::from_static("host"))
            .is_err());
    }

    #[cfg(feature = "reqwest_rustls")]
    #[tokio::test]
    #[ignore = "requires real AKS identity binding TLS and workload identity; see CONTRIBUTING.md"]
    async fn sends_requests_with_custom_ca_and_sni() -> azure_core::Result<()> {
        let env = Env::default();
        let config = CustomTokenProxyConfig::from_env(&env)?;
        let proxy_url = config.proxy_url.expect("set AZURE_KUBERNETES_TOKEN_PROXY");
        let sni_name = config.sni_name.expect("set AZURE_KUBERNETES_SNI_NAME");
        let ca_data = configured_ca_data(config.ca)?;
        let ca_file = TempFile::new(&ca_data);
        let tenant_id = env.var("AZURE_TENANT_ID")?;
        let client_id = env.var("AZURE_CLIENT_ID")?;
        let token_file = PathBuf::from(env.var("AZURE_FEDERATED_TOKEN_FILE")?);

        for (name, value) in [
            (AZURE_KUBERNETES_CA_DATA, ca_data.as_str()),
            (
                AZURE_KUBERNETES_CA_FILE,
                ca_file.path.to_str().expect("CA file path"),
            ),
        ] {
            let credential =
                WorkloadIdentityCredential::new(Some(WorkloadIdentityCredentialOptions {
                    tenant_id: Some(tenant_id.clone()),
                    client_id: Some(client_id.clone()),
                    token_file_path: Some(token_file.clone()),
                    enable_proxy: true,
                    env: Env::from(
                        &[
                            (AZURE_KUBERNETES_TOKEN_PROXY, proxy_url.as_str()),
                            (AZURE_KUBERNETES_SNI_NAME, sni_name.as_str()),
                            (name, value),
                        ][..],
                    ),
                    ..Default::default()
                }))?;
            let token = tokio::time::timeout(
                Duration::from_secs(60),
                credential.get_token(LIVE_TEST_SCOPES, None),
            )
            .await
            .expect("AKS identity binding token request timed out")?;
            assert!(!token.token.secret().is_empty());
            assert!(token.expires_on > SystemTime::now());
        }
        Ok(())
    }

    #[cfg(feature = "reqwest_rustls")]
    fn configured_ca_data(ca: Option<CertificateAuthority>) -> azure_core::Result<String> {
        match ca {
            Some(CertificateAuthority::Data(data)) => Ok(data),
            Some(CertificateAuthority::File(path)) => Ok(String::from_utf8(read_ca_file(&path)?)?),
            None => Err(Error::with_message(
                ErrorKind::Credential,
                "set AZURE_KUBERNETES_CA_FILE or AZURE_KUBERNETES_CA_DATA; see CONTRIBUTING.md",
            )),
        }
    }
}
