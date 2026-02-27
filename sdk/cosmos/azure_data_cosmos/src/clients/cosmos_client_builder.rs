// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating [`CosmosClient`] instances.

use crate::{
    availability_strategy::AvailabilityStrategy,
    pipeline::{AuthorizationPolicy, CosmosHeadersPolicy, GatewayPipeline},
    regions::RegionName,
    resource_context::{ResourceLink, ResourceType},
    CosmosAccountReference, CosmosClient, CosmosClientOptions, CosmosCredential,
};

use std::sync::Arc;

use crate::constants::COSMOS_ALLOWED_HEADERS;
#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
use crate::constants::{
    DEFAULT_CONNECTION_TIMEOUT, DEFAULT_MAX_CONNECTION_POOL_SIZE, DEFAULT_REQUEST_TIMEOUT,
};
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
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint};
/// use std::sync::Arc;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_credential(endpoint, credential);
/// let client = CosmosClientBuilder::new()
///     .build(account)
///     .await?;
/// # Ok(())
/// # }
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClientBuilder, CosmosAccountReference, CosmosAccountEndpoint};
/// use azure_core::credentials::Secret;
///
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_master_key(endpoint, Secret::from("my_account_key"));
/// let client = CosmosClientBuilder::new()
///     .build(account)
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct CosmosClientBuilder {
    options: CosmosClientOptions,
    /// Whether to accept invalid TLS certificates when connecting to the emulator.
    #[cfg(feature = "allow_invalid_certificates")]
    allow_emulator_invalid_certificates: bool,
    /// Fault injection builder for testing error handling
    #[cfg(feature = "fault_injection")]
    fault_injection_builder: Option<crate::fault_injection::FaultInjectionClientBuilder>,
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

    /// Sets the application region for routing.
    ///
    /// When set, the SDK generates a list of preferred regions sorted by
    /// geographic proximity (round-trip time) from the given region.
    /// This allows the client to prefer connecting to regions closest
    /// to where the application is running.
    ///
    /// If not set, the SDK uses the account's configured regions
    /// in the order returned by the service.
    ///
    /// Unknown region names will cause [`build()`](Self::build) to return an error.
    ///
    /// # Arguments
    ///
    /// * `region` - The region where the application is running.
    pub fn with_application_region(mut self, region: RegionName) -> Self {
        self.options.application_region = Some(region);
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

    /// Sets the availability strategy for cross-region hedging.
    ///
    /// When configured, the SDK sends parallel requests to additional regions
    /// when the primary request exceeds a latency threshold. This can reduce
    /// tail latency when a region is experiencing high response times.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The availability strategy to use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azure_data_cosmos::{CosmosClientBuilder, AvailabilityStrategy};
    /// use std::time::Duration;
    ///
    /// let builder = CosmosClientBuilder::new()
    ///     .with_availability_strategy(
    ///         AvailabilityStrategy::cross_region_hedging(
    ///             Duration::from_millis(500),
    ///             Some(Duration::from_millis(100)),
    ///         ),
    ///     );
    /// ```
    pub fn with_availability_strategy(mut self, strategy: AvailabilityStrategy) -> Self {
        self.options.availability_strategy = Some(strategy);
        self
    }

    /// Builds the [`CosmosClient`] with the specified account reference.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub async fn build(
        self,
        account: impl Into<CosmosAccountReference>,
    ) -> azure_core::Result<CosmosClient> {
        let (account_endpoint, credential) = account.into().into_parts();
        let endpoint = account_endpoint.into_url();

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
        let transport: Option<azure_core::http::Transport> =
            if let Some(fault_builder) = self.fault_injection_builder {
                let fault_builder = match base_client {
                    Some(client) => fault_builder.with_inner_client(client),
                    None => fault_builder,
                };
                Some(fault_builder.build())
            } else {
                base_client.map(azure_core::http::Transport::new)
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

        let global_endpoint_manager = Arc::new(GlobalEndpointManager::new(
            endpoint.clone(),
            preferred_regions,
            Vec::new(),
            pipeline_core.clone(),
        ));

        let global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager> =
            GlobalPartitionEndpointManager::new(global_endpoint_manager.clone(), false, true);

        let pipeline = Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline_core,
            global_endpoint_manager.clone(),
            global_partition_endpoint_manager.clone(),
            self.options,
            fault_injection_enabled,
        ));

        Ok(CosmosClient {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline,
            global_endpoint_manager,
            global_partition_endpoint_manager,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{regions, CosmosAccountReference, CosmosClient};
    use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
    use std::sync::Arc;

    #[derive(Debug)]
    struct MockCredential;

    #[async_trait::async_trait]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            Ok(AccessToken::new(
                "mock_token",
                azure_core::time::OffsetDateTime::now_utc(),
            ))
        }
    }

    fn test_account() -> CosmosAccountReference {
        let endpoint = "https://test.documents.azure.com/".parse().unwrap();
        CosmosAccountReference::with_credential(endpoint, Arc::new(MockCredential))
    }

    #[tokio::test]
    async fn build_with_known_region_succeeds() {
        let result = CosmosClient::builder()
            .with_application_region(regions::EAST_US)
            .build(test_account())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn build_with_unknown_region_succeeds() {
        let unknown = regions::RegionName::from("unknown");
        let result = CosmosClient::builder()
            .with_application_region(unknown)
            .build(test_account())
            .await;
        assert!(
            result.is_ok(),
            "unknown region should fall back gracefully, not fail"
        );
    }

    /// When an unknown region is passed, the SDK cannot generate proximity ordering
    /// so it falls back to account-order (same as no application_region set).
    #[tokio::test]
    async fn build_with_unknown_region_uses_account_order() {
        use crate::models::AccountRegion;

        let unknown = regions::RegionName::from("unknown");
        let client = CosmosClient::builder()
            .with_application_region(unknown)
            .build(test_account())
            .await
            .expect("build should succeed");

        // Account regions in a specific order
        let regions_list = vec![
            AccountRegion {
                name: regions::WEST_EUROPE.clone(),
                database_account_endpoint: "https://test-westeurope.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ];
        client
            .global_endpoint_manager
            .update_location_cache(regions_list.clone(), regions_list);

        // Endpoints should reflect account order (no proximity reordering)
        let read_endpoints = client.global_endpoint_manager.read_endpoints();
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-westeurope.documents.azure.com/",
                "https://test-eastus2.documents.azure.com/",
            ],
            "unknown region should not reorder — account order preserved"
        );
    }

    #[tokio::test]
    async fn build_without_application_region_succeeds() {
        let result = CosmosClient::builder().build(test_account()).await;
        assert!(result.is_ok());
    }

    /// Verifies the full flow: builder with application_region → proximity list →
    /// GlobalEndpointManager → LocationCache → correctly ordered endpoints.
    ///
    /// Passing in East US should produce a proximity-ordered list of regions with East US 2 first,
    /// then West US, then West Europe.
    #[tokio::test]
    async fn application_region_produces_proximity_ordered_endpoints() {
        use crate::models::AccountRegion;

        let client = CosmosClient::builder()
            .with_application_region(regions::EAST_US)
            .build(test_account())
            .await
            .expect("build should succeed with known region");

        // Simulate receiving account properties with 3 regions
        let regions_list = vec![
            AccountRegion {
                name: regions::WEST_EUROPE.clone(),
                database_account_endpoint: "https://test-westeurope.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ];

        // Feed the account regions into the location cache
        client
            .global_endpoint_manager
            .update_location_cache(regions_list.clone(), regions_list);

        // Verify read endpoints are in proximity order from East US:
        // East US 2 (closest), West US, West Europe (farthest)
        let read_endpoints = client.global_endpoint_manager.read_endpoints();
        let endpoint_strings: Vec<&str> = read_endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            endpoint_strings,
            vec![
                "https://test-eastus2.documents.azure.com/",
                "https://test-westus.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
            "endpoints should be in proximity order from East US"
        );
    }

    /// Verifies that request-level excluded regions interact correctly with
    /// proximity-ordered preferred regions through the full builder flow.
    #[tokio::test]
    async fn application_region_with_excluded_regions() {
        use crate::models::AccountRegion;
        use crate::operation_context::OperationType;

        let client = CosmosClient::builder()
            .with_application_region(regions::EAST_US)
            .build(test_account())
            .await
            .expect("build should succeed");

        let regions_list = vec![
            AccountRegion {
                name: regions::EAST_US_2.clone(),
                database_account_endpoint: "https://test-eastus2.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_US.clone(),
                database_account_endpoint: "https://test-westus.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
            AccountRegion {
                name: regions::WEST_EUROPE.clone(),
                database_account_endpoint: "https://test-westeurope.documents.azure.com/"
                    .parse()
                    .unwrap(),
            },
        ];
        client
            .global_endpoint_manager
            .update_location_cache(regions_list.clone(), regions_list);

        // Without excluded regions: full proximity order
        let endpoints = client
            .global_endpoint_manager
            .applicable_endpoints(OperationType::Read, None);
        let strings: Vec<&str> = endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            strings,
            vec![
                "https://test-eastus2.documents.azure.com/",
                "https://test-westus.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
        );

        // Exclude the closest region (East US 2): falls back to next closest
        let excluded = vec![regions::EAST_US_2];
        let endpoints = client
            .global_endpoint_manager
            .applicable_endpoints(OperationType::Read, Some(&excluded));
        let strings: Vec<&str> = endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            strings,
            vec![
                "https://test-westus.documents.azure.com/",
                "https://test-westeurope.documents.azure.com/",
            ],
            "West US should be first after excluding East US 2"
        );

        // Exclude all account regions: falls back to default
        let excluded = vec![regions::EAST_US_2, regions::WEST_US, regions::WEST_EUROPE];
        let endpoints = client
            .global_endpoint_manager
            .applicable_endpoints(OperationType::Read, Some(&excluded));
        let strings: Vec<&str> = endpoints.iter().map(|u| u.as_str()).collect();
        assert_eq!(
            strings,
            vec!["https://test.documents.azure.com/"],
            "should fall back to default endpoint"
        );
    }
}
