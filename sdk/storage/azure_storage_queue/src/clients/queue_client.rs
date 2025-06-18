// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::{
    clients::{AzureQueueStorageClient as GeneratedQueueClient, AzureQueueStorageClientOptions},
    models::{
        AzureQueueStorageMessageIdOperationsClientDeleteOptions,
        AzureQueueStorageMessageIdOperationsClientUpdateOptions,
        AzureQueueStorageMessagesOperationsClientClearOptions,
        AzureQueueStorageMessagesOperationsClientDequeueOptions,
        AzureQueueStorageMessagesOperationsClientEnqueueOptions,
        AzureQueueStorageMessagesOperationsClientPeekOptions,
        AzureQueueStorageQueueOperationsClientCreateOptions,
        AzureQueueStorageQueueOperationsClientDeleteOptions,
        AzureQueueStorageServiceOperationsClientGetPropertiesOptions, DequeuedMessageItem,
        ListOfDequeuedMessageItem, ListOfEnqueuedMessage, ListOfPeekedMessageItem,
        PeekedMessageItem, QueueMessage, ServicePropertiesCompType,
        StorageServicePropertiesResponse,
    },
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        Context, Method, RawResponse, Request, RequestContent, Response, StatusCode, Url,
        XmlFormat,
    },
    xml, Bytes, Result,
};
use std::{collections::HashMap, sync::Arc};

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueClient {
    pub(super) endpoint: Url,
    pub(super) queue_name: String,
    pub(super) client: GeneratedQueueClient,
}

impl QueueClient {
    /// Returns the endpoint URL of the Azure storage account this client is associated with.
    ///
    /// # Returns
    ///
    /// A reference to the URL of the storage account.
    pub fn endpoint(&self) -> &Url {
        self.client.endpoint()
    }

    /// Returns the name of the queue this client is associated with.
    ///
    /// # Returns
    ///
    /// A reference to the name of the queue.
    pub fn queue_name(&self) -> &str {
        &self.queue_name
    }

    /// Creates a new QueueClient using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://<storage_account_name>.queue.core.windows.net/`
    /// * `queue_name` - The name of the queue to interact with
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token for authentication
    /// * `options` - Optional configuration for the client
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `QueueClient` if successful, or an error if the endpoint URL is invalid
    pub fn new(
        endpoint: &str,
        queue_name: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<AzureQueueStorageClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let client = GeneratedQueueClient::new(endpoint, credential.clone(), Some(options))?;
        Ok(Self {
            endpoint: endpoint.parse()?,
            queue_name: queue_name.to_string(),
            client,
        })
    }

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue already exists or if the request fails
    pub async fn create(
        &self,
        options: Option<AzureQueueStorageQueueOperationsClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_queue_operations_client()
            .create(&self.queue_name, options)
            .await
    }

