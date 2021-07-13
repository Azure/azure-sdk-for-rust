mod access_tier;
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
pub mod prelude;
mod snapshot;
mod version_id;

pub use access_tier::AccessTier;
use azure_core::{AddAsHeader, AppendToUrlQuery};
pub use blob_content_md5::BlobContentMD5;
pub use block_id::BlockId;
pub use condition_append_position::ConditionAppendPosition;
pub use condition_max_size::ConditionMaxSize;
pub use delete_snapshot_method::DeleteSnapshotsMethod;
pub use hash::Hash;
use http::request::Builder;
pub use snapshot::Snapshot;
pub use version_id::VersionId;

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

impl AppendToUrlQuery for &BlobVersioning {
    fn append_to_url_query(&self, url: &mut url::Url) {
        match self {
            BlobVersioning::Snapshot(snapshot) => snapshot.append_to_url_query(url),
            BlobVersioning::VersionId(version_id) => version_id.append_to_url_query(url),
        }
    }
}

create_enum!(RehydratePriority, (High, "High"), (Standard, "Standard"));

impl AddAsHeader for RehydratePriority {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::REHYDRATE_PRIORITY, &format!("{}", self))
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HTTPHeaderError> {
        request.headers_mut().append(
            headers::REHYDRATE_PRIORITY,
            http::header::HeaderValue::from_str(&self.to_string())?,
        );

        Ok(())
    }
}
