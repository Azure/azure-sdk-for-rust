// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{offers_client, ClientContext, ContainerClient},
    feed::QueryItemIterator,
    models::ResourceResponse,
    models::{ContainerProperties, DatabaseProperties, ThroughputProperties},
    options::{
        CreateContainerOptions, DeleteDatabaseOptions, QueryContainersOptions, ReadDatabaseOptions,
        ThroughputOptions,
    },
    Query, ResourceId, ResourceIdentity,
};
use azure_data_cosmos_driver::models::{CosmosOperation, DatabaseReference};

use super::ThroughputPoller;

/// A client for working with a specific database in a Cosmos DB account.
///
/// You can get a `DatabaseClient` by calling [`CosmosClient::database_client()`](crate::CosmosClient::database_client()).
pub struct DatabaseClient {
    identity: ResourceIdentity,
    context: ClientContext,
    database_ref: DatabaseReference,
}

impl DatabaseClient {
    pub(crate) fn new(context: ClientContext, identity: ResourceIdentity) -> Self {
        let account = context.driver.account().clone();
        let database_ref = match &identity {
            ResourceIdentity::Name(name) => {
                DatabaseReference::from_name(account, name.clone().into_owned())
            }
            ResourceIdentity::Rid(rid) => {
                DatabaseReference::from_rid(account, rid.as_str().to_owned())
            }
        };

        Self {
            identity,
            context,
            database_ref,
        }
    }

    /// Gets a [`ContainerClient`] that can be used to access the container with the
    /// specified identity.
    ///
    /// This method eagerly resolves immutable container metadata (resource ID and partition key
    /// definition) from the service, so the returned client is ready for immediate use without
    /// per-operation cache lookups.
    ///
    /// The container's addressing mode must match this database's: a name-addressed
    /// database accepts only name-addressed containers, and a RID-addressed database
    /// accepts only [`ResourceId`](crate::ResourceId)-addressed containers.
    ///
    /// # Arguments
    /// * `container` - The name or RID of the container.
    ///
    /// # Errors
    ///
    /// Returns an error if the container does not exist, the metadata cannot be
    /// resolved, or the addressing mode does not match this database's.
    pub async fn container_client(
        &self,
        container: impl Into<ResourceIdentity>,
    ) -> crate::Result<ContainerClient> {
        ContainerClient::new(self.context.clone(), &self.identity, container.into()).await
    }

    /// Returns the identifier used to construct this client: the database name
    /// when addressed by name, or the RID string when addressed by RID.
    pub fn id(&self) -> &str {
        match &self.identity {
            ResourceIdentity::Name(name) => name,
            ResourceIdentity::Rid(rid) => rid.as_str(),
        }
    }

    /// Returns the identity (name or RID) used to construct this client.
    pub fn identity(&self) -> &ResourceIdentity {
        &self.identity
    }

    /// Returns the database name, or `None` if this client was addressed by RID.
    pub fn name(&self) -> Option<&str> {
        self.identity.as_name()
    }

