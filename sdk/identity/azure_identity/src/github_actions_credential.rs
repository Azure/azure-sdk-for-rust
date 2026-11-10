// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    env::Env, get_authority_host, validate_not_empty, validate_tenant_id, ClientAssertion,
    ClientAssertionCredential, ClientAssertionCredentialOptions,
};
use azure_core::{
    credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::{
        headers::AUTHORIZATION, ClientMethodOptions, Method, Pipeline, PipelineSendOptions,
        Request, StatusCode, Url,
    },
    Error,
};
use serde::Deserialize;
use std::{any::type_name, fmt, sync::Arc};

// cspell:ignore chinacloudapi microsoftonline sovcloud
const OIDC_REQUEST_URL: &str = "ACTIONS_ID_TOKEN_REQUEST_URL";
const OIDC_REQUEST_TOKEN: &str = "ACTIONS_ID_TOKEN_REQUEST_TOKEN";

/// Authenticates a service principal using GitHub Actions workload identity federation.
///
/// Configure a [federated identity credential] on the service principal and grant the workflow
/// `permissions: id-token: write`. GitHub Actions supplies the `ACTIONS_ID_TOKEN_REQUEST_URL` and
/// `ACTIONS_ID_TOKEN_REQUEST_TOKEN` environment variables used by this credential.
///
/// [federated identity credential]: https://learn.microsoft.com/entra/workload-id/workload-identity-federation-create-trust-github
pub struct GitHubActionsCredential(ClientAssertionCredential<Client>);

impl fmt::Debug for GitHubActionsCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(type_name::<Self>()).finish_non_exhaustive()
    }
}

/// Options for constructing a new [`GitHubActionsCredential`].
#[derive(Default)]
pub struct GitHubActionsCredentialOptions {
    /// Options for the [`ClientAssertionCredential`] used by the [`GitHubActionsCredential`].
    ///
    /// The configured cloud determines the audience requested from GitHub Actions.
    /// When no cloud is configured, `AZURE_AUTHORITY_HOST` is used, defaulting to Azure Public.
    pub credential_options: ClientAssertionCredentialOptions,

    #[cfg(test)]
    pub(crate) env: Env,
}

impl fmt::Debug for GitHubActionsCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

impl GitHubActionsCredential {
    /// Creates a new [`GitHubActionsCredential`].
    ///
    /// # Arguments
    /// - `tenant_id`: The tenant (directory) ID of the service principal.
    /// - `client_id`: The client (application) ID of the service principal.
    /// - `options`: Options for configuring the credential. If `None`, the credential uses its default options.
    ///
    /// # Errors
    ///
    /// Returns an error if an ID is invalid, a required GitHub Actions environment variable is
    /// missing or empty, the OIDC endpoint isn't an HTTPS URL without user information or a
    /// fragment, or the authority host isn't supported.
    pub fn new(
        tenant_id: String,
        client_id: String,
        options: Option<GitHubActionsCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        validate_tenant_id(&tenant_id)?;
        validate_not_empty(&client_id, "no client ID specified")?;
        let options = options.unwrap_or_default();
        #[cfg(test)]
        let env = options.env;
        #[cfg(not(test))]
        let env = Env::default();

        let read_variable = |name| -> azure_core::Result<String> {
            let message = format!(
                "{name} must be set by GitHub Actions. Run in a workflow with permissions: id-token: write"
            );
            let value = env
                .var(name)
                .with_context(ErrorKind::Credential, message.clone())?;
            validate_not_empty(&value, message)?;
            Ok(value)
        };
        let endpoint = read_variable(OIDC_REQUEST_URL)?;
        let request_token = Secret::new(read_variable(OIDC_REQUEST_TOKEN)?);
        let mut endpoint = Url::parse(&endpoint).with_context(
            ErrorKind::Credential,
            "invalid URL for ACTIONS_ID_TOKEN_REQUEST_URL",
        )?;
        if endpoint.scheme() != "https"
            || endpoint.host_str().is_none()
            || !endpoint.username().is_empty()
            || endpoint.password().is_some()
            || endpoint.fragment().is_some()
        {
            return Err(Error::with_message(
                ErrorKind::Credential,
                "ACTIONS_ID_TOKEN_REQUEST_URL must be an HTTPS URL without user information or a fragment",
            ));
        }

        let authority = get_authority_host(
            Some(env),
            options.credential_options.client_options.cloud.as_deref(),
        )?;
        let audience = match authority.host_str() {
            Some("login.microsoftonline.com") => "api://AzureADTokenExchange",
            Some("login.chinacloudapi.cn") => "api://AzureADTokenExchangeChina",
            Some("login.microsoftonline.us") => "api://AzureADTokenExchangeUSGov",
            Some("login.sovcloud-identity.fr") => "api://AzureADTokenExchangeFrance",
            Some("login.sovcloud-identity.de") => "api://AzureADTokenExchangeGermany",
            Some("login.sovcloud-identity.sg") => "api://AzureADTokenExchangeGovSG",
            _ => {
                return Err(Error::with_message(
                    ErrorKind::Credential,
                    "the authority host isn't supported by GitHubActionsCredential",
                ))
            }
        };
        let query: Vec<(String, String)> = endpoint
            .query_pairs()
            .filter(|(key, _)| key != "audience")
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();
        endpoint
            .query_pairs_mut()
            .clear()
            .extend_pairs(query)
            .append_pair("audience", audience);

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.credential_options.client_options.clone(),
            Vec::default(),
            Vec::default(),
            None,
        );
        let credential = ClientAssertionCredential::new_exclusive(
            tenant_id,
            client_id,
            Client {
                endpoint,
                pipeline,
                request_token,
            },
            stringify!(GitHubActionsCredential),
            Some(options.credential_options),
        )?;
        Ok(Arc::new(Self(credential)))
    }
}

