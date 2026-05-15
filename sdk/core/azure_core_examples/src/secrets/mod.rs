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
    error::ErrorKind,
    fmt::SafeDebug,
    http::{
        pager::{PagerContinuation, PagerOptions, PagerResult, PagerState},
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, Method, Pager, Pipeline, RawResponse, Request, RequestContent, Response,
        Url, UrlExt as _,
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
        parse_resource_id(self.id.as_deref())
    }
}

fn parse_resource_id(id: Option<&str>) -> Result<ResourceId> {
    let id = id.ok_or_else(|| {
        azure_core::Error::with_message(ErrorKind::DataConversion, "missing resource id")
    })?;
    let url: Url = id
        .parse()
        .map_err(|e| azure_core::Error::new(ErrorKind::DataConversion, e))?;
    let vault_url = format!("{}://{}", url.scheme(), url.authority());
    let mut segments = url
        .path_segments()
        .ok_or_else(|| azure_core::Error::with_message(ErrorKind::DataConversion, "invalid url"))?
        .filter(|s| !s.is_empty());
    // skip the collection segment (e.g. "secrets")
    segments.next().ok_or_else(|| {
        azure_core::Error::with_message(ErrorKind::DataConversion, "missing collection")
    })?;
    let name = segments
        .next()
        .ok_or_else(|| azure_core::Error::with_message(ErrorKind::DataConversion, "missing name"))?
        .to_owned();
    let version = segments.next().map(|s| s.to_owned());
    Ok(ResourceId {
        source_id: id.to_owned(),
        vault_url,
        name,
        version,
    })
}

const DEFAULT_API_VERSION: &str = "7.5";

/// Options for configuring a [`SecretClient`].
#[derive(Clone, SafeDebug)]
pub struct SecretClientOptions {
    /// The API version to use.
    pub api_version: String,
    /// Common client options including transport and policies.
    pub client_options: ClientOptions,
}

impl Default for SecretClientOptions {
    fn default() -> Self {
        Self {
            api_version: DEFAULT_API_VERSION.to_owned(),
            client_options: ClientOptions::default(),
        }
    }
}

/// A minimal secrets client backed by azure_core.
#[derive(Debug)]
pub struct SecretClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

impl SecretClient {
    /// Creates a new `SecretClient`.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<SecretClientOptions>,
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

    /// Gets a secret by name.
    pub async fn get_secret(
        &self,
        secret_name: &str,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        let mut url = self.endpoint.clone();
        url.append_path(&format!("/secrets/{secret_name}/"));
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/json");
        let rsp = self
            .pipeline
            .send(&Default::default(), &mut request, None)
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
        url.append_path(&format!("/secrets/{secret_name}"));
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        let body: azure_core::http::Body = body.into();
        request.set_body(&body);
        let rsp = self
            .pipeline
            .send(&Default::default(), &mut request, None)
            .await?;
        Ok(rsp.into())
    }

    /// Updates secret properties.
    pub async fn update_secret_properties(
        &self,
        secret_name: &str,
        body: RequestContent<UpdateSecretPropertiesParameters>,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        let mut url = self.endpoint.clone();
        url.append_path(&format!("/secrets/{secret_name}/"));
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);
        let mut request = Request::new(url, Method::Patch);
        request.insert_header("accept", "application/json");
        request.insert_header("content-type", "application/json");
        let body: azure_core::http::Body = body.into();
        request.set_body(&body);
        let rsp = self
            .pipeline
            .send(&Default::default(), &mut request, None)
            .await?;
        Ok(rsp.into())
    }

    /// Lists secret properties (paginated).
    pub fn list_secret_properties(
        &self,
        options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<Pager<ListSecretPropertiesResult>> {
        let pager_options = options.map(|o| {
            let method_options = o.method_options;
            PagerOptions {
                context: method_options.context.into_owned(),
                ..method_options
            }
        });
        let pipeline = self.pipeline.clone();
        let api_version = self.api_version.clone();
        let mut first_url = self.endpoint.clone();
        first_url.append_path("/secrets");
        first_url
            .query_pairs_mut()
            .append_pair("api-version", &api_version);

        Ok(Pager::new(
            move |state: PagerState, pager_options| {
                let pipeline = pipeline.clone();
                let api_version = api_version.clone();
                let url = match state {
                    PagerState::More(next_link) => {
                        let mut next_link: Url = next_link.try_into().expect("expected Url");
                        let qp: Vec<_> = next_link
                            .query_pairs()
                            .filter(|(k, _)| k != "api-version")
                            .map(|(k, v)| (k.into_owned(), v.into_owned()))
                            .collect();
                        next_link.query_pairs_mut().clear().extend_pairs(qp);
                        next_link
                            .query_pairs_mut()
                            .append_pair("api-version", &api_version);
                        next_link
                    }
                    PagerState::Initial => first_url.clone(),
                };
                let mut request = Request::new(url.clone(), Method::Get);
                request.insert_header("accept", "application/json");
                Box::pin(async move {
                    let rsp = pipeline
                        .send(&pager_options.context, &mut request, None)
                        .await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let result: ListSecretPropertiesResult = json::from_json(&body)?;
                    let response: Response<ListSecretPropertiesResult> =
                        RawResponse::from_bytes(status, headers, body).into();
                    Ok(match result.next_link {
                        Some(ref next_link) if !next_link.is_empty() => PagerResult::More {
                            response,
                            continuation: PagerContinuation::Link(url.join(next_link.as_ref())?),
                        },
                        _ => PagerResult::Done { response },
                    })
                })
            },
            pager_options,
        ))
    }
}
