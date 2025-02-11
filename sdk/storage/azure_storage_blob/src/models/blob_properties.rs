// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    headers::{
        FromHeaders, HeaderName, Headers, BLOB_ACCESS_TIER, BLOB_TYPE, CREATION_TIME, LEASE_STATE,
        LEASE_STATUS, SERVER_ENCRYPTED,
    },
    Etag, LeaseStatus,
};
use typespec_client_core::fmt::SafeDebug;

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
#[derive(Clone, Default, SafeDebug)]
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

        let access_tier: Option<AccessTier> = headers.get_optional_as(&BLOB_ACCESS_TIER).unwrap();
        properties.access_tier = access_tier;

        let blob_type: Option<BlobType> = headers.get_optional_as(&BLOB_TYPE).unwrap();
        properties.blob_type = blob_type;

        let content_length: Option<i64> = headers.get_optional_as(&CONTENT_LENGTH).unwrap();
        properties.content_length = content_length;

        let content_md5: Option<String> = headers.get_optional_as(&CONTENT_MD5).unwrap();
        properties.content_md5 = content_md5;

        let content_type: Option<String> = headers.get_optional_as(&CONTENT_TYPE).unwrap();
        properties.content_type = content_type;

        let creation_time: Option<String> = headers.get_optional_as(&CREATION_TIME).unwrap();
        properties.creation_time = creation_time;

        let etag: Option<Etag> = headers.get_optional_as(&ETAG).unwrap();
        properties.etag = etag;

        let last_modified: Option<String> = headers.get_optional_as(&LAST_MODIFIED).unwrap();
        properties.last_modified = last_modified;

        let lease_state: Option<LeaseState> = headers.get_optional_as(&LEASE_STATE).unwrap();
        properties.lease_state = lease_state;

        let lease_status: Option<LeaseStatus> = headers.get_optional_as(&LEASE_STATUS).unwrap();
        properties.lease_status = lease_status;

        let server_encrypted: Option<bool> = headers.get_optional_as(&SERVER_ENCRYPTED).unwrap();
        properties.server_encrypted = server_encrypted;

        Ok(Some(properties))
    }
}
