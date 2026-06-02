// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime environment.

use azure_core::http::ClientOptions;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use crate::{
    diagnostics::ProxyConfiguration,
    models::{
        normalize_wrapping_sdk_identifier, AccountReference, ContainerReference,
        ThroughputControlGroupName, UserAgent,
    },
    options::{
        parse_duration_millis_from_env, ConnectionPoolOptions, CorrelationId, DriverOptions,
        OperationOptions, ThroughputControlGroupOptions, ThroughputControlGroupRegistry,
        UserAgentSuffix, WorkloadId,
    },
    system::{CpuMemoryMonitor, VmMetadataService},
};

use super::cache::{AccountMetadataCache, ContainerCache};
use super::{
    transport::{
        http_client_factory::{DefaultHttpClientFactory, HttpClientFactory},
        CosmosTransport,
    },
    CosmosDriver,
};

/// The Cosmos DB driver runtime environment.
///
/// A runtime represents the global configuration shared across all drivers
/// and connections. It includes connection pool settings, default operation options,
/// and manages singleton driver instances per account.
///
/// # Thread Safety
///
/// The runtime is thread-safe and can be shared across threads. Driver instances
/// are managed as singletons per account endpoint, ensuring efficient resource usage.
///
/// # Example
///
/// ```no_run
/// use azure_data_cosmos_driver::driver::{
///     CosmosDriverRuntime, CosmosDriverRuntimeBuilder,
/// };
/// use azure_data_cosmos_driver::options::{OperationOptions, OperationOptionsBuilder};
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
/// let operation_options = OperationOptionsBuilder::new()
///     .with_max_failover_retry_count(5)
///     .build();
///
/// let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
///     .with_operation_options(operation_options)
///     .build()
///     .await?;
///
/// // Get or create a driver for an account
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-key",
/// );
///
/// let driver = cosmos_runtime.get_or_create_driver(account, None).await?;
///
/// // Later, replace runtime defaults atomically
/// // cosmos_runtime.set_operation_options(new_options);
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Debug)]
pub struct CosmosDriverRuntime {
    /// Unique ID of the driver runtime internally. Used in traces to identify multi-runtime scenarios.
    id: usize,

    /// Core HTTP client options from azure_core.
    client_options: ClientOptions,

    /// Connection pool configuration for managing TCP connections.
    connection_pool: ConnectionPoolOptions,

    /// Bootstrap HTTP transport for initial metadata probes.
    ///
    /// Uses HTTP/2-only to detect protocol support. Individual drivers
    /// create their own `CosmosTransport` after the probe with the
    /// negotiated HTTP version.
    ///
    /// Kept in `Arc` because drivers seed their `ArcSwap<CosmosTransport>`
    /// from this transport during initialization.
    bootstrap_transport: Arc<CosmosTransport>,

    /// Factory for creating HTTP clients, shared across per-account transports.
    http_client_factory: Arc<dyn HttpClientFactory>,

    /// Environment-level operation options, populated once from env vars at build time.
    env_operation_options: Arc<OperationOptions>,

    /// User-provided default operation options, swappable via interior mutability.
    ///
    /// Wrapped in `RwLock<Arc<...>>` so that shared references can atomically
    /// replace the options while readers obtain a cheap `Arc` snapshot.
    operation_options: RwLock<Arc<OperationOptions>>,

    /// Computed user agent string for HTTP requests.
    ///
    /// This is automatically computed from the SDK version, platform info,
    /// and optional suffix (from user_agent_suffix, workload_id, or correlation_id).
    user_agent: UserAgent,

    /// Workload identifier for resource governance (1-50 if set).
    workload_id: Option<WorkloadId>,

    /// Correlation ID for client-side metrics.
    ///
    /// Used as a dimension for client-side metrics. If cardinality is too high,
    /// this may be ignored by metrics aggregation.
    correlation_id: Option<CorrelationId>,

    /// User agent suffix appended to identify request source.
    ///
    /// If `correlation_id` is not set, this suffix is used as the correlation
    /// dimension for client-side metrics. Server-side cardinality enforcement
    /// is more strict for this field.
    user_agent_suffix: Option<UserAgentSuffix>,

    /// Optional wrapping-SDK identifier prepended to the User-Agent header.
    ///
    /// Set by higher-level SDKs (e.g., `azure_data_cosmos`) so requests can be
    /// attributed to the wrapping SDK in addition to the driver. Example value:
    /// `azsdk-rust-cosmos/0.34.0`. When unset, the User-Agent starts with the
    /// driver's own identifier.
    wrapping_sdk_identifier: Option<String>,

