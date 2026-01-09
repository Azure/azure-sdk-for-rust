// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::operation_context::OperationType;
use crate::request_context::RequestContext;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::{constants, PartitionKey};
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue, Headers};
use azure_core::http::{
    request::{options::ContentType, Request},
    Method,
};
use serde::Serialize;

/// Specifies which form of authorization token should be used when signing
/// the request. The SDK generally uses the primary key, but some operations
/// may be signed with a resource token (e.g. restricted delegation scenarios).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorizationTokenType {
    /// Use the account's primary (or secondary) key material.
    Primary,
    /// Use a resource token scoped to a particular resource (fine‑grained auth).
    Resource,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionKeyRangeIdentity {
    pub collection_rid: String,
    pub partition_key_range_id: String,
}

/// Internal representation of a Cosmos DB operation before it is converted
/// into a wire-level `azure_core::http::Request`.
///
/// It collects operation intent (create/read/query/etc.), resource routing
/// information, partition key, optional item-level options and flags that
/// influence retry or gateway behaviors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct CosmosRequest {
    pub operation_type: OperationType,
    pub resource_type: ResourceType,
    pub resource_link: ResourceLink,
    pub resource_id: Option<String>,
    pub database_name: Option<String>,
    pub collection_name: Option<String>,
    pub document_name: Option<String>,
    pub partition_key: Option<PartitionKey>,
    pub is_feed: bool,
    pub use_gateway_mode: bool,
    pub force_name_cache_refresh: bool,
    pub force_partition_key_range_refresh: bool,
    pub force_collection_routing_map_refresh: bool,
    pub request_authorization_token_type: AuthorizationTokenType,
    pub partition_key_range_identity: Option<PartitionKeyRangeIdentity>,
    pub request_context: RequestContext,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
    pub query_string: Option<String>,
    pub continuation: Option<String>,
    pub entity_id: Option<String>,
}

impl CosmosRequest {
    fn new(operation_type: OperationType, resource_link: ResourceLink) -> Self {
        let resource_type = resource_link.resource_type();
        Self {
            operation_type,
            resource_type,
            resource_link,
            resource_id: None,
            database_name: None,
            collection_name: None,
            document_name: None,
            partition_key: Some(PartitionKey::EMPTY),
            is_feed: false,
            use_gateway_mode: false,
            force_name_cache_refresh: false,
            force_partition_key_range_refresh: false,
            force_collection_routing_map_refresh: false,
            request_authorization_token_type: AuthorizationTokenType::Primary,
            partition_key_range_identity: None,
            request_context: RequestContext::default(),
            headers: Headers::new(),
            body: Some(Vec::new()),
            query_string: None,
            continuation: None,
            entity_id: None,
        }
    }

    pub fn builder(
        operation_type: OperationType,
        resource_link: ResourceLink,
    ) -> CosmosRequestBuilder {
        CosmosRequestBuilder::new(operation_type, resource_link)
    }

    /// Determines if the given operation type is read only.
    pub fn is_read_only_request(&self) -> bool {
        self.operation_type.is_read_only()
    }

    pub fn client_headers<T: AsHeaders>(&mut self, headers: &T) {
        // Collect all headers exposed by the `AsHeaders` implementation for client options
        // always prioritize existing headers in the request over client-level headers.
        if let Ok(iter) = headers.as_headers() {
            for (name, value) in iter {
                if self.headers.get_optional_str(&name).is_none() {
                    self.headers.insert(name, value);
                }
            }
        }
    }

    /// Gets the corresponding http method for the given `OperationType`.
    pub fn http_method(&self) -> Method {
        self.operation_type.http_method()
    }

    /// Converts this `CosmosRequest` into a concrete `azure_core::http::Request`.
    ///
    /// Inserts partition key and (if present) item options as headers. For
    /// write operations, sets JSON content type, upsert header (when applicable)
    /// and attaches the body bytes. Panics if location routing information is
    /// missing from `request_context`.
    pub fn into_raw_request(self) -> Request {
        let endpoint = self
            .request_context
            .location_endpoint_to_route
            .as_ref()
            .unwrap()
            .clone();
        let url = self.resource_link.url(&endpoint);
        let mut req = Request::new(url, self.http_method());

        for (name, value) in self.headers.clone() {
            req.insert_header(name, value);
        }

        // Only insert the partition key header if one was provided. A `None`
        // partition key signifies operations where a PK is not applicable
        // (e.g. some account-level or database-level requests).
        if let Some(ref pk) = self.partition_key {
            req.insert_headers(pk).unwrap();
        }

        if !OperationType::is_read_only(&self.operation_type) {
            req.insert_headers(&ContentType::APPLICATION_JSON).unwrap();
            if self.operation_type == OperationType::Upsert {
                req.insert_header(constants::IS_UPSERT, "true");
            }
            if let Some(ref body) = self.body {
                req.set_body(body.clone());
            }
        }

        req
    }
}

