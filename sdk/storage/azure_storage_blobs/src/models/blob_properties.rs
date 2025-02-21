// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    headers::{
        FromHeaders, HeaderName, Headers, BLOB_ACCESS_TIER, BLOB_TYPE, CREATION_TIME, LEASE_STATE,
        LEASE_STATUS, SERVER_ENCRYPTED,
    },
    Error, Etag, LeaseStatus,
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
    type Error = Error;
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

    fn from_headers(headers: &Headers) -> Result<Option<Self>, Error> {
        let mut properties = BlobProperties {
            ..Default::default()
        };

        let access_tier_inferred: Option<bool> =
            headers.get_optional_as(&BLOB_ACCESS_TIER_INFERRED)?;
        properties.access_tier_inferred = access_tier_inferred;

        let access_tier: Option<AccessTier> = headers.get_optional_as(&BLOB_ACCESS_TIER)?;
        properties.access_tier = access_tier;

        let blob_type: Option<BlobType> = headers.get_optional_as(&BLOB_TYPE)?;
        properties.blob_type = blob_type;

        let content_length: Option<i64> = headers.get_optional_as(&CONTENT_LENGTH)?;
        properties.content_length = content_length;

        let content_md5 = headers.get_optional_str(&CONTENT_MD5);
        properties.content_md5 = content_md5.map(|s| s.to_string());

        let content_type = headers.get_optional_str(&CONTENT_TYPE);
        properties.content_type = content_type.map(|s| s.to_string());

        let creation_time = headers.get_optional_str(&CREATION_TIME);
        properties.creation_time = creation_time.map(|s| s.to_string());

        let etag: Option<Etag> = headers.get_optional_as(&ETAG)?;
        properties.etag = etag;

        let last_modified = headers.get_optional_str(&LAST_MODIFIED);
        properties.last_modified = last_modified.map(|s| s.to_string());

        let lease_state: Option<LeaseState> = headers.get_optional_as(&LEASE_STATE)?;
        properties.lease_state = lease_state;

        let lease_status: Option<LeaseStatus> = headers.get_optional_as(&LEASE_STATUS)?;
        properties.lease_status = lease_status;

        let server_encrypted: Option<bool> = headers.get_optional_as(&SERVER_ENCRYPTED)?;
        properties.server_encrypted = server_encrypted;

        Ok(Some(properties))
    }
}
