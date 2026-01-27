// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for deserializing the Cosmos DB Account (DatabaseAccount) JSON payload.
//! This is a focused representation for the sample JSON provided; fields not
//! present in the sample are intentionally omitted for now.

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

/// Represents a single regional endpoint for the Cosmos DB account (readable or writable).
#[derive(SafeDebug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegion {
    pub name: String,

    #[serde(with = "crate::serde::url")]
    pub database_account_endpoint: Url,
}

/// Describes replica set sizing characteristics for user/system replication policies.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
#[safe(true)]
// cSpell:disable
#[serde(rename_all = "camelCase")]
pub struct ReplicationPolicy {
    pub min_replica_set_size: i32,

    // Note: service returns key `maxReplicasetSize` (lowercase 's' in 'set')
    #[serde(rename = "maxReplicasetSize")]
    pub max_replica_set_size: i32,
}

/// User-configured default consistency level for the account.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ConsistencyPolicy {
    pub default_consistency_level: String,
}

/// Read preference coefficients used by the service when selecting regions.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ReadPolicy {
    pub primary_read_coefficient: i32,

    pub secondary_read_coefficient: i32,
}

/// Top-level Cosmos DB DatabaseAccount properties returned by the control plane.
///
/// This struct captures a subset of fields surfaced in the account read payload
/// (not exhaustive). It includes region lists, consistency/replication policies
/// and a raw `query_engine_configuration` JSON string for optional parsing.
#[derive(SafeDebug, Clone, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct AccountProperties {
    #[serde(rename = "_self")]
    pub self_link: String,

    /// The id of the respective account.
    pub id: String,

    /// The resource id of the respective account.
    #[serde(rename = "_rid")]
    pub rid: String,

    /// The media type of the respective account.
    pub media: String,

    /// Root relative path for the addresses endpoint (used to enumerate address/partition routing info).
    pub addresses: String,

    #[serde(rename = "_dbs")]
    /// Root relative path for the databases feed (base path when listing or creating databases).
    pub dbs: String,

    /// Regions currently accepting writes for the account (multi-master may yield >1).
    pub writable_locations: Vec<AccountRegion>,

    /// Regions from which the account can be read (includes writable regions plus any read regions).
    pub readable_locations: Vec<AccountRegion>,

    /// True when multi-master writes are enabled (more than one writable region allowed).
    pub enable_multiple_write_locations: bool,

    /// Indicates if continuous backup (point-in-time restore) is enabled for the account.
    pub continuous_backup_enabled: bool,

    /// Enables synchronous commit across N regions for stricter durability guarantees.
    pub enable_n_region_synchronous_commit: bool,

    /// Allows failover at a per-partition granularity instead of full-region only.
    pub enable_per_partition_failover_behavior: bool,

    /// User replication settings (minimum and maximum replica set sizes).
    pub user_replication_policy: ReplicationPolicy,

    /// Default consistency level configured by the user (e.g. Session, Strong).
    pub user_consistency_policy: ConsistencyPolicy,

    /// System-managed replication sizing policy (service internal settings).
    pub system_replication_policy: ReplicationPolicy,

    /// Coefficients guiding regional read preference selection.
    pub read_policy: ReadPolicy,

    /// Raw JSON string containing query engine feature/configuration flags.
    pub query_engine_configuration: String,
}

impl AccountProperties {
    /// Parses the `query_engine_configuration` JSON string into a dynamic value.
    ///
    /// Returns `None` if the value is missing or deserialization fails.
    pub fn parsed_query_engine_configuration(&self) -> Option<Value> {
        serde_json::from_str(&self.query_engine_configuration).ok()
    }
}

#[cfg(test)]
// cSpell:disable
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "_self" : "",
      "id" : "test",
      "_rid" : "test.documents.azure.com",
      "media" : "//media/",
      "addresses" : "//addresses/",
      "_dbs" : "//dbs/",
      "writableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://test-westus2.documents.azure.com:443/" } ],
      "readableLocations" : [ { "name" : "West US 2", "databaseAccountEndpoint" : "https://test-westus2.documents.azure.com:443/" } ],
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
        assert_eq!(props.id, "test");
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
