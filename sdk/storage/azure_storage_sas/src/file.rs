// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Files resource types for user delegation SAS.
//!
//! | Type | `sr` | Scope |
//! |------|------|-------|
//! | [`FileState`] | `f` | A single file |
//! | [`ShareState`] | `s` | All files in a share |
//!
//! <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas>

use crate::builder::{SasTokenBuilder, SasUrlBuilder};
use crate::common::sealed::Sealed;
use crate::common::{CommonFields, SasResource, ValidatedKey};
use crate::SAS_VERSION;

/// Minimum API version for Azure Files user delegation SAS.
pub const FILE_MIN_VERSION: &str = "2025-07-05";

pub(crate) struct FileResource {
    share: String,
    path: String,
}

impl FileResource {
    pub(crate) fn new(share: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            share: share.into(),
            path: path.into().trim_matches('/').to_string(),
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/file/{}/{}/{}", account, self.share, self.path)
    }

    /// Returns individually encodable path segments.
    ///
    /// The path within the share may contain `/`-separated components; each is
    /// returned as a separate segment so the URL crate encodes them individually.
    pub(crate) fn url_path_segments(&self) -> Vec<&str> {
        let mut segments = vec![self.share.as_str()];
        if !self.path.is_empty() {
            segments.extend(self.path.split('/'));
        }
        segments
    }
}

pub(crate) struct ShareResource {
    share: String,
}

impl ShareResource {
    pub(crate) fn new(share: impl Into<String>) -> Self {
        Self {
            share: share.into(),
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/file/{}/{}", account, self.share)
    }

    pub(crate) fn url_path_segments(&self) -> [&str; 1] {
        [&self.share]
    }
}

/// Shared response-header override options for Azure Files SAS resources.
#[derive(Default)]
pub(crate) struct FileSasOptions {
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_type: Option<String>,
}

/// Permissions for a file SAS.
///
/// Serialization order: `rwd`.
#[derive(Clone, Copy, Default)]
pub(crate) struct FilePermissions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) delete: bool,
}

impl FilePermissions {
    pub(crate) fn to_sas_str(self) -> String {
        let mut s = String::with_capacity(3);
        if self.read {
            s.push('r');
        }
        if self.write {
            s.push('w');
        }
        if self.delete {
            s.push('d');
        }
        s
    }
}

/// Permissions for a share SAS.
///
/// Serialization order: `rwdlc`.
#[derive(Clone, Copy, Default)]
pub(crate) struct SharePermissions {
    pub(crate) read: bool,
    pub(crate) write: bool,
    pub(crate) delete: bool,
    pub(crate) list: bool,
    pub(crate) create: bool,
}

impl SharePermissions {
    pub(crate) fn to_sas_str(self) -> String {
        let mut s = String::with_capacity(5);
        if self.read {
            s.push('r');
        }
        if self.write {
            s.push('w');
        }
        if self.delete {
            s.push('d');
        }
        if self.list {
            s.push('l');
        }
        if self.create {
            s.push('c');
        }
        s
    }
}

/// State after selecting a file resource.
pub struct FileState {
    pub(crate) resource: FileResource,
    pub(crate) permissions: FilePermissions,
    pub(crate) options: FileSasOptions,
}

/// State after selecting a share resource.
pub struct ShareState {
    pub(crate) resource: ShareResource,
    pub(crate) permissions: SharePermissions,
    pub(crate) options: FileSasOptions,
}

impl Sealed for FileState {}
impl Sealed for ShareState {}

impl SasTokenBuilder<'_, FileState> {
    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.state.permissions.read = true;
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
    /// Sets the `Cache-Control` response header override (`rscc`).
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.state.options.cache_control = Some(value.into());
        self
    }
    /// Sets the `Content-Disposition` response header override (`rscd`).
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_disposition = Some(value.into());
        self
    }
    /// Sets the `Content-Encoding` response header override (`rsce`).
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_encoding = Some(value.into());
        self
    }
    /// Sets the `Content-Language` response header override (`rscl`).
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_language = Some(value.into());
        self
    }
    /// Sets the `Content-Type` response header override (`rsct`).
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_type = Some(value.into());
        self
    }
}

impl SasUrlBuilder<'_, FileState> {
    /// Enables read permission.
    pub fn read(self) -> Self {
        self.map(|b| b.read())
    }
    /// Enables write permission.
    pub fn write(self) -> Self {
        self.map(|b| b.write())
    }
    /// Enables delete permission.
    pub fn delete(self) -> Self {
        self.map(|b| b.delete())
    }
    /// Sets the `Cache-Control` response header override (`rscc`).
    pub fn cache_control(self, value: impl Into<String>) -> Self {
        self.map(|b| b.cache_control(value))
    }
    /// Sets the `Content-Disposition` response header override (`rscd`).
    pub fn content_disposition(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_disposition(value))
    }
    /// Sets the `Content-Encoding` response header override (`rsce`).
    pub fn content_encoding(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_encoding(value))
    }
    /// Sets the `Content-Language` response header override (`rscl`).
    pub fn content_language(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_language(value))
    }
    /// Sets the `Content-Type` response header override (`rsct`).
    pub fn content_type(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_type(value))
    }
}

impl SasTokenBuilder<'_, ShareState> {
    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.state.permissions.read = true;
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
    /// Enables list permission.
    pub fn list(mut self) -> Self {
        self.state.permissions.list = true;
        self
    }
    /// Enables create permission.
    pub fn create(mut self) -> Self {
        self.state.permissions.create = true;
        self
    }
    /// Sets the `Cache-Control` response header override (`rscc`).
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.state.options.cache_control = Some(value.into());
        self
    }
    /// Sets the `Content-Disposition` response header override (`rscd`).
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_disposition = Some(value.into());
        self
    }
    /// Sets the `Content-Encoding` response header override (`rsce`).
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_encoding = Some(value.into());
        self
    }
    /// Sets the `Content-Language` response header override (`rscl`).
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_language = Some(value.into());
        self
    }
    /// Sets the `Content-Type` response header override (`rsct`).
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.state.options.content_type = Some(value.into());
        self
    }
}

