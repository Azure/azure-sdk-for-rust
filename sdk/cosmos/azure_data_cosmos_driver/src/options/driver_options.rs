// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration options.

use std::sync::Arc;

use crate::{
    models::AccountReference,
    options::{OperationOptions, Region},
};

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
///     DriverOptions, DriverOptionsBuilder,
///     OperationOptions, OperationOptionsBuilder,
/// };
/// use url::Url;
///
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-master-key",
/// );
///
/// let operation = OperationOptionsBuilder::new()
///     .with_max_failover_retry_count(5)
///     .with_max_session_retry_count(3)
///     .build();
///
/// let options = DriverOptionsBuilder::new(account)
///     .with_operation_options(operation)
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// The Cosmos DB account reference (required).
    account: AccountReference,
    /// Driver-level operation options (e.g., consistency, excluded regions, failover, session retry).
    operation_options: Arc<OperationOptions>,
    /// Preferred regions for routing, ordered by proximity to the application.
    ///
    /// When non-empty, read and write endpoint lists are reordered so that
    /// endpoints matching these regions appear first. Regions that don't match
    /// any account endpoint are silently skipped.
    preferred_regions: Vec<Region>,
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

    /// Returns the driver-level operation options.
    pub fn operation_options(&self) -> &Arc<OperationOptions> {
        &self.operation_options
    }

    /// Returns the preferred regions for routing.
    pub fn preferred_regions(&self) -> &[Region] {
        &self.preferred_regions
    }
}

/// Builder for creating [`DriverOptions`].
///
/// Use [`OperationOptionsBuilder`](super::OperationOptionsBuilder) to create operation options,
/// then pass them to this builder via [`with_operation_options()`](Self::with_operation_options).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptionsBuilder {
    account: AccountReference,
    operation_options: Option<OperationOptions>,
    preferred_regions: Vec<Region>,
}

impl DriverOptionsBuilder {
    /// Creates a new builder with the required account reference.
    pub fn new(account: AccountReference) -> Self {
        Self {
            account,
            operation_options: None,
            preferred_regions: Vec::new(),
        }
    }

    /// Sets the operation options (e.g., consistency, excluded regions, failover, session retry).
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation_options = Some(options);
        self
    }

    /// Sets the preferred regions for routing.
    ///
    /// Regions should be ordered by proximity to the application (closest first).
    /// The driver reorders endpoint lists to prefer these regions for both reads
    /// and writes. Regions not present in the account are silently skipped.
    pub fn with_preferred_regions(mut self, regions: Vec<Region>) -> Self {
        self.preferred_regions = regions;
        self
    }

    /// Builds the [`DriverOptions`].
    pub fn build(self) -> DriverOptions {
        DriverOptions {
            account: self.account,
            operation_options: Arc::new(self.operation_options.unwrap_or_default()),
            preferred_regions: self.preferred_regions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::OperationOptionsBuilder;
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
            .operation_options()
            .read_consistency_strategy
            .is_none());
        assert!(options
            .operation_options()
            .max_failover_retry_count
            .is_none());
        assert!(options
            .operation_options()
            .max_session_retry_count
            .is_none());
    }

    #[test]
    fn builder_sets_operation_options() {
        let operation = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .with_max_session_retry_count(3)
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_operation_options(operation)
            .build();

        assert_eq!(
            options.operation_options().max_failover_retry_count,
            Some(5)
        );
        assert_eq!(options.operation_options().max_session_retry_count, Some(3));
    }

    #[test]
    fn builder_sets_all_options() {
        let operation = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .with_max_session_retry_count(2)
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_operation_options(operation)
            .build();

        assert_eq!(
            options.operation_options().max_failover_retry_count,
            Some(5)
        );
        assert_eq!(options.operation_options().max_session_retry_count, Some(2));
        assert!(options
            .operation_options()
            .read_consistency_strategy
            .is_none());
        assert!(options.preferred_regions().is_empty());
    }

    #[test]
    fn builder_sets_preferred_regions() {
        let regions = vec![Region::WEST_US_2, Region::EAST_US];

        let options = DriverOptionsBuilder::new(test_account())
            .with_preferred_regions(regions.clone())
            .build();

        assert_eq!(options.preferred_regions(), &regions);
    }
}
