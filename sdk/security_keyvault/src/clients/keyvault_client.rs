use crate::{clients::pipeline::new_pipeline_from_options, prelude::*};
use azure_core::{
    auth::TokenCredential,
    date,
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    Body, Context, Method, Pipeline, Request, Response, Url,
};
use std::sync::Arc;
use time::OffsetDateTime;

pub const API_VERSION: &str = "7.0";
const API_VERSION_PARAM: &str = "api-version";

/// Client for Key Vault operations - getting a secret, listing secrets, etc.
///
/// # Example
///
/// ```no_run
/// use azure_security_keyvault::KeyvaultClient;
/// let credential = azure_identity::create_credential().unwrap();
/// let client = KeyvaultClient::new(&"https://test-key-vault.vault.azure.net", credential).unwrap();
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
    /// let credential = azure_identity::create_credential().unwrap();
    /// let client = KeyvaultClient::new("test-key-vault.vault.azure.net", credential).unwrap();
    /// ```
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let vault_url = Url::parse(vault_url)?;
        let scope = build_scope(&vault_url)?;
        let pipeline = new_pipeline_from_options(token_credential.clone(), scope);
        let client = Self {
            vault_url,
            pipeline,
        };
        Ok(client)
    }

    pub(crate) fn finalize_request(
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> Request {
        let dt = OffsetDateTime::now_utc();
        let time = date::to_rfc1123(&dt);

        // per discussion in #1301, we _always_ override the api-version with
        // the client's version
        let query = url
            .query_pairs()
            .filter(|(name, _)| name != API_VERSION_PARAM);
        let mut url = url.clone();
        url.query_pairs_mut()
            .clear()
            .extend_pairs(query)
            .append_pair(API_VERSION_PARAM, API_VERSION);

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

        request
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
fn build_scope(url: &Url) -> azure_core::Result<String> {
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
    Ok(format!("{}://{}/.default", url.scheme(), endpoint))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_endpoint() {
        let suffix = build_scope(&Url::parse("https://myvault.vault.azure.net").unwrap()).unwrap();
        assert_eq!(suffix, "https://vault.azure.net/.default");

        let suffix =
            build_scope(&Url::parse("https://myvault.mycustom.vault.server.net").unwrap()).unwrap();
        assert_eq!(suffix, "https://mycustom.vault.server.net/.default");

        let suffix = build_scope(&Url::parse("https://myvault.internal").unwrap()).unwrap();
        assert_eq!(suffix, "https://internal/.default");

        let suffix =
            build_scope(&Url::parse("some-scheme://myvault.vault.azure.net").unwrap()).unwrap();
        assert_eq!(suffix, "some-scheme://vault.azure.net/.default");
    }
}
