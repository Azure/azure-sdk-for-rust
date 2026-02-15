// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::constants;
use crate::models::ThroughputProperties;
use crate::regions::RegionName;
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use azure_core::http::{headers, ClientMethodOptions, ClientOptions, Etag};
use azure_core::time::Duration;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fmt;
use std::fmt::Display;

/// Session tokens are intended to be opaque. They are used to ensure session consistency.
///
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SessionToken(String);

impl From<String> for SessionToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for SessionToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
#[non_exhaustive]
#[derive(Clone, Default, Debug)]
pub struct CosmosClientOptions {
    pub(crate) client_options: ClientOptions,
    application_name: Option<String>,
    application_region: Option<RegionName>,
    // when the cosmos client options is changed to builder pattern, this shouldn't be exposed directly to customers
    // right now it is exposed behind a feature flag
    #[cfg(feature = "fault_injection")]
    pub(crate) fault_injection_enabled: bool,
    pub(crate) application_preferred_regions: Vec<RegionName>,
    pub(crate) excluded_regions: Vec<RegionName>,
    account_initialization_custom_endpoints: Option<HashSet<String>>,
    /// Used to specify the consistency level for the operation.
    ///
    /// The default value is the consistency level set on the Cosmos DB account.
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    consistency_level: Option<ConsistencyLevel>,
    request_timeout: Option<Duration>,
    enable_remote_region_preferred_for_session_retry: bool,
    enable_partition_level_circuit_breaker: bool,
    disable_partition_level_failover: bool,
    enable_upgrade_consistency_to_local_quorum: bool,
    /// The desired throughput bucket for the client
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    throughput_bucket: Option<usize>,
    pub(crate) session_retry_options: SessionRetryOptions,
    /// Priority based execution allows users to set a priority for each request. Once the user has reached their provisioned throughput, low priority requests are throttled
    /// before high priority requests start getting throttled. Feature must first be enabled at the account level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    priority: Option<PriorityLevel>,
    /// Additional headers to be included in the query request. This allows for custom headers beyond those natively supported.
    /// The following are some example headers that can be added using this api.
    /// Dedicated gateway cache staleness: "x-ms-dedicatedgateway-max-age".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#adjust-maxintegratedcachestaleness for more info.
    /// Bypass dedicated gateway cache: "x-ms-dedicatedgateway-bypass-cache".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#bypass-the-integrated-cache for more info.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl CosmosClientOptions {
    /// Gets the client options.
    pub fn client_options(&self) -> &ClientOptions {
        &self.client_options
    }

    /// Sets the client options.
    pub fn with_client_options(mut self, client_options: ClientOptions) -> Self {
        self.client_options = client_options;
        self
    }

    /// Gets the application name.
    pub fn application_name(&self) -> Option<&str> {
        self.application_name.as_deref()
    }

    /// Sets the application name.
    pub fn with_application_name(mut self, application_name: impl Into<String>) -> Self {
        self.application_name = Some(application_name.into());
        self
    }

    /// Gets the application region.
    pub fn application_region(&self) -> Option<&RegionName> {
        self.application_region.as_ref()
    }

    /// Sets the application region.
    pub fn with_application_region(mut self, application_region: RegionName) -> Self {
        self.application_region = Some(application_region);
        self
    }

    /// Returns `true` if fault injection is enabled.
    #[cfg(feature = "fault_injection")]
    pub fn fault_injection_enabled(&self) -> bool {
        self.fault_injection_enabled
    }

    /// Enables or disables fault injection.
    #[cfg(feature = "fault_injection")]
    pub fn with_fault_injection_enabled(mut self, enabled: bool) -> Self {
        self.fault_injection_enabled = enabled;
        self
    }

    /// Gets the application preferred regions.
    pub fn application_preferred_regions(&self) -> &[RegionName] {
        &self.application_preferred_regions
    }

