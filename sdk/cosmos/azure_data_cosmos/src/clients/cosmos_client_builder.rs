// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating [`CosmosClient`] instances.

#[cfg(feature = "fault_injection")]
use std::sync::Arc;

use crate::{
    clients::ClientContext,
    options::{
        CosmosClientOptions, OperationOptions, PartitionFailoverOptions,
        ThroughputControlGroupOptions, UserAgentSuffix,
    },
    AccountReference, CosmosClient, CosmosCredential, CosmosRuntime, RoutingStrategy,
};

/// Builder for creating [`CosmosClient`] instances.
///
/// Use this builder to configure and create a `CosmosClient` for interacting with Azure Cosmos DB.
///
/// An account reference (endpoint + credential) is required when calling [`build()`](Self::build).
/// Construct an [`AccountReference`] via [`AccountReference::with_credential`] (for token-credential
/// auth) or [`AccountReference::with_authentication_key`] (for shared-key auth, requires the
/// `key_auth` feature), then pass it to `build`.
///
/// A [`RoutingStrategy`] is also required to specify how the SDK should select regions.
///
/// # Clients and Runtimes
///
/// [`CosmosClient`] instances share some common state and mechanics between each other.
/// This allows for efficiently creating differently-configured clients for the same account,
/// as well as connecting to multiple accounts in the same application. To do this efficiently,
/// the [`CosmosRuntime`] serves as a single hub for all this background state and management.
/// When building a [`CosmosClient`], the [`CosmosClientBuilder`] uses a single shared process-wide
/// [`CosmosRuntime`]. However, by calling [`CosmosClientBuilder::with_runtime`], you can override
/// this and provide your own [`CosmosRuntime`] configured as necessary.
///
/// Configuring a [`CosmosRuntime`] manually for a production application is an advanced operation.
/// For most applications, the default global [`CosmosRuntime`] is sufficient.
/// However, it may sometimes be necessary to configure a custom [`CosmosRuntime`] in testing scenarios.
/// For example, when running against an emulator with an untrusted TLS certificate.
/// Server certificate validation is configured at the runtime layer.
///
/// See the documentation for [`CosmosRuntime`] for more information.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{CosmosClientBuilder, AccountReference, AccountEndpoint, RoutingStrategy};
/// use azure_data_cosmos::options::Region;
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = AccountReference::with_credential(endpoint, credential);
/// let client = CosmosClientBuilder::new()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClientBuilder, AccountReference, AccountEndpoint, RoutingStrategy};
/// use azure_data_cosmos::options::Region;
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = AccountReference::with_authentication_key(endpoint, Secret::from("my_account_key"));
/// let client = CosmosClientBuilder::new()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct CosmosClientBuilder {
    options: CosmosClientOptions,
    /// Pre-built runtime to attach. If `None`, the client falls back to
    /// a default global runtime.
    runtime: Option<CosmosRuntime>,
    /// Throughput control groups to register on this client's driver options.
    throughput_control_groups: Vec<ThroughputControlGroupOptions>,
    /// Fault-injection rules to apply on this client's driver.
    ///
    /// Evaluated by the driver's transport-layer fault-injection client.
    /// Empty by default.
    #[cfg(feature = "fault_injection")]
    fault_injection_rules: Vec<Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
    /// Fallback endpoints tried when the primary endpoint is unavailable.
    backup_endpoints: Vec<azure_core::http::Url>,
    /// Options to use for per-partition failover (PPAF, PPCB)
    partition_failover_options: Option<PartitionFailoverOptions>,
}

