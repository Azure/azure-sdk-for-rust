// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::constants;
use crate::models::ThroughputProperties;
use crate::regions::RegionName;
use azure_core::http::headers::{HeaderName, HeaderValue, Headers};
use azure_core::http::{headers, Etag};
use std::collections::HashMap;
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
#[non_exhaustive]
pub struct CosmosClientOptions {
    pub(crate) user_agent_suffix: Option<String>,
    pub(crate) application_region: Option<RegionName>,
    pub(crate) custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.user_agent_suffix = Some(suffix.into());
        self
    }

    pub fn with_application_region(mut self, application_region: impl Into<RegionName>) -> Self {
        self.application_region = Some(application_region.into());
        self
    }

    pub fn with_custom_headers(mut self, custom_headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = custom_headers;
        self
    }
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        for (header_name, header_value) in &self.custom_headers {
            // Only insert if not already set — request-level headers take priority.
            if !headers.iter().any(|(n, _)| n == header_name) {
                headers.insert(header_name.clone(), header_value.clone());
            }
        }
    }
}

/// Options to be passed to [`DatabaseClient::create_container()`](crate::clients::DatabaseClient::create_container()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateContainerOptions {
    pub(crate) throughput: Option<ThroughputProperties>,
}

impl CreateContainerOptions {
    pub fn with_throughput(mut self, throughput: ThroughputProperties) -> Self {
        self.throughput = Some(throughput);
        self
    }
}

/// Options to be passed to [`ContainerClient::replace()`](crate::clients::ContainerClient::replace()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReplaceContainerOptions;

/// Options to be passed to [`CosmosClient::create_database()`](crate::CosmosClient::create_database()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateDatabaseOptions {
    pub(crate) throughput: Option<ThroughputProperties>,
}

impl CreateDatabaseOptions {
    pub fn with_throughput(mut self, throughput: ThroughputProperties) -> Self {
        self.throughput = Some(throughput);
        self
    }
}

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteContainerOptions;

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
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

/// Options to be passed to APIs that manipulate items.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemOptions {
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    session_token: Option<SessionToken>,
    /// If specified, the operation will only be performed if the item matches the provided Etag.
    ///
    /// See [Optimistic Concurrency Control](https://learn.microsoft.com/azure/cosmos-db/nosql/database-transactions-optimistic-concurrency#optimistic-concurrency-control) for more.
    if_match_etag: Option<Etag>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    ///
    /// The default for this is `false`, which reduces the network and CPU burden that comes from serializing and deserializing the response.
    content_response_on_write_enabled: bool,
    /// Additional headers to be included in the query request. This allows for custom headers beyond those natively supported.
    /// The following are some example headers that can be added using this api.
    /// Dedicated gateway cache staleness: "x-ms-dedicatedgateway-max-age".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#adjust-maxintegratedcachestaleness for more info.
    /// Bypass dedicated gateway cache: "x-ms-dedicatedgateway-bypass-cache".
    /// See https://learn.microsoft.com/azure/cosmos-db/how-to-configure-integrated-cache?tabs=dotnet#bypass-the-integrated-cache for more info.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    custom_headers: HashMap<HeaderName, HeaderValue>,
    /// Regions to be skipped from regional routing preferences. The regions in this list are specified as the names of the Azure Cosmos locations like, 'West US', 'East US' and so on.
    /// If all regions were excluded, the primary/hub region will be used to route requests.
    /// If None is provided, client-level excluded regions will be used.
    /// If an empty vector is provided, no regions will be excluded for this request.
    pub(crate) excluded_regions: Option<Vec<RegionName>>,
}

impl ItemOptions {
    pub fn with_session_token(mut self, session_token: SessionToken) -> Self {
        self.session_token = Some(session_token);
        self
    }

    pub fn with_if_match_etag(mut self, if_match_etag: Etag) -> Self {
        self.if_match_etag = Some(if_match_etag);
        self
    }

    pub fn with_content_response_on_write_enabled(
        mut self,
        content_response_on_write_enabled: bool,
    ) -> Self {
        self.content_response_on_write_enabled = content_response_on_write_enabled;
        self
    }

    pub fn with_custom_headers(mut self, custom_headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = custom_headers;
        self
    }

    pub fn with_excluded_regions(mut self, excluded_regions: Vec<RegionName>) -> Self {
        self.excluded_regions = Some(excluded_regions);
        self
    }
}

impl ItemOptions {
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.insert(header_name.clone(), header_value.clone());
        }

        if let Some(session_token) = &self.session_token {
            headers.insert(constants::SESSION_TOKEN, session_token.to_string());
        }

        if let Some(etag) = &self.if_match_etag {
            headers.insert(headers::IF_MATCH, etag.to_string());
        }

        if !self.content_response_on_write_enabled {
            headers.insert(headers::PREFER, constants::PREFER_MINIMAL);
        }
    }
}

/// Options to be passed to [`ContainerClient::execute_transactional_batch()`](crate::clients::ContainerClient::execute_transactional_batch()).
///
/// This is similar to [`ItemOptions`] but excludes ETag-based conditional options,
/// since those are specified per-operation within the batch itself.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct BatchOptions {
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    session_token: Option<SessionToken>,
    /// When this value is true, write operations will respond with the new value of the resource being written.
    ///
    /// The default for this is `false`, which reduces the network and CPU burden that comes from serializing and deserializing the response.
    content_response_on_write_enabled: bool,
    /// Additional headers to be included in the batch request. This allows for custom headers beyond those natively supported.
    ///
    /// Custom headers will not override headers that are already set by the SDK.
    custom_headers: HashMap<HeaderName, HeaderValue>,
}

