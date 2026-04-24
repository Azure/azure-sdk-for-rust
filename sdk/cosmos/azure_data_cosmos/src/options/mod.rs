// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::constants;
use crate::models::ThroughputProperties;
use azure_core::http::headers::{self, Headers};
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
    ThroughputControlGroupOptions,
};

// Temporary: these helpers allow the SDK pipeline to apply OperationOptions values
// as HTTP headers. They will be removed when individual operations use the internal
// pipeline directly.
fn apply_precondition_headers(precondition: &Precondition, headers: &mut Headers) {
    match precondition {
        Precondition::IfMatch(etag) => {
            headers.insert(headers::IF_MATCH, etag.to_string());
        }
        Precondition::IfNoneMatch(etag) => {
            headers.insert(constants::IF_NONE_MATCH, etag.to_string());
        }
        _ => {}
    }
}

// Temporary: applies the prefer header based on the content_response_on_write option.
// Will be removed when write operations use the internal pipeline directly.
fn apply_content_response_on_write_header(
    content_response_on_write: Option<&ContentResponseOnWrite>,
    headers: &mut Headers,
) {
    match content_response_on_write {
        Some(ContentResponseOnWrite::Enabled) => {}
        _ => {
            headers.insert(headers::PREFER, constants::PREFER_MINIMAL);
        }
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
    /// Default [`OperationOptions`] applied to all requests made by this client,
    /// unless overridden by per-request options.
    pub(crate) operation: OperationOptions,
    pub(crate) user_agent_suffix: Option<String>,
    pub(crate) application_region: Option<Region>,
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.user_agent_suffix = Some(suffix.into());
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

impl ItemWriteOptions {
    // Temporary: applies option values as HTTP headers for the SDK pipeline.
    // Will be removed when write operations use the internal pipeline directly.
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        if let Some(custom_headers) = self.operation.custom_headers() {
            for (name, value) in custom_headers {
                // Only insert if not already set — SDK/request headers take priority.
                if headers.get_optional_str(name).is_none() {
                    headers.insert(name.clone(), value.clone());
                }
            }
        }
        if let Some(session_token) = &self.session_token {
            headers.insert(constants::SESSION_TOKEN, session_token.to_string());
        }
        if let Some(precondition) = &self.precondition {
            apply_precondition_headers(precondition, headers);
        }
        apply_content_response_on_write_header(
            self.operation.content_response_on_write.as_ref(),
            headers,
        );
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

impl BatchOptions {
    // Temporary: applies option values as HTTP headers for the SDK pipeline.
    // Will be removed when batch operations use the internal pipeline directly.
    pub(crate) fn apply_headers(&self, headers: &mut Headers) {
        if let Some(custom_headers) = self.operation.custom_headers() {
            for (name, value) in custom_headers {
                // Only insert if not already set — SDK/request headers take priority.
                if headers.get_optional_str(name).is_none() {
                    headers.insert(name.clone(), value.clone());
                }
            }
        }
        if let Some(session_token) = &self.session_token {
            headers.insert(constants::SESSION_TOKEN, session_token.to_string());
        }
        apply_content_response_on_write_header(
            self.operation.content_response_on_write.as_ref(),
            headers,
        );
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
    fn item_write_options_as_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let operation = OperationOptions::default().with_custom_headers(custom_headers);

        let options = ItemWriteOptions {
            operation,
            ..Default::default()
        }
        .with_session_token("SessionToken".to_string())
        .with_precondition(Precondition::IfMatch(ETag::from("etag_value")));

        let mut headers_result = Headers::new();
        options.apply_headers(&mut headers_result);

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

        let operation = OperationOptions::default().with_custom_headers(custom_headers);

        let options = ItemWriteOptions {
            operation,
            ..Default::default()
        }
        .with_session_token("RealSessionToken".to_string());

        let mut headers_result = Headers::new();
        options.apply_headers(&mut headers_result);

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

    #[test]
    fn item_write_options_default_as_headers() {
        let options = ItemWriteOptions::default();

        let mut headers_result = Headers::new();
        options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> =
            vec![(headers::PREFER, constants::PREFER_MINIMAL)];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn item_write_options_with_content_response_enabled() {
        let mut operation = OperationOptions::default();
        operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);

        let options = ItemWriteOptions {
            operation,
            ..Default::default()
        };

        let mut headers_result = Headers::new();
        options.apply_headers(&mut headers_result);
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

        let mut operation = OperationOptions::default().with_custom_headers(custom_headers);
        operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);

        let batch_options = BatchOptions {
            operation,
            ..Default::default()
        }
        .with_session_token("BatchSessionToken".to_string());

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

        let operation = OperationOptions::default().with_custom_headers(custom_headers);

        let batch_options = BatchOptions {
            operation,
            ..Default::default()
        }
        .with_session_token("RealSessionToken".to_string());

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
        let mut operation = OperationOptions::default();
        operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);

        let batch_options = BatchOptions {
            operation,
            ..Default::default()
        };

        let mut headers_result = Headers::new();
        batch_options.apply_headers(&mut headers_result);
        let headers_result: Vec<(HeaderName, HeaderValue)> = headers_result.into_iter().collect();

        let headers_expected: Vec<(HeaderName, HeaderValue)> = vec![];

        assert_eq!(headers_result, headers_expected);
    }

    #[test]
    fn no_throughput_control_headers_from_apply_headers_alone() {
        // apply_headers() does not set throughput control headers — those are
        // applied by the driver pipeline. Verify that priority and bucket
        // headers are absent after apply_headers() only.
        let options = ItemWriteOptions::default();
        let mut headers = Headers::new();
        options.apply_headers(&mut headers);

        assert!(headers
            .get_optional_str(&constants::PRIORITY_LEVEL)
            .is_none());
        assert!(headers
            .get_optional_str(&constants::THROUGHPUT_BUCKET)
            .is_none());
    }
}
