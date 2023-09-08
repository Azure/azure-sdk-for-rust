use crate::{clients::pipeline::new_pipeline_from_options, prelude::*};
use azure_core::{
    auth::TokenCredential,
    date,
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    Body, Context, Method, Pipeline, Request, Response,
};
use const_format::formatcp;
use std::sync::Arc;
use time::OffsetDateTime;
use url::Url;

pub const API_VERSION: &str = "7.0";
const API_VERSION_PARAM: &str = formatcp!("api-version={}", API_VERSION);

/// Client for Key Vault operations - getting a secret, listing secrets, etc.
///
/// # Example
///
/// ```no_run
/// use azure_security_keyvault::KeyvaultClient;
/// use azure_identity::DefaultAzureCredential;
/// let creds = DefaultAzureCredential::default();
/// let client = KeyvaultClient::new(&"https://test-key-vault.vault.azure.net", std::sync::Arc::new(creds)).unwrap();
/// ```
#[derive(Clone)]
pub struct KeyvaultClient {
    pub(crate) vault_url: Url,
    pub(crate) pipeline: Pipeline,
}

impl std::fmt::Debug for KeyvaultClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyvaultClient")
            .field("vault_url", &self.vault_url)
            .finish_non_exhaustive()
    }
}

impl KeyvaultClient {
    /// Creates a new `KeyClient`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use std::sync::Arc;
    /// let creds = Arc::new(DefaultAzureCredential::default());
    /// let client = KeyvaultClient::new("test-key-vault.vault.azure.net", creds).unwrap();
    /// ```
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let vault_url = Url::parse(vault_url)?;
        let endpoint = extract_endpoint(&vault_url)?;
        let pipeline = new_pipeline_from_options(token_credential.clone(), endpoint);
        let client = Self {
            vault_url,
            pipeline,
        };
        Ok(client)
    }

    pub(crate) fn finalize_request(
        &self,
        mut url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        let dt = OffsetDateTime::now_utc();
        let time = date::to_rfc1123(&dt);

        url.set_query(Some(API_VERSION_PARAM));

        let mut request = Request::new(url, method);
        for (k, v) in headers {
            request.insert_header(k, v);
        }

        request.insert_header(MS_DATE, time);

        if let Some(request_body) = request_body {
            if request.headers().get_optional_str(&CONTENT_TYPE).is_none() {
                request.insert_headers(&ContentType::APPLICATION_JSON);
            }
            request.insert_header(CONTENT_LENGTH, request_body.len().to_string());
            request.set_body(request_body);
        } else {
            request.insert_header(CONTENT_LENGTH, "0");
            request.set_body(azure_core::EMPTY_BODY);
        };

        Ok(request)
    }

    pub(crate) async fn send(
        &self,
        context: &Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.pipeline.send(context, request).await
    }

    pub fn secret_client(&self) -> SecretClient {
        SecretClient::new_with_client(self.clone())
    }

    pub fn certificate_client(&self) -> CertificateClient {
        CertificateClient::new_with_client(self.clone())
    }

    pub fn key_client(&self) -> KeyClient {
        KeyClient::new_with_client(self.clone())
    }
}

/// Helper to get vault endpoint with a scheme and a trailing slash
/// ex. `https://vault.azure.net/` where the full client url is `https://myvault.vault.azure.net`
fn extract_endpoint(url: &Url) -> azure_core::Result<String> {
    let endpoint = url
        .host_str()
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("failed to parse host from url. url: {url}")
            })
        })?
        .splitn(2, '.') // FIXME: replace with split_once() when it is in stable
        .last()
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, || {
                format!("failed to extract endpoint from url. url: {url}")
            })
        })?;
    Ok(format!("{}://{}", url.scheme(), endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_endpoint() {
        let suffix =
            extract_endpoint(&Url::parse("https://myvault.vault.azure.net").unwrap()).unwrap();
        assert_eq!(suffix, "https://vault.azure.net");

        let suffix =
            extract_endpoint(&Url::parse("https://myvault.mycustom.vault.server.net").unwrap())
                .unwrap();
        assert_eq!(suffix, "https://mycustom.vault.server.net");

        let suffix = extract_endpoint(&Url::parse("https://myvault.internal").unwrap()).unwrap();
        assert_eq!(suffix, "https://internal");

        let suffix =
            extract_endpoint(&Url::parse("some-scheme://myvault.vault.azure.net").unwrap())
                .unwrap();
        assert_eq!(suffix, "some-scheme://vault.azure.net");
    }
}
