// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::constants;
use crate::models::ThroughputProperties;
use crate::regions::RegionName;
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use azure_core::http::{headers, Etag};
use azure_core::time::Duration;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Display;

/// Session tokens are intended to be opaque. They are used to ensure session consistency.
///
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
///
/// This struct is used internally by [`CosmosClientBuilder`](crate::CosmosClientBuilder).
/// Use the builder pattern via [`CosmosClient::builder()`](crate::CosmosClient::builder())
/// to configure client options.
#[derive(Clone, Default, Debug)]
pub struct CosmosClientOptions {
    pub(crate) application_name: Option<String>,
    pub(crate) application_region: Option<RegionName>,
    #[cfg(feature = "fault_injection")]
    pub(crate) fault_injection_enabled: bool,
    pub(crate) application_preferred_regions: Vec<RegionName>,
    pub(crate) excluded_regions: Vec<RegionName>,
    /// Used to specify the consistency level for the operation.
    ///
    /// The default value is the consistency level set on the Cosmos DB account.
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub(crate) consistency_level: Option<ConsistencyLevel>,
    pub(crate) request_timeout: Option<Duration>,
    pub(crate) enable_partition_level_circuit_breaker: bool,
    /// When set to true, disables partition-level failover.
    pub(crate) disable_partition_level_failover: bool,
    /// The desired throughput bucket for the client
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub(crate) throughput_bucket: Option<usize>,
    pub(crate) session_retry_options: SessionRetryOptions,
    /// Priority based execution allows users to set a priority for each request. Once the user has reached their provisioned throughput, low priority requests are throttled
    /// before high priority requests start getting throttled. Feature must first be enabled at the account level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub(crate) priority: Option<PriorityLevel>,
    /// Additional headers to be included in the query request. This allows for custom headers beyond those natively supported.
    /// The following are some example headers that can be added using this api.
    /// Dedicated gateway cache staleness: "x-ms-dedicatedgateway-max-age".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#adjust-maxintegratedcachestaleness for more info.
    /// Bypass dedicated gateway cache: "x-ms-dedicatedgateway-bypass-cache".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#bypass-the-integrated-cache for more info.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    pub(crate) custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl CosmosClientOptions {
    /// Returns the application preferred regions.
    pub fn application_preferred_regions(&self) -> &[RegionName] {
        &self.application_preferred_regions
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
#[derive(Clone, Debug, Default)]
pub struct SessionRetryOptions {
    /// Minimum retry time for 404/1002 retries within each region for read and write operations.
    /// The minimum value is 100ms. Default is 500ms.
    pub min_in_region_retry_time: Duration,
    /// Maximum number of retries within each region for read and write operations. Minimum is 1.
    pub max_in_region_retry_count: usize,
    /// Hints to SDK-internal retry policies on how early to switch retries to a different region.
    /// If true, will retry all replicas once and add a minimum delay before switching to the next region.
    /// If false, will retry in the local region up to 5s.
    pub remote_region_preferred: bool,
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Default)]
pub struct CreateContainerOptions {
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[derive(Clone, Default)]
pub struct ReplaceContainerOptions;

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Default)]
pub struct CreateDatabaseOptions {
    pub throughput: Option<ThroughputProperties>,
}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteContainerOptions;

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
pub struct DeleteDatabaseOptions;

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
#[derive(Clone, Default)]
pub struct ItemOptions {
    /// Triggers executed before the operation.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub pre_triggers: Option<Vec<String>>,
    /// Triggers executed after the operation.
    ///
    /// See [Triggers](https://learn.microsoft.com/rest/api/cosmos-db/triggers) for more.
    pub post_triggers: Option<Vec<String>>,
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub session_token: Option<SessionToken>,
    /// Used to specify the consistency level for the operation.
    ///
    /// The default value is the consistency level set on the Cosmos DB account.
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub consistency_level: Option<ConsistencyLevel>,
    /// Sets indexing directive for the operation.
    pub indexing_directive: Option<IndexingDirective>,
    /// If specified, the operation will only be performed if the item matches the provided Etag.
    ///
    /// See [Optimistic Concurrency Control](https://learn.microsoft.com/azure/cosmos-db/nosql/database-transactions-optimistic-concurrency#optimistic-concurrency-control) for more.
    pub if_match_etag: Option<Etag>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    ///
    /// The default for this is `false`, which reduces the network and CPU burden that comes from serializing and deserializing the response.
    pub enable_content_response_on_write: bool,
    /// The desired throughput bucket for this request
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub throughput_bucket: Option<usize>,
    /// Priority based execution allows users to set a priority for each request. Once the user has reached their provisioned throughput, low priority requests are throttled
    /// before high priority requests start getting throttled. Feature must first be enabled at the account level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub priority: Option<PriorityLevel>,
    /// Additional headers to be included in the query request. This allows for custom headers beyond those natively supported.
    /// The following are some example headers that can be added using this api.
    /// Dedicated gateway cache staleness: "x-ms-dedicatedgateway-max-age".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#adjust-maxintegratedcachestaleness for more info.
    /// Bypass dedicated gateway cache: "x-ms-dedicatedgateway-bypass-cache".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#bypass-the-integrated-cache for more info.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    pub custom_headers: HashMap<HeaderName, HeaderValue>,
    /// Regions to be skipped from regional routing preferences. The regions in this list are specified as the names of the Azure Cosmos locations like, 'West US', 'East US' and so on.
    /// If all regions were excluded, the primary/hub region will be used to route requests.
    /// If None is provided, client-level excluded regions will be used.
    /// If an empty vector is provided, no regions will be excluded for this request.
    pub excluded_regions: Option<Vec<RegionName>>,
}

impl AsHeaders for ItemOptions {
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
#[derive(Clone, Default)]
pub struct QueryContainersOptions;

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[derive(Clone, Default)]
pub struct QueryDatabasesOptions;

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Default)]
pub struct QueryOptions {
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    pub session_token: Option<SessionToken>,
    /// Used to specify the consistency level for the operation.
    ///
    /// The default value is the consistency level set on the Cosmos DB account.
    /// See [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels)
    pub consistency_level: Option<ConsistencyLevel>,
    /// The desired throughput bucket for this query operation
    ///
    /// See [Throughput Control in Azure Cosmos DB](https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets) for more.
    pub throughput_bucket: Option<usize>,
    /// Priority based execution allows users to set a priority for each request. Once the user has reached their provisioned throughput, low priority requests are throttled
    /// before high priority requests start getting throttled. Feature must first be enabled at the account level.
    ///
    /// See [Priority based-execution](https://learn.microsoft.com/azure/cosmos-db/priority-based-execution) for more.
    pub priority: Option<PriorityLevel>,
    /// Additional headers to be included in the query request. This allows for custom headers beyond those natively supported.
    /// The following are some example headers that can be added using this api.
    /// Dedicated gateway cache staleness: "x-ms-dedicatedgateway-max-age".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#adjust-maxintegratedcachestaleness for more info.
    /// Bypass dedicated gateway cache: "x-ms-dedicatedgateway-bypass-cache".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#bypass-the-integrated-cache for more info.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    pub custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl AsHeaders for QueryOptions {
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
#[derive(Clone, Default)]
pub struct ReadContainerOptions;

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Default)]
pub struct ReadDatabaseOptions;

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
pub struct ThroughputOptions;

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