impl SasUrlBuilder<'_, ShareState> {
    /// Enables read permission.
    pub fn read(self) -> Self {
        self.map(|b| b.read())
    }
    /// Enables write permission.
    pub fn write(self) -> Self {
        self.map(|b| b.write())
    }
    /// Enables delete permission.
    pub fn delete(self) -> Self {
        self.map(|b| b.delete())
    }
    /// Enables list permission.
    pub fn list(self) -> Self {
        self.map(|b| b.list())
    }
    /// Enables create permission.
    pub fn create(self) -> Self {
        self.map(|b| b.create())
    }
    /// Sets the `Cache-Control` response header override (`rscc`).
    pub fn cache_control(self, value: impl Into<String>) -> Self {
        self.map(|b| b.cache_control(value))
    }
    /// Sets the `Content-Disposition` response header override (`rscd`).
    pub fn content_disposition(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_disposition(value))
    }
    /// Sets the `Content-Encoding` response header override (`rsce`).
    pub fn content_encoding(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_encoding(value))
    }
    /// Sets the `Content-Language` response header override (`rscl`).
    pub fn content_language(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_language(value))
    }
    /// Sets the `Content-Type` response header override (`rsct`).
    pub fn content_type(self, value: impl Into<String>) -> Self {
        self.map(|b| b.content_type(value))
    }
}

impl SasResource for FileState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        file_udk_string_to_sign(&sp, common, key, &canonical, "f", &self.options)
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        file_udk_query_parameters(&sp, common, key, signature, "f", &self.options)
    }

    fn url_path_segments(&self) -> Vec<&str> {
        self.resource.url_path_segments()
    }

    fn default_endpoint(account: &str) -> azure_core::Result<url::Url> {
        crate::url::file_endpoint(account)
    }
}

impl SasResource for ShareState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        file_udk_string_to_sign(&sp, common, key, &canonical, "s", &self.options)
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        file_udk_query_parameters(&sp, common, key, signature, "s", &self.options)
    }

    fn url_path_segments(&self) -> Vec<&str> {
        self.resource.url_path_segments().to_vec()
    }

    fn default_endpoint(account: &str) -> azure_core::Result<url::Url> {
        crate::url::file_endpoint(account)
    }
}

fn file_udk_string_to_sign(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    canonicalized_resource: &str,
    sr: &str,
    options: &FileSasOptions,
) -> String {
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = common.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = common.ip_str();
    let spr = common.protocol_str();
    let st = common.start_str();
    let se = common.expiry_str();
    let skt = CommonFields::format_time(key.signed_start);
    let ske = CommonFields::format_time(key.signed_expiry);
    let rscc = options.cache_control.as_deref().unwrap_or("");
    let rscd = options.content_disposition.as_deref().unwrap_or("");
    let rsce = options.content_encoding.as_deref().unwrap_or("");
    let rscl = options.content_language.as_deref().unwrap_or("");
    let rsct = options.content_type.as_deref().unwrap_or("");
    let _ = sr; // sr is in query params but not string-to-sign for files

    #[rustfmt::skip]
    let parts: Vec<&str> = vec![
        permissions,            // [0]  signedPermissions
        &st,                    // [1]  signedStart
        &se,                    // [2]  signedExpiry
        canonicalized_resource, // [3]  canonicalizedResource
        key.signed_oid,         // [4]  signedKeyObjectId
        key.signed_tid,         // [5]  signedKeyTenantId
        &skt,                   // [6]  signedKeyStart
        &ske,                   // [7]  signedKeyExpiry
        key.signed_service,     // [8]  signedKeyService
        key.signed_version,     // [9]  signedKeyVersion
        skdutid,                // [10] signedDelegatedUserTenantId (from key)
        sduoid,                 // [11] signedDelegatedUserObjectId
        &sip,                   // [12] signedIP
        &spr,                   // [13] signedProtocol
        SAS_VERSION,            // [14] signedVersion
        rscc,                   // [15] rscc
        rscd,                   // [16] rscd
        rsce,                   // [17] rsce
        rscl,                   // [18] rscl
        rsct,                   // [19] rsct
    ];
    parts.join("\n")
}

fn file_udk_query_parameters(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    signature: &str,
    sr: &str,
    options: &FileSasOptions,
) -> String {
    let mut parts = Vec::with_capacity(20);
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
    if let Some(v) = key.signed_delegated_user_tid {
        parts.push(format!("skdutid={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = common.delegated_user_object_id {
        parts.push(format!("sduoid={}", CommonFields::encode(v)));
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

    #[test]
    fn file_string_to_sign_has_20_fields() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let options = FileSasOptions::default();
        let sts =
            file_udk_string_to_sign("rw", &common, &key, "/file/acct/share/path", "f", &options);
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 20, "file STS must have exactly 20 fields");
        assert_eq!(lines[0], "rw");
        assert_eq!(lines[3], "/file/acct/share/path");
        assert_eq!(lines[14], "2026-04-06");
        assert_eq!(lines[15], "");
        assert_eq!(lines[19], "");
    }

    #[test]
    fn share_string_to_sign_has_20_fields() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let options = FileSasOptions::default();
        let sts =
            file_udk_string_to_sign("rwdlc", &common, &key, "/file/acct/share", "s", &options);
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 20);
        assert_eq!(lines[3], "/file/acct/share");
    }
}
