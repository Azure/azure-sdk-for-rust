use crate::{operations::*, QueueClient, QueueServiceProperties};
use azure_core::{ClientOptions, Context, Pipeline, Request, Response};
use azure_storage::{
    clients::{new_pipeline_from_options, ServiceType},
    prelude::StorageCredentials,
    CloudLocation,
};
use std::fmt::Debug;

/// A builder for the queue service client.
#[derive(Debug, Clone)]
pub struct QueueServiceClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
    credentials: StorageCredentials,
}

impl QueueServiceClientBuilder {
    /// Create a new instance of `QueueServiceClientBuilder`.
    #[must_use]
    pub fn new<A, C>(account: A, credentials: C) -> Self
    where
        A: Into<String>,
        C: Into<StorageCredentials>,
    {
        Self::with_location(
            CloudLocation::Public {
                account: account.into(),
            },
            credentials,
        )
    }

    /// Create a new instance of `QueueServiceClientBuilder` with a cloud location.
    #[must_use]
    pub fn with_location<C>(cloud_location: CloudLocation, credentials: C) -> Self
    where
        C: Into<StorageCredentials>,
    {
        Self {
            options: ClientOptions::default(),
            cloud_location,
            credentials: credentials.into(),
        }
    }

    /// Use the emulator with default settings
    #[must_use]
    pub fn emulator() -> Self {
        Self::with_location(
            CloudLocation::Emulator {
                address: "127.0.0.1".to_owned(),
                port: 10001,
            },
            StorageCredentials::emulator(),
        )
    }

    /// Convert the builder into a `QueueServiceClient` instance.
    #[must_use]
    pub fn build(self) -> QueueServiceClient {
        let Self {
            cloud_location,
            options,
            credentials,
        } = self;

        QueueServiceClient {
            pipeline: new_pipeline_from_options(options, credentials),
            cloud_location,
        }
    }

    /// Set the cloud location.
    #[must_use]
    pub fn cloud_location(mut self, cloud_location: CloudLocation) -> Self {
        self.cloud_location = cloud_location;
        self
    }

    /// Set the retry options.
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }

    /// Set the transport options.
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }

    /// Override all of the client options.
    ///
    /// *Warning!*: This overrides all client options that have been previously set on this builder.
    #[must_use]
    pub fn client_options(mut self, options: impl Into<azure_core::ClientOptions>) -> Self {
        self.options = options.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct QueueServiceClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl QueueServiceClient {
    /// Create a new `QueueServiceClient` which connects to the account's instance in the public Azure cloud.
    #[must_use]
    pub fn new(account: impl Into<String>, credentials: impl Into<StorageCredentials>) -> Self {
        QueueServiceClientBuilder::new(account, credentials).build()
    }

    pub fn list_queues(&self) -> ListQueuesBuilder {
        ListQueuesBuilder::new(self.clone())
    }

    pub fn get_queue_service_properties(&self) -> GetQueueServicePropertiesBuilder {
        GetQueueServicePropertiesBuilder::new(self.clone())
    }

    /// Set queue service properties.
    ///
    /// More info here:
    /// <https://docs.microsoft.com/rest/api/storageservices/set-queue-service-properties>
    pub fn set_queue_service_properties(
        &self,
        properties: QueueServiceProperties,
    ) -> SetQueueServicePropertiesBuilder {
        SetQueueServicePropertiesBuilder::new(self.clone(), properties)
    }

    pub fn get_queue_service_stats(&self) -> GetQueueServiceStatsBuilder {
        GetQueueServiceStatsBuilder::new(self.clone())
    }

    pub fn queue_client<S: Into<String>>(&self, queue_name: S) -> QueueClient {
        QueueClient::new(self.clone(), queue_name.into())
    }

    pub fn url(&self) -> azure_core::Result<url::Url> {
        self.cloud_location.url(ServiceType::Queue)
    }

    pub(crate) fn finalize_request(
        url: url::Url,
        method: azure_core::Method,
        headers: azure_core::headers::Headers,
        request_body: Option<azure_core::Body>,
    ) -> azure_core::Result<Request> {
        azure_storage::clients::finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.pipeline
            .send(context.insert(ServiceType::Queue), request)
            .await
    }
}
