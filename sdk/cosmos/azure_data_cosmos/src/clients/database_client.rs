// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::DatabaseProperties;
use crate::options::ReadDatabaseOptions;
use crate::pipeline::ResourceType;
use crate::utils::AppendPathSegments;
use crate::{clients::ContainerClient, pipeline::CosmosPipeline};

use azure_core::{Context, Request};
use url::Url;

#[cfg(doc)]
use crate::CosmosClientMethods;

/// Defines the methods provided by a [`DatabaseClient`]
///
/// This trait is intended to allow you to mock out the `DatabaseClient` when testing your application.
/// Rather than depending on `DatabaseClient`, you can depend on a generic parameter constrained by this trait, or an `impl DatabaseClientMethods` type.
pub trait DatabaseClientMethods {
    /// Reads the properties of the database.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// # async fn doc() {
    /// use azure_data_cosmos::{CosmosClient, CosmosClientMethods, clients::DatabaseClientMethods};
    ///
    /// let credential = Arc::new(azure_identity::DefaultAzureCredential::new().unwrap());
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// let db_client = client.database_client("my_database");
    /// let response = db_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>>;

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient;
}

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](CosmosClient::database_client()).
pub struct DatabaseClient {
    database_url: Url,
    pipeline: CosmosPipeline,
}

impl DatabaseClient {
    pub(crate) fn new(pipeline: CosmosPipeline, base_url: &Url, database_id: &str) -> Self {
        let mut database_url = base_url.clone();
        database_url.append_path_segments(["dbs", database_id]);

        Self {
            database_url,
            pipeline,
        }
    }
}

impl DatabaseClientMethods for DatabaseClient {
    async fn read(
        &self,

        #[allow(unused_variables)]
        // This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<azure_core::Response<DatabaseProperties>> {
        let mut req = Request::new(self.database_url.clone(), azure_core::Method::Get);
        self.pipeline
            .send(Context::new(), &mut req, ResourceType::Databases)
            .await
    }

    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient {
        ContainerClient::new(self.pipeline.clone(), &self.database_url, name.as_ref())
    }
}
