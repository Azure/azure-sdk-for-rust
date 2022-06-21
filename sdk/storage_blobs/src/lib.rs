#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub use azure_core::error::{Error, ErrorKind, ResultExt};

mod access_tier;
mod ba512_range;
#[allow(clippy::module_inception)]
pub mod blob;
mod blob_content_md5;
mod block_id;
mod clients;
mod condition_append_position;
mod condition_max_size;
pub mod container;
mod delete_snapshot_method;
mod hash;
mod headers;
mod incomplete_vector;
pub mod prelude;
mod snapshot;
mod version_id;

pub use access_tier::AccessTier;
use azure_core::{AppendToUrlQuery, Header};
pub use ba512_range::BA512Range;
pub use blob_content_md5::BlobContentMD5;
pub use block_id::BlockId;
pub use condition_append_position::ConditionAppendPosition;
pub use condition_max_size::ConditionMaxSize;
pub use delete_snapshot_method::DeleteSnapshotsMethod;
pub use hash::Hash;
pub use snapshot::Snapshot;
pub use version_id::VersionId;

use incomplete_vector::IncompleteVector;

#[derive(Debug, Clone)]
pub enum BlobVersioning {
    Snapshot(Snapshot),
    VersionId(VersionId),
}

impl From<Snapshot> for BlobVersioning {
    fn from(snapshot: Snapshot) -> Self {
        BlobVersioning::Snapshot(snapshot)
    }
}

impl From<VersionId> for BlobVersioning {
    fn from(version_id: VersionId) -> Self {
        BlobVersioning::VersionId(version_id)
    }
}

impl AppendToUrlQuery for BlobVersioning {
    fn append_to_url_query(&self, url: &mut url::Url) {
        match self {
            BlobVersioning::Snapshot(snapshot) => snapshot.append_to_url_query(url),
            BlobVersioning::VersionId(version_id) => version_id.append_to_url_query(url),
        }
    }
}

impl AppendToUrlQuery for &BlobVersioning {
    fn append_to_url_query(&self, url: &mut url::Url) {
        match self {
            BlobVersioning::Snapshot(snapshot) => snapshot.append_to_url_query(url),
            BlobVersioning::VersionId(version_id) => version_id.append_to_url_query(url),
        }
    }
}

create_enum!(RehydratePriority, (High, "High"), (Standard, "Standard"));

impl Header for RehydratePriority {
    fn name(&self) -> azure_core::headers::HeaderName {
        headers::REHYDRATE_PRIORITY.into()
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        self.to_string().into()
    }
}
