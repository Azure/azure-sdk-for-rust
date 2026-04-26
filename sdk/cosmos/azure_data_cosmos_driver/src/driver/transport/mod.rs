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

pub(crate) mod adaptive_transport;
mod authorization_policy;
#[cfg(feature = "tokio")]
pub(crate) mod background_task_manager;
pub(crate) mod cosmos_headers;
pub(crate) mod cosmos_transport_client;
mod emulator;
pub(crate) mod http_client_factory;
pub(crate) mod request_signing;
#[cfg(feature = "reqwest")]
pub(crate) mod reqwest_transport_client;
mod sharded_transport;
pub(crate) use sharded_transport::EndpointKey;
mod tracked_transport;
pub(crate) mod transport_pipeline;

use crate::{
    driver::pipeline::components::TransportMode,
    models::{AccountEndpoint, OperationType, ResourceType},
    options::ConnectionPoolOptions,
};
use std::sync::{Arc, OnceLock};

use self::{
    adaptive_transport::AdaptiveTransport,
    http_client_factory::{HttpClientConfig, HttpClientFactory},
};
use crate::diagnostics::TransportHttpVersion;

#[cfg(test)]
use self::http_client_factory::DefaultHttpClientFactory;

pub(crate) use authorization_policy::generate_authorization;
pub(crate) use authorization_policy::AuthorizationContext;
pub(crate) use emulator::is_emulator_host;
pub(crate) use tracked_transport::infer_request_sent_status;

/// Cosmos DB REST API version.
///
/// This must match the version supported by the service. The value `2020-07-15`
/// is the same as used by the Java SDK for compatibility.
pub(crate) const COSMOS_API_VERSION: &str = "2020-07-15";

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

/// HTTP transport manager for a single Cosmos DB account.
///
/// Each `CosmosDriver` instance (one per account) owns its own transport.
/// The transport type (sharded HTTP/2 vs unsharded HTTP/1.1) is determined
/// by an HTTP/2 probe during initialization and may be updated on periodic
/// metadata refreshes.
///
/// # Transport Selection
///
/// - **HTTP/2 confirmed**: Sharded transport with HTTP/2 PING keepalive.
///   No TCP keepalive needed — HTTP/2 PING frames serve that role.
/// - **HTTP/1.1 fallback**: Unsharded transport with TCP keepalive for
///   connection liveness detection.
/// - **Gateway 2.0**: Always HTTP/2 (unchanged).
/// - **Emulator**: Lazily created with insecure TLS.
#[derive(Debug)]
pub(crate) struct CosmosTransport {
    /// Connection pool configuration.
    connection_pool: ConnectionPoolOptions,

    /// Factory used to create protocol-specific HTTP transports.
    http_client_factory: Arc<dyn HttpClientFactory>,

    /// The detected HTTP version for this account's gateway.
    negotiated_version: TransportHttpVersion,

    /// Transport for metadata operations.
    metadata_transport: AdaptiveTransport,

    /// Transport for dataplane gateway operations.
    dataplane_gateway_transport: AdaptiveTransport,

    /// Lazily-initialized transport for dataplane Gateway 2.0 operations.
    dataplane_gateway20_transport: OnceLock<AdaptiveTransport>,

    /// Lazily-initialized transport for emulator metadata operations.
    insecure_emulator_metadata_transport: OnceLock<AdaptiveTransport>,

    /// Lazily-initialized transport for emulator dataplane operations.
    insecure_emulator_dataplane_transport: OnceLock<AdaptiveTransport>,
}

impl CosmosTransport {
    /// Creates a transport for an account with the given negotiated HTTP version.
    ///
    /// The `negotiated_version` should be determined by probing the gateway
    /// during `CosmosDriver::initialize()`.
    #[cfg(test)]
    pub(crate) fn for_tests(
        connection_pool: ConnectionPoolOptions,
        negotiated_version: TransportHttpVersion,
    ) -> azure_core::Result<Self> {
        let http_client_factory: Arc<dyn HttpClientFactory> =
            Arc::new(DefaultHttpClientFactory::new());

        Self::with_factory(connection_pool, http_client_factory, negotiated_version)
    }

