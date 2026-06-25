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
    ///
    /// When [`with_partition_failover_options`](Self::with_partition_failover_options)
    /// was **not** called, the partition-failover / PPCB options are resolved
    /// from the `AZURE_COSMOS_PPCB_*` environment variables (including the
    /// `AZURE_COSMOS_PPCB_ENABLED` master switch and the
    /// `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` kill switch). This is the single
    /// place those variables are honored when the caller does not supply
    /// explicit options, so omitting them no longer silently forces PPCB on.
    /// If the environment carries an out-of-bounds value the error is logged
    /// and the options fall back to [`PartitionFailoverOptions::default`]
    /// (so `build` stays infallible); callers that want strict validation
    /// should build [`PartitionFailoverOptions`] themselves and pass them via
    /// [`with_partition_failover_options`](Self::with_partition_failover_options).
    pub fn build(self) -> DriverOptions {
        self.build_from_env(&|k| std::env::var(k).ok())
    }

    /// Builds the [`DriverOptions`], resolving any omitted environment-backed
    /// option groups through the supplied `get_env` accessor instead of the
    /// process environment directly.
    ///
    /// [`build`](Self::build) delegates here with `|k| std::env::var(k).ok()`.
    /// The seam lets tests exercise the "caller omitted the options, so they
    /// come from the environment" path for every override combination
    /// deterministically, without mutating process-wide environment.
    pub(crate) fn build_from_env(self, get_env: &dyn Fn(&str) -> Option<String>) -> DriverOptions {
        // When the caller supplied explicit partition-failover options, honor
        // them verbatim. Otherwise resolve them from the environment — this is
        // the fix for "PPCB stays enabled even though AZURE_COSMOS_PPCB_ENABLED
        // (or the kill switch) is set to false": the env is only consulted by
        // the builder's `build`, which a caller that omits the options would
        // never reach, so the bare `unwrap_or_default()` used to bypass every
        // AZURE_COSMOS_PPCB_* variable.
        let partition_failover_options = match self.partition_failover_options {
            Some(options) => options,
            None => PartitionFailoverOptions::builder()
                .build_from_env(get_env)
                .unwrap_or_else(|e| {
                    tracing::warn!(
                        error = %e,
                        "failed to resolve PartitionFailoverOptions from the environment \
                         (AZURE_COSMOS_PPCB_*); falling back to defaults",
                    );
                    PartitionFailoverOptions::default()
                }),
        };

        DriverOptions {
            account: self.account,
            operation_options: Arc::new(self.operation_options.unwrap_or_default()),
            preferred_regions: self.preferred_regions,
            user_agent_suffix: self.user_agent_suffix,
            #[cfg(feature = "fault_injection")]
            fault_injection_rules: self.fault_injection_rules.filter(|r| !r.is_empty()),
            throughput_control_groups: self.throughput_control_groups,
            partition_failover_options,
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

    // ── Partition-failover / PPCB end-to-end env resolution ─────────────────
    //
    // These guard the customer-reported bug: when the caller omits
    // `with_partition_failover_options`, `DriverOptionsBuilder::build` must
    // resolve PPCB from `AZURE_COSMOS_PPCB_*` (rather than a bare `Default`
    // that bypasses the environment). They drive `build_from_env` with an
    // injected map so they don't race on process-wide `std::env`.

    use std::collections::HashMap;

    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        move |k: &str| map.get(k).cloned()
    }

    #[test]
    fn omitted_pfo_defaults_ppcb_on_with_empty_env() {
        let options = DriverOptionsBuilder::new(test_account()).build_from_env(&|_| None);
        assert!(options
            .partition_failover_options()
            .circuit_breaker_enabled());
        assert_eq!(
            options
                .partition_failover_options()
                .circuit_breaker_enabled_override(),
            None
        );
    }

    #[test]
    fn omitted_pfo_honors_env_disable() {
        // The exact customer scenario: no explicit options, env disables PPCB.
        // Before the fix this silently stayed enabled.
        let options = DriverOptionsBuilder::new(test_account())
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED", "false")]));
        assert!(!options
            .partition_failover_options()
            .circuit_breaker_enabled());
    }

    #[test]
    fn omitted_pfo_honors_kill_switch_from_env() {
        // The authoritative kill switch must be picked up on the omitted path
        // too, so an operator can force PPCB off fleet-wide without code.
        let options = DriverOptionsBuilder::new(test_account())
            .build_from_env(&env_of(&[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", "false")]));
        assert_eq!(
            options
                .partition_failover_options()
                .circuit_breaker_enabled_override(),
            Some(false)
        );
    }

    #[test]
    fn omitted_pfo_honors_env_thresholds_and_durations() {
        let options = DriverOptionsBuilder::new(test_account()).build_from_env(&env_of(&[
            ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "21"),
            ("AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS", "60000"),
        ]));
        let pfo = options.partition_failover_options();
        assert_eq!(pfo.read_failure_threshold(), 21);
        assert_eq!(
            pfo.counter_reset_window(),
            std::time::Duration::from_millis(60_000)
        );
    }

    #[test]
    fn explicit_pfo_takes_precedence_over_env() {
        // An explicitly-supplied options value is honored verbatim and the
        // environment is *not* consulted for that group.
        let explicit = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(true)
            .with_read_failure_threshold(7)
            .build()
            .expect("valid options");

        let options = DriverOptionsBuilder::new(test_account())
            .with_partition_failover_options(explicit)
            // Env would say "disabled / threshold 99" but must be ignored.
            .build_from_env(&env_of(&[
                ("AZURE_COSMOS_PPCB_ENABLED", "false"),
                ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "99"),
            ]));
        let pfo = options.partition_failover_options();
        assert!(pfo.circuit_breaker_enabled());
        assert_eq!(pfo.read_failure_threshold(), 7);
    }

    #[test]
    fn omitted_pfo_out_of_bounds_env_falls_back_to_default_infallibly() {
        // `build` is infallible: an out-of-bounds env value on the omitted
        // path is logged and the whole group falls back to defaults rather
        // than panicking or losing the builder's other (non-PPCB) settings.
        let options = DriverOptionsBuilder::new(test_account()).build_from_env(&env_of(&[(
            "AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD",
            "0",
        )]));
        let pfo = options.partition_failover_options();
        assert_eq!(
            pfo.read_failure_threshold(),
            PartitionFailoverOptions::default().read_failure_threshold()
        );
        assert!(pfo.circuit_breaker_enabled());
    }
}

/// Real-process-environment tests for the bug chokepoint:
/// [`DriverOptionsBuilder::build`] resolving omitted partition-failover options
/// from the actual `AZURE_COSMOS_PPCB_*` variables.
///
/// These complement the injected-closure tests above by exercising the public,
/// production `build()` (which reads `std::env::var`) end to end — the exact
/// path a customer hits when they set the env and never call
/// `with_partition_failover_options`. Every case runs inside [`with_scoped_env`]
/// (shared lock + clear/restore) so it is hermetic and parallel-safe.
#[cfg(test)]
mod real_env_tests {
    use super::*;
    use crate::options::env_parsing::test_env::{with_scoped_env, PPCB_ENV_VARS};
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[test]
    fn real_env_omitted_options_honor_disable() {
        // The exact customer scenario, end to end: env disables PPCB and the
        // caller never supplies options, so the driver must observe `false`.
        with_scoped_env(
            PPCB_ENV_VARS,
            &[("AZURE_COSMOS_PPCB_ENABLED", "false")],
            || {
                let options = DriverOptionsBuilder::new(test_account()).build();
                assert!(!options
                    .partition_failover_options()
                    .circuit_breaker_enabled());
            },
        );
    }

    #[test]
    fn real_env_omitted_options_default_enabled_when_unset() {
        with_scoped_env(PPCB_ENV_VARS, &[], || {
            let options = DriverOptionsBuilder::new(test_account()).build();
            assert!(options
                .partition_failover_options()
                .circuit_breaker_enabled());
        });
    }

    #[test]
    fn real_env_omitted_options_honor_kill_switch() {
        with_scoped_env(
            PPCB_ENV_VARS,
            &[("AZURE_COSMOS_PPCB_ENABLED_OVERRIDE", "false")],
            || {
                let options = DriverOptionsBuilder::new(test_account()).build();
                assert_eq!(
                    options
                        .partition_failover_options()
                        .circuit_breaker_enabled_override(),
                    Some(false)
                );
            },
        );
    }

    #[test]
    fn real_env_omitted_options_honor_tuning_combination() {
        with_scoped_env(
            PPCB_ENV_VARS,
            &[
                ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "21"),
                ("AZURE_COSMOS_PPCB_WRITE_FAILURE_THRESHOLD", "6"),
                ("AZURE_COSMOS_PPCB_COUNTER_RESET_WINDOW_MS", "60000"),
                ("AZURE_COSMOS_PPCB_FAILBACK_SWEEP_INTERVAL_MS", "90000"),
            ],
            || {
                let options = DriverOptionsBuilder::new(test_account()).build();
                let pfo = options.partition_failover_options();
                assert_eq!(pfo.read_failure_threshold(), 21);
                assert_eq!(pfo.write_failure_threshold(), 6);
                assert_eq!(
                    pfo.counter_reset_window(),
                    std::time::Duration::from_millis(60_000)
                );
                assert_eq!(
                    pfo.failback_sweep_interval(),
                    std::time::Duration::from_millis(90_000)
                );
            },
        );
    }

    #[test]
    fn real_env_explicit_options_beat_env() {
        // With explicit options supplied, the env is ignored entirely.
        with_scoped_env(
            PPCB_ENV_VARS,
            &[
                ("AZURE_COSMOS_PPCB_ENABLED", "false"),
                ("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "99"),
            ],
            || {
                let explicit = PartitionFailoverOptions::builder()
                    .with_circuit_breaker_enabled(true)
                    .with_read_failure_threshold(7)
                    .build_from_env(&|_| None)
                    .expect("valid options");
                let options = DriverOptionsBuilder::new(test_account())
                    .with_partition_failover_options(explicit)
                    .build();
                let pfo = options.partition_failover_options();
                assert!(pfo.circuit_breaker_enabled());
                assert_eq!(pfo.read_failure_threshold(), 7);
            },
        );
    }

    #[test]
    fn real_env_out_of_bounds_falls_back_infallibly() {
        // `build()` is infallible; an out-of-bounds env value is logged and the
        // group falls back to defaults rather than panicking.
        with_scoped_env(
            PPCB_ENV_VARS,
            &[("AZURE_COSMOS_PPCB_READ_FAILURE_THRESHOLD", "0")],
            || {
                let options = DriverOptionsBuilder::new(test_account()).build();
                let pfo = options.partition_failover_options();
                assert_eq!(
                    pfo.read_failure_threshold(),
                    PartitionFailoverOptions::default().read_failure_threshold()
                );
                assert!(pfo.circuit_breaker_enabled());
            },
        );
    }
}
