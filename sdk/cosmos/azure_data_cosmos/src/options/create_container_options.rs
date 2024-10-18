// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::DatabaseClientMethods;

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Debug, Default)]
pub struct CreateContainerOptions {}

impl CreateContainerOptions {
    /// Creates a new [`CreateContainerOptionsBuilder`](CreateContainerOptionsBuilder) that can be used to construct a [`CreateContainerOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::CreateContainerOptions::builder().build();
    /// ```
    pub fn builder() -> CreateContainerOptionsBuilder {
        CreateContainerOptionsBuilder::default()
    }
}

/// Builder used to construct a [`CreateContainerOptions`].
///
/// Obtain a [`CreateContainerOptionsBuilder`] by calling [`CreateContainerOptions::builder()`]
#[derive(Default)]
pub struct CreateContainerOptionsBuilder(CreateContainerOptions);

impl CreateContainerOptionsBuilder {
    /// Builds a [`CreateContainerOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> CreateContainerOptions {
        self.0.clone()
    }
}
