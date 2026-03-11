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
pub(crate) mod cosmos_headers;
mod emulator;
pub(crate) mod http_client_factory;
pub(crate) mod request_signing;
mod tracked_transport;
pub(crate) mod transport_pipeline;

use crate::{
    driver::cache::AccountProperties,
    models::{AccountEndpoint, OperationType, ResourceType},
    options::ConnectionPoolOptions,
};
use std::sync::{Arc, OnceLock};

use self::{
    adaptive_transport::{thin_client_endpoint_overrides, AdaptiveTransport, TransportContext},
    http_client_factory::{DefaultHttpClientFactory, HttpClientConfig, HttpClientFactory},
};

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
/// # Thread Safety
///
/// All pools are thread-safe and can be accessed concurrently. The transport
/// is designed to be shared across all drivers in a runtime.
#[derive(Debug)]
pub(crate) struct CosmosTransport {
    /// Connection pool configuration.
    connection_pool: ConnectionPoolOptions,

    /// Factory used to create protocol-specific HTTP transports.
    http_client_factory: Arc<dyn HttpClientFactory>,

    /// Transport for metadata operations.
    metadata_transport: AdaptiveTransport,

    /// Transport for dataplane gateway operations.
    dataplane_gateway_transport: AdaptiveTransport,

    /// Lazily-initialized transport for dataplane Gateway 2.0 operations.
    /// Only allocated when `is_gateway20_allowed()` is true and the account
    /// has thin-client endpoints — most deployments never create this.
    dataplane_gateway20_transport: OnceLock<AdaptiveTransport>,

    /// Lazily-initialized transport for emulator metadata operations.
    insecure_emulator_metadata_transport: OnceLock<AdaptiveTransport>,

    /// Lazily-initialized transport for emulator dataplane operations.
    /// The emulator does not support Gateway 2.0, so this always uses
    /// the standard gateway configuration.
    insecure_emulator_dataplane_transport: OnceLock<AdaptiveTransport>,
}

