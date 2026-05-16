// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example secrets client backed by azure_core primitives.

pub mod models;

use crate::secrets::models::{
    ListSecretPropertiesResult, Secret, SecretClientListSecretPropertiesOptions, SecretProperties,
    SetSecretParameters, UpdateSecretPropertiesParameters,
};
use azure_core::{
    credentials::TokenCredential,
    error::{CheckSuccessOptions, ErrorKind},
    fmt::SafeDebug,
    http::{
        pager::{PagerContinuation, PagerResult, PagerState},
        ClientOptions, Method, Pager, Pipeline, PipelineSendOptions, RawResponse, Request,
        RequestContent, Response, Url,
    },
    json, Result,
};
use std::sync::Arc;

/// Resource identifier parsed from a Key Vault URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceId {
    /// The original URL.
    pub source_id: String,
    /// The vault base URL.
    pub vault_url: String,
    /// The resource name.
    pub name: String,
    /// The optional resource version.
    pub version: Option<String>,
}

/// Extension trait to obtain a [`ResourceId`] from a model that has an `id` field.
pub trait ResourceExt {
    /// Returns the [`ResourceId`] parsed from the resource's id URL.
    fn resource_id(&self) -> Result<ResourceId>;
}

impl ResourceExt for SecretProperties {
    fn resource_id(&self) -> Result<ResourceId> {
        let id = self.id.as_deref().ok_or_else(|| {
            azure_core::Error::with_message(ErrorKind::DataConversion, "missing resource id")
        })?;
        let url: Url = id.parse()?;
        let vault_url = format!("{}://{}", url.scheme(), url.authority());
        let mut segments = url
            .path_segments()
            .ok_or_else(|| {
                azure_core::Error::with_message(ErrorKind::DataConversion, "invalid url")
            })?
            .filter(|s| !s.is_empty());
        // skip the collection segment (e.g. "secrets")
        let _ = segments.next();
        let name = segments
            .next()
            .ok_or_else(|| {
                azure_core::Error::with_message(ErrorKind::DataConversion, "missing name")
            })?
            .to_string();
        let version = segments.next().map(String::from);
        Ok(ResourceId {
            source_id: id.to_string(),
            vault_url,
            name,
            version,
        })
    }
}

/// Options for configuring a [`SecretClient`].
#[derive(Clone, Default, SafeDebug)]
pub struct SecretClientOptions {
    /// The API version to use.
    pub api_version: String,
    /// Common client options including transport and policies.
    pub client_options: ClientOptions,
}

/// A minimal secrets client backed by azure_core.
#[derive(Debug)]
pub struct SecretClient {
    endpoint: Url,
    pipeline: Pipeline,
}

impl SecretClient {
    /// Creates a new `SecretClient`.
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<SecretClientOptions>,
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

    /// Gets a secret by name.
    pub async fn get_secret(
        &self,
        secret_name: &str,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{secret_name}"));
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        let rsp = self
            .pipeline
            .send(
                &Default::default(),
                &mut request,
                Some(PipelineSendOptions {
                    check_success: CheckSuccessOptions {
                        success_codes: &[200],
                    },
                    ..Default::default()
                }),
            )
            .await?;
        Ok(rsp.into())
    }

    /// Sets a secret.
    pub async fn set_secret(
        &self,
        secret_name: &str,
        body: RequestContent<SetSecretParameters>,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        let mut url = self.endpoint.clone();
        url.set_path(&format!("secrets/{secret_name}"));
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        request.set_body(body);
        let rsp = self
            .pipeline
            .send(
                &Default::default(),
                &mut request,
                Some(PipelineSendOptions {
                    check_success: CheckSuccessOptions {
                        success_codes: &[200],
                    },
                    ..Default::default()
                }),
            )
            .await?;
        Ok(rsp.into())
    }

    /// Updates secret properties.
    pub async fn update_secret_properties(
        &self,
        _secret_name: &str,
        _body: RequestContent<UpdateSecretPropertiesParameters>,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        unimplemented!()
    }

    /// Lists secret properties (paginated).
    pub fn list_secret_properties(
        &self,
        _options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<Pager<ListSecretPropertiesResult>> {
        let pipeline = self.pipeline.clone();
        let mut first_url = self.endpoint.clone();
        first_url.set_path("secrets");
        Ok(Pager::new(
            move |state: PagerState, pager_options| {
                let url = match state {
                    PagerState::More(continuation) => {
                        continuation.try_into().expect("expected Url")
                    }
                    PagerState::Initial => first_url.clone(),
                };
                let mut request = Request::new(url, Method::Get);
                request.insert_header("accept", "application/json");
                let pipeline = pipeline.clone();
                let first_url = first_url.clone();
                Box::pin(async move {
                    let rsp = pipeline
                        .send(
                            &pager_options.context,
                            &mut request,
                            Some(PipelineSendOptions {
                                check_success: CheckSuccessOptions {
                                    success_codes: &[200],
                                },
                                ..Default::default()
                            }),
                        )
                        .await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let res: ListSecretPropertiesResult = json::from_json(&body)?;
                    let rsp = RawResponse::from_bytes(status, headers, body).into();
                    Ok(match res.next_link {
                        Some(next_link) if !next_link.is_empty() => PagerResult::More {
                            response: rsp,
                            continuation: PagerContinuation::Link(
                                first_url.join(next_link.as_ref())?,
                            ),
                        },
                        _ => PagerResult::Done { response: rsp },
                    })
                })
            },
            None,
        ))
    }
}
