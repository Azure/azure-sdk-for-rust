// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::{ClientContext, DatabaseClient},
    feed::QueryItemIterator,
    models::DatabaseProperties,
    models::ResourceResponse,
    options::{CreateDatabaseOptions, QueryDatabasesOptions},
    Query,
};
use azure_core::http::Url;
use azure_data_cosmos_driver::models::CosmosOperation;
use serde::Serialize;

pub use super::cosmos_client_builder::CosmosClientBuilder;

/// Client for Azure Cosmos DB.
///
/// Use [`CosmosClient::builder()`] to obtain a [`CosmosClientBuilder`] and
/// construct a configured client.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{CosmosClient, AccountReference, AccountEndpoint, RoutingStrategy};
/// use azure_data_cosmos::options::{Region};
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/"
///     .parse()
///     .unwrap();
/// let account = AccountReference::with_credential(endpoint, credential);
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
/// use azure_data_cosmos::{CosmosClient, AccountReference, AccountEndpoint, RoutingStrategy};
/// use azure_data_cosmos::options::{Region};
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/"
///     .parse()
///     .unwrap();
/// let account = AccountReference::with_authentication_key(
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
    /// use azure_data_cosmos::{CosmosClient, AccountReference, AccountEndpoint, RoutingStrategy};
    /// use azure_data_cosmos::options::{Region};
    ///
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential> =
    ///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
    /// let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/"
    ///     .parse()
    ///     .unwrap();
    /// let account = AccountReference::with_credential(endpoint, credential);
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

    /// Commits a preview distributed write transaction.
    ///
    /// **Preview / work in progress.** Requires the disabled-by-default
    /// `preview_dtx` feature and a service account with the DTX feature enabled.
    /// Not supported for production use; the API may change without notice.
    #[cfg(feature = "preview_dtx")]
    pub async fn commit_distributed_write(
        &self,
        transaction: crate::clients::DistributedWriteTransaction,
    ) -> crate::Result<crate::clients::DistributedTransactionResponse> {
        crate::clients::distributed_transaction::commit_distributed_write(
            &self.context,
            transaction,
        )
        .await
    }

    /// Executes a preview distributed read transaction.
    ///
    /// **Preview / work in progress.** Requires the disabled-by-default
    /// `preview_dtx` feature and a service account with the DTX feature enabled.
    /// Not supported for production use; the API may change without notice.
    #[cfg(feature = "preview_dtx")]
    pub async fn execute_distributed_read(
        &self,
        transaction: crate::clients::DistributedReadTransaction,
    ) -> crate::Result<crate::clients::DistributedTransactionResponse> {
        crate::clients::distributed_transaction::execute_distributed_read(
            &self.context,
            transaction,
        )
        .await
    }

    /// Gets the endpoint of the database account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        self.context.driver.account().endpoint()
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
    /// let dbs = client
    ///     .query_databases("SELECT * FROM dbs", None)
    ///     .await?;
    /// # }
    /// ```
    ///
    /// See [`Query`] for more information on how to specify a query.
    pub async fn query_databases(
        &self,
        query: impl Into<Query>,
        options: Option<QueryDatabasesOptions>,
    ) -> crate::Result<QueryItemIterator<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let query = query.into();
        let account = self.context.driver.account().clone();
        let initial_operation =
            CosmosOperation::query_databases(account).with_body(serde_json::to_vec(&query)?);
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

    /// Creates a new database.
    ///
    #[doc = include_str!("../../docs/control-plane-warning.md")]
    ///
    #[doc = include_str!("../../docs/control-plane-always-returns-body.md")]
    ///
    /// # Arguments
    /// * `id` - The ID of the new database.
    /// * `options` - Optional parameters for the request.
    pub async fn create_database(
        &self,
        id: &str,
        options: Option<CreateDatabaseOptions>,
    ) -> crate::Result<ResourceResponse<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        #[derive(Serialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }

        let body = serde_json::to_vec(&RequestBody { id })?;
        let operation =
            CosmosOperation::create_database(self.context.driver.account().clone()).with_body(body);

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
        assert_send(client.query_databases(Query::from("SELECT * FROM dbs"), todo!()));
        assert_send(client.create_database(todo!(), todo!()));
    }
}
