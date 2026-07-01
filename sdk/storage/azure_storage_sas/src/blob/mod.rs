// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Blob-service resource types: blob, container, and directory.
//!
//! # Examples
//!
//! ## Blob user delegation SAS
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(1))?
//!     .blob("images", "photo.jpg")
//!     .read()
//!     .content_type("image/jpeg")
//!     .build();
//! # Ok(())
//! # }
//! ```
//!
//! ## Container SAS
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, UserDelegationKey};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(4))?
//!     .container("logs")
//!     .read()
//!     .list()
//!     .build();
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

pub(crate) use blob_resource::{BlobPermissions, BlobResource};
pub(crate) use container_resource::ContainerPermissions;
pub(crate) use container_resource::ContainerResource;
pub(crate) use directory_resource::DirectoryResource;

use crate::builder::SasBuilder;
use crate::common::sealed::Sealed;
use crate::common::{CommonFields, SasResource, ValidatedKey};
use crate::SAS_VERSION;
use options_access::BlobOptions;
use std::collections::BTreeMap;

/// Marker trait for blob-service typestate markers.
///
/// Types implementing this trait support response header overrides and
/// other blob-service-specific SAS fields.
pub trait BlobServiceState: Sealed {}
impl BlobServiceState for BlobState {}
impl BlobServiceState for ContainerState {}
impl BlobServiceState for DirectoryState {}

/// State after selecting a blob resource.
pub struct BlobState {
    pub(crate) resource: BlobResource,
    pub(crate) permissions: BlobPermissions,
    pub(crate) options: BlobSasOptions,
}

/// State after selecting a container resource.
pub struct ContainerState {
    pub(crate) resource: ContainerResource,
    pub(crate) permissions: ContainerPermissions,
    pub(crate) options: BlobSasOptions,
}

/// State after selecting a directory resource.
pub struct DirectoryState {
    pub(crate) resource: DirectoryResource,
    pub(crate) permissions: ContainerPermissions,
    pub(crate) options: BlobSasOptions,
}

impl Sealed for BlobState {}
impl Sealed for ContainerState {}
impl Sealed for DirectoryState {}

/// Blob-service-only SAS options, owned by each blob-service typestate.
///
/// Keeping these separate from the shared `CommonFields` means the queue
/// builder can never carry blob-only fields, and adding a new blob option
/// only touches this module.
#[derive(Default)]
pub(crate) struct BlobSasOptions {
    pub encryption_scope: Option<String>,
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_type: Option<String>,
    pub authorized_object_id: Option<String>,
    pub unauthorized_object_id: Option<String>,
    pub correlation_id: Option<String>,
    pub signed_request_headers: Option<BTreeMap<String, String>>,
    pub signed_request_query_parameters: Option<BTreeMap<String, String>>,
}

impl BlobSasOptions {
    pub fn encryption_scope_str(&self) -> String {
        self.encryption_scope.clone().unwrap_or_default()
    }

    /// Canonicalizes signed request headers for the string-to-sign.
    ///
    /// Each entry is formatted as `key:value\n`. Returns an empty string
    /// when no headers are set.
    pub fn signed_request_headers_str(&self) -> String {
        match &self.signed_request_headers {
            Some(headers) if !headers.is_empty() => {
                let mut s = String::new();
                for (k, v) in headers {
                    s.push_str(k);
                    s.push(':');
                    s.push_str(v);
                    s.push('\n');
                }
                s
            }
            _ => String::new(),
        }
    }

    /// Canonicalizes signed request query parameters for the string-to-sign.
    ///
    /// Each entry is formatted as `\nkey:value`. Returns an empty string
    /// when no parameters are set.
    pub fn signed_request_query_parameters_str(&self) -> String {
        match &self.signed_request_query_parameters {
            Some(params) if !params.is_empty() => {
                let mut s = String::new();
                for (k, v) in params {
                    s.push('\n');
                    s.push_str(k);
                    s.push(':');
                    s.push_str(v);
                }
                s
            }
            _ => String::new(),
        }
    }
}

/// Sealed accessor granting the blob-service setters mutable access to each
/// state's [`BlobSasOptions`] without exposing the field publicly.
///
/// The blob-service state markers are publicly re-exported, so the
/// conservative `private_interfaces` lint flags this crate-internal accessor
/// even though `BlobSasOptions` is unreachable outside the crate.
mod options_access {
    #![allow(private_interfaces)]
    use super::{BlobSasOptions, BlobState, ContainerState, DirectoryState};

