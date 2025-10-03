// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::ContainerClient,
    connection::CosmosConnection,
    models::{ContainerProperties, DatabaseProperties, ThroughputProperties},
    options::ReadDatabaseOptions,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, FeedPager, Query, QueryContainersOptions,
    ThroughputOptions,
};

use azure_core::http::{
    request::{options::ContentType, Request},
    response::Response,
    Method,
};

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
#[derive(Clone)]
pub struct DatabaseClient {
    link: ResourceLink,
    containers_link: ResourceLink,
    database_id: String,
    connection: CosmosConnection,
}

impl DatabaseClient {
    pub(crate) fn new(connection: CosmosConnection, database_id: &str) -> Self {
        let database_id = database_id.to_string();
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);
        let containers_link = link.feed(ResourceType::Containers);

        Self {
            link,
            containers_link,
            database_id,
            connection,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    pub fn container_client(&self, name: &str) -> ContainerClient {
        ContainerClient::new(self.connection.clone(), &self.link, name)
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
    ///     .into_body()?;
    /// # }
    /// ```
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let url = self.connection.url(&self.link);
        let mut req = Request::new(url, Method::Get);
        self.connection
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
    ) -> azure_core::Result<FeedPager<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let url = self.connection.url(&self.containers_link);

        self.connection.send_query_request(
            options.method_options.context,
            query.into(),
            url,
            self.containers_link.clone(),
            |_| Ok(()),
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
        let url = self.connection.url(&self.containers_link);
        let mut req = Request::new(url, Method::Post);
        req.insert_headers(&options.throughput)?;
        req.insert_headers(&ContentType::APPLICATION_JSON)?;
        req.set_json(&properties)?;

        self.connection
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
    ) -> azure_core::Result<Response<()>> {
        let options = options.unwrap_or_default();
        let url = self.connection.url(&self.link);
        let mut req = Request::new(url, Method::Delete);
        self.connection
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
        let db = self.read(None).await?.into_body()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        self.connection
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
        let db = self.read(None).await?.into_body()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        self.connection
            .replace_throughput_offer(options.method_options.context, &resource_id, throughput)
            .await
    }
}