impl CosmosClientBuilder {
    /// Creates a new empty builder.
    ///
    /// Configure the builder with the desired options, then call [`build()`](Self::build)
    /// with the account endpoint and credential.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attaches a pre-built [`CosmosRuntime`] for the client to use.
    ///
    /// When set, the client uses the supplied runtime instead of the
    /// default global runtime. Clients constructed with the same
    /// runtime share its HTTP transport, connection pool, CPU sampler,
    /// and runtime-level defaults.
    pub fn with_runtime(mut self, runtime: CosmosRuntime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    /// Overrides the per-client default [`OperationOptions`] applied to all
    /// requests on this client unless a per-request override is supplied.
    ///
    /// The runtime carries its own runtime-wide defaults; values set here
    /// override them for this client, and per-request options override
    /// either.
    pub fn with_default_operation_options(mut self, options: OperationOptions) -> Self {
        self.options.operation = options;
        self
    }

    /// Configures the driver-level partition-failover / PPCB tuning for this
    /// client.
    ///
    /// These knobs are read once when the client's underlying driver is
    /// constructed (in [`build`](Self::build)) and govern the per-partition
    /// circuit breaker and partition-level failover for the lifetime of the
    /// client. They are independent of per-request [`OperationOptions`].
    ///
    /// When this setter is **not** called, the driver resolves these options
    /// from the `AZURE_COSMOS_PPCB_*` environment variables — including the
    /// `AZURE_COSMOS_PPCB_ENABLED` master switch and the
    /// `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` kill switch — falling back to
    /// compile-time defaults for anything unset. Passing an explicit value
    /// here takes precedence over those variables (except the
    /// `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` kill switch, which is read from the
    /// environment when you build the [`PartitionFailoverOptions`] and remains
    /// authoritative). To disable PPCB regardless of the account property, set
    /// `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE=false`.
    pub fn with_partition_failover_options(mut self, options: PartitionFailoverOptions) -> Self {
        self.partition_failover_options = Some(options);
        self
    }

    /// Sets a per-client suffix to append to the User-Agent header for
    /// telemetry, overriding any runtime-wide default suffix.
    ///
    /// Construct the suffix explicitly via
    /// [`UserAgentSuffix::new`](crate::options::UserAgentSuffix::new) for trusted
    /// values, or [`UserAgentSuffix::try_new`](crate::options::UserAgentSuffix::try_new)
    /// for untrusted input. Validation rules (max 25 characters,
    /// HTTP-header-safe) are enforced at the construction site rather than
    /// here, which keeps any panic local to the caller's input handling.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to append to the User-Agent header.
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.options.user_agent_suffix = Some(suffix);
        self
    }

