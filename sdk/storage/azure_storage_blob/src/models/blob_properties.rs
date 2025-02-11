// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    headers::{
        FromHeaders, HeaderName, Headers, BLOB_ACCESS_TIER, BLOB_TYPE, CREATION_TIME, LEASE_STATE,
        LEASE_STATUS, SERVER_ENCRYPTED,
    },
    Etag, LeaseStatus,
};

use crate::models::{AccessTier, BlobType, LeaseState};

pub const CONTENT_LENGTH: HeaderName = HeaderName::from_static("content-length");
pub const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
pub const CONTENT_TYPE: HeaderName = HeaderName::from_static("content-type");
pub const ETAG: HeaderName = HeaderName::from_static("etag");
pub const LAST_MODIFIED: HeaderName = HeaderName::from_static("last-modified");
pub const BLOB_ACCESS_TIER_INFERRED: HeaderName =
    HeaderName::from_static("x-ms-access-tier-inferred");

/// Properties of an Azure Storage blob.
///
#[derive(Clone, Default, Debug)]
pub struct BlobProperties {
    pub access_tier_inferred: Option<bool>,
    pub access_tier: Option<AccessTier>,
    pub blob_type: Option<BlobType>,
    pub content_length: Option<i64>,
    pub content_md5: Option<String>,
    pub content_type: Option<String>,
    pub creation_time: Option<String>,
    pub etag: Option<Etag>,
    pub last_modified: Option<String>,
    pub lease_state: Option<LeaseState>,
    pub lease_status: Option<LeaseStatus>,
    pub server_encrypted: Option<bool>,
}

impl FromHeaders for BlobProperties {
    type Error = url::ParseError;

    fn header_names() -> &'static [&'static str] {
        &[
            "content-length",
            "content-md5",
            "content-type",
            "etag",
            "last-modified",
            "x-ms-access-tier-inferred",
        ]
    }

    fn from_headers(headers: &Headers) -> Result<Option<Self>, Self::Error> {
        let mut properties = BlobProperties {
            ..Default::default()
        };

        let access_tier_inferred: bool = headers
            .get_optional_as(&BLOB_ACCESS_TIER_INFERRED)
            .unwrap()
            .unwrap();
        properties.access_tier_inferred = Some(access_tier_inferred);

        let access_tier: AccessTier = headers.get_optional_as(&BLOB_ACCESS_TIER).unwrap().unwrap();
        properties.access_tier = Some(access_tier);

        let blob_type: BlobType = headers.get_optional_as(&BLOB_TYPE).unwrap().unwrap();
        properties.blob_type = Some(blob_type);

        let content_length: i64 = headers.get_optional_as(&CONTENT_LENGTH).unwrap().unwrap();
        properties.content_length = Some(content_length);

        let content_md5: String = headers.get_optional_as(&CONTENT_MD5).unwrap().unwrap();
        properties.content_md5 = Some(content_md5);

        let content_type: String = headers.get_optional_as(&CONTENT_TYPE).unwrap().unwrap();
        properties.content_type = Some(content_type);

        let creation_time: String = headers.get_optional_as(&CREATION_TIME).unwrap().unwrap();
        properties.creation_time = Some(creation_time);

        let etag: Etag = headers.get_optional_as(&ETAG).unwrap().unwrap();
        properties.etag = Some(etag);

        let last_modified: String = headers.get_optional_as(&LAST_MODIFIED).unwrap().unwrap();
        properties.last_modified = Some(last_modified);

        let lease_state: LeaseState = headers.get_optional_as(&LEASE_STATE).unwrap().unwrap();
        properties.lease_state = Some(lease_state);

        let lease_status: LeaseStatus = headers.get_optional_as(&LEASE_STATUS).unwrap().unwrap();
        properties.lease_status = Some(lease_status);

        let server_encrypted: bool = headers.get_optional_as(&SERVER_ENCRYPTED).unwrap().unwrap();
        properties.server_encrypted = Some(server_encrypted);

        Ok(Some(properties))
    }
}
