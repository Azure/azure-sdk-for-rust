// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    ClientAssertion, ClientAssertionCredential, ClientAssertionCredentialOptions,
    TokenCredentialOptions,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::ErrorKind,
    http::{
        headers::{FromHeaders, HeaderName, Headers, AUTHORIZATION, CONTENT_LENGTH},
        request::Request,
        HttpClient, Method, StatusCode, Url,
    },
};
use serde::Deserialize;
use std::{convert::Infallible, fmt, sync::Arc};

// cspell:ignore fedauthredirect msedge oidcrequesturi
const OIDC_VARIABLE_NAME: &str = "SYSTEM_OIDCREQUESTURI";
const OIDC_VERSION: &str = "7.1";
const TFS_FEDAUTHREDIRECT_HEADER: HeaderName = HeaderName::from_static("x-tfs-fedauthredirect");

// TODO: https://github.com/Azure/azure-sdk-for-rust/issues/682
const ALLOWED_HEADERS: &[&str] = &["x-msedge-ref", "x-vss-e2eid"];

#[derive(Debug)]
pub struct AzurePipelinesCredential(ClientAssertionCredential<Client>);

/// Options for constructing a new [`AzurePipelinesCredential`].
#[derive(Debug, Default)]
pub struct AzurePipelinesCredentialOptions {
    /// Options for the [`ClientAssertionCredential`] used by the [`AzurePipelinesCredential`].
    pub credential_options: ClientAssertionCredentialOptions,
}

// TODO: Should probably remove this once we consolidate and unify credentials.
impl From<TokenCredentialOptions> for AzurePipelinesCredentialOptions {
    fn from(value: TokenCredentialOptions) -> Self {
        Self {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: value,
                ..Default::default()
            },
        }
    }
}

impl AzurePipelinesCredential {
    /// Creates a new [`AzurePipelinesCredential`] for connecting to resources from Azure Pipelines.
    pub fn new<T>(
        tenant_id: String,
        client_id: String,
        service_connection_id: &str,
        system_access_token: T,
        options: Option<AzurePipelinesCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>>
    where
        T: Into<Secret>,
    {
        let system_access_token = system_access_token.into();

        crate::validate_tenant_id(&tenant_id)?;
        crate::validate_not_empty(&client_id, "no client ID specified")?;
        crate::validate_not_empty(service_connection_id, "no service connection ID specified")?;
        crate::validate_not_empty(
            system_access_token.secret(),
            "no system access token specified",
        )?;

        let options = options.unwrap_or_default();
        let env = options.credential_options.credential_options.env();
        let endpoint = env
            .var(OIDC_VARIABLE_NAME)
            .map_err(|err| azure_core::Error::full(
                ErrorKind::Credential,
                err,
                format!("no value for environment variable {OIDC_VARIABLE_NAME}. This should be set by Azure Pipelines"),
            ))?;
        let mut endpoint: Url = endpoint.parse().map_err(|err| {
            azure_core::Error::full(
                ErrorKind::Credential,
                err,
                format!("invalid URL for environment variable {OIDC_VARIABLE_NAME}"),
            )
        })?;
        endpoint
            .query_pairs_mut()
            .append_pair("api-version", OIDC_VERSION)
            .append_pair("serviceConnectionId", service_connection_id);
        let client = Client {
            endpoint,
            http_client: options.credential_options.credential_options.http_client(),
            system_access_token,
        };
        let credential = ClientAssertionCredential::new_exclusive(
            tenant_id,
            client_id,
            client,
            Some(options.credential_options),
        )?;

        Ok(Arc::new(Self(credential)))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for AzurePipelinesCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        self.0.get_token(scopes, options).await
    }
}

#[derive(Debug)]
struct Client {
    endpoint: Url,
    http_client: Arc<dyn HttpClient>,
    system_access_token: Secret,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl ClientAssertion for Client {
    async fn secret(&self) -> azure_core::Result<String> {
        let mut req = Request::new(self.endpoint.clone(), Method::Post);
        req.insert_header(
            AUTHORIZATION,
            String::from("Bearer ") + self.system_access_token.secret(),
        );
        req.insert_header(TFS_FEDAUTHREDIRECT_HEADER, "Suppress");
        req.insert_header(CONTENT_LENGTH, "0");

        // TODO: Consider defining and using azure_identity-specific pipeline, or even from azure_core.
        let resp = self.http_client.execute_request(&req).await?;
        if resp.status() != StatusCode::Ok {
            let status_code = resp.status();
            let err_headers: ErrorHeaders = resp.headers().get()?;

            return Err(
                azure_core::Error::message(
                    ErrorKind::http_response(status_code, Some(status_code.canonical_reason().to_string())),
                     format!("{status_code} response from the OIDC endpoint. Check service connection ID and pipeline configuration. {err_headers}"),
                )
            );
        }

        let assertion: Assertion = resp.into_body().json().await?;
        Ok(assertion.oidc_token.secret().to_string())
    }
}

#[derive(Debug, Deserialize)]
struct Assertion {
    #[serde(rename = "oidcToken")]
    oidc_token: Secret,
}

#[derive(Debug)]
struct ErrorHeaders {
    msedge_ref: Option<String>,
    vss_e2eid: Option<String>,
}

const MSEDGE_REF: HeaderName = HeaderName::from_static("x-msedge-ref");
const VSS_E2EID: HeaderName = HeaderName::from_static("x-vss-e2eid");

impl FromHeaders for ErrorHeaders {
    type Error = Infallible;

