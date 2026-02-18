// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime environment.

use azure_core::http::ClientOptions;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    models::{
        AccountEndpoint, AccountReference, ContainerReference, ThroughputControlGroupName,
        UserAgent,
    },
    options::{
        ConnectionPoolOptions, CorrelationId, DiagnosticsOptions, DriverOptions, RuntimeOptions,
        SharedRuntimeOptions, ThroughputControlGroupOptions,
        ThroughputControlGroupRegistrationError, ThroughputControlGroupRegistry, UserAgentSuffix,
        WorkloadId,
    },
    system::{AzureVmMetadata, CpuMemoryHistory, CpuMemoryMonitor, VmMetadataService},
};

use super::{
    cache::{AccountMetadataCache, ContainerCache},
    transport::CosmosTransport,
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
/// use azure_data_cosmos_driver::options::{RuntimeOptions, ContentResponseOnWrite};
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// # async fn example() -> azure_core::Result<()> {
/// let runtime = RuntimeOptions::builder()
///     .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
///     .build();
///
/// let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
///     .with_runtime_options(runtime)
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
/// // Later, modify defaults at runtime
/// cosmos_runtime.runtime_options().set_content_response_on_write(Some(ContentResponseOnWrite::ENABLED));
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct CosmosDriverRuntime {
    /// Core HTTP client options from azure_core.
    client_options: ClientOptions,

    /// Connection pool configuration for managing TCP connections.
    connection_pool: ConnectionPoolOptions,

    /// HTTP transport manager with connection pools.
    ///
    /// Manages separate pools for metadata and data plane operations,
    /// with lazy initialization of emulator-specific pools.
    transport: Arc<CosmosTransport>,

    /// Diagnostics configuration for output verbosity and size limits.
    diagnostics_options: Arc<DiagnosticsOptions>,

    /// Thread-safe runtime options for operation options.
    runtime_options: SharedRuntimeOptions,

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

    /// Process-wide CPU and memory monitor singleton.
    ///
    /// Provides access to historical CPU/memory snapshots for client telemetry.
    /// The monitor runs in a background thread and samples every 5 seconds.
    cpu_memory_monitor: CpuMemoryMonitor,

    /// Process-wide Azure VM metadata service singleton.
    ///
    /// Provides access to VM metadata from the Instance Metadata Service (IMDS).
    /// Metadata is fetched once on first access and cached for the process lifetime.
    vm_metadata_service: VmMetadataService,

    /// Registry of throughput control groups.
    ///
    /// Groups are registered during builder construction and are immutable after
    /// runtime creation (except for mutable target values within each group).
    throughput_control_groups: ThroughputControlGroupRegistry,

    /// Registry of driver instances keyed by account endpoint.
    ///
    /// Ensures singleton driver per account reference.
    driver_registry: Arc<RwLock<HashMap<String, Arc<CosmosDriver>>>>,

    /// Cache for account metadata (regions, capabilities).
    ///
    /// Entries are populated on first access to an account and used for routing.
    /// Wrapped in `Arc` for cheap cloning.
    account_metadata_cache: Arc<AccountMetadataCache>,

    /// Cache for container metadata (partition key definition, indexing policy).
    ///
    /// Entries are populated on first access to a container and used for
    /// partition key extraction and routing. Wrapped in `Arc` for cheap cloning.
    container_cache: Arc<ContainerCache>,
}

impl CosmosDriverRuntime {
    /// Returns a new builder for creating a runtime.
    pub fn builder() -> CosmosDriverRuntimeBuilder {
        CosmosDriverRuntimeBuilder::new()
    }

    /// Returns the HTTP client options.
    pub fn client_options(&self) -> &ClientOptions {
        &self.client_options
    }

    /// Returns the connection pool options.
    pub fn connection_pool(&self) -> &ConnectionPoolOptions {
        &self.connection_pool
    }

    /// Returns the HTTP transport manager.
    ///
    /// The transport provides access to connection pools configured for
    /// metadata and data plane operations, with automatic emulator detection.
    pub(crate) fn transport(&self) -> &Arc<CosmosTransport> {
        &self.transport
    }

    /// Returns the diagnostics options.
    ///
    /// Use this to access verbosity and size settings for diagnostic output.
    pub fn diagnostics_options(&self) -> &Arc<DiagnosticsOptions> {
        &self.diagnostics_options
    }

    /// Returns the thread-safe runtime options.
    ///
    /// Use this to modify default operation options at runtime.
    pub fn runtime_options(&self) -> &SharedRuntimeOptions {
        &self.runtime_options
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

    /// Returns the effective correlation dimension.
    ///
    /// Returns `correlation_id` if set, otherwise falls back to `user_agent_suffix`.
    pub fn effective_correlation(&self) -> Option<&str> {
        self.correlation_id
            .as_ref()
            .map(|c| c.as_str())
            .or_else(|| self.user_agent_suffix.as_ref().map(|s| s.as_str()))
    }

    /// Returns a snapshot of the current CPU and memory usage history.
    ///
    /// The history contains the most recent CPU load and memory usage samples,
    /// typically covering the last 30 seconds (6 samples at 5-second intervals).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let history = runtime.cpu_memory_snapshot();
    ///
    /// if let Some(cpu) = history.latest_cpu() {
    ///     println!("Latest CPU: {:.1}%", cpu.value());
    /// }
    ///
    /// if history.is_cpu_overloaded() {
    ///     println!("Warning: CPU is overloaded");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn cpu_memory_snapshot(&self) -> CpuMemoryHistory {
        self.cpu_memory_monitor.snapshot()
    }

    /// Returns the cached Azure VM metadata, if available.
    ///
    /// Returns `None` if:
    /// - Not running on an Azure VM
    /// - The `COSMOS_DISABLE_IMDS` environment variable is set
    /// - The IMDS endpoint is unreachable
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// if let Some(metadata) = runtime.vm_metadata() {
    ///     println!("VM ID: {}", metadata.vm_id());
    ///     println!("Location: {}", metadata.location());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn vm_metadata(&self) -> Option<&AzureVmMetadata> {
        self.vm_metadata_service.metadata()
    }

    /// Returns the unique machine ID.
    ///
    /// This is always available:
    /// - On Azure VMs: "vmId_{vm-id}" from IMDS
    /// - Off Azure: "uuid_{generated-uuid}" (stable for process lifetime)
    pub fn machine_id(&self) -> &str {
        self.vm_metadata_service.machine_id()
    }

    /// Returns `true` if running on an Azure VM with accessible IMDS.
    pub fn is_on_azure(&self) -> bool {
        self.vm_metadata_service.is_on_azure()
    }

    /// Returns the throughput control group registry.
    ///
    /// The registry contains all groups registered during runtime construction.
    /// Groups are identified by the combination of container reference and group name.
    pub fn throughput_control_groups(&self) -> &ThroughputControlGroupRegistry {
        &self.throughput_control_groups
    }

    /// Returns a throughput control group by container and name.
    ///
    /// This is a convenience method for looking up a specific group.
    pub fn get_throughput_control_group(
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
    pub fn get_default_throughput_control_group(
        &self,
        container: &ContainerReference,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.throughput_control_groups
            .get_default_for_container(container)
    }

    // ===== Cache Access Methods =====

    /// Returns a cached resolved container reference looked up by name.
    ///
    /// Returns `None` if the container hasn't been resolved and cached yet.
    /// Use [`resolve_container_by_name`](Self::resolve_container_by_name) to fetch and cache if needed.
    pub(crate) async fn get_cached_container_by_name(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
    ) -> Option<Arc<ContainerReference>> {
        self.container_cache
            .get_by_name(account_endpoint, db_name, container_name)
            .await
    }

    /// Resolves a container by name, fetching and caching if not already cached.
    ///
    /// The `fetch_fn` is only called if the container is not in the cache.
    /// On a cache miss the resolved reference is cross-populated into the
    /// by-RID cache as well. Concurrent requests for the same container
    /// share the same fetch operation.
    async fn resolve_container_by_name<F, Fut>(
        &self,
        account_endpoint: &str,
        db_name: &str,
        container_name: &str,
        fetch_fn: F,
    ) -> Arc<ContainerReference>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ContainerReference>,
    {
        self.container_cache
            .get_or_fetch_by_name(account_endpoint, db_name, container_name, fetch_fn)
            .await
    }

    /// Resolves a container by RID, fetching and caching if not already cached.
    ///
    /// The `fetch_fn` is only called if the container is not in the cache.
    /// On a cache miss the resolved reference is cross-populated into the
    /// by-name cache as well.
    async fn resolve_container_by_rid<F, Fut>(
        &self,
        account_endpoint: &str,
        container_rid: &str,
        fetch_fn: F,
    ) -> Arc<ContainerReference>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = ContainerReference>,
    {
        self.container_cache
            .get_or_fetch_by_rid(account_endpoint, container_rid, fetch_fn)
            .await
    }

    /// Invalidates the cached container reference from both name and RID caches.
    ///
    /// Call this when container properties may have changed (e.g., after
    /// updating indexing policy) or when a container has been deleted/recreated.
    async fn invalidate_container_cache(&self, container: &ContainerReference) {
        self.container_cache.invalidate(container).await;
    }

    /// Inserts a resolved container reference into the cache.
    ///
    /// Populates both the by-name and by-RID indices.
    pub(crate) async fn cache_container(&self, container: ContainerReference) {
        self.container_cache.put(container).await;
    }

    /// Invalidates cached account metadata.
    ///
    /// Call this when account configuration may have changed (e.g., after
    /// adding/removing regions).
    async fn invalidate_account_cache(&self, endpoint: &AccountEndpoint) {
        self.account_metadata_cache.invalidate(endpoint).await;
    }

    /// Clears all caches.
    ///
    /// This is primarily useful for testing or when the connection needs
    /// to be fully refreshed.
    async fn clear_all_caches(&self) {
        self.account_metadata_cache.clear().await;
        self.container_cache.clear().await;
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
    /// # async fn example() -> azure_core::Result<()> {
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
        &self,
        account: AccountReference,
        driver_options: Option<DriverOptions>,
    ) -> azure_core::Result<Arc<CosmosDriver>> {
        let key = account.endpoint().to_string();

        // Check if driver already exists (read lock)
        {
            let registry = self.driver_registry.read().unwrap();
            if let Some(driver) = registry.get(&key) {
                return Ok(driver.clone());
            }
        }

        // Create new driver (write lock)
        let mut registry = self.driver_registry.write().unwrap();

        // Double-check after acquiring write lock
        if let Some(driver) = registry.get(&key) {
            return Ok(driver.clone());
        }

        // Build driver options if not provided
        let options = driver_options.unwrap_or_else(|| DriverOptions::builder(account).build());

        let driver = Arc::new(CosmosDriver::new(self.clone(), options));
        registry.insert(key, driver.clone());

        Ok(driver)
    }
}

/// Builder for creating [`CosmosDriverRuntime`].
///
/// Use [`RuntimeOptions::builder()`] to create runtime options, then pass them
/// to this builder via [`with_runtime_options()`](Self::with_runtime_options).
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
    runtime_options: Option<RuntimeOptions>,
    workload_id: Option<WorkloadId>,
    correlation_id: Option<CorrelationId>,
    user_agent_suffix: Option<UserAgentSuffix>,
    throughput_control_groups: ThroughputControlGroupRegistry,
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

    /// Sets the diagnostics options.
    ///
    /// Controls verbosity and size limits for diagnostic output.
    pub fn with_diagnostics_options(mut self, options: DiagnosticsOptions) -> Self {
        self.diagnostics_options = Some(options);
        self
    }

    /// Sets the runtime options (defaults for operations).
    ///
    /// Use [`RuntimeOptions::builder()`] to create the runtime options.
    pub fn with_runtime_options(mut self, options: RuntimeOptions) -> Self {
        self.runtime_options = Some(options);
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
    /// use azure_data_cosmos_driver::options::{ThroughputControlGroupOptions, ThroughputTarget};
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
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
    ///         ThroughputControlGroupOptions::client_side(
    ///             "default-group",
    ///             container.clone(),
    ///             ThroughputTarget::Threshold(0.5),
    ///             None,
    ///             true, // is_default
    ///         )
    ///     )?
    ///     .build()
    ///     .await;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::result_large_err)]
    pub fn register_throughput_control_group(
        mut self,
        group: ThroughputControlGroupOptions,
    ) -> Result<Self, ThroughputControlGroupRegistrationError> {
        self.throughput_control_groups.register(group)?;
        Ok(self)
    }

    /// Builds the [`CosmosDriverRuntime`].
    ///
    /// This automatically initializes the process-wide CPU/memory monitor and
    /// VM metadata service singletons if they haven't been initialized already.
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
    /// # Note
    ///
    /// This method is async because it may need to fetch Azure VM metadata from
    /// the Instance Metadata Service (IMDS) on first initialization.
    pub async fn build(self) -> azure_core::Result<CosmosDriverRuntime> {
        // Compute user agent from suffix/workloadId/correlationId (in priority order)
        let user_agent = if let Some(ref suffix) = self.user_agent_suffix {
            UserAgent::from_suffix(suffix)
        } else if let Some(workload_id) = self.workload_id {
            UserAgent::from_workload_id(workload_id)
        } else if let Some(ref correlation_id) = self.correlation_id {
            UserAgent::from_correlation_id(correlation_id)
        } else {
            UserAgent::default()
        };

        let connection_pool = self.connection_pool.unwrap_or_default();
        let transport = Arc::new(CosmosTransport::new(
            connection_pool.clone(),
            user_agent.as_str(),
        )?);

        Ok(CosmosDriverRuntime {
            client_options: self.client_options.unwrap_or_default(),
            connection_pool,
            transport,
            diagnostics_options: Arc::new(self.diagnostics_options.unwrap_or_default()),
            runtime_options: SharedRuntimeOptions::from_options(
                self.runtime_options.unwrap_or_default(),
            ),
            user_agent,
            workload_id: self.workload_id,
            correlation_id: self.correlation_id,
            user_agent_suffix: self.user_agent_suffix,
            cpu_memory_monitor: CpuMemoryMonitor::get_or_init(),
            vm_metadata_service: VmMetadataService::get_or_init().await,
            throughput_control_groups: self.throughput_control_groups,
            driver_registry: Arc::new(RwLock::new(HashMap::new())),
            account_metadata_cache: Arc::new(AccountMetadataCache::new()),
            container_cache: Arc::new(ContainerCache::new()),
        })
    }
}
