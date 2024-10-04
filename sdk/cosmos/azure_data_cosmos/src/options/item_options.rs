// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(doc)]
use crate::clients::ContainerClientMethods;

/// Options to be passed to [`ContainerClient::create_item()`](crate::clients::ContainerClient::create_item()).
#[derive(Clone, Debug, Default)]
pub struct ItemOptions {}

impl ItemOptions {
    /// Creates a new [`ItemOptionsBuilder`](ItemOptionsBuilder) that can be used to construct a [`ItemOptions`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// let options = azure_data_cosmos::ItemOptions::builder().build();
    /// ```
    pub fn builder() -> ItemOptionsBuilder {
        ItemOptionsBuilder::default()
    }
}

/// Builder used to construct a [`ItemOptions`].
///
/// Obtain a [`ItemOptionsBuilder`] by calling [`ItemOptions::builder()`]
#[derive(Default)]
pub struct ItemOptionsBuilder(ItemOptions);

impl ItemOptionsBuilder {
    /// Builds a [`ItemOptions`] from the builder.
    ///
    /// This does not consume the builder, and can be called multiple times.
    pub fn build(&self) -> ItemOptions {
        self.0.clone()
    }
}
