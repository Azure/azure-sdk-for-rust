// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ContainerClient,
    models::{
        ContainerProperties, ContainerQueryResults, DatabaseProperties, ThroughputProperties,
    },
    options::ReadDatabaseOptions,
    pipeline::CosmosPipeline,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, Query, QueryContainersOptions,
    ThroughputOptions,
};

use azure_core::{Method, Pager, Request, Response};

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    link: ResourceLink,
    containers_link: ResourceLink,
    database_id: String,
    pipeline: CosmosPipeline,
}

impl DatabaseClient {
    pub(crate) fn new(pipeline: CosmosPipeline, database_id: &str) -> Self {
        let database_id = database_id.to_string();
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);
        let containers_link = link.feed(ResourceType::Containers);

        Self {
            link,
            containers_link,
            database_id,
            pipeline,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    pub fn container_client(&self, name: &str) -> ContainerClient {
        ContainerClient::new(self.pipeline.clone(), &self.link, name)
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
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let database_client: DatabaseClient = panic!("this is a non-running example");
    /// let response = database_client.read(None)
    ///     .await?
    ///     .into_body()
    ///     .await?;
    /// # }
    /// ```
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::GET);
        self.pipeline
            .send(options.method_options.context, &mut req, self.link.clone())
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
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// # use azure_data_cosmos::clients::DatabaseClient;
    /// # let db_client: DatabaseClient = panic!("this is a non-running example");
    /// let containers = db_client.query_containers(
    ///     "SELECT * FROM dbs",
    ///     None)?;
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions<'_>>,
    ) -> azure_core::Result<Pager<ContainerQueryResults>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.containers_link);
        let base_request = Request::new(url, Method::POST);

        self.pipeline.send_query_request(
            options.method_options.context,
            query.into(),
            base_request,
            self.containers_link.clone(),
        )
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
        options: Option<CreateContainerOptions<'_>>,
    ) -> azure_core::Result<Response<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.containers_link);
        let mut req = Request::new(url, Method::POST);
        req.insert_headers(&options.throughput)?;
        req.set_json(&properties)?;

        self.pipeline
            .send(
                options.method_options.context,
                &mut req,
                self.containers_link.clone(),
            )
            .await
    }

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn delete(
        &self,
        options: Option<DeleteDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.link);
        let mut req = Request::new(url, Method::DELETE);
        self.pipeline
            .send(options.method_options.context, &mut req, self.link.clone())
            .await
    }

    /// Reads database throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn read_throughput(
        &self,
        options: Option<ThroughputOptions<'_>>,
    ) -> azure_core::Result<Option<Response<ThroughputProperties>>> {
        let options = options.unwrap_or_default();

        // We need to get the RID for the database.
        let db = self.read(None).await?.into_body().await?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        self.pipeline
            .read_throughput_offer(options.method_options.context, &resource_id)
            .await
    }

    /// Replaces the database throughput properties.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    pub async fn replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions<'_>>,
    ) -> azure_core::Result<Response<ThroughputProperties>> {
        let options = options.unwrap_or_default();

        // We need to get the RID for the database.
        let db = self.read(None).await?.into_body().await?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        self.pipeline
            .replace_throughput_offer(options.method_options.context, &resource_id, throughput)
            .await
    }
}
