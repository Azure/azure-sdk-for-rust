// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::clients::{
    AzureQueueStorageClient as GeneratedQueueClient, AzureQueueStorageClientOptions,
};
use crate::generated::models::{
    AzureQueueStorageMessageIdOperationsClientDeleteOptions,
    AzureQueueStorageMessagesOperationsClientDequeueOptions,
    AzureQueueStorageMessagesOperationsClientEnqueueOptions,
    AzureQueueStorageQueueOperationsClientCreateOptions,
    AzureQueueStorageQueueOperationsClientDeleteOptions, ListOfDequeuedMessageItem,
    ListOfEnqueuedMessage, QueueApiVersion, QueueMessage, ServicePropertiesCompType,
    StorageServicePropertiesResponse,
};
use azure_core::http::StatusCode;
use azure_core::xml;
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        Context, Method, Request, RequestContent, Response, Url, XmlFormat,
    },
    Bytes, Result,
};
use std::collections::HashMap;
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
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://<storage_account_name>.queue.core.windows.net/`
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<AzureQueueStorageClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

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

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The queue name.
    /// * `version` - Specifies the version of the operation to use for this request.
    /// * `options` - Optional parameters for the request.
    ///
    /// Will fail if the queue already exists.
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

    /// Creates a new queue under the given account. Will not fail if the queue already exists.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The queue name.
    /// * `version` - Specifies the version of the operation to use for this request.
    /// * `options` - Optional parameters for the request.
    pub async fn create_if_not_exists(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageQueueOperationsClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        // Attempt to create the queue, if it already exists, this will return an error.
        match self.create(queue_name, options).await {
            Ok(response) => Ok(response),
            Err(e) if e.http_status().unwrap() == StatusCode::Conflict => {
                // If the error is a conflict (queue already exists), we return Ok with no content.
                use azure_core::http::{headers::Headers, RawResponse};
                Ok(
                    RawResponse::from_bytes(StatusCode::NoContent, Headers::new(), Bytes::new())
                        .into(),
                )
            }
            Err(e) => Err(e), // Propagate other errors.
        }
    }

    /// Deletes the specified queue if it exists.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The queue name.
    /// * `options` - Optional parameters for the request.
    ///
    /// This method will not fail if the queue does not exist; it will return a 204 No Content response.
    pub async fn delete_if_exists(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageQueueOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        // Attempt to delete the queue, if it does not exist, this will return an error.
        match self.delete(queue_name, options).await {
            Ok(response) => Ok(response),
            Err(e) if e.http_status().unwrap() == StatusCode::NotFound => {
                // If the error is a not found (queue does not exist), we return Ok with no content.
                use azure_core::http::{headers::Headers, RawResponse};
                Ok(
                    RawResponse::from_bytes(StatusCode::NoContent, Headers::new(), Bytes::new())
                        .into(),
                )
            }
            Err(e) => Err(e), // Propagate other errors.
        }
    }

    /// Permanently delete the specified queue
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

    /// Retrieves the properties of the specified queue service.
    pub async fn get_properties(
        &self,
    ) -> Result<Response<StorageServicePropertiesResponse, XmlFormat>> {
        self.client
            .get_azure_queue_storage_service_operations_client()
            .get_properties(
                crate::generated::models::ServiceRestypeType::Service,
                crate::generated::models::ServicePropertiesCompType::Properties,
                self.version.clone(),
                None,
            )
            .await
    }

    /// Checks if a queue with the specified name exists.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to check.
    ///
    /// Returns `Ok(true)` if the queue exists, `Ok(false)` if it does not exist, or an error if the request fails for any other reason.
    pub async fn exists(&self, queue_name: &str) -> Result<bool> {
        match self.get_metadata(queue_name).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status().unwrap() == StatusCode::NotFound => {
                // If the queue does not exist, we return false.
                Ok(false)
            }
            Err(e) => {
                // Propagate other errors.
                Err(e)
            }
        }
    }

    /// Deletes all messages in the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue from which to delete messages.
    ///
    /// Returns a `Response` indicating the result of the operation.
    pub async fn delete_messages(&self, queue_name: &str) -> Result<Response<()>> {
        let messages_client = self
            .client
            .get_azure_queue_storage_messages_operations_client();

        let result = messages_client
            .clear(queue_name, self.version.clone(), None)
            .await?;
        Ok(result)
    }

    /// Sets the metadata for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue for which to set metadata.
    /// * `metadata` - A map of metadata key-value pairs to set for the queue. If `None`, no metadata will be set.
    ///
    /// Returns a `Response` indicating the result of the operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or if the queue does not exist.
    pub async fn set_metadata(
        &self,
        queue_name: &str,
        metadata: Option<HashMap<&str, &str>>,
    ) -> Result<Response<()>> {
        let mut url = self.client.endpoint.clone();
        let ctx = Context::new();

        url.path_segments_mut()
            .expect("Invalid URL")
            .push(queue_name);
        url.query_pairs_mut()
            .append_pair("api-version", &self.client.api_version);
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/xml");

        request.insert_header("version", self.version.to_string());
        request.insert_header("x-ms-version", self.version.to_string());

        if let Some(metadata) = metadata {
            for (key, value) in metadata {
                let header_name = format!("x-ms-meta-{}", key);
                request.insert_header(header_name.to_string(), value.to_string());
            }
        }

        self.client
            .pipeline
            .send(&ctx, &mut request)
            .await
            .map(Into::into)
    }

    /// Retrieves the metadata of the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to retrieve metadata for.
    /// * `version` - Specifies the version of the operation to use for this request.
    ///
    /// Returns a `Response` containing the metadata if the queue exists, or an error if it does not.
    async fn get_metadata(
        &self,
        queue_name: &str,
    ) -> Result<Response<StorageServicePropertiesResponse, XmlFormat>> {
        let mut url = self.client.endpoint.clone();

        let ctx = Context::new();

        url.path_segments_mut()
            .expect("Invalid URL")
            .push(queue_name);
        url.query_pairs_mut()
            .append_pair("api-version", &self.client.api_version);
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");

        request.insert_header("version", self.version.to_string());
        request.insert_header("x-ms-version", self.version.to_string());

        self.client
            .pipeline
            .send(&ctx, &mut request)
            .await
            .map(Into::into)
    }

    /// Sends a message to the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to which the message will be sent.
    /// * `message` - The message to be sent to the queue.
    /// * `options` - Optional parameters for the enqueue operation.
    pub async fn send_message(
        &self,
        queue_name: &str,
        message: &str,
        options: Option<AzureQueueStorageMessagesOperationsClientEnqueueOptions<'_>>,
    ) -> Result<Response<ListOfEnqueuedMessage, XmlFormat>> {
        let queue_message = QueueMessage {
            message_text: Some(message.to_owned()),
        };

        let xml_body = xml::to_xml(&queue_message)?;
        self.client
            .get_azure_queue_storage_messages_operations_client()
            .enqueue(
                queue_name,
                self.version.clone(),
                RequestContent::try_from(xml_body)?,
                options,
            )
            .await
    }

    /// Deletes a specific message from the queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue from which to delete the message.
    /// * `message_id` - The ID of the message to delete.
    /// * `pop_receipt` - The pop receipt of the message to delete.
    ///
    /// Returns a `Response` indicating the result of the operation.
    pub async fn delete_message(
        &self,
        queue_name: &str,
        message_id: &str,
        pop_receipt: &str,
        options: Option<AzureQueueStorageMessageIdOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_message_id_operations_client()
            .delete(
                queue_name,
                message_id,
                pop_receipt,
                self.version.clone(),
                options,
            )
            .await
    }

    pub async fn receive_message(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageMessagesOperationsClientDequeueOptions<'_>>,
    ) -> Result<Response<ListOfDequeuedMessageItem, XmlFormat>> {
        let options = Some(AzureQueueStorageMessagesOperationsClientDequeueOptions {
            number_of_messages: Some(1),
            ..options.unwrap_or_default()
        });

        self.receive_messages(queue_name, options).await
    }

    pub async fn receive_messages(
        &self,
        queue_name: &str,
        options: Option<AzureQueueStorageMessagesOperationsClientDequeueOptions<'_>>,
    ) -> Result<Response<ListOfDequeuedMessageItem, XmlFormat>> {
        self.client
            .get_azure_queue_storage_messages_operations_client()
            .dequeue(queue_name, self.version.clone(), options)
            .await
    }
}
