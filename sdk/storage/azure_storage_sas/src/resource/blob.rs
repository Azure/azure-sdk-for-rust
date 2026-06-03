//! Azure Blob Storage SAS resources.
//!
//! Provides resource types for the five addressable Blob Storage scopes:
//!
//! | Type | `sr` | Scope |
//! |------|------|-------|
//! | [`BlobResource`] | `b` | A single blob |
//! | [`ContainerResource`] | `c` | All blobs in a container |
//! | [`DirectoryResource`] | `d` | A virtual directory (requires hierarchical namespace) |
//! | [`BlobSnapshotResource`] | `bs` | A specific blob snapshot |
//! | [`BlobVersionResource`] | `bv` | A specific blob version |
//!
//! Permissions are expressed with [`Resource::Permissions`](crate::resource::Resource::Permissions) and encoded in spec order (`racwdxyltmeopi`).
//!
//! <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signed-resource-field-blob-storage-only>

use std::fmt;

use time::macros::format_description;
use time::OffsetDateTime;
use uuid::Uuid;

use url::Url;

use crate::{
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

pub mod container;
pub mod directory;
pub mod snapshot;
pub mod version;
pub use container::ContainerResource;
pub use directory::DirectoryResource;
pub use snapshot::BlobSnapshotResource;
pub use version::BlobVersionResource;

/// Default API version for Blob Storage user delegation SAS tokens.
pub const BLOB_DEFAULT_VERSION: &str = "2026-04-06";

pub(crate) fn format_blob_time(dt: OffsetDateTime) -> String {
    dt.format(format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:7]Z"
    ))
    .expect("snapshot time formatting failed")
}

/// Permissions for a Blob Storage SAS token.
///
/// Permissions are emitted in spec order: `racwdxyltmeopi`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#permissions-for-a-directory-container-or-blob>
#[derive(Default)]
pub struct BlobSasPermissions {
    /// Read content, blocklist, properties, and metadata.
    pub read: bool,
    /// Add a block to an append blob.
    pub add: bool,
    /// Write a new blob, snapshot, or copy to a new blob.
    pub create: bool,
    /// Create or write content, properties, metadata, or blocklist.
    pub write: bool,
    /// Delete a blob.
    pub delete: bool,
    /// Delete a blob version.
    pub delete_version: bool,
    /// Permanently delete a blob snapshot or version.
    pub permanent_delete: bool,
    /// Read or write blob tags.
    pub tags: bool,
    /// Move a blob or directory.
    pub move_blob: bool,
    /// Get system properties; set POSIX ACL if hierarchical namespace is enabled.
    pub execute: bool,
    /// Set owner or owning group when hierarchical namespace is enabled.
    pub ownership: bool,
    /// Set permissions and POSIX ACLs when hierarchical namespace is enabled.
    pub permissions: bool,
    /// Set or delete immutability policy or legal hold on a blob.
    pub set_immutability_policy: bool,
}

impl fmt::Display for BlobSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Order per spec table: racwdxyltmeopi
        if self.read {
            write!(f, "r")?;
        }
        if self.add {
            write!(f, "a")?;
        }
        if self.create {
            write!(f, "c")?;
        }
        if self.write {
            write!(f, "w")?;
        }
        if self.delete {
            write!(f, "d")?;
        }
        if self.delete_version {
            write!(f, "x")?;
        }
        if self.permanent_delete {
            write!(f, "y")?;
        }
        if self.tags {
            write!(f, "t")?;
        }
        if self.move_blob {
            write!(f, "m")?;
        }
        if self.execute {
            write!(f, "e")?;
        }
        if self.ownership {
            write!(f, "o")?;
        }
        if self.permissions {
            write!(f, "p")?;
        }
        if self.set_immutability_policy {
            write!(f, "i")?;
        }
        Ok(())
    }
}

/// Optional fields for blob SAS resources.
///
/// All five blob resource types accept `options: Option<BlobResourceOptions>`.
///
/// ```rust,ignore
/// BlobResource {
///     container: "mycontainer".into(),
///     blob: "myblob.txt".into(),
///     options: Some(BlobResourceOptions {
///         cache_control: Some("no-cache".into()),
///         ..Default::default()
///     }),
/// }
/// ```
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
#[derive(Default)]
pub struct BlobResourceOptions {
    /// Signed correlation ID (`scid`) — ties the SAS to a specific request for auditing.
    pub correlation_id: Option<Uuid>,
    /// Encryption scope (`ses`) — restricts the SAS to a named encryption scope.
    pub encryption_scope: Option<String>,
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
    /// Required request headers (`srh`). Requires API version 2026-04-06+.
    ///
    /// Provide `(name, value)` pairs. The names are emitted as a comma-separated list in the
    /// `srh` URL parameter; the values are locked into the signature via the canonicalized
    /// `name:value\n` form. Every listed header must be present with exactly that value on
    /// any request that uses this SAS.
    pub signed_request_headers: Vec<(String, String)>,
    /// Required request query parameters (`srq`). Requires API version 2026-04-06+.
    ///
    /// Provide `(name, value)` pairs. The names are emitted as a comma-separated list in the
    /// `srq` URL parameter; the values are emitted as individual query parameters and locked
    /// into the signature via the canonicalized `\nname=value` form.
    pub signed_request_query_parameters: Vec<(String, String)>,
}