    /// Configures fault injection for testing.
    ///
    /// Accepts a vector of [`FaultInjectionRule`](crate::fault_injection::FaultInjectionRule)
    /// values (the driver type re-exported through
    /// [`fault_injection`](crate::fault_injection)). Build each rule with
    /// [`FaultInjectionRuleBuilder`](crate::fault_injection::FaultInjectionRuleBuilder).
    /// The rules are forwarded to the driver runtime at
    /// [`build()`](Self::build) time and evaluated by the driver's
    /// transport-layer fault-injection client.
    ///
    /// Calling this multiple times replaces the previously-configured rule
    /// set; pass the complete final set on the last call.
    ///
    /// This is only available when the `fault_injection` feature is enabled.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection_rules(
        mut self,
        rules: Vec<Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
    ) -> crate::Result<Self> {
        // Defer validation to the driver-options layer which already enforces unique IDs.
        // Storing the vec here keeps build() lazy and lets us detect duplicates at the
        // single concatenation point in build_driver_options.
        self.fault_injection_rules = rules;
        Ok(self)
    }

    /// Throughput-control groups are scoped to this client's driver — the
    /// per-runtime registry has been removed, so every client owns its own
    /// set of groups. Duplicate group names supplied to the same builder are
    /// surfaced as an error at `build()` time.
    pub fn register_throughput_control_group(
        mut self,
        group: ThroughputControlGroupOptions,
    ) -> crate::Result<Self> {
        // Defer cross-layer validation to build_driver_options where the
        // full registry is composed; here we only collect.
        self.throughput_control_groups.push(group);
        Ok(self)
    }

    /// Sets backup endpoints for resilience when the primary global endpoint
    /// is unavailable during initialization.
    ///
    /// # When to use
    ///
    /// Configure backup endpoints when you want the client to survive a
    /// global endpoint outage during startup. Provide at least two regional
    /// endpoints (e.g., `https://myaccount-eastus.documents.azure.com/`).
    ///
    /// This is especially important in **non-public clouds** (sovereign,
    /// government) where the SDK cannot infer regional endpoints from the
    /// account name — without backup endpoints, a global endpoint failure
    /// during bootstrap is unrecoverable.
    ///
    /// # Behavior
    ///
    /// If the primary endpoint fails during driver bootstrap, the SDK tries
    /// each backup endpoint in order until one succeeds. Once initialized,
    /// regional endpoints discovered during bootstrap handle subsequent
    /// refreshes automatically.
    ///
    /// # Arguments
    ///
    /// * `endpoints` - Ordered list of fallback endpoint URLs.
    pub fn with_backup_endpoints(mut self, endpoints: Vec<crate::AccountEndpoint>) -> Self {
        self.backup_endpoints = endpoints.into_iter().map(|e| e.into_url()).collect();
        self
    }

    /// Builds the [`CosmosClient`] with the specified account reference and region selection strategy.
    ///
    /// The account reference bundles an endpoint and credential. Construct one using
    /// [`AccountReference::with_credential()`] or [`AccountReference::with_authentication_key()`]
    /// (the latter requires the `key_auth` feature).
    ///
    /// # Arguments
    ///
    /// * `account` - The account reference containing the endpoint and credential.
    /// * `routing_strategy` - The strategy for selecting which Azure regions to route requests to.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub async fn build(
        self,
        account: AccountReference,
        routing_strategy: RoutingStrategy,
    ) -> crate::Result<CosmosClient> {
        let (account_endpoint, credential) = account.into_parts();
        let endpoint = account_endpoint.into_url();

        // Clone credential for the driver before the SDK consumes it for auth policy.
        let driver_credential = credential.clone();

        let runtime = match self.runtime {
            Some(rt) => rt,
            None => CosmosRuntime::global().await?,
        };

        let driver_account =
            build_driver_account(endpoint, driver_credential, self.backup_endpoints);
        let driver_options = build_driver_options(
            driver_account,
            routing_strategy,
            self.options.operation,
            self.options.user_agent_suffix,
            self.partition_failover_options,
            #[cfg(feature = "fault_injection")]
            self.fault_injection_rules,
            self.throughput_control_groups,
        )?;
        let driver = runtime.into_inner().create_driver(driver_options).await?;

        Ok(CosmosClient {
            context: ClientContext { driver },
        })
    }
}

