// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB driver instance.

use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, ExecutionContext, PipelineType, RequestSentStatus,
        TransportHttpVersion, TransportSecurity,
    },
    driver::{
        cache::{PartitionKeyRangeCache, PkRangeFetchResult},
        dataflow::{
            planner,
            query_plan::{QueryPlan, RawQueryPlan},
            CachedTopologyProvider, OperationPlan, PartitionRoutingRefresh, PipelineContext,
            PipelineNodeState, RequestExecutor, RequestTarget, TopologyProvider,
        },
        pipeline::operation_pipeline::OperationOverrides,
        routing::{
            partition_key_range_id::PartitionKeyRangeId, session_manager::SessionManager,
            CosmosEndpoint, LocationStateStore,
        },
        transport::uses_dataplane_pipeline,
    },
    models::{
        effective_partition_key::EffectivePartitionKey, AccountEndpoint, AccountReference,
        ContainerProperties, ContainerReference, ContinuationToken, CosmosOperation,
        DatabaseReference, FeedRange, PartitionKey, ResolvedToken, ResourceType, UserAgent,
        UserAgentFeatureFlags,
    },
    options::{
        ConnectionPoolOptions, DriverOptions, OperationOptions, OperationOptionsView,
        ResolvedThroughputControl, ThroughputControlGroupSnapshot,
    },
    ActivityId, CosmosResponse,
};
use arc_swap::ArcSwap;
use futures::future::BoxFuture;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "preview_dtx")]
use std::time::Instant;
use url::Url;

/// Gateway 2.0 endpoint-discovery opt-in header, sent on every
/// `getDatabaseAccount` request. Aliases the canonical wire string in
/// `models::cosmos_headers`.
const GATEWAY_V2_DISCOVERY_OPT_IN: azure_core::http::headers::HeaderName =
    azure_core::http::headers::HeaderName::from_static(
        crate::models::cosmos_headers::request_header_names::USE_THINCLIENT,
    );

#[cfg(feature = "preview_dtx")]
const DTX_OUTER_MAX_RETRIES: u32 = 10;
// Matches .NET DistributedTransactionCommitter.MaxCumulativeRetryDelay (30 s).
#[cfg(feature = "preview_dtx")]
const DTX_OUTER_MAX_CUMULATIVE_DELAY: Duration = Duration::from_secs(30);
#[cfg(feature = "preview_dtx")]
const DTX_OUTER_BASE_DELAY: Duration = Duration::from_secs(1);
#[cfg(feature = "preview_dtx")]
const DTX_OUTER_MAX_EXPONENT: u32 = 5;
#[cfg(feature = "preview_dtx")]
const DTX_OUTER_JITTER_RATIO: f64 = 0.25;
const ACCOUNT_PROPERTIES_CONNECTIVITY_MAX_RETRIES: u32 = 2;
const ACCOUNT_PROPERTIES_CONNECTIVITY_BASE_DELAY: Duration = Duration::from_millis(100);

fn should_retry_account_properties_connectivity_error(
    error: &crate::error::CosmosError,
    request_sent: RequestSentStatus,
    retry_count: u32,
) -> bool {
    retry_count < ACCOUNT_PROPERTIES_CONNECTIVITY_MAX_RETRIES
        && request_sent.definitely_not_sent()
        && is_account_properties_connectivity_error(error)
}

fn is_account_properties_connectivity_error(error: &crate::error::CosmosError) -> bool {
    if error.is_from_wire() {
        return false;
    }

    matches!(
        error.status().sub_status(),
        Some(crate::models::SubStatusCode::TRANSPORT_GENERATED_503)
            | Some(crate::models::SubStatusCode::TRANSPORT_CONNECTION_FAILED)
            | Some(crate::models::SubStatusCode::TRANSPORT_IO_FAILED)
            | Some(crate::models::SubStatusCode::TRANSPORT_DNS_FAILED)
            | Some(crate::models::SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE)
            | Some(crate::models::SubStatusCode::TRANSPORT_BODY_READ_FAILED)
            | Some(crate::models::SubStatusCode::CLIENT_OPERATION_TIMEOUT)
    )
}

fn account_properties_connectivity_retry_delay(retry_count: u32) -> Duration {
    ACCOUNT_PROPERTIES_CONNECTIVITY_BASE_DELAY * retry_count
}

#[cfg(feature = "tokio")]
use super::routing::EndpointProbeFn;

use super::{
    cache::{parse_pk_ranges_response, AccountRegion},
    transport::{
        connectivity_probe::{ConnectivityProbe, Http2ConnectivityProbe},
        cosmos_headers,
        cosmos_transport_client::HttpRequest,
        http_client_factory::HttpClientConfig,
        request_signing, AuthorizationContext, CosmosTransport,
    },
    CosmosDriverRuntime,
};

struct DriverRequestExecutor<'a> {
    driver: &'a CosmosDriver,
    options: &'a OperationOptions,
}

fn request_target_overrides(
    operation_partition_key: Option<&PartitionKey>,
    target: RequestTarget,
    continuation: Option<String>,
) -> OperationOverrides {
    match target {
        RequestTarget::LogicalPartitionKey(pk) => OperationOverrides {
            partition_key: Some(pk),
            continuation,
            ..Default::default()
        },
        RequestTarget::EffectivePartitionKeyRange {
            partition_key_range_id,
            range,
            partition_key_range,
        } => OperationOverrides {
            partition_key_range_id: Some(partition_key_range_id),
            // Only emit `x-ms-start-epk`/`x-ms-end-epk` for the narrowed case
            // (range < partition_key_range). The public EPK headers paired with
            // `partitionkeyrangeid` are accepted by Gateway 2.0 but rejected by
            // the standard gateway with HTTP 400 (verified against live
            // accounts, independent of the min bound value), so we never emit
            // them on the full-pkrange XPK fan-out path that must also work on
            // the standard gateway. Because the narrowed case emits both
            // headers together, it is only valid on the wire against Gateway
            // 2.0. The GW_V2 dispatcher derives its RNTBD
            // `StartEpkHash`/`EndEpkHash` tokens from `pkrange_bounds` (below)
            // when the public headers are absent.
            feed_range: range,
            // Always carry the physical pkrange bounds so the GW_V2 dispatcher
            // can synthesize StartEpkHash/EndEpkHash tokens (which the
            // thin-client proxy requires on every Query frame). Surfaced via
            // internal `x-ms-thinclient-pkrange-min`/`-max` headers in
            // `apply_headers`; the standard gateway ignores unknown headers.
            pkrange_bounds: Some(partition_key_range),
            // Propagate the operation's logical partition key (e.g. the
            // partial-HPK prefix from `FeedScope::partition(...)`) so the
            // `x-ms-documentdb-partitionkey` HTTP header is emitted on
            // per-pkrange query fan-out requests. The thin-client proxy
            // uses this to filter docs within a pkrange by the supplied
            // prefix; without it, the
            // backend returns every document in the physical partition.
            partition_key: operation_partition_key.cloned(),
            continuation,
        },
        RequestTarget::NonPartitioned => OperationOverrides {
            continuation,
            ..Default::default()
        },
    }
}

impl RequestExecutor for DriverRequestExecutor<'_> {
    fn execute_request<'a>(
        &'a mut self,
        operation: &'a CosmosOperation,
        target: RequestTarget,
        _partition_routing_refresh: PartitionRoutingRefresh,
        continuation: Option<String>,
    ) -> BoxFuture<'a, crate::error::Result<CosmosResponse>> {
        let driver = self.driver;
        let overrides = request_target_overrides(operation.partition_key(), target, continuation);

        Box::pin(async move {
            driver
                .execute_operation_direct(operation, overrides, self.options)
                .await
        })
    }
}

/// Cosmos DB driver instance.
///
/// A driver represents a connection to a specific Cosmos DB account. It is
/// created via [`CosmosDriverRuntime::create_driver()`]; each call returns a
/// fresh instance — drivers built from the same runtime share runtime-owned
/// resources (bootstrap transport, metadata caches, CPU monitor, etc.) but the
/// driver lifetime is owned by the caller.
///
/// The driver handles executing operations against Cosmos DB, merging options
/// from operation, driver, and runtime levels.
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
    /// Cache for partition key range routing maps.
    /// Used to pre-resolve partition key range IDs for PPAF/PPCB
    /// before the first request attempt.
    pk_range_cache: PartitionKeyRangeCache,
    /// Session token cache for session consistency.
    session_manager: SessionManager,
    /// Set to `true` after [`initialize()`](Self::initialize) completes successfully.
    /// Operations check this flag to fail fast if the driver is used before
    /// initialization. In normal usage `create_driver` awaits `initialize()`
    /// before returning, so this guard only catches misuse.
    initialized: AtomicBool,
    /// User-Agent string stamped on every request issued by this driver.
    ///
    /// When the driver's [`DriverOptions::user_agent_suffix()`] is `None`, this
    /// is a clone of the runtime's `Arc<UserAgent>` (cheap atomic refcount bump;
    /// drivers without an override share one `UserAgent` allocation with the
    /// runtime). When the suffix is `Some`, this is a freshly-computed
    /// `UserAgent` wrapped in its own `Arc`.
    user_agent: Arc<UserAgent>,
    /// HTTP client factory used by every per-account transport this driver
    /// builds.
    ///
    /// When the driver has no fault injection rules configured, this is a
    /// clone of the runtime's factory `Arc`. When the driver has fault
    /// injection rules, this is a fault-injecting wrapper around the runtime's
    /// factory; the wrapper is owned by this driver alone and never affects
    /// other drivers built from the same runtime.
    http_client_factory: Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
    /// Whether this driver has fault injection rules installed.
    ///
    /// Mirrors `options.fault_injection_rules().is_some()`; cached at
    /// construction time so the data-plane hot path can stamp the diagnostics
    /// flag without re-checking the option.
    #[cfg(feature = "fault_injection")]
    fault_injection_enabled: bool,
    /// Driver-level throughput-control group registry.
    ///
    /// Populated from `DriverOptions::throughput_control_groups()` once at
    /// driver construction. The runtime no longer owns its own registry —
    /// throughput-control groups are a driver-level concern.
    throughput_control_groups: crate::options::ThroughputControlGroupRegistry,
    /// Native FFI query plan provider. Lazily loads the native library on
    /// first use; returns errors if unavailable.
    #[cfg(feature = "__internal_native_query_plan")]
    native_query_plan_provider: crate::query_plan_native::NativeQueryPlanProvider,
}