    pub(crate) trait BlobOptions {
        fn options_mut(&mut self) -> &mut BlobSasOptions;
    }
    impl BlobOptions for BlobState {
        fn options_mut(&mut self) -> &mut BlobSasOptions {
            &mut self.options
        }
    }
    impl BlobOptions for ContainerState {
        fn options_mut(&mut self) -> &mut BlobSasOptions {
            &mut self.options
        }
    }
    impl BlobOptions for DirectoryState {
        fn options_mut(&mut self) -> &mut BlobSasOptions {
            &mut self.options
        }
    }
}

// Blob-service-specific builder setters.
//
// The `BlobOptions` bound grants mutable access to each blob state's
// `BlobSasOptions` without exposing it on the public `BlobServiceState` trait.
#[allow(private_bounds)]
impl<S: BlobServiceState + BlobOptions> SasBuilder<'_, S> {
    /// Sets the encryption scope for the SAS.
    pub fn encryption_scope(mut self, scope: impl Into<String>) -> Self {
        self.state.options_mut().encryption_scope = Some(scope.into());
        self
    }

    /// Sets the `Cache-Control` response header override.
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().cache_control = Some(value.into());
        self
    }

    /// Sets the `Content-Disposition` response header override.
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().content_disposition = Some(value.into());
        self
    }

    /// Sets the `Content-Encoding` response header override.
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().content_encoding = Some(value.into());
        self
    }

    /// Sets the `Content-Language` response header override.
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().content_language = Some(value.into());
        self
    }

    /// Sets the `Content-Type` response header override.
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().content_type = Some(value.into());
        self
    }

    /// Sets the authorized AAD object ID (saoid).
    pub fn authorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().authorized_object_id = Some(value.into());
        self
    }

    /// Sets the unauthorized AAD object ID (suoid).
    pub fn unauthorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().unauthorized_object_id = Some(value.into());
        self
    }

    /// Sets the correlation ID (scid).
    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.state.options_mut().correlation_id = Some(value.into());
        self
    }

    /// Adds a signed request header constraint (srh).
    ///
    /// When set, requests using the SAS must include each specified header
    /// with the given value. Multiple headers can be added by calling this
    /// method repeatedly. Headers are sorted by key in the string-to-sign.
    pub fn signed_request_header(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.state
            .options_mut()
            .signed_request_headers
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Adds a signed request query parameter constraint (srq).
    ///
    /// When set, requests using the SAS must include each specified query
    /// parameter with the given value. Multiple parameters can be added by
    /// calling this method repeatedly. Parameters are sorted by key in the
    /// string-to-sign.
    pub fn signed_request_query_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.state
            .options_mut()
            .signed_request_query_parameters
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }
}

/// Permission and target setters for a blob SAS, gated on [`BlobState`].
impl SasBuilder<'_, BlobState> {
    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.state.permissions.read = true;
        self
    }

    /// Enables add permission.
    pub fn add(mut self) -> Self {
        self.state.permissions.add = true;
        self
    }

    /// Enables create permission.
    pub fn create(mut self) -> Self {
        self.state.permissions.create = true;
        self
    }

    /// Enables write permission.
    pub fn write(mut self) -> Self {
        self.state.permissions.write = true;
        self
    }

    /// Enables delete permission.
    pub fn delete(mut self) -> Self {
        self.state.permissions.delete = true;
        self
    }

    /// Enables delete version permission.
    pub fn delete_version(mut self) -> Self {
        self.state.permissions.delete_version = true;
        self
    }

    /// Enables permanent delete permission.
    pub fn permanent_delete(mut self) -> Self {
        self.state.permissions.permanent_delete = true;
        self
    }

    /// Enables tags permission.
    pub fn tags(mut self) -> Self {
        self.state.permissions.tags = true;
        self
    }

    /// Enables move blob permission.
    pub fn move_blob(mut self) -> Self {
        self.state.permissions.move_blob = true;
        self
    }

    /// Enables execute permission.
    pub fn execute(mut self) -> Self {
        self.state.permissions.execute = true;
        self
    }

    /// Enables ownership permission.
    pub fn ownership(mut self) -> Self {
        self.state.permissions.ownership = true;
        self
    }

    /// Enables permissions permission.
    pub fn permissions(mut self) -> Self {
        self.state.permissions.permissions = true;
        self
    }

    /// Enables set immutability policy permission.
    pub fn set_immutability_policy(mut self) -> Self {
        self.state.permissions.set_immutability_policy = true;
        self
    }

    /// Targets a specific snapshot of the blob (`sr=bs`).
    ///
    /// `snapshot` is the snapshot timestamp (e.g.,
    /// `"2025-01-15T12:00:00.0000000Z"`). It is emitted as the `snapshot=`
    /// query parameter of the token.
    pub fn snapshot(mut self, snapshot: impl Into<String>) -> Self {
        self.state.resource.snapshot = Some(snapshot.into());
        self
    }

    /// Targets a specific version of the blob (`sr=bv`).
    ///
    /// The version ID is not included in the SAS token; it must travel on the
    /// request URL as a `versionid=` query parameter. Append the token to a URL
    /// that already carries `versionid=`.
    pub fn version(mut self, version_id: impl Into<String>) -> Self {
        self.state.resource.version_id = Some(version_id.into());
        self
    }
}

