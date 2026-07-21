// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::BlobItem;
use async_trait::async_trait;
use azure_core::{fmt::SafeDebug, http::pager::Page, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The result of the List Blobs API.
#[derive(Clone, Default, Deserialize, SafeDebug, Serialize)]
#[non_exhaustive]
#[serde(rename = "EnumerationResults")]
pub struct ListBlobsResponse {
    /// The list of blobs.
    #[serde(
        default,
        deserialize_with = "deserialize_blob_items",
        rename = "Blobs",
        serialize_with = "serialize_blob_items"
    )]
    pub blob_items: Vec<BlobItem>,

    /// The container name.
    #[serde(rename = "@ContainerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,

    /// An opaque string value that identifies the portion of the result set returned with this operation.
    #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    /// The maximum number of blobs to be returned with this operation.
    #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,

    /// An opaque string value that identifies the portion of the result set to be returned with the next operation.
    #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,

    /// The prefix of the list operation.
    #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The service endpoint.
    #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "Blobs")]
struct BlobItems {
    #[serde(default, rename = "Blob")]
    items: Vec<BlobItem>,
}

fn deserialize_blob_items<'de, D>(deserializer: D) -> std::result::Result<Vec<BlobItem>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(BlobItems::deserialize(deserializer)?.items)
}

fn serialize_blob_items<S>(
    items: &[BlobItem],
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    BlobItems {
        items: items.to_vec(),
    }
    .serialize(serializer)
}

#[async_trait]
impl Page for ListBlobsResponse {
    type Item = BlobItem;
    type IntoIter = <Vec<BlobItem> as IntoIterator>::IntoIter;

    async fn into_items(self) -> Result<Self::IntoIter> {
        Ok(self.blob_items.into_iter())
    }
}
