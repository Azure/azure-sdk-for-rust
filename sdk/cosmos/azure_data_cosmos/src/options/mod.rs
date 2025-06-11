// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{ClientMethodOptions, ClientOptions};

use crate::models::ThroughputProperties;

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
#[derive(Clone, Default)]
pub struct CosmosClientOptions {
    pub client_options: ClientOptions,
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Default)]
pub struct CreateContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[derive(Clone, Default)]
pub struct ReplaceContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Default)]
pub struct CreateDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to APIs that manipulate items.
#[derive(Clone, Default)]
pub struct ItemOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub if_match_etag: Option<String>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    ///
    /// The default for this is `false`, which reduces the network and CPU burden that comes from serializing and deserializing the response.
    pub enable_content_response_on_write: bool,
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers())
#[derive(Clone, Default)]
pub struct QueryContainersOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[derive(Clone, Default)]
pub struct QueryDatabasesOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Default)]
pub struct QueryOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,

    /// An external query engine to use for executing the query.
    ///
    /// NOTE: This is an unstable feature and may change in the future.
    /// Specifically, the query engine may be built-in to the SDK in the future, and this option may be removed entirely.
    #[cfg(feature = "preview_query_engine")]
    pub query_engine: Option<crate::query::QueryEngineRef>,
}

impl QueryOptions<'_> {
    pub fn into_owned(self) -> QueryOptions<'static> {
        QueryOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
            #[cfg(feature = "preview_query_engine")]
            query_engine: self.query_engine,
        }
    }
}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Default)]
pub struct ReadContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Default)]
pub struct ReadDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
pub struct ThroughputOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}
