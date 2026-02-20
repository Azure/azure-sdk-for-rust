// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating [`CosmosClient`] instances.

use crate::{
    pipeline::{AuthorizationPolicy, GatewayPipeline},
    regions::RegionName,
    resource_context::{ResourceLink, ResourceType},
    CosmosAccountReference, CosmosClient, CosmosClientOptions, CosmosCredential,
};

use std::sync::Arc;

use crate::constants::COSMOS_ALLOWED_HEADERS;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use azure_core::http::{ClientOptions, InstrumentationOptions, LoggingOptions, RetryOptions};

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
/// use azure_data_cosmos::{CosmosClientBuilder, CosmosAccountReference};
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let account = CosmosAccountReference::with_credential(
///     "https://myaccount.documents.azure.com/",
///     credential,
/// ).unwrap();
/// let client = CosmosClientBuilder::new()
///     .build(account)
///     .unwrap();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::{CosmosClientBuilder, CosmosAccountReference};
/// use azure_core::credentials::Secret;
///
/// let account = CosmosAccountReference::with_master_key(
///     "https://myaccount.documents.azure.com/",
///     Secret::from("my_account_key"),
/// ).unwrap();
/// let client = CosmosClientBuilder::new()
///     .build(account)
///     .unwrap();
/// ```
#[derive(Default)]
pub struct CosmosClientBuilder {
    options: CosmosClientOptions,
    /// Instrumentation options for distributed tracing
    instrumentation: InstrumentationOptions,
    /// Whether to accept invalid TLS certificates (e.g., for emulator testing)
    #[cfg(feature = "allow_invalid_certificates")]
    allow_invalid_certificates: bool,
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

    /// Sets the preferred regions for the client.
    ///
    /// The client will prefer to connect to regions in the order specified.
    /// This is useful for geo-distributed applications that want to minimize latency.
    ///
    /// # Arguments
    ///
    /// * `regions` - The regions to prefer, in order of preference.
    pub fn with_application_preferred_regions(
        mut self,
        regions: impl Into<Vec<RegionName>>,
    ) -> Self {
        self.options.application_preferred_regions = regions.into();
        self
    }

    /// Sets the default priority level for operations.
    ///
    /// Priority-based execution allows throttling low-priority requests before
    /// high-priority ones. This feature must be enabled at the account level.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level to use.
    pub fn with_priority(mut self, priority: crate::options::PriorityLevel) -> Self {
        self.options.priority = Some(priority);
        self
    }

    /// Sets the throughput bucket for the client.
    ///
    /// See [Throughput Control](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The throughput bucket identifier.
    pub fn with_throughput_bucket(mut self, bucket: usize) -> Self {
        self.options.throughput_bucket = Some(bucket);
        self
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

    /// Sets the application region for telemetry.
    ///
    /// # Arguments
    ///
    /// * `region` - The region where the application is running.
    pub fn with_application_region(mut self, region: RegionName) -> Self {
        self.options.application_region = Some(region);
        self
    }

    /// Sets the request timeout.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for requests.
    pub fn with_request_timeout(mut self, timeout: azure_core::time::Duration) -> Self {
        self.options.request_timeout = Some(timeout);
        self
    }

    /// Configures fault injection for testing.
    ///
    /// Pass a [`FaultInjectionClientBuilder`](crate::fault_injection::FaultInjectionClientBuilder)
    /// configured with the desired fault injection rules. The builder will be used
    /// to construct the transport internally when [`build()`](Self::build) is called.
    ///
    /// This is only available when the `fault_injection` feature is enabled.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection(
        mut self,
        builder: crate::fault_injection::FaultInjectionClientBuilder,
    ) -> Self {
        self.fault_injection_builder = Some(builder);
        self
    }

    /// Sets the session retry options.
    ///
    /// # Arguments
    ///
    /// * `options` - The session retry configuration.
    pub fn with_session_retry_options(
        mut self,
        options: crate::options::SessionRetryOptions,
    ) -> Self {
        self.options.session_retry_options = options;
        self
    }

    /// Sets the instrumentation options for distributed tracing.
    ///
    /// # Arguments
    ///
    /// * `options` - The instrumentation configuration, including tracer provider.
    ///
    /// # Examples
    ///
    /// ```rust,no_run,ignore
    /// use azure_data_cosmos::CosmosClientBuilder;
    /// use azure_core::http::InstrumentationOptions;
    /// use std::sync::Arc;
    ///
    /// let tracer_provider = /* your TracerProvider implementation */;
    /// let options = InstrumentationOptions {
    ///     tracer_provider: Some(Arc::new(tracer_provider)),
    /// };
    ///
    /// let client = CosmosClientBuilder::new()
    ///     .with_instrumentation(options)
    ///     .build(CosmosAccountReference::with_master_key(
    ///         "https://myaccount.documents.azure.com/",
    ///         Secret::from("my_account_key"),
    ///     ).unwrap())
    ///     .unwrap();
    /// ```
    pub fn with_instrumentation(mut self, options: InstrumentationOptions) -> Self {
        self.instrumentation = options;
        self
    }

    /// Configures the client to accept invalid TLS certificates.
    ///
    /// This is intended for testing with the Azure Cosmos DB emulator,
    /// which uses a self-signed certificate.
    ///
    /// # Arguments
    ///
    /// * `allow` - Whether to accept invalid certificates.
    #[doc(hidden)]
    #[cfg(feature = "allow_invalid_certificates")]
    pub fn with_allow_invalid_certificates(mut self, allow: bool) -> Self {
        self.allow_invalid_certificates = allow;
        self
    }

    /// Builds the [`CosmosClient`] with the specified account reference.
    ///
    /// The account reference bundles an endpoint and credential. You can create one using
    /// [`CosmosAccountReference::with_credential()`], [`CosmosAccountReference::with_master_key()`],
    /// or [`CosmosAccountReferenceBuilder`](crate::CosmosAccountReferenceBuilder).
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
    pub fn build(
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

        // Build custom transport if needed
        #[cfg(feature = "allow_invalid_certificates")]
        let base_client: Option<Arc<dyn azure_core::http::HttpClient>> =
            if self.allow_invalid_certificates {
                let client = reqwest::ClientBuilder::new()
                    .danger_accept_invalid_certs(true)
                    .pool_max_idle_per_host(0)
                    .build()
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;
                Some(Arc::new(client))
            } else {
                None
            };
        #[cfg(not(feature = "allow_invalid_certificates"))]
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
            instrumentation: self.instrumentation,
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

        let pipeline_core = azure_core::http::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            client_options,
            Vec::new(),
            vec![auth_policy],
            None,
        );

        let preferred_regions = self.options.application_preferred_regions.clone();
        let excluded_regions = self.options.excluded_regions.clone();

        let global_endpoint_manager = Arc::new(GlobalEndpointManager::new(
            endpoint.clone(),
            preferred_regions,
            excluded_regions,
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
