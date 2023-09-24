use crate::clients::FileSystemClient;
use crate::operations::ListFileSystemsBuilder;
use azure_core::{ClientOptions, Pipeline};
use azure_storage::clients::{new_pipeline_from_options, ServiceType};
use azure_storage::prelude::StorageCredentials;
use azure_storage::CloudLocation;

/// A builder for the blob service client.
#[derive(Debug, Clone)]
pub struct DataLakeClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
    credentials: StorageCredentials,
}

impl DataLakeClientBuilder {
    /// Create a new instance of `DataLakeClientBuilder`.
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

    /// Create a new instance of `DataLakeClientBuilder` with a cloud location.
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

    /// Convert the builder into a `DataLakeClient` instance.
    #[must_use]
    pub fn build(self) -> DataLakeClient {
        let Self {
            credentials,
            cloud_location,
            options,
        } = self;
        DataLakeClient {
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
pub struct DataLakeClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl DataLakeClient {
    pub fn new(account: impl Into<String>, credentials: StorageCredentials) -> Self {
        DataLakeClientBuilder::new(account, credentials).build()
    }

    /// Create a new `DataLakeClientBuilder`.
    #[must_use]
    pub fn builder(
        account: impl Into<String>,
        credentials: StorageCredentials,
    ) -> DataLakeClientBuilder {
        DataLakeClientBuilder::new(account, credentials)
    }

    pub(crate) fn url(&self) -> azure_core::Result<url::Url> {
        self.cloud_location.url(ServiceType::DataLake)
    }

    pub fn list_file_systems(&self) -> ListFileSystemsBuilder {
        ListFileSystemsBuilder::new(self.clone())
    }

    pub fn file_system_client<FS>(&self, file_system_name: FS) -> FileSystemClient
    where
        FS: Into<String>,
    {
        FileSystemClient::new(self.clone(), file_system_name.into())
    }

    pub(crate) async fn send(
        &self,
        ctx: &mut azure_core::Context,
        request: &mut azure_core::Request,
    ) -> azure_core::Result<azure_core::Response> {
        // This is a bit of a hack:
        // We deconstruct the passed in request in order to finalize it.
        // We then set the new request to the old request so that callers observe any changes.
        let mut r = azure_storage::clients::finalize_request(
            request.url().clone(),
            *request.method(),
            request.headers().clone(),
            Some(request.body().clone()),
        )?;
        let result = self
            .pipeline
            .send(ctx.insert(ServiceType::DataLake), &mut r)
            .await;

        *request = r;
        result
    }
}
