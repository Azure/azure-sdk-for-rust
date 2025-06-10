// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::models::{
    AzureQueueStorageQueueOperationsClientCreateOptions,
    AzureQueueStorageQueueOperationsClientDeleteOptions, QueueApiVersion,
};
use crate::{
    generated::clients::AzureQueueStorageClient as GeneratedQueueClient,
    generated::clients::AzureQueueStorageClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        RequestContent, Response, Url, XmlFormat,
    },
    Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedQueueClient,
    pub(super) version: QueueApiVersion,
}

impl QueueClient {
    /// Creates a new BlobClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myqueue.queue.core.windows.net/`
    /// * `queue_name` - The name of the queue to interact with.
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        queue_name: String,
        credential: Arc<dyn TokenCredential>,
        options: Option<AzureQueueStorageClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();

        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            ["https://storage.azure.com/.default"],
        );
        options
            .client_options
            .per_try_policies
            .push(Arc::new(oauth_token_policy) as Arc<dyn Policy>);

        let client = GeneratedQueueClient::new(
            endpoint,
            credential.clone(),
            QueueApiVersion::StringValue2018_03_28.to_string(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
            version: QueueApiVersion::StringValue2018_03_28,
        })
    }

    /// creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The queue name.
    /// * `version` - Specifies the version of the operation to use for this request.
    /// * `options` - Optional parameters for the request.
    pub async fn create(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageQueueOperationsClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_queue_operations_client()
            .create(queue_name, self.version.clone(), options)
            .await
    }

    /// operation permanently deletes the specified queue
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The queue name.
    /// * `version` - Specifies the version of the operation to use for this request.
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageQueueOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_queue_operations_client()
            .delete(queue_name, self.version.clone(), options)
            .await
    }
}