/// Builds [`DriverOptions`](azure_data_cosmos_driver::options::DriverOptions) for the given
/// account and routing strategy.
///
/// The routing strategy is converted to an ordered preferred-regions list:
///
/// - [`RoutingStrategy::ProximityTo`] expands to a proximity-sorted list of all
///   Azure regions, with the specified region first. An unrecognized region logs
///   a warning and falls back to an empty list, which causes the driver to use
///   the account's own region order.
/// - [`RoutingStrategy::PreferredRegions`] passes the caller's list through unchanged.
fn build_driver_options(
    account: azure_data_cosmos_driver::models::AccountReference,
    strategy: RoutingStrategy,
    operation_options: OperationOptions,
    user_agent_suffix: Option<UserAgentSuffix>,
    partition_failover_options: Option<PartitionFailoverOptions>,
    #[cfg(feature = "fault_injection")] fault_injection_rules: Vec<
        Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>,
    >,
    throughput_control_groups: Vec<ThroughputControlGroupOptions>,
) -> crate::Result<azure_data_cosmos_driver::options::DriverOptions> {
    let preferred_regions = match strategy {
        RoutingStrategy::ProximityTo(region) =>
            crate::region_proximity::generate_preferred_region_list(&region)
                .map(|s| s.to_vec())
                .unwrap_or_else(|| {
                    tracing::warn!(
                        region = %region,
                        "unrecognized application region; falling back to account-defined region order"
                    );
                    Vec::new()
                }),
        RoutingStrategy::PreferredRegions(regions) => regions,
    };
    let mut builder = azure_data_cosmos_driver::options::DriverOptions::builder(account)
        .with_preferred_regions(preferred_regions)
        .with_operation_options(operation_options);
    if let Some(suffix) = user_agent_suffix {
        builder = builder.with_user_agent_suffix(suffix);
    }
    if let Some(pfo) = partition_failover_options {
        builder = builder.with_partition_failover_options(pfo);
    }
    #[cfg(feature = "fault_injection")]
    if !fault_injection_rules.is_empty() {
        builder = builder
            .with_fault_injection_rules(fault_injection_rules)
            .map_err(crate::CosmosError::from)?;
    }
    for group in throughput_control_groups {
        builder = builder
            .register_throughput_control_group(group)
            .map_err(crate::CosmosError::from)?;
    }
    Ok(builder.build())
}

/// Builds a driver [`AccountReference`](azure_data_cosmos_driver::models::AccountReference)
/// from the SDK's credential and endpoint.
fn build_driver_account(
    endpoint: azure_core::http::Url,
    credential: CosmosCredential,
    backup_endpoints: Vec<azure_core::http::Url>,
) -> azure_data_cosmos_driver::models::AccountReference {
    let base = match credential {
        CosmosCredential::TokenCredential(tc) => {
            azure_data_cosmos_driver::models::AccountReference::with_credential(endpoint, tc)
        }
        #[cfg(feature = "key_auth")]
        CosmosCredential::MasterKey(key) => {
            azure_data_cosmos_driver::models::AccountReference::with_master_key(endpoint, key)
        }
    };
    base.with_backup_endpoints(backup_endpoints)
}

#[cfg(test)]
mod tests {
    use azure_data_cosmos_driver::CosmosDriverRuntimeBuilder;

    use super::*;
    use crate::{
        options::{PartitionFailoverOptions, Region, UserAgentSuffix},
        RoutingStrategy,
    };

    /// Reproduces the bug where `CosmosClientBuilder::with_user_agent_suffix`
    /// did not forward the suffix to the driver runtime, causing the
    /// User-Agent header on data-plane requests to lack the configured suffix.
    ///
    /// Mirrors the relevant wiring from `CosmosClientBuilder::build()`:
    /// the SDK options carry a `UserAgentSuffix`, which `build()` forwards
    /// onto `CosmosDriverRuntimeBuilder::with_user_agent_suffix`.
    #[tokio::test]
    async fn user_agent_suffix_is_forwarded_to_driver_runtime() {
        let suffix = UserAgentSuffix::new("myapp-westus2");

        let options = CosmosClientOptions {
            user_agent_suffix: Some(suffix.clone()),
            ..Default::default()
        };

        let mut driver_builder = CosmosDriverRuntimeBuilder::new();
        if let Some(s) = options.user_agent_suffix.clone() {
            driver_builder = driver_builder.with_user_agent_suffix(s);
        }
        let runtime = driver_builder.build().await.expect("runtime builds");

        assert_eq!(
            runtime.user_agent_suffix(),
            Some(&suffix),
            "driver runtime did not receive the user-agent suffix"
        );
        assert!(
            runtime.user_agent().as_str().contains(suffix.as_str()),
            "computed driver user-agent {:?} does not contain suffix {:?}",
            runtime.user_agent().as_str(),
            suffix.as_str(),
        );
    }

