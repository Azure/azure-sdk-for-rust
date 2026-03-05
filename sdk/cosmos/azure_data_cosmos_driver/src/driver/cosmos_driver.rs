// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver instance.

use crate::{
    diagnostics::{DiagnosticsContextBuilder, PipelineType, TransportSecurity},
    models::{
        AccountEndpoint, AccountReference, ActivityId, ContainerProperties, ContainerReference,
        CosmosOperation, DatabaseProperties, DatabaseReference,
    },
    options::{
        DiagnosticsOptions, DriverOptions, OperationOptions, RuntimeOptions,
        ThroughputControlGroupSnapshot,
    },
};
use azure_core::http::Request;
use std::sync::Arc;

use super::{
    cache::AccountRegion,
    transport::{
        cosmos_headers, is_emulator_host, request_signing, uses_dataplane_pipeline,
        AuthorizationContext,
    },
    CosmosDriverRuntime,
};

/// Cosmos DB driver instance.
///
/// A driver represents a connection to a specific Cosmos DB account. It is created
/// via [`CosmosDriverRuntime::get_or_create_driver()`] and is managed as a singleton
/// per account endpoint.
///
/// The driver handles executing operations against Cosmos DB, merging options from
/// operation, driver, and runtime levels.
#[non_exhaustive]
#[derive(Debug)]
pub struct CosmosDriver {
    /// Reference to the parent runtime.
    runtime: CosmosDriverRuntime,
    /// Driver-level options including account reference.
    options: DriverOptions,
}

