// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{QueueClient, QueueClientOptions};

use crate::{generated::models::*, logging::apply_storage_logging_defaults};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pipeline, Response, StatusCode, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl QueueClient {
    /// Creates a new QueueClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `queue_name` - The name of the queue to interact with.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    ///   If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
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
        Self::from_url(url, credential, options)
    }

    /// Creates a new QueueClient from a queue URL.
    ///
    /// # Arguments
    ///
    /// * `queue_url` - The full URL of the queue, for example `https://myaccount.queue.core.windows.net/myqueue`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    ///   If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Queue")]
    pub fn from_url(
        queue_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        if let Some(token_credential) = credential {
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
            options.client_options.per_try_policies.push(auth_policy);
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            Vec::default(),
            None,
        );

        Ok(Self {
            endpoint: queue_url,
            version: options.version,
            pipeline,
        })
    }

    /// Checks if the queue exists.
    ///
    /// Returns `true` if the queue exists, `false` if the queue does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_metadata(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Updates the visibility timeout and optionally the content of a queued message.
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
        self.update(message_id, pop_receipt, visibility_timeout, options)
            .await
    }
}
