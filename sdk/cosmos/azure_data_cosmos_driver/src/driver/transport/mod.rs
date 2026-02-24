// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP transport layer for Cosmos DB driver.
//!
//! This module provides connection pooling and transport management for HTTP
//! requests to Azure Cosmos DB. It maintains separate connection pools for:
//!
//! - **Metadata operations**: REST/JSON requests for account, database, and
//!   container management. Uses HTTP/2 when allowed.
//! - **Data plane operations**: Point read/write operations and queries.
//!   Uses HTTP/2 when allowed.
//! - **Emulator operations**: Lazily-initialized pools for local emulator
//!   with certificate validation disabled.

mod authorization_policy;
mod emulator;
mod headers_policy;
mod pipeline;
mod tracked_transport;

use crate::{
    models::{AccountEndpoint, Credential, OperationType, ResourceType},
    options::ConnectionPoolOptions,
};
use authorization_policy::AuthorizationPolicy;
use azure_core::http::{policies::Policy, Transport};
use headers_policy::CosmosHeadersPolicy;
use pipeline::CosmosPipeline;
use std::sync::{Arc, OnceLock};

pub(crate) use authorization_policy::AuthorizationContext;
pub(crate) use emulator::is_emulator_host;
pub(crate) use tracked_transport::{
    infer_request_sent_status, RequestAttemptTelemetryContext, RequestAttemptTelemetrySink,
    RequestSentStatus,
};

/// Determines whether the dataplane pipeline should be used for a given operation.
///
/// The dataplane pipeline is optimized for document operations and stored procedure
/// execution. All other operations use the metadata pipeline.
///
/// # Returns
///
/// Returns `true` for:
/// - Any operation on `ResourceType::Document`
/// - `OperationType::Execute` on `ResourceType::StoredProcedure`
///
/// Returns `false` for all other combinations.
pub(crate) fn uses_dataplane_pipeline(
    resource_type: ResourceType,
    operation_type: OperationType,
) -> bool {
    match resource_type {
        ResourceType::Document => true,
        ResourceType::StoredProcedure => matches!(operation_type, OperationType::Execute),
        _ => false,
    }
}

/// HTTP transport manager for Cosmos DB connections.
///
/// Manages connection pools with separate settings for metadata and data plane
/// operations. Supports both production endpoints and local emulator with
/// lazy initialization of emulator-specific pools.
///
/// # Connection Pools
///
/// - **Metadata pool**: For REST/JSON operations (account/database/container
///   management). Prefers HTTP/2 multiplexing when enabled.
/// - **Data plane pool**: For point operations and queries. Will support RNTBD
///   envelope encapsulation in future versions.
/// - **Emulator pools**: Lazily created when connecting to emulator hosts with
///   certificate validation disabled.
///
/// # Custom Pipeline
///
/// This transport uses a custom [`CosmosPipeline`] that does not include any
/// default azure_core policies (no automatic retry, logging, or telemetry).
/// The Cosmos driver has full control over request processing.
///
/// # Thread Safety
///
/// All pools are thread-safe and can be accessed concurrently. The transport
/// is designed to be shared across all drivers in a runtime.
#[derive(Debug)]
pub(crate) struct CosmosTransport {
    /// Connection pool configuration.
    connection_pool: ConnectionPoolOptions,

    /// Headers policy for setting Cosmos-specific headers.
    headers_policy: Arc<CosmosHeadersPolicy>,

    /// Unauthenticated pipeline for metadata operations (REST/JSON).
    /// Used as a base for creating authenticated pipelines per-driver.
    metadata_transport: Transport,

    /// Unauthenticated pipeline for data plane operations.
    /// Used as a base for creating authenticated pipelines per-driver.
    dataplane_transport: Transport,

    /// Lazily-initialized transport for emulator metadata operations.
    /// Uses insecure TLS that accepts invalid/self-signed certificates.
    insecure_emulator_metadata_transport: OnceLock<Transport>,

    /// Lazily-initialized transport for emulator data plane operations.
    /// Uses insecure TLS that accepts invalid/self-signed certificates.
    insecure_emulator_dataplane_transport: OnceLock<Transport>,
}

