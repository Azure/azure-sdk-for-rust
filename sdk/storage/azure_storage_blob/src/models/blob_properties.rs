// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

use azure_core::headers::Headers;

/// Properties of a Azure Storage blob.
///
#[derive(Clone, Default, Debug)]
pub struct BlobProperties {
    pub blob_type: String,
    pub content_length: String,
    pub date: String,
    pub version: String,
    pub lease_status: String,
    pub access_tier: String,
    pub access_tier_inferred: String,
    pub client_request_id: String,
    pub last_modified: String,
    pub server: String,
    pub content_md5: String,
    pub accept_ranges: String,
    pub request_id: String,
    pub content_type: String,
    pub server_encrypted: String,
    pub etag: String,
    pub lease_state: String,
    pub creation_time: String,
    pub vary: String,
}

pub fn build_from_response_headers(response_headers: &Headers) -> BlobProperties {
    let mut properties = BlobProperties {
        ..Default::default()
    };
    for (key, value) in response_headers.iter() {
        match key.as_str() {
            "x-ms-blob-type" => properties.blob_type = String::from(value.as_str()),
            "content-length" => properties.content_length = String::from(value.as_str()),
            "date" => properties.date = String::from(value.as_str()),
            "x-ms-version" => properties.version = String::from(value.as_str()),
            "x-ms-lease-status" => properties.lease_status = String::from(value.as_str()),
            "x-ms-access-tier" => properties.access_tier = String::from(value.as_str()),
            "x-ms-access-tier-inferred" => {
                properties.access_tier_inferred = String::from(value.as_str())
            }
            "x-ms-client-request-id" => properties.client_request_id = String::from(value.as_str()),
            "last-modified" => properties.last_modified = String::from(value.as_str()),
            "server" => properties.server = String::from(value.as_str()),
            "content-md5" => properties.content_md5 = String::from(value.as_str()),
            "accept-ranges" => properties.accept_ranges = String::from(value.as_str()),
            "x-ms-request-id" => properties.request_id = String::from(value.as_str()),
            "content-type" => properties.content_type = String::from(value.as_str()),
            "x-ms-server-encrypted" => properties.server_encrypted = String::from(value.as_str()),
            "etag" => properties.etag = String::from(value.as_str()),
            "x-ms-lease-status" => properties.lease_status = String::from(value.as_str()),
            "x-ms-creation-time" => properties.creation_time = String::from(value.as_str()),
            "vary" => properties.vary = String::from(value.as_str()),
            _ => println!("Unknown key encountered: {}", String::from(value.as_str())), // TODO: Raise Error (Unknown Header in Response)
        }
    }
    properties
}
