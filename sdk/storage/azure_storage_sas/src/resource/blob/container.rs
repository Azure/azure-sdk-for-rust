use std::fmt;

use url::Url;

use crate::{
    blob::{BlobResourceOptions, BlobStringToSign, BLOB_DEFAULT_VERSION},
    error::SasError,
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// Permissions for a Blob Storage Container SAS token.
///
/// Permissions are emitted in spec order: `racwdxyltmeopi`
///
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#permissions-for-a-directory-container-or-blob>
#[derive(Default)]
pub struct ContainerSasPermissions {
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
    /// List blobs non-recursively.
    pub list: bool,
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

impl fmt::Display for ContainerSasPermissions {
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
        if self.list {
            write!(f, "l")?;
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

/// A blob container (`sr=c`). Grants access to all blobs in the container.
pub struct ContainerResource {
    pub container: String,
    pub options: Option<BlobResourceOptions>,
}

impl Resource for ContainerResource {
    type Permissions = ContainerSasPermissions;
}

impl sealed::BlobService for ContainerResource {}

impl sealed::Resource for ContainerResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.blob.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        BLOB_DEFAULT_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}", account, self.container)
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        let opts = self.options.as_ref();
        let correlation_id = opts.and_then(|o| o.correlation_id).map(|id| id.to_string());
        BlobStringToSign {
            ctx,
            sr: "c",
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
    fn sas_url(&self, account_endpoint: &Url, params: &SasUrlParams<'_>) -> Result<Url, SasError> {
        let mut url = append_path(account_endpoint, &self.container);
        let opts = self.options.as_ref();
        let srh = opts.map_or(&[][..], |o| &o.signed_request_headers[..]);
        let srq = opts.map_or(&[][..], |o| &o.signed_request_query_parameters[..]);
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version).append_pair("sr", "c");
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
