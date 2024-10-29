// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::ClientOptions;

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
#[derive(Clone, Debug, Default)]
pub struct CosmosClientOptions {
    pub client_options: ClientOptions,
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Debug, Default)]
pub struct CreateContainerOptions {}

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Debug, Default)]
pub struct CreateDatabaseOptions {}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Debug, Default)]
pub struct DeleteContainerOptions {}

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Debug, Default)]
pub struct DeleteDatabaseOptions {}

/// Options to be passed to APIs that manipulate items.
#[derive(Clone, Debug, Default)]
pub struct ItemOptions {}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers())
#[derive(Clone, Debug, Default)]
pub struct QueryContainersOptions {}

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[derive(Clone, Debug, Default)]
pub struct QueryDatabasesOptions {}

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Debug, Default)]
pub struct QueryOptions {}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Debug, Default)]
pub struct ReadContainerOptions {}

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Debug, Default)]
pub struct ReadDatabaseOptions {}
