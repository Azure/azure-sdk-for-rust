// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob-service resource types: blob, container, and directory.
//!
//! # Examples
//!
//! ## Blob user delegation SAS
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey, resource::blob::{BlobResource, BlobPermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(1))?
//!     .blob(BlobResource::new("images", "photo.jpg"), BlobPermissions::new().read())
//!     .content_type("image/jpeg")
//!     .token();
//! # Ok(())
//! # }
//! ```
//!
//! ## Container SAS
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey, resource::blob::{ContainerResource, ContainerPermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(4))?
//!     .container(ContainerResource::new("logs"), ContainerPermissions::new().read().list())
//!     .token();
//! # Ok(())
//! # }
//! ```
//!
//! # SAS field abbreviations
//!
//! The string-to-sign and the emitted query string use the short field names
//! (`sp`, `st`, `se`, `sv`, `sr`, `skoid`, ...) defined by the Storage REST
//! API. For the meaning of each field, see
//! [Create a user delegation SAS](https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas).

mod blob_resource;
mod container_resource;
mod directory_resource;

pub use blob_resource::{BlobPermissions, BlobResource};
pub use container_resource::ContainerResource;
pub use directory_resource::DirectoryResource;

/// Permissions shared by container and directory resources.
///
/// Serialization order: `racwdxyltmeopi`.
pub use container_resource::ContainerPermissions;

use crate::builder::{Fields, ValidatedKey};
use crate::SAS_VERSION;

/// Builds the blob-service user delegation SAS string-to-sign.
///
/// Used by all blob-service resource types (blob, snapshot, version, container, directory).
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
pub(crate) fn blob_udk_string_to_sign(
    permissions: &str,
    fields: &Fields,
    key: &ValidatedKey<'_>,
    sr: &str,
    canonicalized_resource: &str,
    snapshot_time: &str,
) -> String {
    let saoid = fields.authorized_object_id.as_deref().unwrap_or("");
    let suoid = fields.unauthorized_object_id.as_deref().unwrap_or("");
    let scid = fields.correlation_id.as_deref().unwrap_or("");
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = fields.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = fields.ip_str();
    let spr = fields.protocol_str();
    let ses = fields.encryption_scope_str();
    let srh = fields.signed_request_headers_str();
    let srq = fields.signed_request_query_parameters_str();
    let st = fields.start_str();
    let se = fields.expiry_str();
    let skt = Fields::format_time(key.signed_start);
    let ske = Fields::format_time(key.signed_expiry);
    let rscc = fields.cache_control.as_deref().unwrap_or("");
    let rscd = fields.content_disposition.as_deref().unwrap_or("");
    let rsce = fields.content_encoding.as_deref().unwrap_or("");
    let rscl = fields.content_language.as_deref().unwrap_or("");
    let rsct = fields.content_type.as_deref().unwrap_or("");

    #[rustfmt::skip]
    let parts: Vec<&str> = vec![
        permissions,                                              // [0]  signedPermissions
        &st,                                                      // [1]  signedStart
        &se,                                                      // [2]  signedExpiry
        canonicalized_resource,                                   // [3]  canonicalizedResource
        key.signed_oid,                                           // [4]  signedKeyObjectId
        key.signed_tid,                                           // [5]  signedKeyTenantId
        &skt,                                                     // [6]  signedKeyStart
        &ske,                                                     // [7]  signedKeyExpiry
        key.signed_service,                                       // [8]  signedKeyService
        key.signed_version,                                       // [9]  signedKeyVersion
        saoid,                                                    // [10] signedAuthorizedUserObjectId
        suoid,                                                    // [11] signedUnauthorizedUserObjectId
        scid,                                                     // [12] signedCorrelationId
        skdutid,                                                  // [13] signedDelegatedUserTenantId
        sduoid,                                                   // [14] signedDelegatedUserObjectId
        &sip,                                                     // [15] signedIP
        &spr,                                                     // [16] signedProtocol
        SAS_VERSION,                                              // [17] signedVersion
        sr,                                                       // [18] signedResource
        snapshot_time,                                            // [19] signedSnapshotTime
        &ses,                                                     // [20] signedEncryptionScope
        &srh,                                                     // [21] canonicalizedSignedRequestHeaders
        &srq,                                                     // [22] canonicalizedSignedRequestQueryParameters
        rscc,                                                     // [23] rscc
        rscd,                                                     // [24] rscd
        rsce,                                                     // [25] rsce
        rscl,                                                     // [26] rscl
        rsct,                                                     // [27] rsct
    ];
    parts.join("\n")
}

/// Builds the blob-service user delegation SAS query parameters.
pub(crate) fn blob_udk_query_parameters(
    permissions: &str,
    fields: &Fields,
    key: &ValidatedKey<'_>,
    sr: &str,
    snapshot_time: Option<&str>,
    directory_depth: Option<u32>,
    signature: &str,
) -> String {
    let mut parts = Vec::with_capacity(26);
    parts.push(format!("sv={SAS_VERSION}"));
    parts.push(format!("sr={sr}"));
    if let Some(ref start) = fields.start {
        parts.push(format!("st={}", Fields::format_time(start)));
    }
    parts.push(format!("se={}", fields.expiry_str()));
    parts.push(format!("sp={permissions}"));
    if let Some(ref ip) = fields.ip_range {
        parts.push(format!("sip={ip}"));
    }
    if let Some(ref proto) = fields.protocol {
        parts.push(format!("spr={proto}"));
    }
    parts.push(format!("skoid={}", key.signed_oid));
    parts.push(format!("sktid={}", key.signed_tid));
    parts.push(format!("skt={}", Fields::format_time(key.signed_start)));
    parts.push(format!("ske={}", Fields::format_time(key.signed_expiry)));
    parts.push(format!("sks={}", key.signed_service));
    parts.push(format!("skv={}", key.signed_version));
    if let Some(ref v) = fields.authorized_object_id {
        parts.push(format!("saoid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.unauthorized_object_id {
        parts.push(format!("suoid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.correlation_id {
        parts.push(format!("scid={}", Fields::encode(v)));
    }
    if let Some(v) = key.signed_delegated_user_tid {
        parts.push(format!("skdutid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.delegated_user_object_id {
        parts.push(format!("sduoid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.encryption_scope {
        parts.push(format!("ses={}", Fields::encode(v)));
    }
    // The srh/srq query parameter values are comma-separated lists of
    // individually percent-encoded keys. Each key is encoded on its own,
    // then joined with literal commas. The commas are structural separators
    // and must NOT be percent-encoded.
    if let Some(ref headers) = fields.signed_request_headers {
        if !headers.is_empty() {
            let encoded_keys: Vec<String> = headers.keys().map(|k| Fields::encode(k)).collect();
            parts.push(format!("srh={}", encoded_keys.join(",")));
        }
    }
    if let Some(ref params) = fields.signed_request_query_parameters {
        if !params.is_empty() {
            let encoded_keys: Vec<String> = params.keys().map(|k| Fields::encode(k)).collect();
            parts.push(format!("srq={}", encoded_keys.join(",")));
        }
    }
    if let Some(depth) = directory_depth {
        parts.push(format!("sdd={depth}"));
    }
    if let Some(v) = snapshot_time {
        parts.push(format!("snapshot={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.cache_control {
        parts.push(format!("rscc={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.content_disposition {
        parts.push(format!("rscd={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.content_encoding {
        parts.push(format!("rsce={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.content_language {
        parts.push(format!("rscl={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.content_type {
        parts.push(format!("rsct={}", Fields::encode(v)));
    }
    parts.push(format!("sig={}", Fields::encode(signature)));
    parts.join("&")
}
