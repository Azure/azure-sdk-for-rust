// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobServiceClient, BlobServiceClientOptions};

use crate::{
    generated::models::{BlobServiceClientGetUserDelegationKeyOptions, KeyInfo, UserDelegationKey},
    BlobClient, BlobContainerClient,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, RequestContent, Url, XmlFormat,
    },
    tracing, Result,
};
use std::sync::Arc;
use time::{macros::format_description, OffsetDateTime};

impl BlobServiceClient {
    /// Creates a new BlobServiceClient from a service URL.
    ///
    /// # Arguments
    ///
    /// * `service_url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn new(
        service_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if service_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{service_url} is not a valid base URL"),
            ));
        }
        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !service_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{service_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: service_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of BlobContainerClient.
    ///
    /// # Arguments
    ///
    /// * `container_name` - The name of the container.
    pub fn blob_container_client(&self, container_name: &str) -> BlobContainerClient {
        let mut container_url = self.url().clone();
        container_url
            .path_segments_mut()
            // This should not fail as service URL has already been validated on client construction.
            .expect("Cannot be a base URL.")
            .push(container_name);

        BlobContainerClient {
            endpoint: container_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `container_name` - The name of the container.
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, container_name: &str, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as service URL has already been validated on client construction.
            .expect("Cannot be a base URL.")
            .extend([container_name, blob_name]);

        BlobClient {
            endpoint: blob_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Fetches a user delegation key valid from now until `expiry`.
    ///
    /// The key can be used to sign user delegation SAS tokens without embedding a storage account
    /// key. When the `sas` feature is enabled, convert it to
    /// [`azure_storage_sas::UserDelegationKey`] with `.into()` and pass it to
    /// [`azure_storage_sas::UserDelegationSasBuilder::with_key`].
    ///
    /// # Arguments
    ///
    /// * `expiry` - The expiry time for the user delegation key. Must be within 7 days from now.
    /// * `options` - Optional parameters for the request.
    pub async fn get_user_delegation_key(
        &self,
        expiry: OffsetDateTime,
        options: Option<BlobServiceClientGetUserDelegationKeyOptions<'_>>,
    ) -> Result<UserDelegationKey> {
        let fmt = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
        let start = OffsetDateTime::now_utc()
            .format(fmt)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let expiry_str = expiry
            .format(fmt)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let key_info = RequestContent::<KeyInfo, XmlFormat>::try_from(KeyInfo {
            start,
            expiry: expiry_str,
        })?;
        let rsp = self
            .get_user_delegation_key_internal(key_info, options)
            .await?;
        rsp.into_model()
    }
}
