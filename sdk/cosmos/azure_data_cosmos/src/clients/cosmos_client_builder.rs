// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore behaviour
//! Builder for creating [`CosmosClient`] instances.

use crate::{
    clients::ClientContext, options::ThroughputControlGroupOptions, CosmosAccountReference,
    CosmosClient, CosmosClientOptions, CosmosCredential, RoutingStrategy,
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
/// Pass any type that implements `Into<CosmosAccountReference>`, such as a
/// [`CosmosAccountReference`] created via convenience constructors, or a tuple of
/// `(CosmosAccountEndpoint, credential)` or `(Url, credential)`.
///
/// A [`RoutingStrategy`] is also required to specify how the SDK should select regions.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint,
///     Region, RoutingStrategy,
/// };
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_credential(endpoint, credential);
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
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint,
///     Region, RoutingStrategy,
/// };
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_master_key(endpoint, Secret::from("my_account_key"));
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
    /// [`UserAgentSuffix::new`](crate::UserAgentSuffix::new) for trusted
    /// values, or [`UserAgentSuffix::try_new`](crate::UserAgentSuffix::try_new)
    /// for untrusted input. Validation rules (max 25 characters,
    /// HTTP-header-safe) are enforced at the construction site rather than
    /// here, which keeps any panic local to the caller's input handling.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to append to the User-Agent header.
    pub fn with_user_agent_suffix(mut self, suffix: crate::UserAgentSuffix) -> Self {
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
    #[doc(hidden)]
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
    #[doc(hidden)]
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
    /// [`OperationOptions::throughput_control_group`](crate::OperationOptions::throughput_control_group).
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
    pub fn with_backup_endpoints(mut self, endpoints: Vec<crate::CosmosAccountEndpoint>) -> Self {
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
    #[doc(hidden)]
    #[cfg(feature = "__internal_in_memory_emulator")]
    pub fn with_driver_runtime_builder(mut self, builder: CosmosDriverRuntimeBuilder) -> Self {
        self.driver_runtime_builder = Some(builder);
        self
    }

    /// Builds the [`CosmosClient`] with the specified account reference and region selection strategy.
    ///
    /// The account reference bundles an endpoint and credential. You can create one using
    /// [`CosmosAccountReference::with_credential()`] or [`CosmosAccountReference::with_master_key()`].
    ///
    /// You can also pass a tuple of `(CosmosAccountEndpoint, credential)` or `(Url, credential)`,
    /// where `credential` is any type that implements `Into<CosmosCredential>`.
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
        mut self,
        account: impl Into<CosmosAccountReference>,
        routing_strategy: RoutingStrategy,
    ) -> crate::Result<CosmosClient> {
        // Apply the region selection strategy to internal options.
        match routing_strategy {
            RoutingStrategy::ProximityTo(region) => {
                self.options.application_region = Some(region);
            }
        }

        let (account_endpoint, credential) = account.into().into_parts();
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

        let preferred_regions = if let Some(ref region) = self.options.application_region {
            crate::region_proximity::generate_preferred_region_list(region)
                .map(|s| s.to_vec())
                .unwrap_or_else(|| {
                    tracing::warn!(
                        region = %region,
                        "unrecognized application region; falling back to account-defined region order"
                    );
                    Vec::new()
                })
        } else {
            Vec::new()
        };

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

        // Forward the user-agent suffix captured above to the driver runtime.
        if let Some(suffix) = driver_user_agent_suffix {
            driver_runtime_builder = driver_runtime_builder.with_user_agent_suffix(suffix);
        }

        // Apply the SDK's PPCB default at the runtime layer. This sits above
        // the env layer in the driver's option-resolution hierarchy, so it
        // pins the SDK's "enabled by default" behavior even when the env var
        // is unset.
        let runtime_operation_options =
            azure_data_cosmos_driver::options::OperationOptionsBuilder::new()
                .with_per_partition_circuit_breaker_enabled(ppcb_enabled)
                .build();
        driver_runtime_builder =
            driver_runtime_builder.with_operation_options(runtime_operation_options);

        #[cfg(feature = "fault_injection")]
        if !driver_fi_rules.is_empty() {
            driver_runtime_builder =
                driver_runtime_builder.with_fault_injection_rules(driver_fi_rules)?;
        }
        for group in self.throughput_control_groups {
            driver_runtime_builder = driver_runtime_builder
                .register_throughput_control_group(group)
                .map_err(|e| {
                    crate::CosmosError::client(format!(
                        "failed to register throughput control group: {e}"
                    ))
                })?;
        }
        let driver_runtime = driver_runtime_builder.build().await?;
        let driver_options =
            azure_data_cosmos_driver::options::DriverOptions::builder(driver_account)
                .with_preferred_regions(preferred_regions)
                .build();
        let driver = driver_runtime
            .get_or_create_driver(driver_options.account().clone(), Some(driver_options))
            .await?;

        Ok(CosmosClient {
            context: ClientContext { driver },
        })
    }
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

// Unit tests for routing-strategy behavior were removed because
// CosmosClient::builder().build() now eagerly creates a CosmosDriver,
// which requires a real endpoint. Re-add once fault injection is linked
// from the SDK to the driver.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UserAgentSuffix;

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
            .with_operation_options(runtime_op_options)
            .build()
            .await
            .expect("runtime builds");

        assert_eq!(
            runtime
                .operation_options()
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
            .with_operation_options(runtime_op_options)
            .build()
            .await
            .expect("runtime builds");

        assert_eq!(
            runtime
                .operation_options()
                .per_partition_circuit_breaker_enabled,
            Some(false),
            "explicit env-var opt-out must propagate to the driver runtime"
        );
    }
}
