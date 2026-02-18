// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver instance.

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineType, RequestSentStatus,
        TransportSecurity,
    },
    models::{
        AccountEndpoint, AccountProperties, AccountReference, ActivityId, ContainerProperties,
        ContainerReference, CosmosOperation, CosmosResponseHeaders, CosmosResult, DatabaseProperties,
        DatabaseReference, RequestCharge, SubStatusCode,
    },
    options::{
        DriverOptions, OperationOptions, Region, RuntimeOptions, ThroughputControlGroupSnapshot,
    },
};
use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::{Context, Request};
use std::sync::Arc;

use super::{
    transport::{
        event_channel, is_emulator_host, uses_dataplane_pipeline, AuthorizationContext,
        EventEmitter, TrackedRequestState,
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
    fn should_retry_transport_failure(
        attempt: usize,
        max_transport_retries: usize,
        is_idempotent: bool,
        request_sent: RequestSentStatus,
    ) -> bool {
        attempt < max_transport_retries && is_idempotent && request_sent.definitely_not_sent()
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
    /// Returns a [`CosmosResult`] containing the response body, headers, and diagnostics.
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
    ///     .with_content_response_on_write(ContentResponseOnWrite::DISABLED);
    ///
    /// // let result = driver.execute_operation(operation, options).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> azure_core::Result<CosmosResult> {
        // Step 1: Derive effective runtime options
        let effective_options = self.effective_runtime_options(&options);

        // Step 2: Get effective throughput control group (if any)
        let _effective_control_group = operation.container().and_then(|container| {
            self.effective_throughput_control_group(&effective_options, container)
        });

        // Step 3: Initialize diagnostics
        let activity_id = ActivityId::new_uuid();
        let mut diagnostics_builder = DiagnosticsContextBuilder::new(
            activity_id.clone(),
            Arc::clone(self.runtime.diagnostics_options()),
        );

        // Step 4: Get authentication (guaranteed to be present by AccountReference)
        let account = operation.resource_reference().account();
        let auth = account.auth();

        // Resolve account metadata through AccountMetadataCache (demo behavior).
        // For now, derive placeholder properties from AccountReference on cache miss.
        let account_properties = self
            .runtime
            .get_or_fetch_account_properties(account, || async {
                let fallback_region = Region::new("Unknown");
                AccountProperties::new(fallback_region.clone(), vec![fallback_region])
            })
            .await;

        // Step 5: Build resource link for authorization
        let resource_ref = operation.resource_reference();
        let resource_link = resource_ref.link_for_signing();

        // Step 6: Build request URL
        let request_path = resource_ref.request_path();
        let endpoint = AccountEndpoint::from(account);
        let url = endpoint.join_path(&request_path);

        // Step 7: Determine HTTP method
        let operation_type = operation.operation_type();
        let resource_type = operation.resource_type();
        let method = operation_type.http_method();

        // Step 8: Create authorization context
        // Strip leading slash from resource link for signing
        let signing_link = resource_link.trim_start_matches('/');
        let auth_context = AuthorizationContext::new(method, resource_type, signing_link);

        // Step 9: Select and create appropriate pipeline
        let transport = self.runtime.transport();
        let is_dataplane = uses_dataplane_pipeline(resource_type, operation_type);
        let pipeline = if is_dataplane {
            transport.create_dataplane_pipeline(&endpoint, auth)
        } else {
            transport.create_metadata_pipeline(&endpoint, auth)
        };

        // Determine pipeline type and transport security for diagnostics
        let pipeline_type = if is_dataplane {
            PipelineType::DataPlane
        } else {
            PipelineType::Metadata
        };
        let transport_security = if is_emulator_host(&endpoint) {
            TransportSecurity::EmulatorWithInsecureCertificates
        } else {
            TransportSecurity::Secure
        };

        // Step 12+: Execute with a slim transport retry wrapper.
        //
        // Retry only once, and only when it is safe:
        // - operation is idempotent
        // - transport failure happened before bytes were sent
        const MAX_TRANSPORT_RETRIES: usize = 1;
        let mut attempt = 0usize;

        loop {
            let mut request = Request::new(url.clone(), method);

            if let Some(body) = operation.body() {
                request.set_body(body.to_vec());
            }

            operation
                .request_headers()
                .write_to_headers(request.headers_mut());

            if operation.request_headers().activity_id().is_none() {
                request.insert_header(
                    HeaderName::from_static("x-ms-activity-id"),
                    HeaderValue::from(diagnostics_builder.activity_id().as_str().to_owned()),
                );
            }

            if let Some(pk) = operation.partition_key() {
                let _partition_key_definition =
                    operation.container().map(|container| container.partition_key_definition());

                use azure_core::http::headers::AsHeaders;
                let pk_headers = match pk.as_headers() {
                    Ok(headers) => headers,
                    Err(e) => {
                        self.runtime
                            .clear_all_caches(Some(account), operation.container())
                            .await;
                        return Err(e);
                    }
                };

                for (name, value) in pk_headers {
                    request.insert_header(name, value);
                }
            }

            let mut ctx = Context::default();
            ctx.insert(auth_context.clone());

            let (event_sender, event_receiver) = event_channel();
            ctx.insert(EventEmitter::new(event_sender));

            let region = account_properties.write_region.clone();
            let execution_context = if diagnostics_builder.request_count() == 0 {
                ExecutionContext::Initial
            } else {
                ExecutionContext::Retry
            };
            let request_handle = diagnostics_builder.start_request(
                execution_context,
                pipeline_type,
                transport_security,
                region,
                endpoint.host().to_owned(),
            );

            let result = pipeline.send(&ctx, &mut request).await;
            let tracked_state = TrackedRequestState::collect(event_receiver);

            match result {
                Ok(response) => {
                    let status_code = response.status();
                    let cosmos_headers = CosmosResponseHeaders::from_headers(response.headers());
                    let sub_status = cosmos_headers.substatus();

                    if let Some(charge) = response
                        .headers()
                        .get_optional_str(&CosmosResponseHeaders::request_charge_header_name())
                        .and_then(|s| s.parse::<f64>().ok())
                        .map(RequestCharge::new)
                    {
                        diagnostics_builder.update_request(request_handle, |req| {
                            req.with_charge(charge);
                        });
                    }

                    if let Some(activity_id) = cosmos_headers.activity_id().cloned() {
                        diagnostics_builder.update_request(request_handle, |req| {
                            req.with_activity_id(activity_id);
                        });
                    }

                    if let Some(session_token) = cosmos_headers.session_token() {
                        diagnostics_builder.update_request(request_handle, |req| {
                            req.with_session_token(session_token.to_string());
                        });
                    }

                    if let Some(sub_status) = sub_status {
                        diagnostics_builder.update_request(request_handle, |req| {
                            req.with_sub_status(sub_status);
                        });
                    }

                    for event in tracked_state.into_events() {
                        diagnostics_builder.add_event(request_handle, event);
                    }

                    diagnostics_builder.complete_request(request_handle, status_code, sub_status);
                    diagnostics_builder.set_operation_status(status_code, sub_status);

                    let body = response.into_body();
                    let diagnostics = Arc::new(diagnostics_builder.complete());

                    return Ok(CosmosResult::new(
                        body.as_ref().to_vec(),
                        cosmos_headers,
                        diagnostics,
                    ));
                }
                Err(e) => {
                    let request_sent = tracked_state.request_sent_status_with_error(&e);
                    let error_message = e.to_string();

                    for event in tracked_state.into_events() {
                        diagnostics_builder.add_event(request_handle, event);
                    }

                    diagnostics_builder.update_request(request_handle, |req| {
                        req.with_error(error_message.clone());
                        req.with_sub_status(SubStatusCode::TRANSPORT_GENERATED_503);
                    });

                    diagnostics_builder.fail_request(request_handle, error_message, request_sent);

                    let should_retry = Self::should_retry_transport_failure(
                        attempt,
                        MAX_TRANSPORT_RETRIES,
                        operation.is_idempotent(),
                        request_sent,
                    );

                    if should_retry {
                        attempt += 1;
                        self.runtime
                            .clear_all_caches(Some(account), operation.container())
                            .await;
                        continue;
                    }

                    diagnostics_builder.set_operation_status(
                        azure_core::http::StatusCode::ServiceUnavailable,
                        Some(SubStatusCode::TRANSPORT_GENERATED_503),
                    );

                    self.runtime
                        .clear_all_caches(Some(account), operation.container())
                        .await;
                    return Err(e);
                }
            }
        }
    }
}

impl CosmosDriver {
    /// Resolves a container by database and container name.
    ///
    /// Reads the database and container from the service to obtain their
    /// resource IDs (RIDs) and container properties (partition key, unique key
    /// policy). The resolved [`ContainerReference`] is cached so that
    /// subsequent calls for the same database/container return immediately
    /// without a network round-trip.
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
    /// // Resolve the container (fetched from service on first call, cached after)
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
        let account = self.account().clone();
        let endpoint = account.endpoint().to_string();
        let db_name_key = db_name.to_owned();
        let container_name_key = container_name.to_owned();
        let db_name = db_name_key.clone();
        let container_name = container_name_key.clone();

        self.runtime
            .resolve_container_by_name(&endpoint, &db_name_key, &container_name_key, || async {
                // Cache miss — read database and container from the service.
                let db_ref = DatabaseReference::from_name(account.clone(), db_name.clone());
                let options = OperationOptions::new();

                // 1. Read the database to obtain its _rid.
                let db_result = self
                    .execute_operation(
                        CosmosOperation::read_database(db_ref.clone()),
                        options.clone(),
                    )
                    .await?;
                let db_props: DatabaseProperties = serde_json::from_slice(db_result.body())
                    .map_err(|e| {
                        azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
                    })?;
                let db_rid = db_props.system_properties.rid.ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "database response missing _rid",
                    )
                })?;

                // 2. Read the container to obtain its _rid and properties.
                let container_result = self
                    .execute_operation(
                        CosmosOperation::read_container_by_name(db_ref, container_name.clone()),
                        options,
                    )
                    .await?;
                let container_props: ContainerProperties = serde_json::from_slice(container_result.body())
                    .map_err(|e| {
                        azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
                    })?;
                let container_rid = container_props.system_properties.rid.clone().ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "container response missing _rid",
                    )
                })?;

                Ok(ContainerReference::new(
                    account,
                    db_name,
                    db_rid,
                    container_name,
                    container_rid,
                    &container_props,
                ))
            })
            .await
            .map(|container| container.as_ref().clone())
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
        // machine_id is always available
        assert!(!runtime.machine_id().is_empty());
    }

    #[tokio::test]
    async fn builder_sets_runtime_options() {
        let opts = RuntimeOptions::builder()
            .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
            .build();

        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_runtime_options(opts)
            .build()
            .await
            .unwrap();

        let snapshot = runtime.runtime_options().snapshot();
        assert_eq!(
            snapshot.content_response_on_write,
            Some(ContentResponseOnWrite::DISABLED)
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
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::ENABLED)
                    .build(),
            )
            .build()
            .await
            .unwrap();

        // Driver has DISABLED
        let driver_options = DriverOptions::builder(test_account())
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::DISABLED)
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
            OperationOptions::new().with_content_response_on_write(ContentResponseOnWrite::ENABLED);
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
            .with_runtime_options(
                RuntimeOptions::builder()
                    .with_content_response_on_write(ContentResponseOnWrite::ENABLED)
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
            Some(ContentResponseOnWrite::ENABLED)
        );
    }

    #[tokio::test]
    async fn machine_id_always_available() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

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

    #[test]
    fn retry_gate_allows_only_idempotent_not_sent_with_budget() {
        assert!(CosmosDriver::should_retry_transport_failure(
            0,
            1,
            true,
            RequestSentStatus::NotSent
        ));
    }

    #[test]
    fn retry_gate_blocks_when_request_may_have_been_sent() {
        assert!(!CosmosDriver::should_retry_transport_failure(
            0,
            1,
            true,
            RequestSentStatus::Unknown
        ));
        assert!(!CosmosDriver::should_retry_transport_failure(
            0,
            1,
            true,
            RequestSentStatus::Sent
        ));
    }

    #[test]
    fn retry_gate_blocks_non_idempotent_or_exhausted_budget() {
        assert!(!CosmosDriver::should_retry_transport_failure(
            0,
            1,
            false,
            RequestSentStatus::NotSent
        ));
        assert!(!CosmosDriver::should_retry_transport_failure(
            1,
            1,
            true,
            RequestSentStatus::NotSent
        ));
    }
}