    /// Registry of throughput control groups.
    ///
    /// Groups are registered during builder construction and are immutable after
    /// runtime creation (except for mutable target values within each group).
    throughput_control_groups: ThroughputControlGroupRegistry,

    /// Registry of driver instances keyed by account endpoint.
    ///
    /// Ensures singleton driver per account reference.
    driver_registry: RwLock<HashMap<String, Arc<CosmosDriver>>>,

    /// Shared container metadata cache used by drivers in this runtime.
    container_cache: ContainerCache,

    /// Shared account metadata cache used by drivers in this runtime.
    ///
    /// Kept in `Arc` because it is shared with `LocationStateStore` instances
    /// which independently hold a reference.
    account_metadata_cache: Arc<AccountMetadataCache>,

    /// CPU and memory monitor for diagnostics.
    cpu_monitor: CpuMemoryMonitor,

    /// Machine identifier for diagnostics (VM ID on Azure, generated UUID otherwise).
    ///
    /// Kept in `Arc` because it is cloned into every diagnostics context.
    machine_id: Arc<String>,

    /// Whether fault injection is enabled for this runtime.
    fault_injection_enabled: bool,

    /// Proxy configuration snapshot for diagnostics.
    proxy_configuration: ProxyConfiguration,
}

impl CosmosDriverRuntime {
    /// Returns a new builder for creating a runtime.
    pub fn builder() -> CosmosDriverRuntimeBuilder {
        CosmosDriverRuntimeBuilder::new()
    }

    /// Returns a unique identifier for the runtime, for internal tracing.
    #[expect(dead_code, reason = "will be used when tracing spans are re-added")]
    pub(crate) fn id(&self) -> usize {
        self.id
    }

    /// Returns the HTTP client options.
    pub fn client_options(&self) -> &ClientOptions {
        &self.client_options
    }

    /// Returns the connection pool options.
    pub fn connection_pool(&self) -> &ConnectionPoolOptions {
        &self.connection_pool
    }

    /// Returns the bootstrap transport for initial metadata probes.
    pub(crate) fn bootstrap_transport(&self) -> &Arc<CosmosTransport> {
        &self.bootstrap_transport
    }

    /// Returns the shared HTTP client factory for creating per-account transports.
    pub(crate) fn http_client_factory(&self) -> &Arc<dyn HttpClientFactory> {
        &self.http_client_factory
    }

    /// Returns the shared container cache.
    pub(crate) fn container_cache(&self) -> &ContainerCache {
        &self.container_cache
    }

    /// Returns the shared account metadata cache.
    pub(crate) fn account_metadata_cache(&self) -> &Arc<AccountMetadataCache> {
        &self.account_metadata_cache
    }

    /// Returns the CPU/memory monitor for diagnostics.
    pub(crate) fn cpu_monitor(&self) -> &CpuMemoryMonitor {
        &self.cpu_monitor
    }

    /// Returns the machine identifier for diagnostics.
    pub(crate) fn machine_id(&self) -> &Arc<String> {
        &self.machine_id
    }

    /// Returns whether fault injection is enabled for this runtime.
    pub(crate) fn fault_injection_enabled(&self) -> bool {
        self.fault_injection_enabled
    }

    /// Returns the proxy configuration snapshot.
    ///
    /// Captures whether proxy is allowed and the proxy environment variable
    /// values at client creation time, for diagnostic purposes.
    pub fn proxy_configuration(&self) -> &ProxyConfiguration {
        &self.proxy_configuration
    }

    /// Returns the environment-level operation options (populated from env vars at build time).
    pub fn env_operation_options(&self) -> &Arc<OperationOptions> {
        &self.env_operation_options
    }

