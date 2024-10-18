// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::CosmosClientMethods;

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Debug, Default)]
pub struct CreateDatabaseOptions {}

impl CreateDatabaseOptions {
    /// Creates a new [`CreateDatabaseOptionsBuilder`](CreateDatabaseOptionsBuilder) that can be used to construct a [`CreateDatabaseOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::CreateDatabaseOptions::builder().build();
    /// ```
    pub fn builder() -> CreateDatabaseOptionsBuilder {
        CreateDatabaseOptionsBuilder::default()
    }
}

/// Builder used to construct a [`CreateDatabaseOptions`].
///
/// Obtain a [`CreateDatabaseOptionsBuilder`] by calling [`CreateDatabaseOptions::builder()`]
#[derive(Default)]
pub struct CreateDatabaseOptionsBuilder(CreateDatabaseOptions);

impl CreateDatabaseOptionsBuilder {
    /// Builds a [`CreateDatabaseOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> CreateDatabaseOptions {
        self.0.clone()
    }
}
