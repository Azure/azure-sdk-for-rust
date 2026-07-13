// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime environment.

use azure_core::http::ClientOptions;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use crate::{
    diagnostics::ProxyConfiguration,
    models::{normalize_wrapping_sdk_identifier, UserAgent, UserAgentFeatureFlags},
    options::{
        parse_duration_millis_from_env, ConnectionPoolOptions, CorrelationId, DiagnosticsOptions,
        DriverOptions, OperationOptions, PartitionFailoverOptions, UserAgentSuffix, WorkloadId,
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
/// The runtime is thread-safe and can be shared across threads. Each call to
/// [`create_driver`](Self::create_driver) produces a fresh [`CosmosDriver`];
/// drivers built from the same runtime share its long-lived resources
/// (bootstrap transport, metadata caches, CPU monitor, etc.).
///
/// # Example
///
/// ```no_run
/// use azure_data_cosmos_driver::driver::{
///     CosmosDriverRuntime, CosmosDriverRuntimeBuilder,
/// };
/// use azure_data_cosmos_driver::options::{
///     DriverOptions, OperationOptions, OperationOptionsBuilder,
/// };
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
/// let operation_options = OperationOptionsBuilder::new()
///     .with_max_failover_retry_count(5)
///     .build();
///
/// let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
///     .with_default_operation_options(operation_options)
///     .build()
///     .await?;
///
/// // Create a driver for an account
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-key",
/// );
///
/// let driver = cosmos_runtime
///     .create_driver(DriverOptions::builder(account).build())
///     .await?;
///
/// // Later, replace runtime defaults atomically
/// // cosmos_runtime.set_default_operation_options(new_options);
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

    /// Diagnostics output configuration captured at runtime initialization.
    diagnostics_options: Arc<DiagnosticsOptions>,

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

    /// Highest-priority kill-switch operation options, populated once from the
    /// `{ENV}_OVERRIDE` variants at build time. Only `overridable` fields are
    /// populated; this layer wins over every other layer (including
    /// per-operation values).
    env_override_operation_options: Arc<OperationOptions>,

    /// User-provided default operation options, swappable via interior mutability.
    ///
    /// Wrapped in `RwLock<Arc<...>>` so that shared references can atomically
    /// replace the options while readers obtain a cheap `Arc` snapshot.
    operation_options: RwLock<Arc<OperationOptions>>,

    /// Computed user agent string for HTTP requests.
    ///
    /// This is automatically computed from the SDK version, platform info,
    /// and optional suffix (from user_agent_suffix, workload_id, or correlation_id).
    /// Stored as an `Arc` so drivers without a per-driver UA override can clone
    /// it cheaply and stamp requests with the same shared value.
    user_agent: Arc<UserAgent>,

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

    /// Cross-SDK feature flags advertised in this runtime's base `User-Agent`.
    ///
    /// Computed from runtime-scoped client configuration (HTTP/2 transport and
    /// the default per-partition circuit breaker setting). Drivers built from
    /// this runtime compare their own computed flags against this value: when
    /// they match (the common case), they share the runtime's `Arc<UserAgent>`;
    /// otherwise they recompute their own `User-Agent`.
    user_agent_feature_flags: UserAgentFeatureFlags,

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

    /// Returns the diagnostics output options.
    pub fn diagnostics_options(&self) -> &DiagnosticsOptions {
        &self.diagnostics_options
    }

    /// Returns the shared diagnostics output options.
    pub(crate) fn diagnostics_options_arc(&self) -> &Arc<DiagnosticsOptions> {
        &self.diagnostics_options
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

    /// Returns the highest-priority kill-switch operation options (populated
    /// from the `{ENV}_OVERRIDE` variants at build time).
    pub fn env_override_operation_options(&self) -> &Arc<OperationOptions> {
        &self.env_override_operation_options
    }

    /// Returns a snapshot of the default operation options.
    ///
    /// The returned `Arc` is a cheap clone of the current value.
    /// In-flight readers are unaffected by concurrent calls to
    /// [`set_default_operation_options`](Self::set_default_operation_options).
    pub fn default_operation_options(&self) -> Arc<OperationOptions> {
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
    /// [`default_operation_options`](Self::default_operation_options) are
    /// unaffected.
    pub fn set_default_operation_options(&self, options: OperationOptions) {
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
    ///
    /// Stored as an `Arc` so [`CosmosDriver`](super::CosmosDriver) instances
    /// without a per-driver suffix override can clone this shared value
    /// instead of recomputing the User-Agent.
    pub fn user_agent(&self) -> &Arc<UserAgent> {
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

    /// Returns the cross-SDK feature flags advertised in this runtime's base
    /// `User-Agent` header.
    pub(crate) fn user_agent_feature_flags(&self) -> UserAgentFeatureFlags {
        self.user_agent_feature_flags
    }

    /// Recomputes a `User-Agent` from this runtime's suffix source (suffix,
    /// workload id, or correlation id, in priority order) plus the supplied
    /// feature flags.
    ///
    /// Used by a driver that overrode a feature-affecting option (e.g. disabled
    /// PPCB) without supplying its own suffix, so it cannot share the runtime's
    /// shared `Arc<UserAgent>`.
    pub(crate) fn user_agent_with_feature_flags(
        &self,
        feature_flags: UserAgentFeatureFlags,
    ) -> UserAgent {
        compute_user_agent(
            self.wrapping_sdk_identifier.as_deref(),
            self.user_agent_suffix.as_ref(),
            self.workload_id,
            self.correlation_id.as_ref(),
            feature_flags,
        )
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

    /// Creates a fresh driver bound to this runtime.
    ///
    /// Each call returns a new [`CosmosDriver`] — the runtime no longer caches
    /// drivers by account endpoint. Callers that want a single driver per
    /// account must hold onto the returned `Arc` themselves. Drivers built from
    /// the same runtime share runtime-owned resources (bootstrap transport,
    /// account-metadata cache, container cache, CPU monitor, etc.).
    ///
    /// # Parameters
    ///
    /// - `driver_options`: Driver-level options, including the account reference.
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
    /// let driver = runtime
    ///     .create_driver(DriverOptions::builder(account).build())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_driver(
        self: &Arc<Self>,
        driver_options: DriverOptions,
    ) -> crate::error::Result<Arc<CosmosDriver>> {
        tracing::trace!("creating new driver");
        let driver = Arc::new(CosmosDriver::new(Arc::clone(self), driver_options)?);
        driver.initialize().await?;
        Ok(driver)
    }
}

/// Builder for creating [`CosmosDriverRuntime`].
///
/// Use `OperationOptionsBuilder` to create operation options, then pass them
/// to this builder via [`with_default_operation_options()`](Self::with_default_operation_options).
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
    diagnostics_options: Option<DiagnosticsOptions>,
    operation_options: Option<OperationOptions>,
    workload_id: Option<WorkloadId>,
    correlation_id: Option<CorrelationId>,
    user_agent_suffix: Option<UserAgentSuffix>,
    wrapping_sdk_identifier: Option<String>,
    cpu_refresh_interval: Option<Duration>,
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

    /// Sets diagnostics output options captured by this runtime.
    ///
    /// If unset, the runtime reads `AZURE_COSMOS_DIAGNOSTICS_*` environment
    /// variables and falls back to [`DiagnosticsOptions::default`].
    pub fn with_diagnostics_options(mut self, options: DiagnosticsOptions) -> Self {
        self.diagnostics_options = Some(options);
        self
    }

    /// Sets the default operation options at the runtime layer.
    ///
    /// These act as the lowest-priority layer in the option-resolution
    /// hierarchy (per-op → per-driver → runtime → env → built-in default).
    /// Use `OperationOptionsBuilder` to create the operation options.
    pub fn with_default_operation_options(mut self, options: OperationOptions) -> Self {
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
        let connection_pool = self.connection_pool.unwrap_or_default();
        let diagnostics_options = Arc::new(self.diagnostics_options.unwrap_or_default());

        // Compute the base feature flags advertised in the User-Agent from
        // runtime-scoped client configuration. HTTP/2 comes from the connection
        // pool; PPCB uses the driver default (its per-driver value is folded in
        // later by `CosmosDriver::new`). PPAF is server-driven per-partition and
        // therefore unknown here, so it is not advertised in the shared header.
        let user_agent_feature_flags = UserAgentFeatureFlags::from_client_config(
            connection_pool.is_http2_allowed(),
            PartitionFailoverOptions::default().circuit_breaker_enabled(),
        );

        // Compute user agent from suffix/workloadId/correlationId (in priority order),
        // optionally prepending a wrapping-SDK identifier.
        let user_agent = Arc::new(compute_user_agent(
            self.wrapping_sdk_identifier.as_deref(),
            self.user_agent_suffix.as_ref(),
            self.workload_id,
            self.correlation_id.as_ref(),
            user_agent_feature_flags,
        ));

        let proxy_configuration = ProxyConfiguration::from_env(connection_pool.proxy_allowed());
        let http_client_factory: Arc<dyn HttpClientFactory> = {
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
        // The CPU-refresh interval resolves `builder → env → default` (with
        // bounds validation) through the same `parse_duration_millis_from_env`
        // helper the driver-level option builders use (e.g.
        // `PartitionFailoverOptions`), keeping every duration env var on one path.
        let refresh_interval = parse_duration_millis_from_env(
            self.cpu_refresh_interval,
            "AZURE_COSMOS_CPU_REFRESH_INTERVAL_MS",
            5_000,
            1_000,
            60_000,
            &|k| std::env::var(k).ok(),
        )?;
        let cpu_monitor = CpuMemoryMonitor::get_or_init(refresh_interval);
        let vm_metadata = VmMetadataService::get_or_init().await;

        Ok(Arc::new(CosmosDriverRuntime {
            id: NEXT_RUNTIME_ID.fetch_add(1, Ordering::Relaxed),
            client_options: self.client_options.unwrap_or_default(),
            connection_pool,
            diagnostics_options,
            bootstrap_transport,
            http_client_factory,
            env_operation_options: Arc::new(OperationOptions {
                // INVARIANT — when adding a new `#[option(nested)]` field to
                // `OperationOptions`, you MUST add an explicit
                // `<NestedType>::from_env()` call here under a matching field
                // initializer. The `CosmosOptions` derive macro's
                // `from_env_vars` does *not* recurse into nested option
                // groups (today; tracked as a macro follow-up to fix this
                // ergonomically — see the comment in
                // `azure_data_cosmos_macros/src/env.rs`). Skipping the
                // explicit call here silently drops the nested group's env
                // vars at the env layer, which surfaces only as "per-env
                // overrides for the new group are ignored" at runtime — no
                // compile-time guard catches it.
                throttling_retry_options: Some(crate::options::ThrottlingRetryOptions::from_env()),
                ..OperationOptions::from_env()
            }),
            // Kill-switch layer: only `overridable` fields (read from their
            // `{ENV}_OVERRIDE` variants) are populated. No nested groups carry
            // overridable fields today, so no explicit nested call is needed
            // here (unlike `env_operation_options` above).
            env_override_operation_options: Arc::new(OperationOptions::from_env_override()),
            operation_options: RwLock::new(Arc::new(self.operation_options.unwrap_or_default())),
            user_agent,
            workload_id: self.workload_id,
            correlation_id: self.correlation_id,
            user_agent_suffix: self.user_agent_suffix,
            wrapping_sdk_identifier: self.wrapping_sdk_identifier,
            user_agent_feature_flags,
            container_cache: ContainerCache::new(),
            account_metadata_cache: Arc::new(AccountMetadataCache::new()),
            cpu_monitor,
            machine_id: Arc::new(vm_metadata.machine_id().to_owned()),
            proxy_configuration,
        }))
    }
}

static NEXT_RUNTIME_ID: AtomicUsize = AtomicUsize::new(0);

/// Builds a [`UserAgent`] from a suffix source (suffix, workload id, or
/// correlation id, in priority order) plus the supplied feature flags.
///
/// Shared by [`CosmosDriverRuntimeBuilder::build`] and
/// [`CosmosDriverRuntime::user_agent_with_feature_flags`] so the priority
/// ordering is defined in exactly one place.
fn compute_user_agent(
    wrapping_sdk_identifier: Option<&str>,
    user_agent_suffix: Option<&UserAgentSuffix>,
    workload_id: Option<WorkloadId>,
    correlation_id: Option<&CorrelationId>,
    feature_flags: UserAgentFeatureFlags,
) -> UserAgent {
    if let Some(suffix) = user_agent_suffix {
        UserAgent::from_suffix(wrapping_sdk_identifier, suffix, feature_flags)
    } else if let Some(workload_id) = workload_id {
        UserAgent::from_workload_id(wrapping_sdk_identifier, workload_id, feature_flags)
    } else if let Some(correlation_id) = correlation_id {
        UserAgent::from_correlation_id(wrapping_sdk_identifier, correlation_id, feature_flags)
    } else {
        UserAgent::from_wrapping_sdk_identifier(wrapping_sdk_identifier, feature_flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AccountReference;
    use crate::options::{DiagnosticsOptions, DiagnosticsVerbosity};
    use url::Url;

    #[tokio::test]
    async fn create_driver_propagates_initialization_failures() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "***not-base64***",
        );

        // Two attempts back-to-back — each call must surface the bad-credential
        // failure independently; there is no driver cache, so the second call
        // re-runs the full initialization path.
        let error = runtime
            .create_driver(DriverOptions::builder(account.clone()).build())
            .await
            .expect_err("invalid signing key should fail initialization");
        assert!(!error.to_string().is_empty());

        let second_error = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await
            .expect_err("subsequent attempts must also surface the failure");
        assert!(!second_error.to_string().is_empty());
    }

    #[tokio::test]
    async fn default_diagnostics_options_use_summary_verbosity() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

        assert_eq!(
            runtime.diagnostics_options().default_verbosity(),
            DiagnosticsVerbosity::Summary
        );
    }

    #[tokio::test]
    async fn diagnostics_options_can_be_configured_on_runtime() {
        let diagnostics = DiagnosticsOptions::builder()
            .with_default_verbosity(DiagnosticsVerbosity::Detailed)
            .build()
            .unwrap();
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_diagnostics_options(diagnostics)
            .build()
            .await
            .unwrap();

        assert_eq!(
            runtime.diagnostics_options().default_verbosity(),
            DiagnosticsVerbosity::Detailed
        );
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