#[async_trait::async_trait]
impl TokenCredential for GitHubActionsCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.0.get_token(scopes, options).await
    }
}

struct Client {
    endpoint: Url,
    pipeline: Pipeline,
    request_token: Secret,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl ClientAssertion for Client {
    async fn secret(&self, options: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
        let mut request = Request::new(self.endpoint.clone(), Method::Get);
        request.insert_header(
            AUTHORIZATION,
            String::from("Bearer ") + self.request_token.secret(),
        );
        let options = options.unwrap_or_default();
        let response = self
            .pipeline
            .send(
                &options.context,
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await?;
        let status = response.status();
        if status != StatusCode::Ok {
            return Err(Error::with_message(
                ErrorKind::HttpResponse {
                    status,
                    error_code: Some(status.canonical_reason().to_string()),
                    raw_response: Some(Box::new(response)),
                },
                format!("{status} response from the GitHub Actions OIDC endpoint"),
            ));
        }
        let assertion: Assertion = response.into_body().json().map_err(|_| {
            Error::with_message(
                ErrorKind::DataConversion,
                "the GitHub Actions OIDC endpoint returned an invalid token response",
            )
        })?;
        validate_not_empty(
            assertion.value.secret(),
            "the GitHub Actions OIDC endpoint returned an empty token",
        )?;
        Ok(assertion.value.secret().to_string())
    }
}

#[derive(Deserialize)]
struct Assertion {
    value: Secret,
}

#[cfg(test)]
mod tests {
    use super::{
        ClientAssertionCredentialOptions, Env, ErrorKind, GitHubActionsCredential,
        GitHubActionsCredentialOptions, Method, StatusCode, TokenCredential, AUTHORIZATION,
        OIDC_REQUEST_TOKEN, OIDC_REQUEST_URL,
    };
    use crate::{
        client_assertion_credential::tests::is_valid_request,
        tests::{
            cloud_configuration_cases, token_response, MockSts, FAKE_CLIENT_ID, FAKE_TENANT_ID,
            FAKE_TOKEN, LIVE_TEST_SCOPES,
        },
    };
    use azure_core::{
        cloud::{CloudConfiguration, CustomConfiguration},
        http::{headers::Headers, AsyncRawResponse, ClientOptions, Transport},
        Bytes,
    };
    use std::sync::Arc;

    const ENDPOINT: &str = "https://oidc.contoso.com/token";

    fn default_options() -> GitHubActionsCredentialOptions {
        GitHubActionsCredentialOptions {
            env: Env::from(
                &[
                    (OIDC_REQUEST_URL, ENDPOINT),
                    (OIDC_REQUEST_TOKEN, "request-token"),
                ][..],
            ),
            credential_options: ClientAssertionCredentialOptions {
                client_options: ClientOptions {
                    cloud: Some(Arc::new(CloudConfiguration::AzurePublic)),
                    ..Default::default()
                },
            },
        }
    }

    #[test]
    fn invalid_configuration() {
        for (tenant, client) in [("", "client"), ("invalid_tenant", "client"), ("tenant", "")] {
            assert!(GitHubActionsCredential::new(
                tenant.into(),
                client.into(),
                Some(default_options()),
            )
            .is_err());
        }
        for variables in [
            vec![],
            vec![(OIDC_REQUEST_URL, ENDPOINT)],
            vec![(OIDC_REQUEST_TOKEN, "request-token")],
            vec![
                (OIDC_REQUEST_URL, ""),
                (OIDC_REQUEST_TOKEN, "request-token"),
            ],
            vec![(OIDC_REQUEST_URL, ENDPOINT), (OIDC_REQUEST_TOKEN, "")],
        ] {
            let mut options = default_options();
            options.env = Env::from(variables.as_slice());
            let err = GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options))
                .unwrap_err();
            assert!(matches!(err.kind(), ErrorKind::Credential));
            assert!(err.to_string().contains("permissions: id-token: write"));
            assert!(!format!("{err:?}").contains("request-token"));
        }
        for url in [
            "",
            "not a URL",
            "http://oidc.contoso.com/token",
            concat!(
                "https://",
                "user",
                ":",
                "password",
                "@oidc.contoso.com/token"
            ),
            "https://oidc.contoso.com/token#fragment",
        ] {
            let mut options = default_options();
            options.env = Env::from(
                &[
                    (OIDC_REQUEST_URL, url),
                    (OIDC_REQUEST_TOKEN, "request-token"),
                ][..],
            );
            let err = GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options))
                .unwrap_err();
            assert!(matches!(err.kind(), ErrorKind::Credential));
            assert!(!format!("{err:?}").contains("password"));
        }
    }

    #[test]
    fn unsupported_authority() {
        let mut options = default_options();
        let mut cloud = CustomConfiguration::default();
        cloud.authority_host = "https://login.contoso.com".into();
        options.credential_options.client_options.cloud =
            Some(Arc::new(CloudConfiguration::Custom(cloud)));
        let err = GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options))
            .unwrap_err();
        assert!(err.to_string().contains("authority host isn't supported"));
    }

    #[test]
    fn authority_environment() {
        let mut options = default_options();
        options.env = Env::from(
            &[
                (OIDC_REQUEST_URL, ENDPOINT),
                (OIDC_REQUEST_TOKEN, "request-token"),
                (
                    crate::AZURE_AUTHORITY_HOST_ENV_KEY,
                    "https://login.contoso.com",
                ),
            ][..],
        );
        assert!(
            GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options),).is_ok()
        );

        let mut options = default_options();
        options.credential_options.client_options.cloud = None;
        assert!(
            GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options),).is_ok()
        );

        let mut options = default_options();
        options.credential_options.client_options.cloud = None;
        options.env = Env::from(
            &[
                (OIDC_REQUEST_URL, ENDPOINT),
                (OIDC_REQUEST_TOKEN, "request-token"),
                (
                    crate::AZURE_AUTHORITY_HOST_ENV_KEY,
                    "https://login.contoso.com",
                ),
            ][..],
        );
        let err = GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options))
            .unwrap_err();
        assert!(err.to_string().contains("authority host isn't supported"));
    }

    #[tokio::test]
    async fn token_exchange_and_cache() {
        let mut cases: Vec<_> = cloud_configuration_cases()
            .into_iter()
            .filter_map(|(cloud, authority)| {
                let audience = match cloud {
                    CloudConfiguration::AzurePublic => "api://AzureADTokenExchange",
                    CloudConfiguration::AzureGovernment => "api://AzureADTokenExchangeUSGov",
                    CloudConfiguration::AzureChina => "api://AzureADTokenExchangeChina",
                    _ => return None,
                };
                Some((cloud, authority, audience))
            })
            .collect();
        for (host, audience) in [
            (
                "login.sovcloud-identity.fr",
                "api://AzureADTokenExchangeFrance",
            ),
            (
                "login.sovcloud-identity.de",
                "api://AzureADTokenExchangeGermany",
            ),
            (
                "login.sovcloud-identity.sg",
                "api://AzureADTokenExchangeGovSG",
            ),
        ] {
            let mut cloud = CustomConfiguration::default();
            cloud.authority_host = format!("https://{host}/");
            cases.push((
                CloudConfiguration::Custom(cloud),
                format!("https://{host}/{FAKE_TENANT_ID}"),
                audience,
            ));
        }
        for (cloud, authority, audience) in cases {
            for query in [
                "",
                "?existing=a%2Bb",
                "?existing=a%2Bb&audience=old&audience=other",
            ] {
                let has_existing_query = !query.is_empty();
                let validate_exchange =
                    is_valid_request(authority.clone(), Some("assertion-token".into()));
                let mock = MockSts::new(
                    vec![
                        AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            Bytes::from_static(br#"{"value":"assertion-token"}"#),
                        ),
                        token_response(),
                    ],
                    Some(Arc::new(move |request| {
                        if request.url().host_str() == Some("oidc.contoso.com") {
                            assert_eq!(request.method(), Method::Get);
                            assert_eq!(
                                request.headers().get_str(&AUTHORIZATION)?,
                                format!("{} {}", "Bearer", "request-token")
                            );
                            let pairs: Vec<_> = request.url().query_pairs().collect();
                            assert_eq!(
                                pairs.iter().filter(|(key, _)| key == "audience").count(),
                                1
                            );
                            assert!(pairs
                                .iter()
                                .any(|(key, value)| key == "audience" && value == audience));
                            assert_eq!(pairs.len(), if has_existing_query { 2 } else { 1 });
                            if has_existing_query {
                                assert!(pairs
                                    .iter()
                                    .any(|(key, value)| key == "existing" && value == "a+b"));
                            }
                            Ok(())
                        } else {
                            validate_exchange(request)
                        }
                    })),
                );
                let mut options = default_options();
                let endpoint = format!("{ENDPOINT}{query}");
                options.env = Env::from(
                    &[
                        (OIDC_REQUEST_URL, endpoint.as_str()),
                        (OIDC_REQUEST_TOKEN, "request-token"),
                    ][..],
                );
                options.credential_options.client_options.cloud = Some(Arc::new(cloud.clone()));
                options.credential_options.client_options.transport =
                    Some(Transport::new(Arc::new(mock)));
                let credential = GitHubActionsCredential::new(
                    FAKE_TENANT_ID.into(),
                    FAKE_CLIENT_ID.into(),
                    Some(options),
                )
                .unwrap();
                for _ in 0..2 {
                    let token = credential.get_token(LIVE_TEST_SCOPES, None).await.unwrap();
                    assert_eq!(token.token.secret(), FAKE_TOKEN);
                }
                assert!(!format!("{credential:?}").contains("request-token"));
            }
        }
    }

    #[tokio::test]
    async fn invalid_response() {
        for body in [
            "",
            "not JSON",
            "{}",
            r#"{"value":""}"#,
            r#"{"value":null}"#,
            r#"{"value":42}"#,
            r#"{"value":["sensitive-token"]}"#,
            r#"{"value":"sensitive-token"} trailing"#,
        ] {
            let mut options = default_options();
            options.credential_options.client_options.transport =
                Some(Transport::new(Arc::new(MockSts::new(
                    vec![AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        Bytes::from(body.to_string()),
                    )],
                    None,
                ))));
            let credential =
                GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options))
                    .unwrap();
            let err = credential
                .get_token(LIVE_TEST_SCOPES, None)
                .await
                .unwrap_err();
            assert!(matches!(err.kind(), ErrorKind::Credential));
            assert!(err
                .to_string()
                .starts_with("GitHubActionsCredential authentication failed."));
            assert!(!format!("{err:?}").contains("sensitive-token"));
        }
    }

    #[tokio::test]
    async fn error_response() {
        let mut options = default_options();
        options.credential_options.client_options.transport =
            Some(Transport::new(Arc::new(MockSts::new(
                vec![AsyncRawResponse::from_bytes(
                    StatusCode::Forbidden,
                    Headers::new(),
                    Bytes::from_static(b"sensitive-token"),
                )],
                None,
            ))));
        let credential =
            GitHubActionsCredential::new("tenant".into(), "client".into(), Some(options)).unwrap();
        let err = credential
            .get_token(LIVE_TEST_SCOPES, None)
            .await
            .unwrap_err();
        assert!(matches!(err.kind(), ErrorKind::Credential));
        assert_eq!(
            err.to_string(),
            "GitHubActionsCredential authentication failed. 403 response from the GitHub Actions OIDC endpoint"
        );
        assert!(!format!("{err:?}").contains("sensitive-token"));
        let inner = err.downcast_ref::<azure_core::Error>().unwrap();
        assert!(matches!(
            inner.kind(),
            ErrorKind::HttpResponse {
                status: StatusCode::Forbidden,
                raw_response: Some(_),
                ..
            }
        ));
    }
}
