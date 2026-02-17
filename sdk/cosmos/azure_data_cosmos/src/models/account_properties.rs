// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for deserializing the Cosmos DB Account (DatabaseAccount) JSON payload.
//! This is a focused representation for the sample JSON provided; fields not
//! present in the sample are intentionally omitted for now.

use crate::regions::RegionName;
use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

/// Represents a single regional endpoint for the Cosmos DB account (readable or writable).
#[non_exhaustive]
#[derive(SafeDebug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct AccountRegion {
    name: RegionName,

    #[serde(with = "crate::serde::url")]
    database_account_endpoint: Url,
}

impl AccountRegion {
    /// Creates a new account region with name and endpoint.
    pub fn new(name: RegionName, database_account_endpoint: Url) -> Self {
        Self {
            name,
            database_account_endpoint,
        }
    }

    /// Gets the region name.
    pub fn name(&self) -> &RegionName {
        &self.name
    }

    /// Gets the database account endpoint for this region.
    pub fn database_account_endpoint(&self) -> &Url {
        &self.database_account_endpoint
    }
}

/// Describes replica set sizing characteristics for user/system replication policies.
#[non_exhaustive]
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[safe(true)]
// cSpell:disable
#[serde(rename_all = "camelCase")]
pub struct ReplicationPolicy {
    min_replica_set_size: i32,

    // Note: service returns key `maxReplicasetSize` (lowercase 's' in 'set')
    #[serde(rename = "maxReplicasetSize")]
    max_replica_set_size: i32,
}

impl ReplicationPolicy {
    /// Gets the minimum replica set size.
    pub fn min_replica_set_size(&self) -> i32 {
        self.min_replica_set_size
    }

    /// Gets the maximum replica set size.
    pub fn max_replica_set_size(&self) -> i32 {
        self.max_replica_set_size
    }
}

/// User-configured default consistency level for the account.
#[non_exhaustive]
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ConsistencyPolicy {
    default_consistency_level: String,
}

impl ConsistencyPolicy {
    /// Gets the default consistency level.
    pub fn default_consistency_level(&self) -> &str {
        &self.default_consistency_level
    }
}

/// Read preference coefficients used by the service when selecting regions.
#[non_exhaustive]
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ReadPolicy {
    primary_read_coefficient: i32,

    secondary_read_coefficient: i32,
}

impl ReadPolicy {
    /// Gets the primary read coefficient.
    pub fn primary_read_coefficient(&self) -> i32 {
        self.primary_read_coefficient
    }

    /// Gets the secondary read coefficient.
    pub fn secondary_read_coefficient(&self) -> i32 {
        self.secondary_read_coefficient
    }
}

/// Top-level Cosmos DB DatabaseAccount properties returned by the control plane.
///
/// This struct captures a subset of fields surfaced in the account read payload
/// (not exhaustive). It includes region lists, consistency/replication policies
/// and a raw `query_engine_configuration` JSON string for optional parsing.
#[non_exhaustive]
#[derive(SafeDebug, Clone, Default, Serialize, Deserialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct AccountProperties {
    #[serde(rename = "_self")]
    self_link: String,

    /// The id of the respective account.
    id: String,

    /// The resource id of the respective account.
    #[serde(rename = "_rid")]
    rid: String,

    /// The media type of the respective account.
    media: String,

    /// Root relative path for the addresses endpoint (used to enumerate address/partition routing info).
    addresses: String,

    #[serde(rename = "_dbs")]
    /// Root relative path for the databases feed (base path when listing or creating databases).
    dbs: String,

    /// Regions currently accepting writes for the account (multi-master may yield >1).
    writable_locations: Vec<AccountRegion>,

    /// Regions from which the account can be read (includes writable regions plus any read regions).
    readable_locations: Vec<AccountRegion>,

    /// True when multi-master writes are enabled (more than one writable region allowed).
    enable_multiple_write_locations: bool,

    /// Indicates if continuous backup (point-in-time restore) is enabled for the account.
    continuous_backup_enabled: bool,

    /// Enables synchronous commit across N regions for stricter durability guarantees.
    enable_n_region_synchronous_commit: bool,

    /// Allows failover at a per-partition granularity instead of full-region only.
    enable_per_partition_failover_behavior: bool,

    /// User replication settings (minimum and maximum replica set sizes).
    user_replication_policy: ReplicationPolicy,

    /// Default consistency level configured by the user (e.g. Session, Strong).
    user_consistency_policy: ConsistencyPolicy,

    /// System-managed replication sizing policy (service internal settings).
    system_replication_policy: ReplicationPolicy,

    /// Coefficients guiding regional read preference selection.
    read_policy: ReadPolicy,

    /// Raw JSON string containing query engine feature/configuration flags.
    query_engine_configuration: String,
}

impl AccountProperties {
    /// Gets the self-link for the account.
    pub fn self_link(&self) -> &str {
        &self.self_link
    }

    /// Gets the id of the account.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the resource id of the account.
    pub fn rid(&self) -> &str {
        &self.rid
    }

    /// Gets the media type of the account.
    pub fn media(&self) -> &str {
        &self.media
    }

    /// Gets the root relative path for the addresses endpoint.
    pub fn addresses(&self) -> &str {
        &self.addresses
    }

    /// Gets the root relative path for the databases feed.
    pub fn dbs(&self) -> &str {
        &self.dbs
    }

    /// Gets the regions currently accepting writes for the account.
    pub fn writable_locations(&self) -> &[AccountRegion] {
        &self.writable_locations
    }

    /// Gets the regions from which the account can be read.
    pub fn readable_locations(&self) -> &[AccountRegion] {
        &self.readable_locations
    }

    /// Returns `true` when multi-master writes are enabled.
    pub fn enable_multiple_write_locations(&self) -> bool {
        self.enable_multiple_write_locations
    }

    /// Returns `true` if continuous backup is enabled for the account.
    pub fn continuous_backup_enabled(&self) -> bool {
        self.continuous_backup_enabled
    }

    /// Returns `true` if N-region synchronous commit is enabled.
    pub fn enable_n_region_synchronous_commit(&self) -> bool {
        self.enable_n_region_synchronous_commit
    }

    /// Returns `true` if per-partition failover behavior is enabled.
    pub fn enable_per_partition_failover_behavior(&self) -> bool {
        self.enable_per_partition_failover_behavior
    }

    /// Gets the user replication policy.
    pub fn user_replication_policy(&self) -> &ReplicationPolicy {
        &self.user_replication_policy
    }

    /// Gets the user consistency policy.
    pub fn user_consistency_policy(&self) -> &ConsistencyPolicy {
        &self.user_consistency_policy
    }

    /// Gets the system replication policy.
    pub fn system_replication_policy(&self) -> &ReplicationPolicy {
        &self.system_replication_policy
    }

    /// Gets the read policy.
    pub fn read_policy(&self) -> &ReadPolicy {
        &self.read_policy
    }

    /// Gets the raw query engine configuration JSON string.
    pub fn query_engine_configuration(&self) -> &str {
        &self.query_engine_configuration
    }

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
        assert_eq!(props.id(), "test");
        assert_eq!(props.writable_locations().len(), 1);
        assert_eq!(props.readable_locations().len(), 1);
        assert_eq!(props.user_replication_policy().min_replica_set_size(), 3);
        assert_eq!(
            props.user_consistency_policy().default_consistency_level(),
            "Session"
        );
        assert!(props.parsed_query_engine_configuration().is_some());
    }
}
