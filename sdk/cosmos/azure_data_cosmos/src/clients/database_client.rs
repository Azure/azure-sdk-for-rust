// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ContainerClient,
    models::{ContainerProperties, ContainerQueryResults, DatabaseProperties, Item},
    options::ReadDatabaseOptions,
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, Query, QueryContainersOptions,
};

use azure_core::{Context, Method, Pager, Request, Response};
use url::Url;

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    endpoint: Url,
    link: ResourceLink,
    database_id: String,
    pipeline: CosmosPipeline,
}

impl DatabaseClient {
    pub(crate) fn new(pipeline: CosmosPipeline, endpoint: Url, database_id: String) -> Self {
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);

        Self {
            endpoint,
            link,
            database_id,
            pipeline,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    pub fn container_client(&self, name: impl AsRef<str>) -> ContainerClient {
        ContainerClient::new(self.pipeline.clone(), &self.database_url, name.as_ref())
    }

    /// Returns the identifier of the Cosmos database.
    pub fn id(&self) -> &str {
        &self.database_id
    }

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
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let database_client: DatabaseClient = panic!("this is a non-running example");
    /// let response = database_client.read(None)
    ///     .await.unwrap()
    ///     .deserialize_body()
    ///     .await.unwrap();
    /// # }
    /// ```
    pub async fn read(
        &self,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let mut req = Request::new(self.link.url(&self.endpoint), Method::Get);
        self.pipeline
            .send(Context::new(), &mut req, self.link.clone())
            .await
    }

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
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let db_client: DatabaseClient = panic!("this is a non-running example");
    /// let containers = db_client.query_containers(
    ///     "SELECT * FROM dbs",
    ///     None).unwrap();
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub fn query_containers(
        &self,
        query: impl Into<Query>,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<QueryContainersOptions>,
    ) -> azure_core::Result<Pager<ContainerQueryResults>> {
        let link = self.link.feed(ResourceType::Containers);
        let base_request = Request::new(link.url(&self.endpoint), Method::Post);

        self.pipeline
            .send_query_request(query.into(), base_request, link)
    }

    /// Creates a new container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `properties` - A [`ContainerProperties`] describing the new container.
    /// * `options` - Optional parameters for the request.
    pub async fn create_container(
        &self,
        properties: ContainerProperties,

        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<CreateContainerOptions>,
    ) -> azure_core::Result<Response<Item<ContainerProperties>>> {
        let link = self.link.feed(ResourceType::Containers);
        let mut req = Request::new(link.url(&self.endpoint), Method::Post);
        req.set_json(&properties)?;

        self.pipeline.send(Context::new(), &mut req, link).await
    }

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        #[allow(unused_variables)]
        // REASON: This is a documented public API so prefixing with '_' is undesirable.
        options: Option<DeleteDatabaseOptions>,
    ) -> azure_core::Result<Response> {
        let link = self.link.feed(ResourceType::Containers);
        let mut req = Request::new(link.url(&self.endpoint), Method::Delete);
        self.pipeline.send(Context::new(), &mut req, link).await
    }
}
