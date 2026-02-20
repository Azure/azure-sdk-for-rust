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
    /// Custom transport for testing - not part of the public API
    transport: Option<azure_core::http::Transport>,
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

    /// Sets the regions to exclude from routing.
    ///
    /// Requests will not be routed to these regions. If all regions are excluded,
    /// the primary/hub region will be used.
    ///
    /// # Arguments
    ///
    /// * `regions` - The regions to exclude.
    pub fn with_excluded_regions(mut self, regions: impl Into<Vec<RegionName>>) -> Self {
        self.options.excluded_regions = regions.into();
        self
    }

    /// Sets the default consistency level for operations.
    ///
    /// This can be overridden on a per-request basis. If not set, the account's
    /// default consistency level is used.
    ///
    /// # Arguments
    ///
    /// * `level` - The consistency level to use.
    pub fn with_consistency_level(mut self, level: crate::options::ConsistencyLevel) -> Self {
        self.options.consistency_level = Some(level);
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

    /// Sets the application name for telemetry.
    ///
    /// # Arguments
    ///
    /// * `name` - The application name to include in telemetry.
    pub fn with_application_name(mut self, name: impl Into<String>) -> Self {
        self.options.application_name = Some(name.into());
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

    /// Enables fault injection for testing.
    ///
    /// This is only available when the `fault_injection` feature is enabled.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection(mut self, enabled: bool) -> Self {
        self.options.fault_injection_enabled = enabled;
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

    /// Enables partition-level circuit breaker.
    ///
    /// When enabled, the client will track failures at the partition level and
    /// temporarily avoid partitions that are experiencing issues.
    pub fn with_partition_level_circuit_breaker(mut self, enabled: bool) -> Self {
        self.options.enable_partition_level_circuit_breaker = enabled;
        self
    }

    /// Disables partition-level failover.
    ///
    /// # Arguments
    ///
    /// * `disabled` - If true, partition-level failover is disabled.
    pub fn with_disable_partition_level_failover(mut self, disabled: bool) -> Self {
        self.options.disable_partition_level_failover = disabled;
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

    /// Internal method to set a custom HTTP transport.
    /// This is used for testing purposes (e.g., to accept invalid certificates).
    ///
    /// Microsoft cannot guarantee support when using alternate transports.
    #[doc(hidden)]
    pub fn transport(mut self, transport: azure_core::http::Transport) -> Self {
        self.transport = Some(transport);
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
            transport: self.transport,
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
        #[cfg(feature = "fault_injection")]
        let fault_injection_enabled = self.options.fault_injection_enabled;
        #[cfg(not(feature = "fault_injection"))]
        let fault_injection_enabled = false;
        let excluded_regions = self.options.excluded_regions.clone();

        let global_endpoint_manager = Arc::new(GlobalEndpointManager::new(
            endpoint.clone(),
            preferred_regions,
            excluded_regions,
            pipeline_core.clone(),
        ));

        let global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager> =
            GlobalPartitionEndpointManager::new(
                global_endpoint_manager.clone(),
                false,
                options.enable_partition_level_circuit_breaker,
            );

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
