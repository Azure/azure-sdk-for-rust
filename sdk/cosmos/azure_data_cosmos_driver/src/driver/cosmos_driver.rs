// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver instance.

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, PipelineType, TransportHttpVersion, TransportSecurity,
    },
    driver::routing::{session_manager::SessionManager, CosmosEndpoint, LocationStateStore},
    models::{
        AccountEndpoint, AccountReference, ActivityId, ContainerProperties, ContainerReference,
        CosmosOperation, DatabaseProperties, DatabaseReference,
    },
    options::{
        ConnectionPoolOptions, DiagnosticsOptions, DriverOptions, OperationOptions,
        OperationOptionsView, ThroughputControlGroupSnapshot,
    },
};
use arc_swap::ArcSwap;
use futures::future::BoxFuture;
use std::error::Error as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use super::{
    cache::AccountRegion,
    transport::{
        cosmos_headers, cosmos_transport_client::HttpRequest, is_emulator_host, request_signing,
        uses_dataplane_pipeline, AuthorizationContext, CosmosTransport,
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
    runtime: Arc<CosmosDriverRuntime>,
    /// Driver-level options including account reference.
    options: DriverOptions,
    /// Per-account transport (created after HTTP/2 probe during initialization).
    /// Wrapped in `Arc<ArcSwap<...>>` so the metadata refresh callback can
    /// re-probe the HTTP version and swap the transport atomically.
    /// Reads are lock-free (no cache-line contention between readers).
    transport: Arc<ArcSwap<CosmosTransport>>,
    /// Shared operation routing state for multi-region failover.
    location_state_store: Arc<LocationStateStore>,
    /// Session token cache for session consistency.
    session_manager: SessionManager,
    /// Set to `true` after [`initialize()`](Self::initialize) completes successfully.
    /// Operations check this flag to fail fast if the driver is used before
    /// initialization. In normal usage `get_or_create_driver` awaits `initialize()`
    /// before returning, so this guard only catches misuse.
    initialized: AtomicBool,
}

impl CosmosDriver {
    #[cfg(feature = "reqwest")]
    fn has_explicit_http2_incompatibility(error: &azure_core::Error) -> bool {
        let mut source = error.source();
        while let Some(cause) = source {
            if let Some(h2_error) = cause.downcast_ref::<h2::Error>() {
                return matches!(
                    h2_error.reason(),
                    Some(
                        h2::Reason::HTTP_1_1_REQUIRED
                            | h2::Reason::PROTOCOL_ERROR
                            | h2::Reason::FRAME_SIZE_ERROR
                    )
                );
            }
            source = cause.source();
        }
        false
    }

    #[cfg(not(feature = "reqwest"))]
    fn has_explicit_http2_incompatibility(_error: &azure_core::Error) -> bool {
        false
    }

    fn should_downgrade_http2(
        current_version: TransportHttpVersion,
        error: &azure_core::Error,
        http2_allowed: bool,
    ) -> bool {
        http2_allowed
            && matches!(current_version, TransportHttpVersion::Http2)
            && Self::has_explicit_http2_incompatibility(error)
    }

    fn alternate_http_version(current_version: TransportHttpVersion) -> TransportHttpVersion {
        match current_version {
            TransportHttpVersion::Http2 => TransportHttpVersion::Http11,
            TransportHttpVersion::Http11 => TransportHttpVersion::Http2,
        }
    }

    fn build_metadata_transport_for_version(
        connection_pool: &ConnectionPoolOptions,
        http_client_factory: Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        version: TransportHttpVersion,
        endpoint: &AccountEndpoint,
    ) -> azure_core::Result<(
        CosmosTransport,
        super::transport::adaptive_transport::AdaptiveTransport,
    )> {
        let transport =
            CosmosTransport::with_factory(connection_pool.clone(), http_client_factory, version)?;
        let metadata_transport = transport.get_metadata_transport(endpoint)?;
        Ok((transport, metadata_transport))
    }

    async fn fetch_account_properties_with_version(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        version: TransportHttpVersion,
    ) -> azure_core::Result<(super::cache::AccountProperties, CosmosTransport)> {
        let endpoint = AccountEndpoint::from(account);
        let (transport, metadata_transport) = Self::build_metadata_transport_for_version(
            runtime.connection_pool(),
            Arc::clone(runtime.http_client_factory()),
            version,
            &endpoint,
        )?;
        let user_agent = Self::user_agent_header(runtime);
        let props = Self::fetch_account_properties_with_transport(
            &metadata_transport,
            account,
            &user_agent,
        )
        .await?;
        Ok((props, transport))
    }

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
        let user_agent =
            azure_core::http::headers::HeaderValue::from(runtime.user_agent().as_str().to_owned());
        Self::fetch_account_properties_with_transport(&metadata_transport, account, &user_agent)
            .await
    }

    /// Probes the gateway's HTTP version and returns the negotiated version.
    ///
    /// Tries HTTP/2-only first. If that fails with an explicit HTTP/2
    /// incompatibility signal, falls back to HTTP/1.1 using the same
    /// emulator-aware metadata transport selection as the steady-state path.
    ///
    /// If the primary endpoint fails, tries each backup endpoint in order.
    ///
    /// Callers that need to force HTTP/1.1 can disable HTTP/2 in
    /// [`crate::options::ConnectionPoolOptionsBuilder::with_is_http2_allowed`].
    /// The returned version is used to create the per-account `CosmosTransport`.
    async fn fetch_initial_account_properties(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
    ) -> azure_core::Result<(TransportHttpVersion, super::cache::AccountProperties)> {
        match Self::fetch_initial_account_properties_for_endpoint(runtime, account).await {
            Ok(result) => Ok(result),
            Err(primary_error) if !account.backup_endpoints().is_empty() => {
                tracing::warn!(
                    endpoint = %AccountEndpoint::from(account),
                    error = %primary_error,
                    "primary endpoint probe failed; trying backup endpoints"
                );

                for backup_url in account.backup_endpoints() {
                    let backup_account = Self::with_endpoint(account, backup_url.clone());
                    match Self::fetch_initial_account_properties_for_endpoint(
                        runtime,
                        &backup_account,
                    )
                    .await
                    {
                        Ok(result) => {
                            // The HTTP version is negotiated with the backup's gateway,
                            // which may differ from the primary. Any mismatch is
                            // self-correcting: handle_refresh_failure will re-probe
                            // when the primary recovers.
                            return Ok(result);
                        }
                        Err(e) => {
                            tracing::warn!(
                                backup_endpoint = %backup_url,
                                error = %e,
                                "backup endpoint probe failed; trying next"
                            );
                        }
                    }
                }

                tracing::error!(
                    endpoint = %AccountEndpoint::from(account),
                    backup_count = account.backup_endpoints().len(),
                    "all endpoints exhausted during HTTP version probe"
                );
                Err(primary_error)
            }
            Err(error) => Err(error),
        }
    }

    /// Probes the HTTP version for a single endpoint.
    async fn fetch_initial_account_properties_for_endpoint(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
    ) -> azure_core::Result<(TransportHttpVersion, super::cache::AccountProperties)> {
        if !runtime.connection_pool().is_http2_allowed() {
            // User explicitly disabled HTTP/2 — skip the probe.
            let (props, _) = Self::fetch_account_properties_with_version(
                runtime,
                account,
                TransportHttpVersion::Http11,
            )
            .await?;
            return Ok((TransportHttpVersion::Http11, props));
        }

        // Try HTTP/2-only via the bootstrap transport (which is HTTP/2-only).
        match Self::fetch_account_properties_with_runtime(runtime, account).await {
            Ok(props) => {
                tracing::trace!(
                    endpoint = %AccountEndpoint::from(account),
                    "HTTP/2 probe succeeded; using HTTP/2 transport"
                );
                Ok((TransportHttpVersion::Http2, props))
            }
            Err(error)
                if Self::should_downgrade_http2(
                    TransportHttpVersion::Http2,
                    &error,
                    runtime.connection_pool().is_http2_allowed(),
                ) =>
            {
                tracing::warn!(
                    endpoint = %AccountEndpoint::from(account),
                    error = %error,
                    "HTTP/2 probe failed with protocol incompatibility; falling back to HTTP/1.1"
                );

                let (props, _) = Self::fetch_account_properties_with_version(
                    runtime,
                    account,
                    TransportHttpVersion::Http11,
                )
                .await?;
                Ok((TransportHttpVersion::Http11, props))
            }
            Err(error) => Err(error),
        }
    }

    /// Creates a temporary `AccountReference` targeting a single backup endpoint.
    ///
    /// `backup_endpoints` are intentionally omitted: this reference is used for
    /// a single-endpoint probe inside the fallback loop and must not trigger
    /// its own recursive fallback.
    fn with_endpoint(account: &AccountReference, endpoint: Url) -> AccountReference {
        AccountReference::builder(endpoint)
            .auth(account.auth().clone())
            .build()
            .expect("auth is always present when cloned from existing AccountReference")
    }

    /// Fetches account properties using a specific adaptive transport.
    async fn fetch_account_properties_with_transport(
        transport: &super::transport::adaptive_transport::AdaptiveTransport,
        account: &AccountReference,
        user_agent: &azure_core::http::headers::HeaderValue,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let mut request = HttpRequest {
            url: endpoint.join_path("/"),
            method: azure_core::http::Method::Get,
            headers: azure_core::http::headers::Headers::new(),
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        };
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

        let response = transport.send(&request).await.map_err(|e| e.error)?;
        let props = Self::parse_account_properties_payload(&response.body)?;
        tracing::info!(
            endpoint = %endpoint,
            write_region = ?props.write_region(),
            "AccountProperties retrieved successfully"
        );
        Ok(props)
    }

    fn parse_account_properties_payload(
        payload: &[u8],
    ) -> azure_core::Result<super::cache::AccountProperties> {
        serde_json::from_slice(payload)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))
    }

    fn user_agent_header(runtime: &CosmosDriverRuntime) -> azure_core::http::headers::HeaderValue {
        azure_core::http::headers::HeaderValue::from(runtime.user_agent().as_str().to_owned())
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
        Self::refresh_account_properties(&self.runtime, account, &self.transport, None).await
    }

    /// Fetches account properties using the current per-account transport.
    ///
    /// Uses the existing transport for the refresh. If the primary endpoint
    /// fails (including HTTP version fallback), tries regional endpoints from
    /// previous account metadata as a last resort.
    ///
    /// - **HTTP/1.1 success**: opportunistically re-probes HTTP/2 and upgrades
    ///   the transport on success.
    /// - **HTTP/2 incompatibility failure**: falls back to HTTP/1.1 and swaps
    ///   the transport.
    /// - **Other transport failure with HTTP/2**: re-probes fully (may discover
    ///   the gateway now requires HTTP/1.1).
    /// - **All primary attempts fail**: tries regional endpoints from
    ///   `previous_props` (the last successfully fetched account metadata).
    ///
    /// This avoids creating transient transport infrastructure on every refresh
    /// cycle. A fresh probe only occurs when the driver is currently pinned to
    /// HTTP/1.1 or when the active transport actually fails, both of which are
    /// expected to be rare in steady-state operation.
    async fn refresh_account_properties(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        previous_props: Option<Arc<super::cache::AccountProperties>>,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let current_transport = transport_holder.load_full();
        let current_version = current_transport.negotiated_version();
        let endpoint = AccountEndpoint::from(account);
        let metadata_transport = current_transport.get_metadata_transport(&endpoint)?;

        let user_agent = Self::user_agent_header(runtime);
        match Self::fetch_account_properties_with_transport(
            &metadata_transport,
            account,
            &user_agent,
        )
        .await
        {
            Ok(props) => {
                Self::maybe_restore_http2_after_refresh(
                    runtime,
                    account,
                    transport_holder,
                    current_version,
                    &endpoint,
                )
                .await;
                Ok(props)
            }
            Err(error) => {
                match Self::handle_refresh_failure(
                    runtime,
                    account,
                    transport_holder,
                    current_version,
                    &endpoint,
                    error,
                )
                .await
                {
                    Ok(props) => Ok(props),
                    Err(primary_error) => {
                        // Primary endpoint failed — try regional endpoints from previous metadata.
                        Self::refresh_via_regional_endpoints(
                            runtime,
                            account,
                            transport_holder,
                            &endpoint,
                            primary_error,
                            previous_props,
                        )
                        .await
                    }
                }
            }
        }
    }

    /// Attempts account metadata refresh via regional endpoints.
    ///
    /// Called when the primary global endpoint is unreachable. Iterates through
    /// readable regional endpoints from the previous account metadata and tries
    /// each one.
    async fn refresh_via_regional_endpoints(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        primary_endpoint: &AccountEndpoint,
        primary_error: azure_core::Error,
        previous_props: Option<Arc<super::cache::AccountProperties>>,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        let Some(cached_props) = previous_props else {
            return Err(primary_error);
        };

        // Parse regional URLs once, filtering out the primary and any invalid URLs.
        let regional_endpoints: Vec<Url> = cached_props
            .readable_locations
            .iter()
            .filter_map(|loc| {
                let url = Url::parse(&loc.database_account_endpoint).ok()?;
                let ep = AccountEndpoint::from(url.clone());
                if ep == *primary_endpoint {
                    None
                } else {
                    Some(url)
                }
            })
            .collect();

        if regional_endpoints.is_empty() {
            return Err(primary_error);
        }

        tracing::warn!(
            endpoint = %primary_endpoint,
            error = %primary_error,
            "primary endpoint refresh failed; trying regional endpoints"
        );

        for regional_url in &regional_endpoints {
            let regional_account = Self::with_endpoint(account, regional_url.clone());
            let regional_ep = AccountEndpoint::from(&regional_account);
            let current_transport = transport_holder.load_full();
            let Ok(regional_transport) = current_transport.get_metadata_transport(&regional_ep)
            else {
                continue;
            };

            let user_agent = Self::user_agent_header(runtime);
            match Self::fetch_account_properties_with_transport(
                &regional_transport,
                &regional_account,
                &user_agent,
            )
            .await
            {
                Ok(props) => {
                    // Regional metadata may differ slightly from the primary
                    // (e.g., location ordering). This is acceptable as a transient
                    // fallback; the next successful primary refresh will restore
                    // canonical metadata.
                    return Ok(props);
                }
                Err(e) => {
                    tracing::warn!(
                        regional_endpoint = %regional_url,
                        error = %e,
                        "regional endpoint refresh failed; trying next"
                    );
                }
            }
        }

        tracing::error!(
            endpoint = %primary_endpoint,
            regional_count = regional_endpoints.len(),
            "all endpoints exhausted during account properties refresh"
        );
        Err(primary_error)
    }

    async fn maybe_restore_http2_after_refresh(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        current_version: TransportHttpVersion,
        endpoint: &AccountEndpoint,
    ) {
        if !matches!(current_version, TransportHttpVersion::Http11)
            || !runtime.connection_pool().is_http2_allowed()
        {
            return;
        }

        match Self::fetch_account_properties_with_runtime(runtime, account).await {
            Ok(_) => match CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http2,
            ) {
                Ok(transport) => {
                    transport_holder.store(Arc::new(transport));
                    tracing::info!(
                        endpoint = %endpoint,
                        "Metadata refresh restored HTTP/2 transport after successful probe"
                    );
                }
                Err(error) => {
                    tracing::warn!(
                        endpoint = %endpoint,
                        %error,
                        "HTTP/2 probe succeeded after metadata refresh, but recreating the HTTP/2 transport failed"
                    );
                }
            },
            Err(error) => {
                tracing::debug!(
                    endpoint = %endpoint,
                    %error,
                    "Metadata refresh succeeded over HTTP/1.1; HTTP/2 reprobe failed, keeping HTTP/1.1 transport"
                );
            }
        }
    }

    /// Handles a metadata refresh failure by re-probing the HTTP version.
    ///
    /// If the error indicates explicit HTTP/2 incompatibility, falls back to
    /// the alternate version directly. Otherwise, performs a full version probe
    /// to determine whether the gateway's protocol support has changed.
    async fn handle_refresh_failure(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        current_version: TransportHttpVersion,
        endpoint: &AccountEndpoint,
        error: azure_core::Error,
    ) -> azure_core::Result<super::cache::AccountProperties> {
        if Self::should_downgrade_http2(
            current_version,
            &error,
            runtime.connection_pool().is_http2_allowed(),
        ) {
            // Explicit HTTP/2 incompatibility — try the alternate version.
            let fallback_version = Self::alternate_http_version(current_version);
            tracing::warn!(
                endpoint = %endpoint,
                current = ?current_version,
                fallback = ?fallback_version,
                error = %error,
                "Metadata refresh failed with protocol incompatibility; falling back to alternate HTTP version"
            );

            let (props, fallback_transport) =
                Self::fetch_account_properties_with_version(runtime, account, fallback_version)
                    .await?;

            transport_holder.store(Arc::new(fallback_transport));

            return Ok(props);
        }

        // Not a protocol incompatibility — propagate the original error.
        Err(error)
    }

    async fn fetch_container_by_name(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> azure_core::Result<ContainerReference> {
        let db_ref = DatabaseReference::from_name(self.account().clone(), db_name.to_owned());
        let options = OperationOptions::default();

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
        let options = OperationOptions::default();

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
    pub(crate) fn new(runtime: Arc<CosmosDriverRuntime>, options: DriverOptions) -> Self {
        let account = options.account().clone();
        let account_endpoint = AccountEndpoint::from(&account);
        let default_endpoint = CosmosEndpoint::global(account.endpoint().clone());

        // Shared transport holder — used by both the driver and the refresh callback.
        // ArcSwap provides lock-free reads on the hot path (every operation)
        // and only incurs overhead on writes (transport swap, ~every 5 min).
        let transport: Arc<ArcSwap<CosmosTransport>> =
            Arc::new(ArcSwap::from(Arc::clone(runtime.bootstrap_transport())));

        let runtime_for_callback = Arc::clone(&runtime);
        let account_for_callback = account.clone();
        let transport_for_callback = Arc::clone(&transport);
        let refresh_callback = Arc::new(
            move |previous_props: Option<Arc<super::cache::AccountProperties>>| {
                let runtime = Arc::clone(&runtime_for_callback);
                let account = account_for_callback.clone();
                let transport_holder = Arc::clone(&transport_for_callback);
                let fut: BoxFuture<'static, azure_core::Result<super::cache::AccountProperties>> =
                    Box::pin(async move {
                        CosmosDriver::refresh_account_properties(
                            &runtime,
                            &account,
                            &transport_holder,
                            previous_props,
                        )
                        .await
                    });
                fut
            },
        );

        // Resolve endpoint_unavailability_ttl from driver → runtime layers, then
        // fall back to env var.
        let endpoint_unavailability_ttl = options
            .operation_options()
            .endpoint_unavailability_ttl
            .or(runtime.operation_options().endpoint_unavailability_ttl)
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

        Self {
            runtime,
            options,
            transport,
            location_state_store,
            session_manager: SessionManager::new(),
            initialized: AtomicBool::new(false),
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
    ///
    /// Lock-free via `ArcSwap::load_full()` — returns a cloned `Arc` with no
    /// reader-counter contention between concurrent callers.
    fn transport(&self) -> Arc<CosmosTransport> {
        self.transport.load_full()
    }

    /// Eagerly primes the account metadata cache and creates the per-account transport.
    ///
    /// Performs an HTTP/2 probe to detect protocol support, then creates the
    /// appropriate transport (sharded HTTP/2 or unsharded HTTP/1.1). Also caches
    /// the account properties for regional endpoint resolution.
    ///
    /// This method is called automatically by
    /// [`CosmosDriverRuntime::get_or_create_driver`](crate::CosmosDriverRuntime::get_or_create_driver).
    /// Callers may invoke it again to retry if the initial attempt failed
    /// (the result is idempotent).
    pub async fn initialize(&self) -> azure_core::Result<()> {
        let account = self.options.account();
        let account_endpoint = AccountEndpoint::from(account);

        // Probe HTTP version and fetch account properties in one step.
        let (negotiated_version, properties) =
            Self::fetch_initial_account_properties(&self.runtime, account).await?;

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

        self.transport.store(new_transport);
        self.initialized.store(true, Ordering::Release);
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

    /// Constructs an [`OperationOptionsView`] for resolving options across all layers.
    ///
    /// The view resolves options in priority order (highest first):
    /// 1. `OperationOptions` - operation-specific overrides
    /// 2. `DriverOptions` - driver-level defaults
    /// 3. `CosmosDriverRuntime` - global runtime defaults
    /// 4. Environment - env vars read at startup
    pub fn operation_options_view<'a>(
        &self,
        operation_options: &'a OperationOptions,
    ) -> OperationOptionsView<'a> {
        OperationOptionsView::new(
            Some(Arc::clone(self.runtime.env_operation_options())),
            Some(self.runtime.operation_options()),
            Some(self.options.operation_options().clone()),
            Some(operation_options),
        )
    }

    /// Computes the effective throughput control group for an operation.
    ///
    /// Resolution order:
    /// 1. Explicit group name from the resolved options — looked up in the registry
    ///    and snapshotted.
    /// 2. Default group for the operation's container.
    ///
    /// Returns `Ok(None)` if no applicable control group is found.
    ///
    /// # Errors
    ///
    /// Returns an error if an explicitly named group is not found in the registry.
    pub(crate) fn effective_throughput_control_group(
        &self,
        effective_options: &OperationOptionsView<'_>,
        container: &ContainerReference,
    ) -> azure_core::Result<Option<ThroughputControlGroupSnapshot>> {
        if let Some(name) = effective_options.throughput_control_group() {
            let group = self
                .runtime
                .get_throughput_control_group(container, name)
                .ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!(
                            "throughput control group '{}' not found in registry for container '{}'",
                            name,
                            container.name()
                        ),
                    )
                })?;
            return Ok(Some(ThroughputControlGroupSnapshot::from(group.as_ref())));
        }

        // No explicit name — fall back to the default group for the container.
        Ok(self
            .runtime
            .get_default_throughput_control_group(container)
            .map(|group| ThroughputControlGroupSnapshot::from(group.as_ref())))
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
    /// use azure_data_cosmos_driver::options::{OperationOptions, OperationOptionsBuilder, ContentResponseOnWrite};
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
    /// let options = OperationOptionsBuilder::new()
    ///     .with_content_response_on_write(ContentResponseOnWrite::Disabled)
    ///     .build();
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
        if !self.initialized.load(Ordering::Acquire) {
            let endpoint = AccountEndpoint::from(self.options.account());
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "CosmosDriver for {endpoint} has not been initialized; call initialize() or \
                     use CosmosDriverRuntime::get_or_create_driver() which initializes automatically"
                ),
            ));
        }
        tracing::debug!("operation started");

        // Step 1: Build the single OperationOptionsView for layered resolution.
        let effective_options = self.operation_options_view(&options);

        // Step 2: Resolve effective throughput control group (if any).
        let effective_control_group = match operation.container() {
            Some(container) => {
                self.effective_throughput_control_group(&effective_options, container)?
            }
            None => None,
        };

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
        if self.runtime.fault_injection_enabled() {
            #[cfg(feature = "fault_injection")]
            diagnostics_builder.set_fault_injection_enabled(true);
        }

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
            &effective_options,
            options.custom_headers(),
            self.location_state_store.as_ref(),
            &transport,
            &endpoint,
            auth,
            &user_agent,
            &activity_id,
            pipeline_type,
            transport_security,
            diagnostics_builder,
            &self.session_manager,
            account_properties
                .user_consistency_policy
                .default_consistency_level,
            effective_control_group.as_ref(),
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
    ///     .execute_operation(CosmosOperation::read_item(item), OperationOptions::default())
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
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use async_trait::async_trait;
    use azure_core::http::headers::Headers;

    use url::Url;

    use azure_core::error::ErrorKind;

    use crate::{
        driver::CosmosDriverRuntimeBuilder,
        models::AccountReference,
        options::{
            ContentResponseOnWrite, CorrelationId, OperationOptionsBuilder, UserAgentSuffix,
            WorkloadId,
        },
    };

    use super::*;
    use crate::driver::cache::AccountProperties as CachedAccountProperties;
    use crate::options::Region;
    use crate::{
        driver::transport::{
            cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError},
            http_client_factory::{HttpClientConfig, HttpClientFactory, HttpVersionPolicy},
        },
        options::ConnectionPoolOptions,
    };

    const ACCOUNT_PROPERTIES_PAYLOAD: &str = r#"{
        "_self": "",
        "id": "test",
        "_rid": "test.documents.azure.com",
        "media": "//media/",
        "addresses": "//addresses/",
        "_dbs": "//dbs/",
        "writableLocations": [
            { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }
        ],
        "readableLocations": [
            { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }
        ],
        "enableMultipleWriteLocations": false,
        "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
        "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
        "queryEngineConfiguration": "{}"
    }"#;

    fn signed_test_account(url: &str) -> AccountReference {
        AccountReference::with_master_key(Url::parse(url).unwrap(), "dGVzdA==")
    }

    #[derive(Clone, Debug)]
    enum ResponsePlan {
        Success,
        Http2Incompatible,
        ConnectionError,
    }

    #[derive(Debug)]
    struct ScriptedClient {
        plan: ResponsePlan,
    }

    #[async_trait]
    impl TransportClient for ScriptedClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            match self.plan {
                ResponsePlan::Success => Ok(HttpResponse {
                    status: 200,
                    headers: Headers::new(),
                    body: ACCOUNT_PROPERTIES_PAYLOAD.as_bytes().to_vec(),
                }),
                ResponsePlan::Http2Incompatible => Err(TransportError::new(
                    azure_core::Error::with_error(
                        ErrorKind::Io,
                        h2::Error::from(h2::Reason::HTTP_1_1_REQUIRED),
                        "http2 not supported",
                    ),
                    crate::diagnostics::RequestSentStatus::NotSent,
                )),
                ResponsePlan::ConnectionError => Err(TransportError::new(
                    azure_core::Error::with_message(
                        ErrorKind::Connection,
                        "simulated connection refused",
                    ),
                    crate::diagnostics::RequestSentStatus::NotSent,
                )),
            }
        }
    }

    #[derive(Debug)]
    struct ScriptedFactory {
        configs: Mutex<Vec<HttpClientConfig>>,
        plans: Mutex<VecDeque<ResponsePlan>>,
    }

    impl ScriptedFactory {
        fn new(plans: impl IntoIterator<Item = ResponsePlan>) -> Self {
            Self {
                configs: Mutex::new(Vec::new()),
                plans: Mutex::new(plans.into_iter().collect()),
            }
        }

        fn configs(&self) -> Vec<HttpClientConfig> {
            self.configs.lock().expect("config lock poisoned").clone()
        }
    }

    impl HttpClientFactory for ScriptedFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            config: HttpClientConfig,
        ) -> azure_core::Result<Arc<dyn TransportClient>> {
            self.configs
                .lock()
                .expect("config lock poisoned")
                .push(config);

            let plan = self
                .plans
                .lock()
                .expect("plan lock poisoned")
                .pop_front()
                .unwrap_or(ResponsePlan::Success);

            Ok(Arc::new(ScriptedClient { plan }))
        }
    }

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[tokio::test]
    async fn default_operation_options() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        assert!(runtime
            .operation_options()
            .throughput_control_group
            .is_none());
        assert!(runtime
            .operation_options()
            .max_failover_retry_count
            .is_none());
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
    async fn builder_sets_operation_options() {
        let opts = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(7)
            .build();

        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_operation_options(opts)
            .build()
            .await
            .unwrap();

        assert_eq!(
            runtime.operation_options().max_failover_retry_count,
            Some(7)
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
            .operation_options()
            .max_failover_retry_count
            .is_none());

        // Replace runtime options atomically
        let new_opts = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .build();
        runtime.set_operation_options(new_opts);

        // Now set
        assert_eq!(
            runtime.operation_options().max_failover_retry_count,
            Some(5)
        );
    }

    #[tokio::test]
    async fn effective_options_merge_priority() {
        // Build runtime (no operation options at runtime level yet)
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

        // Driver has no operation options override either
        let driver_options = DriverOptions::builder(test_account()).build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation has DISABLED - should get DISABLED from operation options view
        let op_options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .build();
        let view = driver.operation_options_view(&op_options);
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Disabled)
        );

        // Operation overrides to ENABLED - should get ENABLED
        let op_options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Enabled)
            .build();
        let view = driver.operation_options_view(&op_options);
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Enabled)
        );
    }

    #[tokio::test]
    async fn effective_options_falls_back_to_runtime() {
        // Build runtime (env-level operation options are auto-loaded)
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

        // Driver has no override
        let driver_options = DriverOptions::builder(test_account()).build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options);

        // Operation sets ENABLED - should get ENABLED from operation options view
        let op_options = OperationOptionsBuilder::new()
            .with_content_response_on_write(ContentResponseOnWrite::Enabled)
            .build();
        let view = driver.operation_options_view(&op_options);
        assert_eq!(
            view.content_response_on_write(),
            Some(&ContentResponseOnWrite::Enabled)
        );

        // Operation has no override - env has no override - should be None
        let op_options = OperationOptions::default();
        let view = driver.operation_options_view(&op_options);
        assert!(view.content_response_on_write().is_none());
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

    #[test]
    #[cfg(feature = "reqwest")]
    fn http2_reason_http11_required_triggers_http11_downgrade() {
        let error = azure_core::Error::with_error(
            ErrorKind::Io,
            h2::Error::from(h2::Reason::HTTP_1_1_REQUIRED),
            "http2 not supported",
        );

        assert!(CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn connection_error_without_http2_signal_does_not_trigger_downgrade() {
        let error = azure_core::Error::with_message(ErrorKind::Connection, "connect failed");

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn io_error_without_http2_signal_does_not_trigger_downgrade() {
        let error = azure_core::Error::with_message(ErrorKind::Io, "socket reset");

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn http11_errors_do_not_trigger_probe_back_to_http2() {
        let error = azure_core::Error::with_message(ErrorKind::Connection, "connect failed");

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http11,
            &error,
            true,
        ));
    }

    #[test]
    fn downgrade_requires_http2_to_be_enabled() {
        let error = azure_core::Error::with_message(ErrorKind::Connection, "connect failed");

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            false,
        ));
    }

    #[test]
    fn alternate_http_version_switches_between_http11_and_http2() {
        assert_eq!(
            CosmosDriver::alternate_http_version(TransportHttpVersion::Http11),
            TransportHttpVersion::Http2
        );
        assert_eq!(
            CosmosDriver::alternate_http_version(TransportHttpVersion::Http2),
            TransportHttpVersion::Http11
        );
    }

    #[test]
    fn build_metadata_transport_for_version_uses_emulator_transport_selection() {
        let connection_pool = ConnectionPoolOptions::builder()
            .with_emulator_server_cert_validation(
                crate::options::EmulatorServerCertValidation::DangerousDisabled,
            )
            .build()
            .unwrap();
        let factory = Arc::new(ScriptedFactory::new([
            ResponsePlan::Success,
            ResponsePlan::Success,
        ]));
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();

        let _ = CosmosDriver::build_metadata_transport_for_version(
            &connection_pool,
            factory.clone(),
            TransportHttpVersion::Http11,
            &endpoint,
        )
        .unwrap();

        assert!(factory.configs().iter().any(|config| {
            matches!(config.version_policy, HttpVersionPolicy::Http11Only) && config.for_emulator
        }));
    }

    #[tokio::test]
    async fn fetch_initial_account_properties_falls_back_to_http11_for_emulator_accounts() {
        // The bootstrap_metadata_only transport eagerly builds 2 unsharded
        // clients (metadata + dataplane) during runtime construction.
        // The emulator probe then lazily builds a sharded client for the
        // insecure emulator transport, and the HTTP/1.1 fallback builds
        // additional clients.
        let factory = Arc::new(ScriptedFactory::new([
            ResponsePlan::Success,           // bootstrap metadata (eager, unused)
            ResponsePlan::Success,           // bootstrap dataplane (eager, unused)
            ResponsePlan::Http2Incompatible, // emulator insecure transport shard
            ResponsePlan::Success,           // fallback HTTP/1.1 metadata
            ResponsePlan::Success,           // fallback HTTP/1.1 dataplane
            ResponsePlan::Success,           // fallback emulator insecure metadata
        ]));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_connection_pool(
                ConnectionPoolOptions::builder()
                    .with_emulator_server_cert_validation(
                        crate::options::EmulatorServerCertValidation::DangerousDisabled,
                    )
                    .build()
                    .unwrap(),
            )
            .with_http_client_factory(factory.clone())
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://localhost:8081/");

        let (version, properties) =
            CosmosDriver::fetch_initial_account_properties(&runtime, &account)
                .await
                .unwrap();

        assert_eq!(version, TransportHttpVersion::Http11);
        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert!(factory.configs().iter().any(|config| {
            matches!(config.version_policy, HttpVersionPolicy::Http11Only) && config.for_emulator
        }));
    }

    #[tokio::test]
    async fn refresh_account_properties_restores_http2_after_http11_success() {
        let factory = Arc::new(ScriptedFactory::new([ResponsePlan::Success]));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http11,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let properties =
            CosmosDriver::refresh_account_properties(&runtime, &account, &transport_holder, None)
                .await
                .unwrap();

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert_eq!(
            transport_holder.load().negotiated_version(),
            TransportHttpVersion::Http2
        );
    }

    #[tokio::test]
    async fn refresh_account_properties_keeps_http11_when_http2_reprobe_fails() {
        let factory = Arc::new(ScriptedFactory::new([
            ResponsePlan::Http2Incompatible,
            ResponsePlan::Success,
            ResponsePlan::Success,
            ResponsePlan::Success,
        ]));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http11,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let properties =
            CosmosDriver::refresh_account_properties(&runtime, &account, &transport_holder, None)
                .await
                .unwrap();

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert_eq!(
            transport_holder.load().negotiated_version(),
            TransportHttpVersion::Http11
        );
    }

    #[tokio::test]
    async fn refresh_account_properties_downgrades_to_http11_after_http2_incompatibility() {
        let factory = Arc::new(ScriptedFactory::new([
            ResponsePlan::Success,
            ResponsePlan::Success,
            ResponsePlan::Http2Incompatible,
            ResponsePlan::Success,
            ResponsePlan::Success,
        ]));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http2,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let properties =
            CosmosDriver::refresh_account_properties(&runtime, &account, &transport_holder, None)
                .await
                .unwrap();

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert_eq!(
            transport_holder.load().negotiated_version(),
            TransportHttpVersion::Http11
        );
    }

    /// Compile-time assertion that the `execute_operation` future is `Send`.
    ///
    /// This function is never called; it only needs to compile.
    /// If the future returned by `execute_operation` is not `Send`, compilation will fail.
    #[allow(dead_code, unreachable_code, unused_variables)]
    fn _assert_execute_operation_future_is_send() {
        fn assert_send<T: Send>(_: T) {}
        let driver: &CosmosDriver = todo!();
        assert_send(driver.execute_operation(todo!(), todo!()));
    }

    // Account properties with two readable locations for regional fallback tests.
    const MULTI_REGION_ACCOUNT_PROPERTIES: &str = r#"{
        "_self": "",
        "id": "test",
        "_rid": "test.documents.azure.com",
        "media": "//media/",
        "addresses": "//addresses/",
        "_dbs": "//dbs/",
        "writableLocations": [
            { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" }
        ],
        "readableLocations": [
            { "name": "West US 2", "databaseAccountEndpoint": "https://test-westus2.documents.azure.com:443/" },
            { "name": "East US", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }
        ],
        "enableMultipleWriteLocations": false,
        "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
        "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
        "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
        "queryEngineConfiguration": "{}"
    }"#;

    fn multi_region_previous_props() -> Arc<CachedAccountProperties> {
        Arc::new(serde_json::from_str(MULTI_REGION_ACCOUNT_PROPERTIES).unwrap())
    }

    #[tokio::test]
    async fn refresh_falls_back_to_regional_endpoints_when_primary_fails() {
        // Primary metadata request fails (connection error), then the
        // regional fallback succeeds on the first regional endpoint.
        let factory = Arc::new(ScriptedFactory::new([
            ResponsePlan::ConnectionError, // primary metadata
            ResponsePlan::ConnectionError, // handle_refresh_failure re-probe
            ResponsePlan::Success,         // regional endpoint succeeds
        ]));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http2,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let result = CosmosDriver::refresh_account_properties(
            &runtime,
            &account,
            &transport_holder,
            Some(multi_region_previous_props()),
        )
        .await;

        assert!(
            result.is_ok(),
            "should succeed via regional fallback: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    async fn refresh_returns_primary_error_when_all_endpoints_fail() {
        // Primary and all regional endpoints fail. Use enough ConnectionError
        // plans to cover bootstrap transport creation + all retry attempts.
        let factory = Arc::new(ScriptedFactory::new(std::iter::repeat_n(
            ResponsePlan::ConnectionError,
            20,
        )));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http2,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let result = CosmosDriver::refresh_account_properties(
            &runtime,
            &account,
            &transport_holder,
            Some(multi_region_previous_props()),
        )
        .await;

        assert!(result.is_err(), "should fail when all endpoints exhausted");
    }

    #[tokio::test]
    async fn refresh_skips_regional_fallback_without_previous_props() {
        // Primary fails and no previous properties — should return error immediately.
        let factory = Arc::new(ScriptedFactory::new(std::iter::repeat_n(
            ResponsePlan::ConnectionError,
            20,
        )));
        let runtime = CosmosDriverRuntimeBuilder::new()
            .with_http_client_factory(factory)
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let current_transport = Arc::new(
            CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(runtime.http_client_factory()),
                TransportHttpVersion::Http2,
            )
            .unwrap(),
        );
        let transport_holder = Arc::new(ArcSwap::from(current_transport));

        let result =
            CosmosDriver::refresh_account_properties(&runtime, &account, &transport_holder, None)
                .await;

        assert!(result.is_err(), "should fail without previous props");
    }
}
