// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating [`CosmosClient`] instances.

use crate::{
    pipeline::{AuthorizationPolicy, CosmosHeadersPolicy, GatewayPipeline},
    resource_context::{ResourceLink, ResourceType},
    CosmosAccountReference, CosmosClient, CosmosClientOptions, CosmosCredential, RoutingStrategy,
};

#[cfg(feature = "allow_invalid_certificates")]
use azure_data_cosmos_driver::options::{ConnectionPoolOptions, EmulatorServerCertValidation};
use azure_data_cosmos_driver::CosmosDriverRuntimeBuilder;
use std::sync::Arc;

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
use crate::constants::{
    AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED, COSMOS_ALLOWED_HEADERS,
    DEFAULT_CONNECTION_TIMEOUT, DEFAULT_MAX_CONNECTION_POOL_SIZE, DEFAULT_REQUEST_TIMEOUT,
};
use crate::models::AccountProperties;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use azure_core::http::{ClientOptions, LoggingOptions, RetryOptions};

/// Builder for creating [`CosmosClient`] instances.
///
/// Use this builder to configure and create a `CosmosClient` for interacting with Azure Cosmos DB.
///
/// An account reference (endpoint + credential) is required when calling [`build()`](Self::build).
/// Pass any type that implements `Into<CosmosAccountReference>`, such as a
/// [`CosmosAccountReference`] created via convenience constructors, or a tuple of
/// `(CosmosAccountEndpoint, credential)` or `(Url, credential)`.
///
/// A [`RoutingStrategy`] is also required to specify how the SDK should select regions.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint,
///     Region, RoutingStrategy,
/// };
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_credential(endpoint, credential);
/// let client = CosmosClientBuilder::new()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{
///     CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint,
///     Region, RoutingStrategy,
/// };
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_master_key(endpoint, Secret::from("my_account_key"));
/// let client = CosmosClientBuilder::new()
///     .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct CosmosClientBuilder {
    options: CosmosClientOptions,
    /// Whether to allow proxy usage. When false (default), `HTTPS_PROXY` is ignored.
    allow_proxy: bool,
    /// Whether to accept invalid TLS certificates when connecting to the emulator.
    #[cfg(feature = "allow_invalid_certificates")]
    allow_emulator_invalid_certificates: bool,
    /// Fault injection builder for testing error handling
    #[cfg(feature = "fault_injection")]
    fault_injection_builder: Option<crate::fault_injection::FaultInjectionClientBuilder>,
    /// Fallback endpoints tried when the primary endpoint is unavailable.
    backup_endpoints: Vec<azure_core::http::Url>,
}