    /// Sets the application preferred regions.
    pub fn with_application_preferred_regions(mut self, regions: Vec<RegionName>) -> Self {
        self.application_preferred_regions = regions;
        self
    }

    /// Gets the excluded regions.
    pub fn excluded_regions(&self) -> &[RegionName] {
        &self.excluded_regions
    }

    /// Sets the excluded regions.
    pub fn with_excluded_regions(mut self, regions: Vec<RegionName>) -> Self {
        self.excluded_regions = regions;
        self
    }

    /// Gets the account initialization custom endpoints.
    pub fn account_initialization_custom_endpoints(&self) -> Option<&HashSet<String>> {
        self.account_initialization_custom_endpoints.as_ref()
    }

    /// Sets the account initialization custom endpoints.
    pub fn with_account_initialization_custom_endpoints(
        mut self,
        endpoints: HashSet<String>,
    ) -> Self {
        self.account_initialization_custom_endpoints = Some(endpoints);
        self
    }

    /// Gets the consistency level.
    pub fn consistency_level(&self) -> Option<&ConsistencyLevel> {
        self.consistency_level.as_ref()
    }

    /// Sets the consistency level.
    pub fn with_consistency_level(mut self, level: ConsistencyLevel) -> Self {
        self.consistency_level = Some(level);
        self
    }

    /// Gets the request timeout.
    pub fn request_timeout(&self) -> Option<Duration> {
        self.request_timeout
    }

    /// Sets the request timeout.
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Returns `true` if remote region preferred for session retry is enabled.
    pub fn enable_remote_region_preferred_for_session_retry(&self) -> bool {
        self.enable_remote_region_preferred_for_session_retry
    }

    /// Sets whether remote region preferred for session retry is enabled.
    pub fn with_enable_remote_region_preferred_for_session_retry(mut self, enabled: bool) -> Self {
        self.enable_remote_region_preferred_for_session_retry = enabled;
        self
    }

    /// Returns `true` if partition level circuit breaker is enabled.
    pub fn enable_partition_level_circuit_breaker(&self) -> bool {
        self.enable_partition_level_circuit_breaker
    }

    /// Sets whether partition level circuit breaker is enabled.
    pub fn with_enable_partition_level_circuit_breaker(mut self, enabled: bool) -> Self {
        self.enable_partition_level_circuit_breaker = enabled;
        self
    }

    /// Returns `true` if partition level failover is disabled.
    pub fn disable_partition_level_failover(&self) -> bool {
        self.disable_partition_level_failover
    }

    /// Sets whether partition level failover is disabled.
    pub fn with_disable_partition_level_failover(mut self, disabled: bool) -> Self {
        self.disable_partition_level_failover = disabled;
        self
    }

    /// Returns `true` if upgrade consistency to local quorum is enabled.
    pub fn enable_upgrade_consistency_to_local_quorum(&self) -> bool {
        self.enable_upgrade_consistency_to_local_quorum
    }

    /// Sets whether upgrade consistency to local quorum is enabled.
    pub fn with_enable_upgrade_consistency_to_local_quorum(mut self, enabled: bool) -> Self {
        self.enable_upgrade_consistency_to_local_quorum = enabled;
        self
    }

    /// Gets the throughput bucket.
    pub fn throughput_bucket(&self) -> Option<usize> {
        self.throughput_bucket
    }

    /// Sets the throughput bucket.
    pub fn with_throughput_bucket(mut self, bucket: usize) -> Self {
        self.throughput_bucket = Some(bucket);
        self
    }

    /// Gets the session retry options.
    pub fn session_retry_options(&self) -> &SessionRetryOptions {
        &self.session_retry_options
    }

    /// Sets the session retry options.
    pub fn with_session_retry_options(mut self, options: SessionRetryOptions) -> Self {
        self.session_retry_options = options;
        self
    }

    /// Gets the priority level.
    pub fn priority(&self) -> Option<&PriorityLevel> {
        self.priority.as_ref()
    }

