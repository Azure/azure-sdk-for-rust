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

#[cfg(feature = "reqwest")]
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

#[cfg(feature = "reqwest")]
fn default_token_proxy_client() -> azure_core::Result<Arc<dyn TokenProxyClient>> {
    Ok(Arc::new(reqwest::Client::new()))
}

#[cfg(not(feature = "reqwest"))]
fn default_token_proxy_client() -> azure_core::Result<Arc<dyn TokenProxyClient>> {
    Err(Error::with_message(
        ErrorKind::Credential,
        "the `reqwest` feature is required to use the default token proxy client; enable `reqwest` or provide a custom `TokenProxyClient` implementation",
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
    #[cfg(feature = "reqwest")]
    use futures::future::join_all;
    #[cfg(all(feature = "reqwest", feature = "client_certificate"))]
    use openssl::{
        asn1::{Asn1Integer, Asn1Time},
        bn::{BigNum, MsbOption},
        hash::MessageDigest,
        nid::Nid,
        pkey::PKey,
        rsa::Rsa,
        ssl::{NameType, SslAcceptor, SslMethod},
        x509::{
            extension::{BasicConstraints, ExtendedKeyUsage, KeyUsage, SubjectAlternativeName},
            X509NameBuilder, X509,
        },
    };
    #[cfg(feature = "reqwest")]
    use std::{
        env,
        fs::File,
        io::Write,
        sync::atomic::{AtomicUsize, Ordering},
    };
    #[cfg(all(feature = "reqwest", feature = "client_certificate"))]
    use std::{io::Read, net::TcpListener, sync::mpsc, thread};

    #[cfg(feature = "reqwest")]
    const TEST_CA: &str = "-----BEGIN CERTIFICATE-----\n\
MIIDZzCCAk+gAwIBAgIUPXdgRBlS4T18QnYJ/+yPV70GOEEwDQYJKoZIhvcNAQEL\n\
BQAwFDESMBAGA1UEAwwJbG9jYWxob3N0MB4XDTI2MDcyMTIwNTYxOVoXDTI3MDcy\n\
MTIwNTYxOVowFDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEF\n\
AAOCAQ8AMIIBCgKCAQEAohtW1OHr/XIAlhxXq+vhvbosa/MvCptI8Pb1eJApnhYk\n\
Zt3wGGMfjPPga4z+a7NSz5v2xD9qhHyMVNrlnt6becBCLm8Az3Q7zdpu6Cp+mEAc\n\
VMLY/ttiPQfMKdj33aJxXZfqtFw++jm5kUCawW6OlvfcmZCVhMp5LQvDbVWULa5v\n\
nsdzAoghf1RPZWyMXSme0vkfZaDN6LuLxhbXQOz9AVHnfX4eXvXO8UAhCV3xTsXU\n\
KqkzzxPZX5Bt6/PEo1Nmp9YhmCYaLrljAr9ShTHdczfCPWJvGtYnSnbzCtapVffe\n\
u/YK2l4uBWP6Nx0xjoXrrA3hM7qZdhinmmQsz870AQIDAQABo4GwMIGtMA8GA1Ud\n\
EwEB/wQFMAMBAf8wDgYDVR0PAQH/BAQDAgGmMBYGA1UdJQEB/wQMMAoGCCsGAQUF\n\
BwMBMBcGA1UdEQEB/wQNMAuCCWxvY2FsaG9zdDA6BgorBgEEAYI3VAEBBCwMKkFT\n\
UC5ORVQgQ29yZSBIVFRQUyBkZXZlbG9wbWVudCBjZXJ0aWZpY2F0ZTAdBgNVHQ4E\n\
FgQUndm3u54Kli+UWZSuG6zjDMf07r0wDQYJKoZIhvcNAQELBQADggEBAIx4ssZM\n\
ET31rNiqhcArt0RP7Yxe59RxIPVWlsh0O3Bh/cT1Q5ESmSs9CA6jaVSkNhJQFF3x\n\
qKz/PaG1an8f6YDTZfb1Eu1xL5E9t26GkjKovmOwZporaQm+d367sCK2Hab/5aJG\n\
bqH23P5sbJQ+TogAf0Uykdq9rSx/5uwQBEv53tAHpSLOQXDWtNXo6AGNcyuouTgt\n\
v/X15v4Gb9clgZpl3WXCvzOtEpaRSdf8dL76KKIiyClOzdvNP4/BpXxsYfAPU4hb\n\
CesVElsCj5WckSkJ23gnTkzIAAeWjNnf+sOwaMgfsqh/XtKzYluV8MtbBljuOz0G\n\
uaZPC0VV2qRwbAE=\n\
-----END CERTIFICATE-----\n";

    #[cfg(feature = "reqwest")]
    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[cfg(feature = "reqwest")]
    struct TempFile {
        path: PathBuf,
    }

    #[cfg(feature = "reqwest")]
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

    #[cfg(feature = "reqwest")]
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
                "https://proxy.example.com",
                "https://login.example.com/tenant/oauth2/v2.0/token?a=1&b=2",
                "https://proxy.example.com/tenant/oauth2/v2.0/token?a=1&b=2",
            ),
            (
                "https://proxy.example.com/base/",
                "https://login.example.com/a%20b?q=1",
                "https://proxy.example.com/base/a%20b?q=1",
            ),
            (
                "https://proxy.example.com/base",
                "https://login.example.com",
                "https://proxy.example.com/base/",
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

    #[cfg(feature = "reqwest")]
    #[test]
    fn rejects_missing_and_invalid_ca_files() {
        for path in ["/file/does/not/exist", file!()] {
            let error = CustomTokenProxy::new(
                Url::parse("https://proxy.example.com").expect("proxy URL"),
                None,
                Some(CertificateAuthority::File(path.into())),
                Arc::new(reqwest::Client::new()),
            )
            .expect_err("CA file should be invalid");
            assert!(error.to_string().contains(AZURE_KUBERNETES_CA_FILE));
        }
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn rejects_invalid_inline_ca_without_exposing_data() {
        let ca_data = "not a certificate";
        let error = CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::Data(ca_data.to_string())),
            Arc::new(reqwest::Client::new()),
        )
        .expect_err("CA data should be invalid");
        assert!(error.to_string().contains(AZURE_KUBERNETES_CA_DATA));
        assert!(!error.to_string().contains(ca_data));
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn accepts_inline_and_file_ca() {
        CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::Data(TEST_CA.to_string())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid inline CA");

        let file = TempFile::new(TEST_CA);
        CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid file CA");
    }

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn reuses_and_rotates_file_ca_client() {
        let file = TempFile::new(TEST_CA);
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(reqwest::Client::new()),
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

        file.write(&format!("{TEST_CA}\n"));
        let rotated = proxy.client().await.expect("rotated client");
        assert!(!Arc::ptr_eq(&original, &rotated));
    }

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn serializes_concurrent_file_ca_refresh() {
        let file = TempFile::new(TEST_CA);
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid file CA");
        let original = proxy.client().await.expect("cached client");

        let clients = join_all((0..16).map(|_| proxy.client())).await;
        for client in clients {
            assert!(Arc::ptr_eq(&original, &client.expect("refreshed client")));
        }
    }

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn surfaces_file_ca_refresh_errors_without_replacing_client() {
        let file = TempFile::new(TEST_CA);
        let proxy = CustomTokenProxy::new(
            Url::parse("https://proxy.example.com").expect("proxy URL"),
            None,
            Some(CertificateAuthority::File(file.path.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("valid file CA");
        let original = proxy.client().await.expect("cached client");

        file.write("not a certificate");
        let error = proxy.client().await.expect_err("invalid rotated CA");
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

    #[cfg(all(feature = "reqwest", feature = "client_certificate"))]
    #[tokio::test]
    async fn sends_requests_with_custom_ca_and_sni() {
        const SNI_NAME: &str = "cluster.example.com";
        let (certificate, key, ca_certificate) = test_server_certificate(SNI_NAME);
        let ca_data = String::from_utf8(ca_certificate.to_pem().expect("CA certificate PEM"))
            .expect("PEM is UTF-8");
        let mut acceptor =
            SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server()).expect("TLS acceptor");
        acceptor.set_certificate(&certificate).expect("certificate");
        acceptor.set_private_key(&key).expect("private key");
        acceptor.check_private_key().expect("matching private key");
        let acceptor = acceptor.build();
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind HTTPS server");
        let address = listener.local_addr().expect("server address");
        let (sender, receiver) = mpsc::channel();
        let server = thread::spawn(move || {
            for _ in 0..2 {
                let (stream, _) = listener.accept().expect("TLS connection");
                let mut stream = acceptor.accept(stream).expect("TLS handshake");
                let sni = stream
                    .ssl()
                    .servername(NameType::HOST_NAME)
                    .map(str::to_string);
                let mut request = Vec::new();
                let mut buffer = [0_u8; 1024];
                while !request.windows(4).any(|value| value == b"\r\n\r\n") {
                    let read = stream.read(&mut buffer).expect("read request");
                    assert_ne!(read, 0, "connection closed before request headers");
                    request.extend_from_slice(&buffer[..read]);
                }
                stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
                    .expect("write response");
                sender
                    .send((sni, String::from_utf8(request).expect("HTTP is UTF-8")))
                    .expect("send captured request");
            }
        });

        let proxy_url = Url::parse(&format!("https://{address}/base")).expect("proxy URL");
        let inline_proxy = CustomTokenProxy::new(
            proxy_url.clone(),
            Some(SNI_NAME.to_string()),
            Some(CertificateAuthority::Data(ca_data.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("proxy transport");
        let request = Request::new(
            Url::parse("https://login.example.com/tenant/token?query=value").expect("request URL"),
            azure_core::http::Method::Get,
        );
        let response = inline_proxy
            .execute_request(&request)
            .await
            .expect("inline CA HTTPS response");
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);

        let ca_file = TempFile::new(&ca_data);
        let file_proxy = CustomTokenProxy::new(
            proxy_url,
            Some(SNI_NAME.to_string()),
            Some(CertificateAuthority::File(ca_file.path.clone())),
            Arc::new(reqwest::Client::new()),
        )
        .expect("proxy transport");
        let response = file_proxy
            .execute_request(&request)
            .await
            .expect("file CA HTTPS response");
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);

        server.join().expect("HTTPS server");
        for _ in 0..2 {
            let (sni, request) = receiver.recv().expect("captured request");
            assert_eq!(sni.as_deref(), Some(SNI_NAME));
            assert!(request.starts_with("GET /base/tenant/token?query=value HTTP/1.1\r\n"));
            assert!(request.contains(&format!("\r\nhost: {address}\r\n")));
        }
    }

    #[cfg(all(feature = "reqwest", feature = "client_certificate"))]
    fn test_server_certificate(name: &str) -> (X509, PKey<openssl::pkey::Private>, X509) {
        let ca_key =
            PKey::from_rsa(Rsa::generate(2048).expect("CA RSA key")).expect("CA private key");
        let mut ca_subject = X509NameBuilder::new().expect("CA subject");
        ca_subject
            .append_entry_by_nid(Nid::COMMONNAME, "test CA")
            .expect("CA common name");
        let ca_subject = ca_subject.build();
        let mut serial = BigNum::new().expect("serial");
        serial
            .rand(128, MsbOption::MAYBE_ZERO, false)
            .expect("random serial");
        let serial = Asn1Integer::from_bn(&serial).expect("ASN.1 serial");

        let mut ca_certificate = X509::builder().expect("CA certificate builder");
        ca_certificate
            .set_version(2)
            .expect("CA certificate version");
        ca_certificate
            .set_serial_number(&serial)
            .expect("CA certificate serial");
        ca_certificate
            .set_subject_name(&ca_subject)
            .expect("CA certificate subject");
        ca_certificate
            .set_issuer_name(&ca_subject)
            .expect("CA certificate issuer");
        ca_certificate
            .set_pubkey(&ca_key)
            .expect("CA certificate key");
        ca_certificate
            .set_not_before(&Asn1Time::days_from_now(0).expect("CA not before"))
            .expect("CA not before");
        ca_certificate
            .set_not_after(&Asn1Time::days_from_now(1).expect("CA not after"))
            .expect("CA not after");
        ca_certificate
            .append_extension(BasicConstraints::new().critical().ca().build().expect("CA"))
            .expect("CA extension");
        ca_certificate
            .append_extension(
                KeyUsage::new()
                    .critical()
                    .key_cert_sign()
                    .crl_sign()
                    .build()
                    .expect("CA key usage"),
            )
            .expect("CA key usage extension");
        ca_certificate
            .sign(&ca_key, MessageDigest::sha256())
            .expect("sign CA certificate");
        let ca_certificate = ca_certificate.build();

        let key = PKey::from_rsa(Rsa::generate(2048).expect("server RSA key"))
            .expect("server private key");
        let mut subject = X509NameBuilder::new().expect("server subject");
        subject
            .append_entry_by_nid(Nid::COMMONNAME, name)
            .expect("server common name");
        let subject = subject.build();
        let mut serial = BigNum::new().expect("server serial");
        serial
            .rand(128, MsbOption::MAYBE_ZERO, false)
            .expect("random server serial");
        let serial = Asn1Integer::from_bn(&serial).expect("ASN.1 server serial");

        let mut certificate = X509::builder().expect("server certificate builder");
        certificate
            .set_version(2)
            .expect("server certificate version");
        certificate
            .set_serial_number(&serial)
            .expect("server certificate serial");
        certificate
            .set_subject_name(&subject)
            .expect("server certificate subject");
        certificate
            .set_issuer_name(ca_certificate.subject_name())
            .expect("server certificate issuer");
        certificate
            .set_pubkey(&key)
            .expect("server certificate key");
        certificate
            .set_not_before(&Asn1Time::days_from_now(0).expect("not before"))
            .expect("not before");
        certificate
            .set_not_after(&Asn1Time::days_from_now(1).expect("not after"))
            .expect("not after");
        certificate
            .append_extension(
                BasicConstraints::new()
                    .critical()
                    .build()
                    .expect("server constraints"),
            )
            .expect("server constraints extension");
        certificate
            .append_extension(
                KeyUsage::new()
                    .critical()
                    .digital_signature()
                    .key_encipherment()
                    .build()
                    .expect("server key usage"),
            )
            .expect("server key usage extension");
        certificate
            .append_extension(
                ExtendedKeyUsage::new()
                    .server_auth()
                    .build()
                    .expect("extended key usage"),
            )
            .expect("extended key usage extension");
        let subject_alt_name = SubjectAlternativeName::new()
            .dns(name)
            .build(&certificate.x509v3_context(None, None))
            .expect("subject alternative name");
        certificate
            .append_extension(subject_alt_name)
            .expect("subject alternative name extension");
        certificate
            .sign(&ca_key, MessageDigest::sha256())
            .expect("sign server certificate");
        (certificate.build(), key, ca_certificate)
    }
}
