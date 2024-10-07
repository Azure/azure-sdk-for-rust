// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::CosmosClientMethods;

/// Options to be passed to [`CosmosClient::query_databases()`]
#[derive(Clone, Debug, Default)]
pub struct QueryDatabasesOptions {}

impl QueryDatabasesOptions {
    /// Creates a new [`QueryDatabasesOptionsBuilder`](QueryDatabasesOptionsBuilder) that can be used to construct a [`QueryDatabasesOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::QueryDatabasesOptions::builder().build();
    /// ```
    pub fn builder() -> QueryDatabasesOptionsBuilder {
        QueryDatabasesOptionsBuilder::default()
    }
}

/// Builder used to construct a [`QueryDatabasesOptions`].
///
/// Obtain a [`QueryDatabasesOptionsBuilder`] by calling [`QueryDatabasesOptions::builder()`]
#[derive(Default)]
pub struct QueryDatabasesOptionsBuilder(QueryDatabasesOptions);

impl QueryDatabasesOptionsBuilder {
    /// Builds a [`QueryDatabasesOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> QueryDatabasesOptions {
        self.0.clone()
    }
}
