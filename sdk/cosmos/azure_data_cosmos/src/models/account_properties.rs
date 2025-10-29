// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Placeholder for serde_json::Value (for AdditionalProperties)
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, Default)]
pub struct AccountRegion {
    pub name: String,
    pub endpoint: String,
    pub additional_properties: HashMap<String, JsonValue>,
}

#[derive(Clone, Debug, Default)]
pub struct AccountConsistency {
    pub default_consistency_level: ConsistencyLevel,
    pub max_staleness_prefix: i32,
    pub max_staleness_interval_in_seconds: i32,
    pub additional_properties: HashMap<String, JsonValue>,
}

#[derive(Clone, Debug, Default)]
pub struct ReplicationPolicy {
    pub max_replica_set_size: i32,
    pub min_replica_set_size: i32,
    pub async_replication: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ReadPolicy {
    pub primary_read_coefficient: i32,
    pub secondary_read_coefficient: i32,
}

#[derive(Clone, Debug)]
pub struct AccountProperties {
    pub id: Option<String>,
    pub etag: Option<String>,
    pub resource_id: Option<String>,
    pub writable_regions: Vec<AccountRegion>,
    pub readable_regions: Vec<AccountRegion>,
    pub thin_client_writable_locations: Vec<AccountRegion>,
    pub thin_client_readable_locations: Vec<AccountRegion>,
    pub max_media_storage_usage_mb: Option<i64>,
    pub media_storage_usage_mb: Option<i64>,
    pub consumed_document_storage_mb: Option<i64>,
    pub reserved_document_storage_mb: Option<i64>,
    pub provisioned_document_storage_mb: Option<i64>,
    pub consistency: Option<AccountConsistency>,
    pub addresses_link: Option<String>,
    pub replication_policy: Option<ReplicationPolicy>,
    pub system_replication_policy: Option<ReplicationPolicy>,
    pub read_policy: Option<ReadPolicy>,
    pub query_engine_configuration: Option<HashMap<String, JsonValue>>,
    pub query_engine_configuration_string: Option<String>,
    pub enable_multiple_write_locations: bool,
    pub enable_partition_level_failover: Option<bool>,
    pub additional_properties: HashMap<String, JsonValue>,
}

impl Default for AccountProperties {
    fn default() -> Self {
        Self {
            id: None,
            etag: None,
            resource_id: None,
            writable_regions: Vec::new(),
            readable_regions: Vec::new(),
            thin_client_writable_locations: Vec::new(),
            thin_client_readable_locations: Vec::new(),
            max_media_storage_usage_mb: None,
            media_storage_usage_mb: None,
            consumed_document_storage_mb: None,
            reserved_document_storage_mb: None,
            provisioned_document_storage_mb: None,
            consistency: None,
            addresses_link: None,
            replication_policy: None,
            system_replication_policy: None,
            read_policy: None,
            query_engine_configuration: None,
            query_engine_configuration_string: None,
            enable_multiple_write_locations: false,
            enable_partition_level_failover: None,
            additional_properties: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConsistencyLevel {
    Strong,
    BoundedStaleness,
    Session,
    Eventual,
    ConsistentPrefix,
}

impl Default for ConsistencyLevel {
    fn default() -> Self {
        ConsistencyLevel::Session
    }
}