    /// Sets the priority level.
    pub fn with_priority(mut self, priority: PriorityLevel) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers(&self) -> &HashMap<HeaderName, HeaderValue> {
        &self.custom_headers
    }

    /// Sets the custom headers.
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = headers;
        self
    }
}

impl AsHeaders for CosmosClientOptions {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let mut headers = Vec::new();

        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.push((header_name.clone(), header_value.clone()));
        }

        if let Some(consistency_level) = &self.consistency_level {
            headers.push((
                constants::CONSISTENCY_LEVEL,
                consistency_level.to_string().into(),
            ));
        }

        if let Some(priority) = &self.priority {
            headers.push((constants::PRIORITY_LEVEL, priority.to_string().into()));
        }

        if let Some(throughput_bucket) = &self.throughput_bucket {
            headers.push((
                constants::THROUGHPUT_BUCKET,
                throughput_bucket.to_string().into(),
            ));
        }

        Ok(headers.into_iter())
    }
}

/// SessionRetryOptions is used to configure retry behavior for session consistency scenarios.
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct SessionRetryOptions {
    /// Minimum retry time for 404/1002 retries within each region for read and write operations.
    /// The minimum value is 100ms. Default is 500ms.
    min_in_region_retry_time: Duration,
    /// Maximum number of retries within each region for read and write operations. Minimum is 1.
    max_in_region_retry_count: usize,
    /// Hints to SDK-internal retry policies on how early to switch retries to a different region.
    /// If true, will retry all replicas once and add a minimum delay before switching to the next region.
    /// If false, will retry in the local region up to 5s.
    remote_region_preferred: bool,
}

impl SessionRetryOptions {
    /// Gets the minimum in-region retry time.
    pub fn min_in_region_retry_time(&self) -> Duration {
        self.min_in_region_retry_time
    }

    /// Sets the minimum in-region retry time.
    pub fn with_min_in_region_retry_time(mut self, time: Duration) -> Self {
        self.min_in_region_retry_time = time;
        self
    }

    /// Gets the maximum in-region retry count.
    pub fn max_in_region_retry_count(&self) -> usize {
        self.max_in_region_retry_count
    }

    /// Sets the maximum in-region retry count.
    pub fn with_max_in_region_retry_count(mut self, count: usize) -> Self {
        self.max_in_region_retry_count = count;
        self
    }

    /// Returns `true` if remote region preferred is set.
    pub fn remote_region_preferred(&self) -> bool {
        self.remote_region_preferred
    }

    /// Sets whether remote region is preferred.
    pub fn with_remote_region_preferred(mut self, preferred: bool) -> Self {
        self.remote_region_preferred = preferred;
        self
    }
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct CreateContainerOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
    pub(crate) throughput: Option<ThroughputProperties>,
}

impl<'a> CreateContainerOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }

    /// Gets the throughput properties.
    pub fn throughput(&self) -> Option<&ThroughputProperties> {
        self.throughput.as_ref()
    }

    /// Sets the throughput properties.
    pub fn with_throughput(mut self, throughput: ThroughputProperties) -> Self {
        self.throughput = Some(throughput);
        self
    }
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct ReplaceContainerOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> ReplaceContainerOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct CreateDatabaseOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
    pub(crate) throughput: Option<ThroughputProperties>,
}

impl<'a> CreateDatabaseOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }

    /// Gets the throughput properties.
    pub fn throughput(&self) -> Option<&ThroughputProperties> {
        self.throughput.as_ref()
    }

    /// Sets the throughput properties.
    pub fn with_throughput(mut self, throughput: ThroughputProperties) -> Self {
        self.throughput = Some(throughput);
        self
    }
}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct DeleteContainerOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> DeleteContainerOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct DeleteDatabaseOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> DeleteDatabaseOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Specifies consistency levels that can be used when working with Cosmos APIs.
///
/// Learn more at [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
#[derive(Clone, Debug)]
pub enum ConsistencyLevel {
    ConsistentPrefix,
    Eventual,
    Session,
    BoundedStaleness,
    Strong,
}

