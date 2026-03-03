// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with the service.
pub use crate::generated::clients::*;
use crate::{
    authorizer::KeyVaultAuthorizer,
    models::{
        CertificateClientCreateCertificateOptions, CertificateOperation,
        CreateCertificateParameters,
    },
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    fmt::SafeDebug,
    http::{
        headers::{RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS},
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        poller::{
            get_retry_after, Poller, PollerContinuation, PollerResult, PollerState, PollerStatus,
            StatusMonitor as _,
        },
        Body, ClientOptions, Method, Pipeline, RawResponse, Request, RequestContent, Url,
    },
    json, tracing, Result,
};
use std::sync::Arc;

/// Options used when creating a [`CertificateClient`]
#[derive(Clone, SafeDebug)]
pub struct CertificateClientOptions {
    /// The API version to use for this operation.
    pub api_version: String,
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Controls whether the client requires the resource specified in authentication
    /// challenges to match the Key Vault or Managed HSM domain. True by default.
    pub verify_challenge_resource: Option<bool>,
}

impl Default for CertificateClientOptions {
    fn default() -> Self {
        Self {
            api_version: String::from("2025-07-01"),
            client_options: ClientOptions::default(),
            verify_challenge_resource: Some(true),
        }
    }
}

impl CertificateClient {
    /// Creates a new CertificateClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Service host
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an
    ///   Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("KeyVault")]
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<CertificateClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{endpoint} must use http(s)"),
            ));
        }
        let authorizer = KeyVaultAuthorizer::new(options.verify_challenge_resource.unwrap_or(true));
        let auth_policy: Arc<dyn Policy> = Arc::new(
            BearerTokenAuthorizationPolicy::new(credential, Vec::<String>::new())
                .with_on_request(authorizer.clone())
                .with_on_challenge(authorizer),
        );
        Ok(Self {
            endpoint,
            api_version: options.api_version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::new(),
                vec![auth_policy],
                None,
            ),
        })
    }

    /// Creates a new certificate and returns a [`Poller<CertificateOperation>`] to monitor the status.
    ///
    /// If this is the first version, the certificate resource is created. This operation requires the certificates/create permission.
    ///
    /// # Arguments
    ///
    /// * `certificate_name` - The name of the certificate. The value you provide may be copied globally for the purpose of running
    ///   the service. The value provided should not include personally identifiable or sensitive information.
    /// * `parameters` - The parameters to create a certificate.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_security_keyvault_certificates::{
    ///     CertificateClient,
    ///     models::{CreateCertificateParameters, CertificatePolicy, X509CertificateProperties, IssuerParameters},
    /// };
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential = DeveloperToolsCredential::new(None)?;
    /// let client = CertificateClient::new(
    ///     "https://your-key-vault-name.vault.azure.net/",
    ///     credential.clone(),
    ///     None,
    /// )?;
    ///
    /// // Create a self-signed certificate.
    /// let policy = CertificatePolicy {
    ///     x509_certificate_properties: Some(X509CertificateProperties {
    ///         subject: Some("CN=DefaultPolicy".into()),
    ///         ..Default::default()
    ///     }),
    ///     issuer_parameters: Some(IssuerParameters {
    ///         name: Some("Self".into()),
    ///         ..Default::default()
    ///     }),
    ///     ..Default::default()
    /// };
    /// let body = CreateCertificateParameters {
    ///     certificate_policy: Some(policy),
    ///     ..Default::default()
    /// };
    ///
    /// // Wait for the certificate operation to complete and get the certificate.
    /// let certificate = client
    ///     .create_certificate("certificate-name", body.try_into()?, None)?
    ///     .await?
    ///     .into_model()?;
    ///
    /// # Ok(()) }
    /// ```
    #[tracing::function("KeyVault.createCertificate")]
    pub fn create_certificate(
        &self,
        certificate_name: &str,
        parameters: RequestContent<CreateCertificateParameters>,
        options: Option<CertificateClientCreateCertificateOptions<'_>>,
    ) -> Result<Poller<CertificateOperation>> {
        let options = options.unwrap_or_default().into_owned();
        let pipeline = self.pipeline.clone();

        let mut url = self.endpoint.clone();
        let mut path = String::from("certificates/{certificate-name}/create");
        path = path.replace("{certificate-name}", certificate_name);
        url = url.join(&path)?;
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let api_version = self.api_version.clone();
        let certificate_name = certificate_name.to_owned();

        let parameters: Body = parameters.into();

        Ok(Poller::new(
            move |poller_state: PollerState, poller_options| {
                let (mut request, next_link) = match poller_state {
                    PollerState::More(continuation) => {
                        let next_link = match continuation {
                            PollerContinuation::Links { next_link, .. } => next_link,
                            _ => unreachable!(),
                        };
                        // Make sure the `api-version` is set appropriately.
                        let qp = next_link
                            .query_pairs()
                            .filter(|(name, _)| name.ne("api-version"));
                        let mut next_link = next_link.clone();
                        next_link
                            .query_pairs_mut()
                            .clear()
                            .extend_pairs(qp)
                            .append_pair("api-version", &api_version);

                        let mut request = Request::new(next_link.clone(), Method::Get);
                        request.insert_header("accept", "application/json");

                        (request, next_link)
                    }
                    PollerState::Initial => {
                        let mut request = Request::new(url.clone(), Method::Post);
                        request.insert_header("accept", "application/json");
                        request.insert_header("content-type", "application/json");
                        request.set_body(&parameters);

                        let mut url = url.clone();
                        let mut path = String::from("certificates/{certificate-name}/pending");
                        path = path.replace("{certificate-name}", &certificate_name);
                        url.set_path(&path);

                        (request, url)
                    }
                };

                let pipeline = pipeline.clone();
                let api_version = api_version.clone();
                let ctx = poller_options.context.clone();
                Box::pin(async move {
                    let rsp = pipeline.send(&ctx, &mut request, None).await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let retry_after = get_retry_after(
                        &headers,
                        &[RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS, RETRY_AFTER],
                        &poller_options,
                    );
                    let res: CertificateOperation = json::from_json(&body)?;
                    let rsp = RawResponse::from_bytes(status, headers, body).into();

                    Ok(match res.status() {
                        PollerStatus::InProgress => PollerResult::InProgress {
                            response: rsp,
                            retry_after,
                            continuation: PollerContinuation::Links {
                                next_link,
                                final_link: None,
                            },
                        },
                        PollerStatus::Succeeded => {
                            PollerResult::Succeeded {
                                response: rsp,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        let final_link: Url = res
                                            .target
                                            .ok_or_else(|| {
                                                azure_core::Error::new(
                                                    ErrorKind::Other,
                                                    "missing target",
                                                )
                                            })?
                                            .parse()?;

                                        // Make sure the `api-version` is set appropriately.
                                        let qp = final_link
                                            .query_pairs()
                                            .filter(|(name, _)| name.ne("api-version"));
                                        let mut final_link = final_link.clone();
                                        final_link
                                            .query_pairs_mut()
                                            .clear()
                                            .extend_pairs(qp)
                                            .append_pair("api-version", &api_version);

                                        let mut request = Request::new(final_link, Method::Get);
                                        request.insert_header("accept", "application/json");

                                        let rsp: RawResponse =
                                            pipeline.send(&ctx, &mut request, None).await?;
                                        let (status, headers, body) = rsp.deconstruct();
                                        Ok(RawResponse::from_bytes(status, headers, body).into())
                                    })
                                }),
                            }
                        }
                        _ => PollerResult::Done { response: rsp },
                    })
                })
            },
            Some(options.method_options),
        ))
    }
}
