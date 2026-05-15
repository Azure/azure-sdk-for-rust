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
    fmt::SafeDebug,
    http::{ClientOptions, Method, Pager, Pipeline, Request, RequestContent, Response, Url},
    Result,
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
        unimplemented!()
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
pub struct SecretClient(Pipeline);

impl SecretClient {
    /// Creates a new `SecretClient`.
    pub fn new(
        _endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<SecretClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self(Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options,
            Vec::new(),
            Vec::new(),
            None,
        )))
    }

    /// Gets a secret by name.
    pub async fn get_secret(
        &self,
        _secret_name: &str,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        let mut request =
            Request::new(Url::parse("https://my-vault.vault.azure.net")?, Method::Get);
        let rsp = self.0.send(&Default::default(), &mut request, None).await?;
        Ok(rsp.into())
    }

    /// Sets a secret.
    pub async fn set_secret(
        &self,
        _secret_name: &str,
        _body: RequestContent<SetSecretParameters>,
        _options: Option<()>,
    ) -> Result<Response<Secret>> {
        unimplemented!()
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
        unimplemented!()
    }
}