/// Builder for `CosmosRequest` allowing fluent configuration while keeping
/// the original `new` constructor for backward compatibility.
#[derive(Clone)]
#[allow(dead_code)]
pub struct CosmosRequestBuilder {
    operation_type: OperationType,
    resource_link: ResourceLink,
    partition_key: PartitionKey,
    resource_id: Option<String>,
    headers: Headers,
    body: Vec<u8>,
    authorization_token_type: AuthorizationTokenType,
    continuation: Option<String>,
    entity_id: Option<String>,
    // Flags
    is_feed: bool,
    use_gateway_mode: bool,
    force_name_cache_refresh: bool,
    force_partition_key_range_refresh: bool,
    force_collection_routing_map_refresh: bool,
}

#[allow(dead_code)]
impl CosmosRequestBuilder {
    pub fn new(operation_type: OperationType, resource_link: ResourceLink) -> CosmosRequestBuilder {
        CosmosRequestBuilder {
            operation_type,
            resource_link,
            partition_key: PartitionKey::EMPTY,
            resource_id: None,
            body: Vec::new(),
            authorization_token_type: AuthorizationTokenType::Primary,
            headers: Headers::new(),
            continuation: None,
            entity_id: None,
            is_feed: false,
            use_gateway_mode: false,
            force_name_cache_refresh: false,
            force_partition_key_range_refresh: false,
            force_collection_routing_map_refresh: false,
        }
    }

    pub fn resource_id(mut self, rid: impl Into<String>) -> Self {
        self.resource_id = Some(rid.into());
        self
    }