impl CosmosDriver {
    /// Returns `true` if `error` indicates an HTTP/2 incompatibility for
    /// which falling back to HTTP/1.1 is appropriate.
    ///
    /// The Cosmos boundary mapper in [`crate::error`] walks the source chain
    /// for `h2::Error` reasons such as `HTTP_1_1_REQUIRED` / `PROTOCOL_ERROR`
    /// / `FRAME_SIZE_ERROR` and mints
    /// [`SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE`] when it sees one, so
    /// pipeline-produced errors carry the sub-status directly. Raw `h2`
    /// errors that arrived through other paths are still detected via a
    /// source-chain downcast.
    #[cfg(feature = "reqwest")]
    fn has_explicit_http2_incompatibility(error: &crate::error::CosmosError) -> bool {
        if error.status().sub_status()
            == Some(crate::models::SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE)
        {
            return true;
        }
        let mut source = std::error::Error::source(error);
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
    fn has_explicit_http2_incompatibility(_error: &crate::error::CosmosError) -> bool {
        false
    }

    fn should_downgrade_http2(
        current_version: TransportHttpVersion,
        error: &crate::error::CosmosError,
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
    ) -> crate::error::Result<(
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
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        version: TransportHttpVersion,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<(super::cache::AccountProperties, CosmosTransport)> {
        let endpoint = AccountEndpoint::from(account);
        let (transport, metadata_transport) = Self::build_metadata_transport_for_version(
            runtime.connection_pool(),
            Arc::clone(http_client_factory),
            version,
            &endpoint,
        )?;
        let user_agent = Self::user_agent_header(runtime.user_agent());
        let props = Self::fetch_account_properties_with_transport(
            runtime,
            &metadata_transport,
            account,
            None,
            &user_agent,
            fault_injection_enabled,
        )
        .await?;
        Ok((props, transport))
    }

    /// Fetches account properties using the bootstrap transport, optionally with
    /// a driver-specific HTTP client factory for fault injection.
    ///
    /// This is used during initialization (before the per-account transport exists).
    /// When `override_http_client_factory` is provided (during driver initialization
    /// with fault injection), a temporary bootstrap transport is created using that
    /// factory, allowing fault injection rules to apply to the bootstrap probe.
    /// Otherwise, the runtime's shared bootstrap transport is used (no fault injection).
    async fn fetch_account_properties_with_runtime(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        override_http_client_factory: Option<
            &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        >,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let user_agent = Self::user_agent_header(runtime.user_agent());

        let metadata_transport = if let Some(factory) = override_http_client_factory {
            // Bootstrap with driver-specific fault-injecting factory:
            // Create a temporary bootstrap transport using the provided factory
            // so fault injection rules apply to this probe.
            let temp_bootstrap = CosmosTransport::bootstrap_metadata_only(
                runtime.connection_pool().clone(),
                Arc::clone(factory),
                TransportHttpVersion::Http2,
            )?;
            temp_bootstrap.get_metadata_transport(&endpoint)?
        } else {
            // Normal bootstrap: use the runtime's shared bootstrap transport
            let transport = runtime.bootstrap_transport();
            transport.get_metadata_transport(&endpoint)?
        };

        Self::fetch_account_properties_with_transport(
            runtime,
            &metadata_transport,
            account,
            None,
            &user_agent,
            fault_injection_enabled,
        )
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
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<(TransportHttpVersion, super::cache::AccountProperties)> {
        match Self::fetch_initial_account_properties_for_endpoint(
            runtime,
            http_client_factory,
            account,
            fault_injection_enabled,
        )
        .await
        {
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
                        http_client_factory,
                        &backup_account,
                        fault_injection_enabled,
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
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<(TransportHttpVersion, super::cache::AccountProperties)> {
        if !runtime.connection_pool().is_http2_allowed() {
            // User explicitly disabled HTTP/2 — skip the probe.
            let (props, _) = Self::fetch_account_properties_with_version(
                runtime,
                http_client_factory,
                account,
                TransportHttpVersion::Http11,
                fault_injection_enabled,
            )
            .await?;
            return Ok((TransportHttpVersion::Http11, props));
        }

        // Try HTTP/2-only via the bootstrap transport (which is HTTP/2-only).
        // If fault injection is enabled, pass the driver's factory so FI rules apply to the probe.
        match Self::fetch_account_properties_with_runtime(
            runtime,
            account,
            if fault_injection_enabled {
                Some(http_client_factory)
            } else {
                None
            },
            fault_injection_enabled,
        )
        .await
        {
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
                    http_client_factory,
                    account,
                    TransportHttpVersion::Http11,
                    fault_injection_enabled,
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

    /// Builds the shared per-request `DiagnosticsContextBuilder` envelope used by both
    /// the operation pipeline (`execute_operation_direct` Step 7) and the off-pipeline
    /// account-properties bootstrap fetch. Returns the builder plus the resolved
    /// `TransportSecurity` for the endpoint.
    fn new_diagnostics_envelope(
        runtime: &CosmosDriverRuntime,
        activity_id: crate::models::ActivityId,
        endpoint: &AccountEndpoint,
        fault_injection_enabled: bool,
    ) -> (DiagnosticsContextBuilder, TransportSecurity) {
        let mut diagnostics = DiagnosticsContextBuilder::new(
            activity_id,
            Arc::clone(runtime.diagnostics_options_arc()),
        );
        diagnostics.set_cpu_monitor(runtime.cpu_monitor().clone());
        diagnostics.set_machine_id(Arc::clone(runtime.machine_id()));
        #[cfg(feature = "fault_injection")]
        if fault_injection_enabled {
            diagnostics.set_fault_injection_enabled(true);
        }
        #[cfg(not(feature = "fault_injection"))]
        let _ = fault_injection_enabled;
        let transport_security = if runtime
            .connection_pool()
            .server_certificate_validation()
            .allows_insecure_connection(endpoint)
        {
            TransportSecurity::EmulatorWithInsecureCertificates
        } else {
            TransportSecurity::Secure
        };
        (diagnostics, transport_security)
    }

    /// Fetches account properties using a specific adaptive transport. Off-pipeline by
    /// design (the driver / operation pipeline does not yet exist at bootstrap, nor for
    /// the 5-minute background refresh callback) but still produces a `DiagnosticsContext`
    /// matching the data-plane shape so error consumers see the same fields.
    async fn fetch_account_properties_with_transport(
        runtime: &CosmosDriverRuntime,
        transport: &super::transport::adaptive_transport::AdaptiveTransport,
        account: &AccountReference,
        region: Option<&crate::options::Region>,
        user_agent: &azure_core::http::headers::HeaderValue,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<super::cache::AccountProperties> {
        let endpoint = AccountEndpoint::from(account);
        let endpoint_url = endpoint.join_path("/");
        // Diagnostics label tracks whether this is the global bootstrap probe (None)
        // or a regional-fallback refresh (Some). Without this, regional refresh
        // failures render with `region = None` and consumers lose the signal that
        // distinguishes "primary down" from "this specific region down".
        let cosmos_endpoint = match region {
            Some(region) => CosmosEndpoint::regional(region.clone(), endpoint_url.clone()),
            None => CosmosEndpoint::global(endpoint_url.clone()),
        };

        // Off-pipeline envelope: same shape as the operation pipeline's Step 7 so
        // err.diagnostics() exposes the same fields data-plane callers see.
        let (mut diagnostics, transport_security) = Self::new_diagnostics_envelope(
            runtime,
            crate::models::ActivityId::new_uuid(),
            &endpoint,
            fault_injection_enabled,
        );
        // NOTE: `transport.diagnostics_http_version()` reflects the *currently configured*
        // version on the adaptive transport. For the very first bootstrap call this is the
        // pre-negotiation policy (the HTTP/2 probe in `CosmosDriver::initialize` runs AFTER
        // this fetch in the `with_runtime` path used by `new`). Subsequent refreshes and
        // `_with_version` calls record the post-negotiation value. Probing first would
        // require a separate diagnostics envelope around the probe itself; we accept the
        // pre-negotiation label on bootstrap as the lower-risk tradeoff.
        let mut connectivity_retry_count = 0_u32;
        let (response, request_handle) = loop {
            let execution_context = if connectivity_retry_count == 0 {
                ExecutionContext::Initial
            } else {
                ExecutionContext::TransportRetry
            };
            let request_handle = diagnostics.start_request(
                execution_context,
                PipelineType::Metadata,
                transport_security,
                transport.diagnostics_kind(),
                transport.diagnostics_http_version(),
                &cosmos_endpoint,
            );
            for _ in 0..connectivity_retry_count {
                diagnostics.increment_local_shard_retry_count(request_handle);
            }

            // `build_account_properties_request` applies the standard cosmos headers
            // and the `x-ms-cosmos-use-thinclient: true` discovery opt-in so the
            // server emits `thinClient*Locations` when the federation supports it.
            let mut request = Self::build_account_properties_request(&endpoint, user_agent);

            // Tag the request so `FaultInjectingHttpClient` can match
            // `FaultOperationType::MetadataReadDatabaseAccount` rules against the
            // bootstrap fetch. Mirrors the data-plane tag in `operation_pipeline`.
            #[cfg(feature = "fault_injection")]
            cosmos_headers::apply_fault_injection_operation_tag(
                &mut request.headers,
                crate::fault_injection::FaultOperationType::MetadataReadDatabaseAccount,
            );

            if let Err(err) = request_signing::sign_request(
                &mut request,
                account.auth(),
                &AuthorizationContext::new(
                    azure_core::http::Method::Get,
                    ResourceType::DatabaseAccount,
                    "",
                ),
            )
            .await
            {
                // Sign failure: request never went on the wire.
                let sign_status = err.status();
                diagnostics.fail_transport_request(
                    request_handle,
                    err.to_string(),
                    RequestSentStatus::NotSent,
                    sign_status,
                );
                diagnostics
                    .set_operation_status(sign_status.status_code(), sign_status.sub_status());
                return Err(crate::error::CosmosErrorBuilder::from_error(err)
                    .with_context(format!("AccountProperties sign_request for {endpoint}"))
                    .with_diagnostics(Arc::new(diagnostics.complete()))
                    .build());
            }

            match transport.send(&request).await {
                Ok(r) => break (r, request_handle),
                Err(e) => {
                    if should_retry_account_properties_connectivity_error(
                        &e.error,
                        e.request_sent,
                        connectivity_retry_count,
                    ) {
                        connectivity_retry_count += 1;
                        let delay =
                            account_properties_connectivity_retry_delay(connectivity_retry_count);
                        tracing::debug!(
                            endpoint = %endpoint,
                            retry_count = connectivity_retry_count,
                            ?delay,
                            error = %e.error,
                            "retrying AccountProperties fetch after connectivity failure"
                        );
                        diagnostics.fail_transport_request(
                            request_handle,
                            e.error.to_string(),
                            e.request_sent,
                            e.error.status(),
                        );
                        azure_core::sleep(
                            azure_core::time::Duration::try_from(delay)
                                .unwrap_or(azure_core::time::Duration::ZERO),
                        )
                        .await;
                        continue;
                    }

                    let send_status = e.error.status();
                    diagnostics.fail_transport_request(
                        request_handle,
                        e.error.to_string(),
                        e.request_sent,
                        send_status,
                    );
                    diagnostics
                        .set_operation_status(send_status.status_code(), send_status.sub_status());
                    return Err(crate::error::CosmosErrorBuilder::from_error(e.error)
                        .with_context(format!("AccountProperties fetch from {endpoint}"))
                        .with_diagnostics(Arc::new(diagnostics.complete()))
                        .build());
                }
            }
        };
        let cosmos_headers = crate::models::CosmosResponseHeaders::from_headers(&response.headers);
        let status_code = azure_core::http::StatusCode::from(response.status);
        let sub_status = cosmos_headers.substatus;
        let cosmos_status = crate::error::CosmosStatus::from_parts(status_code, sub_status);

        diagnostics.record_response(request_handle, status_code, &cosmos_headers);

        // Gate parsing on HTTP status. Non-2xx bodies (5xx envelopes, AAD 401/403, proxy text)
        // would otherwise serde-fail and surface as `SERIALIZATION_RESPONSE_BODY_INVALID`.
        // 3xx is treated as non-success here as a safety net: the production reqwest client
        // built by `DefaultHttpClientFactory` keeps reqwest's default `Policy::limited(10)` and
        // transparently follows 3xx redirects on the wire (see
        // `bootstrap_transport_follows_3xx_redirects_against_real_server` for the end-to-end
        // proof). A 3xx that still reaches this branch therefore means a redirect
        // the transport could not follow — hop-limit exhausted, missing/relative
        // Location, scheme downgrade blocked by reqwest, etc. — and we surface it
        // as `CosmosError` with the upstream status preserved rather than letting
        // the redirect body parse-fail.
        if !status_code.is_success() {
            diagnostics.set_operation_status(status_code, sub_status);
            let diagnostics_arc = Arc::new(diagnostics.complete());
            return Err(crate::error::CosmosError::builder()
                .with_status(cosmos_status)
                .with_response_parts(crate::models::CosmosResponsePayload::new(
                    response.body,
                    cosmos_headers,
                ))
                .with_diagnostics(diagnostics_arc)
                .with_message(format!(
                    "AccountProperties fetch from {endpoint} returned HTTP {status_code}"
                ))
                .build());
        }

        let props = match Self::parse_account_properties_payload(&response.body) {
            Ok(props) => props,
            Err(err) => {
                // Operation-status reflects the synthetic serialization failure, not
                // the wire 2xx — keeps diagnostics consistent with the data-plane
                // pipeline, where parse failures rebrand operation status.
                let parse_status = err.status();
                diagnostics
                    .set_operation_status(parse_status.status_code(), parse_status.sub_status());
                let diagnostics_arc = Arc::new(diagnostics.complete());
                return Err(crate::error::CosmosErrorBuilder::from_error(err)
                    .with_response_parts(crate::models::CosmosResponsePayload::new(
                        crate::models::ResponseBody::NoPayload,
                        cosmos_headers,
                    ))
                    .with_diagnostics(diagnostics_arc)
                    .with_context(format!("AccountProperties payload from {endpoint}"))
                    .build());
            }
        };
        tracing::info!(
            endpoint = %endpoint,
            write_region = ?props.write_region(),
            readable_locations = props.readable_locations.len(),
            writable_locations = props.writable_locations.len(),
            thin_client_readable_locations = props.thin_client_readable_locations.len(),
            thin_client_writable_locations = props.thin_client_writable_locations.len(),
            thin_client_readable_regions = ?props.gateway_v2_readable_regions(),
            thin_client_writable_regions = ?props.gateway_v2_writable_regions(),
            "AccountProperties retrieved successfully"
        );
        Ok(props)
    }

    /// Builds the unsigned `getDatabaseAccount` HTTP request.
    ///
    /// Always sets `x-ms-cosmos-use-thinclient: true` so the server emits
    /// `thinClient*Locations` whenever the federation has thin-client
    /// enabled. Without this header the server suppresses those fields and
    /// Gateway 2.0 is silently disabled.
    fn build_account_properties_request(
        endpoint: &AccountEndpoint,
        user_agent: &azure_core::http::headers::HeaderValue,
    ) -> HttpRequest {
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
        request.headers.insert(
            GATEWAY_V2_DISCOVERY_OPT_IN,
            azure_core::http::headers::HeaderValue::from_static("true"),
        );
        request
    }

    fn parse_account_properties_payload(
        payload: &[u8],
    ) -> crate::error::Result<super::cache::AccountProperties> {
        serde_json::from_slice(payload).map_err(|e| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to parse AccountProperties")
                .with_source(e)
                .build()
        })
    }

    fn user_agent_header(user_agent: &UserAgent) -> azure_core::http::headers::HeaderValue {
        azure_core::http::headers::HeaderValue::from(user_agent.as_str().to_owned())
    }

    fn endpoint_for_write_region(
        account: &AccountReference,
        write_region: Option<&AccountRegion>,
    ) -> AccountEndpoint {
        if let Some(region) = write_region {
            return region.database_account_endpoint.clone();
        }

        // Fall back to the account-level endpoint when there is no writable location.
        AccountEndpoint::from(account)
    }

    async fn fetch_account_properties(
        &self,
        account: &AccountReference,
    ) -> crate::error::Result<super::cache::AccountProperties> {
        let fault_injection_enabled = {
            #[cfg(feature = "fault_injection")]
            {
                self.fault_injection_enabled
            }
            #[cfg(not(feature = "fault_injection"))]
            {
                false
            }
        };
        Self::refresh_account_properties(
            &self.runtime,
            &self.http_client_factory,
            account,
            &self.transport,
            &self.user_agent,
            None,
            fault_injection_enabled,
        )
        .await
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
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        user_agent: &Arc<UserAgent>,
        previous_props: Option<Arc<super::cache::AccountProperties>>,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<super::cache::AccountProperties> {
        let current_transport = transport_holder.load_full();
        let current_version = current_transport.negotiated_version();
        let endpoint = AccountEndpoint::from(account);
        let metadata_transport = current_transport.get_metadata_transport(&endpoint)?;

        let user_agent_header = Self::user_agent_header(user_agent);
        match Self::fetch_account_properties_with_transport(
            runtime,
            &metadata_transport,
            account,
            None,
            &user_agent_header,
            fault_injection_enabled,
        )
        .await
        {
            Ok(props) => {
                Self::maybe_restore_http2_after_refresh(
                    runtime,
                    http_client_factory,
                    account,
                    transport_holder,
                    current_version,
                    &endpoint,
                    fault_injection_enabled,
                )
                .await;
                Ok(props)
            }
            Err(error) => {
                match Self::handle_refresh_failure(
                    runtime,
                    http_client_factory,
                    account,
                    transport_holder,
                    current_version,
                    &endpoint,
                    error,
                    fault_injection_enabled,
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
                            user_agent,
                            &endpoint,
                            primary_error,
                            previous_props,
                            fault_injection_enabled,
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
    #[allow(clippy::too_many_arguments)]
    async fn refresh_via_regional_endpoints(
        runtime: &CosmosDriverRuntime,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        user_agent: &Arc<UserAgent>,
        primary_endpoint: &AccountEndpoint,
        primary_error: crate::error::CosmosError,
        previous_props: Option<Arc<super::cache::AccountProperties>>,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<super::cache::AccountProperties> {
        let Some(cached_props) = previous_props else {
            return Err(primary_error);
        };

        // Parse regional URLs once, filtering out the primary and any invalid URLs.
        let regional_endpoints: Vec<(crate::options::Region, Url)> = cached_props
            .readable_locations
            .iter()
            .filter_map(|loc| {
                let url = loc.database_account_endpoint.url().clone();
                let ep = AccountEndpoint::from(url.clone());
                if ep == *primary_endpoint {
                    None
                } else {
                    Some((loc.name.clone(), url))
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

        for (region, regional_url) in &regional_endpoints {
            let regional_account = Self::with_endpoint(account, regional_url.clone());
            let regional_ep = AccountEndpoint::from(&regional_account);
            let current_transport = transport_holder.load_full();
            let Ok(regional_transport) = current_transport.get_metadata_transport(&regional_ep)
            else {
                continue;
            };

            let user_agent = Self::user_agent_header(user_agent);
            match Self::fetch_account_properties_with_transport(
                runtime,
                &regional_transport,
                &regional_account,
                Some(region),
                &user_agent,
                fault_injection_enabled,
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
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        current_version: TransportHttpVersion,
        endpoint: &AccountEndpoint,
        fault_injection_enabled: bool,
    ) {
        if !matches!(current_version, TransportHttpVersion::Http11)
            || !runtime.connection_pool().is_http2_allowed()
        {
            return;
        }

        // Reprobe HTTP/2 using the fault-injecting factory if enabled
        match Self::fetch_account_properties_with_runtime(
            runtime,
            account,
            if fault_injection_enabled {
                Some(http_client_factory)
            } else {
                None
            },
            fault_injection_enabled,
        )
        .await
        {
            Ok(_) => match CosmosTransport::with_factory(
                runtime.connection_pool().clone(),
                Arc::clone(http_client_factory),
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
    #[allow(clippy::too_many_arguments)]
    async fn handle_refresh_failure(
        runtime: &CosmosDriverRuntime,
        http_client_factory: &Arc<dyn super::transport::http_client_factory::HttpClientFactory>,
        account: &AccountReference,
        transport_holder: &Arc<ArcSwap<CosmosTransport>>,
        current_version: TransportHttpVersion,
        endpoint: &AccountEndpoint,
        error: crate::error::CosmosError,
        fault_injection_enabled: bool,
    ) -> crate::error::Result<super::cache::AccountProperties> {
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

            let (props, fallback_transport) = Self::fetch_account_properties_with_version(
                runtime,
                http_client_factory,
                account,
                fallback_version,
                fault_injection_enabled,
            )
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
    ) -> crate::error::Result<ContainerReference> {
        let db_ref = DatabaseReference::from_name(self.account().clone(), db_name.to_owned());
        let options = OperationOptions::default();

        let container_result = self
            .execute_singleton_operation(
                CosmosOperation::read_container_by_name(db_ref, container_name.to_owned()),
                options,
            )
            .await?;
        let container_headers = container_result.headers().clone();
        let container_diagnostics = container_result.diagnostics();
        let container_props: ContainerProperties =
            container_result.into_body().into_single().map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to deserialize container response")
                    .with_response_parts(crate::models::CosmosResponsePayload::new(
                        crate::models::ResponseBody::NoPayload,
                        container_headers.clone(),
                    ))
                    .with_diagnostics(container_diagnostics.clone())
                    .with_source(e)
                    .build()
            })?;
        let container_rid = container_props
            .system_properties
            .rid
            .clone()
            .ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("container response missing _rid")
                    .with_response_parts(crate::models::CosmosResponsePayload::new(
                        crate::models::ResponseBody::NoPayload,
                        container_headers.clone(),
                    ))
                    .with_diagnostics(container_diagnostics.clone())
                    .with_source(std::io::Error::other("missing _rid"))
                    .build()
            })?;

        // Derive the database RID from the container RID's encoded byte
        // layout. This avoids an extra `read_database` round-trip — the
        // first 4 decoded bytes of the container RID are the parent database RID.
        let db_rid = crate::models::resource_id::ResourceId::new(container_rid.clone())
            .database_rid()
            .ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!(
                        "failed to extract database RID from container RID '{container_rid}'"
                    ))
                    .with_response_parts(crate::models::CosmosResponsePayload::new(
                        crate::models::ResponseBody::NoPayload,
                        container_headers,
                    ))
                    .with_diagnostics(container_diagnostics)
                    .with_source(std::io::Error::other("invalid container _rid"))
                    .build()
            })?;

        Ok(ContainerReference::new(
            self.account().clone(),
            db_name.to_owned(),
            db_rid.as_str().to_owned(),
            container_props.id.clone().into_owned(),
            container_rid,
            &container_props,
        ))
    }

    /// Creates a new driver instance.
    ///
    /// This is internal - use [`CosmosDriverRuntime::create_driver()`] instead.
    pub(crate) fn new(
        runtime: Arc<CosmosDriverRuntime>,
        options: DriverOptions,
    ) -> crate::error::Result<Self> {
        let account = options.account().clone();
        let account_endpoint = AccountEndpoint::from(&account);
        let default_endpoint = CosmosEndpoint::global(account.endpoint().clone());

        // Per-driver User-Agent: compute the cross-SDK feature flags advertised
        // in the header from this driver's effective client configuration —
        // HTTP/2 (runtime connection pool) and PPCB (this driver's partition
        // failover options). When the driver-level suffix override is unset and
        // the flags match the runtime's base flags (the common case), share the
        // runtime's `Arc<UserAgent>` (cheap atomic refcount bump). Otherwise
        // compute a fresh `UserAgent` owned by this driver alone.
        let feature_flags = UserAgentFeatureFlags::from_client_config(
            runtime.connection_pool().is_http2_allowed(),
            options
                .partition_failover_options()
                .circuit_breaker_enabled(),
        );
        let user_agent = match options.user_agent_suffix() {
            Some(suffix) => Arc::new(UserAgent::from_suffix(
                runtime.wrapping_sdk_identifier(),
                suffix,
                feature_flags,
            )),
            None if feature_flags == runtime.user_agent_feature_flags() => {
                Arc::clone(runtime.user_agent())
            }
            None => Arc::new(runtime.user_agent_with_feature_flags(feature_flags)),
        };

        // Per-driver HTTP client factory: wrap with fault injection if rules
        // are installed on this driver's options; otherwise share the
        // runtime's factory `Arc`. Bootstrap transport (built at runtime
        // construction time) is never wrapped, so rules targeting the
        // initial account-metadata probe only take effect on post-bootstrap
        // refreshes — matching the previous runtime-level FI semantics.
        #[cfg(feature = "fault_injection")]
        let fault_injection_enabled = options.fault_injection_rules().is_some();
        let http_client_factory: Arc<dyn super::transport::http_client_factory::HttpClientFactory> = {
            #[cfg(feature = "fault_injection")]
            {
                if let Some(rules) = options.fault_injection_rules() {
                    Arc::new(
                        crate::fault_injection::FaultInjectingHttpClientFactory::new(
                            Arc::clone(runtime.http_client_factory()),
                            rules.to_vec(),
                        ),
                    )
                } else {
                    Arc::clone(runtime.http_client_factory())
                }
            }
            #[cfg(not(feature = "fault_injection"))]
            {
                Arc::clone(runtime.http_client_factory())
            }
        };

        // Shared transport holder — used by both the driver and the refresh callback.
        // ArcSwap provides lock-free reads on the hot path (every operation)
        // and only incurs overhead on writes (transport swap, ~every 5 min).
        let transport: Arc<ArcSwap<CosmosTransport>> =
            Arc::new(ArcSwap::from(Arc::clone(runtime.bootstrap_transport())));

        let runtime_for_callback = Arc::clone(&runtime);
        let account_for_callback = account.clone();
        let transport_for_callback = Arc::clone(&transport);
        let user_agent_for_callback = Arc::clone(&user_agent);
        let factory_for_callback = Arc::clone(&http_client_factory);
        #[cfg(feature = "fault_injection")]
        let fault_injection_for_callback = fault_injection_enabled;
        #[cfg(not(feature = "fault_injection"))]
        let fault_injection_for_callback = false;
        let refresh_callback = Arc::new(
            move |previous_props: Option<Arc<super::cache::AccountProperties>>| {
                let runtime = Arc::clone(&runtime_for_callback);
                let account = account_for_callback.clone();
                let transport_holder = Arc::clone(&transport_for_callback);
                let user_agent = Arc::clone(&user_agent_for_callback);
                let factory = Arc::clone(&factory_for_callback);
                let fault_injection_enabled = fault_injection_for_callback;
                let fut: BoxFuture<'static, crate::error::Result<super::cache::AccountProperties>> =
                    Box::pin(async move {
                        CosmosDriver::refresh_account_properties(
                            &runtime,
                            &factory,
                            &account,
                            &transport_holder,
                            &user_agent,
                            previous_props,
                            fault_injection_enabled,
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
            .or(runtime
                .default_operation_options()
                .endpoint_unavailability_ttl)
            .unwrap_or_else(|| {
                std::env::var("AZURE_COSMOS_ENDPOINT_UNAVAILABLE_TTL_MS")
                    .ok()
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(Duration::from_millis)
                    .unwrap_or(Duration::from_secs(60))
            });

        // Wire the Gateway 2.0 connectivity probe. Before routing data-plane
        // traffic to a thin-client proxy endpoint, the store issues a
        // `POST /connectivity-probe` over HTTP/2 and gates Gateway 2.0 off for
        // all regions unless every probe returns 200. The probe shares the
        // data plane's Gateway 2.0 HTTP/2 config so it negotiates the same
        // protocol the real traffic uses. Skip building it entirely when
        // HTTP/2 is unavailable (the one hard Gateway 2.0 prerequisite);
        // otherwise the store still no-ops the probe when the account
        // advertises no thin-client endpoints.
        let connectivity_probe: Option<Arc<dyn ConnectivityProbe>> =
            if runtime.connection_pool().gateway_v2_disabled() {
                None
            } else {
                let probe_config =
                    HttpClientConfig::dataplane_gateway_v2(runtime.connection_pool());
                let probe_client =
                    http_client_factory.build(runtime.connection_pool(), probe_config)?;
                Some(Arc::new(Http2ConnectivityProbe::new(probe_client)))
            };

        let location_state_store = Arc::new(LocationStateStore::new(
            runtime.account_metadata_cache().clone(),
            account_endpoint,
            default_endpoint,
            refresh_callback,
            !runtime.connection_pool().gateway_v2_disabled(),
            endpoint_unavailability_ttl,
            options.partition_failover_options().clone(),
            options.preferred_regions().to_vec(),
            connectivity_probe,
        ));

        // Spawn the background failback loop for partition-level overrides.
        #[cfg(feature = "tokio")]
        location_state_store.start_failback_loop();

        // Spawn the background account-metadata refresh loop so long-running
        // workloads see periodic re-fetch of the database account properties
        // without paying the latency on the request hot path. Per-operation
        // lookups in `execute_operation` use the cheap `get_or_fetch` fast
        // path because freshness is owned by this loop.
        #[cfg(feature = "tokio")]
        location_state_store.start_account_refresh_loop();

        // Spawn the background endpoint-probe loop. This makes account-level
        // endpoint failback probe-gated: an endpoint marked unavailable (e.g.
        // firewall-blocked) only rejoins the routing rotation after a
        // lightweight connectivity probe (a `GET /probe` request to that
        // specific endpoint) confirms it is reachable. Without this, the
        // endpoint would be failed back purely on cooldown expiry, have real
        // traffic routed to it, time out, and be re-marked unavailable — a
        // sustained low-throughput loop (issue #4597).
        #[cfg(feature = "tokio")]
        {
            let account_for_probe = account.clone();
            let transport_for_probe = Arc::clone(&transport);
            let user_agent_for_probe = Arc::clone(&user_agent);
            let probe_fn: EndpointProbeFn = Arc::new(move |url: Url| {
                let account = account_for_probe.clone();
                let transport_holder = Arc::clone(&transport_for_probe);
                let user_agent = Arc::clone(&user_agent_for_probe);
                Box::pin(async move {
                    let probe_account = CosmosDriver::with_endpoint(&account, url);
                    let endpoint = AccountEndpoint::from(&probe_account);
                    let transport = transport_holder.load_full();
                    let Ok(metadata_transport) = transport.get_metadata_transport(&endpoint) else {
                        return false;
                    };
                    let user_agent = CosmosDriver::user_agent_header(&user_agent);
                    probe_endpoint_connectivity(&metadata_transport, &probe_account, &user_agent)
                        .await
                }) as BoxFuture<'static, bool>
            });
            location_state_store.start_endpoint_probe_loop(probe_fn);
        }

        // Driver-level throughput-control registry.
        //
        // The runtime no longer owns one — TCGs are a driver-level concern.
        // Clone the per-driver registry as-is for the request hot path.
        let throughput_control_groups = options.throughput_control_groups().clone();

        Ok(Self {
            runtime,
            options,
            transport,
            location_state_store,
            pk_range_cache: PartitionKeyRangeCache::new(),
            session_manager: SessionManager::new(),
            initialized: AtomicBool::new(false),
            user_agent,
            http_client_factory,
            #[cfg(feature = "fault_injection")]
            fault_injection_enabled,
            throughput_control_groups,
            #[cfg(feature = "__internal_native_query_plan")]
            native_query_plan_provider: crate::query_plan_native::NativeQueryPlanProvider::new(),
        })
    }

    /// Returns the account reference.
    pub fn account(&self) -> &AccountReference {
        self.options.account()
    }

    /// **Internal test hook -- not part of the public API.**
    ///
    /// Returns cached writable and readable account regions. The in-memory
    /// emulator comparison tests use this to pin a live multi-region account
    /// to one hub region via default `ExcludedRegions`. It does not fetch
    /// account metadata; callers should use it after the driver has been
    /// initialized.
    ///
    /// **Do not call from production code.** Available only because
    /// integration tests live outside the crate and cannot reach the account
    /// metadata cache directly. May be changed or removed at any time without
    /// a semver bump.
    #[cfg(any(test, feature = "__internal_in_memory_emulator"))]
    #[doc(hidden)]
    pub async fn cached_account_regions_for_testing(
        &self,
    ) -> Option<(Vec<crate::options::Region>, Vec<crate::options::Region>)> {
        let endpoint = AccountEndpoint::from(self.options.account());
        let props = self.runtime.account_metadata_cache().get(&endpoint).await?;
        let writable = props
            .writable_locations
            .iter()
            .map(|location| location.name.clone())
            .collect();
        let readable = props
            .readable_locations
            .iter()
            .map(|location| location.name.clone())
            .collect();
        Some((writable, readable))
    }

    /// Returns the runtime.
    pub fn runtime(&self) -> &CosmosDriverRuntime {
        &self.runtime
    }

    /// Returns the driver options.
    pub fn options(&self) -> &DriverOptions {
        &self.options
    }

    /// Returns the User-Agent stamped on every request this driver issues.
    ///
    /// When the driver was constructed with no
    /// [`DriverOptions::user_agent_suffix()`] override, this returns a clone of
    /// the runtime's `Arc<UserAgent>` — drivers built from the same runtime
    /// without an override share one `UserAgent` allocation. Otherwise this
    /// returns the driver's freshly-computed `UserAgent`.
    pub fn user_agent(&self) -> &Arc<UserAgent> {
        &self.user_agent
    }

    /// Returns the per-driver HTTP client factory.
    ///
    /// Used by data-plane and refresh paths to build per-account transports.
    /// Equivalent to the runtime's factory when this driver has no fault
    /// injection rules; otherwise wraps the runtime's factory with a
    /// fault-injecting factory carrying this driver's rules.
    #[allow(dead_code)]
    pub(crate) fn http_client_factory(
        &self,
    ) -> &Arc<dyn super::transport::http_client_factory::HttpClientFactory> {
        &self.http_client_factory
    }

    /// Returns whether fault injection is enabled for this driver.
    #[cfg(feature = "fault_injection")]
    #[allow(dead_code)]
    pub(crate) fn fault_injection_enabled(&self) -> bool {
        self.fault_injection_enabled
    }

    /// **Internal test hook -- not part of the public API.**
    ///
    /// Returns the current value of the partition-level PPAF flag
    /// (`per_partition_automatic_failover_enabled`) from the live
    /// `PartitionEndpointState`. Used by integration tests to verify that
    /// the driver's background account-refresh loop picks up dynamic
    /// changes to `AccountProperties.enable_per_partition_failover_behavior`.
    ///
    /// **Do not call from production code.** Available only because
    /// integration tests live outside the crate and cannot reach the
    /// `pub(crate)` `LocationStateStore::snapshot` API directly. May be
    /// changed or removed at any time without a semver bump.
    #[cfg(any(test, feature = "__internal_in_memory_emulator"))]
    #[doc(hidden)]
    pub fn is_per_partition_automatic_failover_enabled_for_testing(&self) -> bool {
        self.location_state_store
            .snapshot()
            .partitions
            .per_partition_automatic_failover_enabled
    }

    /// Returns the current per-account transport.
    ///
    /// Lock-free via ArcSwap::load_full() — returns a cloned Arc with no
    /// reader-counter contention between concurrent callers.
    fn transport(&self) -> Arc<CosmosTransport> {
        self.transport.load_full()
    }

    /// Test-only API. Returns a snapshot of the per-partition hub-region cache as
    /// `(pk_range_id, current_endpoint_url)` pairs.
    ///
    /// Reads `PartitionEndpointState::failover_overrides`, which on a
    /// PPAF-enabled single-master account doubles as the hub-region cache
    /// for reads (populated by the `hub_region_cache_populate_target`
    /// pipeline hook when a hub-region-only retry succeeds).
    #[cfg(any(test, feature = "__internal_testing"))]
    pub fn __test_only_hub_region_cache_snapshot(&self) -> Vec<(String, String)> {
        let snapshot = self.location_state_store.snapshot();
        snapshot
            .partitions
            .failover_overrides
            .iter()
            .map(|(pk_range_id, entry)| {
                (
                    pk_range_id.as_str().to_owned(),
                    entry.current_endpoint.url().to_string(),
                )
            })
            .collect()
    }

    /// Test-only API. Forces `per_partition_automatic_failover_enabled = true`
    /// on the driver's partition state.
    #[cfg(any(test, feature = "__internal_testing"))]
    pub fn __test_only_force_ppaf_enabled(&self) {
        self.location_state_store.apply_partition(|current| {
            let mut next = current.clone();
            next.per_partition_automatic_failover_enabled = true;
            next
        });
    }

    /// Eagerly primes the account metadata cache and creates the per-account transport.
    ///
    /// Performs an HTTP/2 probe to detect protocol support, then creates the
    /// appropriate transport (sharded HTTP/2 or unsharded HTTP/1.1). Also caches
    /// the account properties for regional endpoint resolution.
    ///
    /// This method is called automatically by
    /// [`CosmosDriverRuntime::create_driver`](crate::CosmosDriverRuntime::create_driver).
    /// Callers may invoke it again to retry if the initial attempt failed
    /// (the result is idempotent).
    pub async fn initialize(&self) -> crate::error::Result<()> {
        let account = self.options.account();
        let account_endpoint = AccountEndpoint::from(account);

        // Probe HTTP version and fetch account properties in one step.
        let fault_injection_enabled = {
            #[cfg(feature = "fault_injection")]
            {
                self.fault_injection_enabled
            }
            #[cfg(not(feature = "fault_injection"))]
            {
                false
            }
        };
        let (negotiated_version, properties) = Self::fetch_initial_account_properties(
            &self.runtime,
            &self.http_client_factory,
            account,
            fault_injection_enabled,
        )
        .await?;

        tracing::info!(
            endpoint = %account_endpoint,
            version = ?negotiated_version,
            "HTTP version negotiated for account"
        );

        // Cache the properties.
        let cached_properties = self
            .runtime
            .account_metadata_cache()
            .get_or_fetch(account_endpoint, || async { Ok(properties) })
            .await?;

        // Seed the routing snapshot with the initial account properties so
        // server-controlled flags (PPAF/PPCB) and writable-region selection
        // are correct before the first operation runs. The probe runs first so
        // the first operation routes against a probe-verified Gateway 2.0
        // snapshot rather than the optimistic snapshot derived from
        // `thinClient*Locations` alone.
        self.location_state_store
            .sync_account_properties_with_probe(
                cached_properties,
                self.location_state_store.default_endpoint(),
            )
            .await;

        // Create the per-account transport with the negotiated version.
        let new_transport = Arc::new(CosmosTransport::with_factory(
            self.runtime.connection_pool().clone(),
            Arc::clone(&self.http_client_factory),
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
    ) -> crate::error::Result<()> {
        self.resolve_container_by_name(db_name, container_name)
            .await?;
        Ok(())
    }

    /// Constructs an [`OperationOptionsView`] for resolving options across all layers.
    ///
    /// The view resolves options in priority order (highest first):
    /// 1. Environment `{ENV}_OVERRIDE` kill switches - fleet-wide incident override
    /// 2. `OperationOptions` - operation-specific overrides
    /// 3. `DriverOptions` - driver-level defaults
    /// 4. `CosmosDriverRuntime` - global runtime defaults
    /// 5. Environment - env vars read at startup
    pub fn operation_options_view<'a>(
        &self,
        operation_options: &'a OperationOptions,
    ) -> OperationOptionsView<'a> {
        OperationOptionsView::new_with_override(
            Some(Arc::clone(self.runtime.env_override_operation_options())),
            Some(Arc::clone(self.runtime.env_operation_options())),
            Some(self.runtime.default_operation_options()),
            Some(self.options.operation_options().clone()),
            Some(operation_options),
        )
    }

    /// Computes the effective throughput-control header values for an operation.
    ///
    /// Resolves the per-request `x-ms-cosmos-throughput-bucket` and
    /// `x-ms-cosmos-priority-level` headers using the public layering
    /// contract:
    ///
    /// 1. If the layered
    ///    [`ThroughputControlOptions`](crate::options::ThroughputControlOptions)
    ///    sets the field directly, use it.
    /// 2. Else, if [`group_name`](crate::options::ThroughputControlOptions::group_name)
    ///    resolves to a group registered on this driver via
    ///    [`DriverOptionsBuilder::register_throughput_control_group`](crate::options::DriverOptionsBuilder::register_throughput_control_group),
    ///    use the group's value for the field.
    /// 3. Else, omit the header.
    ///
    /// The two fields resolve independently — a layered
    /// `throughput_bucket = Some(...)` does not suppress a `priority_level`
    /// carried by the registered group, and vice versa.
    ///
    /// # Errors
    ///
    /// Returns an error if [`group_name`](crate::options::ThroughputControlOptions::group_name)
    /// is set to a name that does not resolve to a registered group for the
    /// operation's container.
    pub(crate) fn effective_throughput_control(
        &self,
        effective_options: &OperationOptionsView<'_>,
        container: &ContainerReference,
    ) -> crate::error::Result<ResolvedThroughputControl> {
        let throughput_view = effective_options.throughput_control();
        let mut bucket = throughput_view.throughput_bucket().copied();
        let mut priority = throughput_view.priority_level().copied();

        if bucket.is_some() && priority.is_some() {
            return Ok(ResolvedThroughputControl {
                throughput_bucket: bucket,
                priority_level: priority,
            });
        }

        if let Some(name) = throughput_view.group_name() {
            let group = self
                .throughput_control_groups
                .get_by_container_and_name(container, name)
                .ok_or_else(|| {
                    crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_THROUGHPUT_CONTROL_GROUP_NOT_REGISTERED)
                        .with_message(format!(
                            "throughput control group '{}' not found in registry for container '{}'",
                            name,
                            container.name()
                        ))
                        .build()
                })?;
            let snapshot = ThroughputControlGroupSnapshot::from(group.as_ref());
            if bucket.is_none() {
                bucket = snapshot.throughput_bucket();
            }
            if priority.is_none() {
                priority = snapshot.priority_level();
            }
        }

        Ok(ResolvedThroughputControl {
            throughput_bucket: bucket,
            priority_level: priority,
        })
    }

    /// Fetches partition key ranges from the service for the given container.
    ///
    /// Builds a GET request to `/dbs/{db_rid}/colls/{container_rid}/pkranges`
    /// using the `A-IM: Incremental feed` header for changefeed semantics.
    /// When `continuation` is provided, it is sent as the `If-None-Match` header
    /// for incremental fetches. The server may return 304 Not Modified if no
    /// new ranges exist since the last fetch.
    ///
    /// # Retry Policy
    ///
    /// The request is dispatched through the standard `execute_operation`
    /// pipeline, which performs in-flight cross-region failover on transient
    /// errors (503, 410, 408, 429, 403/3) by routing successive retries to
    /// the next preferred read region. A single call therefore traverses
    /// every preferred region before giving up — no additional outer retry
    /// loop is needed here.
    ///
    /// Permanent errors (401 Unauthorized, 403 Forbidden, 404 NotFound) are
    /// terminal: `None` is returned immediately so the caller can surface a
    /// clear misconfiguration signal.
    ///
    /// Returns `None` if the pipeline exhausts its cross-region failover
    /// budget or the response cannot be parsed. The caller (the PK range
    /// cache) falls back gracefully on `None`.
    async fn fetch_pk_ranges_from_service(
        &self,
        container: ContainerReference,
        continuation: Option<String>,
    ) -> Option<PkRangeFetchResult> {
        // Build the operation through the standard pipeline to get correct
        // URL construction, signing, and cross-region retry behavior.
        let mut operation = CosmosOperation::read_all_partition_key_ranges(container.clone());

        // Set changefeed If-None-Match precondition for continuation.
        if let Some(token) = continuation.as_deref() {
            operation = operation
                .with_precondition(crate::models::Precondition::if_none_match(token.to_owned()));
        }

        // Typed changefeed headers (`a-im: Incremental feed`, server-decides page size).
        let mut request_headers = operation.request_headers().clone();
        request_headers.incremental_feed = true;
        request_headers.max_item_count = Some(crate::models::MaxItemCountHint::ServerDecides);
        operation = operation.with_request_headers(request_headers);

        let options = OperationOptions::default();

        match self
            .execute_operation_direct(&operation, OperationOverrides::default(), &options)
            .await
        {
            Ok(response) => {
                let etag = response.headers().etag.as_ref().map(|e| e.to_string());

                // 304 Not Modified is a success outcome for conditional
                // changefeed reads: the cached routing map is still current.
                if response.status().status_code() == azure_core::http::StatusCode::NotModified {
                    return Some(PkRangeFetchResult {
                        ranges: vec![],
                        continuation,
                        not_modified: true,
                    });
                }

                let body_bytes = match response.into_body().single() {
                    Ok(b) => b,
                    Err(_) => {
                        tracing::error!(
                            container = %container.name(),
                            "Partition key ranges response was a feed body, expected single payload"
                        );
                        return None;
                    }
                };
                match parse_pk_ranges_response(&body_bytes) {
                    Some(ranges) => Some(PkRangeFetchResult {
                        ranges,
                        continuation: etag,
                        not_modified: false,
                    }),
                    None => {
                        tracing::error!(
                            container = %container.name(),
                            "Failed to parse partition key ranges response body"
                        );
                        None
                    }
                }
            }
            Err(e) => {
                // The error is already a typed Cosmos error; just consult
                // its status when classifying terminal vs. transient.
                let http_status = if e.is_from_wire() {
                    Some(e.status().status_code())
                } else {
                    None
                };
                if let Some(status) = http_status {
                    // Permanent errors (auth/config issues) are logged at error
                    // level so operators can distinguish misconfiguration from
                    // transient blips.
                    // TODO: Consider adding a negative-cache TTL to suppress
                    // repeated fetches on permanent errors (401/403/404).
                    if matches!(
                        status,
                        azure_core::http::StatusCode::Unauthorized
                            | azure_core::http::StatusCode::Forbidden
                            | azure_core::http::StatusCode::NotFound
                    ) {
                        tracing::error!(
                            container = %container.name(),
                            status = %status,
                            error = %e,
                            "Permanent error fetching partition key ranges — check account credentials and container existence"
                        );
                        return None;
                    }
                }

                tracing::warn!(
                    container = %container.name(),
                    error = %e,
                    "Transient error fetching partition key ranges from service after exhausting pipeline cross-region retries"
                );
                None
            }
        }
    }

    /// Pre-resolves the partition key range ID for a data plane operation.
    ///
    /// When PPAF/PPCB is enabled, seeds the partition key range ID before the
    /// first attempt so partition-level failover overrides can take effect from
    /// the very first request instead of only after a retry captures the ID
    /// from response headers.
    ///
    /// Resolution is **`OperationOverrides`-aware**. The dataflow pipeline
    /// fans a query out into per-physical-partition sub-operations and stamps
    /// the owning `partition_key_range_id` (plus the narrowed feed range and/or
    /// partition key) onto [`OperationOverrides`] rather than mutating the
    /// shared [`CosmosOperation`]. The overrides therefore carry the most
    /// specific routing information and are consulted first:
    ///
    /// 1. If the overrides already carry a `partition_key_range_id` (the common
    ///    case for dataflow-planned queries), use it directly — no cache lookup
    ///    and no risk of a multi-range collapse.
    /// 2. Otherwise resolve a logical partition key (from the overrides, then
    ///    the operation) through the point-lookup path.
    /// 3. Otherwise resolve an EPK-range feed range (from the overrides, then
    ///    the operation), seeding only when it maps to exactly one physical
    ///    partition.
    ///
    /// Returns `None` if:
    /// - PPAF/PPCB is disabled **and** the client cannot route over Gateway 2.0
    ///   (an authoritative stamped range id on the overrides is still honored)
    /// - The operation does not target a partitioned resource
    /// - No container reference or routing target is available
    /// - The cache lookup or fetch fails
    async fn pre_resolve_partition_key_range_id(
        &self,
        operation: &CosmosOperation,
        overrides: &OperationOverrides,
    ) -> Option<PartitionKeyRangeId> {
        // Only pre-resolve for partitioned data plane operations.
        if !operation
            .resource_type()
            .is_partitioned(operation.operation_type())
        {
            return None;
        }

        // The dataflow pipeline resolves each query into per-partition
        // sub-operations and stamps the owning physical partition's range ID
        // onto the overrides. When present it is authoritative — use it as-is,
        // skipping any cache lookup (and the multi-range collapse that would
        // otherwise silently drop the seed).
        //
        // This is checked BEFORE the PPAF/PPCB gate below: the resolved range
        // ID is also required to scope the outgoing session token to a single
        // partition. The thin-client/RNTBD backend rejects a multi-range
        // composite session token on a partition-scoped request ("Session token
        // specified is invalid."), so a stamped range id must always be honored
        // even when PPAF/PPCB are both disabled.
        if let Some(pk_range_id) = overrides.partition_key_range_id.as_deref() {
            return Some(PartitionKeyRangeId::from(pk_range_id.to_owned()));
        }

        // A cache-resolved partition key range ID (below) scopes the outgoing
        // session token to a single partition and feeds PPAF/PPCB routing.
        // Resolve it whenever any consumer needs it:
        //   * PPAF or PPCB is enabled (for failure attribution / routing), OR
        //   * the client may route over Gateway 2.0 (thin-client), whose
        //     backend rejects a multi-range composite session token on a
        //     partition-scoped request ("Session token specified is invalid.").
        //
        // The Gateway 2.0 clause mirrors .NET's
        // `ThinClientStoreModel::ShouldResolvePartitionKeyRange() => true`:
        // thin-client-capable clients resolve the range unconditionally, while
        // pure classic-gateway clients keep the PPAF/PPCB-gated behavior. When
        // none of these apply, skip the cache work.
        let snapshot = self.location_state_store.snapshot();
        let partition_state = snapshot.partitions.as_ref();
        if !partition_state.per_partition_automatic_failover_enabled
            && !partition_state.per_partition_circuit_breaker_enabled
            && !self.location_state_store.gateway_v2_enabled()
        {
            return None;
        }

        // Need a container reference for any cache-backed resolution below.
        let container = operation.container()?;

        // Logical-partition-key targets resolve directly from the partition key.
        // Prefer the override (set by the dataflow pipeline) over the operation.
        let partition_key = overrides
            .partition_key
            .as_ref()
            .or_else(|| operation.target().and_then(|t| t.partition_key()));
        if let Some(partition_key) = partition_key {
            return self
                .pk_range_cache
                .resolve_partition_key_range_id(container, partition_key, false, |c, cont| {
                    Box::pin(self.fetch_pk_ranges_from_service(c, cont))
                })
                .await
                .map(PartitionKeyRangeId::from);
        }

        // EPK-range feed ranges (e.g. `SELECT * FROM c` scoped to a single physical
        // partition) carry no logical partition key. Resolve the owning physical
        // partition by EPK range so PPCB/PPAF can attribute failures from the first
        // attempt. Seed only when the range maps to exactly one physical partition:
        // a range that fans out across multiple partitions (or matches none) has no
        // single owner to attribute to, so the pipeline instead captures the range
        // ID from the response headers on a later attempt. `resolve_single_overlapping_range_id`
        // answers this without cloning every overlapping range.
        //
        // Prefer the override feed range (set by the dataflow pipeline) over the
        // operation's own target.
        let target = overrides
            .feed_range
            .as_ref()
            .or_else(|| operation.target())?;
        self.pk_range_cache
            .resolve_single_overlapping_range_id(
                container,
                target.min_inclusive()..target.max_exclusive(),
                false,
                |c, cont| Box::pin(self.fetch_pk_ranges_from_service(c, cont)),
            )
            .await
            .map(PartitionKeyRangeId::from)
    }

    /// Executes a Cosmos DB operation.
    ///
    /// This method executes an operation by planning it first and then immediately
    /// executing one page. This is sufficient for operations with trivial plans,
    /// such as point operations and single-partition queries.
    /// However, if planning is complicated and multiple pages are going to be requested,
    /// in that case, the caller should use the [`plan_operation`](Self::plan_operation)
    /// method to build a [`OperationPlan`] and then call [`execute_plan`](Self::execute_plan)
    /// for each page of the plan.
    /// Retaining the [`OperationPlan`] allows the caller to resume execution from a
    /// previous page, maintaining all state, and avoiding unnecessary replanning
    /// and continuation token management.
    ///
    /// # Parameters
    ///
    /// - `operation`: The operation to execute.
    /// - `options`: Operation-specific options that override driver and runtime defaults.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(response))` when a page of results is produced, or
    /// `Ok(None)` when the pipeline is fully drained (no more pages).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The driver has not been initialized
    /// - Planning fails (e.g. invalid operation target, backend query plan error)
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
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    ///
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    ///
    /// let driver = runtime
    ///     .create_driver(azure_data_cosmos_driver::options::DriverOptions::builder(account).build())
    ///     .await?;
    ///
    /// // Point operation: plan and execute in one call.
    /// let options = OperationOptionsBuilder::new()
    ///     .with_content_response_on_write(ContentResponseOnWrite::Disabled)
    ///     .build();
    ///
    /// // let result = driver.execute_operation(operation, options, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> crate::error::Result<Option<crate::models::CosmosResponse>> {
        // PATCH is a virtual operation type: dispatch it to the dedicated
        // Read-Modify-Write handler before any of the standard pipeline steps
        // run, because the handler issues its own Read/Replace operations
        // through this same entry point. `Box::pin` is required so the
        // resulting async future has a fixed size even though it can recurse.
        if operation.operation_type() == crate::models::OperationType::Patch {
            let max_attempts = operation.patch_max_attempts();
            return Box::pin(async {
                let result = crate::driver::pipeline::patch_handler::execute(
                    self,
                    operation,
                    options,
                    max_attempts,
                )
                .await?;
                Ok(Some(result))
            })
            .await;
        }

        // TODO: This boxing is a temporary fix to avoid a large future.
        // We need to do some refactoring here to shrink the future size and avoid this heap allocation if possible.
        Box::pin(async {
            let container = operation.container().cloned();
            let mut plan = Box::pin(self.plan_operation(operation, &options, None)).await?;
            self.execute_plan(&mut plan, container, options).await
        })
        .await
    }

    /// Executes a singleton operation (operations which return only a single result).
    ///
    /// This is a convenience method around [`execute_operation`](CosmosDriver::execute_operation) that asserts at debug-time that the operation
    /// does not return an empty page.
    pub async fn execute_singleton_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> crate::error::Result<crate::models::CosmosResponse> {
        debug_assert!(
            !operation.operation_type().is_feed(),
            "execute_singleton_operation should only be used for operations that return a single result, but '{} {}' is a feed operation",
            operation.operation_type(),
            operation.resource_type()
        );
        match self.execute_operation(operation, options).await {
            Ok(Some(r)) => Ok(r),
            Ok(None) => {
                if cfg!(debug_assertions) {
                    panic!("singleton operation returned an empty page")
                }
                Err(crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_SINGLETON_OPERATION_RETURNED_EMPTY_PAGE,
                    )
                    .with_message("internal error: singleton operation returned an empty page")
                    .build())
            }
            Err(e) => Err(e),
        }
    }

    /// Executes a preview distributed transaction through the Gateway coordinator.
    #[cfg(feature = "preview_dtx")]
    pub async fn execute_distributed_transaction(
        &self,
        mut request: crate::models::DistributedTransactionRequest,
        mut options: OperationOptions,
    ) -> crate::error::Result<crate::models::DistributedTransactionResponse> {
        if request.operations.is_empty() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("cannot execute a distributed transaction with zero operations")
                .build());
        }

        let operation_count = request.operations.len();
        let is_session_consistency = {
            let effective_options = self.operation_options_view(&options);
            let session_capturing_disabled = effective_options
                .session_capturing_disabled()
                .copied()
                .unwrap_or(false);
            let read_consistency_strategy = effective_options
                .read_consistency_strategy()
                .copied()
                .unwrap_or(crate::options::ReadConsistencyStrategy::Default);
            let account_endpoint = AccountEndpoint::from(self.options.account());
            let account_properties = self
                .runtime
                .account_metadata_cache()
                .get_or_fetch(account_endpoint, || {
                    self.fetch_account_properties(self.options.account())
                })
                .await?;
            !session_capturing_disabled
                && read_consistency_strategy.is_session_effective(
                    account_properties
                        .user_consistency_policy
                        .default_consistency_level,
                )
        };
        // Under Session consistency, stamp each operation that lacks an explicit
        // token with the resolved session token *before* serialization so the
        // coordinator honors read-your-own-writes (mirrors .NET
        // ResolvePartitionLocalToken).
        if is_session_consistency {
            self.resolve_distributed_transaction_session_tokens(&mut request.operations)
                .await;
        }
        let body = request.serialize_body()?;
        let operation = CosmosOperation::distributed_transaction(
            self.options.account().clone(),
            request.transaction_type,
        )
        .with_body(body);

        let custom_headers = options.custom_headers.get_or_insert_with(Default::default);
        custom_headers.insert(
            azure_core::http::headers::HeaderName::from_static(
                crate::models::request_header_names::DTX_IDEMPOTENCY_TOKEN,
            ),
            azure_core::http::headers::HeaderValue::from(request.idempotency_token.to_string()),
        );
        custom_headers.insert(
            azure_core::http::headers::HeaderName::from_static(
                crate::models::request_header_names::DTX_OPERATION_TYPE,
            ),
            azure_core::http::headers::HeaderValue::from(match request.transaction_type {
                crate::models::DistributedTransactionType::Write => "CommitDistributedTransaction",
                crate::models::DistributedTransactionType::Read => "Read",
            }),
        );
        custom_headers.insert(
            azure_core::http::headers::HeaderName::from_static(
                crate::models::request_header_names::DTX_RESOURCE_TYPE,
            ),
            azure_core::http::headers::HeaderValue::from_static(
                crate::models::cosmos_headers::DTX_RESOURCE_TYPE_HEADER_VALUE,
            ),
        );

        // Bound the outer retry loop by the caller's end-to-end latency budget.
        // Each iteration re-enters the pipeline with a *fresh* per-attempt
        // deadline (computed as `now + timeout` inside the pipeline), so without
        // an absolute ceiling here the caller's total timeout would not
        // constrain the retry loop — only the retry-count and cumulative-delay
        // caps would. Captured once, before the first attempt, from the same
        // `end_to_end_latency_policy` the pipeline uses.
        let outer_deadline = self
            .operation_options_view(&options)
            .end_to_end_latency_policy()
            .map(|policy| Instant::now() + policy.timeout());

        let mut retry_count = 0_u32;
        let mut cumulative_delay = Duration::ZERO;

        loop {
            let response = self
                .execute_singleton_operation(operation.clone(), options.clone())
                .await?;
            let status = response.status();
            let headers = response.headers().clone();
            let diagnostics = response.diagnostics();
            let retry_after_ms = headers.retry_after_ms;
            let body = response.into_body().single()?;

            let response = crate::models::DistributedTransactionResponse::from_body(
                status.status_code(),
                status.sub_status(),
                body.as_ref(),
                operation_count,
                request.idempotency_token,
            )
            .with_response_headers(&headers)
            .with_diagnostics(diagnostics);

            let retry_delay = distributed_transaction_outer_retry_delay(
                &response,
                retry_after_ms,
                retry_count,
                cumulative_delay,
                outer_deadline,
            );
            let Some(retry_delay) = retry_delay else {
                self.session_manager
                    .merge_distributed_transaction_session_tokens(
                        &response,
                        &request.operations,
                        is_session_consistency,
                    )?;

                return Ok(response);
            };

            retry_count += 1;
            cumulative_delay = cumulative_delay.saturating_add(retry_delay);
            azure_core::sleep(
                azure_core::time::Duration::try_from(retry_delay)
                    .unwrap_or(azure_core::time::Duration::ZERO),
            )
            .await;
        }
    }

    #[cfg(feature = "preview_dtx")]
    async fn resolve_distributed_transaction_session_tokens(
        &self,
        operations: &mut [crate::models::DistributedTransactionOperation],
    ) {
        for operation in operations.iter_mut() {
            if operation.session_token.is_some() {
                continue;
            }

            let ranges = self
                .resolve_partition_key_ranges_for_key(
                    &operation.target.container,
                    &operation.target.partition_key,
                    false,
                )
                .await;
            let range = ranges.as_deref().and_then(|ranges| match ranges {
                [single] => Some(single),
                _ => None,
            });

            if let Some(token) = self
                .session_manager
                .resolve_distributed_transaction_session_token(operation, range)
            {
                operation.session_token = Some(token);
            }
        }
    }

    /// Executes a single page of a pre-planned operation using the given plan and options.
    ///
    /// This function mutates the plan in place to account for any changes that occur during execution
    /// (e.g. topology repairs, advancing page state, etc.).
    /// After this returns, the plan may be executed again to fetch the next page of results, if any.
    /// Once this returns `None`, there are no more pages to fetch, and the operation is complete.
    pub async fn execute_plan(
        &self,
        plan: &mut OperationPlan,
        container: Option<ContainerReference>,
        options: OperationOptions,
    ) -> crate::error::Result<Option<crate::models::CosmosResponse>> {
        if !self.initialized.load(Ordering::Acquire) {
            let endpoint = AccountEndpoint::from(self.options.account());
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_DRIVER_NOT_INITIALIZED)
                .with_message(format!(
                    "CosmosDriver for {endpoint} has not been initialized; call initialize() or \
                     use CosmosDriverRuntime::create_driver() which initializes automatically"
                ))
                .build());
        }
        tracing::debug!("plan execution started");

        let mut executor = DriverRequestExecutor {
            driver: self,
            options: &options,
        };

        let mut topology = container.map(|c| {
            CachedTopologyProvider::new(&self.pk_range_cache, c, |container, continuation| {
                self.fetch_pk_ranges_from_service(container, continuation)
            })
        });

        let mut context = PipelineContext::new(
            &mut executor,
            topology.as_mut().map(|t| t as &mut dyn TopologyProvider),
        );

        plan.pipeline.next_page(&mut context).await
    }

    async fn execute_operation_direct(
        &self,
        operation: &CosmosOperation,
        overrides: OperationOverrides,
        options: &OperationOptions,
    ) -> crate::error::Result<CosmosResponse> {
        tracing::debug!(
            operation_type = ?operation.operation_type(),
            resource_type = ?operation.resource_type(),
            resource_reference = ?operation.resource_reference(),
            overrides = ?overrides,
            body_length = operation.body().map(|b| b.len()),
            "executing operation");

        // Step 1: Build the single OperationOptionsView for layered resolution.
        let effective_options = self.operation_options_view(options);

        // Step 2: Resolve effective throughput control headers.
        let effective_throughput_control = if let Some(container) = operation.container() {
            Some(self.effective_throughput_control(&effective_options, container)?)
        } else {
            // Throughput control doesn't apply to operations that don't target a container.
            // But it's not an error to specify the settings, they may have been inherited from
            // a parent context. We just disregard them.
            None
        };

        // Step 3: Initialize operation activity id
        let activity_id = ActivityId::new_uuid();

        // Step 4: Get authentication (guaranteed to be present by AccountReference)
        let account = operation.resource_reference().account();
        let auth = account.auth();

        // Step 4.1: Resolve account metadata and select write-region endpoint.
        // Uses `get_or_fetch` (cheap, no staleness check) because the
        // background account-metadata refresh loop spawned in
        // `CosmosDriver::new` keeps this cache fresh on a periodic timer.
        // The lazy `refresh_if_stale` variant is intentionally NOT used here
        // — the timer owns freshness so the per-operation hot path stays
        // free of network round-trips.
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
            Arc::clone(&account_properties),
            self.location_state_store.default_endpoint(),
        );

        let write_region = account_properties.write_account_region();
        let endpoint = Self::endpoint_for_write_region(account, write_region);

        // Step 5: Pre-resolve partition key range ID for PPAF/PPCB.
        // When partition-level failover is enabled, resolving the range ID
        // before the first attempt lets the pipeline apply partition overrides
        // from the very first request instead of only after the first retry.
        // Pass the overrides so dataflow-stamped routing (PK range ID, partition
        // key, EPK range) is honored ahead of the operation's own target.
        let pre_resolved_pk_range_id = self
            .pre_resolve_partition_key_range_id(operation, &overrides)
            .await;

        // Step 6: Select the adaptive transport context for the chosen pipeline
        let transport = self.transport();
        let operation_type = operation.operation_type();
        let resource_type = operation.resource_type();
        let is_dataplane = uses_dataplane_pipeline(resource_type, operation_type);
        // Step 7: Initialize diagnostics (shared envelope shape with the bootstrap fetch).
        let fault_injection_enabled = {
            #[cfg(feature = "fault_injection")]
            {
                self.fault_injection_enabled
            }
            #[cfg(not(feature = "fault_injection"))]
            {
                false
            }
        };
        let (diagnostics_builder, transport_security) = Self::new_diagnostics_envelope(
            &self.runtime,
            activity_id.clone(),
            &endpoint,
            fault_injection_enabled,
        );

        let pipeline_type = if is_dataplane {
            PipelineType::DataPlane
        } else {
            PipelineType::Metadata
        };

        let user_agent =
            azure_core::http::headers::HeaderValue::from(self.user_agent.as_str().to_owned());

        // Step 8: Execute via the new operation pipeline
        super::pipeline::operation_pipeline::execute_operation_pipeline(
            operation,
            overrides,
            &effective_options,
            options.custom_headers.as_ref(),
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
            effective_throughput_control,
            pre_resolved_pk_range_id,
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
    /// # async fn example() -> azure_data_cosmos_driver::error::Result<()> {
    /// let runtime = CosmosDriverRuntime::builder().build().await?;
    /// let account = AccountReference::with_master_key(
    ///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
    ///     "my-key",
    /// );
    /// let driver = runtime
    ///     .create_driver(azure_data_cosmos_driver::options::DriverOptions::builder(account).build())
    ///     .await?;
    ///
    /// // Resolve the container (fetched from service on each call)
    /// let container = driver.resolve_container("mydb", "mycontainer").await?;
    ///
    /// // Use the resolved container for item operations
    /// let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");
    /// let result = driver
    ///     .execute_singleton_operation(CosmosOperation::read_item(item), OperationOptions::default())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resolve_container(
        &self,
        db_name: &str,
        container_name: &str,
    ) -> crate::error::Result<ContainerReference> {
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
    ) -> crate::error::Result<ContainerReference> {
        let endpoint = self.account().endpoint().as_str().to_owned();
        let db_name_owned = db_name.to_owned();
        let container_name_owned = container_name.to_owned();

        let resolved = self
            .runtime
            .container_cache()
            .get_or_fetch_by_name(&endpoint, db_name, container_name, || async move {
                self.fetch_container_by_name(&db_name_owned, &container_name_owned)
                    .await
                    .map_err(|err| {
                        crate::error::CosmosErrorBuilder::from_error(err)
                            .with_context(format!(
                                "resolve container by name (db='{db_name_owned}', container='{container_name_owned}')"
                            ))
                            .build()
                    })
            })
            .await?;

        Ok(resolved.as_ref().clone())
    }

    /// Plans the execution of a Cosmos DB operation.
    ///
    /// For trivial operations (non-query or single-partition), returns a
    /// singleton pipeline immediately. For cross-partition queries, fetches a
    /// query plan from the backend and builds a fan-out pipeline.
    ///
    /// `continuation` optionally provides resume state from a prior call. Two
    /// kinds of tokens are accepted:
    ///
    /// - SDK-issued tokens (`c1.…`) carry a serialized snapshot of the
    ///   previous pipeline's state and can resume any operation.
    /// - Opaque server-issued tokens (no `c<N>.` prefix) are accepted only
    ///   for trivial operations; passing one to a cross-partition query
    ///   returns a `Client`-shaped error.
    pub async fn plan_operation(
        &self,
        operation: CosmosOperation,
        options: &OperationOptions,
        continuation: Option<&ContinuationToken>,
    ) -> crate::error::Result<OperationPlan> {
        if !self.initialized.load(Ordering::Acquire) {
            let endpoint = AccountEndpoint::from(self.options.account());
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_DRIVER_NOT_INITIALIZED)
                .with_message(format!(
                    "CosmosDriver for {endpoint} has not been initialized; call initialize() or \
                     use CosmosDriverRuntime::create_driver() which initializes automatically"
                ))
                .build());
        }

        tracing::debug!(operation_type = ?operation.operation_type(), resource_type = ?operation.resource_type(), resource_reference = ?operation.resource_reference(), "planning operation");

        // Share the operation across every Request node in the resulting plan.
        // Per-Request differences are layered on at execution time via
        // OperationOverrides; the operation itself is never mutated.
        let operation = Arc::new(operation);

        // Resolve the continuation token (if any) into a planner-ready resume
        // state. Server-issued tokens are only valid for trivial operations.
        let resume_state = match continuation {
            None => None,
            Some(token) => {
                match token.resolve()? {
                    ResolvedToken::ClientV1(state) => {
                        // Validate the state is valid for this operation.
                        state.is_valid_for_operation(&operation)?;
                        Some(state.into_root_node_state())
                    }
                    ResolvedToken::ServerOpaque(server_token) => {
                        if !operation.is_trivial() {
                            return Err(crate::error::CosmosError::builder().with_status(crate::error::CosmosStatus::CLIENT_OPAQUE_TOKEN_INVALID_FOR_CROSS_PARTITION_QUERY)
                        .with_message(
                            "an opaque server continuation token cannot be used to resume a \
                             cross-partition query; use the SDK-issued continuation token from \
                             QueryPageIterator::to_continuation_token()",
                        )
                        .build());
                        }
                        Some(PipelineNodeState::Request {
                            server_continuation: Some(server_token),
                        })
                    }
                }
            }
        };

        // 1. Trivial plan: anything that isn't a cross-partition query.
        //    The internal pipeline only supports projection and filtering over
        //    a sequential drain, with no support for ordering. Trivial
        //    operations (targeting a single logical partition) are sent directly
        //    to the gateway without query planning.
        if operation.is_trivial() {
            let pipeline = planner::build_trivial_pipeline(operation.clone(), resume_state)?;
            return Ok(OperationPlan::new(pipeline, operation));
        }

        // 2. Change feed: resolve the target feed range against the current
        //    topology and build an UnorderedMerge pipeline (no query plan
        //    needed). Children are polled round-robin and never evicted on
        //    304 so the stream is infinite.
        if operation.is_change_feed() {
            let container = operation.container().ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(
                        crate::error::CosmosStatus::CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF,
                    )
                    .with_message("cross-partition change feed requires a container reference")
                    .build()
            })?;
            let feed_range = operation.target().cloned().unwrap_or_else(FeedRange::full);
            let container_ref = container.clone();
            let mut topology = CachedTopologyProvider::new(
                &self.pk_range_cache,
                container_ref,
                |container, continuation| {
                    self.fetch_pk_ranges_from_service(container, continuation)
                },
            );
            let pipeline = planner::build_unordered_merge(
                &feed_range,
                &mut topology,
                &operation,
                resume_state,
            )
            .await?;
            return Ok(OperationPlan::new(pipeline, operation));
        }

        // 3. Cross-partition query: obtain a query plan and build the fan-out
        //    pipeline. Try the native FFI provider first (no network call),
        //    falling back to the Gateway if unavailable.
        let container = operation.container().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(
                    crate::error::CosmosStatus::CLIENT_CROSS_PARTITION_QUERY_REQUIRES_CONTAINER_REF,
                )
                .with_message("cross-partition query requires a container reference")
                .build()
        })?;

        let query_plan = self
            .resolve_query_plan(container, &operation, options)
            .await?;

        // Build the fan-out pipeline using the query plan.
        let container_ref = container.clone();
        let mut topology = CachedTopologyProvider::new(
            &self.pk_range_cache,
            container_ref,
            |container, continuation| self.fetch_pk_ranges_from_service(container, continuation),
        );

        let pipeline =
            planner::build_sequential_drain(&query_plan, &mut topology, &operation, resume_state)
                .await?;
        Ok(OperationPlan::new(pipeline, operation))
    }

    /// Fetches a query plan from the Gateway backend.
    async fn gateway_query_plan(
        &self,
        container: &ContainerReference,
        operation: &CosmosOperation,
        options: &OperationOptions,
    ) -> crate::error::Result<QueryPlan> {
        // Advertise the SDK's supported query-rewrite features. The default
        // (`SUPPORTED_QUERY_FEATURES`) is `"None"` until the cross-partition
        // pipeline gains rewrite support, but the value must be non-empty so
        // the Gateway V2 thin-client proxy accepts the QueryPlan request.
        let query_plan_operation = CosmosOperation::query_plan(
            container.clone(),
            std::borrow::Cow::Borrowed(crate::query::SUPPORTED_QUERY_FEATURES),
        )
        .with_body(operation.body().unwrap_or_default().to_vec());

        let response = self
            .execute_operation_direct(
                &query_plan_operation,
                OperationOverrides::default(),
                options,
            )
            .await?;

        let query_plan_body = match response.body() {
            crate::models::ResponseBody::Bytes(b) => b.clone(),
            _ => {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("query plan response did not contain a body")
                    .with_source(std::io::Error::other("missing body"))
                    .build());
            }
        };
        let raw_query_plan: RawQueryPlan =
            serde_json::from_slice(&query_plan_body).map_err(|e| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("failed to parse query plan response")
                    .with_source(e)
                    .build()
            })?;
        // Resolve proxy-format `queryRanges` (PartitionKeyInternal arrays)
        // into canonical EPK hex strings using the container's PK definition.
        let query_plan = raw_query_plan.resolve(container.partition_key_definition())?;
        Ok(query_plan)
    }

    /// Resolves a query plan, trying the native FFI provider first and
    /// falling back to the Gateway if unavailable or if generation fails.
    #[cfg(feature = "__internal_native_query_plan")]
    async fn resolve_query_plan(
        &self,
        container: &ContainerReference,
        operation: &CosmosOperation,
        options: &OperationOptions,
    ) -> crate::error::Result<QueryPlan> {
        // Fetch the query engine configuration from the account metadata cache.
        let account = operation.resource_reference().account();
        let account_endpoint = AccountEndpoint::from(account);
        let query_engine_config = self
            .runtime
            .account_metadata_cache()
            .get(&account_endpoint)
            .await
            .map(|props| props.query_engine_configuration.clone())
            .unwrap_or_default();

        let native_result = operation
            .body()
            .and_then(|b| std::str::from_utf8(b).ok())
            .map(|json| self.try_native_query_plan(json, container, &query_engine_config));

        match native_result {
            Some(Ok(plan)) => {
                tracing::debug!("using native FFI query plan");
                Ok(plan)
            }
            Some(Err(crate::query_plan_native::error::QueryPlanError::LibraryNotAvailable {
                ..
            })) => {
                tracing::debug!("native query plan library not available, falling back to gateway");
                self.gateway_query_plan(container, operation, options).await
            }
            Some(Err(e)) => {
                tracing::warn!(error = %e, "native query plan generation failed, falling back to gateway");
                self.gateway_query_plan(container, operation, options).await
            }
            None => {
                tracing::debug!("using gateway query plan");
                self.gateway_query_plan(container, operation, options).await
            }
        }
    }

    /// Resolves a query plan from the Gateway backend.
    #[cfg(not(feature = "__internal_native_query_plan"))]
    async fn resolve_query_plan(
        &self,
        container: &ContainerReference,
        operation: &CosmosOperation,
        options: &OperationOptions,
    ) -> crate::error::Result<QueryPlan> {
        self.gateway_query_plan(container, operation, options).await
    }

    /// Attempts to generate a query plan using the native FFI provider.
    /// Returns `Err` if the native library is not available or if plan
    /// generation fails for any reason. The caller should fall through
    /// to the Gateway path on error.
    #[cfg(feature = "__internal_native_query_plan")]
    fn try_native_query_plan(
        &self,
        query_spec_json: &str,
        container: &ContainerReference,
        query_engine_config: &str,
    ) -> Result<QueryPlan, crate::query_plan_native::error::QueryPlanError> {
        let pk_def = container.partition_key_definition();
        let pk_paths: Vec<&str> = pk_def.paths().iter().map(|p| p.as_ref()).collect();
        self.native_query_plan_provider.get_query_plan(
            query_spec_json,
            &pk_paths,
            pk_def.kind(),
            query_engine_config,
        )
    }

    /// Returns all partition key ranges for a container, ordered by min EPK.
    ///
    /// Uses the driver's internal `PartitionKeyRangeCache`. When `force_refresh`
    /// is `true`, the cached routing map is refreshed from the service before
    /// returning results. Returns `None` if the routing map cannot be resolved.
    pub async fn resolve_all_partition_key_ranges(
        &self,
        container: &ContainerReference,
        force_refresh: bool,
    ) -> Option<Vec<crate::models::partition_key_range::PartitionKeyRange>> {
        let routing_map = self
            .pk_range_cache
            .try_lookup(container, force_refresh, |c, cont| {
                Box::pin(self.fetch_pk_ranges_from_service(c, cont))
            })
            .await?;

        let ranges = routing_map.ranges();
        if ranges.is_empty() {
            // A valid container always has at least one partition key range.
            // An empty routing map indicates a service/parse failure.
            return None;
        }
        Some(ranges.to_vec())
    }

    /// Returns the partition key ranges covering the given partition key.
    ///
    /// Handles both full keys (single range via point lookup) and prefix keys
    /// on MultiHash containers (multiple ranges via overlapping range lookup).
    ///
    /// Returns `None` if the partition key is empty or the routing map cannot
    /// be resolved. When `force_refresh` is `true`, the cached routing map is
    /// refreshed from the service before lookup.
    pub async fn resolve_partition_key_ranges_for_key(
        &self,
        container: &ContainerReference,
        partition_key: &PartitionKey,
        force_refresh: bool,
    ) -> Option<Vec<crate::models::partition_key_range::PartitionKeyRange>> {
        if partition_key.is_empty() {
            return None;
        }

        let pk_def = container.partition_key_definition();
        let epk_range = match EffectivePartitionKey::compute_range(partition_key.values(), pk_def) {
            Ok(range) => range,
            Err(e) => {
                tracing::warn!("EPK computation failed for partition key: {e}");
                return None;
            }
        };

        if epk_range.start == epk_range.end {
            // Full key — point lookup
            let routing_map = self
                .pk_range_cache
                .try_lookup(container, force_refresh, |c, cont| {
                    Box::pin(self.fetch_pk_ranges_from_service(c, cont))
                })
                .await?;
            if routing_map.ranges().is_empty() {
                return None;
            }
            Some(
                routing_map
                    .get_range_by_effective_partition_key(&epk_range.start)
                    .cloned()
                    .map_or_else(Vec::new, |r| vec![r]),
            )
        } else {
            // Prefix key — overlapping range lookup
            self.pk_range_cache
                .resolve_overlapping_ranges(
                    container,
                    &epk_range.start..&epk_range.end,
                    force_refresh,
                    |c, cont| Box::pin(self.fetch_pk_ranges_from_service(c, cont)),
                )
                .await
        }
    }
}

#[cfg(feature = "preview_dtx")]
fn distributed_transaction_outer_retry_delay(
    response: &crate::models::DistributedTransactionResponse,
    retry_after_ms: Option<u64>,
    retry_count: u32,
    cumulative_delay: Duration,
    deadline: Option<Instant>,
) -> Option<Duration> {
    if response.is_completed_status_code() || !response.is_retriable {
        return None;
    }

    if retry_count >= DTX_OUTER_MAX_RETRIES {
        return None;
    }

    let computed_delay = distributed_transaction_outer_computed_delay(retry_count);
    let server_delay = retry_after_ms
        .map(Duration::from_millis)
        .unwrap_or(Duration::ZERO);
    let delay = computed_delay.max(server_delay);
    let next_cumulative = cumulative_delay.checked_add(delay)?;
    if next_cumulative > DTX_OUTER_MAX_CUMULATIVE_DELAY {
        return None;
    }

    // Never sleep past the caller's end-to-end deadline: if the next attempt
    // could not start until at or after the deadline, stop and surface the
    // current response instead of retrying.
    if let Some(deadline) = deadline {
        if Instant::now() + delay >= deadline {
            return None;
        }
    }

    Some(delay)
}

#[cfg(feature = "preview_dtx")]
fn distributed_transaction_outer_computed_delay(retry_count: u32) -> Duration {
    let exponent = retry_count.min(DTX_OUTER_MAX_EXPONENT);
    let delay_seconds = DTX_OUTER_BASE_DELAY.as_secs_f64() * 2_f64.powi(exponent as i32);
    Duration::from_secs_f64(crate::driver::jitter::with_jitter(
        delay_seconds,
        DTX_OUTER_JITTER_RATIO,
    ))
}

/// Sends a lightweight `GET /probe` connectivity check to a single endpoint
/// and reports whether the endpoint is reachable.
///
/// Account-level failback is gated on *network reachability*, not on a full
/// database-account read succeeding. Any wire response — even a non-2xx
/// envelope (401/403/429/503/5xx) — proves the endpoint accepted the
/// connection and is reachable. Only a transport error with no response
/// (firewall block, DNS failure, connection refused, or connection timeout)
/// means the endpoint is unreachable and must stay out of rotation.
///
/// Hitting the dedicated `/probe` path (rather than re-reading the database
/// account) keeps the probe off the metadata code path and minimizes the
/// load it places on the service. See issue #4597.
async fn probe_endpoint_connectivity(
    transport: &super::transport::adaptive_transport::AdaptiveTransport,
    account: &AccountReference,
    user_agent: &azure_core::http::headers::HeaderValue,
) -> bool {
    let endpoint = AccountEndpoint::from(account);
    let mut request = HttpRequest {
        url: endpoint.join_path("/probe"),
        method: azure_core::http::Method::Get,
        headers: azure_core::http::headers::Headers::new(),
        body: None,
        timeout: None,
        #[cfg(feature = "fault_injection")]
        evaluation_collector: None,
    };
    cosmos_headers::apply_cosmos_headers(&mut request, user_agent);

    // Any wire response (including a non-2xx envelope) proves reachability;
    // only a transport error with no response means the endpoint could not
    // be reached.
    transport.send(&request).await.is_ok()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use async_trait::async_trait;
    use azure_core::http::headers::Headers;

    use url::Url;

    use crate::{
        driver::CosmosDriverRuntimeBuilder,
        models::AccountReference,
        options::{
            ContentResponseOnWrite, CorrelationId, DriverOptionsBuilder, OperationOptionsBuilder,
            UserAgentSuffix, WorkloadId,
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
        /// 503 body the gateway returns under load (Cosmos-flavored JSON, no `_self`).
        /// Without status-gating the driver would relabel it as a deserialization failure.
        ServiceUnavailable503,
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
                    crate::error::CosmosError::builder()
                        .with_status(crate::models::CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE)
                        .with_message("http2 not supported")
                        .with_source(h2::Error::from(h2::Reason::HTTP_1_1_REQUIRED))
                        .build(),
                    crate::diagnostics::RequestSentStatus::NotSent,
                )),
                ResponsePlan::ConnectionError => Err(TransportError::new(
                    crate::error::CosmosError::builder()
                        .with_status(crate::models::CosmosStatus::TRANSPORT_CONNECTION_FAILED)
                        .with_message("simulated connection refused")
                        .build(),
                    crate::diagnostics::RequestSentStatus::NotSent,
                )),
                ResponsePlan::ServiceUnavailable503 => Ok(HttpResponse {
                    status: 503,
                    headers: Headers::new(),
                    body: br#"{"code":"ServiceUnavailable","message":"pgcosmos extension is still starting; retry request shortly"}"#.to_vec(),
                }),
            }
        }
    }

    fn scripted_client(plan: ResponsePlan) -> Arc<dyn TransportClient> {
        let client: Box<dyn TransportClient> = Box::new(ScriptedClient { plan });
        Arc::from(client)
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
        ) -> crate::error::Result<Arc<dyn TransportClient>> {
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

            Ok(scripted_client(plan))
        }
    }

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    #[cfg(feature = "preview_dtx")]
    fn dtx_response(
        status_code: azure_core::http::StatusCode,
        is_retriable: bool,
    ) -> crate::models::DistributedTransactionResponse {
        crate::models::DistributedTransactionResponse {
            status_code,
            sub_status_code: None,
            operation_results: Vec::new(),
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable,
            diagnostic_string: None,
            error_message: None,
        }
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_outer_retry_delay_stops_on_success_or_non_retriable() {
        assert!(distributed_transaction_outer_retry_delay(
            &dtx_response(azure_core::http::StatusCode::Ok, true),
            None,
            0,
            Duration::ZERO,
            None,
        )
        .is_none());
        assert!(distributed_transaction_outer_retry_delay(
            &dtx_response(azure_core::http::StatusCode::from(449_u16), false),
            None,
            0,
            Duration::ZERO,
            None,
        )
        .is_none());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_outer_retry_delay_uses_larger_retry_after() {
        let delay = distributed_transaction_outer_retry_delay(
            &dtx_response(azure_core::http::StatusCode::from(449_u16), true),
            Some(5_000),
            0,
            Duration::ZERO,
            None,
        )
        .unwrap();

        assert_eq!(delay, Duration::from_secs(5));
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_outer_retry_delay_stops_at_retry_cap_and_cumulative_budget() {
        let response = dtx_response(azure_core::http::StatusCode::from(449_u16), true);
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            None,
            DTX_OUTER_MAX_RETRIES,
            Duration::ZERO,
            None,
        )
        .is_none());
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            Some(1_000),
            0,
            DTX_OUTER_MAX_CUMULATIVE_DELAY,
            None,
        )
        .is_none());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_outer_retry_delay_stops_at_caller_deadline() {
        let response = dtx_response(azure_core::http::StatusCode::from(449_u16), true);

        // A deadline already in the past stops the outer loop even though the
        // retry-count and cumulative-delay budgets still allow a retry.
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            None,
            0,
            Duration::ZERO,
            Some(Instant::now() - Duration::from_secs(1)),
        )
        .is_none());

        // A generous deadline leaves the normal retry behavior intact.
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            None,
            0,
            Duration::ZERO,
            Some(Instant::now() + Duration::from_secs(3600)),
        )
        .is_some());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_bodyless_infra_envelope_stops_outer_loop() {
        // Two-tier retry composition: after the inner bodyless classifier
        // exhausts its infra budget it surfaces the body-less 500/5411 envelope
        // as a completed transport result. `from_body` must parse that empty
        // body as non-retriable (`isRetriable` defaults to false), so the outer
        // coordinator loop stops instead of retrying a body-less envelope.
        let response = crate::models::DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::InternalServerError,
            Some(crate::models::SubStatusCode::DTC_LEDGER_FAILURE),
            &[],
            1,
            uuid::Uuid::nil(),
        );
        assert!(!response.is_retriable);
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            None,
            0,
            Duration::ZERO,
            None,
        )
        .is_none());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_body_bearing_retriable_envelope_drives_outer_loop() {
        // The complement: a body-bearing coordinator envelope that declares
        // `isRetriable` keeps the outer loop retrying (within budget). This is
        // the hand-off the inner classifier defers to for body-bearing results.
        let response = crate::models::DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::from(449_u16),
            Some(crate::models::SubStatusCode::DTC_COORDINATOR_RACE_CONFLICT),
            br#"{"isRetriable":true}"#,
            1,
            uuid::Uuid::nil(),
        );
        assert!(response.is_retriable);
        assert!(distributed_transaction_outer_retry_delay(
            &response,
            None,
            0,
            Duration::ZERO,
            None,
        )
        .is_some());
    }

    #[tokio::test]
    async fn default_operation_options() {
        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        assert!(runtime
            .default_operation_options()
            .throughput_control
            .is_none());
        assert!(runtime
            .default_operation_options()
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
            .with_default_operation_options(opts)
            .build()
            .await
            .unwrap();

        assert_eq!(
            runtime.default_operation_options().max_failover_retry_count,
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
            .default_operation_options()
            .max_failover_retry_count
            .is_none());

        // Replace runtime options atomically
        let new_opts = OperationOptionsBuilder::new()
            .with_max_failover_retry_count(5)
            .build();
        runtime.set_default_operation_options(new_opts);

        // Now set
        assert_eq!(
            runtime.default_operation_options().max_failover_retry_count,
            Some(5)
        );
    }

    #[tokio::test]
    async fn effective_options_merge_priority() {
        // Build runtime (no operation options at runtime level yet)
        let cosmos_runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();

        // Driver has no operation options override either
        let driver_options = DriverOptions::builder(test_account()).build();

        let driver = CosmosDriver::new(cosmos_runtime, driver_options)
            .expect("CosmosDriver::new should succeed in tests");

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

        let driver = CosmosDriver::new(cosmos_runtime, driver_options)
            .expect("CosmosDriver::new should succeed in tests");

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
            database_account_endpoint: AccountEndpoint::try_from(
                "https://myaccount-westus.documents.azure.com:443/",
            )
            .unwrap(),
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
    fn build_account_properties_request_sets_thinclient_discovery_header() {
        let endpoint = AccountEndpoint::try_from("https://test.documents.azure.com:443/").unwrap();
        let user_agent = azure_core::http::headers::HeaderValue::from_static("test-ua");

        let request = CosmosDriver::build_account_properties_request(&endpoint, &user_agent);

        let opt_in = request
            .headers
            .get_optional_str(&GATEWAY_V2_DISCOVERY_OPT_IN)
            .expect("getDatabaseAccount must send x-ms-cosmos-use-thinclient so the server emits thinClient*Locations");
        assert_eq!(
            opt_in, "true",
            "x-ms-cosmos-use-thinclient must be `true` to enable thin-client discovery"
        );
        assert_eq!(request.method, azure_core::http::Method::Get);
        assert!(request.url.as_str().starts_with(endpoint.url().as_str()));
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
        let error = crate::error::CosmosError::builder()
            .with_status(crate::models::CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE)
            .with_message("http2 not supported")
            .with_source(h2::Error::from(h2::Reason::HTTP_1_1_REQUIRED))
            .build();

        assert!(CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn connection_error_without_http2_signal_does_not_trigger_downgrade() {
        let error = crate::error::CosmosError::builder()
            .with_status(crate::models::CosmosStatus::TRANSPORT_CONNECTION_FAILED)
            .with_message("connect failed")
            .build();

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn io_error_without_http2_signal_does_not_trigger_downgrade() {
        let error = crate::error::CosmosError::builder()
            .with_status(crate::models::CosmosStatus::TRANSPORT_IO_FAILED)
            .with_message("socket reset")
            .build();

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http2,
            &error,
            true,
        ));
    }

    #[test]
    fn http11_errors_do_not_trigger_probe_back_to_http2() {
        let error = crate::error::CosmosError::builder()
            .with_status(crate::models::CosmosStatus::TRANSPORT_CONNECTION_FAILED)
            .with_message("connect failed")
            .build();

        assert!(!CosmosDriver::should_downgrade_http2(
            TransportHttpVersion::Http11,
            &error,
            true,
        ));
    }

    #[test]
    fn downgrade_requires_http2_to_be_enabled() {
        let error = crate::error::CosmosError::builder()
            .with_status(crate::models::CosmosStatus::TRANSPORT_CONNECTION_FAILED)
            .with_message("connect failed")
            .build();

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
            .with_server_certificate_validation(
                crate::options::ServerCertificateValidation::RequiredUnlessEmulator,
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
            matches!(config.version_policy, HttpVersionPolicy::Http11Only)
                && config.allow_invalid_cert
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
                    .with_server_certificate_validation(
                        crate::options::ServerCertificateValidation::RequiredUnlessEmulator,
                    )
                    .build()
                    .unwrap(),
            )
            .with_http_client_factory(factory.clone())
            .build()
            .await
            .unwrap();
        let account = signed_test_account("https://localhost:8081/");

        let (version, properties) = CosmosDriver::fetch_initial_account_properties(
            &runtime,
            runtime.http_client_factory(),
            &account,
            false,
        )
        .await
        .unwrap();

        assert_eq!(version, TransportHttpVersion::Http11);
        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert!(factory.configs().iter().any(|config| {
            matches!(config.version_policy, HttpVersionPolicy::Http11Only)
                && config.allow_invalid_cert
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

        let properties = CosmosDriver::refresh_account_properties(
            &runtime,
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            None,
            false,
        )
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

        let properties = CosmosDriver::refresh_account_properties(
            &runtime,
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            None,
            false,
        )
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

        let properties = CosmosDriver::refresh_account_properties(
            &runtime,
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            None,
            false,
        )
        .await
        .unwrap();

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
        assert_eq!(
            transport_holder.load().negotiated_version(),
            TransportHttpVersion::Http11
        );
    }

    /// Compile-time assertion that functions are send.
    ///
    /// This function is never called; it only needs to compile.
    #[allow(dead_code, unreachable_code, unused_variables)]
    fn _assert_functions_are_send() {
        fn assert_send<T: Send>(_: T) {}
        let driver: &CosmosDriver = todo!();
        assert_send(driver.execute_operation(todo!(), todo!()));
        assert_send(driver.execute_singleton_operation(todo!(), todo!()));
        assert_send(driver.execute_plan(todo!(), todo!(), todo!()));
        assert_send(driver.plan_operation(todo!(), todo!(), todo!()));
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

    #[test]
    fn effective_partition_key_range_override_sets_feed_range() {
        let range = crate::models::FeedRange::new(
            EffectivePartitionKey::from("10"),
            EffectivePartitionKey::from("20"),
        )
        .unwrap();
        let pkrange = crate::models::FeedRange::new(
            EffectivePartitionKey::from("00"),
            EffectivePartitionKey::from("40"),
        )
        .unwrap();
        let overrides = request_target_overrides(
            None,
            RequestTarget::effective_partition_key_range(
                range.clone(),
                "merged".to_string(),
                pkrange.clone(),
            ),
            Some("ct".to_string()),
        );

        assert_eq!(overrides.partition_key_range_id.as_deref(), Some("merged"));
        assert_eq!(overrides.continuation.as_deref(), Some("ct"));
        assert_eq!(overrides.feed_range, Some(range));
        assert_eq!(overrides.pkrange_bounds, Some(pkrange));
    }

    #[test]
    fn effective_partition_key_range_override_omits_feed_range_when_full_pkrange() {
        // When the request covers the FULL pkrange (range == partition_key_range),
        // `feed_range` collapses to None — we do NOT emit the public
        // `x-ms-start-epk`/`x-ms-end-epk` headers on the legacy gateway path
        // (it rejects them paired with `partitionkeyrangeid` when min is the
        // empty-string sentinel). The pkrange bounds are still carried in
        // `pkrange_bounds` for the GW_V2 dispatcher to derive its
        // `StartEpkHash`/`EndEpkHash` RNTBD tokens.
        let range = crate::models::FeedRange::new(
            EffectivePartitionKey::from("10"),
            EffectivePartitionKey::from("20"),
        )
        .unwrap();
        let overrides = request_target_overrides(
            None,
            RequestTarget::effective_partition_key_range(
                range.clone(),
                "pkrange".to_string(),
                range.clone(),
            ),
            None,
        );

        assert_eq!(overrides.partition_key_range_id.as_deref(), Some("pkrange"));
        assert_eq!(overrides.feed_range, None);
        assert_eq!(overrides.pkrange_bounds, Some(range));
    }

    #[test]
    fn effective_partition_key_range_override_forwards_logical_partition_key() {
        // Regression: partition-scoped queries (e.g. `FeedScope::partition(partial_hpk)`)
        // decompose into per-pkrange `EffectivePartitionKeyRange` targets. The operation's
        // logical partition key must be forwarded into the override so the
        // `x-ms-documentdb-partitionkey` HTTP header (and the RNTBD `PartitionKey` 0x002B
        // token on the thin-client proxy) is emitted on every per-pkrange fan-out request.
        // Without this, the thin-client backend returns every document in the physical
        // partition because it has no per-component prefix to filter by.
        let range = crate::models::FeedRange::new(
            EffectivePartitionKey::from("10"),
            EffectivePartitionKey::from("20"),
        )
        .unwrap();
        let pk = PartitionKey::from("tenant-prefix");
        let overrides = request_target_overrides(
            Some(&pk),
            RequestTarget::effective_partition_key_range(
                range.clone(),
                "pkrange".to_string(),
                range.clone(),
            ),
            None,
        );

        assert_eq!(overrides.partition_key.as_ref(), Some(&pk));
        assert_eq!(overrides.partition_key_range_id.as_deref(), Some("pkrange"));
        assert_eq!(overrides.feed_range, None);
        assert_eq!(overrides.pkrange_bounds, Some(range));
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
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            Some(multi_region_previous_props()),
            false,
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
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            Some(multi_region_previous_props()),
            false,
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

        let result = CosmosDriver::refresh_account_properties(
            &runtime,
            runtime.http_client_factory(),
            &account,
            &transport_holder,
            runtime.user_agent(),
            None,
            false,
        )
        .await;

        assert!(result.is_err(), "should fail without previous props");
    }

    /// Regression: 503 with a Cosmos error envelope must surface as upstream HTTP status,
    /// not relabeled as `SERIALIZATION_RESPONSE_BODY_INVALID` ("missing field `_self`").
    #[tokio::test]
    async fn fetch_account_properties_surfaces_5xx_body_as_status_error() {
        let client = scripted_client(ResponsePlan::ServiceUnavailable503);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let err = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
        .expect_err(
            "503 ServiceUnavailable response with a non-empty JSON envelope must surface as an error",
        );

        let status = err.status();
        let rendered = format!("{err:?}");

        assert_ne!(
            status,
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "5xx body must NOT be reported as a deserialization failure; \
             expected an upstream-status error (e.g. 503 ServiceUnavailable). \
             Got status={status:?} err={rendered}"
        );
        assert!(
            !rendered.contains("missing field `_self`"),
            "the user-visible error must not leak the internal \
             `missing field \\`_self\\`` serde detail. Got: {rendered}"
        );
        assert_eq!(
            u16::from(status.status_code()),
            503,
            "the surfaced error should reflect the upstream HTTP 503 status. \
             Got status={status:?} err={rendered}"
        );
        assert_eq!(
            status.sub_status(),
            None,
            "no x-ms-substatus header should remain None, not Some(0). Got: {status:?}"
        );
        let diag = err.diagnostics().expect(
            "Wire-attached diagnostics must be present once the metadata fetch is enveloped",
        );
        assert_eq!(
            diag.requests().len(),
            1,
            "single bootstrap request must produce exactly one request record. Got: {diag:?}"
        );
        let req = &diag.requests()[0];
        assert_eq!(
            u16::from(req.status().status_code()),
            503,
            "request diagnostics must echo the upstream HTTP 503. Got: {req:?}"
        );
        assert!(
            req.endpoint().contains("test.documents.azure.com"),
            "request diagnostics must record the regional endpoint contacted. Got: {req:?}"
        );
        assert!(
            err.response().is_some(),
            "with_response_parts + with_diagnostics must promote the error to Wire, exposing response(). Got: {err:?}"
        );
    }

    // Coverage for the other non-2xx shapes the status-gating fix must handle: AAD 401
    // envelopes, plain-text proxy bodies, empty bodies, oversize bodies, and 2xx schema mismatches.

    /// Minimal `TransportClient` returning one canned `(status, body)`.
    /// Lets each coverage-gap test declare its exact wire response without growing `ResponsePlan`.
    #[derive(Debug)]
    struct RawResponseClient {
        status: u16,
        body: Vec<u8>,
    }

    #[async_trait]
    impl TransportClient for RawResponseClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            Ok(HttpResponse {
                status: self.status,
                headers: Headers::new(),
                body: self.body.clone(),
            })
        }
    }

    fn raw_response_client(status: u16, body: Vec<u8>) -> Arc<dyn TransportClient> {
        let client: Box<dyn TransportClient> = Box::new(RawResponseClient { status, body });
        Arc::from(client)
    }

    #[derive(Debug)]
    struct SequenceClient {
        plans: Mutex<VecDeque<ResponsePlan>>,
    }

    impl SequenceClient {
        fn new(plans: impl IntoIterator<Item = ResponsePlan>) -> Self {
            Self {
                plans: Mutex::new(plans.into_iter().collect()),
            }
        }
    }

    #[async_trait]
    impl TransportClient for SequenceClient {
        async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            let plan = self
                .plans
                .lock()
                .expect("plan lock poisoned")
                .pop_front()
                .unwrap_or(ResponsePlan::Success);

            ScriptedClient { plan }.send(request).await
        }
    }

    fn sequence_client(plans: impl IntoIterator<Item = ResponsePlan>) -> Arc<dyn TransportClient> {
        let client: Box<dyn TransportClient> = Box::new(SequenceClient::new(plans));
        Arc::from(client)
    }

    async fn drive_fetch_with(
        status: u16,
        body: Vec<u8>,
    ) -> std::result::Result<crate::driver::cache::AccountProperties, crate::error::CosmosError>
    {
        let client = raw_response_client(status, body);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
    }

    #[tokio::test]
    async fn fetch_account_properties_retries_not_sent_connectivity_failures() {
        let client = sequence_client([
            ResponsePlan::ConnectionError,
            ResponsePlan::ConnectionError,
            ResponsePlan::Success,
        ]);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let properties = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
        .expect("not-sent connectivity failures should be retried");

        assert_eq!(properties.write_region().unwrap().as_str(), "westus2");
    }

    #[tokio::test]
    async fn fetch_account_properties_records_retried_connectivity_attempts() {
        let client = sequence_client([
            ResponsePlan::ConnectionError,
            ResponsePlan::ConnectionError,
            ResponsePlan::ConnectionError,
        ]);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let error = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
        .expect_err("connectivity failures should surface after retry budget exhaustion");

        let diagnostics = error.diagnostics().expect("error should carry diagnostics");
        let requests = diagnostics.requests();
        assert_eq!(
            requests.len(),
            3,
            "initial attempt plus two retries should each have diagnostics: {diagnostics:?}"
        );
        assert_eq!(requests[0].execution_context(), ExecutionContext::Initial);
        assert_eq!(
            requests[1].execution_context(),
            ExecutionContext::TransportRetry
        );
        assert_eq!(
            requests[2].execution_context(),
            ExecutionContext::TransportRetry
        );
        assert!(requests.iter().all(|request| {
            request.request_sent() == crate::diagnostics::RequestSentStatus::NotSent
        }));
    }

    #[tokio::test]
    async fn fetch_account_properties_does_not_retry_wire_failures() {
        let err = drive_fetch_with(
            503,
            br#"{"code":"ServiceUnavailable","message":"server busy"}"#.to_vec(),
        )
        .await
        .expect_err("wire 503 should not use connectivity retries");

        let diagnostics = err.diagnostics().expect("error should carry diagnostics");
        assert_eq!(diagnostics.requests().len(), 1);
        assert_eq!(u16::from(err.status().status_code()), 503);
    }

    /// `TransportClient` whose `send` always fails at the connection level
    /// (no wire response), modeling a firewall-blocked / unreachable endpoint.
    #[derive(Debug)]
    struct UnreachableClient;

    #[async_trait]
    impl TransportClient for UnreachableClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            Err(TransportError::new(
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::TRANSPORT_GENERATED_503)
                    .with_message("injected connection failure")
                    .build(),
                RequestSentStatus::NotSent,
            ))
        }
    }

    fn unreachable_client() -> Arc<dyn TransportClient> {
        let client: Box<dyn TransportClient> = Box::new(UnreachableClient);
        Arc::from(client)
    }

    async fn drive_probe_with(status: u16) -> bool {
        let client = raw_response_client(status, Vec::new());
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        probe_endpoint_connectivity(&transport, &account, &user_agent).await
    }

    async fn drive_probe_unreachable() -> bool {
        let client = unreachable_client();
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        probe_endpoint_connectivity(&transport, &account, &user_agent).await
    }

    /// The endpoint probe gates failback on *connectivity*, not on the account
    /// read succeeding. Any wire response — including a non-2xx envelope
    /// (401/403/429/503) — proves the endpoint is reachable and should fail
    /// back, while a connection-level failure with no response keeps it out of
    /// rotation. Regression guard for issue #4597.
    #[tokio::test]
    async fn probe_treats_wire_response_as_reachable_and_transport_error_as_unreachable() {
        // Happy path: a 2xx `/probe` response is reachable.
        assert!(
            drive_probe_with(200).await,
            "a 2xx /probe response must classify as reachable"
        );

        // Non-2xx wire responses still prove connectivity → reachable.
        for status in [401u16, 403, 429, 503] {
            assert!(
                drive_probe_with(status).await,
                "HTTP {status} is a wire response and must classify as reachable \
                 (failback), not unreachable"
            );
        }

        // A connection-level failure (no wire response) → unreachable.
        assert!(
            !drive_probe_unreachable().await,
            "a transport error with no wire response must classify as unreachable"
        );
    }

    /// AAD 401 envelope on GET / (RBAC race / token expiry / IMDS hiccup) must surface
    /// upstream HTTP 401, not the synthetic `SERIALIZATION_RESPONSE_BODY_INVALID`.
    #[tokio::test]
    async fn fetch_account_properties_surfaces_aad_401_envelope() {
        let body =
            br#"{"code":"Unauthorized","message":"The input authorization token can't serve the request."}"#
                .to_vec();
        let err = drive_fetch_with(401, body)
            .await
            .expect_err("401 must surface as an error");

        let status = err.status();
        let rendered = format!("{err:?}");

        assert_ne!(
            status,
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "401 AAD envelope must not be relabeled as a serde failure. Got: {rendered}"
        );
        assert_eq!(
            u16::from(status.status_code()),
            401,
            "expected the upstream HTTP 401 to be preserved. Got status={status:?} err={rendered}"
        );
        assert!(
            !rendered.contains("missing field `_self`"),
            "must not leak the internal serde `missing field _self` detail. Got: {rendered}"
        );
    }

    /// Plain-text non-2xx body (proxy / LB / fault injector) must surface upstream HTTP status,
    /// not the opaque "expected value at line 1 column 1" serde error. The body must be
    /// reachable verbatim via `wire_payload()` for upstream-log correlation (the message
    /// itself no longer embeds it now that diagnostics are wired in).
    #[tokio::test]
    async fn fetch_account_properties_surfaces_plain_text_non_2xx_body() {
        let err = drive_fetch_with(502, b"Bad Gateway - injected upstream proxy fault".to_vec())
            .await
            .expect_err("502 must surface as an error");

        let status = err.status();
        let rendered = format!("{err:?}");

        assert_ne!(
            status,
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "plain-text non-2xx body must not be relabeled as a serde failure. Got: {rendered}"
        );
        assert_eq!(
            u16::from(status.status_code()),
            502,
            "expected upstream HTTP 502 to be preserved. Got status={status:?} err={rendered}"
        );
        let payload = err
            .wire_payload()
            .expect("non-2xx must attach the upstream wire payload for correlation");
        let body_text = match payload.body() {
            crate::models::ResponseBody::Bytes(b) => std::str::from_utf8(b).unwrap_or_default(),
            _ => "",
        };
        assert!(
            body_text.contains("Bad Gateway"),
            "wire_payload() must preserve the upstream body verbatim. Got: {body_text}"
        );
    }

    /// Empty body on a non-2xx (some intermediaries strip bodies entirely) must not panic
    /// and must still surface the upstream HTTP status.
    #[tokio::test]
    async fn fetch_account_properties_surfaces_empty_non_2xx_body() {
        let err = drive_fetch_with(503, Vec::new())
            .await
            .expect_err("503 with empty body must still surface as an error");

        let status = err.status();
        let rendered = format!("{err:?}");

        assert_ne!(
            status,
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "empty non-2xx body must not be relabeled as a serde failure. Got: {rendered}"
        );
        assert_eq!(
            u16::from(status.status_code()),
            503,
            "expected upstream HTTP 503 to be preserved. Got status={status:?} err={rendered}"
        );
    }

    /// Oversize bodies must be preserved verbatim via `wire_payload()` (no in-message
    /// excerpting / truncation any more). Diagnostics still bounds log volume because
    /// per-request entries don't carry the body — only headers, status, and timing.
    #[tokio::test]
    async fn fetch_account_properties_preserves_large_non_2xx_body_via_wire_payload() {
        let mut body = vec![b'A'; 600];
        body.extend_from_slice(b"TAIL_SENTINEL");
        let err = drive_fetch_with(500, body.clone())
            .await
            .expect_err("500 must surface as an error");

        let rendered = format!("{err}");
        assert!(
            !rendered.contains("…[truncated]"),
            "error message must no longer embed a body excerpt or truncation marker. Got: {rendered}"
        );
        let payload = err
            .wire_payload()
            .expect("non-2xx must attach the wire payload");
        let body_bytes: &[u8] = match payload.body() {
            crate::models::ResponseBody::Bytes(b) => b.as_ref(),
            _ => &[],
        };
        assert_eq!(
            body_bytes.len(),
            body.len(),
            "wire_payload() must preserve the full upstream body verbatim"
        );
        assert!(
            body_bytes.ends_with(b"TAIL_SENTINEL"),
            "wire_payload() must not truncate the tail of the body"
        );
        assert_eq!(
            u16::from(err.status().status_code()),
            500,
            "upstream HTTP 500 must still be preserved alongside the body"
        );
    }

    /// Non-UTF-8-safe truncation used to require byte-then-lossy conversion; with diagnostics
    /// wired in, the body is no longer rendered into the message at all, so multi-byte
    /// codepoints simply round-trip through `wire_payload()`.
    #[tokio::test]
    async fn fetch_account_properties_handles_non_ascii_body_without_panicking() {
        let mut body = vec![b'A'; 511];
        body.extend_from_slice("é".as_bytes());
        body.extend_from_slice(b"tail");

        let err = drive_fetch_with(500, body.clone())
            .await
            .expect_err("500 must surface as an error");

        let payload = err
            .wire_payload()
            .expect("non-2xx must attach the wire payload");
        let body_bytes: &[u8] = match payload.body() {
            crate::models::ResponseBody::Bytes(b) => b.as_ref(),
            _ => &[],
        };
        assert_eq!(
            body_bytes,
            body.as_slice(),
            "multi-byte codepoints must round-trip through wire_payload() unchanged"
        );
    }

    /// 2xx with valid JSON but wrong shape must still surface as `SERIALIZATION_RESPONSE_BODY_INVALID`
    /// — the status-gating fix must not swallow legitimate schema mismatches. Diagnostics are
    /// still attached so the call site is debuggable.
    #[tokio::test]
    async fn fetch_account_properties_2xx_invalid_body_still_reports_serialization_error() {
        let err = drive_fetch_with(200, br#"{"unexpected":"shape"}"#.to_vec())
            .await
            .expect_err("2xx with non-AccountProperties body must still error");

        assert_eq!(
            err.status(),
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "2xx parse failures must continue to be classified as \
             SERIALIZATION_RESPONSE_BODY_INVALID (the status-gating fix only \
             changes the non-2xx branch). Got: {err:?}"
        );
        assert!(
            err.wire_payload().is_some(),
            "parse-failure branch must still attach CosmosResponseHeaders / payload. Got: {err:?}"
        );
        assert!(
            err.diagnostics().is_some(),
            "parse-failure branch must also carry diagnostics now that the bootstrap fetch is enveloped. Got: {err:?}"
        );
        let diagnostics = err.diagnostics().expect("diagnostics attached above");
        assert_eq!(
            diagnostics.status(),
            Some(&crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID),
            "operation_status must reflect the synthetic serialization status, not the wire 200. \
             Otherwise diagnostics consumers see an HTTP 200 alongside a parse error. Got: {:?}",
            diagnostics.status()
        );
    }

    /// 3xx (redirect) must surface as a non-success error with the wire status preserved,
    /// not silently retried or relabeled. The bootstrap transport does NOT follow redirects
    /// — that responsibility belongs to the transport layer when explicitly configured, not
    /// to the off-pipeline metadata fetch. A 3xx therefore means "the gateway is telling us
    /// to go elsewhere and we can't honor that here," which must be visible to the caller.
    #[tokio::test]
    async fn fetch_account_properties_surfaces_3xx_as_non_success_with_wire_payload() {
        let body = br#"<html><body>Moved</body></html>"#.to_vec();
        let err = drive_fetch_with(307, body.clone())
            .await
            .expect_err("3xx must surface as an error — the bootstrap fetch must not parse a redirect body as AccountProperties");

        assert_ne!(
            err.status(),
            crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
            "3xx must NOT be reclassified as a deserialization failure (the parse must be skipped). Got: {err:?}"
        );
        assert_eq!(
            u16::from(err.status().status_code()),
            307,
            "the surfaced error must reflect the upstream redirect status. Got: {err:?}"
        );
        let payload = err
            .wire_payload()
            .expect("3xx must attach the wire payload so callers can inspect the redirect body");
        match payload.body() {
            crate::models::ResponseBody::Bytes(b) => {
                assert_eq!(
                    b.as_ref(),
                    body.as_slice(),
                    "redirect body must round-trip through wire_payload() unchanged"
                );
            }
            other => panic!("expected Bytes payload, got: {other:?}"),
        }
        assert!(
            err.diagnostics().is_some(),
            "3xx must also carry diagnostics, matching every other status-error path. Got: {err:?}"
        );
    }

    /// Transport-layer failure (e.g. connection refused, TLS handshake error) must produce
    /// a `CosmosError` with diagnostics attached. The request is marked `Sent` because
    /// `transport.send` returned an error from the wire side — the request reached the
    /// network stack but the transport layer rejected it. Without this, network failures
    /// during the 5-min background refresh would lose the diagnostics envelope and become
    /// unattributable text strings.
    #[tokio::test]
    async fn fetch_account_properties_transport_error_produces_diagnostics() {
        #[derive(Debug)]
        struct FailingTransportClient;

        #[async_trait]
        impl TransportClient for FailingTransportClient {
            async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
                // Synthesize a transport-layer failure: the wire never produced an HTTP
                // status, only an azure_core / network-style error.
                let err = crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::TRANSPORT_CONNECTION_FAILED)
                    .with_message("connection refused")
                    .build();
                Err(TransportError::new(
                    err,
                    crate::diagnostics::RequestSentStatus::Sent,
                ))
            }
        }

        let client: Box<dyn TransportClient> = Box::new(FailingTransportClient);
        let client = Arc::from(client);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account("https://test.documents.azure.com:443/");
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let err = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
        .expect_err("transport-layer failure must surface as an error");

        let diag = err.diagnostics().expect(
            "transport-error path must attach diagnostics so off-pipeline failures stay debuggable",
        );
        assert_eq!(
            diag.requests().len(),
            1,
            "single bootstrap request must produce exactly one request record. Got: {diag:?}"
        );
        let req = &diag.requests()[0];
        assert!(
            req.endpoint().contains("test.documents.azure.com"),
            "request diagnostics must record the endpoint contacted. Got: {req:?}"
        );
        assert_eq!(
            req.request_sent(),
            crate::diagnostics::RequestSentStatus::Sent,
            "transport.send returned an error after invocation; the request reached the wire side. Got: {req:?}"
        );
    }

    /// Sign-request failure (e.g. broken TokenCredential, IMDS unreachable) must produce
    /// a `CosmosError` with diagnostics attached and the request marked `NotSent`. The
    /// sign step runs before `transport.send`, so the request never reached the wire.
    /// Without this, AAD/MSI failures during the off-pipeline bootstrap fetch would lose
    /// the diagnostics envelope entirely.
    #[tokio::test]
    async fn fetch_account_properties_sign_failure_produces_diagnostics_not_sent() {
        use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};

        #[derive(Debug)]
        struct BrokenCredential;

        #[async_trait]
        impl TokenCredential for BrokenCredential {
            async fn get_token(
                &self,
                _scopes: &[&str],
                _options: Option<TokenRequestOptions<'_>>,
            ) -> azure_core::Result<AccessToken> {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Credential,
                    "broken credential",
                ))
            }
        }

        // Use a transport that would succeed if we ever got there, so a failed assertion
        // produces an obviously-wrong shape rather than a confused network-error message.
        let client = scripted_client(ResponsePlan::Success);
        let transport =
            crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(client);

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let credential: Box<dyn TokenCredential> = Box::new(BrokenCredential);
        let account = AccountReference::with_credential(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            Arc::from(credential),
        );
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let err = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await
        .expect_err("sign_request failure must surface as an error");

        let diag = err.diagnostics().expect(
            "sign-failure path must attach diagnostics so credential/IMDS failures stay debuggable",
        );
        assert_eq!(
            diag.requests().len(),
            1,
            "single bootstrap request entry must exist even when sign fails. Got: {diag:?}"
        );
        let req = &diag.requests()[0];
        assert_eq!(
            req.request_sent(),
            crate::diagnostics::RequestSentStatus::NotSent,
            "sign_request runs before transport.send; the request must be recorded as NotSent. Got: {req:?}"
        );
        assert!(
            req.endpoint().contains("test.documents.azure.com"),
            "request diagnostics must record the endpoint that would have been contacted. Got: {req:?}"
        );
    }

    /// End-to-end proof that the production reqwest transport built by
    /// `DefaultHttpClientFactory` actually follows 3xx redirects on the wire,
    /// which is what the inline comment in `fetch_account_properties_with_transport`
    /// promises. Without this test, a future change that flipped the redirect
    /// policy to `Policy::none()` (or any other regression that stopped
    /// following) would silently rebrand every endpoint fronted by a redirecting
    /// proxy (custom Front Door / proxy returning 307/308) as a `CosmosError` 307,
    /// even though no real client wants that behavior.
    ///
    /// Spins up a localhost HTTP/1.1 server that returns `307 Temporary
    /// Redirect` with `Location: <self>/follow` on the first request and
    /// the canonical `AccountProperties` JSON on the second, then drives a
    /// real `ReqwestTransportClient` through `fetch_account_properties_with_transport`
    /// and asserts the call succeeds with the JSON-derived `id`.
    #[tokio::test]
    async fn bootstrap_transport_follows_3xx_redirects_against_real_server() {
        use std::sync::atomic::{AtomicU32, Ordering as AtomicOrdering};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let port = addr.port();
        let request_count = Arc::new(AtomicU32::new(0));

        let counter = Arc::clone(&request_count);
        let server = tokio::spawn(async move {
            for _ in 0..2 {
                let Ok((mut socket, _peer)) = listener.accept().await else {
                    return;
                };
                let mut buf = [0u8; 8192];
                let mut read = 0;
                // Read headers until CRLF CRLF. The bootstrap GET has no body so we
                // don't need Content-Length parsing here.
                loop {
                    let n = match socket.read(&mut buf[read..]).await {
                        Ok(0) | Err(_) => break,
                        Ok(n) => n,
                    };
                    read += n;
                    if buf[..read].windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                    if read == buf.len() {
                        break;
                    }
                }
                let request = std::str::from_utf8(&buf[..read]).unwrap_or("");
                let request_line = request.lines().next().unwrap_or("");
                let n = counter.fetch_add(1, AtomicOrdering::SeqCst);

                let response = if n == 0 {
                    assert!(
                        request_line.starts_with("GET / "),
                        "first request must hit the root path; got: {request_line:?}"
                    );
                    format!(
                        "HTTP/1.1 307 Temporary Redirect\r\n\
                         Location: http://127.0.0.1:{port}/follow\r\n\
                         Content-Length: 5\r\n\
                         Connection: close\r\n\
                         \r\n\
                         MOVED"
                    )
                } else {
                    assert!(
                        request_line.starts_with("GET /follow "),
                        "redirected request must hit /follow; got: {request_line:?}"
                    );
                    format!(
                        "HTTP/1.1 200 OK\r\n\
                         Content-Type: application/json\r\n\
                         Content-Length: {}\r\n\
                         Connection: close\r\n\
                         \r\n\
                         {}",
                        ACCOUNT_PROPERTIES_PAYLOAD.len(),
                        ACCOUNT_PROPERTIES_PAYLOAD,
                    )
                };
                let _ = socket.write_all(response.as_bytes()).await;
                let _ = socket.shutdown().await;
            }
        });

        let pool = ConnectionPoolOptions::default();
        let config = HttpClientConfig {
            version_policy: HttpVersionPolicy::Http11Only,
            request_timeout: std::time::Duration::from_secs(5),
            allow_invalid_cert: false,
            http2_keep_alive_while_idle: false,
            transport_kind: None,
        };
        let transport_client =
            crate::driver::transport::http_client_factory::DefaultHttpClientFactory::new()
                .build(&pool, config)
                .expect("DefaultHttpClientFactory must build a real reqwest-backed transport");
        let transport = crate::driver::transport::adaptive_transport::AdaptiveTransport::Gateway(
            transport_client,
        );

        let runtime = CosmosDriverRuntimeBuilder::new().build().await.unwrap();
        let account = signed_test_account(&format!("http://127.0.0.1:{port}/"));
        let user_agent = azure_core::http::headers::HeaderValue::from("cosmos-driver-test/0.0.0");

        let result = CosmosDriver::fetch_account_properties_with_transport(
            &runtime,
            &transport,
            &account,
            None,
            &user_agent,
            false,
        )
        .await;

        // Ensure the server task has fully drained so the assertion message can
        // report the final hop count (especially if the bootstrap fetch failed).
        let _ = server.await;

        let final_count = request_count.load(AtomicOrdering::SeqCst);
        let props = result.unwrap_or_else(|err| panic!(
            "bootstrap fetch must succeed against a redirecting proxy that returns 307 -> 200 JSON; \
             this proves the reqwest transport follows redirects. saw {final_count} request(s). err: {err:?}"
        ));
        assert_eq!(
            props.id, "test",
            "fetched AccountProperties must come from the /follow hop, proving the transport followed the 307"
        );
        assert_eq!(
            final_count, 2,
            "transport must have made exactly 2 wire requests (initial + one redirect follow). Got: {final_count}"
        );
    }

    #[tokio::test]
    async fn drivers_without_user_agent_suffix_share_runtime_arc() {
        // Two drivers built from the same runtime with no per-driver
        // suffix override must share the runtime's `Arc<UserAgent>`
        // (no per-driver allocation, just an atomic refcount bump).
        let factory = Arc::new(ScriptedFactory::new(std::iter::repeat_n(
            ResponsePlan::Success,
            10,
        )));
        let runtime = Arc::new(
            CosmosDriverRuntimeBuilder::new()
                .with_http_client_factory(factory)
                .build()
                .await
                .unwrap(),
        );

        let driver_a = CosmosDriver::new(
            Arc::clone(&runtime),
            DriverOptionsBuilder::new(signed_test_account(
                "https://account-a.documents.azure.com:443/",
            ))
            .build(),
        )
        .expect("CosmosDriver::new should succeed in tests");
        let driver_b = CosmosDriver::new(
            Arc::clone(&runtime),
            DriverOptionsBuilder::new(signed_test_account(
                "https://account-b.documents.azure.com:443/",
            ))
            .build(),
        )
        .expect("CosmosDriver::new should succeed in tests");

        assert!(
            Arc::ptr_eq(driver_a.user_agent(), runtime.user_agent()),
            "driver A must share the runtime's User-Agent Arc"
        );
        assert!(
            Arc::ptr_eq(driver_b.user_agent(), runtime.user_agent()),
            "driver B must share the runtime's User-Agent Arc"
        );
        assert!(
            Arc::ptr_eq(driver_a.user_agent(), driver_b.user_agent()),
            "drivers A and B must share the same User-Agent Arc"
        );
    }

    #[tokio::test]
    async fn driver_user_agent_suffix_override_owns_distinct_arc() {
        // A driver built with a per-driver suffix must compute its own
        // `UserAgent` (distinct allocation; suffix actually appears).
        let factory = Arc::new(ScriptedFactory::new(std::iter::repeat_n(
            ResponsePlan::Success,
            4,
        )));
        let runtime = Arc::new(
            CosmosDriverRuntimeBuilder::new()
                .with_http_client_factory(factory)
                .with_user_agent_suffix(UserAgentSuffix::new("runtime-default"))
                .build()
                .await
                .unwrap(),
        );

        let driver = CosmosDriver::new(
            Arc::clone(&runtime),
            DriverOptionsBuilder::new(signed_test_account(
                "https://account.documents.azure.com:443/",
            ))
            .with_user_agent_suffix(UserAgentSuffix::new("driver-override"))
            .build(),
        )
        .expect("CosmosDriver::new should succeed in tests");

        assert!(
            !Arc::ptr_eq(driver.user_agent(), runtime.user_agent()),
            "driver with override must NOT share the runtime's User-Agent Arc"
        );
        assert!(driver.user_agent().as_str().contains("driver-override"));
        assert!(runtime.user_agent().as_str().contains("runtime-default"));
        assert!(!driver.user_agent().as_str().contains("runtime-default"));
    }

    #[tokio::test]
    async fn driver_disabling_ppcb_recomputes_user_agent_without_ppcb_bit() {
        // A driver that disables PPCB (with no per-driver suffix override) must
        // NOT share the runtime's base `Arc<UserAgent>`: its feature flags
        // differ from the runtime's, so it recomputes its own `UserAgent` whose
        // cross-SDK feature token drops the PPCB bit (0x2) while retaining
        // HTTP/2 (0x10) -> `|F10`. This exercises the `None => recompute` branch
        // in `CosmosDriver::new` and proves the emitted token tracks per-driver
        // client configuration rather than a hardcoded value.
        let factory = Arc::new(ScriptedFactory::new(std::iter::repeat_n(
            ResponsePlan::Success,
            10,
        )));
        let runtime = Arc::new(
            CosmosDriverRuntimeBuilder::new()
                .with_http_client_factory(factory)
                .build()
                .await
                .unwrap(),
        );

        // The runtime's base header advertises HTTP/2 + PPCB by default (|F12).
        assert_eq!(
            runtime.user_agent_feature_flags(),
            UserAgentFeatureFlags::HTTP2 | UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER,
        );
        assert!(
            runtime.user_agent().as_str().ends_with("|F12"),
            "unexpected runtime User-Agent: {}",
            runtime.user_agent().as_str()
        );

        let driver = CosmosDriver::new(
            Arc::clone(&runtime),
            DriverOptionsBuilder::new(signed_test_account(
                "https://account.documents.azure.com:443/",
            ))
            .with_partition_failover_options(
                crate::options::PartitionFailoverOptions::builder()
                    .with_circuit_breaker_enabled(false)
                    .build()
                    .unwrap(),
            )
            .build(),
        )
        .expect("CosmosDriver::new should succeed in tests");

        // Distinct allocation (the recompute branch), not the shared runtime Arc.
        assert!(
            !Arc::ptr_eq(driver.user_agent(), runtime.user_agent()),
            "driver disabling PPCB must own a distinct User-Agent Arc"
        );
        // PPCB bit (0x2) dropped, HTTP/2 (0x10) retained -> |F10.
        assert!(
            driver.user_agent().as_str().ends_with("|F10"),
            "expected driver User-Agent to drop the PPCB bit (|F10): {}",
            driver.user_agent().as_str()
        );
    }

    // =========================================================================
    // pre_resolve_partition_key_range_id — EPK-range seeding (#4611 fix)
    //
    // The single→Some / multi→(warn + debug_assert, then None) classification
    // lives in `ContainerRoutingMap::single_overlapping_range_id` (unit-tested
    // there). These cache-backed tests drive the *real*
    // `PartitionKeyRangeCache::resolve_single_overlapping_range_id` (mocked
    // fetch, per the existing `resolve_overlapping_ranges_*` tests), exercising
    // the exact path `pre_resolve_partition_key_range_id` takes.
    // =========================================================================

    /// Builds a `ContainerReference` from a partition-key-definition JSON blob.
    fn epk_test_container(pk_json: &str) -> ContainerReference {
        let container_props = crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: serde_json::from_str(pk_json).unwrap(),
            system_properties: Default::default(),
        };
        ContainerReference::new(
            signed_test_account("https://test.documents.azure.com:443/"),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &container_props,
        )
    }

    /// Single-page fetch returning one range that owns the whole EPK space.
    async fn whole_space_single_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> Option<crate::driver::cache::PkRangeFetchResult> {
        use crate::models::partition_key_range::PartitionKeyRange as PkRange;
        if continuation.is_some() {
            Some(crate::driver::cache::PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(crate::driver::cache::PkRangeFetchResult {
                ranges: vec![PkRange::new("0".into(), "", "FF")],
                continuation: Some("etag".to_string()),
                not_modified: false,
            })
        }
    }

    /// Single-page fetch returning two ranges split at "80".
    async fn whole_space_two_range_fetch(
        _container: ContainerReference,
        continuation: Option<String>,
    ) -> Option<crate::driver::cache::PkRangeFetchResult> {
        use crate::models::partition_key_range::PartitionKeyRange as PkRange;
        if continuation.is_some() {
            Some(crate::driver::cache::PkRangeFetchResult {
                ranges: vec![],
                continuation,
                not_modified: true,
            })
        } else {
            Some(crate::driver::cache::PkRangeFetchResult {
                ranges: vec![
                    PkRange::new("0".into(), "", "80"),
                    PkRange::new("1".into(), "80", "FF"),
                ],
                continuation: Some("etag".to_string()),
                not_modified: false,
            })
        }
    }

    #[tokio::test]
    async fn epk_range_owned_by_single_partition_resolves_to_that_range() {
        use crate::driver::cache::PartitionKeyRangeCache;
        use crate::models::effective_partition_key::EffectivePartitionKey;

        let container = epk_test_container(r#"{"paths":["/pk"],"version":2}"#);
        let cache = PartitionKeyRangeCache::new();

        // An EPK-range feed range spanning the whole space, resolved against a
        // container with a single physical partition, is owned by exactly one
        // range — so pre-resolution seeds that range's ID (single → Some).
        let resolved = cache
            .resolve_single_overlapping_range_id(
                &container,
                &EffectivePartitionKey::MIN..&EffectivePartitionKey::MAX,
                false,
                whole_space_single_range_fetch,
            )
            .await
            .map(PartitionKeyRangeId::from);

        assert_eq!(resolved.as_ref().map(|id| id.as_str()), Some("0"));
    }

    #[tokio::test]
    #[should_panic(expected = "physical partitions")]
    async fn epk_range_spanning_multiple_partitions_panics_in_debug() {
        use crate::driver::cache::PartitionKeyRangeCache;
        use crate::models::effective_partition_key::EffectivePartitionKey;

        let container = epk_test_container(r#"{"paths":["/pk"],"version":2}"#);
        let cache = PartitionKeyRangeCache::new();

        // A whole-space feed range overlapping both physical partitions is an
        // invariant violation at this layer (the dataflow pipeline should have
        // split it first), so single-owner resolution trips the `debug_assert!`.
        // In release builds it returns `None` and the caller degrades gracefully.
        let _ = cache
            .resolve_single_overlapping_range_id(
                &container,
                &EffectivePartitionKey::MIN..&EffectivePartitionKey::MAX,
                false,
                whole_space_two_range_fetch,
            )
            .await;
    }

    #[tokio::test]
    async fn logical_partition_key_resolves_to_owning_range_unchanged() {
        use crate::driver::cache::PartitionKeyRangeCache;

        // The logical-partition-key path is unchanged by the EPK-range fix: a
        // concrete partition key still resolves through the point-lookup path to
        // exactly its owning physical partition's ID (mapped to a
        // `PartitionKeyRangeId`, as `pre_resolve_partition_key_range_id` does).
        let container = epk_test_container(r#"{"paths":["/pk"],"version":2}"#);
        let cache = PartitionKeyRangeCache::new();
        let pk = PartitionKey::from("hello");

        let resolved = cache
            .resolve_partition_key_range_id(&container, &pk, false, whole_space_two_range_fetch)
            .await
            .map(PartitionKeyRangeId::from);

        // "hello" hashes into one of the two ranges — exactly one, never both.
        let id = resolved.expect("logical PK resolves to its owning range");
        assert!(
            id.as_str() == "0" || id.as_str() == "1",
            "logical PK must resolve to a single owning range, got {id}",
        );
    }
}
