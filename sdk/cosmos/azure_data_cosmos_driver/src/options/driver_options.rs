// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration options.

use std::sync::Arc;

use crate::{
    models::AccountReference,
    options::{OperationOptions, RetryOptions, RuntimeOptions},
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
///     RetryOptions, RetryOptionsBuilder,
///     RuntimeOptions, RuntimeOptionsBuilder,
///     SessionRetryOptionsBuilder,
/// };
/// use url::Url;
///
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-master-key",
/// );
///
/// let runtime = RuntimeOptionsBuilder::new()
///     .with_max_failover_retry_count(5)
///     .build();
///
/// let operation = OperationOptionsBuilder::new().build();
///
/// let retry = RetryOptionsBuilder::new()
///     .with_session_retry(
///         SessionRetryOptionsBuilder::new()
///             .with_max_session_retry_count(3)
///             .build(),
///     )
///     .build();
///
/// let options = DriverOptionsBuilder::new(account)
///     .with_runtime_options(runtime)
///     .with_operation_options(operation)
///     .with_retry_options(retry)
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct DriverOptions {
    /// The Cosmos DB account reference (required).
    account: AccountReference,
    /// Driver-level runtime options, wrapped in Arc for cheap cloning and snapshot sharing.
    runtime_options: Arc<RuntimeOptions>,
    /// Driver-level operation options (e.g., consistency, excluded regions).
    operation_options: Arc<OperationOptions>,
    /// Driver-level retry options (e.g., session retry configuration).
    retry_options: Arc<RetryOptions>,
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

    /// Returns the driver-level operation options.
    pub fn operation_options(&self) -> &Arc<OperationOptions> {
        &self.operation_options
    }

    /// Returns the driver-level retry options.
    pub fn retry_options(&self) -> &Arc<RetryOptions> {
        &self.retry_options
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
    operation_options: Option<OperationOptions>,
    retry_options: Option<RetryOptions>,
}

impl DriverOptionsBuilder {
    /// Creates a new builder with the required account reference.
    pub fn new(account: AccountReference) -> Self {
        Self {
            account,
            runtime_options: None,
            operation_options: None,
            retry_options: None,
        }
    }

    /// Sets the runtime options (defaults for operations).
    pub fn with_runtime_options(mut self, options: RuntimeOptions) -> Self {
        self.runtime_options = Some(options);
        self
    }

    /// Sets the operation options (e.g., consistency, excluded regions).
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation_options = Some(options);
        self
    }

    /// Sets the retry options (e.g., session retry configuration).
    pub fn with_retry_options(mut self, options: RetryOptions) -> Self {
        self.retry_options = Some(options);
        self
    }

    /// Builds the [`DriverOptions`].
    pub fn build(self) -> DriverOptions {
        DriverOptions {
            account: self.account,
            runtime_options: Arc::new(self.runtime_options.unwrap_or_default()),
            operation_options: Arc::new(self.operation_options.unwrap_or_default()),
            retry_options: Arc::new(self.retry_options.unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{
        OperationOptionsBuilder, RetryOptionsBuilder, RuntimeOptionsBuilder,
        SessionRetryOptionsBuilder,
    };
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
        assert!(options.runtime_options().max_failover_retry_count.is_none());
        assert!(options
            .operation_options()
            .read_consistency_strategy
            .is_none());
        assert!(options.retry_options().session_retry.is_none());
    }

    #[test]
    fn builder_sets_runtime_options() {
        let runtime = RuntimeOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_runtime_options(runtime)
            .build();

        assert_eq!(options.runtime_options().max_failover_retry_count, Some(5));
    }

    #[test]
    fn builder_sets_operation_options() {
        let operation = OperationOptionsBuilder::new().build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_operation_options(operation)
            .build();

        assert!(options
            .operation_options()
            .read_consistency_strategy
            .is_none());
    }

    #[test]
    fn builder_sets_retry_options() {
        let retry = RetryOptionsBuilder::new()
            .with_session_retry(
                SessionRetryOptionsBuilder::new()
                    .with_max_session_retry_count(3)
                    .build(),
            )
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_retry_options(retry)
            .build();

        assert_eq!(
            options
                .retry_options()
                .session_retry
                .as_ref()
                .and_then(|sr| sr.max_session_retry_count),
            Some(3),
        );
    }

    #[test]
    fn builder_sets_all_options() {
        let runtime = RuntimeOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .build();
        let operation = OperationOptionsBuilder::new().build();
        let retry = RetryOptionsBuilder::new()
            .with_session_retry(
                SessionRetryOptionsBuilder::new()
                    .with_max_session_retry_count(2)
                    .build(),
            )
            .build();

        let options = DriverOptionsBuilder::new(test_account())
            .with_runtime_options(runtime)
            .with_operation_options(operation)
            .with_retry_options(retry)
            .build();

        assert_eq!(options.runtime_options().max_failover_retry_count, Some(5));
        assert!(options
            .operation_options()
            .read_consistency_strategy
            .is_none());
        assert_eq!(
            options
                .retry_options()
                .session_retry
                .as_ref()
                .and_then(|sr| sr.max_session_retry_count),
            Some(2),
        );
    }
}