    /// Creates a transport with a custom HTTP client factory (for testing).
    pub(crate) fn with_factory(
        connection_pool: ConnectionPoolOptions,
        http_client_factory: Arc<dyn HttpClientFactory>,
        negotiated_version: TransportHttpVersion,
    ) -> azure_core::Result<Self> {
        let metadata_config = HttpClientConfig::metadata(&connection_pool, negotiated_version);
        let metadata_transport = AdaptiveTransport::from_config(
            &connection_pool,
            http_client_factory.clone(),
            metadata_config,
        )?;

        let gateway_config =
            HttpClientConfig::dataplane_gateway(&connection_pool, negotiated_version);
        let dataplane_gateway_transport = AdaptiveTransport::from_config(
            &connection_pool,
            http_client_factory.clone(),
            gateway_config,
        )?;

        Ok(Self {
            connection_pool,
            http_client_factory,
            negotiated_version,
            metadata_transport,
            dataplane_gateway_transport,
            dataplane_gateway20_transport: OnceLock::new(),
            insecure_emulator_metadata_transport: OnceLock::new(),
            insecure_emulator_dataplane_transport: OnceLock::new(),
        })
    }

    /// Creates a lightweight bootstrap transport for one-shot metadata probes.
    ///
    /// Uses a single unsharded HTTP client for metadata (no per-endpoint shard
    /// pools, no background health sweep). The dataplane gateway transport is
    /// created lazily only if somehow needed, keeping the bootstrap footprint
    /// minimal.
    pub(crate) fn bootstrap_metadata_only(
        connection_pool: ConnectionPoolOptions,
        http_client_factory: Arc<dyn HttpClientFactory>,
        negotiated_version: TransportHttpVersion,
    ) -> azure_core::Result<Self> {
        let metadata_config = HttpClientConfig::metadata(&connection_pool, negotiated_version);
        let metadata_transport = AdaptiveTransport::unsharded(
            &connection_pool,
            http_client_factory.clone(),
            metadata_config,
        )?;

        // Dataplane transport is unused for bootstrap probes. Create a
        // minimal unsharded instance to satisfy the struct layout; the
        // overhead is a single HTTP client with no background tasks.
        let gateway_config =
            HttpClientConfig::dataplane_gateway(&connection_pool, negotiated_version);
        let dataplane_gateway_transport = AdaptiveTransport::unsharded(
            &connection_pool,
            http_client_factory.clone(),
            gateway_config,
        )?;

        Ok(Self {
            connection_pool,
            http_client_factory,
            negotiated_version,
            metadata_transport,
            dataplane_gateway_transport,
            dataplane_gateway20_transport: OnceLock::new(),
            insecure_emulator_metadata_transport: OnceLock::new(),
            insecure_emulator_dataplane_transport: OnceLock::new(),
        })
    }

