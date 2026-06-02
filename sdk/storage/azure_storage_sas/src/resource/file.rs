//! Azure Files SAS resources.
//!
//! Provides resource types for the two addressable Azure Files scopes:
//!
//! | Type | `sr` | Scope |
//! |------|------|-------|
//! | [`FileResource`] | `f` | A single file |
//! | [`ShareResource`] | `s` | All files in a share |
//!
//! Permissions are expressed with [`Resource::Permissions`](crate::resource::Resource::Permissions) and encoded in spec order (`rwdl`).
//!
//! Requires API version [`FILE_MIN_VERSION`] or later.
//!
//! <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signed-resource-field-azure-files-only>

use std::fmt;

use url::Url;

use crate::{
    error::SasError,
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

pub mod share;
pub use share::{ShareResource, ShareSasPermissions};

/// Minimum API version for File Storage user delegation SAS.
pub const FILE_MIN_VERSION: &str = "2025-07-05";

/// Permissions for an Azure Files SAS token.
///
/// Permissions are emitted in spec order: `rwd`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#permissions-for-a-file>
#[derive(Default)]
pub struct FileSasPermissions {
    /// Read file content, properties, and metadata.
    pub read: bool,
    /// Write file content, properties, and metadata.
    pub write: bool,
    /// Delete a file.
    pub delete: bool,
}

impl fmt::Display for FileSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.read {
            write!(f, "r")?;
        }
        if self.write {
            write!(f, "w")?;
        }
        if self.delete {
            write!(f, "d")?;
        }
        Ok(())
    }
}

/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
pub(crate) struct FileStringToSign<'a> {
    pub ctx: &'a SasSigningContext<'a>,
    pub cache_control: Option<&'a str>,
    pub content_disposition: Option<&'a str>,
    pub content_encoding: Option<&'a str>,
    pub content_language: Option<&'a str>,
    pub content_type: Option<&'a str>,
}

impl std::fmt::Display for FileStringToSign<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctx = self.ctx;
        write!(
            f,
            "{}",
            [
                ctx.permissions,                            // [0]  signedPermissions
                ctx.start,                                  // [1]  signedStart
                ctx.expiry,                                 // [2]  signedExpiry
                ctx.canon,                                  // [3]  canonicalizedResource
                &ctx.key.signed_oid,                        // [4]  signedKeyObjectId
                &ctx.key.signed_tid,                        // [5]  signedKeyTenantId
                &ctx.key.signed_start,                      // [6]  signedKeyStart
                &ctx.key.signed_expiry,                     // [7]  signedKeyExpiry
                &ctx.key.signed_service,                    // [8]  signedKeyService
                &ctx.key.signed_version,                    // [9]  signedKeyVersion
                ctx.delegated_user_tenant_id.unwrap_or(""), // [10] signedKeyDelegatedUserTenantId
                ctx.delegated_user_object_id.unwrap_or(""), // [11] signedDelegatedUserObjectId
                ctx.ip.unwrap_or(""),                       // [12] signedIP
                ctx.protocol.as_str(),                      // [13] signedProtocol
                ctx.version,                                // [14] signedVersion
                self.cache_control.unwrap_or(""),           // [15] rscc
                self.content_disposition.unwrap_or(""),     // [16] rscd
                self.content_encoding.unwrap_or(""),        // [17] rsce
                self.content_language.unwrap_or(""),        // [18] rscl
                self.content_type.unwrap_or(""),            // [19] rsct
            ]
            .join("\n")
        )
    }
}

/// Optional response-header overrides for Azure Files SAS resources.
///
/// Both [`FileResource`] and [`ShareResource`] accept `options: Option<FileResourceOptions>`.
///
/// ```rust,ignore
/// FileResource {
///     share: "myshare".into(),
///     path: "dir/file.txt".into(),
///     options: Some(FileResourceOptions {
///         content_type: Some("text/plain".into()),
///         ..Default::default()
///     }),
/// }
/// ```
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
#[derive(Default)]
pub struct FileResourceOptions {
    /// Override `Cache-Control` response header (`rscc`).
    pub cache_control: Option<String>,
    /// Override `Content-Disposition` response header (`rscd`).
    pub content_disposition: Option<String>,
    /// Override `Content-Encoding` response header (`rsce`).
    pub content_encoding: Option<String>,
    /// Override `Content-Language` response header (`rscl`).
    pub content_language: Option<String>,
    /// Override `Content-Type` response header (`rsct`).
    pub content_type: Option<String>,
}

/// A single file in Azure Files (`sr=f`). Requires API version [`FILE_MIN_VERSION`] or later.
pub struct FileResource {
    pub share: String,
    /// File path within the share (e.g. `"dir/file.txt"`).
    pub path: String,
    pub options: Option<FileResourceOptions>,
}

impl Resource for FileResource {
    type Permissions = FileSasPermissions;
}

impl sealed::DelegatedUserService for FileResource {}

