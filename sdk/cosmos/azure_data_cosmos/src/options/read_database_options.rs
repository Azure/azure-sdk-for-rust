// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::DatabaseClientMethods;

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Debug, Default)]
pub struct ReadDatabaseOptions {}

impl ReadDatabaseOptions {
    /// Creates a new [`ReadDatabaseOptionsBuilder`](ReadDatabaseOptionsBuilder) that can be used to construct a [`ReadDatabaseOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::ReadDatabaseOptions::builder().build();
    /// ```
    pub fn builder() -> ReadDatabaseOptionsBuilder {
        ReadDatabaseOptionsBuilder::default()
    }
}

/// Builder used to construct a [`ReadDatabaseOptions`].
#[derive(Default)]
pub struct ReadDatabaseOptionsBuilder(ReadDatabaseOptions);

impl ReadDatabaseOptionsBuilder {
    /// Builds a [`ReadDatabaseOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> ReadDatabaseOptions {
        self.0.clone()
    }
}
