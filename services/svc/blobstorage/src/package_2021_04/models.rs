#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Access policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    #[doc = "the date-time the policy is active"]
    #[serde(rename = "Start", with = "azure_core::date::rfc3339::option")]
    pub start: Option<time::OffsetDateTime>,
    #[doc = "the date-time the policy expires"]
    #[serde(rename = "Expiry", with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<time::OffsetDateTime>,
    #[doc = "the permissions for the acl policy"]
    #[serde(rename = "Permission", default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
impl AccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccessTier")]
pub enum AccessTier {
    P4,
    P6,
    P10,
    P15,
    P20,
    P30,
    P40,
    P50,
    P60,
    P70,
    P80,
    Hot,
    Cool,
    Archive,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccessTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccessTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccessTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P4 => serializer.serialize_unit_variant("AccessTier", 0u32, "P4"),
            Self::P6 => serializer.serialize_unit_variant("AccessTier", 1u32, "P6"),
            Self::P10 => serializer.serialize_unit_variant("AccessTier", 2u32, "P10"),
            Self::P15 => serializer.serialize_unit_variant("AccessTier", 3u32, "P15"),
            Self::P20 => serializer.serialize_unit_variant("AccessTier", 4u32, "P20"),
            Self::P30 => serializer.serialize_unit_variant("AccessTier", 5u32, "P30"),
            Self::P40 => serializer.serialize_unit_variant("AccessTier", 6u32, "P40"),
            Self::P50 => serializer.serialize_unit_variant("AccessTier", 7u32, "P50"),
            Self::P60 => serializer.serialize_unit_variant("AccessTier", 8u32, "P60"),
            Self::P70 => serializer.serialize_unit_variant("AccessTier", 9u32, "P70"),
            Self::P80 => serializer.serialize_unit_variant("AccessTier", 10u32, "P80"),
            Self::Hot => serializer.serialize_unit_variant("AccessTier", 11u32, "Hot"),
            Self::Cool => serializer.serialize_unit_variant("AccessTier", 12u32, "Cool"),
            Self::Archive => serializer.serialize_unit_variant("AccessTier", 13u32, "Archive"),
            Self::Premium => serializer.serialize_unit_variant("AccessTier", 14u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArchiveStatus")]
pub enum ArchiveStatus {
    #[serde(rename = "rehydrate-pending-to-hot")]
    RehydratePendingToHot,
    #[serde(rename = "rehydrate-pending-to-cool")]
    RehydratePendingToCool,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArchiveStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArchiveStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArchiveStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RehydratePendingToHot => serializer.serialize_unit_variant("ArchiveStatus", 0u32, "rehydrate-pending-to-hot"),
            Self::RehydratePendingToCool => serializer.serialize_unit_variant("ArchiveStatus", 1u32, "rehydrate-pending-to-cool"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Groups the settings used for formatting the response if the response should be Arrow formatted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrowConfiguration {
    #[serde(rename = "Schema")]
    pub schema: Vec<ArrowField>,
}
impl ArrowConfiguration {
    pub fn new(schema: Vec<ArrowField>) -> Self {
        Self { schema }
    }
}
#[doc = "Groups settings regarding specific field of an arrow schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArrowField {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Precision", default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[serde(rename = "Scale", default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<i64>,
}
impl ArrowField {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            name: None,
            precision: None,
            scale: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobFlatListSegment {
    #[serde(rename = "BlobItems")]
    pub blob_items: Vec<BlobItemInternal>,
}
impl BlobFlatListSegment {
    pub fn new(blob_items: Vec<BlobItemInternal>) -> Self {
        Self { blob_items }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobHierarchyListSegment {
    #[serde(rename = "BlobPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub blob_prefixes: Vec<BlobPrefix>,
    #[serde(rename = "BlobItems")]
    pub blob_items: Vec<BlobItemInternal>,
}
impl BlobHierarchyListSegment {
    pub fn new(blob_items: Vec<BlobItemInternal>) -> Self {
        Self {
            blob_prefixes: Vec::new(),
            blob_items,
        }
    }
}
#[doc = "An Azure Storage blob"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobItemInternal {
    #[serde(rename = "Name")]
    pub name: BlobName,
    #[serde(rename = "Deleted")]
    pub deleted: bool,
    #[serde(rename = "Snapshot")]
    pub snapshot: String,
    #[serde(rename = "VersionId", default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "IsCurrentVersion", default, skip_serializing_if = "Option::is_none")]
    pub is_current_version: Option<bool>,
    #[doc = "Properties of a blob"]
    #[serde(rename = "Properties")]
    pub properties: BlobPropertiesInternal,
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlobMetadata>,
    #[doc = "Blob tags"]
    #[serde(rename = "BlobTags", default, skip_serializing_if = "Option::is_none")]
    pub blob_tags: Option<BlobTags>,
    #[serde(rename = "ObjectReplicationMetadata", default, skip_serializing_if = "Option::is_none")]
    pub object_replication_metadata: Option<ObjectReplicationMetadata>,
    #[serde(rename = "HasVersionsOnly", default, skip_serializing_if = "Option::is_none")]
    pub has_versions_only: Option<bool>,
}
impl BlobItemInternal {
    pub fn new(name: BlobName, deleted: bool, snapshot: String, properties: BlobPropertiesInternal) -> Self {
        Self {
            name,
            deleted,
            snapshot,
            version_id: None,
            is_current_version: None,
            properties,
            metadata: None,
            blob_tags: None,
            object_replication_metadata: None,
            has_versions_only: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobMetadata {
    #[serde(rename = "Encrypted", default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<String>,
}
impl BlobMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobName {
    #[doc = "Indicates if the blob name is encoded."]
    #[serde(rename = "Encoded", default, skip_serializing_if = "Option::is_none")]
    pub encoded: Option<bool>,
    #[doc = "The name of the blob."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl BlobName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobPrefix {
    #[serde(rename = "Name")]
    pub name: BlobName,
}
impl BlobPrefix {
    pub fn new(name: BlobName) -> Self {
        Self { name }
    }
}
#[doc = "Properties of a blob"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobPropertiesInternal {
    #[serde(rename = "Creation-Time", with = "azure_core::date::rfc1123::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[serde(rename = "Last-Modified", with = "azure_core::date::rfc1123")]
    pub last_modified: time::OffsetDateTime,
    #[serde(rename = "Etag")]
    pub etag: String,
    #[doc = "Size in bytes"]
    #[serde(rename = "Content-Length", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "Content-Type", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Content-Encoding", default, skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    #[serde(rename = "Content-Language", default, skip_serializing_if = "Option::is_none")]
    pub content_language: Option<String>,
    #[serde(rename = "Content-MD5", default, skip_serializing_if = "Option::is_none")]
    pub content_md5: Option<String>,
    #[serde(rename = "Content-Disposition", default, skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    #[serde(rename = "Cache-Control", default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "x-ms-blob-sequence-number", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_blob_sequence_number: Option<i64>,
    #[serde(rename = "BlobType", default, skip_serializing_if = "Option::is_none")]
    pub blob_type: Option<blob_properties_internal::BlobType>,
    #[serde(rename = "LeaseStatus", default, skip_serializing_if = "Option::is_none")]
    pub lease_status: Option<LeaseStatus>,
    #[serde(rename = "LeaseState", default, skip_serializing_if = "Option::is_none")]
    pub lease_state: Option<LeaseState>,
    #[serde(rename = "LeaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<LeaseDuration>,
    #[serde(rename = "CopyId", default, skip_serializing_if = "Option::is_none")]
    pub copy_id: Option<String>,
    #[serde(rename = "CopyStatus", default, skip_serializing_if = "Option::is_none")]
    pub copy_status: Option<CopyStatus>,
    #[serde(rename = "CopySource", default, skip_serializing_if = "Option::is_none")]
    pub copy_source: Option<String>,
    #[serde(rename = "CopyProgress", default, skip_serializing_if = "Option::is_none")]
    pub copy_progress: Option<String>,
    #[serde(rename = "CopyCompletionTime", with = "azure_core::date::rfc1123::option")]
    pub copy_completion_time: Option<time::OffsetDateTime>,
    #[serde(rename = "CopyStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub copy_status_description: Option<String>,
    #[serde(rename = "ServerEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub server_encrypted: Option<bool>,
    #[serde(rename = "IncrementalCopy", default, skip_serializing_if = "Option::is_none")]
    pub incremental_copy: Option<bool>,
    #[serde(rename = "DestinationSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub destination_snapshot: Option<String>,
    #[serde(rename = "DeletedTime", with = "azure_core::date::rfc1123::option")]
    pub deleted_time: Option<time::OffsetDateTime>,
    #[serde(rename = "RemainingRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<i64>,
    #[serde(rename = "AccessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<AccessTier>,
    #[serde(rename = "AccessTierInferred", default, skip_serializing_if = "Option::is_none")]
    pub access_tier_inferred: Option<bool>,
    #[serde(rename = "ArchiveStatus", default, skip_serializing_if = "Option::is_none")]
    pub archive_status: Option<ArchiveStatus>,
    #[serde(rename = "CustomerProvidedKeySha256", default, skip_serializing_if = "Option::is_none")]
    pub customer_provided_key_sha256: Option<String>,
    #[doc = "The name of the encryption scope under which the blob is encrypted."]
    #[serde(rename = "EncryptionScope", default, skip_serializing_if = "Option::is_none")]
    pub encryption_scope: Option<String>,
    #[serde(rename = "AccessTierChangeTime", with = "azure_core::date::rfc1123::option")]
    pub access_tier_change_time: Option<time::OffsetDateTime>,
    #[serde(rename = "TagCount", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    #[serde(rename = "Expiry-Time", with = "azure_core::date::rfc1123::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[serde(rename = "Sealed", default, skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
    #[doc = "If an object is in rehydrate pending state then this header is returned with priority of rehydrate. Valid values are High and Standard."]
    #[serde(rename = "RehydratePriority", default, skip_serializing_if = "Option::is_none")]
    pub rehydrate_priority: Option<RehydratePriority>,
    #[serde(rename = "LastAccessTime", with = "azure_core::date::rfc1123::option")]
    pub last_access_time: Option<time::OffsetDateTime>,
    #[serde(rename = "ImmutabilityPolicyUntilDate", with = "azure_core::date::rfc1123::option")]
    pub immutability_policy_until_date: Option<time::OffsetDateTime>,
    #[serde(rename = "ImmutabilityPolicyMode", default, skip_serializing_if = "Option::is_none")]
    pub immutability_policy_mode: Option<blob_properties_internal::ImmutabilityPolicyMode>,
    #[serde(rename = "LegalHold", default, skip_serializing_if = "Option::is_none")]
    pub legal_hold: Option<bool>,
}
impl BlobPropertiesInternal {
    pub fn new(last_modified: time::OffsetDateTime, etag: String) -> Self {
        Self {
            creation_time: None,
            last_modified,
            etag,
            content_length: None,
            content_type: None,
            content_encoding: None,
            content_language: None,
            content_md5: None,
            content_disposition: None,
            cache_control: None,
            x_ms_blob_sequence_number: None,
            blob_type: None,
            lease_status: None,
            lease_state: None,
            lease_duration: None,
            copy_id: None,
            copy_status: None,
            copy_source: None,
            copy_progress: None,
            copy_completion_time: None,
            copy_status_description: None,
            server_encrypted: None,
            incremental_copy: None,
            destination_snapshot: None,
            deleted_time: None,
            remaining_retention_days: None,
            access_tier: None,
            access_tier_inferred: None,
            archive_status: None,
            customer_provided_key_sha256: None,
            encryption_scope: None,
            access_tier_change_time: None,
            tag_count: None,
            expiry_time: None,
            sealed: None,
            rehydrate_priority: None,
            last_access_time: None,
            immutability_policy_until_date: None,
            immutability_policy_mode: None,
            legal_hold: None,
        }
    }
}
pub mod blob_properties_internal {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BlobType {
        BlockBlob,
        PageBlob,
        AppendBlob,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImmutabilityPolicyMode {
        Mutable,
        Unlocked,
        Locked,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobTag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}
impl BlobTag {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}
#[doc = "Blob tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobTags {
    #[serde(rename = "BlobTagSet")]
    pub blob_tag_set: Vec<BlobTag>,
}
impl BlobTags {
    pub fn new(blob_tag_set: Vec<BlobTag>) -> Self {
        Self { blob_tag_set }
    }
}
#[doc = "Represents a single block in a block blob.  It describes the block's ID and size."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    #[doc = "The base64 encoded block ID."]
    #[serde(rename = "Name")]
    pub name: String,
    #[doc = "The block size in bytes."]
    #[serde(rename = "Size")]
    pub size: i64,
}
impl Block {
    pub fn new(name: String, size: i64) -> Self {
        Self { name, size }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockList {
    #[serde(rename = "CommittedBlocks", default, skip_serializing_if = "Vec::is_empty")]
    pub committed_blocks: Vec<Block>,
    #[serde(rename = "UncommittedBlocks", default, skip_serializing_if = "Vec::is_empty")]
    pub uncommitted_blocks: Vec<Block>,
}
impl BlockList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockLookupList {
    #[serde(rename = "Committed", default, skip_serializing_if = "Vec::is_empty")]
    pub committed: Vec<String>,
    #[serde(rename = "Uncommitted", default, skip_serializing_if = "Vec::is_empty")]
    pub uncommitted: Vec<String>,
    #[serde(rename = "Latest", default, skip_serializing_if = "Vec::is_empty")]
    pub latest: Vec<String>,
}
impl BlockLookupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearRange {
    #[serde(rename = "Start")]
    pub start: i64,
    #[serde(rename = "End")]
    pub end: i64,
}
impl ClearRange {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}
#[doc = "An Azure Storage container"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Deleted", default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Properties of a container"]
    #[serde(rename = "Properties")]
    pub properties: ContainerProperties,
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ContainerMetadata>,
}
impl ContainerItem {
    pub fn new(name: String, properties: ContainerProperties) -> Self {
        Self {
            name,
            deleted: None,
            version: None,
            properties,
            metadata: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerMetadata {}
impl ContainerMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a container"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    #[serde(rename = "Last-Modified", with = "azure_core::date::rfc1123")]
    pub last_modified: time::OffsetDateTime,
    #[serde(rename = "Etag")]
    pub etag: String,
    #[serde(rename = "LeaseStatus", default, skip_serializing_if = "Option::is_none")]
    pub lease_status: Option<LeaseStatus>,
    #[serde(rename = "LeaseState", default, skip_serializing_if = "Option::is_none")]
    pub lease_state: Option<LeaseState>,
    #[serde(rename = "LeaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<LeaseDuration>,
    #[serde(rename = "PublicAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_access: Option<PublicAccessType>,
    #[serde(rename = "HasImmutabilityPolicy", default, skip_serializing_if = "Option::is_none")]
    pub has_immutability_policy: Option<bool>,
    #[serde(rename = "HasLegalHold", default, skip_serializing_if = "Option::is_none")]
    pub has_legal_hold: Option<bool>,
    #[serde(rename = "DefaultEncryptionScope", default, skip_serializing_if = "Option::is_none")]
    pub default_encryption_scope: Option<String>,
    #[serde(rename = "DenyEncryptionScopeOverride", default, skip_serializing_if = "Option::is_none")]
    pub deny_encryption_scope_override: Option<bool>,
    #[serde(rename = "DeletedTime", with = "azure_core::date::rfc1123::option")]
    pub deleted_time: Option<time::OffsetDateTime>,
    #[serde(rename = "RemainingRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<i64>,
    #[doc = "Indicates if version level worm is enabled on this container."]
    #[serde(rename = "ImmutableStorageWithVersioningEnabled", default, skip_serializing_if = "Option::is_none")]
    pub immutable_storage_with_versioning_enabled: Option<bool>,
}
impl ContainerProperties {
    pub fn new(last_modified: time::OffsetDateTime, etag: String) -> Self {
        Self {
            last_modified,
            etag,
            lease_status: None,
            lease_state: None,
            lease_duration: None,
            public_access: None,
            has_immutability_policy: None,
            has_legal_hold: None,
            default_encryption_scope: None,
            deny_encryption_scope_override: None,
            deleted_time: None,
            remaining_retention_days: None,
            immutable_storage_with_versioning_enabled: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CopyStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
#[doc = "CORS is an HTTP feature that enables a web application running under one domain to access resources in another domain. Web browsers implement a security restriction known as same-origin policy that prevents a web page from calling APIs in a different domain; CORS provides a secure way to allow one domain (the origin domain) to call APIs in another domain"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[doc = "The origin domains that are permitted to make a request against the storage service via CORS. The origin domain is the domain from which the request originates. Note that the origin must be an exact case-sensitive match with the origin that the user age sends to the service. You can also use the wildcard character '*' to allow all origin domains to make requests via CORS."]
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[doc = "The methods (HTTP request verbs) that the origin domain may use for a CORS request. (comma separated)"]
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[doc = "the request headers that the origin domain may specify on the CORS request."]
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[doc = "The response headers that may be sent in the response to the CORS request and exposed by the browser to the request issuer"]
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[doc = "The maximum amount time that a browser should cache the preflight OPTIONS request."]
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
impl CorsRule {
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        exposed_headers: String,
        max_age_in_seconds: i64,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            exposed_headers,
            max_age_in_seconds,
        }
    }
}
#[doc = "Groups the settings used for interpreting the blob data if the blob is delimited text formatted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DelimitedTextConfiguration {
    #[doc = "The string used to separate columns."]
    #[serde(rename = "ColumnSeparator", default, skip_serializing_if = "Option::is_none")]
    pub column_separator: Option<String>,
    #[doc = "The string used to quote a specific field."]
    #[serde(rename = "FieldQuote", default, skip_serializing_if = "Option::is_none")]
    pub field_quote: Option<String>,
    #[doc = "The string used to separate records."]
    #[serde(rename = "RecordSeparator", default, skip_serializing_if = "Option::is_none")]
    pub record_separator: Option<String>,
    #[doc = "The string used as an escape character."]
    #[serde(rename = "EscapeChar", default, skip_serializing_if = "Option::is_none")]
    pub escape_char: Option<String>,
    #[doc = "Represents whether the data has headers."]
    #[serde(rename = "HeadersPresent", default, skip_serializing_if = "Option::is_none")]
    pub headers_present: Option<bool>,
}
impl DelimitedTextConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error codes returned by the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorCode")]
pub enum ErrorCode {
    AccountAlreadyExists,
    AccountBeingCreated,
    AccountIsDisabled,
    AuthenticationFailed,
    AuthorizationFailure,
    ConditionHeadersNotSupported,
    ConditionNotMet,
    EmptyMetadataKey,
    InsufficientAccountPermissions,
    InternalError,
    InvalidAuthenticationInfo,
    InvalidHeaderValue,
    InvalidHttpVerb,
    InvalidInput,
    InvalidMd5,
    InvalidMetadata,
    InvalidQueryParameterValue,
    InvalidRange,
    InvalidResourceName,
    InvalidUri,
    InvalidXmlDocument,
    InvalidXmlNodeValue,
    Md5Mismatch,
    MetadataTooLarge,
    MissingContentLengthHeader,
    MissingRequiredQueryParameter,
    MissingRequiredHeader,
    MissingRequiredXmlNode,
    MultipleConditionHeadersNotSupported,
    OperationTimedOut,
    OutOfRangeInput,
    OutOfRangeQueryParameterValue,
    RequestBodyTooLarge,
    ResourceTypeMismatch,
    RequestUrlFailedToParse,
    ResourceAlreadyExists,
    ResourceNotFound,
    ServerBusy,
    UnsupportedHeader,
    UnsupportedXmlNode,
    UnsupportedQueryParameter,
    UnsupportedHttpVerb,
    AppendPositionConditionNotMet,
    BlobAlreadyExists,
    BlobImmutableDueToPolicy,
    BlobNotFound,
    BlobOverwritten,
    BlobTierInadequateForContentLength,
    BlobUsesCustomerSpecifiedEncryption,
    BlockCountExceedsLimit,
    BlockListTooLong,
    CannotChangeToLowerTier,
    CannotVerifyCopySource,
    ContainerAlreadyExists,
    ContainerBeingDeleted,
    ContainerDisabled,
    ContainerNotFound,
    ContentLengthLargerThanTierLimit,
    CopyAcrossAccountsNotSupported,
    CopyIdMismatch,
    FeatureVersionMismatch,
    IncrementalCopyBlobMismatch,
    IncrementalCopyOfEralierVersionSnapshotNotAllowed,
    IncrementalCopySourceMustBeSnapshot,
    InfiniteLeaseDurationRequired,
    InvalidBlobOrBlock,
    InvalidBlobTier,
    InvalidBlobType,
    InvalidBlockId,
    InvalidBlockList,
    InvalidOperation,
    InvalidPageRange,
    InvalidSourceBlobType,
    InvalidSourceBlobUrl,
    InvalidVersionForPageBlobOperation,
    LeaseAlreadyPresent,
    LeaseAlreadyBroken,
    LeaseIdMismatchWithBlobOperation,
    LeaseIdMismatchWithContainerOperation,
    LeaseIdMismatchWithLeaseOperation,
    LeaseIdMissing,
    LeaseIsBreakingAndCannotBeAcquired,
    LeaseIsBreakingAndCannotBeChanged,
    LeaseIsBrokenAndCannotBeRenewed,
    LeaseLost,
    LeaseNotPresentWithBlobOperation,
    LeaseNotPresentWithContainerOperation,
    LeaseNotPresentWithLeaseOperation,
    MaxBlobSizeConditionNotMet,
    NoAuthenticationInformation,
    NoPendingCopyOperation,
    OperationNotAllowedOnIncrementalCopyBlob,
    PendingCopyOperation,
    PreviousSnapshotCannotBeNewer,
    PreviousSnapshotNotFound,
    PreviousSnapshotOperationNotSupported,
    SequenceNumberConditionNotMet,
    SequenceNumberIncrementTooLarge,
    SnapshotCountExceeded,
    SnapshotOperationRateExceeded,
    SnapshotsPresent,
    SourceConditionNotMet,
    SystemInUse,
    TargetConditionNotMet,
    UnauthorizedBlobOverwrite,
    BlobBeingRehydrated,
    BlobArchived,
    BlobNotArchived,
    #[serde(rename = "AuthorizationSourceIPMismatch")]
    AuthorizationSourceIpMismatch,
    AuthorizationProtocolMismatch,
    AuthorizationPermissionMismatch,
    AuthorizationServiceMismatch,
    AuthorizationResourceTypeMismatch,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AccountAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 0u32, "AccountAlreadyExists"),
            Self::AccountBeingCreated => serializer.serialize_unit_variant("ErrorCode", 1u32, "AccountBeingCreated"),
            Self::AccountIsDisabled => serializer.serialize_unit_variant("ErrorCode", 2u32, "AccountIsDisabled"),
            Self::AuthenticationFailed => serializer.serialize_unit_variant("ErrorCode", 3u32, "AuthenticationFailed"),
            Self::AuthorizationFailure => serializer.serialize_unit_variant("ErrorCode", 4u32, "AuthorizationFailure"),
            Self::ConditionHeadersNotSupported => serializer.serialize_unit_variant("ErrorCode", 5u32, "ConditionHeadersNotSupported"),
            Self::ConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 6u32, "ConditionNotMet"),
            Self::EmptyMetadataKey => serializer.serialize_unit_variant("ErrorCode", 7u32, "EmptyMetadataKey"),
            Self::InsufficientAccountPermissions => serializer.serialize_unit_variant("ErrorCode", 8u32, "InsufficientAccountPermissions"),
            Self::InternalError => serializer.serialize_unit_variant("ErrorCode", 9u32, "InternalError"),
            Self::InvalidAuthenticationInfo => serializer.serialize_unit_variant("ErrorCode", 10u32, "InvalidAuthenticationInfo"),
            Self::InvalidHeaderValue => serializer.serialize_unit_variant("ErrorCode", 11u32, "InvalidHeaderValue"),
            Self::InvalidHttpVerb => serializer.serialize_unit_variant("ErrorCode", 12u32, "InvalidHttpVerb"),
            Self::InvalidInput => serializer.serialize_unit_variant("ErrorCode", 13u32, "InvalidInput"),
            Self::InvalidMd5 => serializer.serialize_unit_variant("ErrorCode", 14u32, "InvalidMd5"),
            Self::InvalidMetadata => serializer.serialize_unit_variant("ErrorCode", 15u32, "InvalidMetadata"),
            Self::InvalidQueryParameterValue => serializer.serialize_unit_variant("ErrorCode", 16u32, "InvalidQueryParameterValue"),
            Self::InvalidRange => serializer.serialize_unit_variant("ErrorCode", 17u32, "InvalidRange"),
            Self::InvalidResourceName => serializer.serialize_unit_variant("ErrorCode", 18u32, "InvalidResourceName"),
            Self::InvalidUri => serializer.serialize_unit_variant("ErrorCode", 19u32, "InvalidUri"),
            Self::InvalidXmlDocument => serializer.serialize_unit_variant("ErrorCode", 20u32, "InvalidXmlDocument"),
            Self::InvalidXmlNodeValue => serializer.serialize_unit_variant("ErrorCode", 21u32, "InvalidXmlNodeValue"),
            Self::Md5Mismatch => serializer.serialize_unit_variant("ErrorCode", 22u32, "Md5Mismatch"),
            Self::MetadataTooLarge => serializer.serialize_unit_variant("ErrorCode", 23u32, "MetadataTooLarge"),
            Self::MissingContentLengthHeader => serializer.serialize_unit_variant("ErrorCode", 24u32, "MissingContentLengthHeader"),
            Self::MissingRequiredQueryParameter => serializer.serialize_unit_variant("ErrorCode", 25u32, "MissingRequiredQueryParameter"),
            Self::MissingRequiredHeader => serializer.serialize_unit_variant("ErrorCode", 26u32, "MissingRequiredHeader"),
            Self::MissingRequiredXmlNode => serializer.serialize_unit_variant("ErrorCode", 27u32, "MissingRequiredXmlNode"),
            Self::MultipleConditionHeadersNotSupported => {
                serializer.serialize_unit_variant("ErrorCode", 28u32, "MultipleConditionHeadersNotSupported")
            }
            Self::OperationTimedOut => serializer.serialize_unit_variant("ErrorCode", 29u32, "OperationTimedOut"),
            Self::OutOfRangeInput => serializer.serialize_unit_variant("ErrorCode", 30u32, "OutOfRangeInput"),
            Self::OutOfRangeQueryParameterValue => serializer.serialize_unit_variant("ErrorCode", 31u32, "OutOfRangeQueryParameterValue"),
            Self::RequestBodyTooLarge => serializer.serialize_unit_variant("ErrorCode", 32u32, "RequestBodyTooLarge"),
            Self::ResourceTypeMismatch => serializer.serialize_unit_variant("ErrorCode", 33u32, "ResourceTypeMismatch"),
            Self::RequestUrlFailedToParse => serializer.serialize_unit_variant("ErrorCode", 34u32, "RequestUrlFailedToParse"),
            Self::ResourceAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 35u32, "ResourceAlreadyExists"),
            Self::ResourceNotFound => serializer.serialize_unit_variant("ErrorCode", 36u32, "ResourceNotFound"),
            Self::ServerBusy => serializer.serialize_unit_variant("ErrorCode", 37u32, "ServerBusy"),
            Self::UnsupportedHeader => serializer.serialize_unit_variant("ErrorCode", 38u32, "UnsupportedHeader"),
            Self::UnsupportedXmlNode => serializer.serialize_unit_variant("ErrorCode", 39u32, "UnsupportedXmlNode"),
            Self::UnsupportedQueryParameter => serializer.serialize_unit_variant("ErrorCode", 40u32, "UnsupportedQueryParameter"),
            Self::UnsupportedHttpVerb => serializer.serialize_unit_variant("ErrorCode", 41u32, "UnsupportedHttpVerb"),
            Self::AppendPositionConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 42u32, "AppendPositionConditionNotMet"),
            Self::BlobAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 43u32, "BlobAlreadyExists"),
            Self::BlobImmutableDueToPolicy => serializer.serialize_unit_variant("ErrorCode", 44u32, "BlobImmutableDueToPolicy"),
            Self::BlobNotFound => serializer.serialize_unit_variant("ErrorCode", 45u32, "BlobNotFound"),
            Self::BlobOverwritten => serializer.serialize_unit_variant("ErrorCode", 46u32, "BlobOverwritten"),
            Self::BlobTierInadequateForContentLength => {
                serializer.serialize_unit_variant("ErrorCode", 47u32, "BlobTierInadequateForContentLength")
            }
            Self::BlobUsesCustomerSpecifiedEncryption => {
                serializer.serialize_unit_variant("ErrorCode", 48u32, "BlobUsesCustomerSpecifiedEncryption")
            }
            Self::BlockCountExceedsLimit => serializer.serialize_unit_variant("ErrorCode", 49u32, "BlockCountExceedsLimit"),
            Self::BlockListTooLong => serializer.serialize_unit_variant("ErrorCode", 50u32, "BlockListTooLong"),
            Self::CannotChangeToLowerTier => serializer.serialize_unit_variant("ErrorCode", 51u32, "CannotChangeToLowerTier"),
            Self::CannotVerifyCopySource => serializer.serialize_unit_variant("ErrorCode", 52u32, "CannotVerifyCopySource"),
            Self::ContainerAlreadyExists => serializer.serialize_unit_variant("ErrorCode", 53u32, "ContainerAlreadyExists"),
            Self::ContainerBeingDeleted => serializer.serialize_unit_variant("ErrorCode", 54u32, "ContainerBeingDeleted"),
            Self::ContainerDisabled => serializer.serialize_unit_variant("ErrorCode", 55u32, "ContainerDisabled"),
            Self::ContainerNotFound => serializer.serialize_unit_variant("ErrorCode", 56u32, "ContainerNotFound"),
            Self::ContentLengthLargerThanTierLimit => {
                serializer.serialize_unit_variant("ErrorCode", 57u32, "ContentLengthLargerThanTierLimit")
            }
            Self::CopyAcrossAccountsNotSupported => serializer.serialize_unit_variant("ErrorCode", 58u32, "CopyAcrossAccountsNotSupported"),
            Self::CopyIdMismatch => serializer.serialize_unit_variant("ErrorCode", 59u32, "CopyIdMismatch"),
            Self::FeatureVersionMismatch => serializer.serialize_unit_variant("ErrorCode", 60u32, "FeatureVersionMismatch"),
            Self::IncrementalCopyBlobMismatch => serializer.serialize_unit_variant("ErrorCode", 61u32, "IncrementalCopyBlobMismatch"),
            Self::IncrementalCopyOfEralierVersionSnapshotNotAllowed => {
                serializer.serialize_unit_variant("ErrorCode", 62u32, "IncrementalCopyOfEralierVersionSnapshotNotAllowed")
            }
            Self::IncrementalCopySourceMustBeSnapshot => {
                serializer.serialize_unit_variant("ErrorCode", 63u32, "IncrementalCopySourceMustBeSnapshot")
            }
            Self::InfiniteLeaseDurationRequired => serializer.serialize_unit_variant("ErrorCode", 64u32, "InfiniteLeaseDurationRequired"),
            Self::InvalidBlobOrBlock => serializer.serialize_unit_variant("ErrorCode", 65u32, "InvalidBlobOrBlock"),
            Self::InvalidBlobTier => serializer.serialize_unit_variant("ErrorCode", 66u32, "InvalidBlobTier"),
            Self::InvalidBlobType => serializer.serialize_unit_variant("ErrorCode", 67u32, "InvalidBlobType"),
            Self::InvalidBlockId => serializer.serialize_unit_variant("ErrorCode", 68u32, "InvalidBlockId"),
            Self::InvalidBlockList => serializer.serialize_unit_variant("ErrorCode", 69u32, "InvalidBlockList"),
            Self::InvalidOperation => serializer.serialize_unit_variant("ErrorCode", 70u32, "InvalidOperation"),
            Self::InvalidPageRange => serializer.serialize_unit_variant("ErrorCode", 71u32, "InvalidPageRange"),
            Self::InvalidSourceBlobType => serializer.serialize_unit_variant("ErrorCode", 72u32, "InvalidSourceBlobType"),
            Self::InvalidSourceBlobUrl => serializer.serialize_unit_variant("ErrorCode", 73u32, "InvalidSourceBlobUrl"),
            Self::InvalidVersionForPageBlobOperation => {
                serializer.serialize_unit_variant("ErrorCode", 74u32, "InvalidVersionForPageBlobOperation")
            }
            Self::LeaseAlreadyPresent => serializer.serialize_unit_variant("ErrorCode", 75u32, "LeaseAlreadyPresent"),
            Self::LeaseAlreadyBroken => serializer.serialize_unit_variant("ErrorCode", 76u32, "LeaseAlreadyBroken"),
            Self::LeaseIdMismatchWithBlobOperation => {
                serializer.serialize_unit_variant("ErrorCode", 77u32, "LeaseIdMismatchWithBlobOperation")
            }
            Self::LeaseIdMismatchWithContainerOperation => {
                serializer.serialize_unit_variant("ErrorCode", 78u32, "LeaseIdMismatchWithContainerOperation")
            }
            Self::LeaseIdMismatchWithLeaseOperation => {
                serializer.serialize_unit_variant("ErrorCode", 79u32, "LeaseIdMismatchWithLeaseOperation")
            }
            Self::LeaseIdMissing => serializer.serialize_unit_variant("ErrorCode", 80u32, "LeaseIdMissing"),
            Self::LeaseIsBreakingAndCannotBeAcquired => {
                serializer.serialize_unit_variant("ErrorCode", 81u32, "LeaseIsBreakingAndCannotBeAcquired")
            }
            Self::LeaseIsBreakingAndCannotBeChanged => {
                serializer.serialize_unit_variant("ErrorCode", 82u32, "LeaseIsBreakingAndCannotBeChanged")
            }
            Self::LeaseIsBrokenAndCannotBeRenewed => {
                serializer.serialize_unit_variant("ErrorCode", 83u32, "LeaseIsBrokenAndCannotBeRenewed")
            }
            Self::LeaseLost => serializer.serialize_unit_variant("ErrorCode", 84u32, "LeaseLost"),
            Self::LeaseNotPresentWithBlobOperation => {
                serializer.serialize_unit_variant("ErrorCode", 85u32, "LeaseNotPresentWithBlobOperation")
            }
            Self::LeaseNotPresentWithContainerOperation => {
                serializer.serialize_unit_variant("ErrorCode", 86u32, "LeaseNotPresentWithContainerOperation")
            }
            Self::LeaseNotPresentWithLeaseOperation => {
                serializer.serialize_unit_variant("ErrorCode", 87u32, "LeaseNotPresentWithLeaseOperation")
            }
            Self::MaxBlobSizeConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 88u32, "MaxBlobSizeConditionNotMet"),
            Self::NoAuthenticationInformation => serializer.serialize_unit_variant("ErrorCode", 89u32, "NoAuthenticationInformation"),
            Self::NoPendingCopyOperation => serializer.serialize_unit_variant("ErrorCode", 90u32, "NoPendingCopyOperation"),
            Self::OperationNotAllowedOnIncrementalCopyBlob => {
                serializer.serialize_unit_variant("ErrorCode", 91u32, "OperationNotAllowedOnIncrementalCopyBlob")
            }
            Self::PendingCopyOperation => serializer.serialize_unit_variant("ErrorCode", 92u32, "PendingCopyOperation"),
            Self::PreviousSnapshotCannotBeNewer => serializer.serialize_unit_variant("ErrorCode", 93u32, "PreviousSnapshotCannotBeNewer"),
            Self::PreviousSnapshotNotFound => serializer.serialize_unit_variant("ErrorCode", 94u32, "PreviousSnapshotNotFound"),
            Self::PreviousSnapshotOperationNotSupported => {
                serializer.serialize_unit_variant("ErrorCode", 95u32, "PreviousSnapshotOperationNotSupported")
            }
            Self::SequenceNumberConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 96u32, "SequenceNumberConditionNotMet"),
            Self::SequenceNumberIncrementTooLarge => {
                serializer.serialize_unit_variant("ErrorCode", 97u32, "SequenceNumberIncrementTooLarge")
            }
            Self::SnapshotCountExceeded => serializer.serialize_unit_variant("ErrorCode", 98u32, "SnapshotCountExceeded"),
            Self::SnapshotOperationRateExceeded => serializer.serialize_unit_variant("ErrorCode", 99u32, "SnapshotOperationRateExceeded"),
            Self::SnapshotsPresent => serializer.serialize_unit_variant("ErrorCode", 100u32, "SnapshotsPresent"),
            Self::SourceConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 101u32, "SourceConditionNotMet"),
            Self::SystemInUse => serializer.serialize_unit_variant("ErrorCode", 102u32, "SystemInUse"),
            Self::TargetConditionNotMet => serializer.serialize_unit_variant("ErrorCode", 103u32, "TargetConditionNotMet"),
            Self::UnauthorizedBlobOverwrite => serializer.serialize_unit_variant("ErrorCode", 104u32, "UnauthorizedBlobOverwrite"),
            Self::BlobBeingRehydrated => serializer.serialize_unit_variant("ErrorCode", 105u32, "BlobBeingRehydrated"),
            Self::BlobArchived => serializer.serialize_unit_variant("ErrorCode", 106u32, "BlobArchived"),
            Self::BlobNotArchived => serializer.serialize_unit_variant("ErrorCode", 107u32, "BlobNotArchived"),
            Self::AuthorizationSourceIpMismatch => serializer.serialize_unit_variant("ErrorCode", 108u32, "AuthorizationSourceIPMismatch"),
            Self::AuthorizationProtocolMismatch => serializer.serialize_unit_variant("ErrorCode", 109u32, "AuthorizationProtocolMismatch"),
            Self::AuthorizationPermissionMismatch => {
                serializer.serialize_unit_variant("ErrorCode", 110u32, "AuthorizationPermissionMismatch")
            }
            Self::AuthorizationServiceMismatch => serializer.serialize_unit_variant("ErrorCode", 111u32, "AuthorizationServiceMismatch"),
            Self::AuthorizationResourceTypeMismatch => {
                serializer.serialize_unit_variant("ErrorCode", 112u32, "AuthorizationResourceTypeMismatch")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Blob info from a Filter Blobs API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterBlobItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[doc = "Blob tags"]
    #[serde(rename = "Tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BlobTags>,
}
impl FilterBlobItem {
    pub fn new(name: String, container_name: String) -> Self {
        Self {
            name,
            container_name,
            tags: None,
        }
    }
}
#[doc = "The result of a Filter Blobs API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterBlobSegment {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Where")]
    pub where_: String,
    #[serde(rename = "Blobs")]
    pub blobs: Vec<FilterBlobItem>,
    #[serde(rename = "NextMarker", default, skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}
impl FilterBlobSegment {
    pub fn new(service_endpoint: String, where_: String, blobs: Vec<FilterBlobItem>) -> Self {
        Self {
            service_endpoint,
            where_,
            blobs,
            next_marker: None,
        }
    }
}
#[doc = "Geo-Replication information for the Secondary Storage Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoReplication {
    #[doc = "The status of the secondary location"]
    #[serde(rename = "Status")]
    pub status: geo_replication::Status,
    #[doc = "A GMT date/time value, to the second. All primary writes preceding this value are guaranteed to be available for read operations at the secondary. Primary writes after this point in time may or may not be available for reads."]
    #[serde(rename = "LastSyncTime", with = "azure_core::date::rfc1123")]
    pub last_sync_time: time::OffsetDateTime,
}
impl GeoReplication {
    pub fn new(status: geo_replication::Status, last_sync_time: time::OffsetDateTime) -> Self {
        Self { status, last_sync_time }
    }
}
pub mod geo_replication {
    use super::*;
    #[doc = "The status of the secondary location"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "live")]
        Live,
        #[serde(rename = "bootstrap")]
        Bootstrap,
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Live => serializer.serialize_unit_variant("Status", 0u32, "live"),
                Self::Bootstrap => serializer.serialize_unit_variant("Status", 1u32, "bootstrap"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 2u32, "unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "json text configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonTextConfiguration {
    #[doc = "The string used to separate records."]
    #[serde(rename = "RecordSeparator", default, skip_serializing_if = "Option::is_none")]
    pub record_separator: Option<String>,
}
impl JsonTextConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyInfo {
    #[doc = "The date-time the key is active in ISO 8601 UTC time"]
    #[serde(rename = "Start")]
    pub start: String,
    #[doc = "The date-time the key expires in ISO 8601 UTC time"]
    #[serde(rename = "Expiry")]
    pub expiry: String,
}
impl KeyInfo {
    pub fn new(start: String, expiry: String) -> Self {
        Self { start, expiry }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseDuration {
    #[serde(rename = "infinite")]
    Infinite,
    #[serde(rename = "fixed")]
    Fixed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "leased")]
    Leased,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "breaking")]
    Breaking,
    #[serde(rename = "broken")]
    Broken,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseStatus {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked,
}
#[doc = "An enumeration of blobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBlobsFlatSegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "Segment")]
    pub segment: BlobFlatListSegment,
    #[serde(rename = "NextMarker", default, skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}
impl azure_core::Continuable for ListBlobsFlatSegmentResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}
impl ListBlobsFlatSegmentResponse {
    pub fn new(service_endpoint: String, container_name: String, segment: BlobFlatListSegment) -> Self {
        Self {
            service_endpoint,
            container_name,
            prefix: None,
            marker: None,
            max_results: None,
            segment,
            next_marker: None,
        }
    }
}
#[doc = "An enumeration of blobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBlobsHierarchySegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "ContainerName")]
    pub container_name: String,
    #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "Delimiter", default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "Segment")]
    pub segment: BlobHierarchyListSegment,
    #[serde(rename = "NextMarker", default, skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}
impl azure_core::Continuable for ListBlobsHierarchySegmentResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}
impl ListBlobsHierarchySegmentResponse {
    pub fn new(service_endpoint: String, container_name: String, segment: BlobHierarchyListSegment) -> Self {
        Self {
            service_endpoint,
            container_name,
            prefix: None,
            marker: None,
            max_results: None,
            delimiter: None,
            segment,
            next_marker: None,
        }
    }
}
#[doc = "An enumeration of containers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListContainersSegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "ContainerItems")]
    pub container_items: Vec<ContainerItem>,
    #[serde(rename = "NextMarker", default, skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}
impl azure_core::Continuable for ListContainersSegmentResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}
impl ListContainersSegmentResponse {
    pub fn new(service_endpoint: String, container_items: Vec<ContainerItem>) -> Self {
        Self {
            service_endpoint,
            prefix: None,
            marker: None,
            max_results: None,
            container_items,
            next_marker: None,
        }
    }
}
#[doc = "Azure Analytics Logging settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    #[doc = "The version of Storage Analytics to configure."]
    #[serde(rename = "Version")]
    pub version: String,
    #[doc = "Indicates whether all delete requests should be logged."]
    #[serde(rename = "Delete")]
    pub delete: bool,
    #[doc = "Indicates whether all read requests should be logged."]
    #[serde(rename = "Read")]
    pub read: bool,
    #[doc = "Indicates whether all write requests should be logged."]
    #[serde(rename = "Write")]
    pub write: bool,
    #[doc = "the retention policy which determines how long the associated data should persist"]
    #[serde(rename = "RetentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
