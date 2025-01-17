// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::blob_blob_client::BlobBlobClientGetPropertiesOptions;
use crate::blob_client::BlobClientOptions;
use crate::models::blob_properties::{build_from_response_headers, BlobProperties};
use crate::policies::storage_headers_policy::StorageHeadersPolicy;
use crate::BlobClient as GeneratedBlobClient;
use azure_core::credentials::TokenCredential;
use azure_core::headers::HeaderName;
use azure_core::{
    BearerTokenCredentialPolicy, Context, Error, Method, Policy, Request, RequestContent, Response,
    Result, Url,
};
use azure_identity::DefaultAzureCredentialBuilder;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct BlobClient {
    pub(crate) endpoint: String,
    pub(crate) container_name: String,
    pub(crate) blob_name: String,
    pub(crate) credential: Arc<dyn TokenCredential>,
    pub(crate) client: GeneratedBlobClient,
}

impl BlobClient {
    pub fn new(
        endpoint: String,
        container_name: String,
        blob_name: String,
        credential: Arc<dyn TokenCredential>,
        mut options: BlobClientOptions,
    ) -> Result<Self> {
        let storage_headers_policy = Arc::new(StorageHeadersPolicy::new());
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

        let client = GeneratedBlobClient::new(
            &endpoint.clone(),
            credential.clone(),
            container_name.clone(),
            Some(options),
        )?;

        Ok(Self {
            endpoint: endpoint.clone(),
            container_name: container_name.clone(),
            blob_name: blob_name.clone(),
            credential,
            client: client,
        })
    }

    pub async fn get_blob_properties(
        &self,
        options: Option<BlobBlobClientGetPropertiesOptions<'_>>,
    ) -> Result<BlobProperties> {
        let response = self
            .client
            .get_blob_blob_client()
            .get_properties(self.container_name.clone(), self.blob_name.clone(), options)
            .await?;

        Ok(build_from_response_headers(response.headers()))
    }
}