impl Display for ConsistencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ConsistencyLevel::ConsistentPrefix => "ConsistentPrefix",
            ConsistencyLevel::Eventual => "Eventual",
            ConsistencyLevel::Session => "Session",
            ConsistencyLevel::BoundedStaleness => "BoundedStaleness",
            ConsistencyLevel::Strong => "Strong",
        };
        write!(f, "{}", value)
    }
}

/// Priority based execution allows users to set a priority for each request. Once the user has reached their provisioned throughput, low priority requests are throttled
/// before high priority requests start getting throttled. Feature must first be enabled at the account level.
///
/// Learn more at [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution)
#[derive(Clone, Debug)]
pub enum PriorityLevel {
    High,
    Low,
}

impl Display for PriorityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PriorityLevel::High => "High",
            PriorityLevel::Low => "Low",
        };
        write!(f, "{}", value)
    }
}

/// Specifies indexing directives that can be used when working with Cosmos APIs.
#[derive(Clone)]
pub enum IndexingDirective {
    Default,
    Include,
    Exclude,
}

impl Display for IndexingDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            IndexingDirective::Default => "Default",
            IndexingDirective::Include => "Include",
            IndexingDirective::Exclude => "Exclude",
        };
        write!(f, "{}", value)
    }
}

/// Options to be passed to APIs that manipulate items.
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct ItemOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
    /// Triggers executed before the operation.
    pub(crate) pre_triggers: Option<Vec<String>>,
    /// Triggers executed after the operation.
    pub(crate) post_triggers: Option<Vec<String>>,
    /// Applies when working with Session consistency.
    pub(crate) session_token: Option<SessionToken>,
    /// Used to specify the consistency level for the operation.
    pub(crate) consistency_level: Option<ConsistencyLevel>,
    /// Sets indexing directive for the operation.
    pub(crate) indexing_directive: Option<IndexingDirective>,
    /// If specified, the operation will only be performed if the item matches the provided Etag.
    pub(crate) if_match_etag: Option<Etag>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    pub(crate) enable_content_response_on_write: bool,
    /// The desired throughput bucket for this request.
    pub(crate) throughput_bucket: Option<usize>,
    /// Priority based execution allows users to set a priority for each request.
    pub(crate) priority: Option<PriorityLevel>,
    /// Additional headers to be included in the request.
    pub(crate) custom_headers: HashMap<HeaderName, HeaderValue>,
    /// Regions to be skipped from regional routing preferences.
    pub(crate) excluded_regions: Option<Vec<RegionName>>,
}

