// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::{
    clients::{ContainerClient, OffersClient},
    models::{ContainerProperties, CosmosResponse, DatabaseProperties, ThroughputProperties},
    options::ReadDatabaseOptions,
    pipeline::GatewayPipeline,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, FeedItemIterator, Query, QueryContainersOptions,
    ThroughputOptions,
};
use azure_core::http::Context;
use std::sync::Arc;

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    link: ResourceLink,
    containers_link: ResourceLink,
    database_id: String,
    pipeline: Arc<GatewayPipeline>,
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

impl DatabaseClient {
    pub(crate) fn new(
        pipeline: Arc<GatewayPipeline>,
        database_id: &str,
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    ) -> Self {
        let database_id = database_id.to_string();
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);
        let containers_link = link.feed(ResourceType::Containers);

        Self {
            link,
            containers_link,
            database_id,
            pipeline,
            global_endpoint_manager,
            global_partition_endpoint_manager,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    pub async fn container_client(&self, name: &str) -> ContainerClient {
        ContainerClient::new(
            self.pipeline.clone(),
            &self.link,
            name,
            self.global_endpoint_manager.clone(),
            self.global_partition_endpoint_manager.clone(),
        )
        .await
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
    ///     .into_model()?;
    /// # }
    /// ```
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<CosmosResponse<DatabaseProperties>> {
        let cosmos_request = CosmosRequest::builder(OperationType::Read, self.link.clone()).build();

        self.pipeline
            .send(cosmos_request?, Context::default())
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
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions>,
    ) -> azure_core::Result<FeedItemIterator<ContainerProperties>> {
        crate::query::executor::QueryExecutor::new(
            self.pipeline.clone(),
            self.containers_link.clone(),
            Context::default(),
            query.into(),
            azure_core::http::headers::Headers::new(),
        )
        .into_stream()
    }

    /// Creates a new container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `properties` - A [`ContainerProperties`] describing the new container.
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn create_container(
        &self,
        properties: ContainerProperties,
        options: Option<CreateContainerOptions>,
    ) -> azure_core::Result<CosmosResponse<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let cosmos_request =
            CosmosRequest::builder(OperationType::Create, self.containers_link.clone())
                .request_headers(&options.throughput)
                .json(&properties)
                .build()?;

        self.pipeline.send(cosmos_request, Context::default()).await
    }

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn delete(
        &self,
        options: Option<DeleteDatabaseOptions>,
    ) -> azure_core::Result<CosmosResponse<()>> {
        let cosmos_request =
            CosmosRequest::builder(OperationType::Delete, self.link.clone()).build();
        self.pipeline
            .send(cosmos_request?, Context::default())
            .await
    }

    /// Reads database throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn read_throughput(
        &self,
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        let offers_client = OffersClient::new(self.pipeline.clone(), resource_id);
        offers_client.read(Context::default()).await
    }

    /// Replaces the database throughput properties.
    ///
    /// Note that throughput changes may not take effect immediately.
    /// The service processes the change asynchronously, so you may need to poll
    /// [`DatabaseClient::read_throughput()`] to confirm the new throughput is in effect.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    #[tracing::instrument(skip_all, fields(id = self.database_id))]
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<CosmosResponse<ThroughputProperties>> {
        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        let offers_client = OffersClient::new(self.pipeline.clone(), resource_id);
        offers_client.replace(Context::default(), throughput).await
    }
}
