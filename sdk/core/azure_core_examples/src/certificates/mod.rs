// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example certificates client backed by azure_core primitives.

pub mod models;

use crate::certificates::models::{CertificateOperation, CreateCertificateParameters};
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
        UrlExt as _,
    },
    json, Result,
};
use std::sync::Arc;

const DEFAULT_API_VERSION: &str = "7.5";

/// Options for configuring a [`CertificateClient`].
#[derive(Clone, SafeDebug)]
pub struct CertificateClientOptions {
    /// The API version to use.
    pub api_version: String,
    /// Common client options including transport and policies.
    pub client_options: ClientOptions,
}

impl Default for CertificateClientOptions {
    fn default() -> Self {
        Self {
            api_version: DEFAULT_API_VERSION.to_owned(),
            client_options: ClientOptions::default(),
        }
    }
}

/// A minimal certificates client backed by azure_core.
#[derive(Debug)]
pub struct CertificateClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

impl CertificateClient {
    /// Creates a new `CertificateClient`.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<CertificateClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let endpoint = Url::parse(endpoint)?;
        let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
            credential,
            vec!["https://vault.azure.net/.default"],
        ));
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

    /// Begins creating a certificate and returns a poller to track the operation.
    pub fn begin_create_certificate(
        &self,
        certificate_name: &str,
        parameters: RequestContent<CreateCertificateParameters>,
        _options: Option<()>,
    ) -> Result<Poller<CertificateOperation>> {
        let pipeline = self.pipeline.clone();
        let api_version = self.api_version.clone();
        let certificate_name = certificate_name.to_owned();
        let parameters: Body = parameters.into();

        let mut create_url = self.endpoint.clone();
        create_url.append_path(&format!("/certificates/{certificate_name}/create"));
        create_url
            .query_pairs_mut()
            .append_pair("api-version", &api_version);

        let mut pending_url = self.endpoint.clone();
        pending_url.append_path(&format!("/certificates/{certificate_name}/pending"));
        pending_url
            .query_pairs_mut()
            .append_pair("api-version", &api_version);

        Ok(Poller::new(
            move |poller_state: PollerState, poller_options| {
                let (mut request, next_link) = match poller_state {
                    PollerState::More(continuation) => {
                        let next_link = match continuation {
                            PollerContinuation::Links { next_link, .. } => next_link,
                            _ => unreachable!("unexpected continuation variant"),
                        };
                        // Preserve api-version
                        let qp: Vec<_> = next_link
                            .query_pairs()
                            .filter(|(k, _)| k != "api-version")
                            .map(|(k, v)| (k.into_owned(), v.into_owned()))
                            .collect();
                        let mut next_link = next_link.clone();
                        next_link.query_pairs_mut().clear().extend_pairs(qp);
                        next_link
                            .query_pairs_mut()
                            .append_pair("api-version", &api_version);
                        let mut req = Request::new(next_link.clone(), Method::Get);
                        req.insert_header("accept", "application/json");
                        (req, next_link)
                    }
                    PollerState::Initial => {
                        let mut req = Request::new(create_url.clone(), Method::Post);
                        req.insert_header("accept", "application/json");
                        req.insert_header("content-type", "application/json");
                        req.set_body(&parameters);
                        (req, pending_url.clone())
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
                    let operation: CertificateOperation = json::from_json(&body)?;
                    let response: azure_core::http::Response<CertificateOperation> =
                        RawResponse::from_bytes(status, headers, body).into();

                    Ok(match operation.status() {
                        PollerStatus::InProgress => PollerResult::InProgress {
                            response,
                            retry_after,
                            continuation: PollerContinuation::Links {
                                next_link,
                                final_link: None,
                            },
                        },
                        PollerStatus::Succeeded => PollerResult::Succeeded {
                            response,
                            target: Box::new(move || {
                                Box::pin(async move {
                                    let final_link: Url = operation
                                        .target
                                        .ok_or_else(|| {
                                            azure_core::Error::new(
                                                ErrorKind::Other,
                                                "missing target",
                                            )
                                        })?
                                        .parse()
                                        .map_err(|e| azure_core::Error::new(ErrorKind::Other, e))?;

                                    let qp: Vec<_> = final_link
                                        .query_pairs()
                                        .filter(|(k, _)| k != "api-version")
                                        .map(|(k, v)| (k.into_owned(), v.into_owned()))
                                        .collect();
                                    let mut final_link = final_link.clone();
                                    final_link.query_pairs_mut().clear().extend_pairs(qp);
                                    final_link
                                        .query_pairs_mut()
                                        .append_pair("api-version", &api_version);

                                    let mut req = Request::new(final_link, Method::Get);
                                    req.insert_header("accept", "application/json");
                                    let rsp = pipeline.send(&ctx, &mut req, None).await?;
                                    let (status, headers, body) = rsp.deconstruct();
                                    Ok(RawResponse::from_bytes(status, headers, body).into())
                                })
                            }),
                        },
                        _ => PollerResult::Done { response },
                    })
                })
            },
            None,
        ))
    }
}
