// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::clients::DatabaseClient;
use crate::models::{DatabaseProperties, QueryResults};
use crate::pipeline::{AuthorizationPolicy, CosmosPipeline, ResourceType};
use crate::utils::AppendPathSegments;
use crate::{CosmosClientOptions, Query, QueryDatabasesOptions};
use azure_core::credentials::TokenCredential;
use azure_core::{Request, Url};
use std::sync::Arc;

#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;

/// Client for Azure Cosmos DB.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    endpoint: Url,
    pub(crate) pipeline: CosmosPipeline,

    #[allow(dead_code)]
    options: CosmosClientOptions,
}

/// Defines the methods provided by a [`CosmosClient`]
///
/// This trait is intended to allow you to mock out the `CosmosClient` when testing your application.
/// Rather than depending on `CosmosClient`, you can depend on a generic parameter constrained by this trait, or an `impl CosmosClientMethods` type.
pub trait CosmosClientMethods {
    /// Gets a [`DatabaseClient`] that can be used to access the database with the specified ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the database.
    fn database_client(&self, id: impl AsRef<str>) -> DatabaseClient;

    /// Gets the URL of the Cosmos DB Account this client is connected to.
    fn endpoint(&self) -> &Url;

    fn query_databases(
        &self,
        query: impl Into<Query>,
        options: Option<QueryDatabasesOptions>,
    ) -> azure_core::Result<azure_core::Pager<QueryResults<DatabaseProperties>>>;
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
    /// let credential = azure_identity::DefaultAzureCredential::new().unwrap();
    /// let client = CosmosClient::new("https://myaccount.documents.azure.com/", credential, None).unwrap();
    /// ```
    pub fn new(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            endpoint: endpoint.as_ref().parse()?,
            pipeline: CosmosPipeline::new(
                AuthorizationPolicy::from_token_credential(credential),
                options.client_options.clone(),
            ),
            options,
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
    ///
    /// let client = CosmosClient::with_key("https://myaccount.documents.azure.com/", "my_key", None).unwrap();
    /// ```
    #[cfg(feature = "key_auth")]
    pub fn with_key(
        endpoint: impl AsRef<str>,
        key: impl Into<Secret>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            endpoint: endpoint.as_ref().parse()?,
            pipeline: CosmosPipeline::new(
                AuthorizationPolicy::from_shared_key(key.into()),
                options.client_options.clone(),
            ),
            options,
        })
    }
}

impl CosmosClientMethods for CosmosClient {
    /// Gets a [`DatabaseClient`] that can be used to access the database with the specified ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the database.
    fn database_client(&self, id: impl AsRef<str>) -> DatabaseClient {
        DatabaseClient::new(self.pipeline.clone(), &self.endpoint, id.as_ref())
    }

    /// Gets the endpoint of the database account this client is connected to.
    fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    fn query_databases(
        &self,
        query: impl Into<Query>,
        options: Option<QueryDatabasesOptions>,
    ) -> azure_core::Result<azure_core::Pager<QueryResults<DatabaseProperties>>> {
        let mut url = self.endpoint.clone();
        url.append_path_segments(["dbs"]);
        let base_request = Request::new(url, azure_core::Method::Post);

        self.pipeline
            .send_query_request(query.into(), base_request, ResourceType::Databases)
    }
}