    /// Creates a new queue under the given account if it doesn't already exist.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response. If the queue already exists,
    /// returns a success response with no content (204 No Content)
    pub async fn create_if_not_exists(
        &self,
        options: Option<AzureQueueStorageQueueOperationsClientCreateOptions<'_>>,
    ) -> Result<Response<()>> {
        // Attempt to create the queue, if it already exists, this will return an error.
        match self.create(options).await {
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

    /// Permanently deletes the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn delete(
        &self,
        options: Option<AzureQueueStorageQueueOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_queue_operations_client()
            .delete(&self.queue_name, options)
            .await
    }

    /// Deletes the specified queue if it exists.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response. If the queue doesn't exist,
    /// returns a success response with no content (204 No Content)
    pub async fn delete_if_exists(
        &self,
        options: Option<AzureQueueStorageQueueOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        // Attempt to delete the queue, if it does not exist, this will return an error.
        match self.delete(options).await {
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
    // TODO: Validate that this is correctly implemented. This returns properties for the entire service, not just a single queue.
    pub async fn get_properties(
        &self,
        options: Option<AzureQueueStorageServiceOperationsClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServicePropertiesResponse, XmlFormat>> {
        self.client
            .get_azure_queue_storage_service_operations_client()
            .get_properties(
                crate::generated::models::ServiceRestypeType::Service,
                crate::generated::models::ServicePropertiesCompType::Properties,
                options,
            )
            .await
    }

    /// Checks if the queue exists.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(true)` if the queue exists
    /// - `Ok(false)` if the queue does not exist
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails for any reason other than a non-existent queue
    pub async fn exists(&self) -> Result<bool> {
        match self.get_metadata().await {
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
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn delete_messages(
        &self,
        options: Option<AzureQueueStorageMessagesOperationsClientClearOptions<'_>>,
    ) -> Result<Response<()>> {
        let messages_client = self
            .client
            .get_azure_queue_storage_messages_operations_client();

        let result = messages_client.clear(&self.queue_name, options).await?;
        Ok(result)
    }

    /// Sets the metadata for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `metadata` - A map of metadata key-value pairs to set for the queue. If `None`, all metadata will be removed
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn set_metadata(
        &self,
        metadata: Option<HashMap<&str, &str>>,
    ) -> Result<Response<()>> {
        let mut url = self.client.endpoint.clone();
        let ctx = Context::new();

        url.path_segments_mut()
            .expect("Invalid URL")
            .push(&self.queue_name);
        url.query_pairs_mut()
            .append_pair("api-version", &self.client.api_version);
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut request = Request::new(url, Method::Put);
        request.insert_header("accept", "application/xml");

        request.insert_header("x-ms-version", self.client.api_version.to_string());

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
    /// # Returns
    ///
    /// Returns a `Result` containing the queue's metadata if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn get_metadata(
        &self,
    ) -> Result<Response<StorageServicePropertiesResponse, XmlFormat>> {
        let mut url = self.client.endpoint.clone();

        let ctx = Context::new();

        url.path_segments_mut()
            .expect("Invalid URL")
            .push(&self.queue_name);
        url.query_pairs_mut()
            .append_pair("api-version", &self.client.api_version);
        url.query_pairs_mut().append_pair("comp", "metadata");

        let mut request = Request::new(url, Method::Get);
        request.insert_header("accept", "application/xml");

        request.insert_header("x-ms-version", self.client.api_version.to_string());

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
    /// * `message` - The message text to be added to the queue
    /// * `options` - Optional parameters for the enqueue operation, including visibility timeout and message time-to-live
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the enqueued message details if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn send_message(
        &self,
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
                &self.queue_name,
                RequestContent::try_from(xml_body)?,
                options,
            )
            .await
    }

    /// Deletes a specific message from the queue.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message to delete
    /// * `pop_receipt` - The pop receipt obtained when the message was retrieved
    /// * `options` - Optional parameters for the delete operation
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist, the pop receipt is invalid,
    /// or if the request fails
    pub async fn delete_message(
        &self,
        message_id: &str,
        pop_receipt: &str,
        options: Option<AzureQueueStorageMessageIdOperationsClientDeleteOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_message_id_operations_client()
            .delete(&self.queue_name, message_id, pop_receipt, options)
            .await
    }

    /// Updates a specific message in the queue.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message to update
    /// * `pop_receipt` - The pop receipt obtained when the message was retrieved
    /// * `visibility_timeout` - The new visibility timeout for the message, in seconds
    /// * `options` - Optional parameters for the update operation
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist, the pop receipt is invalid,
    /// or if the request fails
    pub async fn update_message(
        &self,
        message_id: &str,
        pop_receipt: &str,
        visibility_timeout: i32,
        options: Option<AzureQueueStorageMessageIdOperationsClientUpdateOptions<'_>>,
    ) -> Result<Response<()>> {
        self.client
            .get_azure_queue_storage_message_id_operations_client()
            .update(
                &self.queue_name,
                message_id,
                pop_receipt,
                visibility_timeout,
                options,
            )
            .await
    }

    /// Retrieves a single message from the queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the dequeue operation. If `number_of_messages` is specified in the options,
    ///   it will be overridden to 1. Use this to set the visibility timeout for the message
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the dequeued message if successful. The message will be invisible
    /// to other consumers for the duration specified in the visibility timeout
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn receive_message(
        &self,
        options: Option<AzureQueueStorageMessagesOperationsClientDequeueOptions<'_>>,
    ) -> Result<Response<Option<DequeuedMessageItem>, XmlFormat>> {
        let options = Some(AzureQueueStorageMessagesOperationsClientDequeueOptions {
            number_of_messages: Some(1),
            ..options.unwrap_or_default()
        });

        match self.receive_messages(options).await {
            Ok(response) => {
                // Extract the first message from the list of dequeued messages.
                // If the list is empty, this will return None.

                let status = response.status();
                let headers = response.headers().clone();
                let messages = response.into_body().await?;
                if let Some(messages) = messages.value {
                    if let Some(first_message) = messages.into_iter().next() {
                        // Serialize the first message to XML format.
                        // Construct a response using the serialized message as the body and the status code from the original response.
                        let xml_body = xml::to_xml(&first_message)?;
                        let raw_response =
                            RawResponse::from_bytes(status, headers.clone(), xml_body);

                        Ok(raw_response.into())
                    } else {
                        let raw_response =
                            RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));

                        Ok(raw_response.into())
                    }
                } else {
                    let raw_response =
                        RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));

                    Ok(raw_response.into())
                }
            }
            Err(e) => {
                // Propagate other errors.
                Err(e)
            }
        }
    }

    /// Retrieves multiple messages from the queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the dequeue operation. Use `number_of_messages` to specify
    ///   how many messages to retrieve (up to 32) and set the visibility timeout
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the dequeued messages if successful. The messages will be invisible
    /// to other consumers for the duration specified in the visibility timeout
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn receive_messages(
        &self,
        options: Option<AzureQueueStorageMessagesOperationsClientDequeueOptions<'_>>,
    ) -> Result<Response<ListOfDequeuedMessageItem, XmlFormat>> {
        self.client
            .get_azure_queue_storage_messages_operations_client()
            .dequeue(&self.queue_name, options)
            .await
    }

    /// Peeks a single message from the front of the queue without removing it.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the peek operation. If `number_of_messages` is specified,
    ///   it will be overridden to 1
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the message at the front of the queue if successful.
    /// The message remains visible to other consumers
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn peek_message(
        &self,
        options: Option<AzureQueueStorageMessagesOperationsClientPeekOptions<'_>>,
    ) -> Result<Response<Option<PeekedMessageItem>, XmlFormat>> {
        let options = Some(AzureQueueStorageMessagesOperationsClientPeekOptions {
            number_of_messages: Some(1),
            ..options.unwrap_or_default()
        });

        match self.peek_messages(options).await {
            Ok(response) => {
                // Extract the first message from the list of dequeued messages.
                // If the list is empty, this will return None.

                let status = response.status();
                let headers = response.headers().clone();
                let messages = response.into_body().await?;
                if let Some(messages) = messages.value {
                    if let Some(first_message) = messages.into_iter().next() {
                        // Serialize the first message to XML format.
                        // Construct a response using the serialized message as the body and the status code from the original response.
                        let xml_body = xml::to_xml(&first_message)?;
                        let raw_response =
                            RawResponse::from_bytes(status, headers.clone(), xml_body);

                        Ok(raw_response.into())
                    } else {
                        let raw_response =
                            RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));

                        Ok(raw_response.into())
                    }
                } else {
                    let raw_response =
                        RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));

                    Ok(raw_response.into())
                }
            }
            Err(e) => {
                // Propagate other errors.
                Err(e)
            }
        }
    }

    /// Peeks multiple messages from the front of the queue without removing them.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the peek operation. Use `number_of_messages`
    ///   to specify how many messages to peek (up to 32)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the messages at the front of the queue if successful.
    /// The messages remain visible to other consumers
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn peek_messages(
        &self,
        options: Option<AzureQueueStorageMessagesOperationsClientPeekOptions<'_>>,
    ) -> Result<Response<ListOfPeekedMessageItem, XmlFormat>> {
        self.client
            .get_azure_queue_storage_messages_operations_client()
            .peek(&self.queue_name, options)
            .await
    }
}
