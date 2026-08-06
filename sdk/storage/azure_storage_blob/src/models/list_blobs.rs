// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::BlobItem;
use async_trait::async_trait;
use azure_core::{fmt::SafeDebug, http::pager::Page, Result};
use serde::{Deserialize, Serialize};

/// The result of a flat blob listing operation.
#[derive(Clone, Default, Deserialize, SafeDebug, Serialize)]
#[non_exhaustive]
#[serde(rename = "EnumerationResults")]
pub struct ListBlobsResponse {
    /// The list of blobs.
    #[serde(
        default,
        deserialize_with = "blob_items::deserialize",
        rename = "Blobs",
        serialize_with = "blob_items::serialize"
    )]
    pub blob_items: Vec<BlobItem>,

    /// The container name.
    #[serde(rename = "@ContainerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// Identifies the portion of the result set returned with this operation.
    #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of blobs returned with this operation.
    #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// Identifies the portion of the result set to return with the next operation.
    #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// The prefix of the list operation.
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The service endpoint.
    #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
}

#[async_trait]
impl Page for ListBlobsResponse {
    type Item = BlobItem;
    type IntoIter = <Vec<BlobItem> as IntoIterator>::IntoIter;

    async fn into_items(self) -> Result<Self::IntoIter> {
        Ok(self.blob_items.into_iter())
    }
}

mod blob_items {
    use crate::models::BlobItem;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Deserialize, Serialize)]
    struct BlobItems {
        #[serde(default, rename = "Blob")]
        items: Vec<BlobItem>,
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BlobItem>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BlobItems::deserialize(deserializer)?.items)
    }

    pub(super) fn serialize<S>(items: &[BlobItem], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        BlobItems {
            items: items.to_vec(),
        }
        .serialize(serializer)
    }
}
