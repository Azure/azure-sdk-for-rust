// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver runtime and driver singleton management.

use azure_core::http::ClientOptions;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    models::{AccountReference, ContainerReference, CosmosOperation, ThroughputControlGroupName},
    options::{
        ConnectionPoolOptions, CorrelationId, DriverOptions, OperationOptions, RuntimeOptions,
        SharedRuntimeOptions, ThroughputControlGroupOptions,
        ThroughputControlGroupRegistrationError, ThroughputControlGroupRegistry,
        ThroughputControlGroupSnapshot, UserAgent, UserAgentSuffix, WorkloadId,
    },
    system::{AzureVmMetadata, CpuMemoryHistory, CpuMemoryMonitor, VmMetadataService},
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
/// use azure_data_cosmos_driver::options::{
///     CosmosDriverRuntime, CosmosDriverRuntimeBuilder, RuntimeOptions, ContentResponseOnWrite,
/// };
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// # async fn example() -> azure_core::Result<()> {
/// let runtime = RuntimeOptions::builder()
///     .content_response_on_write(ContentResponseOnWrite::DISABLED)
///     .build();
///
/// let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
///     .runtime_options(runtime)
///     .build()
///     .await;
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
#[derive(Clone, Debug)]
pub struct CosmosDriverRuntime {
    /// Core HTTP client options from azure_core.
    client_options: ClientOptions,

    /// Connection pool configuration for managing TCP connections.
    connection_pool: ConnectionPoolOptions,

    /// Thread-safe runtime options for operation options.
    runtime_options: SharedRuntimeOptions,

    /// Effective user agent string for HTTP requests.
    user_agent: Option<UserAgent>,

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

    /// Returns the thread-safe runtime options.
    ///
    /// Use this to modify default operation options at runtime.
    pub fn runtime_options(&self) -> &SharedRuntimeOptions {
        &self.runtime_options
    }