impl sealed::Resource for FileResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.file.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        FILE_MIN_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/file/{}/{}/{}", account, self.share, self.path)
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        let opts = self.options.as_ref();
        FileStringToSign {
            ctx,
            cache_control: opts.and_then(|o| o.cache_control.as_deref()),
            content_disposition: opts.and_then(|o| o.content_disposition.as_deref()),
            content_encoding: opts.and_then(|o| o.content_encoding.as_deref()),
            content_language: opts.and_then(|o| o.content_language.as_deref()),
            content_type: opts.and_then(|o| o.content_type.as_deref()),
        }
        .to_string()
    }
    fn sas_url(&self, account_endpoint: &Url, params: &SasUrlParams<'_>) -> Result<Url, SasError> {
        let mut url = append_path(account_endpoint, &format!("{}/{}", self.share, self.path));
        let opts = self.options.as_ref();
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version).append_pair("sr", "f");
        append_common_sas_params(&mut q, params);
        if let Some(v) = opts.and_then(|o| o.cache_control.as_deref()) {
            q.append_pair("rscc", v);
        }
        if let Some(v) = opts.and_then(|o| o.content_disposition.as_deref()) {
            q.append_pair("rscd", v);
        }
        if let Some(v) = opts.and_then(|o| o.content_encoding.as_deref()) {
            q.append_pair("rsce", v);
        }
        if let Some(v) = opts.and_then(|o| o.content_language.as_deref()) {
            q.append_pair("rscl", v);
        }
        if let Some(v) = opts.and_then(|o| o.content_type.as_deref()) {
            q.append_pair("rsct", v);
        }
        q.append_pair("sig", params.signature);
        drop(q);
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resource::sealed::Resource as _,
        sas::{SasSigningContext, SasUrlParams},
        test_utils::{make_key, url_params, ACCOUNT},
    };

    const SHARE: &str = "testshare";
    const FILE_PATH: &str = "dir/testfile.txt";

    fn make_resource() -> FileResource {
        FileResource {
            share: SHARE.to_string(),
            path: FILE_PATH.to_string(),
            options: None,
        }
    }

    #[test]
    fn test_file_string_to_sign_has_20_fields() {
        let key = make_key();
        let resource = make_resource();
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "rw",
            start: "2024-01-01T00:00:00Z",
            expiry: "2024-01-08T00:00:00Z",
            canon: &canon,
            key: &key,
            version: resource.default_api_version(),
            ip: None,
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(parts.len(), 20, "file string-to-sign must have 20 fields");
        assert_eq!(parts[13], "https,http", "[13] signedProtocol");
        assert_eq!(
            parts[14],
            resource.default_api_version(),
            "[14] signedVersion"
        );
        assert_eq!(parts[15], "", "[15] rscc — empty");
        assert_eq!(parts[19], "", "[19] rsct — empty");
    }

    #[test]
    fn test_file_sas_url_has_sr_param() {
        let key = make_key();
        let resource = make_resource();
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "rw",
            start: "2024-01-01T00:00:00Z",
            expiry: "2024-01-08T00:00:00Z",
            canon: &canon,
            key: &key,
            version: resource.default_api_version(),
            ip: None,
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let sig = key.compute_signature(&s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.file.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "rw",
                    start: Some("2024-01-01T00:00:00Z"),
                    expiry: "2024-01-08T00:00:00Z",
                    key: &key,
                    signature: &sig,
                    version: resource.default_api_version(),
                    ip: None,
                    protocol: Default::default(),
                    authorized_user_object_id: None,
                    unauthorized_user_object_id: None,
                    delegated_user_object_id: None,
                    delegated_user_tenant_id: None,
                },
            )
            .unwrap();
        let params = url_params(&url);
        assert_eq!(params["sr"], "f", "file SAS URL must include sr=f");
        assert_eq!(params.len(), 13, "file SAS must have 13 parameters");
        assert!(url.path().ends_with(FILE_PATH));
    }

    #[test]
    fn test_file_string_to_sign_with_ip_and_rsc() {
        let key = make_key();
        let resource = FileResource {
            share: SHARE.to_string(),
            path: FILE_PATH.to_string(),
            options: Some(FileResourceOptions {
                cache_control: Some("no-store".to_string()),
                content_language: Some("en-US".to_string()),
                content_type: Some("text/plain".to_string()),
                ..Default::default()
            }),
        };
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "r",
            start: "2024-01-01T00:00:00Z",
            expiry: "2024-01-08T00:00:00Z",
            canon: &canon,
            key: &key,
            version: resource.default_api_version(),
            ip: Some("10.1.2.3"),
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(parts[12], "10.1.2.3", "[12] signedIP");
        assert_eq!(parts[15], "no-store", "[15] rscc");
        assert_eq!(parts[16], "", "[16] rscd — empty");
        assert_eq!(parts[18], "en-US", "[18] rscl");
        assert_eq!(parts[19], "text/plain", "[19] rsct");
    }
}
