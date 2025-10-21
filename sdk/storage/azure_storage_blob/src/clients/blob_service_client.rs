// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::BlobContainerClient as GeneratedBlobContainerClient,
    generated::clients::BlobServiceClient as GeneratedBlobServiceClient,
    generated::models::BlobServiceClientGetAccountInfoResult,
    models::{
        BlobServiceClientFindBlobsByTagsOptions, BlobServiceClientGetAccountInfoOptions,
        BlobServiceClientGetPropertiesOptions, BlobServiceClientListContainersSegmentOptions,
        BlobServiceClientSetPropertiesOptions, BlobServiceProperties, FilterBlobSegment,
        ListContainersSegmentResponse,
    },
    pipeline::StorageHeadersPolicy,
    BlobContainerClient, BlobServiceClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, PageIterator, Pipeline, RequestContent, Response, Url, XmlFormat,
    },
    tracing, Result,
};
use std::sync::Arc;

/// A client to interact with an Azure storage account.
pub struct BlobServiceClient {
    pub(super) client: GeneratedBlobServiceClient,
}

impl GeneratedBlobServiceClient {
    /// Creates a new GeneratedBlobServiceClient from the URL of the Azure storage account.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Service")]
    pub fn from_url(
        url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let per_retry_policies = if let Some(token_credential) = credential {
            if !url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{url} must use https"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            ));
            vec![auth_policy]
        } else {
            Vec::default()
        };

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: url,
            version: options.version,
            pipeline,
        })
    }
}

impl BlobServiceClient {
    /// Creates a new BlobServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let url = Url::parse(endpoint)?;

        let client = GeneratedBlobServiceClient::from_url(url, Some(credential), options)?;
        Ok(Self { client })
    }

    /// Creates a new BlobServiceClient, without providing any authentication information.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `options` - Optional configuration for the client.
    pub fn with_no_credential(
        endpoint: &str,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let url = Url::parse(endpoint)?;

        let client = GeneratedBlobServiceClient::from_url(url, None, options)?;
        Ok(Self { client })
    }

    /// Creates a new BlobServiceClient from the URL of the Azure storage account.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn from_url(
        url: Url,
        credential: Arc<dyn TokenCredential>,
        options: Option<BlobServiceClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedBlobServiceClient::from_url(url, Some(credential), options)?;

        Ok(Self { client })
    }

    /// Creates a new BlobServiceClient from the URL of the Azure storage account containing the SAS (shared access signature) query parameters.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL of the Azure storage account, including the SAS query parameters.
    /// * `options` - Optional configuration for the client.
    pub fn from_sas_url(url: Url, options: Option<BlobServiceClientOptions>) -> Result<Self> {
        let client = GeneratedBlobServiceClient::from_url(url, None, options)?;

        Ok(Self { client })
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
            .expect("Cannot be base")
            .push(container_name);

        let client = GeneratedBlobContainerClient {
            endpoint: container_url,
            pipeline: self.client.pipeline.clone(),
            version: self.client.version.clone(),
            tracer: self.client.tracer.clone(),
        };

        BlobContainerClient { client }
    }

    /// Gets the URL of the Storage account this client is connected to.
    pub fn url(&self) -> &Url {
        &self.client.endpoint
    }

    /// Gets the properties of a Storage account's Blob service, including Azure Storage Analytics.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<BlobServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<BlobServiceProperties, XmlFormat>> {
        self.client.get_properties(options).await
    }

    /// Returns a list of the containers under the specified Storage account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub fn list_containers(
        &self,
        options: Option<BlobServiceClientListContainersSegmentOptions<'_>>,
    ) -> Result<PageIterator<Response<ListContainersSegmentResponse, XmlFormat>>> {
        self.client.list_containers_segment(options)
    }

    /// Returns a list of blobs across all containers whose tags match a given search expression.
    ///
    /// # Arguments
    ///
    /// * `filter_expression` - The expression to find blobs whose tags matches the specified condition.
    ///   eg.
    /// ```text
    /// "\"yourtagname\"='firsttag' and \"yourtagname2\"='secondtag'"
    /// ```
    ///   To specify a container, eg.
    /// ```text
    /// "@container='containerName' and \"Name\"='C'"
    /// ```
    /// See [`format_filter_expression()`](crate::format_filter_expression) for help with the expected String format.
    /// * `options` - Optional parameters for the request.
    pub async fn find_blobs_by_tags(
        &self,
        filter_expression: &str,
        options: Option<BlobServiceClientFindBlobsByTagsOptions<'_>>,
    ) -> Result<Response<FilterBlobSegment, XmlFormat>> {
        self.client
            .find_blobs_by_tags(filter_expression, options)
            .await
    }

    /// Sets properties for a Storage account's Blob service endpoint, including properties for Storage Analytics and CORS rules.
    ///
    /// # Arguments
    ///
    /// * `storage_service_properties` - The Storage service properties to set.
    /// * `options` - Optional configuration for the request.
    pub async fn set_properties(
        &self,
        storage_service_properties: RequestContent<BlobServiceProperties, XmlFormat>,
        options: Option<BlobServiceClientSetPropertiesOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .set_properties(storage_service_properties, options)
            .await
    }

    /// Gets information related to the Storage account.
    /// This includes the `sku_name` and `account_kind`.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_account_info(
        &self,
        options: Option<BlobServiceClientGetAccountInfoOptions<'_>>,
    ) -> Result<Response<BlobServiceClientGetAccountInfoResult, NoFormat>> {
        self.client.get_account_info(options).await
    }
}