impl<'a> ItemOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }

    /// Gets the pre-triggers.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub fn pre_triggers(&self) -> Option<&[String]> {
        self.pre_triggers.as_deref()
    }

    /// Sets the pre-triggers.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub fn with_pre_triggers(mut self, triggers: Vec<String>) -> Self {
        self.pre_triggers = Some(triggers);
        self
    }

    /// Gets the post-triggers.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub fn post_triggers(&self) -> Option<&[String]> {
        self.post_triggers.as_deref()
    }

    /// Sets the post-triggers.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub fn with_post_triggers(mut self, triggers: Vec<String>) -> Self {
        self.post_triggers = Some(triggers);
        self
    }

    /// Gets the session token.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Sets the session token.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub fn with_session_token(mut self, token: SessionToken) -> Self {
        self.session_token = Some(token);
        self
    }

    /// Gets the consistency level.
    ///
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub fn consistency_level(&self) -> Option<&ConsistencyLevel> {
        self.consistency_level.as_ref()
    }

    /// Sets the consistency level.
    ///
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub fn with_consistency_level(mut self, level: ConsistencyLevel) -> Self {
        self.consistency_level = Some(level);
        self
    }

    /// Gets the indexing directive.
    pub fn indexing_directive(&self) -> Option<&IndexingDirective> {
        self.indexing_directive.as_ref()
    }

    /// Sets the indexing directive.
    pub fn with_indexing_directive(mut self, directive: IndexingDirective) -> Self {
        self.indexing_directive = Some(directive);
        self
    }

    /// Gets the if-match ETag.
    ///
    /// See [Optimistic Concurrency Control](https://learn.microsoft.com/azure/cosmos-db/nosql/database-transactions-optimistic-concurrency#optimistic-concurrency-control) for more.
    pub fn if_match_etag(&self) -> Option<&Etag> {
        self.if_match_etag.as_ref()
    }

    /// Sets the if-match ETag.
    ///
    /// See [Optimistic Concurrency Control](https://learn.microsoft.com/azure/cosmos-db/nosql/database-transactions-optimistic-concurrency#optimistic-concurrency-control) for more.
    pub fn with_if_match_etag(mut self, etag: Etag) -> Self {
        self.if_match_etag = Some(etag);
        self
    }

    /// Returns `true` if content response on write is enabled.
    pub fn enable_content_response_on_write(&self) -> bool {
        self.enable_content_response_on_write
    }

    /// Sets whether the write response should include the full content.
    pub fn with_enable_content_response_on_write(mut self, enabled: bool) -> Self {
        self.enable_content_response_on_write = enabled;
        self
    }

    /// Gets the throughput bucket.
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub fn throughput_bucket(&self) -> Option<usize> {
        self.throughput_bucket
    }

    /// Sets the throughput bucket.
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub fn with_throughput_bucket(mut self, bucket: usize) -> Self {
        self.throughput_bucket = Some(bucket);
        self
    }

    /// Gets the priority level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub fn priority(&self) -> Option<&PriorityLevel> {
        self.priority.as_ref()
    }

    /// Sets the priority level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub fn with_priority(mut self, priority: PriorityLevel) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers(&self) -> &HashMap<HeaderName, HeaderValue> {
        &self.custom_headers
    }

    /// Sets the custom headers.
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = headers;
        self
    }

    /// Gets the excluded regions.
    pub fn excluded_regions(&self) -> Option<&[RegionName]> {
        self.excluded_regions.as_deref()
    }

    /// Sets the excluded regions.
    pub fn with_excluded_regions(mut self, regions: Vec<RegionName>) -> Self {
        self.excluded_regions = Some(regions);
        self
    }
}

