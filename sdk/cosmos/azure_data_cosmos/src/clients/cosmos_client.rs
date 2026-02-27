// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::DatabaseClient,
    cosmos_request::CosmosRequest,
    models::{CosmosResponse, DatabaseProperties},
    operation_context::OperationType,
    pipeline::GatewayPipeline,
    resource_context::ResourceLink,
    routing::{
        global_endpoint_manager::GlobalEndpointManager,
        global_partition_endpoint_manager::GlobalPartitionEndpointManager,
    },
    CreateDatabaseOptions, FeedItemIterator, Query, QueryDatabasesOptions,
};
use azure_core::http::{Context, Url};
use serde::Serialize;
use std::sync::Arc;

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
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint};
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
///     .build(account)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint};
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
///     .build(account)
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pub(crate) databases_link: ResourceLink,
    pub(crate) pipeline: Arc<GatewayPipeline>,
    pub(crate) global_endpoint_manager: Arc<GlobalEndpointManager>,
    pub(crate) global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

impl CosmosClient {
    /// Creates a new [`CosmosClientBuilder`] for constructing a `CosmosClient`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint};
    ///
    /// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
    /// let credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential> =
    ///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
    /// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/"
    ///     .parse()
    ///     .unwrap();
    /// let account = CosmosAccountReference::with_credential(endpoint, credential);
    /// let client = CosmosClient::builder()
    ///     .build(account)
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
        DatabaseClient::new(
            self.pipeline.clone(),
            id,
            self.global_endpoint_manager.clone(),
            self.global_partition_endpoint_manager.clone(),
        )
    }

    /// Gets the endpoint of the database account this client is connected to.
    pub fn endpoint(&self) -> &Url {
        &self.pipeline.endpoint
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
    #[tracing::instrument(skip_all)]
    pub fn query_databases(
        &self,
        query: impl Into<Query>,
        _options: Option<QueryDatabasesOptions>,
    ) -> azure_core::Result<FeedItemIterator<DatabaseProperties>> {
        crate::query::executor::QueryExecutor::new(
            self.pipeline.clone(),
            self.databases_link.clone(),
            Context::default(),
            query.into(),
            azure_core::http::headers::Headers::new(),
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
    #[tracing::instrument(skip_all)]
    pub async fn create_database(
        &self,
        id: &str,
        options: Option<CreateDatabaseOptions>,
    ) -> azure_core::Result<CosmosResponse<DatabaseProperties>> {
        let options = options.unwrap_or_default();

        #[derive(Serialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }

        let cosmos_request =
            CosmosRequest::builder(OperationType::Create, self.databases_link.clone())
                .request_headers(&options.throughput)
                .json(&RequestBody { id })
                .build()?;

        self.pipeline.send(cosmos_request, Context::default()).await
    }
}