impl CosmosTransport {
    /// Creates a new transport with the given connection pool configuration.
    ///
    /// # Arguments
    ///
    /// * `connection_pool` - Connection pool settings for HTTP clients
    /// * `user_agent` - User agent string to use for all requests
    pub(crate) fn new(
        connection_pool: ConnectionPoolOptions,
        user_agent: impl Into<String>,
    ) -> azure_core::Result<Self> {
        let headers_policy = Arc::new(CosmosHeadersPolicy::new(user_agent));

        let metadata_client = Self::create_reqwest_client(&connection_pool, true, false)?;
        let metadata_transport = Transport::new(Arc::new(metadata_client));

        let dataplane_client = Self::create_reqwest_client(&connection_pool, false, false)?;
        let dataplane_transport = Transport::new(Arc::new(dataplane_client));

        Ok(Self {
            connection_pool,
            headers_policy,
            metadata_transport,
            dataplane_transport,
            insecure_emulator_metadata_transport: OnceLock::new(),
            insecure_emulator_dataplane_transport: OnceLock::new(),
        })
    }

    /// Creates an authenticated pipeline for metadata operations.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The account endpoint to determine emulator vs production transport
    /// * `auth` - Authentication options for signing requests
    pub(crate) fn create_metadata_pipeline(
        &self,
        endpoint: &AccountEndpoint,
        credential: &Credential,
    ) -> CosmosPipeline {
        let transport = self.get_metadata_transport(endpoint);
        self.create_authenticated_pipeline(transport, credential)
    }

    /// Creates an authenticated pipeline for data plane operations.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The account endpoint to determine emulator vs production transport
    /// * `auth` - Authentication options for signing requests
    pub(crate) fn create_dataplane_pipeline(
        &self,
        endpoint: &AccountEndpoint,
        credential: &Credential,
    ) -> CosmosPipeline {
        let transport = self.get_dataplane_transport(endpoint);
        self.create_authenticated_pipeline(transport, credential)
    }

    /// Gets the transport for metadata operations.
    fn get_metadata_transport(&self, endpoint: &AccountEndpoint) -> Transport {
        if self.should_use_insecure_emulator_transport(endpoint) {
            self.insecure_emulator_metadata_transport
                .get_or_init(|| {
                    let client = Self::create_reqwest_client(&self.connection_pool, true, true)
                        .expect("failed to create emulator metadata client");
                    Transport::new(Arc::new(client))
                })
                .clone()
        } else {
            self.metadata_transport.clone()
        }
    }

    /// Gets the transport for data plane operations.
    fn get_dataplane_transport(&self, endpoint: &AccountEndpoint) -> Transport {
        if self.should_use_insecure_emulator_transport(endpoint) {
            self.insecure_emulator_dataplane_transport
                .get_or_init(|| {
                    let client = Self::create_reqwest_client(&self.connection_pool, false, true)
                        .expect("failed to create emulator dataplane client");
                    Transport::new(Arc::new(client))
                })
                .clone()
        } else {
            self.dataplane_transport.clone()
        }
    }

    /// Creates an authenticated pipeline with headers and authorization policies.
    fn create_authenticated_pipeline(
        &self,
        transport: Transport,
        credential: &Credential,
    ) -> CosmosPipeline {
        let auth_policy = Arc::new(AuthorizationPolicy::new(credential));

        let policies: Vec<Arc<dyn Policy>> = vec![
            Arc::clone(&self.headers_policy) as Arc<dyn Policy>,
            auth_policy as Arc<dyn Policy>,
        ];

        CosmosPipeline::new(policies, transport)
    }

    /// Determines if insecure emulator transport should be used for the given endpoint.
    ///
    /// Returns `true` when both conditions are met:
    /// - Emulator server certificate validation is disabled
    /// - The endpoint is a known emulator host (localhost, 127.0.0.1)
    fn should_use_insecure_emulator_transport(&self, endpoint: &AccountEndpoint) -> bool {
        bool::from(self.connection_pool.emulator_server_cert_validation())
            && is_emulator_host(endpoint)
    }

