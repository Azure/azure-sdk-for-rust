// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::QueueClient,
    generated::{
        clients::{QueueServiceClient as GeneratedQueueServiceClient, QueueServiceClientOptions},
        models::*,
    },
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pager, Pipeline, RequestContent, Response, Url, XmlFormat,
    },
    tracing, Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueServiceClient {
    pub(super) client: GeneratedQueueServiceClient,
    pub(super) credential: Option<Arc<dyn TokenCredential>>,
    pub(super) options: Option<QueueServiceClientOptions>,
}

impl GeneratedQueueServiceClient {
    /// Creates a new GeneratedQueueServiceClient from a service URL.
    ///
    /// # Arguments
    ///
    /// * `service_url` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token for authentication. If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Service")]
    pub fn from_url(
        service_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();

        let per_retry_policies = if let Some(token_credential) = credential.clone() {
            if !service_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{service_url} must use https"),
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
            endpoint: service_url,
            version: options.version,
            pipeline,
        })
    }
}

impl QueueServiceClient {
    /// Creates a new QueueServiceClient using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token for authentication. If None, the URL must contain authentication information (e.g., SAS token).
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let url = Url::parse(endpoint)?;
        let client =
            GeneratedQueueServiceClient::from_url(url, credential.clone(), options.clone())?;
        Ok(Self {
            client,
            credential,
            options,
        })
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
    pub fn queue_client(&self, queue_name: String) -> Result<QueueClient> {
        use crate::clients::QueueClientOptions;
        let queue_options = self.options.as_ref().map(|opts| QueueClientOptions {
            client_options: opts.client_options.clone(),
            ..Default::default()
        });
        QueueClient::new(
            self.endpoint().as_str(),
            &queue_name,
            self.credential.clone(),
            queue_options,
        )
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
        self.queue_client(queue_name.to_string())?
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
        self.queue_client(queue_name.to_string())?
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
    ) -> Result<Pager<ListQueuesResponse, XmlFormat>> {
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