    /// Returns a snapshot of the default operation options.
    ///
    /// The returned `Arc` is a cheap clone of the current value.
    /// In-flight readers are unaffected by concurrent calls to
    /// [`set_operation_options`](Self::set_operation_options).
    pub fn operation_options(&self) -> Arc<OperationOptions> {
        // Poisoning is safe to ignore: the write side is an atomic Arc swap with no
        // multi-step mutation, so the value is always in a consistent state.
        self.operation_options
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Replaces the default operation options atomically.
    ///
    /// In-flight operations that already obtained a snapshot via
    /// [`operation_options`](Self::operation_options) are unaffected.
    pub fn set_operation_options(&self, options: OperationOptions) {
        *self
            .operation_options
            .write()
            .unwrap_or_else(|e| e.into_inner()) = Arc::new(options);
    }

    /// Returns the computed user agent string.
    ///
    /// The user agent is automatically computed with a static prefix containing
    /// SDK version and platform info, plus an optional suffix derived from
    /// `user_agent_suffix`, `workload_id`, or `correlation_id` (in priority order).
    pub fn user_agent(&self) -> &UserAgent {
        &self.user_agent
    }

    /// Returns the workload identifier.
    pub fn workload_id(&self) -> Option<WorkloadId> {
        self.workload_id
    }

    /// Returns the correlation ID for client-side metrics.
    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Returns the user agent suffix.
    pub fn user_agent_suffix(&self) -> Option<&UserAgentSuffix> {
        self.user_agent_suffix.as_ref()
    }

    /// Returns the wrapping-SDK identifier, if one was supplied via
    /// [`CosmosDriverRuntimeBuilder::with_wrapping_sdk_identifier`].
    pub fn wrapping_sdk_identifier(&self) -> Option<&str> {
        self.wrapping_sdk_identifier.as_deref()
    }

    /// Returns the effective correlation dimension.
    ///
    /// Returns `correlation_id` if set, otherwise falls back to `user_agent_suffix`.
    pub fn effective_correlation(&self) -> Option<&str> {
        self.correlation_id
            .as_ref()
            .map(|c| c.as_str())
            .or_else(|| self.user_agent_suffix.as_ref().map(|s| s.as_str()))
    }

    /// Returns a throughput control group by container and name.
    ///
    /// This is a convenience method for looking up a specific group.
    pub(crate) fn get_throughput_control_group(
        &self,
        container: &ContainerReference,
        name: &ThroughputControlGroupName,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.throughput_control_groups
            .get_by_container_and_name(container, name)
    }

    /// Returns the default throughput control group for a container.
    ///
    /// Returns `None` if no default group is registered for the container.
    pub(crate) fn get_default_throughput_control_group(
        &self,
        container: &ContainerReference,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.throughput_control_groups
            .get_default_for_container(container)
    }

    /// Gets or creates a driver for the specified account.
    ///
    /// This method ensures singleton behavior - only one driver instance exists
    /// per account endpoint. Subsequent calls with the same account endpoint
    /// return the existing driver.
    ///
    /// # Parameters
    ///
    /// - `account`: The account reference (endpoint + credentials)
    /// - `driver_options`: Optional driver-level options. If not provided, defaults are used.
    ///
    /// # Note
    ///
    /// If a driver already exists for the account, the `driver_options` parameter is ignored.
    /// The existing driver with its original options is returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::options::DriverOptions;
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// // First call creates the driver
    /// let driver = runtime.get_or_create_driver(account.clone(), None).await?;
    ///
    /// // Subsequent calls return the same driver instance
    /// let driver2 = runtime.get_or_create_driver(account, None).await?;
    /// // driver and driver2 point to the same instance
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_or_create_driver(
        self: &Arc<Self>,
        account: AccountReference,
        driver_options: Option<DriverOptions>,
    ) -> crate::error::Result<Arc<CosmosDriver>> {
        let key = account.endpoint().to_string();

        // Fast path: return an already-initialized driver.
        {
            let registry = self.driver_registry.read().unwrap();
            if let Some(driver) = registry.get(&key) {
                tracing::trace!("retrieved existing driver");
                return Ok(driver.clone());
            }
        }

        tracing::trace!("creating new driver");

        // Slow path: create and initialize the driver *before* inserting into
        // the registry. This ensures concurrent callers never observe an
        // uninitialized driver. If two callers race, both will probe — but the
        // first to finish inserts; the second discovers the existing entry and
        // drops its duplicate.
        let options = driver_options.unwrap_or_else(|| DriverOptions::builder(account).build());
        let driver = Arc::new(CosmosDriver::new(Arc::clone(self), options));

        driver.initialize().await?;

        let mut registry = self.driver_registry.write().unwrap();
        let entry = registry.entry(key).or_insert_with(|| driver.clone());
        Ok(entry.clone())
    }
}

/// Builder for creating [`CosmosDriverRuntime`].
///
/// Use `OperationOptionsBuilder` to create operation options, then pass them
/// to this builder via [`with_operation_options()`](Self::with_operation_options).
///
/// # User Agent
///
/// The user agent string is automatically computed with a static prefix containing
/// SDK version and platform info. The suffix is derived from (in priority order):
/// 1. [`with_user_agent_suffix()`](Self::with_user_agent_suffix) if set
/// 2. [`with_workload_id()`](Self::with_workload_id) if set (formatted as `w{id}`)
/// 3. [`with_correlation_id()`](Self::with_correlation_id) if set
/// 4. No suffix (base user agent only)
///
/// If [`with_wrapping_sdk_identifier()`](Self::with_wrapping_sdk_identifier) is
/// set, its value is prepended to the prefix so requests can be attributed to
/// both the wrapping SDK and the driver.
///
/// # Throughput Control Groups
///
/// Throughput control groups must be registered during builder construction.
/// Once `build()` is called, the set of groups is immutable (though mutable
/// values within each group can still be updated).
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct CosmosDriverRuntimeBuilder {
    client_options: Option<ClientOptions>,
    connection_pool: Option<ConnectionPoolOptions>,
    operation_options: Option<OperationOptions>,
    workload_id: Option<WorkloadId>,
    correlation_id: Option<CorrelationId>,
    user_agent_suffix: Option<UserAgentSuffix>,
    wrapping_sdk_identifier: Option<String>,
    throughput_control_groups: ThroughputControlGroupRegistry,
    cpu_refresh_interval: Option<Duration>,
    #[cfg(feature = "fault_injection")]
    fault_injection_rules: Option<Vec<std::sync::Arc<crate::fault_injection::FaultInjectionRule>>>,
    #[cfg(any(
        test,
        feature = "__internal_in_memory_emulator",
        feature = "__internal_mocking"
    ))]
    http_client_factory: Option<Arc<dyn HttpClientFactory>>,
}

impl CosmosDriverRuntimeBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the HTTP client options.
    pub fn with_client_options(mut self, options: ClientOptions) -> Self {
        self.client_options = Some(options);
        self
    }

    /// Sets the connection pool options.
    pub fn with_connection_pool(mut self, options: ConnectionPoolOptions) -> Self {
        self.connection_pool = Some(options);
        self
    }

    /// Sets the operation options (defaults for operations at the runtime layer).
    ///
    /// Use `OperationOptionsBuilder` to create the operation options.
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation_options = Some(options);
        self
    }

    /// Sets the workload identifier (must be 1-50 if set).
    ///
    /// The workload ID is used as a fallback for the user agent suffix
    /// if [`with_user_agent_suffix()`](Self::with_user_agent_suffix) is not set.
    pub fn with_workload_id(mut self, workload_id: WorkloadId) -> Self {
        self.workload_id = Some(workload_id);
        self
    }

    /// Sets the correlation ID for client-side metrics.
    ///
    /// The correlation ID is used as a fallback for the user agent suffix
    /// if neither [`with_user_agent_suffix()`](Self::with_user_agent_suffix) nor
    /// [`with_workload_id()`](Self::with_workload_id) is set.
    ///
    /// # Cardinality Warning
    ///
    /// If the cardinality of correlation IDs is too high, metrics aggregation
    /// may ignore this dimension. Choose values with moderate cardinality
    /// (e.g., cluster names, environment identifiers, deployment IDs).
    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Sets the user agent suffix.
    ///
    /// If `correlation_id` is not set, this suffix is used as the correlation
    /// dimension for client-side metrics.
    ///
    /// # Server-Side Enforcement
    ///
    /// The Cosmos DB service enforces cardinality limits more strictly for
    /// user agent suffixes. High-cardinality suffixes may be rejected.
    ///
    /// Good examples: AKS cluster name, Azure VM ID (if limited nodes),
    /// app name with region.
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    /// Sets a wrapping-SDK identifier prepended to the User-Agent header.
    ///
    /// Higher-level SDKs (such as `azure_data_cosmos`) call this to identify
    /// themselves alongside the driver. The supplied value should already be a
    /// complete token (e.g., `azsdk-rust-cosmos/0.34.0`); the driver only
    /// sanitizes non-ASCII characters and trims whitespace. An empty or
    /// whitespace-only value is treated as unset and clears any previously
    /// configured identifier.
    ///
    /// When set, the User-Agent looks like:
    /// `azsdk-rust-cosmos/0.34.0 azsdk-rust-cosmos-driver/0.3.0 linux/x86_64 rustc/1.85.0`
    pub fn with_wrapping_sdk_identifier(mut self, identifier: impl Into<String>) -> Self {
        let raw = identifier.into();
        self.wrapping_sdk_identifier = normalize_wrapping_sdk_identifier(&raw);
        self
    }

    /// Sets the CPU/memory monitoring refresh interval.
    ///
    /// Controls how frequently the background CPU and memory sampling thread
    /// collects new data points. If not set, the value is read from the
    /// `AZURE_COSMOS_CPU_REFRESH_INTERVAL_MS` environment variable. If the
    /// environment variable is also absent, the default of 5000 ms is used.
    ///
    /// Valid range: 1000–60000 ms (1–60 seconds).
    pub fn with_cpu_refresh_interval(mut self, interval: Duration) -> Self {
        self.cpu_refresh_interval = Some(interval);
        self
    }

    #[cfg(any(test, feature = "__internal_in_memory_emulator"))]
    pub(crate) fn with_http_client_factory(mut self, factory: Arc<dyn HttpClientFactory>) -> Self {
        self.http_client_factory = Some(factory);
        self
    }

    /// Sets a custom HTTP client factory, replacing the default reqwest-based transport.
    ///
    /// **Unsupported internal API** — only available under the `__internal_mocking` feature
    /// flag. Intended for benchmarks and test harnesses; no stability guarantees.
    #[cfg(feature = "__internal_mocking")]
    pub fn with_mock_http_client_factory(mut self, factory: Arc<dyn HttpClientFactory>) -> Self {
        self.http_client_factory = Some(factory);
        self
    }

    /// Registers a throughput control group.
    ///
    /// Groups are identified by the combination of container reference and group name.
    /// At most one group per container can be marked as `is_default = true`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A group with the same (container, name) key already exists
    /// - Another group is already marked as default for the same container
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder;
    /// use azure_data_cosmos_driver::options::{PriorityLevel, ThroughputControlGroupOptions};
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// // Build the runtime first, then resolve the container via the driver.
    /// let runtime = CosmosDriverRuntimeBuilder::new().build().await?;
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    /// let container = driver.resolve_container("mydb", "mycollection").await?;
    ///
    /// // Register a throughput control group on a new runtime builder.
    /// let runtime = CosmosDriverRuntimeBuilder::new()
    ///     .register_throughput_control_group(
    ///         ThroughputControlGroupOptions::new(
    ///             "default-group",
    ///             container.clone(),
    ///             true, // is_default
    ///         )
    ///         .with_priority_level(PriorityLevel::High)
    ///     )?
    ///     .build()
    ///     .await;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Sets the fault injection rules for testing.
    ///
    /// When set, all HTTP clients created by the transport layer will
    /// evaluate these rules before delegating to the real transport
    /// (per Transport Pipeline Spec §7).
    /// Appends the supplied rules to any rules already configured on this
    /// builder (additive). Calling this multiple times accumulates rules in
    /// insertion order. Mirrors the additive semantics of
    /// [`Self::register_throughput_control_group`] so callers that compose a
    /// runtime builder from multiple sources (e.g. the
    /// `azure_data_cosmos::CosmosClientBuilder` adding its own rules on top of
    /// a user-supplied builder) do not silently lose previously-configured
    /// rules.
    ///
    /// # Errors
    ///
    /// Returns `Err` when any rule's `id` collides with another rule already
    /// configured on this builder, or with another rule in the same call.
    /// Rule ids identify a fault injection rule across reconfigure /
    /// disable / re-enable operations, so silently keeping one of two rules
    /// with the same id would surface as "my fault injection didn't fire"
    /// long after the duplicate was introduced — and depend on insertion
    /// order. Surfacing the collision at builder time keeps the failure
    /// local to the misconfiguration.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection_rules(
        mut self,
        rules: Vec<std::sync::Arc<crate::fault_injection::FaultInjectionRule>>,
    ) -> crate::error::Result<Self> {
        if rules.is_empty() {
            return Ok(self);
        }

        // Build the set of ids already present so collisions across both
        // (existing-vs-new) and (new-vs-new) are caught in one pass.
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

    /// Builds the [`CosmosDriverRuntime`].
    ///
    /// The user agent is computed from (in priority order):
    /// 1. `user_agent_suffix` if set
    /// 2. `workload_id` if set (formatted as `w{id}`)
    /// 3. `correlation_id` if set
    /// 4. No suffix (base user agent only)
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP transport cannot be created (e.g., TLS
    /// configuration failure).
    ///
    pub async fn build(self) -> crate::error::Result<Arc<CosmosDriverRuntime>> {
        // Compute user agent from suffix/workloadId/correlationId (in priority order),
        // optionally prepending a wrapping-SDK identifier.
        let wrapping = self.wrapping_sdk_identifier.as_deref();
        let user_agent = if let Some(ref suffix) = self.user_agent_suffix {
            UserAgent::from_suffix(wrapping, suffix)
        } else if let Some(workload_id) = self.workload_id {
            UserAgent::from_workload_id(wrapping, workload_id)
        } else if let Some(ref correlation_id) = self.correlation_id {
            UserAgent::from_correlation_id(wrapping, correlation_id)
        } else {
            UserAgent::from_wrapping_sdk_identifier(wrapping)
        };

        let connection_pool = self.connection_pool.unwrap_or_default();
        let proxy_configuration = ProxyConfiguration::from_env(connection_pool.proxy_allowed());
        #[allow(unused_mut)]
        let mut fault_injection_enabled = false;
        let http_client_factory: Arc<dyn HttpClientFactory> = {
            let base_factory: Arc<dyn HttpClientFactory> = {
                #[cfg(any(
                    test,
                    feature = "__internal_in_memory_emulator",
                    feature = "__internal_mocking"
                ))]
                {
                    self.http_client_factory
                        .unwrap_or_else(|| Arc::new(DefaultHttpClientFactory::new()))
                }

                #[cfg(not(any(
                    test,
                    feature = "__internal_in_memory_emulator",
                    feature = "__internal_mocking"
                )))]
                {
                    Arc::new(DefaultHttpClientFactory::new())
                }
            };

            #[cfg(feature = "fault_injection")]
            {
                if let Some(rules) = self.fault_injection_rules {
                    fault_injection_enabled = true;
                    Arc::new(
                        crate::fault_injection::FaultInjectingHttpClientFactory::new(
                            base_factory,
                            rules,
                        ),
                    )
                } else {
                    base_factory
                }
            }

            #[cfg(not(feature = "fault_injection"))]
            {
                base_factory
            }
        };

        // Bootstrap transport: lightweight metadata-only transport for the
        // initial HTTP version probe. Uses an unsharded client (no per-endpoint
        // shard pools, no background health sweep) since it only performs
        // one-shot metadata requests during driver initialization.
        let bootstrap_version = if connection_pool.is_http2_allowed() {
            crate::diagnostics::TransportHttpVersion::Http2
        } else {
            crate::diagnostics::TransportHttpVersion::Http11
        };
        let bootstrap_transport = Arc::new(CosmosTransport::bootstrap_metadata_only(
            connection_pool.clone(),
            http_client_factory.clone(),
            bootstrap_version,
        )?);

        // Initialize system monitoring singletons.
        // CpuMemoryMonitor starts a background thread on first call;
        // VmMetadataService makes a single IMDS request (or falls back to a UUID).
        let refresh_interval = parse_duration_millis_from_env(
            self.cpu_refresh_interval,
            "AZURE_COSMOS_CPU_REFRESH_INTERVAL_MS",
            5_000,
            1_000,
            60_000,
        )?;
        let cpu_monitor = CpuMemoryMonitor::get_or_init(refresh_interval);
        let vm_metadata = VmMetadataService::get_or_init().await;

        Ok(Arc::new(CosmosDriverRuntime {
            id: NEXT_RUNTIME_ID.fetch_add(1, Ordering::Relaxed),
            client_options: self.client_options.unwrap_or_default(),
            connection_pool,
            bootstrap_transport,
            http_client_factory,
            env_operation_options: Arc::new(OperationOptions::from_env()),
            operation_options: RwLock::new(Arc::new(self.operation_options.unwrap_or_default())),
            user_agent,
            workload_id: self.workload_id,
            correlation_id: self.correlation_id,
            user_agent_suffix: self.user_agent_suffix,
            wrapping_sdk_identifier: self.wrapping_sdk_identifier,
            throughput_control_groups: self.throughput_control_groups,
            driver_registry: RwLock::new(HashMap::new()),
            container_cache: ContainerCache::new(),
            account_metadata_cache: Arc::new(AccountMetadataCache::new()),
            cpu_monitor,
            machine_id: Arc::new(vm_metadata.machine_id().to_owned()),
            fault_injection_enabled,
            proxy_configuration,
        }))
    }
}

