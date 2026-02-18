// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Builder for creating [`CosmosClient`] instances.

use crate::{
    pipeline::{AuthorizationPolicy, GatewayPipeline},
    regions::RegionName,
    resource_context::{ResourceLink, ResourceType},
    CosmosClient, CosmosClientOptions,
};

use azure_core::{credentials::TokenCredential, http::Url};
use std::sync::Arc;

use crate::constants::COSMOS_ALLOWED_HEADERS;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;
use azure_core::http::{ClientOptions, InstrumentationOptions, LoggingOptions, RetryOptions};

/// Credential type for authentication with Cosmos DB.
enum Credential {
    /// Entra ID (Azure AD) token credential
    Token(Arc<dyn TokenCredential>),
    /// Primary or secondary account key
    #[cfg(feature = "key_auth")]
    Key(Secret),
}

/// Builder for creating [`CosmosClient`] instances.
///
/// Use this builder to configure and create a `CosmosClient` for interacting with Azure Cosmos DB.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::CosmosClientBuilder;
/// use std::sync::Arc;
///
/// let credential = azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let client = CosmosClientBuilder::new()
///     .endpoint("https://myaccount.documents.azure.com/")
///     .credential(credential)
///     .build()
///     .unwrap();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::CosmosClientBuilder;
/// use azure_core::credentials::Secret;
///
/// let client = CosmosClientBuilder::new()
///     .endpoint("https://myaccount.documents.azure.com/")
///     .key(Secret::from("my_account_key"))
///     .build()
///     .unwrap();
/// ```
///
/// Using a connection string (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::CosmosClientBuilder;
/// use azure_core::credentials::Secret;
///
/// let client = CosmosClientBuilder::new()
///     .connection_string(Secret::from("AccountEndpoint=https://myaccount.documents.azure.com:443/;AccountKey=mykey"))
///     .unwrap()
///     .build()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct CosmosClientBuilder {
    endpoint: Option<String>,
    credential: Option<Credential>,
    options: CosmosClientOptions,
    /// Instrumentation options for distributed tracing
    instrumentation: InstrumentationOptions,
    /// Custom transport for testing - not part of the public API
    transport: Option<azure_core::http::Transport>,
}

impl CosmosClientBuilder {
    /// Creates a new empty builder.
    ///
    /// Use [`endpoint()`](Self::endpoint) and either [`credential()`](Self::credential)
    /// or [`key()`](Self::key) to configure authentication, or use
    /// [`connection_string()`](Self::connection_string) to set both at once.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the endpoint URL for the Cosmos DB account.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Cosmos DB account, for example `https://myaccount.documents.azure.com/`.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Sets the Entra ID (Azure AD) credential for authentication.
    ///
    /// # Arguments
    ///
    /// * `credential` - An implementation of [`TokenCredential`] that can provide an Entra ID token.
    pub fn credential(mut self, credential: Arc<dyn TokenCredential>) -> Self {
        self.credential = Some(Credential::Token(credential));
        self
    }

    /// Sets the account key for authentication.
    ///
    /// # Arguments
    ///
    /// * `key` - The primary or secondary key for the Cosmos DB account.
    #[cfg(feature = "key_auth")]
    pub fn key(mut self, key: Secret) -> Self {
        self.credential = Some(Credential::Key(key));
        self
    }

    /// Sets both the endpoint and credential from a connection string.
    ///
    /// This is a convenience method that parses the connection string and sets
    /// both the endpoint and key credential.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The connection string to use for the client, e.g.
    ///   `AccountEndpoint=https://accountname.documents.azure.com:443/;AccountKey=accountkey`
    ///
    /// # Errors
    ///
    /// Returns an error if the connection string cannot be parsed.
    #[cfg(feature = "key_auth")]
    pub fn connection_string(mut self, connection_string: Secret) -> azure_core::Result<Self> {
        let connection_str = crate::ConnectionString::try_from(&connection_string)?;
        self.endpoint = Some(connection_str.account_endpoint);
        self.credential = Some(Credential::Key(connection_str.account_key));
        Ok(self)
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
    /// use azure_core::http::options::InstrumentationOptions;
    /// use std::sync::Arc;
    ///
    /// let tracer_provider = /* your TracerProvider implementation */;
    /// let options = InstrumentationOptions {
    ///     tracer_provider: Some(Arc::new(tracer_provider)),
    /// };
    ///
    /// let client = CosmosClientBuilder::new()
    ///     .endpoint("https://myaccount.documents.azure.com/")
    ///     .key(Secret::from("my_account_key"))
    ///     .with_instrumentation(options)
    ///     .build()
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

    /// Builds the [`CosmosClient`].
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The endpoint is not set
    /// - The credential is not set
    /// - The endpoint URL is invalid
    pub fn build(self) -> azure_core::Result<CosmosClient> {
        let endpoint_str = self.endpoint.ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "endpoint is required - use endpoint() or connection_string()",
            )
        })?;

        let credential = self.credential.ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "credential is required - use credential(), key(), or connection_string()",
            )
        })?;

        let endpoint: Url = endpoint_str.parse()?;

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
            Credential::Token(cred) => Arc::new(AuthorizationPolicy::from_token_credential(cred)),
            #[cfg(feature = "key_auth")]
            Credential::Key(key) => Arc::new(AuthorizationPolicy::from_shared_key(key)),
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

        let pipeline = Arc::new(GatewayPipeline::new(
            endpoint,
            pipeline_core,
            global_endpoint_manager.clone(),
            self.options,
            fault_injection_enabled,
        ));

        Ok(CosmosClient {
            databases_link: ResourceLink::root(ResourceType::Databases),
            pipeline,
            global_endpoint_manager,
        })
    }
}