/// Sealed accessor granting the shared container/directory permission setters
/// mutable access to each state's [`ContainerPermissions`] without exposing the
/// field publicly.
mod permissions_access {
    #![allow(private_interfaces)]
    use super::{ContainerPermissions, ContainerState, DirectoryState};

    pub(crate) trait ContainerPermsAccess {
        fn permissions_mut(&mut self) -> &mut ContainerPermissions;
    }
    impl ContainerPermsAccess for ContainerState {
        fn permissions_mut(&mut self) -> &mut ContainerPermissions {
            &mut self.permissions
        }
    }
    impl ContainerPermsAccess for DirectoryState {
        fn permissions_mut(&mut self) -> &mut ContainerPermissions {
            &mut self.permissions
        }
    }
}
use permissions_access::ContainerPermsAccess;

/// Permission setters shared by container and directory SAS.
///
/// The `ContainerPermsAccess` bound grants mutable access to each state's
/// container permission set without exposing it publicly. `BlobState` does not
/// implement the bound, so its own `read`/`write`/... setters do not conflict.
#[allow(private_bounds)]
impl<S: ContainerPermsAccess> SasBuilder<'_, S> {
    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.state.permissions_mut().read = true;
        self
    }

    /// Enables add permission.
    pub fn add(mut self) -> Self {
        self.state.permissions_mut().add = true;
        self
    }

    /// Enables create permission.
    pub fn create(mut self) -> Self {
        self.state.permissions_mut().create = true;
        self
    }

    /// Enables write permission.
    pub fn write(mut self) -> Self {
        self.state.permissions_mut().write = true;
        self
    }

    /// Enables delete permission.
    pub fn delete(mut self) -> Self {
        self.state.permissions_mut().delete = true;
        self
    }

    /// Enables delete version permission.
    pub fn delete_version(mut self) -> Self {
        self.state.permissions_mut().delete_version = true;
        self
    }

    /// Enables permanent delete permission.
    pub fn permanent_delete(mut self) -> Self {
        self.state.permissions_mut().permanent_delete = true;
        self
    }

    /// Enables list permission.
    pub fn list(mut self) -> Self {
        self.state.permissions_mut().list = true;
        self
    }

    /// Enables tags permission.
    pub fn tags(mut self) -> Self {
        self.state.permissions_mut().tags = true;
        self
    }

    /// Enables move blob permission.
    pub fn move_blob(mut self) -> Self {
        self.state.permissions_mut().move_blob = true;
        self
    }

    /// Enables execute permission.
    pub fn execute(mut self) -> Self {
        self.state.permissions_mut().execute = true;
        self
    }

    /// Enables ownership permission.
    pub fn ownership(mut self) -> Self {
        self.state.permissions_mut().ownership = true;
        self
    }

    /// Enables permissions permission.
    pub fn permissions(mut self) -> Self {
        self.state.permissions_mut().permissions = true;
        self
    }

    /// Enables set immutability policy permission.
    pub fn set_immutability_policy(mut self) -> Self {
        self.state.permissions_mut().set_immutability_policy = true;
        self
    }
}

impl SasResource for BlobState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        // The string-to-sign snapshot slot carries the snapshot timestamp or,
        // for a version SAS (`sr=bv`), the version ID. The `snapshot=` query
        // parameter remains snapshot-only; the version ID is not emitted there.
        blob_udk_string_to_sign(
            &sp,
            common,
            &self.options,
            key,
            self.resource.signed_resource(),
            &canonical,
            self.resource.snapshot_or_version_time().unwrap_or(""),
        )
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        blob_udk_query_parameters(
            &sp,
            common,
            &self.options,
            key,
            self.resource.signed_resource(),
            self.resource.snapshot_time(),
            None,
            signature,
        )
    }
}

impl SasResource for ContainerState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        blob_udk_string_to_sign(&sp, common, &self.options, key, "c", &canonical, "")
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        blob_udk_query_parameters(&sp, common, &self.options, key, "c", None, None, signature)
    }
}

