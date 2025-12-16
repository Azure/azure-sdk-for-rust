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
    http::{NoFormat, Pager, RequestContent, Response, Url, XmlFormat},
    Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueServiceClient {
    pub(super) client: GeneratedQueueClient,
}

impl QueueServiceClient {
    /// Creates a new QueueServiceClient using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token for authentication
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let client = GeneratedQueueClient::new(endpoint, credential.clone(), Some(options))?;
        Ok(Self { client })
    }

    /// Returns the endpoint URL of the Azure storage account this client is associated with.
    pub fn endpoint(&self) -> &Url {
        self.client.endpoint()
    }

    /// Returns a new instance of QueueClient.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue.
    pub fn queue_client(&self, queue_name: String) -> QueueClient {
        QueueClient {
            client: self.client.get_queue_client(queue_name),
        }
    }

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    /// * `queue_name` - The name of the queue to create.
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
    /// * `options` - Optional configuration for the request.
    /// * `queue_name` - The name of the queue to delete.
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

    /// Retrieves the properties for the entire queue service.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_properties(
        &self,
        options: Option<QueueServiceClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<QueueServiceProperties, XmlFormat>> {
        self.client.get_properties(options).await
    }

    /// Sets the properties of the queue service.
    ///
    /// # Arguments
    ///
    /// * `storage_service_properties` - The properties to set for the queue service.
    /// * `content_type` - The content type of the request body, typically "application/xml"
    /// * `options` - Optional configuration for the request.
    pub async fn set_properties(
        &self,
        queue_service_properties: RequestContent<QueueServiceProperties, XmlFormat>,
        options: Option<QueueServiceClientSetPropertiesOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .set_properties(queue_service_properties, options)
            .await
    }

    /// Lists queues in the storage account, returning a segment of results.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub fn list_queues(
        &self,
        options: Option<QueueServiceClientListQueuesOptions<'_>>,
    ) -> Result<Pager<ListQueuesResponse, XmlFormat, String>> {
        self.client.list_queues(options)
    }

    /// Retrieves statistics related to replication for the Queue service. Note: Queue statistics are only available on
    /// the secondary location endpoint when read-access geo-redundant replication is enabled for the Storage account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn get_statistics(
        &self,
        options: Option<QueueServiceClientGetStatisticsOptions<'_>>,
    ) -> Result<Response<QueueServiceStats, XmlFormat>> {
        self.client.get_statistics(options).await
    }
}