/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
pub(crate) struct BlobStringToSign<'a> {
    pub ctx: &'a SasSigningContext<'a>,
    pub sr: &'a str,
    pub snapshot_time: &'a str,
    pub correlation_id: Option<&'a str>,
    pub encryption_scope: Option<&'a str>,
    pub cache_control: Option<&'a str>,
    pub content_disposition: Option<&'a str>,
    pub content_encoding: Option<&'a str>,
    pub content_language: Option<&'a str>,
    pub content_type: Option<&'a str>,
    pub signed_request_headers: &'a [(String, String)],
    pub signed_request_query_parameters: &'a [(String, String)],
}

impl std::fmt::Display for BlobStringToSign<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctx = self.ctx;

        // canonicalizedSignedRequestHeaders: "name:value\n" per header (spec §srh).
        // canonicalizedSignedRequestQueryParameters: "\nname=value" per param (spec §srq).
        // Both collapse to "" when the respective slice is empty.
        let canonical_srh: String = self
            .signed_request_headers
            .iter()
            .map(|(n, v)| format!("{}:{}\n", n, v))
            .collect();
        let canonical_srq: String = self
            .signed_request_query_parameters
            .iter()
            .map(|(n, v)| format!("\n{}={}", n, v))
            .collect();

        // Fields [0]-[12] are common across all supported versions.
        let mut fields: Vec<&str> = vec![
            ctx.permissions,                               // [0]  signedPermissions
            ctx.start,                                     // [1]  signedStart
            ctx.expiry,                                    // [2]  signedExpiry
            ctx.canon,                                     // [3]  canonicalizedResource
            &ctx.key.signed_oid,                           // [4]  signedKeyObjectId
            &ctx.key.signed_tid,                           // [5]  signedKeyTenantId
            &ctx.key.signed_start,                         // [6]  signedKeyStart
            &ctx.key.signed_expiry,                        // [7]  signedKeyExpiry
            &ctx.key.signed_service,                       // [8]  signedKeyService
            &ctx.key.signed_version,                       // [9]  signedKeyVersion
            ctx.authorized_user_object_id.unwrap_or(""),   // [10] signedAuthorizedUserObjectId
            ctx.unauthorized_user_object_id.unwrap_or(""), // [11] signedUnauthorizedUserObjectId
            self.correlation_id.unwrap_or(""),             // [12] signedCorrelationId
        ];
        // YYYY-MM-DD strings sort lexicographically in chronological order, so >= is correct.
        // 2025-07-05+: delegated user tenant/object IDs added before IP.
        if ctx.version >= "2025-07-05" {
            fields.push(ctx.delegated_user_tenant_id.unwrap_or("")); // [13] skdutid
            fields.push(ctx.delegated_user_object_id.unwrap_or("")); // [14] sduoid
        }
        fields.extend_from_slice(&[
            ctx.ip.unwrap_or(""),                // [13/15] signedIP
            ctx.protocol.as_str(),               // [14/16] signedProtocol
            ctx.version,                         // [15/17] signedVersion
            self.sr,                             // [16/18] signedResource
            self.snapshot_time,                  // [17/19] signedSnapshotTime
            self.encryption_scope.unwrap_or(""), // [18/20] signedEncryptionScope
        ]);
        // 2026-04-06+: canonicalized srh/srq inserted BEFORE rscc per the spec ordering.
        if ctx.version >= "2026-04-06" {
            fields.push(&canonical_srh); // [21] canonicalizedSignedRequestHeaders
            fields.push(&canonical_srq); // [22] canonicalizedSignedRequestQueryParameters
        }
        fields.extend_from_slice(&[
            self.cache_control.unwrap_or(""),       // [19/21/23] rscc
            self.content_disposition.unwrap_or(""), // [20/22/24] rscd
            self.content_encoding.unwrap_or(""),    // [21/23/25] rsce
            self.content_language.unwrap_or(""),    // [22/24/26] rscl
            self.content_type.unwrap_or(""),        // [23/25/27] rsct
        ]);
        write!(f, "{}", fields.join("\n"))
    }
}

