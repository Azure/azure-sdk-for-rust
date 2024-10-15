// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::ContainerClientMethods;

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Debug, Default)]
pub struct DeleteContainerOptions {}

impl DeleteContainerOptions {
    /// Deletes a new [`DeleteContainerOptionsBuilder`](DeleteContainerOptionsBuilder) that can be used to construct a [`DeleteContainerOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::DeleteContainerOptions::builder().build();
    /// ```
    pub fn builder() -> DeleteContainerOptionsBuilder {
        DeleteContainerOptionsBuilder::default()
    }
}

/// Builder used to construct a [`DeleteContainerOptions`].
///
/// Obtain a [`DeleteContainerOptionsBuilder`] by calling [`DeleteContainerOptions::builder()`]
#[derive(Default)]
pub struct DeleteContainerOptionsBuilder(DeleteContainerOptions);

impl DeleteContainerOptionsBuilder {
    /// Builds a [`DeleteContainerOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> DeleteContainerOptions {
        self.0.clone()
    }
}
