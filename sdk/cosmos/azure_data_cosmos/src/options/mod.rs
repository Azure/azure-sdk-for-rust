// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::ThroughputProperties;
use azure_core::http::headers::Headers;
use std::fmt;
use std::fmt::Display;

// Re-exported types that form part of the azure_data_cosmos public API.
#[doc(inline)]
pub use azure_data_cosmos_driver::models::{
    ETag, Precondition, SessionToken, ThroughputControlGroupName,
};
#[doc(inline)]
pub use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions,
    OperationOptionsBuilder, OperationOptionsView, PriorityLevel, ReadConsistencyStrategy, Region,
    ThroughputControlGroupOptions, UserAgentSuffix,
};

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
///
/// This struct is used internally by [`CosmosClientBuilder`](crate::CosmosClientBuilder).
/// Use the builder pattern via [`CosmosClient::builder()`](crate::CosmosClient::builder())
/// to configure client options.
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    /// Default [`OperationOptions`] applied to all requests made by this client,
    /// unless overridden by per-request options.
    pub(crate) operation: OperationOptions,
    pub(crate) user_agent_suffix: Option<UserAgentSuffix>,
    pub(crate) application_region: Option<Region>,
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    // Temporary: extracts custom headers from the embedded OperationOptions and
    // applies them to the HTTP request. Will be removed when operations use the
    // internal pipeline directly.
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        if let Some(custom_headers) = self.operation.custom_headers() {
            for (header_name, header_value) in custom_headers {
                // Only insert if not already set — SDK/request headers take priority.
                if headers.get_optional_str(header_name).is_none() {
                    headers.insert(header_name.clone(), header_value.clone());
                }
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
    /// Sets the throughput properties for the new container.
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
pub struct CreateDatabaseOptions;

/// Options to be passed to [`ContainerClient::delete()`](crate::clients::ContainerClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteContainerOptions;

/// Options to be passed to [`DatabaseClient::delete()`](crate::clients::DatabaseClient::delete()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteDatabaseOptions;

/// Specifies consistency levels for Cosmos DB accounts.
///
/// This is a model type for account-level consistency properties returned by the service.
/// For per-request consistency, use [`ReadConsistencyStrategy`] via [`OperationOptions`].
///
/// Learn more at [Consistency Levels](https://learn.microsoft.com/azure/cosmos-db/consistency-levels).
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

/// Options for item point-read operations.
///
/// Used by [`ContainerClient::read_item()`](crate::clients::ContainerClient::read_item).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`operation`](Self::operation) field. See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemReadOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent reads.
    pub session_token: Option<SessionToken>,

    /// Conditional ETag check. For reads, typically [`Precondition::IfNoneMatch`]
    /// (returns 304 Not Modified if unchanged).
    pub precondition: Option<Precondition>,
}

impl ItemReadOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets a conditional ETag check for this request.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for item write operations.
///
/// Used by [`ContainerClient::create_item()`](crate::clients::ContainerClient::create_item),
/// [`ContainerClient::replace_item()`](crate::clients::ContainerClient::replace_item),
/// [`ContainerClient::upsert_item()`](crate::clients::ContainerClient::upsert_item), and
/// [`ContainerClient::delete_item()`](crate::clients::ContainerClient::delete_item).
///
/// General-purpose settings such as custom headers, excluded regions, and content
/// response behavior are configured via the [`operation`](Self::operation) field.
/// See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemWriteOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent writes.
    pub session_token: Option<SessionToken>,

    /// Conditional ETag check. For writes, typically [`Precondition::IfMatch`]
    /// (optimistic concurrency).
    pub precondition: Option<Precondition>,
}

impl ItemWriteOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets a conditional ETag check for this request.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for transactional batch operations.
///
/// Used by [`ContainerClient::execute_transactional_batch()`](crate::clients::ContainerClient::execute_transactional_batch()).
/// ETag-based conditional options are specified per-operation within the batch itself.
///
/// General-purpose settings such as custom headers and content response behavior
/// are configured via the [`operation`](Self::operation) field.
/// See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct BatchOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent batch operations.
    pub session_token: Option<SessionToken>,
}

impl BatchOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options to be passed to [`DatabaseClient::query_containers()`](crate::clients::DatabaseClient::query_containers()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryContainersOptions;

/// Options to be passed to [`CosmosClient::query_databases()`](crate::CosmosClient::query_databases()).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryDatabasesOptions;

/// Options for query operations.
///
/// Used by [`ContainerClient::query_items()`](crate::clients::ContainerClient::query_items()).
///
/// General-purpose settings such as custom headers and excluded regions are configured
/// via the [`operation`](Self::operation) field. See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent queries.
    pub session_token: Option<SessionToken>,

    /// Maximum number of items to return per page.
    ///
    /// When set, the server will return at most this many items in each response page.
    /// This is useful for controlling memory usage and for testing pagination behavior.
    /// If not set, the server uses its default page size.
    pub max_item_count: Option<u32>,
}

impl QueryOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    /// Sets the maximum number of items to return per page.
    pub fn with_max_item_count(mut self, max_item_count: u32) -> Self {
        self.max_item_count = Some(max_item_count);
        self
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

/// Options for [`ContainerClient::read_feed_ranges()`](crate::clients::ContainerClient::read_feed_ranges)
/// and [`ContainerClient::feed_range_from_partition_key()`](crate::clients::ContainerClient::feed_range_from_partition_key).
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct ReadFeedRangesOptions {
    force_refresh: bool,
}

impl ReadFeedRangesOptions {
    /// When `true`, discards any cached routing map and fetches a fresh copy from the service.
    pub fn with_force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = force_refresh;
        self
    }

    pub(crate) fn force_refresh(&self) -> bool {
        self.force_refresh
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::headers::{HeaderName, HeaderValue};
    use std::collections::HashMap;

    fn headers_to_map<I>(headers: I) -> HashMap<HeaderName, HeaderValue>
    where
        I: IntoIterator<Item = (HeaderName, HeaderValue)>,
    {
        headers.into_iter().collect()
    }

    #[test]
    fn client_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let operation = OperationOptions::default().with_custom_headers(custom_headers);

        let client_options = CosmosClientOptions {
            operation,
            ..Default::default()
        };

        let mut headers_result = Headers::new();
        client_options.apply_headers(&mut headers_result);

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![("x-custom-header".into(), "custom_value".into())];

        assert_eq!(
            headers_to_map(headers_result),
            headers_to_map(headers_expected)
        );
    }
}