impl CosmosClientBuilder {
    /// Creates a new empty builder.
    ///
    /// Configure the builder with the desired options, then call [`build()`](Self::build)
    /// with the account endpoint and credential.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a suffix to append to the User-Agent header for telemetry.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to append to the User-Agent header.
    pub fn with_user_agent_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.options.user_agent_suffix = Some(suffix.into());
        self
    }

    /// Configures fault injection for testing.
    ///
    /// Pass a [`FaultInjectionClientBuilder`](crate::fault_injection::FaultInjectionClientBuilder)
    /// configured with the desired fault injection rules. The builder will be used
    /// to construct the transport internally when [`build()`](Self::build) is called.
    ///
    /// This is only available when the `fault_injection` feature is enabled.
    #[doc(hidden)]
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection(
        mut self,
        builder: crate::fault_injection::FaultInjectionClientBuilder,
    ) -> Self {
        self.fault_injection_builder = Some(builder);
        self
    }

    /// Configures the client to accept invalid TLS certificates when connecting
    /// to the Azure Cosmos DB emulator.
    ///
    /// This setting only applies when connecting to the local emulator
    /// (e.g., `https://localhost:8081/`). It should not be used for production endpoints.
    ///
    /// # Arguments
    ///
    /// * `allow` - Whether to accept invalid certificates for emulator connections.
    #[doc(hidden)]
    #[cfg(feature = "allow_invalid_certificates")]
    pub fn with_allow_emulator_invalid_certificates(mut self, allow: bool) -> Self {
        self.allow_emulator_invalid_certificates = allow;
        self
    }

    /// Allows the SDK to use HTTP proxies and respect system proxy settings.
    ///
    /// By default, the Cosmos DB SDK ignores the `HTTPS_PROXY`, `HTTP_PROXY`,
    /// `ALL_PROXY` environment variables and their lowercase variants. Proxies
    /// can cause issues for Cosmos DB connectivity, availability, and throughput.
    ///
    /// When enabled, the SDK will respect system-configured proxy settings
    /// (such as proxy-related environment variables, including any exclusions).
    ///
    /// NOTE: End-to-end latency, availability, and throughput guarantees cannot
    /// be provided when a proxy is in use. Full backend support is provided,
    /// but client/proxy interactions are supported on a best-effort basis only.
    ///
    /// # Arguments
    ///
    /// * `allow` - Whether to allow proxy usage.
    pub fn with_proxy_allowed(mut self, allow: bool) -> Self {
        self.allow_proxy = allow;
        self
    }

    /// Sets backup endpoints that the client will try when the primary global
    /// endpoint is unavailable during initialization.
    ///
    /// If the primary endpoint fails during driver bootstrap, the SDK will try
    /// each backup endpoint in order until one succeeds. A successful connection
    /// allows normal service discovery to proceed. Once initialized, regional
    /// endpoints discovered during bootstrap handle subsequent refreshes.
    ///
    /// # Arguments
    ///
    /// * `endpoints` - Ordered list of fallback endpoint URLs.
    pub fn with_backup_endpoints(mut self, endpoints: Vec<crate::CosmosAccountEndpoint>) -> Self {
        self.backup_endpoints = endpoints.into_iter().map(|e| e.into_url()).collect();
        self
    }

    /// Builds the [`CosmosClient`] with the specified account reference and region selection strategy.
    ///
    /// The account reference bundles an endpoint and credential. You can create one using
    /// [`CosmosAccountReference::with_credential()`] or [`CosmosAccountReference::with_master_key()`].
    ///
    /// You can also pass a tuple of `(CosmosAccountEndpoint, credential)` or `(Url, credential)`,
    /// where `credential` is any type that implements `Into<CosmosCredential>`.
    ///
    /// # Arguments
    ///
    /// * `account` - The account reference containing the endpoint and credential.
    /// * `routing_strategy` - The strategy for selecting which Azure regions to route requests to.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub async fn build(
        mut self,
        account: impl Into<CosmosAccountReference>,
        routing_strategy: RoutingStrategy,
    ) -> azure_core::Result<CosmosClient> {
        // Apply the region selection strategy to internal options.
        match routing_strategy {
            RoutingStrategy::ProximityTo(region) => {
                self.options.application_region = Some(region);
            }
        }

        let (account_endpoint, credential) = account.into().into_parts();
        let endpoint = account_endpoint.into_url();

        // Clone credential for the driver before the SDK consumes it for auth policy.
        let driver_credential = credential.clone();

        // Derive fault_injection_enabled from builder state
        #[cfg(feature = "fault_injection")]
        let fault_injection_enabled = self.fault_injection_builder.is_some();
        #[cfg(not(feature = "fault_injection"))]
        let fault_injection_enabled = false;

        // Build custom transport with default timeouts.
        // When no custom transport is provided, we create a reqwest client with
        // connection and request timeouts per Cosmos DB design principles.
        #[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
        let base_client: Option<Arc<dyn azure_core::http::HttpClient>> = {
            #[allow(unused_mut)]
            let mut builder = reqwest::ClientBuilder::new()
                .http1_only()
                .pool_max_idle_per_host(DEFAULT_MAX_CONNECTION_POOL_SIZE)
                .connect_timeout(DEFAULT_CONNECTION_TIMEOUT)
                .timeout(DEFAULT_REQUEST_TIMEOUT);

            if self.allow_proxy {
                tracing::warn!(
                    "Proxy usage is enabled. Azure Cosmos DB does not provide end-to-end SLAs \
                     when a proxy is in use. Full backend support is provided, but client/proxy \
                     interactions are supported on a best-effort basis only."
                );
            } else {
                builder = builder.no_proxy();
            }

            #[cfg(feature = "allow_invalid_certificates")]
            if self.allow_emulator_invalid_certificates {
                builder = builder.danger_accept_invalid_certs(true);
            }

            let client = builder
                .build()
                .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;
            Some(Arc::new(client))
        };
        #[cfg(not(all(not(target_arch = "wasm32"), feature = "reqwest")))]
        let base_client: Option<Arc<dyn azure_core::http::HttpClient>> = None;

        #[cfg(feature = "fault_injection")]
        let (transport, driver_fi_rules): (
            Option<azure_core::http::Transport>,
            Vec<std::sync::Arc<azure_data_cosmos_driver::fault_injection::FaultInjectionRule>>,
        ) = if let Some(fault_builder) = self.fault_injection_builder {
            // Translate rules for the driver before the builder is consumed.
            let driver_rules =
                crate::driver_bridge::sdk_fi_rules_to_driver_fi_rules(fault_builder.rules());
            let fault_builder = match base_client {
                Some(client) => fault_builder.with_inner_client(client),
                None => fault_builder,
            };
            (Some(fault_builder.build()), driver_rules)
        } else {
            (
                base_client.map(azure_core::http::Transport::new),
                Vec::new(),
            )
        };
        #[cfg(not(feature = "fault_injection"))]
        let transport: Option<azure_core::http::Transport> =
            base_client.map(azure_core::http::Transport::new);

        // Create internal ClientOptions - users cannot configure this directly
        let client_options = ClientOptions {
            retry: RetryOptions::none(),
            logging: LoggingOptions {
                additional_allowed_header_names: COSMOS_ALLOWED_HEADERS
                    .iter()
                    .map(|h| std::borrow::Cow::Borrowed(h.as_str()))
                    .collect(),
                additional_allowed_query_params: vec![],
            },
            transport,
            ..Default::default()
        };

        let auth_policy: Arc<AuthorizationPolicy> = match credential {
            CosmosCredential::TokenCredential(cred) => {
                Arc::new(AuthorizationPolicy::from_token_credential(cred))
            }
            #[cfg(feature = "key_auth")]
            CosmosCredential::MasterKey(key) => Arc::new(AuthorizationPolicy::from_shared_key(key)),
        };

        // Create Cosmos headers policy to override User-Agent with Cosmos-specific value.
        // This runs as a per-call policy after azure_core's UserAgentPolicy.
        let crate_version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
        let cosmos_headers_policy: Arc<dyn azure_core::http::policies::Policy> = Arc::new(
            CosmosHeadersPolicy::new(crate_version, self.options.user_agent_suffix.as_deref()),
        );

        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            vec![cosmos_headers_policy],
            vec![auth_policy],
            None,
        );

        let preferred_regions = if let Some(ref region) = self.options.application_region {
            crate::region_proximity::generate_preferred_region_list(region)
                .map(|s| s.to_vec())
                .unwrap_or_else(|| {
                    tracing::warn!(
                        region = %region,
                        "unrecognized application region; falling back to account-defined region order"
                    );
                    Vec::new()
                })
        } else {
            Vec::new()
        };

        let global_endpoint_manager = GlobalEndpointManager::new(
            endpoint.clone(),
            preferred_regions,
            Vec::new(),
            pipeline_core.clone(),
        );

        // Enable per-partition circuit breaker based on the
        // `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` environment
        // variable. When unset or not parseable, defaults to `true`.
        let enable_partition_level_circuit_breaker =
            std::env::var(AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED)
                .ok()
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(true);

        let global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager> =
            GlobalPartitionEndpointManager::new(
                global_endpoint_manager.clone(),
                false,
                enable_partition_level_circuit_breaker,
            );

        // Register the callback for account refresh to update partition-level failover config
        let partition_manager_clone = Arc::clone(&global_partition_endpoint_manager);

        global_endpoint_manager.set_on_account_refresh_callback(Arc::new(
            move |account_props: &AccountProperties| {
                partition_manager_clone.configure_partition_level_automatic_failover(
                    account_props.enable_per_partition_failover_behavior,
                );

                partition_manager_clone.configure_per_partition_circuit_breaker(
                    account_props.enable_per_partition_failover_behavior
                        || enable_partition_level_circuit_breaker,
                );
            },
        ));

        let pipeline = Arc::new(GatewayPipeline::new(
            endpoint.clone(),
            pipeline_core,
            global_endpoint_manager.clone(),
            global_partition_endpoint_manager.clone(),
            self.options,
            fault_injection_enabled,
        ));

        // Create the CosmosDriver for eager container metadata resolution.
        // TODO: Each CosmosClient currently creates its own CosmosDriverRuntime. The runtime
        // should be shared across clients targeting the same account to avoid duplicate
        // background tasks and connection pools. See https://github.com/Azure/azure-sdk-for-rust/issues/3908
        let driver_account =
            build_driver_account(endpoint, driver_credential, self.backup_endpoints);
        #[allow(unused_mut)]
        let mut driver_runtime_builder = CosmosDriverRuntimeBuilder::new();
        #[cfg(feature = "allow_invalid_certificates")]
        if self.allow_emulator_invalid_certificates {
            let connection_pool = ConnectionPoolOptions::builder()
                .with_emulator_server_cert_validation(
                    EmulatorServerCertValidation::DangerousDisabled,
                )
                .build()?;
            driver_runtime_builder = driver_runtime_builder.with_connection_pool(connection_pool);
        }
        #[cfg(feature = "fault_injection")]
        if !driver_fi_rules.is_empty() {
            driver_runtime_builder =
                driver_runtime_builder.with_fault_injection_rules(driver_fi_rules);
        }
        let driver_runtime = driver_runtime_builder.build().await?;
        let driver = driver_runtime
            .get_or_create_driver(driver_account, None)
            .await?;

        Ok(CosmosClient {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline,
            driver,
            global_endpoint_manager,
            global_partition_endpoint_manager,
        })
    }
}

/// Builds a driver [`AccountReference`](azure_data_cosmos_driver::models::AccountReference)
/// from the SDK's credential and endpoint.
fn build_driver_account(
    endpoint: azure_core::http::Url,
    credential: CosmosCredential,
    backup_endpoints: Vec<azure_core::http::Url>,
) -> azure_data_cosmos_driver::models::AccountReference {
    let base = match credential {
        CosmosCredential::TokenCredential(tc) => {
            azure_data_cosmos_driver::models::AccountReference::with_credential(endpoint, tc)
        }
        #[cfg(feature = "key_auth")]
        CosmosCredential::MasterKey(key) => {
            azure_data_cosmos_driver::models::AccountReference::with_master_key(endpoint, key)
        }
    };
    base.with_backup_endpoints(backup_endpoints)
}

// Unit tests for routing-strategy behavior were removed because
// CosmosClient::builder().build() now eagerly creates a CosmosDriver,
// which requires a real endpoint. Re-add once fault injection is linked
// from the SDK to the driver.
