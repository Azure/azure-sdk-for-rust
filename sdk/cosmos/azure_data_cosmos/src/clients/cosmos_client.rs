// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{ClientContext, DatabaseClient},
    models::{DatabaseProperties, ResourceResponse},
    CreateDatabaseOptions, FeedItemIterator, Query, QueryDatabasesOptions,
};
use azure_core::http::Url;
use azure_data_cosmos_driver::models::CosmosOperation;
use azure_data_cosmos_driver::options::OperationOptions;
use serde::Serialize;

pub use super::cosmos_client_builder::CosmosClientBuilder;

/// Client for Azure Cosmos DB.
///
/// Use [`CosmosClientBuilder`] to create instances of this client.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint, Region, RoutingStrategy};
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/"
///     .parse()
///     .unwrap();
/// let account = CosmosAccountReference::with_credential(endpoint, credential);
/// let client = CosmosClient::builder()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint, Region, RoutingStrategy};
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/"
///     .parse()
///     .unwrap();
/// let account = CosmosAccountReference::with_master_key(
///     endpoint,
///     Secret::from("my_account_key"),
/// );
/// let client = CosmosClient::builder()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pub(crate) context: ClientContext,
}

impl CosmosClient {
    /// Creates a new [`CosmosClientBuilder`] for constructing a `CosmosClient`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint, Region, RoutingStrategy};
    ///
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential> =
    ///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
    /// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/"
    ///     .parse()
    ///     .unwrap();
    /// let account = CosmosAccountReference::with_credential(endpoint, credential);
    /// let client = CosmosClient::builder()
    ///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> CosmosClientBuilder {
        CosmosClientBuilder::new()
    }
    /// Gets a [`DatabaseClient`] that can be used to access the database with the specified ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the database.
    pub fn database_client(&self, id: &str) -> DatabaseClient {
        DatabaseClient::new(self.context.clone(), id)
    }

    /// Gets the endpoint of the database account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.context.pipeline.endpoint
    }

    /// Executes a query against databases in the account.
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
    /// # use azure_data_cosmos::CosmosClient;
    /// # let client: CosmosClient = panic!("this is a non-running example");
    /// let dbs = client.query_databases(
    ///     "SELECT * FROM dbs",
    ///     None)?;
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub fn query_databases(
        &self,
        query: impl Into<Query>,
        _options: Option<QueryDatabasesOptions>,
    ) -> azure_core::Result<FeedItemIterator<DatabaseProperties>> {
        let account = self.context.driver.account().clone();
        let factory = move || CosmosOperation::query_databases(account.clone());

        crate::query::executor::QueryExecutor::new(
            self.context.driver.clone(),
            factory,
            query.into(),
            Default::default(),
            None,
        )
        .into_stream()
    }

    /// Creates a new database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    /// # Arguments
    /// * `id` - The ID of the new database.
    /// * `options` - Optional parameters for the request.
    pub async fn create_database(
        &self,
        id: &str,
        #[allow(unused_variables, reason = "This parameter may be used in the future")]
        options: Option<CreateDatabaseOptions>,
    ) -> azure_core::Result<ResourceResponse<DatabaseProperties>> {
        #[derive(Serialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }

        let body = serde_json::to_vec(&RequestBody { id })?;
        let operation =
            CosmosOperation::create_database(self.context.driver.account().clone()).with_body(body);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time assertion that `CosmosClient` async method futures are `Send`.
    ///
    /// This function is never called; it only needs to compile.
    /// If any future is not `Send`, compilation will fail.
    #[allow(dead_code, unreachable_code, unused_variables)]
    fn _assert_futures_are_send() {
        fn assert_send<T: Send>(_: T) {}
        let client: &CosmosClient = todo!();
        assert_send(client.create_database(todo!(), todo!()));
    }
}
