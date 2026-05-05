// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration options.

use crate::{
    models::AccountReference,
    options::{PartitionLevelFailoverPolicy, RuntimeOptions, SharedRuntimeOptions},
};

/// Configuration options for a Cosmos DB driver instance.
///
/// A driver represents a connection to a specific Cosmos DB account. It inherits
/// environment-level defaults but can override them with driver-specific settings.
///
/// # Thread Safety
///
/// The runtime options can be modified at runtime via the `runtime_options()` accessor.
/// Changes are thread-safe and will be applied to subsequent operations.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::AccountReference;
/// use azure_data_cosmos_driver::options::{
///     DriverOptions, DriverOptionsBuilder, RuntimeOptions, ContentResponseOnWrite,
///     PartitionLevelFailoverPolicy,
/// };
/// use url::Url;
///
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-master-key",
/// );
///
/// let runtime = RuntimeOptions::builder()
///     .with_content_response_on_write(ContentResponseOnWrite::Disabled)
///     .build();
///
/// let options = DriverOptionsBuilder::new(account)
///     .with_runtime_options(runtime)
///     .with_partition_level_failover_policy(PartitionLevelFailoverPolicy::Enabled)
///     .build();
///
/// // Later, modify defaults at runtime
/// options.runtime_options().set_content_response_on_write(Some(ContentResponseOnWrite::Enabled));
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// The Cosmos DB account reference (required).
    account: AccountReference,
    /// Thread-safe runtime options for operation options at the driver level.
    runtime_options: SharedRuntimeOptions,
    /// Client-side PPAF policy controlling per-partition automatic failover.
    partition_level_failover_policy: PartitionLevelFailoverPolicy,
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

    /// Returns the thread-safe runtime options.
    ///
    /// Use this to modify default operation options at runtime.
    pub fn runtime_options(&self) -> &SharedRuntimeOptions {
        &self.runtime_options
    }

    /// Returns the partition-level failover policy.
    ///
    /// This determines how the driver handles per-partition automatic failover
    /// for write operations on single-write accounts.
    pub fn partition_level_failover_policy(&self) -> PartitionLevelFailoverPolicy {
        self.partition_level_failover_policy
    }
}

/// Builder for creating [`DriverOptions`].
///
/// Use [`RuntimeOptions::builder()`] to create runtime options, then pass them
/// to this builder via [`with_runtime_options()`](Self::with_runtime_options).
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptionsBuilder {
    account: AccountReference,
    runtime_options: Option<RuntimeOptions>,
    partition_level_failover_policy: PartitionLevelFailoverPolicy,
}

impl DriverOptionsBuilder {
    /// Creates a new builder with the required account reference.
    pub fn new(account: AccountReference) -> Self {
        Self {
            account,
            runtime_options: None,
            partition_level_failover_policy: PartitionLevelFailoverPolicy::default(),
        }
    }

    /// Sets the runtime options (defaults for operations).
    ///
    /// Use [`RuntimeOptions::builder()`] to create the runtime options.
    pub fn with_runtime_options(mut self, options: RuntimeOptions) -> Self {
        self.runtime_options = Some(options);
        self
    }

    /// Sets the partition-level failover policy.
    ///
    /// Controls how the driver handles per-partition automatic failover (PPAF)
    /// for write operations on single-write accounts.
    ///
    /// - [`PartitionLevelFailoverPolicy::ServerControlled`] (default): respects the server-side
    ///   `enablePerPartitionFailoverBehavior` account property.
    /// - [`PartitionLevelFailoverPolicy::Enabled`]: force-enables PPAF regardless of server config.
    /// - [`PartitionLevelFailoverPolicy::Disabled`]: force-disables PPAF (client override).
    pub fn with_partition_level_failover_policy(
        mut self,
        policy: PartitionLevelFailoverPolicy,
    ) -> Self {
        self.partition_level_failover_policy = policy;
        self
    }

    /// Builds the [`DriverOptions`].
    pub fn build(self) -> DriverOptions {
        DriverOptions {
            account: self.account,
            runtime_options: SharedRuntimeOptions::from_options(
                self.runtime_options.unwrap_or_default(),
            ),
            partition_level_failover_policy: self.partition_level_failover_policy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::ContentResponseOnWrite;
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
            .snapshot()
            .content_response_on_write
            .is_none());
    }

    #[test]
    fn builder_sets_runtime_options() {
        let runtime = RuntimeOptions::builder()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_runtime_options(runtime)
            .build();

        let snapshot = options.runtime_options().snapshot();
        assert_eq!(
            snapshot.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
    }

    #[test]
    fn runtime_modification() {
        let options = DriverOptionsBuilder::new(test_account()).build();

        // Initially none
        assert!(options
            .runtime_options()
            .snapshot()
            .content_response_on_write
            .is_none());

        // Modify at runtime
        options
            .runtime_options()
            .set_content_response_on_write(Some(ContentResponseOnWrite::Enabled));

        // Now set
        assert_eq!(
            options
                .runtime_options()
                .snapshot()
                .content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
    }

    #[test]
    fn default_ppaf_policy_is_server_controlled() {
        let options = DriverOptionsBuilder::new(test_account()).build();
        assert_eq!(
            options.partition_level_failover_policy(),
            PartitionLevelFailoverPolicy::ServerControlled
        );
    }

    #[test]
    fn builder_sets_ppaf_policy() {
        let options = DriverOptionsBuilder::new(test_account())
            .with_partition_level_failover_policy(PartitionLevelFailoverPolicy::Enabled)
            .build();
        assert_eq!(
            options.partition_level_failover_policy(),
            PartitionLevelFailoverPolicy::Enabled
        );
    }
}