/// A single blob (`sr=b`).
pub struct BlobResource {
    pub container: String,
    pub blob: String,
    pub options: Option<BlobResourceOptions>,
}

impl Resource for BlobResource {
    type Permissions = BlobSasPermissions;
}

impl sealed::BlobService for BlobResource {}

impl sealed::Resource for BlobResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.blob.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        BLOB_DEFAULT_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}/{}", account, self.container, self.blob)
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        let opts = self.options.as_ref();
        let correlation_id = opts.and_then(|o| o.correlation_id).map(|id| id.to_string());
        BlobStringToSign {
            ctx,
            sr: "b",
            snapshot_time: "",
            correlation_id: correlation_id.as_deref(),
            encryption_scope: opts.and_then(|o| o.encryption_scope.as_deref()),
            cache_control: opts.and_then(|o| o.cache_control.as_deref()),
            content_disposition: opts.and_then(|o| o.content_disposition.as_deref()),
            content_encoding: opts.and_then(|o| o.content_encoding.as_deref()),
            content_language: opts.and_then(|o| o.content_language.as_deref()),
            content_type: opts.and_then(|o| o.content_type.as_deref()),
            signed_request_headers: opts.map_or(&[], |o| &o.signed_request_headers),
            signed_request_query_parameters: opts
                .map_or(&[], |o| &o.signed_request_query_parameters),
        }
        .to_string()
    }
    fn sas_url(
        &self,
        account_endpoint: &Url,
        params: &SasUrlParams<'_>,
    ) -> azure_core::Result<Url> {
        let mut url = append_path(
            account_endpoint,
            &format!("{}/{}", self.container, self.blob),
        );
        let opts = self.options.as_ref();
        let srh = opts.map_or(&[][..], |o| &o.signed_request_headers[..]);
        let srq = opts.map_or(&[][..], |o| &o.signed_request_query_parameters[..]);
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version).append_pair("sr", "b");
        append_common_sas_params(&mut q, params);
        if let Some(v) = opts.and_then(|o| o.encryption_scope.as_deref()) {
            q.append_pair("ses", v);
        }
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
        if !srh.is_empty() {
            let names: Vec<&str> = srh.iter().map(|(n, _)| n.as_str()).collect();
            q.append_pair("srh", &names.join(","));
        }
        if !srq.is_empty() {
            let names: Vec<&str> = srq.iter().map(|(n, _)| n.as_str()).collect();
            q.append_pair("srq", &names.join(","));
            for (name, value) in srq {
                q.append_pair(name, value);
            }
        }
        if let Some(id) = opts.and_then(|o| o.correlation_id) {
            q.append_pair("scid", &id.to_string());
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

    const CONTAINER: &str = "testcontainer";
    const BLOB: &str = "testblob.txt";

    fn make_resource() -> BlobResource {
        BlobResource {
            container: CONTAINER.to_string(),
            blob: BLOB.to_string(),
            options: None,
        }
    }

    #[test]
    fn test_blob_string_to_sign_has_28_fields_for_2026_04_06() {
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
        assert_eq!(
            parts.len(),
            28,
            "blob string-to-sign must have 28 fields for 2026-04-06"
        );
        assert_eq!(parts[13], "", "[13] skdutid — empty");
        assert_eq!(parts[14], "", "[14] sduoid — empty");
        assert_eq!(parts[15], "", "[15] signedIP — empty");
        assert_eq!(parts[16], "https,http", "[16] signedProtocol");
        assert_eq!(
            parts[17],
            resource.default_api_version(),
            "[17] signedVersion"
        );
        assert_eq!(parts[18], "b", "[18] signedResource");
        assert_eq!(parts[21], "", "[21] srh — empty");
        assert_eq!(parts[22], "", "[22] srq — empty");
        assert_eq!(parts[23], "", "[23] rscc — empty");
        assert_eq!(parts[27], "", "[27] rsct — empty");
    }

    #[test]
    fn test_blob_string_to_sign_has_26_fields_for_2025_11_05() {
        let key = make_key();
        let resource = make_resource();
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "rw",
            start: "2024-01-01T00:00:00Z",
            expiry: "2024-01-08T00:00:00Z",
            canon: &canon,
            key: &key,
            version: "2025-11-05",
            ip: None,
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(
            parts.len(),
            26,
            "blob string-to-sign must have 26 fields for 2025-11-05"
        );
        assert_eq!(parts[13], "", "[13] skdutid — empty");
        assert_eq!(parts[14], "", "[14] sduoid — empty");
        assert_eq!(parts[16], "https,http", "[16] signedProtocol");
        assert_eq!(parts[17], "2025-11-05", "[17] signedVersion");
        assert_eq!(parts[18], "b", "[18] signedResource");
    }

    #[test]
    fn test_blob_sas_url_has_13_params() {
        let key = make_key();
        let resource = make_resource();
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "cw",
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
        let endpoint = Url::parse(&format!("https://{}.blob.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "cw",
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
        assert_eq!(params.len(), 13, "blob SAS must have 13 parameters");
        assert_eq!(params["sr"], "b");
        assert_eq!(params["sv"], resource.default_api_version());
        assert_eq!(params["spr"], "https,http");
        assert!(url.path().ends_with(BLOB));
    }

    #[test]
    fn test_blob_string_to_sign_with_ip() {
        let key = make_key();
        let resource = make_resource();
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "r",
            start: "2024-01-01T00:00:00Z",
            expiry: "2024-01-08T00:00:00Z",
            canon: &canon,
            key: &key,
            version: resource.default_api_version(),
            ip: Some("192.168.1.1"),
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(parts[15], "192.168.1.1", "[15] signedIP");
    }

    #[test]
    fn test_blob_sas_url_with_optional_fields() {
        let key = make_key();
        let resource = BlobResource {
            container: CONTAINER.to_string(),
            blob: BLOB.to_string(),
            options: Some(BlobResourceOptions {
                encryption_scope: Some("myscope".to_string()),
                cache_control: Some("no-cache".to_string()),
                content_type: Some("application/octet-stream".to_string()),
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
            ip: Some("10.0.0.1"),
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(parts[15], "10.0.0.1", "[15] signedIP");
        assert_eq!(parts[20], "myscope", "[20] signedEncryptionScope");
        assert_eq!(parts[21], "", "[21] srh — empty");
        assert_eq!(parts[22], "", "[22] srq — empty");
        assert_eq!(parts[23], "no-cache", "[23] rscc");
        assert_eq!(parts[24], "", "[24] rscd — empty");
        assert_eq!(parts[27], "application/octet-stream", "[27] rsct");

        let sig = key.compute_signature(&s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.blob.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "r",
                    start: Some("2024-01-01T00:00:00Z"),
                    expiry: "2024-01-08T00:00:00Z",
                    key: &key,
                    signature: &sig,
                    version: resource.default_api_version(),
                    ip: Some("10.0.0.1"),
                    protocol: Default::default(),
                    authorized_user_object_id: None,
                    unauthorized_user_object_id: None,
                    delegated_user_object_id: None,
                    delegated_user_tenant_id: None,
                },
            )
            .unwrap();
        let params = url_params(&url);
        assert_eq!(params["ses"], "myscope");
        assert_eq!(params["rscc"], "no-cache");
        assert!(!params.contains_key("rscd"));
        assert_eq!(params["rsct"], "application/octet-stream");
        assert_eq!(params["sip"], "10.0.0.1");
    }

    #[test]
    fn test_blob_srh_srq_in_string_to_sign_and_url() {
        let key = make_key();
        let resource = BlobResource {
            container: CONTAINER.to_string(),
            blob: BLOB.to_string(),
            options: Some(BlobResourceOptions {
                signed_request_headers: vec![
                    ("x-ms-date".into(), "Wed, 20 May 2026 12:00:00 GMT".into()),
                    ("x-ms-version".into(), "2026-04-06".into()),
                ],
                signed_request_query_parameters: vec![
                    ("comp".into(), "block".into()),
                    ("restype".into(), "container".into()),
                ],
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
            ip: None,
            protocol: Default::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        };
        let s2s = resource.string_to_sign(&ctx);
        // canonical_srh embeds newlines so we can't use split; check via contains.
        assert!(
            s2s.contains("x-ms-date:Wed, 20 May 2026 12:00:00 GMT\nx-ms-version:2026-04-06\n"),
            "[21] canonicalizedSignedRequestHeaders"
        );
        assert!(
            s2s.contains("\ncomp=block\nrestype=container"),
            "[22] canonicalizedSignedRequestQueryParameters"
        );

        let sig = key.compute_signature(&s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.blob.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "r",
                    start: None,
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
        // srh/srq list names; srq values are also individual URL params.
        assert_eq!(params["srh"], "x-ms-date,x-ms-version");
        assert_eq!(params["srq"], "comp,restype");
        assert_eq!(params["comp"], "block");
        assert_eq!(params["restype"], "container");
    }
}
