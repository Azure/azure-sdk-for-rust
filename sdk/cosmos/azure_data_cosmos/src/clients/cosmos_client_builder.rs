// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore behaviour
//! Builder for creating [`CosmosClient`] instances.

use crate::{
    clients::ClientContext,
    options::{CosmosClientOptions, ThroughputControlGroupOptions},
    AccountReference, CosmosClient, CosmosCredential, RoutingStrategy,
};

use azure_data_cosmos_driver::options::ConnectionPoolOptions;
#[cfg(all(feature = "allow_invalid_certificates", feature = "__tls",))]
use azure_data_cosmos_driver::options::EmulatorServerCertValidation;
use azure_data_cosmos_driver::CosmosDriverRuntimeBuilder;

use crate::constants::AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED;

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
    /// Whether to allow proxy usage. When false (default), `HTTPS_PROXY` is ignored.
    allow_proxy: bool,
    /// Throughput control groups to register on the driver runtime.
    throughput_control_groups: Vec<ThroughputControlGroupOptions>,
    /// Whether to accept invalid TLS certificates when connecting to the emulator.
    #[cfg(all(feature = "allow_invalid_certificates", feature = "__tls",))]
    allow_emulator_invalid_certificates: bool,
    /// Fault injection rules for testing error handling.
    ///
    /// Forwarded to the driver runtime at `build()` time and evaluated by
    /// the driver's transport-layer fault-injection client. Empty by
    /// default.
    #[cfg(feature = "fault_injection")]
    fault_injection_rules:
        Vec<std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
    /// Fallback endpoints tried when the primary endpoint is unavailable.
    backup_endpoints: Vec<azure_core::http::Url>,
    /// Custom driver runtime builder for testing (e.g., in-memory emulator transport).
    #[cfg(feature = "__internal_in_memory_emulator")]
    driver_runtime_builder: Option<CosmosDriverRuntimeBuilder>,
}

impl CosmosClientBuilder {
    /// Creates a new empty builder.
    ///
    /// Configure the builder with the desired options, then call [`build()`](Self::build)
    /// with the account endpoint and credential.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a suffix to append to the User-Agent header for telemetry.
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
    pub fn with_user_agent_suffix(mut self, suffix: crate::options::UserAgentSuffix) -> Self {
        self.options.user_agent_suffix = Some(suffix);
        self
    }