    /// Returns the effective user agent string.
    pub fn user_agent(&self) -> Option<&UserAgent> {
        self.user_agent.as_ref()
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
    /// use azure_data_cosmos_driver::options::CosmosDriverRuntime;
    ///
    /// # async fn example() {
    /// let runtime = CosmosDriverRuntime::builder().build().await;
    /// let history = runtime.cpu_memory_snapshot();
    ///
    /// if let Some(cpu) = history.latest_cpu() {
    ///     println!("Latest CPU: {:.1}%", cpu.value);
    /// }
    ///
    /// if history.is_cpu_overloaded() {
    ///     println!("Warning: CPU is overloaded");
    /// }
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
    /// use azure_data_cosmos_driver::options::CosmosDriverRuntime;
    ///
    /// # async fn example() {
    /// let runtime = CosmosDriverRuntime::builder().build().await;
    /// if let Some(metadata) = runtime.vm_metadata() {
    ///     println!("VM ID: {}", metadata.vm_id());
    ///     println!("Location: {}", metadata.location());
    /// }
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
    /// use azure_data_cosmos_driver::options::{CosmosDriverRuntime, DriverOptions};
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await;
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
/// to this builder via [`runtime_options()`](Self::runtime_options).
///
/// # Throughput Control Groups
///
/// Throughput control groups must be registered during builder construction.
/// Once `build()` is called, the set of groups is immutable (though mutable
/// values within each group can still be updated).
#[derive(Clone, Debug, Default)]
pub struct CosmosDriverRuntimeBuilder {
    client_options: Option<ClientOptions>,
    connection_pool: Option<ConnectionPoolOptions>,
    runtime_options: Option<RuntimeOptions>,
    user_agent: Option<UserAgent>,
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
    pub fn client_options(mut self, options: ClientOptions) -> Self {
        self.client_options = Some(options);
        self
    }

    /// Sets the connection pool options.
    pub fn connection_pool(mut self, options: ConnectionPoolOptions) -> Self {
        self.connection_pool = Some(options);
        self
    }

    /// Sets the runtime options (defaults for operations).
    ///
    /// Use [`RuntimeOptions::builder()`] to create the runtime options.
    pub fn runtime_options(mut self, options: RuntimeOptions) -> Self {
        self.runtime_options = Some(options);
        self
    }

    /// Sets the effective user agent string.
    pub fn user_agent(mut self, user_agent: impl Into<UserAgent>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Sets the workload identifier (must be 1-50 if set).
    pub fn workload_id(mut self, workload_id: WorkloadId) -> Self {
        self.workload_id = Some(workload_id);
        self
    }

    /// Sets the correlation ID for client-side metrics.
    ///
    /// # Cardinality Warning
    ///
    /// If the cardinality of correlation IDs is too high, metrics aggregation
    /// may ignore this dimension. Choose values with moderate cardinality
    /// (e.g., cluster names, environment identifiers, deployment IDs).
    pub fn correlation_id(mut self, correlation_id: CorrelationId) -> Self {
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
    pub fn user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
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
    /// use azure_data_cosmos_driver::options::{
    ///     CosmosDriverRuntimeBuilder, ThroughputControlGroupOptions, ThroughputTarget,
    /// };
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, ContainerProperties, ContainerReference, PartitionKeyDefinition,
    /// };
    /// use std::sync::Arc;
    /// use url::Url;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let container = ContainerReference::new(
    ///     account, "mydb", "db_rid", "mycollection", "coll_rid",
    ///     &ContainerProperties::new("mycollection", PartitionKeyDefinition::new(["/pk"])),
    /// );
    ///
    /// // Register a default group for the container
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
    /// # Note
    ///
    /// This method is async because it may need to fetch Azure VM metadata from
    /// the Instance Metadata Service (IMDS) on first initialization.
    pub async fn build(self) -> CosmosDriverRuntime {
        CosmosDriverRuntime {
            client_options: self.client_options.unwrap_or_default(),
            connection_pool: self.connection_pool.unwrap_or_default(),
            runtime_options: SharedRuntimeOptions::from_options(
                self.runtime_options.unwrap_or_default(),
            ),
            user_agent: self.user_agent,
            workload_id: self.workload_id,
            correlation_id: self.correlation_id,
            user_agent_suffix: self.user_agent_suffix,
            cpu_memory_monitor: CpuMemoryMonitor::get_or_init(),
            vm_metadata_service: VmMetadataService::get_or_init().await,
            throughput_control_groups: self.throughput_control_groups,
            driver_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

/// Cosmos DB driver instance.
///
/// A driver represents a connection to a specific Cosmos DB account. It is created
/// via [`CosmosDriverRuntime::get_or_create_driver()`] and is managed as a singleton
/// per account endpoint.
///
/// The driver handles executing operations against Cosmos DB, merging options from
/// operation, driver, and runtime levels.
#[derive(Debug)]
pub struct CosmosDriver {
    /// Reference to the parent runtime.
    runtime: CosmosDriverRuntime,
    /// Driver-level options including account reference.
    options: DriverOptions,
}

impl CosmosDriver {
    /// Creates a new driver instance.
    ///
    /// This is internal - use [`CosmosDriverRuntime::get_or_create_driver()`] instead.
    fn new(runtime: CosmosDriverRuntime, options: DriverOptions) -> Self {
        Self { runtime, options }
    }

    /// Returns the account reference.
    pub fn account(&self) -> &AccountReference {
        self.options.account()
    }

    /// Returns the runtime.
    pub fn runtime(&self) -> &CosmosDriverRuntime {
        &self.runtime
    }

    /// Returns the driver options.
    pub fn options(&self) -> &DriverOptions {
        &self.options
    }

    /// Computes the effective runtime options by merging operation, driver, and runtime options.
    ///
    /// The merge order is (highest to lowest priority):
    /// 1. `OperationOptions` - operation-specific overrides
    /// 2. `DriverOptions` - driver-level defaults
    /// 3. `CosmosDriverRuntime` - global defaults
    ///
    /// For each property in `RuntimeOptions`, the first defined value is used.
    pub fn effective_runtime_options(
        &self,
        operation_options: &OperationOptions,
    ) -> RuntimeOptions {
        // Start with operation-level options (highest priority)
        let operation_runtime = operation_options.runtime();

        // Get driver-level options
        let driver_runtime = self.options.runtime_options().snapshot();

        // Get runtime-level options (lowest priority)
        let global_runtime = self.runtime.runtime_options().snapshot();

        // Merge: operation -> driver -> runtime
        // First merge operation with driver
        let merged = operation_runtime.merge_with_base(&driver_runtime);
        // Then merge result with runtime defaults
        merged.merge_with_base(&global_runtime)
    }

    /// Computes the effective throughput control group for an operation.
    ///
    /// Resolution order (first match wins):
    /// 1. Explicit group name from effective runtime options + operation's container
    /// 2. Default group for the operation's container
    ///
    /// Returns `None` if no applicable control group is found.
    ///
    /// # Parameters
    ///
    /// - `effective_options`: The merged runtime options (use `effective_runtime_options()`)
    /// - `container`: The container reference for the operation
    fn effective_throughput_control_group(
        &self,
        effective_options: &RuntimeOptions,
        container: &ContainerReference,
    ) -> Option<ThroughputControlGroupSnapshot> {
        // First, check if an explicit group name is specified in options
        if let Some(group_name) = &effective_options.throughput_control_group_name {
            if let Some(group) = self
                .runtime
                .get_throughput_control_group(container, group_name)
            {
                return Some(ThroughputControlGroupSnapshot::from(group.as_ref()));
            }
        }

        // Fall back to the default group for the container
        self.runtime
            .get_default_throughput_control_group(container)
            .map(|group| ThroughputControlGroupSnapshot::from(group.as_ref()))
    }

    /// Executes a Cosmos DB operation.
    ///
    /// This method computes effective options by merging the provided operation options
    /// with driver and runtime defaults, then executes the operation.
    ///
    /// # Parameters
    ///
    /// - `operation`: The operation to execute
    /// - `options`: Operation-specific options that override driver and runtime defaults
    ///
    /// # Returns
    ///
    /// Returns the raw response bytes on success.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::options::{CosmosDriverRuntime, OperationOptions, ContentResponseOnWrite};
    /// use azure_data_cosmos_driver::models::AccountReference;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    ///
    /// // Execute operations with operation-specific options that override defaults
    /// let options = OperationOptions::new()
    ///     .content_response_on_write(ContentResponseOnWrite::DISABLED);
    ///
    /// // let response = driver.execute_operation(operation, options).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> azure_core::Result<Vec<u8>> {
        // Step 1: Derive effective runtime options
        let effective_options = self.effective_runtime_options(&options);

        // Step 2: Get effective throughput control group (if any)
        let _effective_control_group = operation.container().and_then(|container| {
            self.effective_throughput_control_group(&effective_options, container)
        });

        // TODO: Implement actual operation execution
        // - Build HTTP request based on operation type
        // - Apply effective options (headers, policies)
        // - Apply throughput control group settings if present
        // - Send request via HTTP client
        // - Handle response/errors

        // Placeholder: Return empty response
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::ContentResponseOnWrite;
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[tokio::test]
    async fn default_runtime_options() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await;
        let snapshot = runtime.runtime_options().snapshot();
        assert!(snapshot.throughput_control_group_name.is_none());
        assert!(snapshot.content_response_on_write.is_none());
        assert!(runtime.user_agent().is_none());
        assert!(runtime.workload_id().is_none());
        assert!(runtime.correlation_id().is_none());
        assert!(runtime.user_agent_suffix().is_none());
        // machine_id is always available
        assert!(!runtime.machine_id().is_empty());
    }

    #[tokio::test]
    async fn builder_sets_runtime_options() {
        let opts = RuntimeOptions::builder()
            .content_response_on_write(ContentResponseOnWrite::DISABLED)
            .build();

        let runtime = CosmosDriverRuntimeBuilder::new()
            .runtime_options(opts)
            .build()
            .await;

        let snapshot = runtime.runtime_options().snapshot();
        assert_eq!(
            snapshot.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
        );
    }

    #[tokio::test]
    async fn builder_sets_identity_fields() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .user_agent(UserAgent::new("my-app/1.0"))
            .workload_id(WorkloadId::new(25))
            .correlation_id(CorrelationId::new("aks-prod-eastus"))
            .user_agent_suffix(UserAgentSuffix::new("myapp-westus2"))
            .build()
            .await;

        assert_eq!(runtime.user_agent().unwrap().as_str(), "my-app/1.0");
        assert_eq!(runtime.workload_id().unwrap().value(), 25);
        assert_eq!(
            runtime.correlation_id().unwrap().as_str(),
            "aks-prod-eastus"
        );
        assert_eq!(
            runtime.user_agent_suffix().unwrap().as_str(),
            "myapp-westus2"
        );
    }

    #[tokio::test]
    async fn effective_correlation_prefers_correlation_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .correlation_id(CorrelationId::new("correlation"))
            .user_agent_suffix(UserAgentSuffix::new("suffix"))
            .build()
            .await;

        assert_eq!(runtime.effective_correlation(), Some("correlation"));
    }

    #[tokio::test]
    async fn effective_correlation_falls_back_to_suffix() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .user_agent_suffix(UserAgentSuffix::new("suffix"))
            .build()
            .await;

        assert_eq!(runtime.effective_correlation(), Some("suffix"));
    }

    #[tokio::test]
    async fn effective_correlation_none_when_both_unset() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await;
        assert!(runtime.effective_correlation().is_none());
    }

    #[tokio::test]
    async fn runtime_modification() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await;

        // Initially none
        assert!(runtime
            .runtime_options()
            .snapshot()
            .content_response_on_write
            .is_none());

        // Modify at runtime
        runtime
            .runtime_options()
            .set_content_response_on_write(Some(ContentResponseOnWrite::ENABLED));

        // Now set
        assert_eq!(
            runtime
                .runtime_options()
                .snapshot()
                .content_response_on_write,
            Some(ContentResponseOnWrite::ENABLED)
        );
    }

    #[tokio::test]
    async fn effective_options_merge_priority() {
        // Runtime has ENABLED
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
            .runtime_options(
                RuntimeOptions::builder()
                    .content_response_on_write(ContentResponseOnWrite::ENABLED)
                    .build(),
            )
            .build()
            .await;

        // Driver has DISABLED
        let driver_options = DriverOptions::builder(test_account())
            .runtime_options(
                RuntimeOptions::builder()
                    .content_response_on_write(ContentResponseOnWrite::DISABLED)
                    .build(),
            )
            .build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation has no override - should get driver's DISABLED
        let op_options = OperationOptions::new();
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
        );

        // Operation overrides to ENABLED - should get ENABLED
        let op_options =
            OperationOptions::new().content_response_on_write(ContentResponseOnWrite::ENABLED);
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::ENABLED)
        );
    }

    #[tokio::test]
    async fn effective_options_falls_back_to_runtime() {
        // Runtime has ENABLED
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
            .runtime_options(
                RuntimeOptions::builder()
                    .content_response_on_write(ContentResponseOnWrite::ENABLED)
                    .build(),
            )
            .build()
            .await;

        // Driver has no override
        let driver_options = DriverOptions::builder(test_account()).build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation has no override - should fall back to runtime's ENABLED
        let op_options = OperationOptions::new();
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::ENABLED)
        );
    }

    #[tokio::test]
    async fn machine_id_always_available() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await;

        // machine_id is always available (either VM ID or generated UUID)
        let machine_id = runtime.machine_id();
        assert!(!machine_id.is_empty());

        // It should have one of the known prefixes
        assert!(
            machine_id.starts_with("vmId_") || machine_id.starts_with("uuid_"),
            "machine_id should start with 'vmId_' or 'uuid_', got: {}",
            machine_id
        );
    }
}
