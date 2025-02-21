// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    headers::{
        FromHeaders, HeaderName, Headers, ETAG, HAS_IMMUTABILITY_POLICY, HAS_LEGAL_HOLD,
        LEASE_STATE, LEASE_STATUS, VERSION,
    },
    Error, Etag, LeaseStatus,
};
use typespec_client_core::fmt::SafeDebug;

use crate::models::LeaseState;

pub const LAST_MODIFIED: HeaderName = HeaderName::from_static("last-modified");
pub const IMMUTABLE_STORAGE_WITH_VERSIONING_ENABLED: HeaderName =
    HeaderName::from_static("x-ms-immutable-storage-with-versioning-enabled");

/// Properties of an Azure Storage container.
///
#[derive(Clone, Default, SafeDebug)]
pub struct ContainerProperties {
    pub last_modified: Option<String>,
    pub lease_state: Option<LeaseState>,
    pub lease_status: Option<LeaseStatus>,
    pub has_immutability_policy: Option<bool>,
    pub has_legal_hold: Option<bool>,
    pub immutable_storage_with_versioning_enabled: Option<bool>,
    pub etag: Option<Etag>,
    pub version: Option<String>,
}

impl FromHeaders for ContainerProperties {
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
        let mut properties = ContainerProperties {
            ..Default::default()
        };

        let last_modified = headers.get_optional_str(&LAST_MODIFIED);
        properties.last_modified = last_modified.map(|s| s.to_string());

        let lease_state: Option<LeaseState> = headers.get_optional_as(&LEASE_STATE)?;
        properties.lease_state = lease_state;

        let lease_status: Option<LeaseStatus> = headers.get_optional_as(&LEASE_STATUS)?;
        properties.lease_status = lease_status;

        let has_immutability_policy = headers.get_optional_as(&HAS_IMMUTABILITY_POLICY)?;
        properties.has_immutability_policy = has_immutability_policy;

        let has_legal_hold = headers.get_optional_as(&HAS_LEGAL_HOLD)?;
        properties.has_legal_hold = has_legal_hold;

        let immutable_storage_with_versioning_enabled =
            headers.get_optional_as(&IMMUTABLE_STORAGE_WITH_VERSIONING_ENABLED)?;
        properties.immutable_storage_with_versioning_enabled =
            immutable_storage_with_versioning_enabled;

        let etag: Option<Etag> = headers.get_optional_as(&ETAG)?;
        properties.etag = etag;

        let version = headers.get_optional_str(&VERSION);
        properties.version = version.map(|s| s.to_string());

        Ok(Some(properties))
    }
}
