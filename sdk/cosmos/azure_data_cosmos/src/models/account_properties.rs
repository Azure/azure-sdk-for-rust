// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for deserializing the Cosmos DB Account (DatabaseAccount) JSON payload.
//! This is a focused representation for the sample JSON provided; fields not
//! present in the sample are intentionally omitted for now.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLocation {
    pub name: String,
    #[serde(rename = "databaseAccountEndpoint")]
    pub database_account_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPolicy {
    #[serde(rename = "minReplicaSetSize")]
    pub min_replica_set_size: i32,
    // Note: service returns key `maxReplicasetSize` (lowercase 's' in 'set')
    #[serde(rename = "maxReplicasetSize")]
    pub max_replica_set_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyPolicy {
    #[serde(rename = "defaultConsistencyLevel")]
    pub default_consistency_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadPolicy {
    #[serde(rename = "primaryReadCoefficient")]
    pub primary_read_coefficient: i32,
    #[serde(rename = "secondaryReadCoefficient")]
    pub secondary_read_coefficient: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountProperties {
    #[serde(rename = "_self")]
    pub self_link: String,

    pub id: String,

    #[serde(rename = "_rid")]
    pub rid: String,

    pub media: String,

    pub addresses: String,

    #[serde(rename = "_dbs")]
    pub dbs: String,

    pub writable_locations: Vec<AccountLocation>,

    pub readable_locations: Vec<AccountLocation>,

    pub enable_multiple_write_locations: bool,

    pub continuous_backup_enabled: bool,

    pub enable_n_region_synchronous_commit: bool,

    pub enable_per_partition_failover_behavior: bool,

    pub user_replication_policy: ReplicationPolicy,

    pub user_consistency_policy: ConsistencyPolicy,

    pub system_replication_policy: ReplicationPolicy,

    pub read_policy: ReadPolicy,

    pub query_engine_configuration: String,
}

impl AccountProperties {
    /// Attempts to parse the `query_engine_configuration` JSON string into a dynamic map.
    /// Returns `None` if deserialization fails.
    pub fn parsed_query_engine_configuration(&self) -> Option<Value> {
        serde_json::from_str(&self.query_engine_configuration).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
  "_self" : "",
  "id" : "ananthsessionknowledge",
  "_rid" : "ananthsessionknowledge.documents.azure.com",
  "media" : "//media/",
  "addresses" : "//addresses/",
  "_dbs" : "//dbs/",
  "writableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://ananthsessionknowledge-westus2.documents.azure.com:443/" } ],
  "readableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://ananthsessionknowledge-westus2.documents.azure.com:443/" } ],
  "enableMultipleWriteLocations" : false,
  "continuousBackupEnabled" : false,
  "enableNRegionSynchronousCommit" : false,
  "enablePerPartitionFailoverBehavior" : false,
  "userReplicationPolicy" : { "asyncReplication" : false, "minReplicaSetSize" : 3, "maxReplicasetSize" : 4 },
  "userConsistencyPolicy" : { "defaultConsistencyLevel" : "Session" },
  "systemReplicationPolicy" : { "minReplicaSetSize" : 3, "maxReplicasetSize" : 4, "asyncReplication" : false },
  "readPolicy" : { "primaryReadCoefficient" : 1, "secondaryReadCoefficient" : 1 },
  "queryEngineConfiguration" : "{\"allowNewKeywords\":true}"
}"#;

    #[test]
    fn deserialize_account_props() {
        let props: AccountProperties = serde_json::from_str(SAMPLE).expect("deserialize");
        assert_eq!(props.id, "ananthsessionknowledge");
        assert_eq!(props.writable_locations.len(), 1);
        assert_eq!(props.readable_locations.len(), 1);
        assert_eq!(props.user_replication_policy.min_replica_set_size, 3);
        assert_eq!(
            props.user_consistency_policy.default_consistency_level,
            "Session"
        );
        assert!(props.parsed_query_engine_configuration().is_some());
    }
}
