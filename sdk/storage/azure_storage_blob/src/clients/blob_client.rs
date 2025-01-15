// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::blob_blob_client::BlobBlobClientGetPropertiesOptions;
use crate::blob_client::BlobClientOptions;
use crate::clients::units::*;
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
use std::marker::PhantomData;
use std::sync::Arc;
use uuid::Uuid;

pub struct BlobClient<BlobType = Unset> {
    pub(crate) blob_type: PhantomData<BlobType>,
    pub(crate) endpoint: String,
    pub(crate) container_name: String,
    pub(crate) blob_name: String,
    pub(crate) credential: Option<Arc<dyn TokenCredential>>,
    pub(crate) client: GeneratedBlobClient,
}

impl BlobClient<Unset> {
    pub fn new(
        endpoint: String,
        container_name: String,
        blob_name: String,
        credential: Option<Arc<dyn TokenCredential>>,
        mut options: Option<BlobClientOptions>,
    ) -> Result<Self> {
        let mut options = BlobClientOptions::default();

        // Now that we don't need the setters/getters, we may be able to directly push it on instead of cloning down and reassigning
        let storage_headers_policy = Arc::new(StorageHeadersPolicy::new());
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        // Conditionally add authentication if provided
        if credential.is_some() {
            let oauth_token_policy = BearerTokenCredentialPolicy::new(
                credential.clone().unwrap(),
                ["https://storage.azure.com/.default"],
            );
            let mut per_try_policies = options.client_options.per_call_policies.clone();
            per_try_policies.push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);
            options.client_options.per_call_policies = per_try_policies;
        }

        let client = GeneratedBlobClient::new(
            &endpoint.clone(),
            credential.clone().unwrap(), // This unwrap() is temporary- this may be a straight passthrough dependent on how we handle auth-less
            container_name.clone(),
            Some(options),
        )?;

        Ok(Self {
            blob_type: PhantomData::<Unset>,
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
    ) -> Result<Response<()>> {
        self.client
            .get_blob_blob_client()
            .get_properties(self.container_name.clone(), self.blob_name.clone(), options)
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_get_blob_properties() {
        let credential = DefaultAzureCredentialBuilder::default().build().unwrap();
        let blob_client = BlobClient::new(
            String::from("https://vincenttranpublicac.blob.core.windows.net/"),
            String::from("public"),
            String::from("hello.txt"),
            Some(credential),
            Some(BlobClientOptions::default()),
        )
        .unwrap();
        let response = blob_client
            .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
            .await
            .unwrap();
        print!("{:?}", response);
    }
}
