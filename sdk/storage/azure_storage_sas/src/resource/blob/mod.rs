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

use crate::builder::{BlobSasOptions, CommonFields, ValidatedKey};
use crate::SAS_VERSION;

/// Builds the blob-service user delegation SAS string-to-sign.
///
/// Used by all blob-service resource types (blob, snapshot, version, container, directory).
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
pub(crate) fn blob_udk_string_to_sign(
    permissions: &str,
    common: &CommonFields,
    options: &BlobSasOptions,
    key: &ValidatedKey<'_>,
    sr: &str,
    canonicalized_resource: &str,
    snapshot_time: &str,
) -> String {
    let saoid = options.authorized_object_id.as_deref().unwrap_or("");
    let suoid = options.unauthorized_object_id.as_deref().unwrap_or("");
    let scid = options.correlation_id.as_deref().unwrap_or("");
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = common.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = common.ip_str();
    let spr = common.protocol_str();
    let ses = options.encryption_scope_str();
    let srh = options.signed_request_headers_str();
    let srq = options.signed_request_query_parameters_str();
    let st = common.start_str();
    let se = common.expiry_str();
    let skt = CommonFields::format_time(key.signed_start);
    let ske = CommonFields::format_time(key.signed_expiry);
    let rscc = options.cache_control.as_deref().unwrap_or("");
    let rscd = options.content_disposition.as_deref().unwrap_or("");
    let rsce = options.content_encoding.as_deref().unwrap_or("");
    let rscl = options.content_language.as_deref().unwrap_or("");
    let rsct = options.content_type.as_deref().unwrap_or("");

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
#[allow(clippy::too_many_arguments)]
pub(crate) fn blob_udk_query_parameters(
    permissions: &str,
    common: &CommonFields,
    options: &BlobSasOptions,
    key: &ValidatedKey<'_>,
    sr: &str,
    snapshot_time: Option<&str>,
    directory_depth: Option<u32>,
    signature: &str,
) -> String {
    let mut parts = Vec::with_capacity(26);
    parts.push(format!("sv={SAS_VERSION}"));
    parts.push(format!("sr={sr}"));
    if let Some(ref start) = common.start {
        parts.push(format!("st={}", CommonFields::format_time(start)));
    }
    parts.push(format!("se={}", common.expiry_str()));
    parts.push(format!("sp={permissions}"));
    if let Some(ref ip) = common.ip_range {
        parts.push(format!("sip={ip}"));
    }
    if let Some(ref proto) = common.protocol {
        parts.push(format!("spr={proto}"));
    }
    parts.push(format!("skoid={}", key.signed_oid));
    parts.push(format!("sktid={}", key.signed_tid));
    parts.push(format!(
        "skt={}",
        CommonFields::format_time(key.signed_start)
    ));
    parts.push(format!(
        "ske={}",
        CommonFields::format_time(key.signed_expiry)
    ));
    parts.push(format!("sks={}", key.signed_service));
    parts.push(format!("skv={}", key.signed_version));
    if let Some(ref v) = options.authorized_object_id {
        parts.push(format!("saoid={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.unauthorized_object_id {
        parts.push(format!("suoid={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.correlation_id {
        parts.push(format!("scid={}", CommonFields::encode(v)));
    }
    if let Some(v) = key.signed_delegated_user_tid {
        parts.push(format!("skdutid={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = common.delegated_user_object_id {
        parts.push(format!("sduoid={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.encryption_scope {
        parts.push(format!("ses={}", CommonFields::encode(v)));
    }
    // The srh/srq query parameter values are comma-separated lists of
    // individually percent-encoded keys. Each key is encoded on its own,
    // then joined with literal commas. The commas are structural separators
    // and must NOT be percent-encoded.
    if let Some(ref headers) = options.signed_request_headers {
        if !headers.is_empty() {
            let encoded_keys: Vec<String> =
                headers.keys().map(|k| CommonFields::encode(k)).collect();
            parts.push(format!("srh={}", encoded_keys.join(",")));
        }
    }
    if let Some(ref params) = options.signed_request_query_parameters {
        if !params.is_empty() {
            let encoded_keys: Vec<String> =
                params.keys().map(|k| CommonFields::encode(k)).collect();
            parts.push(format!("srq={}", encoded_keys.join(",")));
        }
    }
    if let Some(depth) = directory_depth {
        parts.push(format!("sdd={depth}"));
    }
    if let Some(v) = snapshot_time {
        parts.push(format!("snapshot={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.cache_control {
        parts.push(format!("rscc={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.content_disposition {
        parts.push(format!("rscd={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.content_encoding {
        parts.push(format!("rsce={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.content_language {
        parts.push(format!("rscl={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = options.content_type {
        parts.push(format!("rsct={}", CommonFields::encode(v)));
    }
    parts.push(format!("sig={}", CommonFields::encode(signature)));
    parts.join("&")
}