    #[tokio::test]
    async fn no_user_agent_suffix_yields_no_driver_suffix() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .build()
            .await
            .expect("runtime builds");
        assert!(runtime.user_agent_suffix().is_none());
    }

    #[test]
    fn user_agent_suffix_setter_records_value() {
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let builder = CosmosClientBuilder::new().with_user_agent_suffix(suffix.clone());
        assert_eq!(builder.options.user_agent_suffix.as_ref(), Some(&suffix));
    }

    fn test_account() -> azure_data_cosmos_driver::models::AccountReference {
        azure_data_cosmos_driver::models::AccountReference::with_master_key(
            "https://test.documents.azure.com/".parse().unwrap(),
            "dGVzdA==",
        )
    }

    /// `ProximityTo` a known region produces a non-empty preferred_regions list
    /// with the source region first.
    #[test]
    fn proximity_to_known_region_starts_with_source() {
        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::ProximityTo(Region::EAST_US),
            OperationOptions::default(),
            None,
            None,
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");
        let regions = opts.preferred_regions();
        assert!(
            !regions.is_empty(),
            "should produce a non-empty list for a known region"
        );
        assert_eq!(regions[0], Region::EAST_US, "source region should be first");
    }

    /// `ProximityTo` an unrecognized region falls back to an empty preferred_regions
    /// list so the driver uses the account's own region order.
    #[test]
    fn proximity_to_unknown_region_returns_empty_list() {
        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::ProximityTo(Region::from("not-a-real-region")),
            OperationOptions::default(),
            None,
            None,
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");
        assert!(
            opts.preferred_regions().is_empty(),
            "unrecognized region should yield an empty list"
        );
    }

    /// `PreferredRegions` passes the caller's list through to the driver options unchanged.
    #[test]
    fn preferred_regions_passes_through_unchanged() {
        let input = vec![Region::WEST_US, Region::EAST_US, Region::WEST_EUROPE];
        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::PreferredRegions(input.clone()),
            OperationOptions::default(),
            None,
            None,
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");
        assert_eq!(opts.preferred_regions(), input.as_slice());
    }

    /// The user-agent suffix must flow through to the per-driver options so
    /// the driver builds a User-Agent that overrides the runtime default.
    #[test]
    fn user_agent_suffix_flows_to_driver_options() {
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::PreferredRegions(Vec::new()),
            OperationOptions::default(),
            Some(suffix.clone()),
            None,
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");
        assert_eq!(opts.user_agent_suffix(), Some(&suffix));
    }

    /// Setting partition-failover options via the builder must thread through
    /// `build_driver_options` and land on the resulting `DriverOptions`.
    #[test]
    fn partition_failover_options_flow_to_driver_options() {
        let pfo = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(true)
            .with_read_failure_threshold(42)
            .build()
            .expect("valid partition failover options");

        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::PreferredRegions(Vec::new()),
            OperationOptions::default(),
            None,
            Some(pfo),
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");

        assert!(opts.partition_failover_options().circuit_breaker_enabled());
        assert_eq!(
            opts.partition_failover_options().read_failure_threshold(),
            42
        );
    }

    /// Omitting the partition-failover options on the builder must resolve
    /// them from the `AZURE_COSMOS_PPCB_*` environment. With none of those
    /// variables set (the CI default), resolution falls back to the type's
    /// compile-time defaults — i.e. PPCB enabled. This guards the wiring that
    /// routes an omitted value through the driver's env-backed builder rather
    /// than a bare `Default` that bypasses the environment entirely.
    #[test]
    fn missing_partition_failover_options_resolves_from_env_then_default() {
        let opts = build_driver_options(
            test_account(),
            RoutingStrategy::PreferredRegions(Vec::new()),
            OperationOptions::default(),
            None,
            None,
            #[cfg(feature = "fault_injection")]
            Vec::new(),
            Vec::new(),
        )
        .expect("build_driver_options should succeed");

        assert_eq!(
            opts.partition_failover_options().circuit_breaker_enabled(),
            PartitionFailoverOptions::default().circuit_breaker_enabled(),
        );
    }
}
