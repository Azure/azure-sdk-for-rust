// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::{
    clients::{QueueClient as GeneratedQueueClient, QueueClientOptions},
    models::*,
};
use azure_core::{
    credentials::TokenCredential,
    http::{NoFormat, RawResponse, RequestContent, Response, StatusCode, Url, XmlFormat},
    xml, Bytes, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueClient {
    pub(super) endpoint: Url,
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
        &self.client.queue_name
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
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let client = GeneratedQueueClient::new(
            endpoint,
            credential.clone(),
            queue_name.to_string(),
            Some(options),
        )?;
        Ok(Self {
            endpoint: endpoint.parse()?,
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
        options: Option<QueueClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.create(options).await
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
        options: Option<QueueClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
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
        options: Option<QueueClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete(options).await
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
        options: Option<QueueClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
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
        match self.get_metadata(None).await {
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

    /// Clears all messages in the specified queue.
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
    pub async fn clear(
        &self,
        options: Option<QueueClientClearOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        let result = self.client.clear(options).await?;
        Ok(result)
    }

    /// Sets the metadata for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request, including the metadata to set for the queue
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
        options: Option<QueueClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(options).await
    }

    /// Retrieves the metadata of the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
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
        options: Option<QueueClientGetMetadataOptions<'_>>,
    ) -> Result<Response<QueueClientGetMetadataResult, NoFormat>> {
        self.client.get_metadata(options).await
    }

    /// Enqueues a message to the specified queue.
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
    pub async fn enqueue_message(
        &self,
        message: &str,
        options: Option<QueueClientEnqueueOptions<'_>>,
    ) -> Result<Response<Option<EnqueuedMessage>, XmlFormat>> {
        let queue_message = QueueMessage {
            message_text: Some(message.to_owned()),
        };

        let response = self
            .client
            .enqueue(queue_message.try_into()?, options)
            .await?;

        Self::extract_first_message(response, |list: &ListOfEnqueuedMessage| list.items.clone())
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
        options: Option<QueueClientDeleteMessageOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .delete_message(message_id, pop_receipt, options)
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
        options: Option<QueueClientUpdateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client
            .update(message_id, pop_receipt, visibility_timeout, options)
            .await
    }

    /// The Dequeue operation retrieves a single message from the front of the queue.
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
    pub async fn dequeue_message(
        &self,
        options: Option<QueueClientDequeueOptions<'_>>,
    ) -> Result<Response<Option<DequeuedMessage>, XmlFormat>> {
        let options = Some(QueueClientDequeueOptions {
            number_of_messages: Some(1),
            ..options.unwrap_or_default()
        });

        let response = self.dequeue_messages(options).await?;
        Self::extract_first_message(response, |list: &ListOfDequeuedMessage| list.items.clone())
            .await
    }

    /// The Dequeue operation retrieves one or more messages from the front of the queue.
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
    pub async fn dequeue_messages(
        &self,
        options: Option<QueueClientDequeueOptions<'_>>,
    ) -> Result<Response<ListOfDequeuedMessage, XmlFormat>> {
        self.client.dequeue(options).await
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
        options: Option<QueueClientPeekOptions<'_>>,
    ) -> Result<Response<Option<PeekedMessage>, XmlFormat>> {
        let options = Some(QueueClientPeekOptions {
            number_of_messages: Some(1),
            ..options.unwrap_or_default()
        });

        let response = self.peek_messages(options).await?;
        Self::extract_first_message(response, |list: &ListOfPeekedMessage| list.items.clone()).await
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
        options: Option<QueueClientPeekOptions<'_>>,
    ) -> Result<Response<ListOfPeekedMessage, XmlFormat>> {
        self.client.peek(options).await
    }

    /// Retrieves the access policy for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the access policy response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn get_access_policy(
        &self,
        options: Option<QueueClientGetAccessPolicyOptions<'_>>,
    ) -> Result<Response<ListOfSignedIdentifier, XmlFormat>> {
        self.client.get_access_policy(options).await
    }

    /// Sets the access policy for the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_acl` - The access control list containing signed identifiers for the queue
    /// * `options` - Optional parameters for the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the response if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the queue doesn't exist or if the request fails
    pub async fn set_access_policy(
        &self,
        queue_acl: RequestContent<ListOfSignedIdentifier>,
        options: Option<QueueClientSetAccessPolicyOptions<'_>>,
    ) -> Result<Response<QueueClientSetAccessPolicyResult, NoFormat>> {
        self.client.set_access_policy(queue_acl, options).await
    }

    /// Helper function to extract the first message from a list response and convert it to a single message response
    async fn extract_first_message<T, U>(
        response: Response<T, XmlFormat>,
        extract_fn: impl Fn(&T) -> Option<Vec<U>>,
    ) -> Result<Response<Option<U>, XmlFormat>>
    where
        T: serde::de::DeserializeOwned,
        U: serde::Serialize + Clone,
    {
        let status = response.status();
        let headers = response.headers().clone();
        let message_list = response.into_body().await?;

        if let Some(messages) = extract_fn(&message_list) {
            if let Some(first_message) = messages.into_iter().next() {
                let xml_body = xml::to_xml(&first_message)?;
                let raw_response = RawResponse::from_bytes(status, headers, xml_body);
                Ok(raw_response.into())
            } else {
                let raw_response =
                    RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));
                Ok(raw_response.into())
            }
        } else {
            let raw_response = RawResponse::from_bytes(status, headers, Bytes::from_static(&[]));
            Ok(raw_response.into())
        }
    }
}
