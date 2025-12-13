// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    clients::DatabaseClient,
    models::DatabaseProperties,
    pipeline::{AuthorizationPolicy, CosmosPipeline},
    resource_context::{ResourceLink, ResourceType},
    CosmosClientOptions, CreateDatabaseOptions, FeedPager, Query, QueryDatabasesOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{response::Response, Url},
};
use serde::Serialize;
use std::sync::Arc;

use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;
use azure_core::http::RetryOptions;
use crate::routing::collection_cache::CollectionCache;
use crate::routing::partition_key_range_cache::PartitionKeyRangeCache;

/// Client for Azure Cosmos DB.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    databases_link: ResourceLink,
    pipeline: Arc<CosmosPipeline>,
    collection_cache: CollectionCache,
    partition_key_range_cache: PartitionKeyRangeCache,
}

impl CosmosClient {
    /// Creates a new CosmosClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Cosmos DB account, for example `https://myaccount.documents.azure.com/`.
    /// * `credential` - An implementation of [`TokenCredential`](azure_core::credentials::TokenCredential) that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::sync::Arc;
    /// use azure_data_cosmos::CosmosClient;
    ///
    /// let credential = azure_identity::DeveloperToolsCredential::new(None).unwrap();
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// ```
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        let mut client_options = options.client_options;
        client_options.retry = RetryOptions::none();

        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::new(),
            vec![Arc::new(AuthorizationPolicy::from_token_credential(
                credential,
            ))],
            None,
        );

        let global_endpoint_manager = GlobalEndpointManager::new(
            endpoint.parse()?,
            options.application_preferred_regions,
            pipeline_core.clone(),
        );

        let collection_cache = CollectionCache::new(pipeline_core.clone(), global_endpoint_manager.clone());
        let partition_key_range_cache = PartitionKeyRangeCache::new(pipeline_core.clone(), Arc::from(collection_cache.clone()), Arc::from(global_endpoint_manager.clone()));

        let pipeline = Arc::new(CosmosPipeline::new(
            endpoint.parse()?,
            pipeline_core,
            global_endpoint_manager,
        ));

        Ok(Self {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline,
            collection_cache,
            partition_key_range_cache,
        })
    }

    /// Creates a new CosmosClient, using key authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Cosmos DB account, for example `https://myaccount.documents.azure.com/`.
    /// * `key` - The key to use when authenticating.
    /// * `options` - Optional configuration for the client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::CosmosClient;
    /// use azure_core::credentials::Secret;
    ///
    /// let client = CosmosClient::with_key("https://myaccount.documents.azure.com/", Secret::from("my_key"), None).unwrap();
    /// ```
    #[cfg(feature = "key_auth")]
    pub fn with_key(
        endpoint: &str,
        key: Secret,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        let mut client_options = options.client_options;
        client_options.retry = RetryOptions::none();

        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::new(),
            vec![Arc::new(AuthorizationPolicy::from_shared_key(key))],
            None,
        );

        let global_endpoint_manager = GlobalEndpointManager::new(
            endpoint.parse()?,
            options.application_preferred_regions,
            pipeline_core.clone(),
        );

        let collection_cache = CollectionCache::new(pipeline_core.clone(), global_endpoint_manager.clone());
        let partition_key_range_cache = PartitionKeyRangeCache::new(pipeline_core.clone(), Arc::from(collection_cache.clone()), Arc::from(global_endpoint_manager.clone()));

        let pipeline = Arc::new(CosmosPipeline::new(
            endpoint.parse()?,
            pipeline_core,
            global_endpoint_manager.clone(),
        ));

        Ok(Self {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline,
            collection_cache,
            partition_key_range_cache,
        })
    }

    /// Creates a new CosmosClient, using a connection string.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - the connection string to use for the client, e.g. `AccountEndpoint=https://accountname.documents.azure.com:443/‌​;AccountKey=accountk‌​ey`
    /// * `options` - Optional configuration for the client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_data_cosmos::CosmosClient;
    /// use azure_core::credentials::Secret;
    ///
    /// let client = CosmosClient::with_connection_string(
    ///     Secret::from("AccountEndpoint=https://accountname.documents.azure.com:443/‌​;AccountKey=accountk‌​ey"),
    ///     None)
    ///     .unwrap();
    /// ```
    #[cfg(feature = "key_auth")]
    pub fn with_connection_string(
        connection_string: Secret,
        options: Option<CosmosClientOptions>,
    ) -> Result<Self, azure_core::Error> {
        let connection_str = crate::ConnectionString::try_from(&connection_string)?;
        let endpoint = connection_str.account_endpoint;
        let key = connection_str.account_key;

        Self::with_key(endpoint.as_str(), key, options)
    }

    /// Gets a [`DatabaseClient`] that can be used to access the database with the specified ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the database.
    pub fn database_client(&self, id: &str) -> DatabaseClient {
        DatabaseClient::new(self.pipeline.clone(), id, &self.collection_cache, &self.partition_key_range_cache)
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
    pub fn query_databases(
        &self,
        query: impl Into<Query>,
        options: Option<QueryDatabasesOptions<'_>>,
    ) -> azure_core::Result<FeedPager<DatabaseProperties>> {
        let options = options.unwrap_or_default();
        let url = self.pipeline.url(&self.databases_link);

        self.pipeline.send_query_request(
            options.method_options.context,
            query.into(),
            url,
            self.databases_link.clone(),
            |_| Ok(()),
        )
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
        options: Option<CreateDatabaseOptions<'_>>,
    ) -> azure_core::Result<Response<DatabaseProperties>> {
        let options = options.unwrap_or_default();

        #[derive(Serialize)]
        struct RequestBody<'a> {
            id: &'a str,
        }

        let cosmos_request =
            CosmosRequest::builder(OperationType::Create, self.databases_link.clone())
                .headers(&options.throughput)
                .json(&RequestBody { id })
                .build()?;

        self.pipeline
            .send(cosmos_request, options.method_options.context)
            .await
    }
}
