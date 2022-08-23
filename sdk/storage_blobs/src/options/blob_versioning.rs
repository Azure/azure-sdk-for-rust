use super::VersionId;
use crate::options::Snapshot;
use azure_core::AppendToUrlQuery;

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