impl Logging {
    pub fn new(version: String, delete: bool, read: bool, write: bool, retention_policy: RetentionPolicy) -> Self {
        Self {
            version,
            delete,
            read,
            write,
            retention_policy,
        }
    }
}
#[doc = "a summary of request statistics grouped by API in hour or minute aggregates for blobs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The version of Storage Analytics to configure."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Indicates whether metrics are enabled for the Blob service."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates whether metrics should generate summary statistics for called API operations."]
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[doc = "the retention policy which determines how long the associated data should persist"]
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl Metrics {
    pub fn new(enabled: bool) -> Self {
        Self {
            version: None,
            enabled,
            include_ap_is: None,
            retention_policy: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectReplicationMetadata {}
impl ObjectReplicationMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "the list of pages"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageList {
    #[serde(rename = "PageRange", default, skip_serializing_if = "Vec::is_empty")]
    pub page_range: Vec<PageRange>,
    #[serde(rename = "ClearRange", default, skip_serializing_if = "Vec::is_empty")]
    pub clear_range: Vec<ClearRange>,
    #[serde(rename = "NextMarker", default, skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}
impl azure_core::Continuable for PageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}
impl PageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageRange {
    #[serde(rename = "Start")]
    pub start: i64,
    #[serde(rename = "End")]
    pub end: i64,
}
impl PageRange {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}
#[doc = "parquet configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParquetConfiguration {}
impl ParquetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublicAccessType")]
pub enum PublicAccessType {
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "blob")]
    Blob,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublicAccessType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublicAccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublicAccessType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Container => serializer.serialize_unit_variant("PublicAccessType", 0u32, "container"),
            Self::Blob => serializer.serialize_unit_variant("PublicAccessType", 1u32, "blob"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFormat {
    #[doc = "The quick query format type."]
    #[serde(rename = "Type")]
    pub type_: QueryType,
    #[doc = "Groups the settings used for interpreting the blob data if the blob is delimited text formatted."]
    #[serde(rename = "DelimitedTextConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub delimited_text_configuration: Option<DelimitedTextConfiguration>,
    #[doc = "json text configuration"]
    #[serde(rename = "JsonTextConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub json_text_configuration: Option<JsonTextConfiguration>,
    #[doc = "Groups the settings used for formatting the response if the response should be Arrow formatted."]
    #[serde(rename = "ArrowConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub arrow_configuration: Option<ArrowConfiguration>,
    #[doc = "parquet configuration"]
    #[serde(rename = "ParquetTextConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub parquet_text_configuration: Option<ParquetConfiguration>,
}
impl QueryFormat {
    pub fn new(type_: QueryType) -> Self {
        Self {
            type_,
            delimited_text_configuration: None,
            json_text_configuration: None,
            arrow_configuration: None,
            parquet_text_configuration: None,
        }
    }
}
#[doc = "Groups the set of query request settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryRequest {
    #[doc = "Required. The type of the provided query expression."]
    #[serde(rename = "QueryType")]
    pub query_type: query_request::QueryType,
    #[doc = "The query expression in SQL. The maximum size of the query expression is 256KiB."]
    #[serde(rename = "Expression")]
    pub expression: String,
    #[serde(rename = "InputSerialization", default, skip_serializing_if = "Option::is_none")]
    pub input_serialization: Option<QuerySerialization>,
    #[serde(rename = "OutputSerialization", default, skip_serializing_if = "Option::is_none")]
    pub output_serialization: Option<QuerySerialization>,
}
impl QueryRequest {
    pub fn new(query_type: query_request::QueryType, expression: String) -> Self {
        Self {
            query_type,
            expression,
            input_serialization: None,
            output_serialization: None,
        }
    }
}
pub mod query_request {
    use super::*;
    #[doc = "Required. The type of the provided query expression."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryType {
        #[serde(rename = "SQL")]
        Sql,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuerySerialization {
    #[serde(rename = "Format")]
    pub format: QueryFormat,
}
impl QuerySerialization {
    pub fn new(format: QueryFormat) -> Self {
        Self { format }
    }
}
#[doc = "The quick query format type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    #[serde(rename = "delimited")]
    Delimited,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "arrow")]
    Arrow,
    #[serde(rename = "parquet")]
    Parquet,
}
#[doc = "If an object is in rehydrate pending state then this header is returned with priority of rehydrate. Valid values are High and Standard."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RehydratePriority")]
pub enum RehydratePriority {
    High,
    Standard,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RehydratePriority {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RehydratePriority {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RehydratePriority {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::High => serializer.serialize_unit_variant("RehydratePriority", 0u32, "High"),
            Self::Standard => serializer.serialize_unit_variant("RehydratePriority", 1u32, "Standard"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "the retention policy which determines how long the associated data should persist"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "Indicates whether a retention policy is enabled for the storage service"]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates the number of days that metrics or logging or soft-deleted data should be retained. All data older than this value will be deleted"]
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[doc = "Indicates whether permanent delete is allowed on this storage account."]
    #[serde(rename = "AllowPermanentDelete", default, skip_serializing_if = "Option::is_none")]
    pub allow_permanent_delete: Option<bool>,
}
impl RetentionPolicy {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            days: None,
            allow_permanent_delete: None,
        }
    }
}
#[doc = "signed identifier"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[doc = "a unique id"]
    #[serde(rename = "Id")]
    pub id: String,
    #[doc = "An Access policy"]
    #[serde(rename = "AccessPolicy")]
    pub access_policy: AccessPolicy,
}
impl SignedIdentifier {
    pub fn new(id: String, access_policy: AccessPolicy) -> Self {
        Self { id, access_policy }
    }
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[doc = "The properties that enable an account to host a static website"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticWebsite {
    #[doc = "Indicates whether this account is hosting a static website"]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "The default name of the index page under each directory"]
    #[serde(rename = "IndexDocument", default, skip_serializing_if = "Option::is_none")]
    pub index_document: Option<String>,
    #[doc = "The absolute path of the custom 404 page"]
    #[serde(rename = "ErrorDocument404Path", default, skip_serializing_if = "Option::is_none")]
    pub error_document404_path: Option<String>,
    #[doc = "Absolute path of the default index page"]
    #[serde(rename = "DefaultIndexDocumentPath", default, skip_serializing_if = "Option::is_none")]
    pub default_index_document_path: Option<String>,
}
impl StaticWebsite {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            index_document: None,
            error_document404_path: None,
            default_index_document_path: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageError {
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for StorageError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Service Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceProperties {
    #[doc = "Azure Analytics Logging settings."]
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[doc = "a summary of request statistics grouped by API in hour or minute aggregates for blobs"]
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[doc = "a summary of request statistics grouped by API in hour or minute aggregates for blobs"]
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[doc = "The set of CORS rules."]
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
    #[doc = "The default version to use for requests to the Blob service if an incoming request's version is not specified. Possible values include version 2008-10-27 and all more recent versions"]
    #[serde(rename = "DefaultServiceVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_service_version: Option<String>,
    #[doc = "the retention policy which determines how long the associated data should persist"]
    #[serde(rename = "DeleteRetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub delete_retention_policy: Option<RetentionPolicy>,
    #[doc = "The properties that enable an account to host a static website"]
    #[serde(rename = "StaticWebsite", default, skip_serializing_if = "Option::is_none")]
    pub static_website: Option<StaticWebsite>,
}
impl StorageServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stats for the storage service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceStats {
    #[doc = "Geo-Replication information for the Secondary Storage Service"]
    #[serde(rename = "GeoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<GeoReplication>,
}
impl StorageServiceStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A user delegation key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDelegationKey {
    #[doc = "The Azure Active Directory object ID in GUID format."]
    #[serde(rename = "SignedOid")]
    pub signed_oid: String,
    #[doc = "The Azure Active Directory tenant ID in GUID format"]
    #[serde(rename = "SignedTid")]
    pub signed_tid: String,
    #[doc = "The date-time the key is active"]
    #[serde(rename = "SignedStart", with = "azure_core::date::rfc3339")]
    pub signed_start: time::OffsetDateTime,
    #[doc = "The date-time the key expires"]
    #[serde(rename = "SignedExpiry", with = "azure_core::date::rfc3339")]
    pub signed_expiry: time::OffsetDateTime,
    #[doc = "Abbreviation of the Azure Storage service that accepts the key"]
    #[serde(rename = "SignedService")]
    pub signed_service: String,
    #[doc = "The service version that created the key"]
    #[serde(rename = "SignedVersion")]
    pub signed_version: String,
    #[doc = "The key as a base64 string"]
    #[serde(rename = "Value")]
    pub value: String,
}
impl UserDelegationKey {
    pub fn new(
        signed_oid: String,
        signed_tid: String,
        signed_start: time::OffsetDateTime,
        signed_expiry: time::OffsetDateTime,
        signed_service: String,
        signed_version: String,
        value: String,
    ) -> Self {
        Self {
            signed_oid,
            signed_tid,
            signed_start,
            signed_expiry,
            signed_service,
            signed_version,
            value,
        }
    }
}