    /// Returns the database RID, or `None` if this client was addressed by name.
    pub fn rid(&self) -> Option<&ResourceId> {
        self.identity.as_rid()
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
    pub async fn read(
        &self,
        options: Option<ReadDatabaseOptions>,
    ) -> crate::Result<ResourceResponse<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let operation = CosmosOperation::read_database(self.database_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
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
    /// let containers = db_client
    ///     .query_containers("SELECT * FROM dbs", None)
    ///     .await?;
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub async fn query_containers(
        &self,
        query: impl Into<Query>,
        options: Option<QueryContainersOptions>,
    ) -> crate::Result<QueryItemIterator<ContainerProperties>> {
        let options = options.unwrap_or_default();
        let query = query.into();
        let initial_operation = CosmosOperation::query_containers(self.database_ref.clone())
            .with_body(serde_json::to_vec(&query)?);
        let operation_options = options.operation;

        let plan = self
            .context
            .driver
            .plan_operation(initial_operation, &operation_options, None)
            .await?;

        Ok(QueryItemIterator::new(
            self.context.driver.clone(),
            None,
            plan,
            operation_options,
        ))
    }

    /// Creates a new container.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    #[doc = include_str!("../../docs/control-plane-always-returns-body.md")]
    ///
    /// # Arguments
    /// * `properties` - A [`ContainerProperties`] describing the new container.
    /// * `options` - Optional parameters for the request.
    pub async fn create_container(
        &self,
        properties: ContainerProperties,
        options: Option<CreateContainerOptions>,
    ) -> crate::Result<ResourceResponse<ContainerProperties>> {
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
        let mut operation_options = options.operation;
        operation_options.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);

        let driver_response = self
            .context
            .driver
            .execute_singleton_operation(operation, operation_options)
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
    pub async fn delete(
        &self,
        options: Option<DeleteDatabaseOptions>,
    ) -> crate::Result<ResourceResponse<()>> {
        let options = options.unwrap_or_default();
        let operation = CosmosOperation::delete_database(self.database_ref.clone());

        let driver_response = self
            .context
            .driver
            .execute_singleton_operation(operation, options.operation)
            .await?;

        Ok(ResourceResponse::new(
            crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
        ))
    }

    /// Returns the database RID, using the client's identity directly when it is
    /// already RID-addressed, or reading the database from the service to obtain
    /// the `_rid` when addressed by name.
    async fn resource_id(&self) -> crate::Result<String> {
        if let Some(rid) = self.rid() {
            return Ok(rid.as_str().to_owned());
        }
        let db = self.read(None).await?.into_model()?;
        resource_id_or_error(db.system_properties.resource_id, "database")
    }

    /// Reads database throughput properties, if any.
    ///
    /// This will return `None` if the database does not have a throughput offer configured.
    ///
    /// # Arguments
    /// * `options` - Optional parameters for the request.
    pub async fn read_throughput(
        &self,
        options: Option<ThroughputOptions>,
    ) -> crate::Result<Option<ThroughputProperties>> {
        let options = options.unwrap_or_default();
        let resource_id = self.resource_id().await?;

        offers_client::find_offer(
            &self.context.driver,
            self.context.driver.account(),
            &resource_id,
            options.operation,
        )
        .await
    }

    /// Begins replacing the database throughput properties.
    ///
    /// The Cosmos DB service may process throughput changes asynchronously. The returned
    /// [`ThroughputPoller`] can be awaited directly for the final result, or polled as a
    /// stream to observe progress.
    ///
    #[doc = include_str!("../../docs/control-plane-always-returns-body.md")]
    ///
    /// # Arguments
    /// * `throughput` - The new throughput properties to set.
    /// * `options` - Optional parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use azure_data_cosmos::models::ThroughputProperties;
    /// # async fn example(db_client: azure_data_cosmos::clients::DatabaseClient) -> azure_data_cosmos::Result<()> {
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
    ) -> crate::Result<ThroughputPoller> {
        let options = options.unwrap_or_default();
        let resource_id = self.resource_id().await?;

        offers_client::begin_replace(
            self.context.driver.clone(),
            self.context.driver.account().clone(),
            &resource_id,
            throughput,
            options.operation,
        )
        .await
    }
}

/// Unwraps the `_rid` from a system-properties response. The Cosmos service
/// is contractually required to populate `_rid` on every resource read; if it
/// is missing we surface a synthetic 500 [`CosmosError`](crate::CosmosError)
/// rather than panicking, since panics in public methods would crash callers'
/// applications. The `debug_assert!` keeps tests honest while still letting
/// release builds recover.
fn resource_id_or_error(rid: Option<String>, resource_kind: &str) -> crate::Result<String> {
    debug_assert!(
        rid.is_some(),
        "service should always return a '_rid' for a {resource_kind}"
    );
    rid.ok_or_else(|| {
        crate::DriverCosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERVICE_RETURNED_OBJECT_WITHOUT_RID)
            .with_message(format!(
                "service did not return a '_rid' for a {resource_kind}; cannot resolve the throughput offer"
            ))
            .build()
            .into()
    })
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
        assert_send(client.container_client(todo!() as ResourceIdentity));
        assert_send(client.read(todo!()));
        assert_send(client.query_containers(Query::from("SELECT * FROM c"), todo!()));
        assert_send(client.create_container(todo!(), todo!()));
        assert_send(client.delete(todo!()));
        assert_send(client.read_throughput(todo!()));
        assert_send(client.begin_replace_throughput(todo!(), todo!()));
    }
}
