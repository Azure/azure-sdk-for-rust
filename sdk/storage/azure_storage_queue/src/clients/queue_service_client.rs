// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::QueueClient,
    generated::{
        clients::{QueueServiceClient as GeneratedQueueClient, QueueServiceClientOptions},
        models::*,
    },
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        Context, Method, NoFormat, PageIterator, RawResponse, Request, RequestContent, Response,
        StatusCode, Url, XmlFormat,
    },
    xml, Bytes, Result,
};
use std::{collections::HashMap, sync::Arc};

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueServiceClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedQueueClient,
}

impl QueueServiceClient {
    /// Returns the endpoint URL of the Azure storage account this client is associated with.
    ///
    /// # Returns
    ///
    /// A reference to the URL of the storage account.
    pub fn endpoint(&self) -> &Url {
        self.client.endpoint()
    }

    /// Creates a new QueueServiceClient using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://<storage_account_name>.queue.core.windows.net/`
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token for authentication
    /// * `options` - Optional configuration for the client
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `QueueServiceClient` if successful, or an error if the endpoint URL is invalid
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let client = GeneratedQueueClient::new(endpoint, credential.clone(), Some(options))?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            client,
        })
    }

    /// Returns a new instance of QueueClient.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue.
    ///
    /// # Returns
    ///
    /// Returns a `QueueClient` that can be used to interact with the specified queue.
    pub fn queue_client(&self, queue_name: String) -> QueueClient {
        QueueClient {
            endpoint: self.endpoint.clone(),
            client: self.client.get_queue_client(queue_name),
        }
    }

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    /// * `queue_name` - The name of the queue to create
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue already exists or if the request fails
    pub async fn create_queue(
        &self,
        queue_name: &str,
        options: Option<QueueClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .get_queue_client(queue_name.to_string())
            .create(options)
            .await
    }

    /// Permanently deletes the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    /// * `queue_name` - The name of the queue to create
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn delete_queue(
        &self,
        queue_name: &str,
        options: Option<QueueClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .get_queue_client(queue_name.to_string())
            .delete(options)
            .await
    }

    /// Retrieves the properties of the queue service.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the service properties response if successful
    ///
    /// # Note
    ///
    /// This returns properties for the entire service, not just a single queue.
    pub async fn get_properties(
        &self,
        options: Option<QueueServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServiceProperties, XmlFormat>> {
        self.client.get_properties(options).await
    }

    /// Sets the properties of the queue service.
    ///
    /// # Arguments
    ///
    /// * `storage_service_properties` - The properties to set for the queue service
    /// * `content_type` - The content type of the request body, typically "application/xml"
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful.
    pub async fn set_properties(
        &self,
        storage_service_properties: RequestContent<StorageServiceProperties>,
        options: Option<QueueServiceClientSetPropertiesOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .set_properties(storage_service_properties, options)
            .await
    }

    /// Lists queues in the storage account, returning a segment of results.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request, such as prefix and max results
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `PageIterator` for paginated results, or an error if the request fails.
    ///
    /// The `PageIterator` can be used to iterate through the results page by page.
    pub fn list_queues(
        &self,
        options: Option<QueueServiceClientListQueuesOptions<'_>>,
    ) -> Result<PageIterator<Response<ListQueuesResponse, XmlFormat>>> {
        self.client.list_queues(options)
    }

    /// Retrieves statistics related to replication for the Queue service. It is only available on the secondary location endpoint
    /// when read-access geo-redundant replication is enabled for the storage account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the service statistics response if successful.
    pub async fn get_statistics(
        &self,
        options: Option<QueueServiceClientGetStatisticsOptions<'_>>,
    ) -> Result<Response<StorageServiceStats, XmlFormat>> {
        self.client.get_statistics(options).await
    }
}
