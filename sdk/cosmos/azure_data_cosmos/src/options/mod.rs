// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod cosmos_client_options;
mod create_database_options;
mod item_options;
mod query_containers_options;
mod query_databases_options;
mod query_options;
mod read_container_options;
mod read_database_options;

pub use cosmos_client_options::CosmosClientOptions;
pub use create_database_options::CreateDatabaseOptions;
pub use item_options::ItemOptions;
pub use query_containers_options::QueryContainersOptions;
pub use query_databases_options::QueryDatabasesOptions;
pub use query_options::QueryOptions;
pub use read_container_options::ReadContainerOptions;
pub use read_database_options::ReadDatabaseOptions;

pub mod builders {
    //! Builders used to create options types.
    //!
    //! You shouldn't need to construct these builders on your own. Instead, use the `builder()` method on the related options type to get an instance of the builder.

    pub use super::cosmos_client_options::CosmosClientOptionsBuilder;
    pub use super::create_database_options::CreateDatabaseOptionsBuilder;
    pub use super::item_options::ItemOptionsBuilder;
    pub use super::query_containers_options::QueryContainersOptionsBuilder;
    pub use super::query_databases_options::QueryDatabasesOptionsBuilder;
    pub use super::query_options::QueryOptionsBuilder;
    pub use super::read_container_options::ReadContainerOptionsBuilder;
    pub use super::read_database_options::ReadDatabaseOptionsBuilder;
}