    // TODO @fabianm: allow the caller to provide a client factory instead of hard-coding reqwest.
    /// Creates a reqwest client with the appropriate settings.
    ///
    /// # Arguments
    ///
    /// * `pool` - Connection pool configuration
    /// * `is_metadata` - Whether this is for metadata operations (uses different timeouts)
    /// * `for_emulator` - Whether to disable TLS certificate validation
    fn create_reqwest_client(
        pool: &ConnectionPoolOptions,
        is_metadata: bool,
        for_emulator: bool,
    ) -> azure_core::Result<reqwest::Client> {
        #[cfg(not(target_arch = "wasm32"))]
        let mut builder = reqwest::ClientBuilder::new();

        #[cfg(target_arch = "wasm32")]
        let builder = reqwest::ClientBuilder::new();

        // Native-only settings (not available on WASM)
        // WASM uses browser's fetch API which handles connection pooling,
        // timeouts, and TLS internally.
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Connection pool settings
            builder = builder.pool_max_idle_per_host(pool.max_idle_connections_per_endpoint());

            if let Some(idle_timeout) = pool.idle_connection_timeout() {
                builder = builder.pool_idle_timeout(idle_timeout);
            }

            // Connect timeout
            builder = builder.connect_timeout(pool.max_connect_timeout());

            // Request timeout (different for metadata vs data plane)
            let request_timeout = if is_metadata {
                pool.max_metadata_request_timeout()
            } else {
                pool.max_dataplane_request_timeout()
            };
            builder = builder.timeout(request_timeout);

            // Proxy settings
            if !pool.is_proxy_allowed() {
                builder = builder.no_proxy();
            }
            // When proxy is allowed, reqwest automatically respects HTTP_PROXY/HTTPS_PROXY env vars

            // Local address binding
            if let Some(local_addr) = pool.local_address() {
                builder = builder.local_address(local_addr);
            }

            // Emulator settings - disable TLS validation
            if for_emulator {
                builder = builder.danger_accept_invalid_certs(true);
            }
        }

        // Suppress unused variable warnings on WASM
        #[cfg(target_arch = "wasm32")]
        let _ = (pool, is_metadata, for_emulator);

        builder.build().map_err(|e| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Failed to create HTTP client: {e}"),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{ConnectionPoolOptionsBuilder, EmulatorServerCertValidation};

    #[test]
    fn transport_creates_with_default_options() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::new(pool, "test-user-agent").unwrap();

        // Should not be using emulator transport for regular endpoints
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn transport_detects_emulator_when_disabled() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_emulator_server_cert_validation(EmulatorServerCertValidation::DangerousDisabled)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool, "test-user-agent").unwrap();

        // localhost is an emulator host
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(transport.should_use_insecure_emulator_transport(&endpoint));

        // 127.0.0.1 is an emulator host
        let endpoint = AccountEndpoint::try_from("https://127.0.0.1:8081/").unwrap();
        assert!(transport.should_use_insecure_emulator_transport(&endpoint));

        // Production endpoint is not an emulator host
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn transport_ignores_emulator_hosts_when_validation_enabled() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::new(pool, "test-user-agent").unwrap();

        // Even localhost should not use emulator transport if validation is enabled
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn uses_dataplane_for_document_operations() {
        // Document operations always use dataplane
        assert!(uses_dataplane_pipeline(
            ResourceType::Document,
            OperationType::Read
        ));
        assert!(uses_dataplane_pipeline(
            ResourceType::Document,
            OperationType::Create
        ));
        assert!(uses_dataplane_pipeline(
            ResourceType::Document,
            OperationType::Replace
        ));
        assert!(uses_dataplane_pipeline(
            ResourceType::Document,
            OperationType::Delete
        ));
        assert!(uses_dataplane_pipeline(
            ResourceType::Document,
            OperationType::Upsert
        ));
    }

    #[test]
    fn uses_dataplane_for_stored_procedure_execute() {
        // StoredProcedure Execute uses dataplane
        assert!(uses_dataplane_pipeline(
            ResourceType::StoredProcedure,
            OperationType::Execute
        ));

        // Other StoredProcedure operations use metadata
        assert!(!uses_dataplane_pipeline(
            ResourceType::StoredProcedure,
            OperationType::Read
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::StoredProcedure,
            OperationType::Create
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::StoredProcedure,
            OperationType::Delete
        ));
    }

    #[test]
    fn uses_metadata_for_other_resources() {
        // Database operations use metadata
        assert!(!uses_dataplane_pipeline(
            ResourceType::Database,
            OperationType::Read
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::Database,
            OperationType::Create
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::Database,
            OperationType::Delete
        ));

        // Container operations use metadata
        assert!(!uses_dataplane_pipeline(
            ResourceType::DocumentCollection,
            OperationType::Read
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::DocumentCollection,
            OperationType::Create
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::DocumentCollection,
            OperationType::Delete
        ));

        // Account operations use metadata
        assert!(!uses_dataplane_pipeline(
            ResourceType::DatabaseAccount,
            OperationType::Read
        ));

        // Trigger, UDF use metadata for CRUD
        assert!(!uses_dataplane_pipeline(
            ResourceType::Trigger,
            OperationType::Read
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::UserDefinedFunction,
            OperationType::Create
        ));

        // Offer uses metadata
        assert!(!uses_dataplane_pipeline(
            ResourceType::Offer,
            OperationType::Read
        ));
        assert!(!uses_dataplane_pipeline(
            ResourceType::Offer,
            OperationType::Replace
        ));
    }
}
