use std::collections::HashMap;
use azure_core::http::{
    request::{options::ContentType, Request},
    Method,
};
use crate::cosmos_request_context::CosmosRequestContext;
use crate::{ItemOptions, PartitionKey};
use crate::operation_context::OperationType;
use crate::resource_context::ResourceType;

/// Placeholder for authorization token type.
#[derive(Clone, Debug)]
pub enum AuthorizationTokenType {
    Primary,
    Resource,
    // ... add other variants as needed
}

/// Placeholder for partition key range identity.
#[derive(Clone, Debug)]
pub struct PartitionKeyRangeIdentity {
    pub id: String,
}

impl PartitionKeyRangeIdentity {
    pub fn from_header(header: &str) -> Self {
        Self { id: header.to_string() }
    }
    pub fn to_header(&self) -> String {
        self.id.clone()
    }
}

/// Placeholder for service identity.
#[derive(Clone, Debug)]
pub struct ServiceIdentity {
    pub uri: String,
}

/// Placeholder for headers.
pub type Headers = HashMap<String, String>;

/// Main struct for DocumentServiceRequest.
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
    pub service_identity: Option<ServiceIdentity>,
    pub partition_key_range_identity: Option<PartitionKeyRangeIdentity>,
    pub request_context: CosmosRequestContext,
    pub properties: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub query_string: Option<String>,
    pub continuation: Option<String>,
    pub entity_id: Option<String>,
    pub is_disposed: bool,
}

// Add a method to create a new Request for the pipeline.
// Add DocumentServiceResponse
// Flow will look like container_client >> request_handler >> retry_handler
impl<'a> CosmosRequest<'a> {
    pub fn new(
        operation_type: OperationType,
        resource_type: ResourceType,
        resource_id: Option<String>,
        partition_key: PartitionKey,
        body: Option<Vec<u8>>,
        is_name_based: bool,
        authorization_token_type: AuthorizationTokenType,
        options: Option<ItemOptions<'a>>
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
            service_identity: None,
            partition_key_range_identity: None,
            request_context: CosmosRequestContext::default(),
            properties: HashMap::new(),
            body,
            query_string: None,
            continuation: None,
            entity_id: None,
            is_disposed: false,
        }
    }

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
            _ => Method::Post, // Default/fallback
        }
    }

    // pub fn add_prefer_header(&mut self, name: &str, value: &str) {
    //     let header_to_add = format!("{}={}", name, value);
    //     let prefer = self.headers.entry("Prefer".to_string()).or_default();
    //     if !prefer.is_empty() {
    //         prefer.push(';');
    //     }
    //     prefer.push_str(&header_to_add);
    // }

    pub fn to_raw_request(&self) -> Request {

        let mut req = Request::new(self.request_context.location_endpoint_to_route.as_ref().unwrap().clone(), self.http_method());
        req.insert_headers(&self.options).unwrap();
        req.insert_headers(&self.partition_key).unwrap();

        if !&self.is_read_only_request() {
            req.insert_headers(&ContentType::APPLICATION_JSON).unwrap();
            if let Some(ref body) = self.body {
                req.set_body(body.clone());
            }
        }

        req
    }
}