impl SasResource for DirectoryState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        blob_udk_string_to_sign(&sp, common, &self.options, key, "d", &canonical, "")
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        let depth = self.resource.depth();
        blob_udk_query_parameters(
            &sp,
            common,
            &self.options,
            key,
            "d",
            None,
            Some(depth),
            signature,
        )
    }
}

/// Builds the blob-service user delegation SAS string-to-sign.
///
/// Used by all blob-service resource types (blob, snapshot, version, container, directory).
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
fn blob_udk_string_to_sign(
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
fn blob_udk_query_parameters(
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
        parts.push(format!("sip={}", ip.sip_value()));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::{test_common, test_udk};
    use time::macros::datetime;

    // ---- String-to-sign layout tests ----
    //
    // These pin the exact field positions of the string-to-sign against the
    // service spec. A reordering or a wrong source for any field (e.g., the
    // `skdutid`/version-slot bugs) would change a line index and fail here.

    #[test]
    fn blob_string_to_sign_has_28_fields_in_order() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("dut".into());
        let key = ValidatedKey::from_key(&udk).unwrap();
        let mut common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        common.start = Some(datetime!(2025-05-01 08:00:00 UTC));
        common.delegated_user_object_id = Some("duoid".into());
        let options = BlobSasOptions {
            authorized_object_id: Some("saoid".into()),
            unauthorized_object_id: Some("suoid".into()),
            correlation_id: Some("scid".into()),
            encryption_scope: Some("ses".into()),
            cache_control: Some("rscc".into()),
            content_disposition: Some("rscd".into()),
            content_encoding: Some("rsce".into()),
            content_language: Some("rscl".into()),
            content_type: Some("rsct".into()),
            ..Default::default()
        };

        let sts = blob_udk_string_to_sign("rw", &common, &options, &key, "b", "/blob/acct/c/b", "");
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 28, "blob STS must have exactly 28 fields");
        assert_eq!(lines[0], "rw"); // sp
        assert_eq!(lines[1], "2025-05-01T08:00:00Z"); // st
        assert_eq!(lines[2], "2025-06-01T12:00:00Z"); // se
        assert_eq!(lines[3], "/blob/acct/c/b"); // cr
        assert_eq!(lines[4], "oid-value"); // skoid
        assert_eq!(lines[5], "tid-value"); // sktid
        assert_eq!(lines[6], "2025-01-15T00:00:00Z"); // skt
        assert_eq!(lines[7], "2025-01-16T00:00:00Z"); // ske
        assert_eq!(lines[8], "b"); // sks
        assert_eq!(lines[9], "2025-11-05"); // skv (from key, not SAS_VERSION)
        assert_eq!(lines[10], "saoid"); // saoid
        assert_eq!(lines[11], "suoid"); // suoid
        assert_eq!(lines[12], "scid"); // scid
        assert_eq!(lines[13], "dut"); // skdutid (from key)
        assert_eq!(lines[14], "duoid"); // sduoid (from builder)
        assert_eq!(lines[17], "2026-04-06"); // sv (SAS_VERSION)
        assert_eq!(lines[18], "b"); // sr
        assert_eq!(lines[19], ""); // snapshot
        assert_eq!(lines[20], "ses"); // ses
        assert_eq!(lines[23], "rscc"); // rscc
        assert_eq!(lines[24], "rscd"); // rscd
        assert_eq!(lines[25], "rsce"); // rsce
        assert_eq!(lines[26], "rscl"); // rscl
        assert_eq!(lines[27], "rsct"); // rsct
    }

    #[test]
    fn blob_version_string_to_sign_places_version_in_snapshot_slot() {
        // Regression for the version-SAS bug: the version id must occupy the
        // snapshot slot (index 19) of the string-to-sign for `sr=bv`.
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let options = BlobSasOptions::default();
        let sts = blob_udk_string_to_sign(
            "r",
            &common,
            &options,
            &key,
            "bv",
            "/blob/acct/c/b",
            "2025-01-15T12:00:00.0000000Z",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[13], ""); // skdutid empty when key omits it
        assert_eq!(lines[18], "bv"); // sr
        assert_eq!(lines[19], "2025-01-15T12:00:00.0000000Z"); // version id in snapshot slot
    }

    #[test]
    fn blob_snapshot_string_to_sign_places_snapshot_in_slot() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let options = BlobSasOptions::default();
        let sts = blob_udk_string_to_sign(
            "r",
            &common,
            &options,
            &key,
            "bs",
            "/blob/acct/c/b",
            "2025-02-20T08:30:00.0000000Z",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[18], "bs"); // sr
        assert_eq!(lines[19], "2025-02-20T08:30:00.0000000Z"); // snapshot in slot
    }
}