impl AsHeaders for ItemOptions<'_> {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let mut headers = Vec::new();

        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.push((header_name.clone(), header_value.clone()));
        }

        if let Some(pre_triggers) = &self.pre_triggers {
            headers.push((
                constants::PRE_TRIGGER_INCLUDE,
                pre_triggers.join(",").into(),
            ));
        }

        if let Some(post_triggers) = &self.post_triggers {
            headers.push((
                constants::POST_TRIGGER_INCLUDE,
                post_triggers.join(",").into(),
            ));
        }

        if let Some(session_token) = &self.session_token {
            headers.push((constants::SESSION_TOKEN, session_token.to_string().into()));
        }

        if let Some(consistency_level) = &self.consistency_level {
            headers.push((
                constants::CONSISTENCY_LEVEL,
                consistency_level.to_string().into(),
            ));
        }

        if let Some(indexing_directive) = &self.indexing_directive {
            headers.push((
                constants::INDEXING_DIRECTIVE,
                indexing_directive.to_string().into(),
            ));
        }

        if let Some(etag) = &self.if_match_etag {
            headers.push((headers::IF_MATCH, etag.to_string().into()));
        }

        if let Some(priority) = &self.priority {
            headers.push((constants::PRIORITY_LEVEL, priority.to_string().into()));
        }

        if let Some(throughput_bucket) = &self.throughput_bucket {
            headers.push((
                constants::THROUGHPUT_BUCKET,
                throughput_bucket.to_string().into(),
            ));
        }

        if !self.enable_content_response_on_write {
            headers.push((headers::PREFER, constants::PREFER_MINIMAL));
        }

        Ok(headers.into_iter())
    }
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers())
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct QueryContainersOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> QueryContainersOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct QueryDatabasesOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> QueryDatabasesOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct QueryOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,

    /// Applies when working with Session consistency.
    pub(crate) session_token: Option<SessionToken>,
    /// Used to specify the consistency level for the operation.
    pub(crate) consistency_level: Option<ConsistencyLevel>,
    /// The desired throughput bucket for this query operation.
    pub(crate) throughput_bucket: Option<usize>,
    /// Priority based execution for this request.
    pub(crate) priority: Option<PriorityLevel>,
    /// Additional headers to be included in the query request.
    pub(crate) custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl<'a> QueryOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }

    /// Gets the session token.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Sets the session token.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub fn with_session_token(mut self, token: SessionToken) -> Self {
        self.session_token = Some(token);
        self
    }

    /// Gets the consistency level.
    ///
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub fn consistency_level(&self) -> Option<&ConsistencyLevel> {
        self.consistency_level.as_ref()
    }

    /// Sets the consistency level.
    ///
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub fn with_consistency_level(mut self, level: ConsistencyLevel) -> Self {
        self.consistency_level = Some(level);
        self
    }

    /// Gets the throughput bucket.
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub fn throughput_bucket(&self) -> Option<usize> {
        self.throughput_bucket
    }

    /// Sets the throughput bucket.
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub fn with_throughput_bucket(mut self, bucket: usize) -> Self {
        self.throughput_bucket = Some(bucket);
        self
    }

    /// Gets the priority level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub fn priority(&self) -> Option<&PriorityLevel> {
        self.priority.as_ref()
    }

    /// Sets the priority level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub fn with_priority(mut self, priority: PriorityLevel) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Gets the custom headers.
    pub fn custom_headers(&self) -> &HashMap<HeaderName, HeaderValue> {
        &self.custom_headers
    }

    /// Sets the custom headers.
    pub fn with_custom_headers(mut self, headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = headers;
        self
    }
}

impl QueryOptions<'_> {
    pub fn into_owned(self) -> QueryOptions<'static> {
        QueryOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
            session_token: self.session_token,
            consistency_level: self.consistency_level,
            throughput_bucket: self.throughput_bucket,
            priority: self.priority,
            custom_headers: self.custom_headers,
        }
    }
}

impl AsHeaders for QueryOptions<'_> {
    type Error = Infallible;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let mut headers = Vec::new();

        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.push((header_name.clone(), header_value.clone()));
        }

        if let Some(session_token) = &self.session_token {
            headers.push((constants::SESSION_TOKEN, session_token.to_string().into()));
        }

        if let Some(consistency_level) = &self.consistency_level {
            headers.push((
                constants::CONSISTENCY_LEVEL,
                consistency_level.to_string().into(),
            ));
        }

        if let Some(priority) = &self.priority {
            headers.push((constants::PRIORITY_LEVEL, priority.to_string().into()));
        }

        if let Some(throughput_bucket) = &self.throughput_bucket {
            headers.push((
                constants::THROUGHPUT_BUCKET,
                throughput_bucket.to_string().into(),
            ));
        }

        Ok(headers.into_iter())
    }
}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct ReadContainerOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> ReadContainerOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct ReadDatabaseOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> ReadDatabaseOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

/// Options to be passed to operations related to Throughput offers.
#[non_exhaustive]
#[derive(Clone, Default)]
pub struct ThroughputOptions<'a> {
    pub(crate) method_options: ClientMethodOptions<'a>,
}

