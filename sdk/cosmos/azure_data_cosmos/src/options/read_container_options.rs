// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::ContainerClientMethods;

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Debug, Default)]
pub struct ReadContainerOptions {}

impl ReadContainerOptions {
    /// Creates a new [`ReadContainerOptionsBuilder`](ReadContainerOptionsBuilder) that can be used to construct a [`ReadContainerOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::ReadContainerOptions::builder().build();
    /// ```
    pub fn builder() -> ReadContainerOptionsBuilder {
        ReadContainerOptionsBuilder::default()
    }
}

/// Builder used to construct a [`ReadContainerOptions`].
#[derive(Default)]
pub struct ReadContainerOptionsBuilder(ReadContainerOptions);

impl ReadContainerOptionsBuilder {
    /// Builds a [`ReadContainerOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> ReadContainerOptions {
        self.0.clone()
    }
}