impl BatchOptions {
    pub fn with_session_token(mut self, session_token: SessionToken) -> Self {
        self.session_token = Some(session_token);
        self
    }

    pub fn with_content_response_on_write_enabled(
        mut self,
        content_response_on_write_enabled: bool,
    ) -> Self {
        self.content_response_on_write_enabled = content_response_on_write_enabled;
        self
    }

    pub fn with_custom_headers(mut self, custom_headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = custom_headers;
        self
    }
}

impl BatchOptions {
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.insert(header_name.clone(), header_value.clone());
        }

        if let Some(session_token) = &self.session_token {
            headers.insert(constants::SESSION_TOKEN, session_token.to_string());
        }

        if !self.content_response_on_write_enabled {
            headers.insert(headers::PREFER, constants::PREFER_MINIMAL);
        }
    }
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers())
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryContainersOptions;

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases())
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryDatabasesOptions;

/// Options to be passed to [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions {
    /// Applies when working with Session consistency.
    /// Each new write request to Azure Cosmos DB is assigned a new Session Token.
    /// The client instance will use this token internally with each read/query request to ensure that the set consistency level is maintained.
    ///
    /// See [Session Tokens](https://learn.microsoft.com/azure/cosmos-db/nosql/how-to-manage-consistency?tabs=portal%2Cdotnetv2%2Capi-async#utilize-session-tokens) for more.
    session_token: Option<SessionToken>,
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

impl QueryOptions {
    pub fn with_session_token(mut self, session_token: SessionToken) -> Self {
        self.session_token = Some(session_token);
        self
    }

    pub fn with_custom_headers(mut self, custom_headers: HashMap<HeaderName, HeaderValue>) -> Self {
        self.custom_headers = custom_headers;
        self
    }
}

impl QueryOptions {
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        // custom headers should be added first so that they don't override SDK-set headers
        for (header_name, header_value) in &self.custom_headers {
            headers.insert(header_name.clone(), header_value.clone());
        }

        if let Some(session_token) = &self.session_token {
            headers.insert(constants::SESSION_TOKEN, session_token.to_string());
        }
    }
}

/// Options to be passed to [`ContainerClient::read()`](crate::clients::ContainerClient::read()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadContainerOptions;

/// Options to be passed to [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadDatabaseOptions;

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ThroughputOptions;

#[cfg(test)]
mod tests {
    use super::*;

    fn headers_to_map<I>(headers: I) -> HashMap<HeaderName, HeaderValue>
    where
        I: IntoIterator<Item = (HeaderName, HeaderValue)>,
    {
        headers.into_iter().collect()
    }

    #[test]
    fn item_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let item_options = ItemOptions::default()
            .with_session_token("SessionToken".to_string().into())
            .with_if_match_etag(Etag::from("etag_value"))
            .with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        item_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            ("x-custom-header".into(), "custom_value".into()),
            (constants::SESSION_TOKEN, "SessionToken".into()),
            (headers::IF_MATCH, "etag_value".into()),
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
            constants::SESSION_TOKEN,
            HeaderValue::from_static("CustomSession"),
        );

        let item_options = ItemOptions::default()
            .with_session_token("RealSessionToken".to_string().into())
            .with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        item_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            (constants::SESSION_TOKEN, "RealSessionToken".into()),
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

        let client_options = CosmosClientOptions::default().with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        client_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![("x-custom-header".into(), "custom_value".into())];

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

        let query_options = QueryOptions::default()
            .with_session_token("QuerySessionToken".to_string().into())
            .with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        query_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            ("x-custom-header".into(), "custom_value".into()),
            (constants::SESSION_TOKEN, "QuerySessionToken".into()),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn item_options_empty_as_headers_with_content_response() {
        let item_options = ItemOptions::default();

        let mut headers_result = Headers::new();
        item_options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![(headers::PREFER, constants::PREFER_MINIMAL)];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn item_options_empty_as_headers() {
        let item_options = ItemOptions::default().with_content_response_on_write_enabled(true);

        let mut headers_result = Headers::new();
        item_options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn batch_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let batch_options = BatchOptions::default()
            .with_session_token("BatchSessionToken".to_string().into())
            .with_content_response_on_write_enabled(true)
            .with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        batch_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            ("x-custom-header".into(), "custom_value".into()),
            (constants::SESSION_TOKEN, "BatchSessionToken".into()),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn batch_options_custom_headers_should_not_override_sdk_set_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            constants::SESSION_TOKEN,
            HeaderValue::from_static("CustomSession"),
        );

        let batch_options = BatchOptions::default()
            .with_session_token("RealSessionToken".to_string().into())
            .with_custom_headers(custom_headers);

        let mut headers_result = Headers::new();
        batch_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![
            (constants::SESSION_TOKEN, "RealSessionToken".into()),
            (headers::PREFER, constants::PREFER_MINIMAL),
        ];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }

    #[test]
    fn batch_options_default_as_headers() {
        let batch_options = BatchOptions::default();

        let mut headers_result = Headers::new();
        batch_options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![(headers::PREFER, constants::PREFER_MINIMAL)];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn batch_options_with_content_response_enabled() {
        let batch_options = BatchOptions::default().with_content_response_on_write_enabled(true);

        let mut headers_result = Headers::new();
        batch_options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![];

        assert_eq!(headers_result, headers_expected);
    }
}
