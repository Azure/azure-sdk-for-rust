// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::{
    clients::{QueueClient as GeneratedQueueClient, QueueClientOptions},
    models::*,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pipeline, RawResponse, RequestContent, Response, StatusCode, Url, XmlFormat,
    },
    tracing, xml, Result,
};
use std::{collections::HashMap, sync::Arc};

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueClient {
    pub(super) client: GeneratedQueueClient,
}

impl GeneratedQueueClient {
    /// Creates a new GeneratedQueueClient from a queue URL.
    ///
    /// # Arguments
    ///
    /// * `queue_url` - The full URL of the queue, for example `https://myaccount.queue.core.windows.net/myqueue`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating. If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Queue")]
    pub fn from_url(
        queue_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let per_retry_policies = if let Some(token_credential) = credential {
            if !queue_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{queue_url} must use https"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
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
            endpoint: queue_url,
            version: options.version,
            pipeline,
        })
    }
}

impl QueueClient {
    /// Creates a new QueueClient from a queue URL.
    ///
    /// # Arguments
    ///
    /// * `queue_url` - The full URL of the queue, for example `https://myaccount.queue.core.windows.net/myqueue`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating. If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `QueueClient` if successful, or an error if the endpoint URL is invalid
    pub fn from_url(
        queue_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        let client = GeneratedQueueClient::from_url(queue_url, credential, options)?;
        Ok(Self { client })
    }

    /// Creates a new QueueClient for a queue within an Azure Storage account.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `queue_name` - The name of the queue to interact with
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating. If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `QueueClient` if successful, or an error if the endpoint URL is invalid
    pub fn new(
        endpoint: &str,
        queue_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;
        url.path_segments_mut()
            .map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?
            .push(queue_name);

        let client = GeneratedQueueClient::from_url(url, credential, options)?;
        Ok(Self { client })
    }

    /// Returns the endpoint URL of the Azure storage account this client is associated with.
    pub fn endpoint(&self) -> &Url {
        self.client.endpoint()
    }

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn create(
        &self,
        options: Option<QueueClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.create(options).await
    }

    /// Permanently deletes the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
    pub async fn delete(
        &self,
        options: Option<QueueClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.delete(options).await
    }

    /// Checks if the queue exists.
    ///
    /// Returns `true` if the queue exists, `false` if the queue does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_metadata(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
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
    /// * `options` - Optional configuration for the request.
    pub async fn clear(
        &self,
        options: Option<QueueClientClearOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.clear(options).await
    }

    /// Sets user-defined metadata for the specified queue as one or more name-value pairs. Each call to this operation
    /// replaces all existing metadata attached to the queue. To remove all metadata from the queue, call this operation with
    /// no metadata headers.
    ///
    /// # Arguments
    ///
    /// * `metadata` - A [`HashMap`] containing the metadata key-value pairs to set for the queue.
    /// * `options` - Optional configuration for the request.
    pub async fn set_metadata(
        &self,
        metadata: HashMap<String, String>,
        options: Option<QueueClientSetMetadataOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.client.set_metadata(&metadata, options).await
    }

    /// Retrieves the metadata of the specified queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request.
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
    /// * `message` - The message text to be added to the queue.
    /// * `options` - Optional configuration for the request.
    pub async fn send_message(
        &self,
        queue_message: RequestContent<QueueMessage, XmlFormat>,
        options: Option<QueueClientSendMessageOptions<'_>>,
    ) -> Result<Response<SentMessage, XmlFormat>> {
        let response = self.client.send_message(queue_message, options).await?;

        Self::extract_first_message(response, |list: &ListOfSentMessage| {
            list.items.clone().unwrap_or_default()
        })
        .await
    }

    /// Deletes the specified message from the queue.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message to delete.
    /// * `pop_receipt` - The pop receipt obtained when the message was retrieved.
    /// * `options` - Optional configuration for the request.
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

    /// Updates the specified message in the queue.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message to update.
    /// * `pop_receipt` - The pop receipt obtained when the message was retrieved.
    /// * `visibility_timeout` - The new visibility timeout for the message, in seconds.
    /// * `options` - Optional configuration for the request.
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

    /// Retrieves one or more messages from the front of the queue.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request. Use `number_of_messages` to specify
    ///   how many messages to retrieve (up to 32) and set the visibility timeout.
    pub async fn receive_messages(
        &self,
        options: Option<QueueClientReceiveMessagesOptions<'_>>,
    ) -> Result<Response<ListOfReceivedMessage, XmlFormat>> {
        self.client.receive_messages(options).await
    }

    /// Peeks multiple messages from the front of the queue without removing them.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for the request. Use `number_of_messages`
    ///   to specify how many messages to peek (up to 32).
    pub async fn peek_messages(
        &self,
        options: Option<QueueClientPeekMessagesOptions<'_>>,
    ) -> Result<Response<ListOfPeekedMessage, XmlFormat>> {
        self.client.peek_messages(options).await
    }

    /// Helper function to extract the first message from a list response and convert it to a single message response.
    async fn extract_first_message<T, U>(
        response: Response<T, XmlFormat>,
        extract_fn: impl Fn(&T) -> Vec<U>,
    ) -> Result<Response<U, XmlFormat>>
    where
        T: serde::de::DeserializeOwned,
        U: serde::Serialize + Clone,
    {
        let status = response.status();
        let headers = response.headers().clone();
        let message_list = response.into_model()?;

        let messages = extract_fn(&message_list);
        let first_message = messages.into_iter().next().ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "No messages found in the response.",
            )
        })?;

        let xml_body = xml::to_xml(&first_message)?;
        let raw_response = RawResponse::from_bytes(status, headers, xml_body);
        Ok(raw_response.into())
    }
}