    /// Returns the negotiated HTTP version for this account.
    pub(crate) fn negotiated_version(&self) -> TransportHttpVersion {
        self.negotiated_version
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

    /// Returns the transport for metadata operations.
    pub(crate) fn get_metadata_transport(
        &self,
        endpoint: &AccountEndpoint,
    ) -> azure_core::Result<AdaptiveTransport> {
        let transport = if self.should_use_insecure_emulator_transport(endpoint) {
            match self.insecure_emulator_metadata_transport.get() {
                Some(t) => t.clone(),
                None => {
                    let config =
                        HttpClientConfig::metadata(&self.connection_pool, self.negotiated_version)
                            .with_allow_invalid_cert();
                    let t = AdaptiveTransport::from_config(
                        &self.connection_pool,
                        self.http_client_factory.clone(),
                        config,
                    )?;
                    self.insecure_emulator_metadata_transport
                        .get_or_init(|| t)
                        .clone()
                }
            }
        } else {
            self.metadata_transport.clone()
        };
        Ok(transport)
    }

    /// Returns the transport for a dataplane attempt based on the routed endpoint kind.
    pub(crate) fn get_dataplane_transport(
        &self,
        endpoint: &AccountEndpoint,
        transport_mode: TransportMode,
    ) -> azure_core::Result<AdaptiveTransport> {
        if self.should_use_insecure_emulator_transport(endpoint) {
            let transport = match self.insecure_emulator_dataplane_transport.get() {
                Some(t) => t.clone(),
                None => {
                    let config = HttpClientConfig::dataplane_gateway(
                        &self.connection_pool,
                        self.negotiated_version,
                    )
                    .with_allow_invalid_cert();
                    let t = AdaptiveTransport::from_config(
                        &self.connection_pool,
                        self.http_client_factory.clone(),
                        config,
                    )?;
                    self.insecure_emulator_dataplane_transport
                        .get_or_init(|| t)
                        .clone()
                }
            };
            return Ok(transport);
        }

        match transport_mode {
            TransportMode::Gateway20 if self.connection_pool.is_gateway20_allowed() => {
                let transport = match self.dataplane_gateway20_transport.get() {
                    Some(t) => t.clone(),
                    None => {
                        let config = HttpClientConfig::dataplane_gateway20(&self.connection_pool);
                        let t = AdaptiveTransport::gateway20(
                            &self.connection_pool,
                            self.http_client_factory.clone(),
                            config,
                        );
                        self.dataplane_gateway20_transport.get_or_init(|| t).clone()
                    }
                };
                Ok(transport)
            }
            _ => Ok(self.dataplane_gateway_transport.clone()),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::diagnostics::TransportHttpVersion;
    use crate::driver::pipeline::components::TransportMode;
    use crate::options::{ConnectionPoolOptionsBuilder, EmulatorServerCertValidation};

    #[test]
    fn transport_creates_with_http2() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();

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
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();

        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(transport.should_use_insecure_emulator_transport(&endpoint));

        let endpoint = AccountEndpoint::try_from("https://127.0.0.1:8081/").unwrap();
        assert!(transport.should_use_insecure_emulator_transport(&endpoint));

        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn transport_ignores_emulator_hosts_when_validation_enabled() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();

        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn metadata_transport_is_sharded_when_http2_negotiated() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        assert!(matches!(
            transport.get_metadata_transport(&endpoint).unwrap(),
            AdaptiveTransport::ShardedGateway(_)
        ));
    }

    #[test]
    fn metadata_transport_is_unsharded_when_http11_negotiated() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http11).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        assert!(matches!(
            transport.get_metadata_transport(&endpoint).unwrap(),
            AdaptiveTransport::Gateway(_)
        ));
    }

    #[test]
    fn dataplane_transport_is_unsharded_when_http11_negotiated() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http11).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx = transport
            .get_dataplane_transport(&endpoint, TransportMode::Gateway)
            .unwrap();
        assert!(matches!(ctx, AdaptiveTransport::Gateway(_)));
    }

    #[test]
    fn dataplane_transport_uses_gateway20_when_selected() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_gateway20_allowed(true)
            .build()
            .unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx = transport
            .get_dataplane_transport(&endpoint, TransportMode::Gateway20)
            .unwrap();
        assert!(matches!(ctx, AdaptiveTransport::ShardedGateway20(_)));
    }

    #[test]
    fn dataplane_transport_falls_back_to_sharded_gateway_when_endpoint_is_standard() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_gateway20_allowed(true)
            .build()
            .unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx = transport
            .get_dataplane_transport(&endpoint, TransportMode::Gateway)
            .unwrap();
        assert!(matches!(ctx, AdaptiveTransport::ShardedGateway(_)));
    }

    #[test]
    fn dataplane_transport_ignores_gateway20_when_gateway20_disabled() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_gateway20_allowed(false)
            .build()
            .unwrap();
        let transport = CosmosTransport::for_tests(pool, TransportHttpVersion::Http2).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx = transport
            .get_dataplane_transport(&endpoint, TransportMode::Gateway20)
            .unwrap();
        assert!(matches!(ctx, AdaptiveTransport::ShardedGateway(_)));
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
