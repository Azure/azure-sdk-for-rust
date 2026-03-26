// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration options.

use std::sync::Arc;

use crate::{models::AccountReference, options::RuntimeOptions};

/// Configuration options for a Cosmos DB driver instance.
///
/// A driver represents a connection to a specific Cosmos DB account. It inherits
/// runtime-level defaults but can override them with driver-specific settings.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::AccountReference;
/// use azure_data_cosmos_driver::options::{
///     DriverOptions, DriverOptionsBuilder, RuntimeOptions, RuntimeOptionsBuilder, ContentResponseOnWrite,
/// };
/// use url::Url;
///
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-master-key",
/// );
///
/// let runtime = RuntimeOptionsBuilder::new()
///     .with_content_response_on_write(ContentResponseOnWrite::Disabled)
///     .build();
///
/// let options = DriverOptionsBuilder::new(account)
///     .with_runtime_options(runtime)
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// The Cosmos DB account reference (required).
    account: AccountReference,
    /// Driver-level runtime options, wrapped in Arc for cheap cloning and snapshot sharing.
    runtime_options: Arc<RuntimeOptions>,
}

impl DriverOptions {
    /// Returns a new builder for creating driver options.
    ///
    /// The account reference is required.
    pub fn builder(account: AccountReference) -> DriverOptionsBuilder {
        DriverOptionsBuilder::new(account)
    }

    /// Returns the account reference.
    pub fn account(&self) -> &AccountReference {
        &self.account
    }

    /// Returns the driver-level runtime options.
    pub fn runtime_options(&self) -> &Arc<RuntimeOptions> {
        &self.runtime_options
    }
}

/// Builder for creating [`DriverOptions`].
///
/// Use [`RuntimeOptionsBuilder`](super::RuntimeOptionsBuilder) to create runtime options,
/// then pass them to this builder via [`with_runtime_options()`](Self::with_runtime_options).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptionsBuilder {
    account: AccountReference,
    runtime_options: Option<RuntimeOptions>,
}

impl DriverOptionsBuilder {
    /// Creates a new builder with the required account reference.
    pub fn new(account: AccountReference) -> Self {
        Self {
            account,
            runtime_options: None,
        }
    }

    /// Sets the runtime options (defaults for operations).
    pub fn with_runtime_options(mut self, options: RuntimeOptions) -> Self {
        self.runtime_options = Some(options);
        self
    }

    /// Builds the [`DriverOptions`].
    pub fn build(self) -> DriverOptions {
        DriverOptions {
            account: self.account,
            runtime_options: Arc::new(self.runtime_options.unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{ContentResponseOnWrite, RuntimeOptionsBuilder};
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[test]
    fn builder_creates_options_with_account() {
        let account = test_account();
        let options = DriverOptionsBuilder::new(account.clone()).build();

        assert_eq!(options.account(), &account);
        assert!(options
            .runtime_options()
            .content_response_on_write
            .is_none());
    }

    #[test]
    fn builder_sets_runtime_options() {
        let runtime = RuntimeOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_runtime_options(runtime)
            .build();

        assert_eq!(
            options.runtime_options().content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
    }
}