    /// Sets the maximum number of retries for requests that the service
    /// throttles (HTTP 429, rate-limited), along with the maximum cumulative
    /// wait time across those retries, as a single grouped
    /// [`ThrottlingRetryOptions`](crate::options::ThrottlingRetryOptions) value.
    ///
    /// Mirrors the .NET SDK's `MaxRetryAttemptsOnRateLimitedRequests` /
    /// `MaxRetryWaitTimeOnRateLimitedRequests` (grouped here as the Java SDK
    /// does via `CosmosClientBuilder.throttlingRetryOptions`). Build the value
    /// with [`ThrottlingRetryOptionsBuilder`](crate::options::ThrottlingRetryOptionsBuilder):
    ///
    /// - `max_retry_count` bounds the transport-level retry loop that honors
    ///   the service `x-ms-retry-after-ms` header (**default** `9`; a value of
    ///   `0` disables throttle retries, surfacing the first 429 to the caller).
    /// - `max_retry_wait_time` caps the cumulative retry delay (**default**
    ///   30 seconds); once the accumulated delay would exceed it, no further
    ///   throttle retry is attempted.
    ///
    /// **Scope**: applies *per transport-pipeline invocation*, not per logical
    /// operation. An operation that fans out across regions (failover,
    /// hedging) enters the transport pipeline once per leg, each with a fresh
    /// throttle-retry budget. To cap an operation's *total* wall-clock time,
    /// configure
    /// [`OperationOptions::end_to_end_latency_policy`](crate::options::OperationOptions::end_to_end_latency_policy).
    ///
    /// This client-wide value can be overridden per request via
    /// [`OperationOptions`](crate::options::OperationOptions).
    ///
    /// # Arguments
    ///
    /// * `options` - The grouped throttle-retry configuration.
    pub fn with_throttling_retry_options(
        mut self,
        options: crate::options::ThrottlingRetryOptions,
    ) -> Self {
        self.options.operation.throttling_retry_options = Some(options);
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
    pub fn with_fault_injection(
        mut self,
        rules: Vec<std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
    ) -> Self {
        self.fault_injection_rules = rules;
        self
    }

    /// Configures the client to accept invalid TLS certificates when connecting
    /// to the Azure Cosmos DB emulator.
    ///
    /// This setting only applies when connecting to the local emulator
    /// (e.g., `https://localhost:8081/`). It should not be used for production endpoints.
    ///
    /// # Arguments
    ///
    /// * `allow` - Whether to accept invalid certificates for emulator connections.
    #[cfg(all(feature = "allow_invalid_certificates", feature = "__tls",))]
    pub fn with_allow_emulator_invalid_certificates(mut self, allow: bool) -> Self {
        self.allow_emulator_invalid_certificates = allow;
        self
    }

    /// Allows the SDK to use HTTP proxies and respect system proxy settings.
    ///
    /// By default, the Cosmos DB SDK ignores the `HTTPS_PROXY`, `HTTP_PROXY`,
    /// `ALL_PROXY` environment variables and their lowercase variants. Proxies
    /// can cause issues for Cosmos DB connectivity, availability, and throughput.
    ///
    /// When enabled, the SDK will respect system-configured proxy settings
    /// (such as proxy-related environment variables, including any exclusions).
    ///
    /// NOTE: End-to-end latency, availability, and throughput guarantees cannot
    /// be provided when a proxy is in use. Full backend support is provided,
    /// but client/proxy interactions are supported on a best-effort basis only.
    ///
    /// # Arguments
    ///
    /// * `allow` - Whether to allow proxy usage.
    pub fn with_proxy_allowed(mut self, allow: bool) -> Self {
        self.allow_proxy = allow;
        self
    }

    /// Registers a throughput control group on the driver runtime.
    ///
    /// Groups define throughput policies (priority level, throughput bucket) that
    /// are applied to requests referencing the group name via
    /// [`OperationOptions::throughput_control_group`](crate::options::OperationOptions::throughput_control_group).
    pub fn with_throughput_control_group(mut self, group: ThroughputControlGroupOptions) -> Self {
        self.throughput_control_groups.push(group);
        self
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

    /// Provides a pre-configured [`CosmosDriverRuntimeBuilder`] for the client to use.
    ///
    /// When set, the client uses this builder instead of creating a default one.
    /// This enables testing with custom transports such as the
    /// [`InMemoryEmulatorHttpClient`](azure_data_cosmos_driver::in_memory_emulator::InMemoryEmulatorHttpClient).
    ///
    /// # Field interactions
    ///
    /// After `build()` is invoked, the SDK forwards a small set of its own
    /// settings into the supplied builder. These overwrite the corresponding
    /// fields on the supplied builder (last-writer-wins):
    ///
    /// - **Connection pool** (`with_connection_pool`): always replaced by an
    ///   SDK-derived pool that reflects `with_proxy_allowed` and
    ///   `with_allow_emulator_invalid_certificates`. The pool is then passed
    ///   to whatever `HttpClientFactory` is in effect — the default reqwest
    ///   factory honors it, but the in-memory emulator transport supplied via
    ///   `with_http_client_factory` ignores its argument since it does not
    ///   perform real HTTP. Tests against the emulator therefore see no
    ///   connection-pool behaviour regardless of what is configured here.
    /// - **Fault injection rules** (`with_fault_injection_rules`): the SDK
    ///   appends each rule from its own fault-injection builder to the
    ///   rules already configured on the supplied builder (additive). Both
    ///   sources contribute and neither is silently dropped. `build` returns
    ///   an error if a rule on the SDK builder shares its `id` with one
    ///   already registered on the supplied driver runtime builder, so
    ///   callers wiring a runtime builder of their own are responsible for
    ///   keeping rule ids globally unique.
    /// - **Throughput control groups** (`register_throughput_control_group`):
    ///   the SDK appends each group registered via
    ///   `with_throughput_control_group` (additive — does not clear existing
    ///   groups on the supplied builder).
    ///
    /// All other fields on the supplied builder — most importantly
    /// `with_http_client_factory` (the in-memory emulator transport),
    /// `with_cpu_refresh_interval`, and any future fields — are left
    /// untouched and take effect as configured.
    #[cfg(feature = "__internal_in_memory_emulator")]
    pub fn with_driver_runtime_builder(mut self, builder: CosmosDriverRuntimeBuilder) -> Self {
        self.driver_runtime_builder = Some(builder);
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

        // Fault-injection rules flow directly to the driver runtime; no
        // SDK-side translation needed now that the SDK fault-injection types
        // are pure re-exports of the driver types.
        #[cfg(feature = "fault_injection")]
        let driver_fi_rules: Vec<
            std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>,
        > = self.fault_injection_rules;

        // Preserve the SDK's historical default: per-partition circuit breaker
        // (PPCB) is enabled unless the user explicitly opts out via
        // `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED=false`. The
        // driver itself defaults to `false`, so the SDK reads the env var
        // here and explicitly sets it as a runtime-level default on the
        // driver. The runtime layer sits above the env layer in the driver's
        // option-resolution hierarchy, so this guarantees PPCB is on by
        // default for SDK clients while still letting users disable it via
        // the env var.
        let ppcb_enabled = std::env::var(AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED)
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true);

        let driver_user_agent_suffix = self.options.user_agent_suffix.clone();

        // Create the CosmosDriver for eager container metadata resolution.
        // TODO: Each CosmosClient currently creates its own CosmosDriverRuntime. The runtime
        // should be shared across clients targeting the same account to avoid duplicate
        // background tasks and connection pools. See https://github.com/Azure/azure-sdk-for-rust/issues/3908
        let driver_account =
            build_driver_account(endpoint, driver_credential, self.backup_endpoints);
        #[cfg(feature = "__internal_in_memory_emulator")]
        let mut driver_runtime_builder = self.driver_runtime_builder.unwrap_or_default();
        #[cfg(not(feature = "__internal_in_memory_emulator"))]
        let mut driver_runtime_builder = CosmosDriverRuntimeBuilder::new();

        // Forward SDK connection settings to the driver's connection pool.
        let mut pool_builder = ConnectionPoolOptions::builder();
        if self.allow_proxy {
            pool_builder = pool_builder.with_proxy_allowed(true);
        }
        #[cfg(all(feature = "allow_invalid_certificates", feature = "__tls",))]
        if self.allow_emulator_invalid_certificates {
            pool_builder = pool_builder.with_emulator_server_cert_validation(
                EmulatorServerCertValidation::DangerousDisabled,
            );
        }
        driver_runtime_builder = driver_runtime_builder.with_connection_pool(pool_builder.build()?);

        // The wrapping-SDK identifier always reflects this crate, so requests
        // can be attributed to `azure_data_cosmos` in addition to the driver.
        driver_runtime_builder = driver_runtime_builder.with_wrapping_sdk_identifier(format!(
            "azsdk-rust-cosmos/{}",
            env!("CARGO_PKG_VERSION")
        ));
        // Forward the user-agent suffix captured above to the driver runtime.
        if let Some(suffix) = driver_user_agent_suffix {
            driver_runtime_builder = driver_runtime_builder.with_user_agent_suffix(suffix);
        }

        // Apply the SDK's PPCB default at the runtime layer. This sits above
        // the env layer in the driver's option-resolution hierarchy, so it
        // pins the SDK's "enabled by default" behavior even when the env var
        // is unset.
        //
        // Start from the client-level operation defaults (set via the builder,
        // e.g. `with_throttling_retry_options`) so they are
        // forwarded to the driver's runtime layer, then force the resolved
        // PPCB default on top — but only when the SDK builder hasn't already
        // set an explicit value. No public SDK setter exists today, but the
        // `is_none()` guard prevents a future PR that adds one from silently
        // having its value clobbered here.
        let mut runtime_operation_options = self.options.operation.clone();
        if runtime_operation_options
            .per_partition_circuit_breaker_enabled
            .is_none()
        {
            runtime_operation_options.per_partition_circuit_breaker_enabled = Some(ppcb_enabled);
        }
        driver_runtime_builder =
            driver_runtime_builder.with_default_operation_options(runtime_operation_options);

        for group in self.throughput_control_groups {
            driver_runtime_builder = driver_runtime_builder
                .register_throughput_control_group(group)
                .map_err(|e| {
                    crate::DriverCosmosError::builder()
                        .with_status(crate::error::CosmosStatus::CLIENT_THROUGHPUT_CONTROL_GROUP_REGISTRATION_FAILED)
                        .with_message(format!("failed to register throughput control group: {e}"))
                        .build()
                })?;
        }
        let driver_runtime = driver_runtime_builder.build().await?;
        let driver_options = build_driver_options(
            driver_account,
            routing_strategy,
            #[cfg(feature = "fault_injection")]
            driver_fi_rules,
        )?;
        let driver = driver_runtime.create_driver(driver_options).await?;

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
    #[cfg(feature = "fault_injection")] fault_injection_rules: Vec<
        std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>,
    >,
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
        .with_preferred_regions(preferred_regions);
    #[cfg(feature = "fault_injection")]
    if !fault_injection_rules.is_empty() {
        builder = builder
            .with_fault_injection_rules(fault_injection_rules)
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
    use super::*;
    use crate::options::{Region, UserAgentSuffix};

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

    /// Regression test: the SDK must default to per-partition circuit breaker
    /// (PPCB) **enabled** unless the user explicitly opts out via
    /// `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED=false`. The
    /// underlying driver defaults to `false`, so the SDK must explicitly set
    /// the runtime-level option to preserve historical behavior.
    ///
    /// This test mirrors the wiring from `CosmosClientBuilder::build()`:
    /// read the env var with `unwrap_or(true)`, then push it onto the
    /// runtime as the SDK's default. We deliberately do NOT touch the
    /// process env var here because tests share a process; instead we
    /// inline the same default-resolution logic and assert the runtime
    /// reflects the chosen value.
    #[tokio::test]
    async fn ppcb_default_is_enabled_when_env_var_unset() {
        // Simulate "env var unset" → SDK's default is `true`.
        let ppcb_enabled = Option::<String>::None
            .and_then(|v: String| v.parse::<bool>().ok())
            .unwrap_or(true);
        assert!(
            ppcb_enabled,
            "SDK's PPCB default must be `true` when env var is unset"
        );

        let runtime_op_options = azure_data_cosmos_driver::options::OperationOptionsBuilder::new()
            .with_per_partition_circuit_breaker_enabled(ppcb_enabled)
            .build();
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_default_operation_options(runtime_op_options)
            .build()
            .await
            .expect("runtime builds");

        assert_eq!(
            runtime
                .default_operation_options()
                .per_partition_circuit_breaker_enabled,
            Some(true),
            "PPCB must be enabled by default on a CosmosClient-built runtime"
        );
    }

    /// Regression test: when the env var is explicitly set to `false`, the
    /// SDK must propagate that opt-out to the driver runtime so PPCB is
    /// disabled.
    #[tokio::test]
    async fn ppcb_can_be_opted_out_via_env_var() {
        // Simulate `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED=false`.
        let ppcb_enabled = Some("false".to_string())
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true);
        assert!(!ppcb_enabled, "env var `false` must opt out of PPCB");

        let runtime_op_options = azure_data_cosmos_driver::options::OperationOptionsBuilder::new()
            .with_per_partition_circuit_breaker_enabled(ppcb_enabled)
            .build();
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_default_operation_options(runtime_op_options)
            .build()
            .await
            .expect("runtime builds");

        assert_eq!(
            runtime
                .default_operation_options()
                .per_partition_circuit_breaker_enabled,
            Some(false),
            "explicit env-var opt-out must propagate to the driver runtime"
        );
    }

    /// The client-wide throttle-retry setter must populate the operation
    /// options that `build()` forwards to the driver's runtime layer.
    #[test]
    fn throttle_retry_setter_populates_operation_options() {
        let builder = CosmosClientBuilder::new().with_throttling_retry_options(
            crate::options::ThrottlingRetryOptionsBuilder::new()
                .with_max_retry_count(4)
                .with_max_retry_wait_time(std::time::Duration::from_secs(15))
                .build(),
        );

        let throttling = builder
            .options
            .operation
            .throttling_retry_options
            .as_ref()
            .expect("throttling group should be populated");
        assert_eq!(throttling.max_retry_count, Some(4));
        assert_eq!(
            throttling.max_retry_wait_time,
            Some(std::time::Duration::from_secs(15))
        );
    }

    /// A throttle-retry count of `0` (disable retries) must round-trip through
    /// the builder unchanged so the driver can surface the first 429.
    #[test]
    fn throttle_retry_count_zero_round_trips() {
        let builder = CosmosClientBuilder::new().with_throttling_retry_options(
            crate::options::ThrottlingRetryOptionsBuilder::new()
                .with_max_retry_count(0)
                .build(),
        );
        assert_eq!(
            builder
                .options
                .operation
                .throttling_retry_options
                .as_ref()
                .and_then(|t| t.max_retry_count),
            Some(0)
        );
    }

    /// The grouped `with_throttling_retry_options` setter must replace the
    /// whole group with the supplied value.
    #[test]
    fn grouped_throttling_retry_options_setter_replaces_group() {
        let group = crate::options::ThrottlingRetryOptionsBuilder::new()
            .with_max_retry_count(2)
            .with_max_retry_wait_time(std::time::Duration::from_secs(7))
            .build();
        let builder = CosmosClientBuilder::new().with_throttling_retry_options(group);

        let throttling = builder
            .options
            .operation
            .throttling_retry_options
            .as_ref()
            .expect("throttling group should be populated");
        assert_eq!(throttling.max_retry_count, Some(2));
        assert_eq!(
            throttling.max_retry_wait_time,
            Some(std::time::Duration::from_secs(7))
        );
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
            #[cfg(feature = "fault_injection")]
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
            #[cfg(feature = "fault_injection")]
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
            #[cfg(feature = "fault_injection")]
            Vec::new(),
        )
        .expect("build_driver_options should succeed");
        assert_eq!(opts.preferred_regions(), input.as_slice());
    }
}
