// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::DatabaseClientMethods;

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient)
#[derive(Clone, Debug, Default)]
pub struct QueryContainersOptions {}

impl QueryContainersOptions {
    /// Creates a new [`QueryContainersOptionsBuilder`](QueryContainersOptionsBuilder) that can be used to construct a [`QueryContainersOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::QueryContainersOptions::builder().build();
    /// ```
    pub fn builder() -> QueryContainersOptionsBuilder {
        QueryContainersOptionsBuilder::default()
    }
}

/// Builder used to construct a [`QueryContainersOptions`].
///
/// Obtain a [`QueryContainersOptionsBuilder`] by calling [`QueryContainersOptions::builder()`]
#[derive(Default)]
pub struct QueryContainersOptionsBuilder(QueryContainersOptions);

impl QueryContainersOptionsBuilder {
    /// Builds a [`QueryContainersOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> QueryContainersOptions {
        self.0.clone()
    }
}
