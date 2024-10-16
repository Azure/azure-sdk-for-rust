// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::DatabaseClientMethods;

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Debug, Default)]
pub struct DeleteDatabaseOptions {}

impl DeleteDatabaseOptions {
    /// Creates a new [`DeleteDatabaseOptionsBuilder`](DeleteDatabaseOptionsBuilder) that can be used to construct a [`DeleteDatabaseOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::DeleteDatabaseOptions::builder().build();
    /// ```
    pub fn builder() -> DeleteDatabaseOptionsBuilder {
        DeleteDatabaseOptionsBuilder::default()
    }
}

/// Builder used to construct a [`DeleteDatabaseOptions`].
///
/// Obtain a [`DeleteDatabaseOptionsBuilder`] by calling [`DeleteDatabaseOptions::builder()`]
#[derive(Default)]
pub struct DeleteDatabaseOptionsBuilder(DeleteDatabaseOptions);

impl DeleteDatabaseOptionsBuilder {
    /// Builds a [`DeleteDatabaseOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> DeleteDatabaseOptions {
        self.0.clone()
    }
}