static NEXT_RUNTIME_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn get_or_create_driver_removes_failed_initialization_from_registry() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "***not-base64***",
        );

        let error = runtime
            .get_or_create_driver(account.clone(), None)
            .await
            .expect_err("invalid signing key should fail initialization");
        assert!(!error.to_string().is_empty());
        assert!(runtime.driver_registry.read().unwrap().is_empty());

        let second_error = runtime
            .get_or_create_driver(account, None)
            .await
            .expect_err("failed initialization should not poison the driver registry");
        assert!(!second_error.to_string().is_empty());
        assert!(runtime.driver_registry.read().unwrap().is_empty());
    }

    /// Verifies that the user-agent suffix set on the runtime appears in the
    /// `User-Agent` HTTP header that the driver attaches to outgoing requests.
    ///
    /// This mirrors how `fetch_account_properties_with_runtime` (and other
    /// driver internals) build the header value: they call
    /// `runtime.user_agent().as_str()` and pass it to `apply_cosmos_headers`.
    ///
    /// Coverage for `runtime.user_agent().suffix()` itself is provided by
    /// `cosmos_driver::tests::user_agent_computed_from_suffix`; this test
    /// covers the additional hop into the request headers, which is otherwise
    /// untested.
    #[tokio::test]
    async fn user_agent_suffix_appears_in_request_headers() {
        use crate::driver::transport::{
            cosmos_headers::apply_cosmos_headers, cosmos_transport_client::HttpRequest,
        };
        use azure_core::http::headers::{HeaderValue, Headers, USER_AGENT};
        use azure_core::http::Method;

        let suffix = UserAgentSuffix::try_new("test-app").expect("valid suffix");
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_user_agent_suffix(suffix)
            .build()
            .await
            .unwrap();

        // Replicate the header construction used in driver request paths.
        let user_agent_hv = HeaderValue::from(runtime.user_agent().as_str().to_owned());
        let mut request = HttpRequest {
            url: Url::parse("https://test.documents.azure.com/").unwrap(),
            method: Method::Get,
            headers: Headers::new(),
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };
        apply_cosmos_headers(&mut request, &user_agent_hv);

        let header_value = request
            .headers
            .get_optional_str(&USER_AGENT)
            .expect("User-Agent header should be set by apply_cosmos_headers");
        assert!(
            header_value.contains("test-app"),
            "User-Agent header '{header_value}' should contain the suffix 'test-app'"
        );
    }

    /// `with_wrapping_sdk_identifier` is documented to treat empty or
    /// whitespace-only input as unset and to strip non-ASCII. Verify that the
    /// `wrapping_sdk_identifier()` accessor reflects that normalization so the
    /// runtime view stays consistent with the rendered `User-Agent`.
    #[tokio::test]
    async fn wrapping_sdk_identifier_is_normalized_at_set_time() {
        // Empty / whitespace-only → cleared.
        for raw in ["", "   ", "\t\n"] {
            let runtime = CosmosDriverRuntimeBuilder::new()
                .with_wrapping_sdk_identifier(raw)
                .build()
                .await
                .unwrap();
            assert!(
                runtime.wrapping_sdk_identifier().is_none(),
                "expected wrapping identifier {raw:?} to normalize to None",
            );
            assert!(
                runtime
                    .user_agent()
                    .as_str()
                    .starts_with("azsdk-rust-cosmos-driver/"),
                "User-Agent should not start with empty wrapping prefix: {}",
                runtime.user_agent().as_str(),
            );
        }

        // Non-ASCII trimmed and replaced; surrounding whitespace stripped.
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_wrapping_sdk_identifier("  azsdk-rust-café/1.0  ")
            .build()
            .await
            .unwrap();
        let identifier = runtime
            .wrapping_sdk_identifier()
            .expect("wrapping identifier should be set");
        assert_eq!(identifier, "azsdk-rust-caf_/1.0");
        assert!(
            runtime
                .user_agent()
                .as_str()
                .starts_with("azsdk-rust-caf_/1.0 azsdk-rust-cosmos-driver/"),
            "unexpected User-Agent: {}",
            runtime.user_agent().as_str(),
        );
    }
}
