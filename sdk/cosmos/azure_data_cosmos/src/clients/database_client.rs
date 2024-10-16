// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ContainerClient,
    models::{ContainerQueryResults, DatabaseProperties},
    pipeline::{CosmosPipeline, ResourceType},
    utils::AppendPathSegments,
    Query, QueryContainersOptions, ReadDatabaseOptions,
};

#[cfg(feature = "control_plane")]
use crate::{
    models::{ContainerProperties, Item},
    CreateContainerOptions, DeleteDatabaseOptions,
};

use azure_core::{Context, Pager, Request, Response};

#[cfg(feature = "control_plane")]
use azure_core::Method;

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
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::{DatabaseClient, DatabaseClientMethods};
    /// # let database_client: DatabaseClient = panic!("this is a non-running example");
    /// let response = database_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<Response<DatabaseProperties>>;

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient;

    /// Returns the identifier of the Cosmos database.
    fn id(&self) -> &str;

    /// Executes a query against containers in the database.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to execute.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// The `query` parameter accepts anything that can be transformed [`Into`] a [`Query`].
    /// This allows simple queries without parameters to be expressed easily:
    ///
    /// ```rust,no_run
    /// # async fn doc() {
    /// # use azure_data_cosmos::clients::{DatabaseClient, DatabaseClientMethods};
    /// # let db_client: DatabaseClient = panic!("this is a non-running example");
    /// let containers = db_client.query_containers(
    ///     "SELECT * FROM dbs",
    ///     None).unwrap();
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions>,
    ) -> azure_core::Result<Pager<ContainerQueryResults>>;

    /// Creates a new container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `properties` - A [`ContainerProperties`] describing the new container.
    /// * `options` - Optional parameters for the request.
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    #[cfg(feature = "control_plane")]
    async fn create_container(
        &self,
        properties: ContainerProperties,
        options: Option<CreateContainerOptions>,
    ) -> azure_core::Result<Response<Item<ContainerProperties>>>;

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[allow(async_fn_in_trait)] // REASON: See https://github.com/Azure/azure-sdk-for-rust/issues/1796 for detailed justification
    #[cfg(feature = "control_plane")]
    async fn delete(&self, options: Option<DeleteDatabaseOptions>) -> azure_core::Result<Response>;
}

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    database_id: String,
    database_url: Url,
    pipeline: CosmosPipeline,
}

impl DatabaseClient {
    pub(crate) fn new(pipeline: CosmosPipeline, base_url: &Url, database_id: &str) -> Self {
        let database_id = database_id.to_string();
        let database_url = base_url.with_path_segments(["dbs", &database_id]);

        Self {
            database_id,
            database_url,
            pipeline,
        }
    }
}

impl DatabaseClientMethods for DatabaseClient {
    async fn read(
        &self,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let mut req = Request::new(self.database_url.clone(), azure_core::Method::Get);
        self.pipeline
            .send(Context::new(), &mut req, ResourceType::Databases)
            .await
    }

    fn container_client(&self, name: impl AsRef<str>) -> ContainerClient {
        ContainerClient::new(self.pipeline.clone(), &self.database_url, name.as_ref())
    }

    fn id(&self) -> &str {
        &self.database_id
    }

    fn query_containers(
        &self,
        query: impl Into<Query>,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<QueryContainersOptions>,
    ) -> azure_core::Result<Pager<ContainerQueryResults>> {
        let mut url = self.database_url.clone();
        url.append_path_segments(["colls"]);
        let base_request = Request::new(url, azure_core::Method::Post);

        self.pipeline
            .send_query_request(query.into(), base_request, ResourceType::Containers)
    }

    #[cfg(feature = "control_plane")]
    async fn create_container(
        &self,
        properties: ContainerProperties,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<CreateContainerOptions>,
    ) -> azure_core::Result<Response<Item<ContainerProperties>>> {
        let url = self.database_url.with_path_segments(["colls"]);
        let mut req = Request::new(url, Method::Post);
        req.set_json(&properties)?;

        self.pipeline
            .send(Context::new(), &mut req, ResourceType::Containers)
            .await
    }

    #[cfg(feature = "control_plane")]
    async fn delete(
        &self,
        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<DeleteDatabaseOptions>,
    ) -> azure_core::Result<Response> {
        let mut req = Request::new(self.database_url.clone(), Method::Delete);
        self.pipeline
            .send(Context::new(), &mut req, ResourceType::Databases)
            .await
    }
}
