// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::DatabaseClient,
    models::{CosmosResponse, DatabaseProperties},
    pipeline::GatewayPipeline,
    resource_context::ResourceLink,
    CreateDatabaseOptions, FeedItemIterator, Query, QueryDatabasesOptions,
};
use azure_core::http::{Context, Url};
use serde::Serialize;
use std::sync::Arc;

use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;

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
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference};
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let account = CosmosAccountReference::with_credential(
///     "https://myaccount.documents.azure.com/",
///     credential,
/// ).unwrap();
/// let client = CosmosClient::builder()
///     .build(account)
///     .unwrap();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClient, CosmosAccountReference};
/// use azure_core::credentials::Secret;
///
/// let account = CosmosAccountReference::with_master_key(
///     "https://myaccount.documents.azure.com/",
///     Secret::from("my_account_key"),
/// ).unwrap();
/// let client = CosmosClient::builder()
///     .build(account)
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pub(crate) databases_link: ResourceLink,
    pub(crate) pipeline: Arc<GatewayPipeline>,
    pub(crate) global_endpoint_manager: Arc<GlobalEndpointManager>,
}

impl CosmosClient {
    /// Creates a new [`CosmosClientBuilder`] for constructing a `CosmosClient`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::{CosmosClient, CosmosAccountReference};
    ///
    /// let credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential> =
    ///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
    /// let account = CosmosAccountReference::with_credential(
    ///     "https://myaccount.documents.azure.com/",
    ///     credential,
    /// ).unwrap();
    /// let client = CosmosClient::builder()
    ///     .build(account)
    ///     .unwrap();
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