impl<'a> ThroughputOptions<'a> {
    /// Gets the method options.
    pub fn method_options(&self) -> &ClientMethodOptions<'a> {
        &self.method_options
    }

    /// Sets the method options.
    pub fn with_method_options(mut self, method_options: ClientMethodOptions<'a>) -> Self {
        self.method_options = method_options;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers_to_map(headers: Vec<(HeaderName, HeaderValue)>) -> HashMap<HeaderName, HeaderValue> {
        headers.into_iter().collect()
    }

    #[test]
    fn item_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let item_options = ItemOptions {
            pre_triggers: Some(vec!["PreTrigger1".to_string(), "PreTrigger2".to_string()]),
            post_triggers: Some(vec!["PostTrigger1".to_string(), "PostTrigger2".to_string()]),
            session_token: Some("SessionToken".to_string().into()),
            consistency_level: Some(ConsistencyLevel::Session),
            indexing_directive: Some(IndexingDirective::Include),
            if_match_etag: Some(Etag::from("etag_value")),
            enable_content_response_on_write: false,
            priority: Some(PriorityLevel::High),
            throughput_bucket: Some(2),
            custom_headers,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            (
                constants::PRE_TRIGGER_INCLUDE,
                "PreTrigger1,PreTrigger2".into(),
            ),
            (
                constants::POST_TRIGGER_INCLUDE,
                "PostTrigger1,PostTrigger2".into(),
            ),
            ("x-custom-header".into(), "custom_value".into()),
            (constants::CONSISTENCY_LEVEL, "Session".into()),
            (headers::IF_MATCH, "etag_value".into()),
            (constants::SESSION_TOKEN, "SessionToken".into()),
            (constants::INDEXING_DIRECTIVE, "Include".into()),
            (constants::PRIORITY_LEVEL, "High".into()),
            (constants::THROUGHPUT_BUCKET, "2".into()),
            (headers::PREFER, constants::PREFER_MINIMAL),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn custom_headers_should_not_override_sdk_set_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            constants::CONSISTENCY_LEVEL,
            HeaderValue::from_static("CustomConsistency"),
        );

        let item_options = ItemOptions {
            consistency_level: Some(ConsistencyLevel::Strong),
            custom_headers,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            (constants::CONSISTENCY_LEVEL, "Strong".into()),
            (headers::PREFER, constants::PREFER_MINIMAL),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn client_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let client_options = CosmosClientOptions {
            consistency_level: Some(ConsistencyLevel::Eventual),
            throughput_bucket: Some(5),
            priority: Some(PriorityLevel::Low),
            custom_headers,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            client_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            ("x-custom-header".into(), "custom_value".into()),
            (constants::CONSISTENCY_LEVEL, "Eventual".into()),
            (constants::PRIORITY_LEVEL, "Low".into()),
            (constants::THROUGHPUT_BUCKET, "5".into()),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn query_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let query_options = QueryOptions {
            session_token: Some("QuerySessionToken".to_string().into()),
            consistency_level: Some(ConsistencyLevel::BoundedStaleness),
            priority: Some(PriorityLevel::High),
            throughput_bucket: Some(10),
            custom_headers,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            query_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            ("x-custom-header".into(), "custom_value".into()),
            (constants::SESSION_TOKEN, "QuerySessionToken".into()),
            (constants::CONSISTENCY_LEVEL, "BoundedStaleness".into()),
            (constants::PRIORITY_LEVEL, "High".into()),
            (constants::THROUGHPUT_BUCKET, "10".into()),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn item_options_empty_as_headers_with_content_response() {
        let item_options = ItemOptions::default();

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![(headers::PREFER, constants::PREFER_MINIMAL)];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn item_options_empty_as_headers() {
        let item_options = ItemOptions {
            enable_content_response_on_write: true,
            ..Default::default()
        };

        let headers_result: Vec<(HeaderName, HeaderValue)> =
            item_options.as_headers().unwrap().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![];

        assert_eq!(headers_result, headers_expected);
    }
}
