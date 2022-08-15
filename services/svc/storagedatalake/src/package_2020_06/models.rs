#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AclFailedEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AclFailedEntry {
    pub fn new() -> Self {
        Self::default()
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
    pub name: String,
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
    #[serde(rename = "DeletionId", default, skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
}
impl BlobItemInternal {
    pub fn new(name: String, deleted: bool, snapshot: String, properties: BlobPropertiesInternal) -> Self {
        Self {
            name,
            deleted,
            snapshot,
            version_id: None,
            is_current_version: None,
            properties,
            deletion_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobPrefix {
    #[serde(rename = "Name")]
    pub name: String,
}
impl BlobPrefix {
    pub fn new(name: String) -> Self {
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
    #[serde(rename = "CopyId", default, skip_serializing_if = "Option::is_none")]
    pub copy_id: Option<String>,
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
    #[serde(rename = "AccessTierInferred", default, skip_serializing_if = "Option::is_none")]
    pub access_tier_inferred: Option<bool>,
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
    #[serde(rename = "LastAccessTime", with = "azure_core::date::rfc1123::option")]
    pub last_access_time: Option<time::OffsetDateTime>,
    #[serde(rename = "DeleteTime", with = "azure_core::date::rfc1123::option")]
    pub delete_time: Option<time::OffsetDateTime>,
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
            copy_id: None,
            copy_source: None,
            copy_progress: None,
            copy_completion_time: None,
            copy_status_description: None,
            server_encrypted: None,
            incremental_copy: None,
            destination_snapshot: None,
            deleted_time: None,
            remaining_retention_days: None,
            access_tier_inferred: None,
            customer_provided_key_sha256: None,
            encryption_scope: None,
            access_tier_change_time: None,
            tag_count: None,
            expiry_time: None,
            sealed: None,
            last_access_time: None,
            delete_time: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileSystem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl FileSystem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileSystemList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filesystems: Vec<FileSystem>,
}
impl azure_core::Continuable for FileSystemList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl FileSystemList {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Path {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDirectory", default, skip_serializing_if = "Option::is_none")]
    pub is_directory: Option<bool>,
    #[serde(rename = "lastModified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}
impl Path {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PathList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
}
impl azure_core::Continuable for PathList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PathList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetAccessControlRecursiveResponse {
    #[serde(rename = "directoriesSuccessful", default, skip_serializing_if = "Option::is_none")]
    pub directories_successful: Option<i32>,
    #[serde(rename = "filesSuccessful", default, skip_serializing_if = "Option::is_none")]
    pub files_successful: Option<i32>,
    #[serde(rename = "failureCount", default, skip_serializing_if = "Option::is_none")]
    pub failure_count: Option<i32>,
    #[serde(rename = "failedEntries", default, skip_serializing_if = "Vec::is_empty")]
    pub failed_entries: Vec<AclFailedEntry>,
}
impl SetAccessControlRecursiveResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageError {
    #[doc = "The service error response object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<storage_error::Error>,
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
pub mod storage_error {
    use super::*;
    #[doc = "The service error response object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "The service error code."]
        #[serde(rename = "Code", default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The service error message."]
        #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