    fn header_names() -> &'static [&'static str] {
        ALLOWED_HEADERS
    }

    fn from_headers(headers: &Headers) -> Result<Option<Self>, Self::Error> {
        Ok(Some(Self {
            msedge_ref: headers.get_optional_string(&MSEDGE_REF),
            vss_e2eid: headers.get_optional_string(&VSS_E2EID),
        }))
    }
}

impl fmt::Display for ErrorHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = f.debug_struct("Headers");
        if let Some(ref msedge_ref) = self.msedge_ref {
            v.field(MSEDGE_REF.as_str(), msedge_ref);
        }
        if let Some(ref vss_e2eid) = self.vss_e2eid {
            v.field(VSS_E2EID.as_str(), vss_e2eid);
        }
        v.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::Env;
    use azure_core::{http::BufResponse, Bytes};
    use azure_core_test::http::MockHttpClient;
    use futures::FutureExt as _;

    #[test]
    fn param_errors() {
        assert!(AzurePipelinesCredential::new("".into(), "".into(), "", "", None).is_err());
        assert!(AzurePipelinesCredential::new("_".into(), "".into(), "", "", None).is_err());
        assert!(AzurePipelinesCredential::new("a".into(), "".into(), "", "", None).is_err());
        assert!(AzurePipelinesCredential::new("a".into(), "b".into(), "", "", None).is_err());
        assert!(AzurePipelinesCredential::new("a".into(), "b".into(), "c", "", None).is_err());

        let options = AzurePipelinesCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: TokenCredentialOptions {
                    env: Env::from(&[(OIDC_VARIABLE_NAME, "http://localhost/get_token")][..]),
                    ..Default::default()
                },
                ..Default::default()
            },
        };
        assert!(
            AzurePipelinesCredential::new("a".into(), "b".into(), "c", "d", Some(options)).is_ok()
        );
    }

    #[tokio::test]
    async fn error_headers() {
        let mock_client = MockHttpClient::new(|req| {
            assert_eq!(
                req.url().as_str(),
                "http://localhost/get_token?api-version=7.1&serviceConnectionId=c"
            );
            let mut headers = Headers::new();
            headers.insert(MSEDGE_REF, "foo");
            headers.insert(VSS_E2EID, "bar");

            async move {
                Ok(BufResponse::from_bytes(
                    StatusCode::Forbidden,
                    headers,
                    Vec::new(),
                ))
            }
            .boxed()
        });
        let options = AzurePipelinesCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: TokenCredentialOptions {
                    env: Env::from(&[(OIDC_VARIABLE_NAME, "http://localhost/get_token")][..]),
                    http_client: Arc::new(mock_client),
                    ..Default::default()
                },
                ..Default::default()
            },
        };
        let credential =
            AzurePipelinesCredential::new("a".into(), "b".into(), "c", "d", Some(options))
                .expect("valid AzurePipelinesCredential");
        assert!(matches!(
            credential.get_token(&["default"], None).await,
            Err(err) if matches!(
                err.kind(),
                ErrorKind::HttpResponse { status, .. }
                    if *status == StatusCode::Forbidden &&
                        err.to_string().contains("foo") &&
                        err.to_string().contains("bar"),
            )
        ));
    }

    #[tokio::test]
    async fn mock_request() {
        let mock_client = MockHttpClient::new(|req| {
            async move {
                if req.url().as_str()
                    == "http://localhost/get_token?api-version=7.1&serviceConnectionId=c"
                {
                    assert!(matches!(
                        req.headers().get_str(&AUTHORIZATION),
                        Ok(value) if value == "Bearer d",
                    ));
                    assert!(matches!(
                        req.headers().get_str(&TFS_FEDAUTHREDIRECT_HEADER),
                        Ok(value) if value == "Suppress",
                    ));

                    let mut headers = Headers::new();
                    headers.insert(MSEDGE_REF, "foo");
                    headers.insert(VSS_E2EID, "bar");

                    return Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        headers,
                        Bytes::from_static(br#"{"oidcToken":"baz"}"#),
                    ));
                }

                if req.url().as_str() == "https://login.microsoftonline.com/a/oauth2/v2.0/token" {
                    return Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        Bytes::from_static(
                            br#"{"token_type":"test","expires_in":0,"ext_expires_in":0,"access_token":"qux"}"#,
                        ),
                    ));
                }

                panic!("not supported")
            }.boxed()
        });
        let options = AzurePipelinesCredentialOptions {
            credential_options: ClientAssertionCredentialOptions {
                credential_options: TokenCredentialOptions {
                    env: Env::from(&[(OIDC_VARIABLE_NAME, "http://localhost/get_token")][..]),
                    http_client: Arc::new(mock_client),
                    ..Default::default()
                },
                ..Default::default()
            },
        };
        let credential =
            AzurePipelinesCredential::new("a".into(), "b".into(), "c", "d", Some(options))
                .expect("valid AzurePipelinesCredential");
        let secret = credential
            .get_token(&["default"], None)
            .await
            .expect("valid response");
        assert_eq!(secret.token.secret(), "qux");
    }
}