impl CosmosDriver {
    fn parse_account_properties_payload(
        payload: &[u8],
    ) -> azure_core::Result<super::cache::AccountProperties> {
        serde_json::from_slice(payload)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))
    }

    fn endpoint_for_write_region(
        account: &AccountReference,
        write_region: Option<&AccountRegion>,
    ) -> AccountEndpoint {
        if let Some(region) = write_region {
            if let Ok(endpoint) =
                AccountEndpoint::try_from(region.database_account_endpoint.as_str())
            {
                return endpoint;
            }
        }

        // Fall back to the account-level endpoint when there is no writable
        // location or the regional URL could not be parsed.
        AccountEndpoint::from(account)
    }

    async fn fetch_account_properties(
        &self,
        account: &AccountReference,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let transport = self.runtime.transport();
        let http_client = transport.get_metadata_http_client(&endpoint);
        let user_agent = self.runtime.user_agent().as_str();

        let mut request = Request::new(endpoint.join_path("/"), azure_core::http::Method::Get);
        cosmos_headers::apply_cosmos_headers(&mut request, user_agent);
        request_signing::sign_request(
            &mut request,
            account.auth(),
            &AuthorizationContext::new(
                azure_core::http::Method::Get,
                crate::models::ResourceType::DatabaseAccount,
                "",
            ),
        )
        .await?;

        let response = http_client.execute_request(&request).await?;
        let raw = response.try_into_raw_response().await?;
        Self::parse_account_properties_payload(raw.body())
    }

    async fn fetch_container_by_name(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> azure_core::Result<ContainerReference> {
        let db_ref = DatabaseReference::from_name(self.account().clone(), db_name.to_owned());
        let options = OperationOptions::new();

        let db_result = self
            .execute_operation(
                CosmosOperation::read_database(db_ref.clone()),
                options.clone(),
            )
            .await?;
        let db_props: DatabaseProperties = serde_json::from_slice(db_result.body())
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let db_rid = db_props.system_properties.rid.ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "database response missing _rid",
            )
        })?;

        let container_result = self
            .execute_operation(
                CosmosOperation::read_container_by_name(db_ref, container_name.to_owned()),
                options,
            )
            .await?;
        let container_props: ContainerProperties = serde_json::from_slice(container_result.body())
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let container_rid = container_props
            .system_properties
            .rid
            .clone()
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    "container response missing _rid",
                )
            })?;

        Ok(ContainerReference::new(
            self.account().clone(),
            db_props.id.into_owned(),
            db_rid,
            container_props.id.clone().into_owned(),
            container_rid,
            &container_props,
        ))
    }

    async fn fetch_container_by_rid(
        &self,
        db_rid: &str,
        container_rid: &str,
    ) -> azure_core::Result<ContainerReference> {
        let db_ref = DatabaseReference::from_rid(self.account().clone(), db_rid.to_owned());
        let options = OperationOptions::new();

        let db_result = self
            .execute_operation(
                CosmosOperation::read_database(db_ref.clone()),
                options.clone(),
            )
            .await?;
        let db_props: DatabaseProperties = serde_json::from_slice(db_result.body())
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let resolved_db_rid = db_props
            .system_properties
            .rid
            .clone()
            .unwrap_or_else(|| db_rid.to_owned());

        let container_result = self
            .execute_operation(
                CosmosOperation::read_container_by_rid(db_ref, container_rid.to_owned()),
                options,
            )
            .await?;
        let container_props: ContainerProperties = serde_json::from_slice(container_result.body())
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
        let resolved_container_rid = container_props
            .system_properties
            .rid
            .clone()
            .unwrap_or_else(|| container_rid.to_owned());

        Ok(ContainerReference::new(
            self.account().clone(),
            db_props.id.into_owned(),
            resolved_db_rid,
            container_props.id.clone().into_owned(),
            resolved_container_rid,
            &container_props,
        ))
    }

    /// Creates a new driver instance.
    ///
    /// This is internal - use [`CosmosDriverRuntime::get_or_create_driver()`] instead.
    pub(crate) fn new(runtime: CosmosDriverRuntime, options: DriverOptions) -> Self {
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
    pub(crate) fn effective_throughput_control_group(
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
    /// Returns a [`crate::models::CosmosResponse`] on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The account has no authentication configured
    /// - The resource reference cannot produce a valid path
    /// - The HTTP request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::options::{OperationOptions, ContentResponseOnWrite};
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
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    ///
    /// // Execute operations with operation-specific options that override defaults
    /// let options = OperationOptions::new()
    ///     .with_content_response_on_write(ContentResponseOnWrite::Disabled);
    ///
    /// // let result = driver.execute_operation(operation, options).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> azure_core::Result<crate::models::CosmosResponse> {
        // Step 1: Derive effective runtime options
        let effective_options = self.effective_runtime_options(&options);

        // Step 2: Get effective throughput control group (if any)
        let _effective_control_group = operation.container().and_then(|container| {
            self.effective_throughput_control_group(&effective_options, container)
        });

        // Step 3: Initialize operation activity id
        let activity_id = ActivityId::new_uuid();

        // Step 4: Get authentication (guaranteed to be present by AccountReference)
        let account = operation.resource_reference().account();
        let auth = account.auth();

        // Step 4.1: Resolve account metadata and select write-region endpoint.
        let account_endpoint = AccountEndpoint::from(account);
        let account_properties = self
            .runtime
            .account_metadata_cache()
            .get_or_fetch(account_endpoint, || self.fetch_account_properties(account))
            .await?;
        let write_region = account_properties.write_account_region();
        let region = write_region.map(|r| r.name.clone());
        let endpoint = Self::endpoint_for_write_region(account, write_region);

        // Step 5: Select appropriate HttpClient based on pipeline type
        let transport = self.runtime.transport();
        let operation_type = operation.operation_type();
        let resource_type = operation.resource_type();
        let is_dataplane = uses_dataplane_pipeline(resource_type, operation_type);
        let http_client = if is_dataplane {
            transport.get_dataplane_http_client(&endpoint)
        } else {
            transport.get_metadata_http_client(&endpoint)
        };

        // Step 6: Initialize diagnostics
        let mut diagnostics_builder = DiagnosticsContextBuilder::new(
            activity_id.clone(),
            std::sync::Arc::new(DiagnosticsOptions::default()),
        );
        diagnostics_builder.set_cpu_monitor(self.runtime.cpu_monitor().clone());
        diagnostics_builder.set_machine_id(Arc::clone(self.runtime.machine_id()));

        let pipeline_type = if is_dataplane {
            PipelineType::DataPlane
        } else {
            PipelineType::Metadata
        };
        let transport_security = if bool::from(
            self.runtime
                .connection_pool()
                .emulator_server_cert_validation(),
        ) && is_emulator_host(&endpoint)
        {
            TransportSecurity::EmulatorWithInsecureCertificates
        } else {
            TransportSecurity::Secure
        };

        let user_agent = self.runtime.user_agent().as_str();

        // Step 7: Execute via the new operation pipeline
        super::pipeline::operation_pipeline::execute_operation_pipeline(
            &operation,
            &options,
            &effective_options,
            &endpoint,
            region,
            http_client,
            auth,
            user_agent,
            &activity_id,
            pipeline_type,
            transport_security,
            diagnostics_builder,
        )
        .await
    }
    /// Resolves a container by database and container name.
    ///
    /// Reads the database and container from the service to obtain their
    /// resource IDs (RIDs) and container properties (partition key, unique key
    /// policy).
    ///
    /// # Parameters
    ///
    /// - `db_name`:  Name of the database.
    /// - `container_name`: Name of the container.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    /// use azure_data_cosmos_driver::models::{
    ///     AccountReference, CosmosOperation, ItemReference, PartitionKey,
    /// };
    /// use azure_data_cosmos_driver::options::OperationOptions;
    /// use url::Url;
    ///
    /// # async fn example() -> azure_core::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime.get_or_create_driver(account, None).await?;
    ///
    /// // Resolve the container (fetched from service on each call)
    /// let container = driver.resolve_container("mydb", "mycontainer").await?;
    ///
    /// // Use the resolved container for item operations
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
    /// let result = driver
    ///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::new())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resolve_container(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> azure_core::Result<ContainerReference> {
        self.resolve_container_by_name(db_name, container_name)
            .await
    }

    /// Resolves a container by database name and container name.
    ///
    /// Attempts to resolve from `ContainerCache` first. On cache miss, fetches
    /// metadata from the service and populates the cache.
    pub async fn resolve_container_by_name(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> azure_core::Result<ContainerReference> {
        let endpoint = self.account().endpoint().as_str().to_owned();
        let db_name_owned = db_name.to_owned();
        let container_name_owned = container_name.to_owned();

        let resolved = self
            .runtime
            .container_cache()
            .get_or_fetch_by_name(&endpoint, db_name, container_name, || async move {
                self.fetch_container_by_name(&db_name_owned, &container_name_owned)
                    .await
            })
            .await?;

        Ok(resolved.as_ref().clone())
    }

    /// Resolves a container by database RID and container RID.
    ///
    /// Attempts to resolve from `ContainerCache` first. On cache miss, fetches
    /// metadata from the service and populates the cache.
    pub async fn resolve_container_by_rid(
        &self,
        db_rid: &str,
        container_rid: &str,
    ) -> azure_core::Result<ContainerReference> {
        let endpoint = self.account().endpoint().as_str().to_owned();
        let db_rid_owned = db_rid.to_owned();
        let container_rid_owned = container_rid.to_owned();

        let resolved = self
            .runtime
            .container_cache()
            .get_or_fetch_by_rid(&endpoint, container_rid, || async move {
                self.fetch_container_by_rid(&db_rid_owned, &container_rid_owned)
                    .await
            })
            .await?;

        Ok(resolved.as_ref().clone())
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{
        driver::CosmosDriverRuntimeBuilder,
        models::AccountReference,
        options::{
            ContentResponseOnWrite, CorrelationId, RuntimeOptions, UserAgentSuffix, WorkloadId,
        },
    };

    use super::*;
    use crate::options::Region;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[tokio::test]
    async fn default_runtime_options() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let snapshot = runtime.runtime_options().snapshot();
        assert!(snapshot.throughput_control_group_name.is_none());
        assert!(snapshot.content_response_on_write.is_none());
        // user_agent is always available with base prefix
        assert!(runtime
            .user_agent()
            .as_str()
            .starts_with("azsdk-rust-cosmos-driver/"));
        assert!(runtime.user_agent().suffix().is_none());
        assert!(runtime.workload_id().is_none());
        assert!(runtime.correlation_id().is_none());
        assert!(runtime.user_agent_suffix().is_none());
    }

    #[tokio::test]
    async fn builder_sets_runtime_options() {
        let opts = RuntimeOptions::builder()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .build();

        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_runtime_options(opts)
            .build()
            .await
            .unwrap();

        let snapshot = runtime.runtime_options().snapshot();
        assert_eq!(
            snapshot.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );
    }

    #[tokio::test]
    async fn builder_sets_identity_fields() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_workload_id(WorkloadId::new(25))
            .with_correlation_id(CorrelationId::new("aks-prod-eastus"))
            .with_user_agent_suffix(UserAgentSuffix::new("myapp-westus2"))
            .build()
            .await
            .unwrap();

        // user_agent_suffix takes priority for user agent computation
        assert!(runtime.user_agent().as_str().contains("myapp-westus2"));
        assert_eq!(runtime.user_agent().suffix(), Some("myapp-westus2"));
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
    async fn user_agent_computed_from_suffix() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_user_agent_suffix(UserAgentSuffix::new("my-suffix"))
            .build()
            .await
            .unwrap();

        assert!(runtime
            .user_agent()
            .as_str()
            .starts_with("azsdk-rust-cosmos-driver/"));
        assert!(runtime.user_agent().as_str().contains("my-suffix"));
        assert_eq!(runtime.user_agent().suffix(), Some("my-suffix"));
    }

    #[tokio::test]
    async fn user_agent_computed_from_workload_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_workload_id(WorkloadId::new(42))
            .build()
            .await
            .unwrap();

        assert!(runtime
            .user_agent()
            .as_str()
            .starts_with("azsdk-rust-cosmos-driver/"));
        assert!(runtime.user_agent().as_str().contains("w42"));
    }

    #[tokio::test]
    async fn user_agent_computed_from_correlation_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_correlation_id(CorrelationId::new("my-correlation"))
            .build()
            .await
            .unwrap();

        assert!(runtime
            .user_agent()
            .as_str()
            .starts_with("azsdk-rust-cosmos-driver/"));
        assert!(runtime.user_agent().as_str().contains("my-correlation"));
    }

    #[tokio::test]
    async fn user_agent_suffix_takes_priority_over_workload_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_user_agent_suffix(UserAgentSuffix::new("suffix"))
            .with_workload_id(WorkloadId::new(25))
            .with_correlation_id(CorrelationId::new("correlation"))
            .build()
            .await
            .unwrap();

        // suffix should be used, not workload_id or correlation_id
        assert!(runtime.user_agent().as_str().contains("suffix"));
        assert!(!runtime.user_agent().as_str().contains("w25"));
        assert!(!runtime.user_agent().as_str().contains("correlation"));
    }

    #[tokio::test]
    async fn workload_id_takes_priority_over_correlation_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_workload_id(WorkloadId::new(25))
            .with_correlation_id(CorrelationId::new("correlation"))
            .build()
            .await
            .unwrap();

        // workload_id should be used, not correlation_id
        assert!(runtime.user_agent().as_str().contains("w25"));
        assert!(!runtime.user_agent().as_str().contains("correlation"));
    }

    #[tokio::test]
    async fn effective_correlation_prefers_correlation_id() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_correlation_id(CorrelationId::new("correlation"))
            .with_user_agent_suffix(UserAgentSuffix::new("suffix"))
            .build()
            .await
            .unwrap();

        assert_eq!(runtime.effective_correlation(), Some("correlation"));
    }

    #[tokio::test]
    async fn effective_correlation_falls_back_to_suffix() {
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_user_agent_suffix(UserAgentSuffix::new("suffix"))
            .build()
            .await
            .unwrap();

        assert_eq!(runtime.effective_correlation(), Some("suffix"));
    }

    #[tokio::test]
    async fn effective_correlation_none_when_both_unset() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        assert!(runtime.effective_correlation().is_none());
    }

    #[tokio::test]
    async fn runtime_modification() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

        // Initially none
        assert!(runtime
            .runtime_options()
            .snapshot()
            .content_response_on_write
            .is_none());

        // Modify at runtime
        runtime
            .runtime_options()
            .set_content_response_on_write(Some(ContentResponseOnWrite::Enabled));

        // Now set
        assert_eq!(
            runtime
                .runtime_options()
                .snapshot()
                .content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
    }

    #[tokio::test]
    async fn effective_options_merge_priority() {
        // Runtime has ENABLED
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::Enabled)
                    .build(),
            )
            .build()
            .await
            .unwrap();

        // Driver has DISABLED
        let driver_options = DriverOptions::builder(test_account())
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::Disabled)
                    .build(),
            )
            .build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation has no override - should get driver's DISABLED
        let op_options = OperationOptions::new();
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::Disabled)
        );

        // Operation overrides to ENABLED - should get ENABLED
        let op_options =
            OperationOptions::new().with_content_response_on_write(ContentResponseOnWrite::Enabled);
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
    }

    #[tokio::test]
    async fn effective_options_falls_back_to_runtime() {
        // Runtime has ENABLED
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new()
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::Enabled)
                    .build(),
            )
            .build()
            .await
            .unwrap();

        // Driver has no override
        let driver_options = DriverOptions::builder(test_account()).build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation has no override - should fall back to runtime's ENABLED
        let op_options = OperationOptions::new();
        let effective = driver.effective_runtime_options(&op_options);
        assert_eq!(
            effective.content_response_on_write,
            Some(ContentResponseOnWrite::Enabled)
        );
    }

    #[test]
    fn endpoint_for_write_region_uses_service_uri() {
        let account = AccountReference::with_master_key(
            Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
            "test-key",
        );

        let region = AccountRegion {
            name: Region::new("West US"),
            database_account_endpoint: "https://myaccount-westus.documents.azure.com:443/"
                .to_string(),
        };

        let endpoint = CosmosDriver::endpoint_for_write_region(&account, Some(&region));
        assert_eq!(
            endpoint.url().host_str(),
            Some("myaccount-westus.documents.azure.com")
        );
        assert_eq!(endpoint.url().port_or_known_default(), Some(443));
    }

    #[test]
    fn endpoint_for_write_region_falls_back_when_none() {
        let account = AccountReference::with_master_key(
            Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
            "test-key",
        );

        let endpoint = CosmosDriver::endpoint_for_write_region(&account, None);
        assert_eq!(endpoint.url().as_str(), account.endpoint().as_str());
    }

    #[test]
    fn endpoint_for_write_region_falls_back_for_invalid_url() {
        let account = AccountReference::with_master_key(
            Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
            "test-key",
        );

        let region = AccountRegion {
            name: Region::new("westus"),
            database_account_endpoint: "not-a-valid-url".to_string(),
        };

        let endpoint = CosmosDriver::endpoint_for_write_region(&account, Some(&region));
        assert_eq!(endpoint.url().as_str(), account.endpoint().as_str());
    }

    #[test]
    fn parse_account_properties_uses_first_writable_and_readable_regions() {
        let payload = br#"{
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [
                { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
                { "name": "East US", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }
            ],
            "readableLocations": [
                { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
                { "name": " East US ", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }
            ],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }"#;

        let properties = CosmosDriver::parse_account_properties_payload(payload).unwrap();

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert_eq!(properties.readable_regions().len(), 2);
        assert_eq!(properties.readable_regions()[0].as_str(), "westus2");
        assert_eq!(properties.readable_regions()[1].as_str(), "eastus");
    }

    #[test]
    fn parse_account_properties_returns_none_when_locations_missing() {
        let payload = br#"{
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 0, "maxReplicasetSize": 0 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 0, "maxReplicasetSize": 0 },
            "readPolicy": { "primaryReadCoefficient": 0, "secondaryReadCoefficient": 0 },
            "queryEngineConfiguration": "{}"
        }"#;

        let properties = CosmosDriver::parse_account_properties_payload(payload).unwrap();

        assert!(properties.write_region().is_none());
        assert!(properties.readable_regions().is_empty());
    }
}
