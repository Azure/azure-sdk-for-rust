// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level configuration options.

use std::sync::Arc;

use crate::{
    models::AccountReference,
    options::{
        OperationOptions, PartitionFailoverOptions, Region, ThroughputControlGroupOptions,
        ThroughputControlGroupRegistry, UserAgentSuffix,
    },
};

#[cfg(feature = "fault_injection")]
use crate::fault_injection::FaultInjectionRule;

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
    /// Optional driver-level override for the User-Agent suffix.
    ///
    /// When `Some`, this driver stamps requests with a User-Agent computed from
    /// this suffix (combined with the runtime's wrapping-SDK identifier and
    /// version metadata). When `None`, the driver inherits the runtime's
    /// precomputed User-Agent string verbatim, including any suffix the runtime
    /// itself was configured with.
    user_agent_suffix: Option<UserAgentSuffix>,
    /// Driver-level fault injection rules.
    ///
    /// When `Some(rules)` and `rules` is non-empty, the driver wraps the
    /// runtime's HTTP client factory with a fault-injecting factory that
    /// evaluates these rules on every data-plane request. The bootstrap
    /// transport (used for the initial account-metadata fetch) is never
    /// wrapped, so fault rules targeting `MetadataReadDatabaseAccount` only
    /// fire on the post-bootstrap refresh path.
    ///
    /// All drivers built from the same runtime can carry independent fault
    /// rules; they do not interact.
    #[cfg(feature = "fault_injection")]
    fault_injection_rules: Option<Vec<Arc<FaultInjectionRule>>>,
    /// Driver-level throughput control group registrations.
    ///
    /// At driver-creation time, the driver merges the runtime's registered
    /// groups with these driver-level groups into a single registry. The
    /// driver uses the merged registry to look up groups for every request.
    /// Cross-layer name collisions (or two `is_default=true` groups for the
    /// same container) error at driver creation.
    throughput_control_groups: ThroughputControlGroupRegistry,
    /// Driver-level partition-failover / PPCB tuning.
    ///
    /// These knobs are read once at driver construction time and govern the
    /// per-partition circuit breaker (PPCB) and partition-level failover
    /// behavior for the lifetime of the driver. They are independent of
    /// per-operation [`OperationOptions`].
    partition_failover_options: PartitionFailoverOptions,
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

    /// Returns the driver-level User-Agent suffix override, if any.
    pub fn user_agent_suffix(&self) -> Option<&UserAgentSuffix> {
        self.user_agent_suffix.as_ref()
    }

    /// Returns the driver-level fault injection rules, if any.
    ///
    /// `None` means no rules were configured on this driver; an empty `Some`
    /// is normalized to `None` at builder time and never returned here.
    #[cfg(feature = "fault_injection")]
    pub fn fault_injection_rules(&self) -> Option<&[Arc<FaultInjectionRule>]> {
        self.fault_injection_rules.as_deref()
    }

    /// Returns the driver-level throughput control group registry.
    ///
    /// This registry is merged with the runtime's registry at driver
    /// creation; the merged registry is what gets consulted on the request
    /// path.
    pub(crate) fn throughput_control_groups(&self) -> &ThroughputControlGroupRegistry {
        &self.throughput_control_groups
    }

    /// Returns the driver-level partition-failover / PPCB tuning options.
    pub fn partition_failover_options(&self) -> &PartitionFailoverOptions {
        &self.partition_failover_options
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
    user_agent_suffix: Option<UserAgentSuffix>,
    #[cfg(feature = "fault_injection")]
    fault_injection_rules: Option<Vec<Arc<FaultInjectionRule>>>,
    throughput_control_groups: ThroughputControlGroupRegistry,
    partition_failover_options: Option<PartitionFailoverOptions>,
}

