// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime environment.

use azure_core::http::ClientOptions;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::{
    models::{AccountReference, ContainerReference, ThroughputControlGroupName, UserAgent},
    options::{
        ConnectionPoolOptions, CorrelationId, DriverOptions, RuntimeOptions, SharedRuntimeOptions,
        ThroughputControlGroupOptions, ThroughputControlGroupRegistrationError,
        ThroughputControlGroupRegistry, UserAgentSuffix, WorkloadId,
        parse_duration_millis_from_env,
    },
    system::{CpuMemoryMonitor, VmMetadataService},
};

use super::cache::{AccountMetadataCache, ContainerCache};
use super::{transport::CosmosTransport, CosmosDriver};

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
///     .with_content_response_on_write(ContentResponseOnWrite::Disabled)
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
/// cosmos_runtime.runtime_options().set_content_response_on_write(Some(ContentResponseOnWrite::Enabled));
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

    /// Registry of throughput control groups.
    ///
    /// Groups are registered during builder construction and are immutable after
    /// runtime creation (except for mutable target values within each group).
    throughput_control_groups: ThroughputControlGroupRegistry,

    /// Registry of driver instances keyed by account endpoint.
    ///
    /// Ensures singleton driver per account reference.
    driver_registry: Arc<RwLock<HashMap<String, Arc<CosmosDriver>>>>,

    /// Shared container metadata cache used by drivers in this runtime.
    container_cache: Arc<ContainerCache>,

    /// Shared account metadata cache used by drivers in this runtime.
    account_metadata_cache: Arc<AccountMetadataCache>,

    /// CPU and memory monitor for diagnostics.
    cpu_monitor: CpuMemoryMonitor,

    /// Machine identifier for diagnostics (VM ID on Azure, generated UUID otherwise).
    machine_id: Arc<String>,
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

    /// Returns the shared container cache.
    pub(crate) fn container_cache(&self) -> &Arc<ContainerCache> {
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
    runtime_options: Option<RuntimeOptions>,
    workload_id: Option<WorkloadId>,
    correlation_id: Option<CorrelationId>,
    user_agent_suffix: Option<UserAgentSuffix>,
    throughput_control_groups: ThroughputControlGroupRegistry,
    cpu_refresh_interval: Option<Duration>,
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
        let machine_id = Arc::new(vm_metadata.machine_id().to_owned());

        Ok(CosmosDriverRuntime {
            client_options: self.client_options.unwrap_or_default(),
            connection_pool,
            transport,
            runtime_options: SharedRuntimeOptions::from_options(
                self.runtime_options.unwrap_or_default(),
            ),
            user_agent,
            workload_id: self.workload_id,
            correlation_id: self.correlation_id,
            user_agent_suffix: self.user_agent_suffix,
            throughput_control_groups: self.throughput_control_groups,
            driver_registry: Arc::new(RwLock::new(HashMap::new())),
            container_cache: Arc::new(ContainerCache::new()),
            account_metadata_cache: Arc::new(AccountMetadataCache::new()),
            cpu_monitor,
            machine_id,
        })
    }
}
