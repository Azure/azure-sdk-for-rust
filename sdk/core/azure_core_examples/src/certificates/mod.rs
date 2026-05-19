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
        poller::{
            get_retry_after, Poller, PollerContinuation, PollerResult, PollerState, PollerStatus,
            StatusMonitor as _,
        },
        Body, ClientOptions, Method, Pipeline, RawResponse, Request, RequestContent, Url,
    },
    json, Result,
};
use std::sync::Arc;

/// Options for configuring a [`CertificateClient`].
#[derive(Clone, Default, SafeDebug)]
pub struct CertificateClientOptions {
    /// The API version to use.
    pub api_version: String,
    /// Common client options including transport and policies.
    pub client_options: ClientOptions,
}

/// A minimal certificates client backed by azure_core.
#[derive(Debug)]
pub struct CertificateClient {
    endpoint: Url,
    pipeline: Pipeline,
}

impl CertificateClient {
    /// Creates a new `CertificateClient`.
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<CertificateClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint)?;
        let options = options.unwrap_or_default();
        Ok(Self {
            endpoint,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::new(),
                Vec::new(),
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
        let certificate_name = certificate_name.to_owned();

        let mut create_url = self.endpoint.clone();
        create_url.set_path(&format!("certificates/{certificate_name}/create"));

        let mut pending_url = self.endpoint.clone();
        pending_url.set_path(&format!("certificates/{certificate_name}/pending"));

        let parameters: Body = parameters.into();

        Ok(Poller::new(
            move |poller_state: PollerState, poller_options| {
                let (mut request, next_link) = match poller_state {
                    PollerState::More(continuation) => {
                        let next_link = match continuation {
                            PollerContinuation::Links { next_link, .. } => next_link,
                            _ => unreachable!(),
                        };
                        let mut request = Request::new(next_link.clone(), Method::Get);
                        request.insert_header("accept", "application/json");
                        (request, next_link)
                    }
                    PollerState::Initial => {
                        let mut request = Request::new(create_url.clone(), Method::Post);
                        request.insert_header("accept", "application/json");
                        request.insert_header("content-type", "application/json");
                        request.set_body(&parameters);
                        (request, pending_url.clone())
                    }
                };
                let pipeline = pipeline.clone();
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
                            let target_url: Url = res
                                .target
                                .ok_or_else(|| {
                                    azure_core::Error::new(ErrorKind::Other, "missing target")
                                })?
                                .parse()?;
                            PollerResult::Succeeded {
                                response: rsp,
                                target: Box::new(move || {
                                    Box::pin(async move {
                                        let mut request = Request::new(target_url, Method::Get);
                                        request.insert_header("accept", "application/json");
                                        let rsp = pipeline.send(&ctx, &mut request, None).await?;
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
            None,
        ))
    }
}
