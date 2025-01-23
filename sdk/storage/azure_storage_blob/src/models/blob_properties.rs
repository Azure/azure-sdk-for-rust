// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::headers::Headers;

/// Properties of an Azure Storage blob.
///
#[derive(Clone, Default, Debug)]
pub struct BlobProperties {
    pub access_tier_inferred: String,
    pub access_tier: String,
    pub blob_type: String,
    pub content_length: String,
    pub content_md5: String,
    pub content_type: String,
    pub creation_time: String,
    pub etag: String,
    pub last_modified: String,
    pub lease_state: String,
    pub lease_status: String,
    pub server_encrypted: String,
}

impl BlobProperties {
    pub fn build_from_response_headers(response_headers: &Headers) -> BlobProperties {
        let mut properties = BlobProperties {
            ..Default::default()
        };
        for (key, value) in response_headers.iter() {
            match key.as_str() {
                "content-length" => properties.content_length = String::from(value.as_str()),
                "content-md5" => properties.content_md5 = String::from(value.as_str()),
                "content-type" => properties.content_type = String::from(value.as_str()),
                "etag" => properties.etag = String::from(value.as_str()),
                "last-modified" => properties.last_modified = String::from(value.as_str()),
                "x-ms-access-tier-inferred" => {
                    properties.access_tier_inferred = String::from(value.as_str())
                }
                "x-ms-access-tier" => properties.access_tier = String::from(value.as_str()),
                "x-ms-blob-type" => properties.blob_type = String::from(value.as_str()),
                "x-ms-creation-time" => properties.creation_time = String::from(value.as_str()),
                "x-ms-lease-state" => properties.lease_state = String::from(value.as_str()),
                "x-ms-lease-status" => properties.lease_status = String::from(value.as_str()),
                "x-ms-server-encrypted" => {
                    properties.server_encrypted = String::from(value.as_str())
                }
                _ => {}
            }
        }
        properties
    }
}