impl CosmosTransport {
    /// Creates a new transport with the given connection pool configuration.
    ///
    /// # Arguments
    ///
    /// * `connection_pool` - Connection pool settings for HTTP clients
    pub(crate) fn new(connection_pool: ConnectionPoolOptions) -> azure_core::Result<Self> {
        let http_client_factory: Arc<dyn HttpClientFactory> =
            Arc::new(DefaultHttpClientFactory::new());

        let metadata_config = HttpClientConfig::metadata(&connection_pool);
        let metadata_transport = AdaptiveTransport::from_policy(
            metadata_config.version_policy,
            http_client_factory.build(&connection_pool, metadata_config)?,
        );

        let gateway_config = HttpClientConfig::dataplane_gateway(&connection_pool);
        let dataplane_gateway_transport = AdaptiveTransport::from_policy(
            gateway_config.version_policy,
            http_client_factory.build(&connection_pool, gateway_config)?,
        );

        Ok(Self {
            connection_pool,
            http_client_factory,
            metadata_transport,
            dataplane_gateway_transport,
            dataplane_gateway20_transport: OnceLock::new(),
            insecure_emulator_metadata_transport: OnceLock::new(),
            insecure_emulator_dataplane_transport: OnceLock::new(),
        })
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

    /// Returns the adaptive transport for metadata operations.
    pub(crate) fn get_metadata_transport(&self, endpoint: &AccountEndpoint) -> AdaptiveTransport {
        if self.should_use_insecure_emulator_transport(endpoint) {
            self.insecure_emulator_metadata_transport
                .get_or_init(|| {
                    let config = HttpClientConfig::metadata(&self.connection_pool).for_emulator();
                    AdaptiveTransport::from_policy(
                        config.version_policy,
                        self.http_client_factory
                            .build(&self.connection_pool, config)
                            .expect("failed to create emulator metadata client"),
                    )
                })
                .clone()
        } else {
            self.metadata_transport.clone()
        }
    }

    /// Returns a [`TransportContext`] for dataplane operations.
    ///
    /// Selects Gateway 2.0 when allowed and thin-client endpoints are available.
    /// Computes thin-client endpoint overrides (merged read + write) when
    /// Gateway 2.0 is selected. The emulator does not support Gateway 2.0.
    pub(crate) fn get_dataplane_transport(
        &self,
        endpoint: &AccountEndpoint,
        account_properties: &AccountProperties,
    ) -> TransportContext {
        if self.should_use_insecure_emulator_transport(endpoint) {
            // The Cosmos emulator does not support Gateway 2.0 — always
            // use the standard gateway transport with insecure TLS.
            let transport = self
                .insecure_emulator_dataplane_transport
                .get_or_init(|| {
                    let config =
                        HttpClientConfig::dataplane_gateway(&self.connection_pool).for_emulator();
                    AdaptiveTransport::from_policy(
                        config.version_policy,
                        self.http_client_factory
                            .build(&self.connection_pool, config)
                            .expect("failed to create emulator dataplane client"),
                    )
                })
                .clone();
            TransportContext {
                transport,
                thin_client_overrides: None,
                is_gateway20: false,
            }
        } else if self.connection_pool.is_gateway20_allowed()
            && account_properties.has_thin_client_endpoints()
        {
            let transport = self
                .dataplane_gateway20_transport
                .get_or_init(|| {
                    let config = HttpClientConfig::dataplane_gateway20(&self.connection_pool);
                    AdaptiveTransport::from_policy(
                        config.version_policy,
                        self.http_client_factory
                            .build(&self.connection_pool, config)
                            .expect("failed to create Gateway 2.0 dataplane client"),
                    )
                })
                .clone();
            let overrides = thin_client_endpoint_overrides(account_properties);
            TransportContext {
                transport,
                thin_client_overrides: Some(Arc::new(overrides)),
                is_gateway20: true,
            }
        } else {
            TransportContext {
                transport: self.dataplane_gateway_transport.clone(),
                thin_client_overrides: None,
                is_gateway20: false,
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::options::{ConnectionPoolOptionsBuilder, EmulatorServerCertValidation};

    /// Shared test fixture: `AccountProperties` with thin-client endpoints.
    pub(crate) fn account_properties_with_thin_client() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}",
            "thinClientReadableLocations": [
                {
                    "name": "westus2",
                    "databaseAccountEndpoint": "https://test-westus2-thin.documents.azure.com:444/"
                }
            ],
            "thinClientWritableLocations": [
                {
                    "name": "eastus",
                    "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/"
                }
            ]
        }))
        .unwrap()
    }

    /// Shared test fixture: `AccountProperties` without thin-client endpoints.
    pub(crate) fn account_properties_without_thin_client() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [],
            "readableLocations": [],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    #[test]
    fn transport_creates_with_default_options() {
        let pool = ConnectionPoolOptionsBuilder::new().build().unwrap();
        let transport = CosmosTransport::new(pool).unwrap();

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
        let transport = CosmosTransport::new(pool).unwrap();

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
        let transport = CosmosTransport::new(pool).unwrap();

        // Even localhost should not use emulator transport if validation is enabled
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(!transport.should_use_insecure_emulator_transport(&endpoint));
    }

    #[test]
    fn metadata_transport_is_http2_preferred_when_http2_allowed() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(true)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        assert!(matches!(
            transport.get_metadata_transport(&endpoint),
            AdaptiveTransport::Gateway(_)
        ));
    }

    #[test]
    fn metadata_transport_uses_gateway_when_http2_flag_disabled() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(false)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        assert!(matches!(
            transport.get_metadata_transport(&endpoint),
            AdaptiveTransport::Gateway(_)
        ));
    }

    #[test]
    fn dataplane_transport_uses_gateway_when_http2_flag_disabled() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(false)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx =
            transport.get_dataplane_transport(&endpoint, &account_properties_without_thin_client());
        assert!(matches!(ctx.transport, AdaptiveTransport::Gateway(_)));
        assert!(!ctx.is_gateway20);
        assert!(ctx.thin_client_overrides.is_none());
    }

    #[test]
    fn dataplane_transport_uses_gateway20_when_allowed_and_available() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(true)
            .with_is_gateway20_allowed(true)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx =
            transport.get_dataplane_transport(&endpoint, &account_properties_with_thin_client());
        assert!(matches!(ctx.transport, AdaptiveTransport::Gateway20(_)));
        assert!(ctx.is_gateway20);
        assert!(ctx.thin_client_overrides.is_some());
    }

    #[test]
    fn dataplane_transport_falls_back_to_gateway_when_thin_client_missing() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(true)
            .with_is_gateway20_allowed(true)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx =
            transport.get_dataplane_transport(&endpoint, &account_properties_without_thin_client());
        assert!(matches!(ctx.transport, AdaptiveTransport::Gateway(_)));
        assert!(!ctx.is_gateway20);
        assert!(ctx.thin_client_overrides.is_none());
    }

    #[test]
    fn dataplane_transport_ignores_thin_client_when_gateway20_disabled() {
        let pool = ConnectionPoolOptionsBuilder::new()
            .with_is_http2_allowed(true)
            .with_is_gateway20_allowed(false)
            .build()
            .unwrap();
        let transport = CosmosTransport::new(pool).unwrap();
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();

        let ctx =
            transport.get_dataplane_transport(&endpoint, &account_properties_with_thin_client());
        assert!(matches!(ctx.transport, AdaptiveTransport::Gateway(_)));
        assert!(!ctx.is_gateway20);
        assert!(ctx.thin_client_overrides.is_none());
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
