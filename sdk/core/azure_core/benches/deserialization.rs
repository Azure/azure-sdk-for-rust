// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{json, time::OffsetDateTime, xml};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use models::ListBlobsFlatSegmentResponse;

use crate::models::{
    AccessTier, BlobItemInternal, BlobName, BlobPropertiesInternal, BlobType, LeaseState,
    LeaseStatus,
};

const ITEM_COUNT: [usize; 3] = [5, 25, 100];

fn deserialize_list_blobs(c: &mut Criterion) {
    for count in ITEM_COUNT {
        // Generate JSON and XML blobs that emulate `azure_storage_blob::BlobContainerClient::list_blobs()`.
        let mut blobs = ListBlobsFlatSegmentResponse {
            service_endpoint: Some("https://t0123456789abcdefprim.blob.core.windows.net/".into()),
            container_name: Some("container01234567".into()),
            ..Default::default()
        };

        let now = OffsetDateTime::now_utc();
        for i in 0..count {
            let blob = BlobItemInternal {
                name: Some(BlobName {
                    content: Some(format!("testBlob{i}")),
                    ..Default::default()
                }),
                properties: Some(BlobPropertiesInternal {
                    creation_time: Some(now),
                    last_modified: Some(now),
                    etag: Some(format!("{i:#15x}")),
                    content_length: Some(17),
                    content_type: Some("application/x-octet-stream".into()),
                    content_md5: Some([i as u8].to_vec()),
                    blob_type: Some(BlobType::BlockBlob),
                    access_tier: Some(AccessTier::Hot),
                    access_tier_inferred: Some(true),
                    lease_status: Some(LeaseStatus::Unlocked),
                    lease_state: Some(LeaseState::Available),
                    server_encrypted: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            };
            blobs.segment.blob_items.push(blob);
        }

        let blobs_json = json::to_json(&blobs).expect("serialize json");
        let blobs_xml = xml::to_xml(&blobs).expect("serialize xml");

        let mut group = c.benchmark_group(format!("deserialize_list_blobs/{count}"));
        group.bench_function("json", |b| {
            b.iter(|| {
                let _: ListBlobsFlatSegmentResponse =
                    black_box(json::from_json(&blobs_json).expect("deserialize json"));
            });
        });
        group.bench_function("xml", |b| {
            b.iter(|| {
                let _: ListBlobsFlatSegmentResponse =
                    black_box(xml::from_xml(&blobs_xml).expect("deserialize xml"));
            });
        });
        group.finish();
    }
}

criterion_group! {
    name = benchmarks;
    config = Criterion::default();
    targets = deserialize_list_blobs
}

criterion_main!(benchmarks);

mod models {
    //! Copied from `azure_storage_blob::models`.
    //!
    //! Model definitions remain intact but some unused fields are replaced with `azure_core::Value`
    //! because we still want to retain the same behavior and performance of the deserialization visitor.

    use azure_core::{
        error::{Error, ErrorKind},
        time::OffsetDateTime,
        Value,
    };
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{convert::Infallible, str::FromStr};

    /// An enumeration of blobs.
    #[derive(Clone, Default, Deserialize, Debug, Serialize)]
    #[non_exhaustive]
    #[serde(rename = "EnumerationResults")]
    pub struct ListBlobsFlatSegmentResponse {
        /// The container name.
        #[serde(rename = "@ContainerName", skip_serializing_if = "Option::is_none")]
        pub container_name: Option<String>,

        /// The marker of the blobs.
        #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
        pub marker: Option<String>,

        /// The max results of the blobs.
        #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
        pub max_results: Option<i32>,

        /// The next marker of the blobs.
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,

        /// The prefix of the blobs.
        #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,

        /// The blob segment.
        #[serde(default, rename = "Blobs")]
        pub segment: BlobFlatListSegment,

        /// The service endpoint.
        #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub service_endpoint: Option<String>,
    }

    /// The blob flat list segment.
    #[derive(Clone, Default, Deserialize, Debug, Serialize)]
    #[non_exhaustive]
    pub struct BlobFlatListSegment {
        /// The blob items.
        #[serde(default, rename = "Blob")]
        pub blob_items: Vec<BlobItemInternal>,
    }

    /// An Azure Storage Blob
    #[derive(Clone, Default, Deserialize, Debug, Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Blob")]
    pub struct BlobItemInternal {
        /// The tags of the blob.
        #[serde(rename = "BlobTags", skip_serializing_if = "Option::is_none")]
        pub blob_tags: Option<Value>,

        /// Whether the blob is deleted.
        #[serde(rename = "Deleted", skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,

        /// Whether the blob has versions only.
        #[serde(rename = "HasVersionsOnly", skip_serializing_if = "Option::is_none")]
        pub has_versions_only: Option<bool>,

        /// Whether the blob is the current version.
        #[serde(rename = "IsCurrentVersion", skip_serializing_if = "Option::is_none")]
        pub is_current_version: Option<bool>,

        /// The metadata of the blob.
        #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
        pub metadata: Option<Value>,

        /// The name of the blob.
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub name: Option<BlobName>,

        /// The object replication metadata of the blob.
        #[serde(rename = "OrMetadata", skip_serializing_if = "Option::is_none")]
        pub object_replication_metadata: Option<Value>,

        /// The properties of the blob.
        #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
        pub properties: Option<BlobPropertiesInternal>,

        /// The snapshot of the blob.
        #[serde(rename = "Snapshot", skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<String>,

        /// The version id of the blob.
        #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
        pub version_id: Option<String>,
    }

    /// The properties of a blob.
    #[derive(Clone, Default, Deserialize, Debug, Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Properties")]
    pub struct BlobPropertiesInternal {
        /// The access tier of the blob.
        #[serde(rename = "AccessTier", skip_serializing_if = "Option::is_none")]
        pub access_tier: Option<AccessTier>,

        /// The access tier change time of the blob.
        #[serde(
            default,
            rename = "AccessTierChangeTime",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub access_tier_change_time: Option<OffsetDateTime>,

        /// Whether the access tier is inferred.
        #[serde(rename = "AccessTierInferred", skip_serializing_if = "Option::is_none")]
        pub access_tier_inferred: Option<bool>,

        /// The archive status of the blob.
        #[serde(rename = "ArchiveStatus", skip_serializing_if = "Option::is_none")]
        pub archive_status: Option<Value>,

        /// The sequence number of the blob.
        #[serde(
            rename = "x-ms-blob-sequence-number",
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_sequence_number: Option<i64>,

        /// The blob type.
        #[serde(rename = "BlobType", skip_serializing_if = "Option::is_none")]
        pub blob_type: Option<BlobType>,

        /// The cache control of the blob.
        #[serde(rename = "Cache-Control", skip_serializing_if = "Option::is_none")]
        pub cache_control: Option<String>,

        /// The content disposition of the blob.
        #[serde(
            rename = "Content-Disposition",
            skip_serializing_if = "Option::is_none"
        )]
        pub content_disposition: Option<String>,

        /// The content encoding of the blob.
        #[serde(rename = "Content-Encoding", skip_serializing_if = "Option::is_none")]
        pub content_encoding: Option<String>,

        /// The content language of the blob.
        #[serde(rename = "Content-Language", skip_serializing_if = "Option::is_none")]
        pub content_language: Option<String>,

        /// The content length of the blob.
        #[serde(rename = "Content-Length", skip_serializing_if = "Option::is_none")]
        pub content_length: Option<u64>,

        /// The content MD5 of the blob.
        #[serde(
            default,
            deserialize_with = "azure_core::base64::option::deserialize",
            rename = "Content-MD5",
            serialize_with = "azure_core::base64::option::serialize",
            skip_serializing_if = "Option::is_none"
        )]
        pub content_md5: Option<Vec<u8>>,

        /// The content type of the blob.
        #[serde(rename = "Content-Type", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,

        /// The copy completion time of the blob.
        #[serde(
            default,
            rename = "CopyCompletionTime",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub copy_completion_time: Option<OffsetDateTime>,

        /// The copy ID of the blob.
        #[serde(rename = "CopyId", skip_serializing_if = "Option::is_none")]
        pub copy_id: Option<String>,

        /// The copy progress of the blob.
        #[serde(rename = "CopyProgress", skip_serializing_if = "Option::is_none")]
        pub copy_progress: Option<String>,

        /// The copy source of the blob.
        #[serde(rename = "CopySource", skip_serializing_if = "Option::is_none")]
        pub copy_source: Option<String>,

        /// The copy status of the blob.
        #[serde(rename = "CopyStatus", skip_serializing_if = "Option::is_none")]
        pub copy_status: Option<Value>,

        /// The copy status description of the blob.
        #[serde(
            rename = "CopyStatusDescription",
            skip_serializing_if = "Option::is_none"
        )]
        pub copy_status_description: Option<String>,

        /// The date-time the blob was created in RFC1123 format.
        #[serde(
            default,
            rename = "Creation-Time",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub creation_time: Option<OffsetDateTime>,

        /// Customer provided key sha256
        #[serde(
            rename = "CustomerProvidedKeySha256",
            skip_serializing_if = "Option::is_none"
        )]
        pub customer_provided_key_sha256: Option<String>,

        /// The time the blob was deleted.
        #[serde(
            default,
            rename = "DeletedTime",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub deleted_time: Option<OffsetDateTime>,

        /// The name of the destination snapshot.
        #[serde(
            rename = "DestinationSnapshot",
            skip_serializing_if = "Option::is_none"
        )]
        pub destination_snapshot: Option<String>,

        /// The encryption scope of the blob.
        #[serde(rename = "EncryptionScope", skip_serializing_if = "Option::is_none")]
        pub encryption_scope: Option<String>,

        /// The blob ETag.
        #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
        pub etag: Option<String>,

        /// The expire time of the blob.
        #[serde(
            default,
            rename = "Expiry-Time",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub expires_on: Option<OffsetDateTime>,

        /// The immutability policy until time of the blob.
        #[serde(
            default,
            rename = "ImmutabilityPolicyUntilDate",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub immutability_policy_expires_on: Option<OffsetDateTime>,

        /// The immutability policy mode of the blob.
        #[serde(
            rename = "ImmutabilityPolicyMode",
            skip_serializing_if = "Option::is_none"
        )]
        pub immutability_policy_mode: Option<Value>,

        /// Whether the blob is incremental copy.
        #[serde(rename = "IncrementalCopy", skip_serializing_if = "Option::is_none")]
        pub incremental_copy: Option<bool>,

        /// Whether the blob is sealed.
        #[serde(rename = "Sealed", skip_serializing_if = "Option::is_none")]
        pub is_sealed: Option<bool>,

        /// The last access time of the blob.
        #[serde(
            default,
            rename = "LastAccessTime",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub last_accessed_on: Option<OffsetDateTime>,

        /// The date-time the blob was last modified in RFC1123 format.
        #[serde(
            default,
            rename = "Last-Modified",
            skip_serializing_if = "Option::is_none",
            with = "azure_core::time::rfc7231::option"
        )]
        pub last_modified: Option<OffsetDateTime>,

        /// The lease duration of the blob.
        #[serde(rename = "LeaseDuration", skip_serializing_if = "Option::is_none")]
        pub lease_duration: Option<Value>,

        /// The lease state of the blob.
        #[serde(rename = "LeaseState", skip_serializing_if = "Option::is_none")]
        pub lease_state: Option<LeaseState>,

        /// The lease status of the blob.
        #[serde(rename = "LeaseStatus", skip_serializing_if = "Option::is_none")]
        pub lease_status: Option<LeaseStatus>,

        /// Whether the blob is under legal hold.
        #[serde(rename = "LegalHold", skip_serializing_if = "Option::is_none")]
        pub legal_hold: Option<bool>,

        /// The rehydrate priority of the blob.
        #[serde(rename = "RehydratePriority", skip_serializing_if = "Option::is_none")]
        pub rehydrate_priority: Option<Value>,

        /// The remaining retention days of the blob.
        #[serde(
            rename = "RemainingRetentionDays",
            skip_serializing_if = "Option::is_none"
        )]
        pub remaining_retention_days: Option<i32>,

        /// Whether the blob is encrypted on the server.
        #[serde(rename = "ServerEncrypted", skip_serializing_if = "Option::is_none")]
        pub server_encrypted: Option<bool>,

        /// The number of tags for the blob.
        #[serde(rename = "TagCount", skip_serializing_if = "Option::is_none")]
        pub tag_count: Option<i32>,
    }

    /// The access tiers.
    #[derive(Debug, PartialEq, Eq, Clone)]
    #[non_exhaustive]
    pub enum AccessTier {
        /// The archive access tier.
        Archive,

        /// The Cold access tier.
        Cold,

        /// The cool access tier.
        Cool,

        /// The hot access tier.
        Hot,

        /// The hot P10 tier.
        P10,

        /// The hot P15 tier.
        P15,

        /// The hot P20 tier.
        P20,

        /// The hot P30 tier.
        P30,

        /// The hot P4 tier.
        P4,

        /// The hot P40 tier.
        P40,

        /// The hot P50 tier.
        P50,

        /// The hot P6 tier.
        P6,

        /// The hot P60 tier.
        P60,

        /// The hot P70 tier.
        P70,

        /// The hot P80 tier.
        P80,

        /// The Premium access tier.
        Premium,

        /// Any other value not defined in `AccessTier`.
        UnknownValue(String),
    }

    impl FromStr for AccessTier {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err> {
            Ok(match s {
                "Archive" => AccessTier::Archive,
                "Cold" => AccessTier::Cold,
                "Cool" => AccessTier::Cool,
                "Hot" => AccessTier::Hot,
                "P10" => AccessTier::P10,
                "P15" => AccessTier::P15,
                "P20" => AccessTier::P20,
                "P30" => AccessTier::P30,
                "P4" => AccessTier::P4,
                "P40" => AccessTier::P40,
                "P50" => AccessTier::P50,
                "P6" => AccessTier::P6,
                "P60" => AccessTier::P60,
                "P70" => AccessTier::P70,
                "P80" => AccessTier::P80,
                "Premium" => AccessTier::Premium,
                _ => AccessTier::UnknownValue(s.to_string()),
            })
        }
    }

    impl AsRef<str> for AccessTier {
        fn as_ref(&self) -> &str {
            match self {
                AccessTier::Archive => "Archive",
                AccessTier::Cold => "Cold",
                AccessTier::Cool => "Cool",
                AccessTier::Hot => "Hot",
                AccessTier::P10 => "P10",
                AccessTier::P15 => "P15",
                AccessTier::P20 => "P20",
                AccessTier::P30 => "P30",
                AccessTier::P4 => "P4",
                AccessTier::P40 => "P40",
                AccessTier::P50 => "P50",
                AccessTier::P6 => "P6",
                AccessTier::P60 => "P60",
                AccessTier::P70 => "P70",
                AccessTier::P80 => "P80",
                AccessTier::Premium => "Premium",
                AccessTier::UnknownValue(s) => s.as_str(),
            }
        }
    }

    impl<'de> Deserialize<'de> for AccessTier {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for AccessTier {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(self.as_ref())
        }
    }

    /// The blob type.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[allow(clippy::enum_variant_names)]
    #[non_exhaustive]
    pub enum BlobType {
        /// The blob is an append blob.
        AppendBlob,

        /// The blob is a block blob.
        BlockBlob,

        /// The blob is a page blob.
        PageBlob,
    }

    impl FromStr for BlobType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err> {
            Ok(match s {
                "AppendBlob" => BlobType::AppendBlob,
                "BlockBlob" => BlobType::BlockBlob,
                "PageBlob" => BlobType::PageBlob,
                _ => {
                    return Err(Error::with_message_fn(ErrorKind::DataConversion, || {
                        format!("unknown variant of BlobType found: \"{s}\"")
                    }))
                }
            })
        }
    }

    impl AsRef<str> for BlobType {
        fn as_ref(&self) -> &str {
            match self {
                BlobType::AppendBlob => "AppendBlob",
                BlobType::BlockBlob => "BlockBlob",
                BlobType::PageBlob => "PageBlob",
            }
        }
    }

    impl<'de> Deserialize<'de> for BlobType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for BlobType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(self.as_ref())
        }
    }

    /// Represents a blob name.
    #[derive(Clone, Default, Deserialize, Debug, Serialize)]
    #[non_exhaustive]
    pub struct BlobName {
        /// The blob name.
        #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,

        /// Whether the blob name is encoded.
        #[serde(rename = "@Encoded", skip_serializing_if = "Option::is_none")]
        pub encoded: Option<bool>,
    }

    /// The lease state.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[non_exhaustive]
    pub enum LeaseState {
        /// The lease is available.
        Available,

        /// The lease is breaking.
        Breaking,

        /// The lease is broken.
        Broken,

        /// The lease is expired.
        Expired,

        /// The lease is currently leased.
        Leased,
    }

    impl FromStr for LeaseState {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err> {
            Ok(match s {
                "available" => LeaseState::Available,
                "breaking" => LeaseState::Breaking,
                "broken" => LeaseState::Broken,
                "expired" => LeaseState::Expired,
                "leased" => LeaseState::Leased,
                _ => {
                    return Err(Error::with_message_fn(ErrorKind::DataConversion, || {
                        format!("unknown variant of LeaseState found: \"{s}\"")
                    }))
                }
            })
        }
    }

    impl AsRef<str> for LeaseState {
        fn as_ref(&self) -> &str {
            match self {
                LeaseState::Available => "available",
                LeaseState::Breaking => "breaking",
                LeaseState::Broken => "broken",
                LeaseState::Expired => "expired",
                LeaseState::Leased => "leased",
            }
        }
    }

    impl<'de> Deserialize<'de> for LeaseState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for LeaseState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(self.as_ref())
        }
    }

    /// The lease status.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    #[non_exhaustive]
    pub enum LeaseStatus {
        /// The lease is locked.
        Locked,

        /// The lease is unlocked.
        Unlocked,
    }

    impl FromStr for LeaseStatus {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err> {
            Ok(match s {
                "locked" => LeaseStatus::Locked,
                "unlocked" => LeaseStatus::Unlocked,
                _ => {
                    return Err(Error::with_message_fn(ErrorKind::DataConversion, || {
                        format!("unknown variant of LeaseStatus found: \"{s}\"")
                    }))
                }
            })
        }
    }

    impl AsRef<str> for LeaseStatus {
        fn as_ref(&self) -> &str {
            match self {
                LeaseStatus::Locked => "locked",
                LeaseStatus::Unlocked => "unlocked",
            }
        }
    }

    impl<'de> Deserialize<'de> for LeaseStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for LeaseStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            s.serialize_str(self.as_ref())
        }
    }
}
