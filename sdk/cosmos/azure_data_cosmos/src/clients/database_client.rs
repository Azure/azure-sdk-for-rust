// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{offers_client, ClientContext, ContainerClient},
    models::{ContainerProperties, DatabaseProperties, ResourceResponse, ThroughputProperties},
    options::ReadDatabaseOptions,
    resource_context::{ResourceLink, ResourceType},
    CreateContainerOptions, DeleteDatabaseOptions, FeedItemIterator, Query, QueryContainersOptions,
    ThroughputOptions,
};
use azure_data_cosmos_driver::models::{CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::OperationOptions;

use super::ThroughputPoller;

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    link: ResourceLink,
    database_id: String,
    context: ClientContext,
    database_ref: DatabaseReference,
}

impl DatabaseClient {
    pub(crate) fn new(context: ClientContext, database_id: &str) -> Self {
        let database_id = database_id.to_string();
        let link = ResourceLink::root(ResourceType::Databases).item(&database_id);
        let database_ref =
            DatabaseReference::from_name(context.driver.account().clone(), database_id.clone());

        Self {
            link,
            database_id,
            context,
            database_ref,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the collection with the specified name.
    ///
    /// This method eagerly resolves immutable container metadata (resource ID and partition key
    /// definition) from the service, so the returned client is ready for immediate use without
    /// per-operation cache lookups.
    ///
    /// # Arguments
    /// * `name` - The name of the container.
    ///
    /// # Errors
    ///
    /// Returns an error if the container does not exist or the metadata cannot be resolved.
    pub async fn container_client(&self, name: &str) -> azure_core::Result<ContainerClient> {
        ContainerClient::new(self.context.clone(), &self.link, name, &self.database_id).await
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
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> azure_core::Result<ResourceResponse<DatabaseProperties>> {
        let operation = CosmosOperation::read_database(self.database_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_operation(operation, OperationOptions::default())
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
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
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions>,
    ) -> azure_core::Result<FeedItemIterator<ContainerProperties>> {
        let db_ref = DatabaseReference::from_name(
            self.context.driver.account().clone(),
            self.database_id.clone(),
        );
        let factory = move || CosmosOperation::query_containers(db_ref.clone());

        crate::query::executor::QueryExecutor::new(
            self.context.driver.clone(),
            factory,
            query.into(),
            Default::default(),
            None,
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
    pub async fn create_container(
        &self,
        properties: ContainerProperties,
        options: Option<CreateContainerOptions>,
    ) -> azure_core::Result<ResourceResponse<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let body = serde_json::to_vec(&properties)?;
        let mut operation =
            CosmosOperation::create_container(self.database_ref.clone()).with_body(body);

        if let Some(throughput) = &options.throughput {
            let mut headers = azure_data_cosmos_driver::models::CosmosRequestHeaders::new();
            throughput.apply_headers(&mut headers);
            operation = operation.with_request_headers(headers);
        }

        // Control-plane creates always need the full response body so the
        // caller can inspect the created resource properties.
        let mut operation_options = OperationOptions::default();
        operation_options.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);

        let driver_response = self
            .context
            .driver
            .execute_operation(operation, operation_options)
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Deletes this database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    #[allow(unused_variables, reason = "This parameter may be used in the future")]
    pub async fn delete(
        &self,
        options: Option<DeleteDatabaseOptions>,
    ) -> azure_core::Result<ResourceResponse<()>> {
        let operation = CosmosOperation::delete_database(self.database_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_operation(operation, OperationOptions::default())
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Reads database throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
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

        offers_client::find_offer(
            &self.context.driver,
            self.context.driver.account(),
            &resource_id,
        )
        .await
    }

    /// Begins replacing the database throughput properties.
    ///
    /// The Cosmos DB service may process throughput changes asynchronously. The returned
    /// [`ThroughputPoller`] can be awaited directly for the final result, or polled as a
    /// stream to observe progress.
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::models::ThroughputProperties;
    /// # async fn example(db_client: azure_data_cosmos::clients::DatabaseClient) -> azure_core::Result<()> {
    /// let throughput = db_client
    ///     .begin_replace_throughput(ThroughputProperties::manual(500), None)
    ///     .await? // start the replace operation
    ///     .await? // wait for completion (polls if async)
    ///     .into_model()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn begin_replace_throughput(
        &self,
        throughput: ThroughputProperties,
        options: Option<ThroughputOptions>,
    ) -> azure_core::Result<ThroughputPoller> {
        #[allow(
            unused_variables,
            reason = "The 'options' variable may be used in the future"
        )]
        let options = options.unwrap_or_default();
        // We need to get the RID for the database.
        let db = self.read(None).await?.into_model()?;
        let resource_id = db
            .system_properties
            .resource_id
            .expect("service should always return a '_rid' for a database");

        offers_client::begin_replace(
            self.context.driver.clone(),
            self.context.driver.account().clone(),
            &resource_id,
            throughput,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time assertion that `DatabaseClient` async method futures are `Send`.
    ///
    /// This function is never called; it only needs to compile.
    /// If any future is not `Send`, compilation will fail.
    #[allow(dead_code, unreachable_code, unused_variables)]
    fn _assert_futures_are_send() {
        fn assert_send<T: Send>(_: T) {}
        let client: &DatabaseClient = todo!();
        assert_send(client.container_client(todo!()));
        assert_send(client.read(todo!()));
        assert_send(client.create_container(todo!(), todo!()));
        assert_send(client.delete(todo!()));
        assert_send(client.read_throughput(todo!()));
        assert_send(client.begin_replace_throughput(todo!(), todo!()));
    }
}