impl DriverOptionsBuilder {
    /// Creates a new builder with the required account reference.
    pub fn new(account: AccountReference) -> Self {
        Self {
            account,
            operation_options: None,
            preferred_regions: Vec::new(),
            user_agent_suffix: None,
            #[cfg(feature = "fault_injection")]
            fault_injection_rules: None,
            throughput_control_groups: ThroughputControlGroupRegistry::new(),
            partition_failover_options: None,
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

    /// Overrides the User-Agent suffix for requests made through this driver.
    ///
    /// When set, the driver computes its own User-Agent string combining the
    /// runtime's wrapping-SDK identifier and version metadata with this suffix.
    /// When unset, the driver inherits the runtime's precomputed User-Agent
    /// string verbatim (cloning the shared `Arc` — no per-driver allocation).
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    /// Installs fault injection rules for this driver.
    ///
    /// Rules are appended to any previously configured rules on the builder
    /// (so multiple `with_fault_injection_rules` calls compose additively).
    /// At driver-creation time, the driver wraps the runtime's HTTP client
    /// factory with a fault-injecting factory that evaluates these rules
    /// on every data-plane request. Bootstrap (the initial account-metadata
    /// probe) is never wrapped, so rules targeting
    /// `MetadataReadDatabaseAccount` only fire on post-bootstrap refreshes.
    ///
    /// # Errors
    ///
    /// Returns `Err` when any rule's `id` collides with another rule already
    /// configured on this builder, or with another rule in the same call.
    /// Surfacing duplicates at builder time keeps the failure local to the
    /// misconfiguration; otherwise a silent late drop would surface as "my
    /// fault injection didn't fire" long after the duplicate was introduced.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection_rules(
        mut self,
        rules: Vec<Arc<FaultInjectionRule>>,
    ) -> crate::error::Result<Self> {
        if rules.is_empty() {
            return Ok(self);
        }

        let mut seen: std::collections::HashSet<String> = self
            .fault_injection_rules
            .as_ref()
            .map(|existing| existing.iter().map(|r| r.id().to_string()).collect())
            .unwrap_or_default();

        for rule in &rules {
            if !seen.insert(rule.id().to_string()) {
                return Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_DUPLICATE_FAULT_INJECTION_RULE_ID,
                    )
                    .with_message(format!("duplicate fault injection rule id: {}", rule.id()))
                    .build());
            }
        }

        match &mut self.fault_injection_rules {
            Some(existing) => existing.extend(rules),
            None => self.fault_injection_rules = Some(rules),
        }
        Ok(self)
    }

    /// Registers a throughput-control group on this driver.
    ///
    /// At driver-creation time, the driver merges the runtime's registry
    /// with the per-driver registry into a single registry consulted on
    /// every request. Cross-layer collisions (duplicate `(container, name)`
    /// key, or two `is_default=true` groups for the same container) are
    /// detected and surfaced at driver creation.
    ///
    /// Calling this multiple times appends groups; collisions within this
    /// builder are surfaced as soon as the conflict is introduced.
    pub fn register_throughput_control_group(
        mut self,
        group: ThroughputControlGroupOptions,
    ) -> crate::error::Result<Self> {
        self.throughput_control_groups
            .register(group)
            .map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED)
                    .with_message(e.to_string())
                    .build()
            })?;
        Ok(self)
    }

    /// Sets the partition-failover / PPCB tuning options for this driver.
    ///
    /// These knobs are read once at driver construction time and control
    /// the per-partition circuit breaker (PPCB) and partition-level failover
    /// for the lifetime of the driver. See [`PartitionFailoverOptions`] for
    /// the individual settings (and their environment-variable defaults).
    pub fn with_partition_failover_options(mut self, options: PartitionFailoverOptions) -> Self {
        self.partition_failover_options = Some(options);
        self
    }

    /// Builds the [`DriverOptions`].
    pub fn build(self) -> DriverOptions {
        DriverOptions {
            account: self.account,
            operation_options: Arc::new(self.operation_options.unwrap_or_default()),
            preferred_regions: self.preferred_regions,
            user_agent_suffix: self.user_agent_suffix,
            #[cfg(feature = "fault_injection")]
            fault_injection_rules: self.fault_injection_rules.filter(|r| !r.is_empty()),
            throughput_control_groups: self.throughput_control_groups,
            partition_failover_options: self.partition_failover_options.unwrap_or_default(),
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
