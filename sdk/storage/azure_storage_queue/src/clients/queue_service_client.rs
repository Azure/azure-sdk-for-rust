// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{QueueServiceClient, QueueServiceClientOptions};

use crate::{
    clients::QueueClient,
    generated::models::{QueueClientCreateOptions, QueueClientDeleteOptions},
    logging::apply_storage_logging_defaults,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pipeline, Response, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl QueueServiceClient {
    /// Creates a new QueueServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token for authentication.
    ///   If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Service")]
    pub fn new(
        endpoint: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint)?;
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        if let Some(token_credential) = credential {
            if !endpoint.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{endpoint} must use https"),
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
            endpoint,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of QueueClient.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue.
    pub fn queue_client(&self, queue_name: &str) -> Result<QueueClient> {
        let mut queue_url = self.endpoint().clone();
        queue_url
            .path_segments_mut()
            .map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?
            .push(queue_name);
        Ok(QueueClient {
            endpoint: queue_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }

    /// Creates a new queue under the given account.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to create.
    /// * `options` - Optional configuration for the request.
    pub async fn create_queue(
        &self,
        queue_name: &str,
        options: Option<QueueClientCreateOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.queue_client(queue_name)?.create(options).await
    }

    /// Permanently deletes the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to delete.
    /// * `options` - Optional configuration for the request.
    pub async fn delete_queue(
        &self,
        queue_name: &str,
        options: Option<QueueClientDeleteOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        self.queue_client(queue_name)?.delete(options).await
    }
}
