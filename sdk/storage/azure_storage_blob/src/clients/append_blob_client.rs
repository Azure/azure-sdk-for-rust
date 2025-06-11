// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::AppendBlobClient as GeneratedAppendBlobClient,
    models::{AppendBlobClientCreateOptions, AppendBlobClientCreateResult},
    pipeline::StorageHeadersPolicy,
    AppendBlobClientOptions, BlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        RequestContent, Response, Url,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage Append blob, although that blob may not yet exist.
pub struct AppendBlobClient {
    pub(crate) endpoint: Url,
    pub(crate) client: GeneratedAppendBlobClient,
}

impl AppendBlobClient {
    /// Creates a new AppendBlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container containing this Append blob.
    /// * `blob_name` - The name of the Append blob to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<AppendBlobClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            ["https://storage.azure.com/.default"],
        );
        options
            .client_options
            .per_try_policies
            .push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);

        let client = GeneratedAppendBlobClient::new(
            endpoint,
            credential.clone(),
            container_name.clone(),
            blob_name.clone(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Gets the endpoint of the Storage account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Gets the container name of the Storage account this client is connected to.
    pub fn container_name(&self) -> &str {
        &self.client.container_name
    }

    /// Gets the blob name of the Storage account this client is connected to.
    pub fn blob_name(&self) -> &str {
        &self.client.blob_name
    }

    /// Creates a new Append blob.
    ///
    /// # Arguments
    ///
    /// * `content_length` - Total length of the blob data to be uploaded.
    /// * `options` - Optional configuration for the request.
    pub async fn create(
        &self,
        options: Option<AppendBlobClientCreateOptions<'_>>,
    ) -> Result<Response<AppendBlobClientCreateResult>> {
        self.client.create(options).await
    }
}