    pub fn request_headers<T: AsHeaders>(mut self, headers: &T) -> Self {
        // Collect all headers exposed by the `AsHeaders` implementation for request options
        // If conversion fails we silently ignore (the caller can always add
        // individual headers via `header()`).
        if let Ok(iter) = headers.as_headers() {
            for (name, value) in iter {
                self.headers.insert(name, value);
            }
        }
        self
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self.headers.insert(key, value);
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn json<T: Serialize>(mut self, value: T) -> Self {
        self.body = serde_json::to_vec(&value).unwrap();
        self
    }

    pub fn authorization_token_type(mut self, ty: AuthorizationTokenType) -> Self {
        self.authorization_token_type = ty;
        self
    }

    pub fn partition_key(mut self, pk: PartitionKey) -> Self {
        self.partition_key = pk;
        self
    }

    /// Finish construction and return the immutable `CosmosRequest`.
    pub fn build(self) -> azure_core::Result<CosmosRequest> {
        let mut req = CosmosRequest::new(self.operation_type, self.resource_link);
        req.partition_key = Some(self.partition_key);
        req.request_authorization_token_type = self.authorization_token_type;
        req.body = Some(self.body);
        req.headers = self.headers;
        req.resource_id = self.resource_id;
        req.is_feed = self.is_feed;
        req.use_gateway_mode = self.use_gateway_mode;
        req.force_name_cache_refresh = self.force_name_cache_refresh;
        req.force_partition_key_range_refresh = self.force_partition_key_range_refresh;
        req.force_collection_routing_map_refresh = self.force_collection_routing_map_refresh;
        req.continuation = self.continuation;
        req.entity_id = self.entity_id;

        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::resource_context::ResourceType;
    use crate::{
        constants, ConsistencyLevel, CosmosClientOptions, ItemOptions, PartitionKey, PriorityLevel,
    };

    fn make_base_request(op: OperationType) -> CosmosRequest {
        let req = CosmosRequest::builder(op, ResourceLink::root(ResourceType::Documents))
            .resource_id("dbs/Db/colls/Coll/docs/Doc")
            .authorization_token_type(AuthorizationTokenType::Primary)
            .partition_key(PartitionKey::from("pk"))
            .body(b"{\"id\":\"1\"}".to_vec())
            .build();

        let mut req = req.unwrap();
        // Provide a routing endpoint expected by to_raw_request()
        req.request_context.location_endpoint_to_route =
            Some("https://example.com/".parse().unwrap());
        req
    }

    #[test]
    fn builder_equivalence_to_new() {
        let from_builder = CosmosRequest::builder(
            OperationType::Create,
            ResourceLink::root(ResourceType::Documents),
        )
        .resource_id("rid")
        .partition_key(PartitionKey::from("pk"))
        .body(b"{}".to_vec())
        .build();

        let builder_request = from_builder.unwrap();

        assert_eq!(OperationType::Create, builder_request.operation_type);
        assert_eq!(ResourceType::Documents, builder_request.resource_type);
        assert_eq!(Some(b"{}".to_vec()), builder_request.body);
    }

    #[test]
    fn http_method_mapping() {
        assert_eq!(
            make_base_request(OperationType::Create).http_method(),
            Method::Post
        );
        assert_eq!(
            make_base_request(OperationType::Read).http_method(),
            Method::Get
        );
        assert_eq!(
            make_base_request(OperationType::Replace).http_method(),
            Method::Put
        );
        assert_eq!(
            make_base_request(OperationType::Delete).http_method(),
            Method::Delete
        );
        assert_eq!(
            make_base_request(OperationType::Patch).http_method(),
            Method::Patch
        );
        assert_eq!(
            make_base_request(OperationType::Upsert).http_method(),
            Method::Post
        );
        assert_eq!(
            make_base_request(OperationType::Query).http_method(),
            Method::Post
        );
    }

    #[test]
    fn is_read_only_flags() {
        assert!(make_base_request(OperationType::Read)
            .operation_type
            .is_read_only());
        assert!(make_base_request(OperationType::Query)
            .operation_type
            .is_read_only());
        assert!(!make_base_request(OperationType::Create)
            .operation_type
            .is_read_only());
        assert!(!make_base_request(OperationType::Upsert)
            .operation_type
            .is_read_only());
    }

    #[test]
    fn to_raw_request_create_sets_headers() {
        let req = make_base_request(OperationType::Create);
        let raw = req.into_raw_request();
        fn header_exists(raw: &Request, name: &azure_core::http::headers::HeaderName) -> bool {
            raw.headers().iter().any(|(n, _)| n == name)
        }
        // Partition key header present
        assert!(header_exists(&raw, &constants::PARTITION_KEY));
        // Upsert header should NOT be present for Create
        assert!(!header_exists(&raw, &constants::IS_UPSERT));
    }

    #[test]
    fn to_raw_request_upsert_sets_upsert_header() {
        let req = make_base_request(OperationType::Upsert);
        let raw = req.into_raw_request();
        let has_upsert = raw
            .headers()
            .iter()
            .any(|(n, _)| n == &constants::IS_UPSERT);
        assert!(has_upsert);
    }

    #[test]
    fn prioritize_request_headers_over_client_headers() {
        let mut request_custom_headers = std::collections::HashMap::new();
        request_custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value"),
        );

        let item_options = ItemOptions {
            consistency_level: Some(ConsistencyLevel::Session),
            throughput_bucket: Some(1),
            priority: Some(PriorityLevel::Low),
            custom_headers: request_custom_headers,
            ..Default::default()
        };
        let req = CosmosRequest::builder(
            OperationType::Create,
            ResourceLink::root(ResourceType::Documents),
        )
        .request_headers(&item_options)
        .build()
        .unwrap();

        let mut req_with_client_headers = req.clone();
        req_with_client_headers
            .request_context
            .location_endpoint_to_route = Some("https://example.com/".parse().unwrap());

        let mut client_custom_headers = std::collections::HashMap::new();
        client_custom_headers.insert(
            HeaderName::from_static("x-custom-header"),
            HeaderValue::from_static("custom_value-2"),
        );

        let client_options = CosmosClientOptions {
            consistency_level: Some(ConsistencyLevel::Strong),
            throughput_bucket: Some(5),
            priority: Some(PriorityLevel::High),
            custom_headers: client_custom_headers,
            ..Default::default()
        };
        req_with_client_headers.client_headers(&client_options);

        let raw = req_with_client_headers.into_raw_request();
        let get_header = |name: &HeaderName| {
            raw.headers()
                .iter()
                .find(|(n, _)| n == &name)
                .map(|(_, v)| v.as_str())
                .unwrap()
        };

        assert_eq!(get_header(&constants::THROUGHPUT_BUCKET), "1");
        assert_eq!(get_header(&constants::PRIORITY_LEVEL), "Low");
        assert_eq!(get_header(&constants::CONSISTENCY_LEVEL), "Session");
        assert_eq!(
            get_header(&HeaderName::from_static("x-custom-header")),
            "custom_value"
        );
    }

    #[test]
    fn to_raw_request_read_omits_write_headers() {
        let req = make_base_request(OperationType::Read);
        let raw = req.into_raw_request();
        // Read should not set content-type or upsert header
        let has_upsert = raw
            .headers()
            .iter()
            .any(|(n, _)| n == &constants::IS_UPSERT);
        assert!(!has_upsert);
    }
}
