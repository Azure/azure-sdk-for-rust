//! Various blob related request options

mod access_tier;
mod ba512_range;
mod blob_cache_control;
mod blob_content_disposition;
mod blob_content_encoding;
mod blob_content_language;
mod blob_content_md5;
mod blob_content_type;
mod blob_expiry;
mod blob_versioning;
mod block_id;
mod condition_append_position;
mod condition_max_size;
mod delete_snapshot_method;
mod hash;
mod rehydrate_policy;
mod tags;

pub use access_tier::AccessTier;
pub use ba512_range::BA512Range;
pub use blob_cache_control::BlobCacheControl;
pub use blob_content_disposition::BlobContentDisposition;
pub use blob_content_encoding::BlobContentEncoding;
pub use blob_content_language::BlobContentLanguage;
pub use blob_content_md5::BlobContentMD5;
pub use blob_content_type::BlobContentType;
pub use blob_expiry::BlobExpiry;
pub use blob_versioning::BlobVersioning;
pub use block_id::BlockId;
pub use condition_append_position::ConditionAppendPosition;
pub use condition_max_size::ConditionMaxSize;
pub use delete_snapshot_method::DeleteSnapshotsMethod;
pub use hash::Hash;
pub use rehydrate_policy::RehydratePriority;
pub use tags::Tags;

use std::str::FromStr;

use azure_core::error::Error;
use azure_core::headers::HeaderName;

request_query!(
    /// This type could also be a DateTime but the docs clearly states to treat is as opaque so we do not convert it in any way.
    ///
    ///See: <https://docs.microsoft.com/rest/api/storageservices/get-blob>"]
    VersionId,
    "version_id"
);

request_query!(
    /// This type could also be a DateTime but the docs clearly states to treat is as opaque so we do not convert it in any way.
    ///
    /// See: <https://docs.microsoft.com/rest/api/storageservices/get-blob>"]
    #[derive(PartialEq, Eq, Serialize, Deserialize)]
    Snapshot,
    "snapshot"
);

impl FromStr for Snapshot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

pub const SNAPSHOT: HeaderName = HeaderName::from_static("x-ms-snapshot");
