use crate::operations::ListTablesBuilder;
use azure_core::{
    headers::Headers, Body, ClientOptions, Context, Method, Pipeline, Request, Response,
};
use azure_storage::{clients::ServiceType, prelude::StorageCredentials, CloudLocation};
use url::Url;

use super::TableClient;

/// A builder for the table service client.
#[derive(Debug, Clone)]
pub struct TableServiceClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
    credentials: StorageCredentials,
}

impl TableServiceClientBuilder {
    /// Create a new instance of `TableServiceClientBuilder`.
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

    /// Create a new instance of `BlobServiceClientBuilder` with a cloud location.
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
                port: 10002,
            },
            StorageCredentials::emulator(),
        )
    }

    /// Convert the builder into a `TableServiceClient` instance.
    #[must_use]
    pub fn build(self) -> TableServiceClient {
        let Self {
            cloud_location,
            options,
            credentials,
        } = self;
        TableServiceClient {
            pipeline: azure_storage::clients::new_pipeline_from_options(options, credentials),
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
pub struct TableServiceClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl TableServiceClient {
    pub fn new(account: impl Into<String>, credentials: impl Into<StorageCredentials>) -> Self {
        TableServiceClientBuilder::new(account, credentials).build()
    }

    pub fn list(&self) -> ListTablesBuilder {
        ListTablesBuilder::new(self.clone())
    }

    pub(crate) fn url(&self) -> azure_core::Result<Url> {
        let mut url = self.cloud_location.url(ServiceType::Table)?;
        url.path_segments_mut().unwrap().push("Tables");
        Ok(url)
    }

    pub fn table_client<S: Into<String>>(&self, table_name: S) -> TableClient {
        TableClient::new(self.clone(), table_name)
    }

    pub(crate) fn finalize_request(
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        azure_storage::clients::finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.pipeline
            .send(context.insert(ServiceType::Table), request)
            .await
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use futures::StreamExt;

    fn get_emulator_client() -> TableServiceClient {
        TableServiceClientBuilder::emulator()
            .retry(azure_core::RetryOptions::none())
            .build()
    }

    #[tokio::test]
    async fn test_list() {
        let service_client = get_emulator_client();

        println!("Create a table in the storage account");
        let table_client = service_client.table_client("TableServiceClientList");
        let _ = table_client.create().await;

        println!("Check that the table is listed correctly");
        let mut stream = service_client.list().into_stream();
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");
            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableServiceClientList");
            assert!(has_table, "the table should be present in the tables list");
        }
    }
}
