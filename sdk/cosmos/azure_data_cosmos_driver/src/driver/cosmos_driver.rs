// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver instance.

use crate::{
    diagnostics::{DiagnosticsContextBuilder, PipelineType, TransportSecurity},
    driver::routing::{CosmosEndpoint, LocationStateStore},
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
use futures::future::BoxFuture;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use super::{
    cache::AccountRegion,
    transport::{
        cosmos_headers,
        http_client_factory::NegotiatedHttpVersion,
        is_emulator_host, request_signing, uses_dataplane_pipeline, AuthorizationContext,
        CosmosTransport,
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
    /// Per-account transport (created after HTTP/2 probe during initialization).
    /// Wrapped in `Arc<RwLock<...>>` so the metadata refresh callback can
    /// re-probe the HTTP version and swap the transport if needed.
    transport: Arc<RwLock<Arc<CosmosTransport>>>,
    /// Shared operation routing state for multi-region failover.
    location_state_store: Arc<LocationStateStore>,
    /// Resolved default for max failover retries (from env or hardcoded default).
    default_max_failover_retries: u32,
    /// Resolved default for max session retries (from env or None = compute at operation time).
    default_max_session_retries: Option<u32>,
}

impl CosmosDriver {
    /// Fetches account properties using the bootstrap transport.
    ///
    /// This is used during initialization (before the per-account transport exists).
    async fn fetch_account_properties_with_runtime(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let transport = runtime.bootstrap_transport();
        let metadata_transport = transport.get_metadata_transport(&endpoint)?;
        Self::fetch_account_properties_with_transport(&metadata_transport, account).await
    }

    /// Probes the gateway's HTTP version and returns the negotiated version.
    ///
    /// Tries HTTP/2-only first. If that fails with a connection/protocol error,
    /// falls back to HTTP/1.1. The returned version is used to create the
    /// per-account `CosmosTransport`.
    async fn probe_http_version(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
    ) -> azure_core::Result<(NegotiatedHttpVersion, super::cache::AccountProperties)> {
        if !runtime.connection_pool().is_http2_allowed() {
            // User explicitly disabled HTTP/2 — skip the probe.
            let props =
                Self::fetch_account_properties_with_runtime(runtime, account).await?;
            return Ok((NegotiatedHttpVersion::Http11, props));
        }

        // Try HTTP/2-only via the bootstrap transport (which is HTTP/2-only).
        match Self::fetch_account_properties_with_runtime(runtime, account).await {
            Ok(props) => Ok((NegotiatedHttpVersion::Http2, props)),
            Err(e) => {
                // If the failure is a connection or protocol error, fall back to HTTP/1.1.
                let is_protocol_failure = matches!(
                    e.kind(),
                    azure_core::error::ErrorKind::Connection | azure_core::error::ErrorKind::Io
                );
                if !is_protocol_failure {
                    // Non-transport error (auth, 4xx, etc.) — propagate as-is.
                    return Err(e);
                }

                tracing::info!(
                    endpoint = %AccountEndpoint::from(account),
                    error = %e,
                    "HTTP/2 probe failed; falling back to HTTP/1.1"
                );

                // Build a temporary HTTP/1.1 transport for the fallback fetch.
                let pool = runtime.connection_pool();
                let factory = runtime.http_client_factory();
                let fallback_config =
                    super::transport::http_client_factory::HttpClientConfig::bootstrap_http11_fallback(pool);
                let fallback_client = factory.build(pool, fallback_config)?;
                let fallback_transport =
                    super::transport::adaptive_transport::AdaptiveTransport::Gateway(fallback_client);

                let props =
                    Self::fetch_account_properties_with_transport(&fallback_transport, account)
                        .await?;
                Ok((NegotiatedHttpVersion::Http11, props))
            }
        }
    }

    /// Fetches account properties using a specific adaptive transport.
    async fn fetch_account_properties_with_transport(
        transport: &super::transport::adaptive_transport::AdaptiveTransport,
        account: &AccountReference,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let mut request = Request::new(endpoint.join_path("/"), azure_core::http::Method::Get);
        cosmos_headers::apply_cosmos_headers(
            &mut request,
            &azure_core::http::headers::HeaderValue::from("probe".to_owned()),
        );
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

        let response = transport.send(&request).await?;
        let raw = response.try_into_raw_response().await?;
        Self::parse_account_properties_payload(raw.body())
    }

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
        Self::refresh_account_properties(&self.runtime, account, &self.transport).await
    }

    /// Fetches account properties using the current per-account transport.
    ///
    /// If the fetch fails with a protocol/connection error, re-probes the HTTP
    /// version and swaps the transport if it has changed (e.g., HTTP/2 was
    /// disabled on the gateway since the last probe).
    async fn refresh_account_properties(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_lock: &Arc<RwLock<Arc<CosmosTransport>>>,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let current_transport = transport_lock
            .read()
            .expect("transport lock poisoned")
            .clone();
        let endpoint = AccountEndpoint::from(account);
        let metadata_transport = current_transport.get_metadata_transport(&endpoint)?;

        match Self::fetch_account_properties_with_transport(&metadata_transport, account).await {
            Ok(props) => Ok(props),
            Err(e) => {
                let is_protocol_failure = matches!(
                    e.kind(),
                    azure_core::error::ErrorKind::Connection | azure_core::error::ErrorKind::Io
                );

                if !is_protocol_failure || !runtime.connection_pool().is_http2_allowed() {
                    return Err(e);
                }

                // The current transport failed with a protocol error.
                // Re-probe to detect if the HTTP version changed.
                let current_version = current_transport.negotiated_version();
                let fallback_version = match current_version {
                    NegotiatedHttpVersion::Http2 => NegotiatedHttpVersion::Http11,
                    NegotiatedHttpVersion::Http11 => NegotiatedHttpVersion::Http2,
                };

                tracing::info!(
                    endpoint = %endpoint,
                    current = ?current_version,
                    fallback = ?fallback_version,
                    error = %e,
                    "Metadata refresh failed; trying alternate HTTP version"
                );

                // Build a temporary transport with the alternate version.
                let fallback_transport = CosmosTransport::with_factory(
                    runtime.connection_pool().clone(),
                    Arc::clone(runtime.http_client_factory()),
                    fallback_version,
                )?;
                let fallback_metadata =
                    fallback_transport.get_metadata_transport(&endpoint)?;

                let props = Self::fetch_account_properties_with_transport(
                    &fallback_metadata,
                    account,
                )
                .await?;

                // The fallback succeeded — swap the transport.
                tracing::info!(
                    endpoint = %endpoint,
                    new_version = ?fallback_version,
                    "Switching to alternate HTTP version after successful re-probe"
                );
                *transport_lock.write().expect("transport lock poisoned") =
                    Arc::new(fallback_transport);

                Ok(props)
            }
        }
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
        let account = options.account().clone();
        let account_endpoint = AccountEndpoint::from(&account);
        let default_endpoint = CosmosEndpoint::global(account.endpoint().clone());

        // Shared transport lock — used by both the driver and the refresh callback.
        let transport: Arc<RwLock<Arc<CosmosTransport>>> =
            Arc::new(RwLock::new(Arc::clone(runtime.bootstrap_transport())));

        let runtime_for_callback = runtime.clone();
        let account_for_callback = account.clone();
        let transport_for_callback = Arc::clone(&transport);
        let refresh_callback = Arc::new(move || {
            let runtime = runtime_for_callback.clone();
            let account = account_for_callback.clone();
            let transport_lock = Arc::clone(&transport_for_callback);
            let fut: BoxFuture<'static, azure_core::Result<super::cache::AccountProperties>> =
                Box::pin(async move {
                    CosmosDriver::refresh_account_properties(&runtime, &account, &transport_lock)
                        .await
                });
            fut
        });

        let endpoint_unavailability_ttl = runtime
            .runtime_options()
            .snapshot()
            .endpoint_unavailability_ttl
            .unwrap_or_else(|| {
                std::env::var("AZURE_COSMOS_ENDPOINT_UNAVAILABLE_TTL_MS")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(Duration::from_millis)
                    .unwrap_or(Duration::from_secs(60))
            });

        let location_state_store = Arc::new(LocationStateStore::new(
            runtime.account_metadata_cache().clone(),
            account_endpoint,
            default_endpoint,
            refresh_callback,
            runtime.connection_pool().is_gateway20_allowed(),
            endpoint_unavailability_ttl,
        ));

        let default_max_failover_retries = runtime
            .runtime_options()
            .snapshot()
            .max_failover_retry_count
            .unwrap_or_else(|| {
                std::env::var("AZURE_COSMOS_FAILOVER_RETRY_COUNT")
                    .ok()
                    .and_then(|v| v.parse::<u32>().ok())
                    .unwrap_or(3)
            });

        let default_max_session_retries = runtime
            .runtime_options()
            .snapshot()
            .max_session_retry_count
            .or_else(|| {
                std::env::var("AZURE_COSMOS_SESSION_RETRY_COUNT")
                    .ok()
                    .and_then(|v| v.parse::<u32>().ok())
            });

        Self {
            runtime: runtime.clone(),
            options,
            transport,
            location_state_store,
            default_max_failover_retries,
            default_max_session_retries,
        }
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

    /// Returns the current per-account transport.
    fn transport(&self) -> Arc<CosmosTransport> {
        self.transport.read().expect("transport lock poisoned").clone()
    }

    /// Eagerly primes the account metadata cache and creates the per-account transport.
    ///
    /// Performs an HTTP/2 probe to detect protocol support, then creates the
    /// appropriate transport (sharded HTTP/2 or unsharded HTTP/1.1). Also caches
    /// the account properties for regional endpoint resolution.
    ///
    /// This method is called automatically by
    /// [`CosmosDriverRuntime::get_or_create_driver`](crate::CosmosDriverRuntime::get_or_create_driver)
    /// on a best-effort basis. Callers may invoke it again to retry if the
    /// initial attempt failed (the result is idempotent).
    pub async fn initialize(&self) -> azure_core::Result<()> {
        let account = self.options.account();
        let account_endpoint = AccountEndpoint::from(account);

        // Probe HTTP version and fetch account properties in one step.
        let (negotiated_version, properties) =
            Self::probe_http_version(&self.runtime, account).await?;

        tracing::info!(
            endpoint = %account_endpoint,
            version = ?negotiated_version,
            "HTTP version negotiated for account"
        );

        // Cache the properties.
        self.runtime
            .account_metadata_cache()
            .get_or_fetch(account_endpoint, || async { Ok(properties) })
            .await?;

        // Create the per-account transport with the negotiated version.
        let new_transport = Arc::new(CosmosTransport::with_factory(
            self.runtime.connection_pool().clone(),
            Arc::clone(self.runtime.http_client_factory()),
            negotiated_version,
        )?);

        *self.transport.write().expect("transport lock poisoned") = new_transport;
        Ok(())
    }

    /// Eagerly primes the container metadata cache.
    ///
    /// Resolves container properties (partition key definition, resource ID)
    /// and caches them so that subsequent operations targeting this container
    /// can skip the metadata lookup round-trip.
    ///
    /// Returns an error if the container does not exist or is unreachable.
    pub async fn prime_container(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> azure_core::Result<()> {
        self.resolve_container_by_name(db_name, container_name)
            .await?;
        Ok(())
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
        let mut effective_options = self.effective_runtime_options(&options);

        // Fill in resolved defaults for retry counts (env vars read once at construction).
        if effective_options.max_failover_retry_count.is_none() {
            effective_options.max_failover_retry_count = Some(self.default_max_failover_retries);
        }
        if effective_options.max_session_retry_count.is_none() {
            effective_options.max_session_retry_count = self.default_max_session_retries;
        }

        // Step 2: Resolve effective throughput control group (if any).
        // Step 1 transport pipeline does not consume this yet.
        // TODO(Step 2): wire resolved throughput control into operation/transport execution.
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

        // Keep the operation routing snapshot in sync with current account metadata.
        // Uses CAS to preserve unavailable_endpoints marks set by concurrent operations.
        // Skips the CAS loop when the etag matches (same server version).
        self.location_state_store.sync_account_properties(
            account_properties.as_ref(),
            self.location_state_store.default_endpoint(),
        );

        let write_region = account_properties.write_account_region();
        let endpoint = Self::endpoint_for_write_region(account, write_region);

        // Step 5: Select the adaptive transport context for the chosen pipeline
        let transport = self.transport();
        let operation_type = operation.operation_type();
        let resource_type = operation.resource_type();
        let is_dataplane = uses_dataplane_pipeline(resource_type, operation_type);
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

        let user_agent = azure_core::http::headers::HeaderValue::from(
            self.runtime.user_agent().as_str().to_owned(),
        );

        // Step 7: Execute via the new operation pipeline
        super::pipeline::operation_pipeline::execute_operation_pipeline(
            &operation,
            &options,
            &effective_options,
            self.location_state_store.as_ref(),
            &transport,
            &endpoint,
            auth,
            &user_agent,
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
