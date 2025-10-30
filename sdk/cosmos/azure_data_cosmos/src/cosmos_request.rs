// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::operation_context::OperationType;
use crate::request_context::RequestContext;
use crate::resource_context::ResourceType;
use crate::{constants, ItemOptions, PartitionKey};
use azure_core::http::{
    request::{options::ContentType, Request},
    Method,
};
use std::collections::HashMap;

/// Specifies which form of authorization token should be used when signing
/// the request. The SDK generally uses the primary key, but some operations
/// may be signed with a resource token (e.g. restricted delegation scenarios).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum AuthorizationTokenType {
    /// Use the account's primary (or secondary) key material.
    Primary,
    /// Use a resource token scoped to a particular resource (fine‑grained auth).
    Resource,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
#[derive(Clone)]
#[allow(dead_code)]
pub struct CosmosRequest<'a> {
    pub operation_type: OperationType,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub resource_address: Option<String>,
    pub database_name: Option<String>,
    pub collection_name: Option<String>,
    pub document_name: Option<String>,
    pub partition_key: PartitionKey,
    pub options: Option<ItemOptions<'a>>,
    pub is_name_based: bool,
    pub is_feed: bool,
    pub is_resource_name_parsed_from_uri: bool,
    pub use_gateway_mode: bool,
    pub use_status_code_for_failures: bool,
    pub use_status_code_for_403: bool,
    pub use_status_code_for_4041002: bool,
    pub use_status_code_for_429: bool,
    pub use_status_code_for_bad_request: bool,
    pub disable_archival_partition_not_found_retry: bool,
    pub disable_retry_with_policy: bool,
    pub force_name_cache_refresh: bool,
    pub force_partition_key_range_refresh: bool,
    pub force_collection_routing_map_refresh: bool,
    pub force_master_refresh: bool,
    pub last_collection_routing_map_hash_code: i32,
    pub request_authorization_token_type: AuthorizationTokenType,
    pub partition_key_range_identity: Option<PartitionKeyRangeIdentity>,
    pub request_context: RequestContext,
    pub properties: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub query_string: Option<String>,
    pub continuation: Option<String>,
    pub entity_id: Option<String>,
}

impl<'a> CosmosRequest<'a> {
    /// Creates a new `CosmosRequest` with core operation metadata and optional
    /// body.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_type: OperationType,
        resource_type: ResourceType,
        resource_id: Option<String>,
        partition_key: PartitionKey,
        body: Option<Vec<u8>>,
        is_name_based: bool,
        authorization_token_type: AuthorizationTokenType,
        options: Option<ItemOptions<'a>>,
    ) -> Self {
        Self {
            operation_type,
            resource_type,
            resource_id: resource_id.clone(),
            resource_address: resource_id,
            database_name: None,
            collection_name: None,
            document_name: None,
            partition_key,
            options,
            is_name_based,
            is_feed: false,
            is_resource_name_parsed_from_uri: false,
            use_gateway_mode: false,
            use_status_code_for_failures: false,
            use_status_code_for_403: false,
            use_status_code_for_4041002: false,
            use_status_code_for_429: false,
            use_status_code_for_bad_request: false,
            disable_archival_partition_not_found_retry: false,
            disable_retry_with_policy: false,
            force_name_cache_refresh: false,
            force_partition_key_range_refresh: false,
            force_collection_routing_map_refresh: false,
            force_master_refresh: false,
            last_collection_routing_map_hash_code: 0,
            request_authorization_token_type: authorization_token_type,
            partition_key_range_identity: None,
            request_context: RequestContext::default(),
            properties: HashMap::new(),
            body,
            query_string: None,
            continuation: None,
            entity_id: None,
        }
    }

    /// Returns true if the operation does not modify server state and can be
    /// treated as read-only for caching / retry heuristics.
    pub fn is_read_only_request(&self) -> bool {
        matches!(
            self.operation_type,
            OperationType::Read
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::Head
                | OperationType::HeadFeed
        )
    }

    /// Maps the logical `OperationType` to its corresponding HTTP verb.
    pub fn http_method(&self) -> Method {
        match self.operation_type {
            OperationType::Create
            | OperationType::Upsert
            | OperationType::Query
            | OperationType::SqlQuery
            | OperationType::Batch
            | OperationType::QueryPlan => Method::Post,
            OperationType::Delete => Method::Delete,
            OperationType::Read => Method::Get,
            OperationType::Replace => Method::Put,
            OperationType::Patch => Method::Patch,
            OperationType::Head | OperationType::HeadFeed => Method::Head,
            _ => Method::Post,
        }
    }

    /// Converts this `CosmosRequest` into a concrete `azure_core::http::Request`.
    ///
    /// Inserts partition key and (if present) item options as headers. For
    /// write operations, sets JSON content type, upsert header (when applicable)
    /// and attaches the body bytes. Panics if location routing information is
    /// missing from `request_context`.
    pub fn to_raw_request(&self) -> Request {
        let mut req = Request::new(
            self.request_context
                .location_endpoint_to_route
                .as_ref()
                .unwrap()
                .clone(),
            self.http_method(),
        );
        req.insert_headers(&self.options).unwrap();
        req.insert_headers(&self.partition_key).unwrap();

        if !self.is_read_only_request() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operation_context::OperationType;
    use crate::resource_context::ResourceType;
    use crate::{constants, PartitionKey};

    fn make_base_request(op: OperationType) -> CosmosRequest<'static> {
        let mut req = CosmosRequest::new(
            op,
            ResourceType::Items,
            Some("dbs/Db/colls/Coll/docs/Doc".to_string()),
            PartitionKey::from("pk"),
            Some(b"{\"id\":\"1\"}".to_vec()),
            true,
            AuthorizationTokenType::Primary,
            None,
        );
        // Provide a routing endpoint expected by to_raw_request()
        req.request_context.location_endpoint_to_route =
            Some("https://example.com/".parse().unwrap());
        req
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
        assert!(make_base_request(OperationType::Read).is_read_only_request());
        assert!(make_base_request(OperationType::Query).is_read_only_request());
        assert!(!make_base_request(OperationType::Create).is_read_only_request());
        assert!(!make_base_request(OperationType::Upsert).is_read_only_request());
    }

    #[test]
    fn to_raw_request_create_sets_headers() {
        let req = make_base_request(OperationType::Create);
        let raw = req.to_raw_request();
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
        let raw = req.to_raw_request();
        let has_upsert = raw
            .headers()
            .iter()
            .any(|(n, _)| n == &constants::IS_UPSERT);
        assert!(has_upsert);
    }

    #[test]
    fn to_raw_request_read_omits_write_headers() {
        let req = make_base_request(OperationType::Read);
        let raw = req.to_raw_request();
        // Read should not set content-type or upsert header
        let has_upsert = raw
            .headers()
            .iter()
            .any(|(n, _)| n == &constants::IS_UPSERT);
        assert!(!has_upsert);
    }
}
