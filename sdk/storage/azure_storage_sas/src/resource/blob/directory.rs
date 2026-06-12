use std::fmt;

use url::Url;

use crate::{
    blob::{BlobResourceOptions, BlobStringToSign, BLOB_DEFAULT_VERSION},
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// Permissions for a Blob Storage Directory SAS token.
///
/// Permissions are emitted in spec order: `racwdxyltmeopi`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#permissions-for-a-directory-container-or-blob>
#[derive(Default)]
pub struct DirectorySasPermissions {
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
}

impl fmt::Display for DirectorySasPermissions {
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
        Ok(())
    }
}

/// A directory in a hierarchical-namespace account (`sr=d`).
///
/// Requires Azure Data Lake Storage Gen2 (HNS enabled). The signed directory depth
/// (`sdd`) is derived automatically from the number of segments in `path`.
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
pub struct DirectoryResource {
    pub container: String,
    /// Directory path within the container (e.g. `"logs/2024/01"`).
    pub path: String,
    pub options: Option<BlobResourceOptions>,
}

impl Resource for DirectoryResource {
    type Permissions = DirectorySasPermissions;
}

impl sealed::BlobService for DirectoryResource {}

impl sealed::Resource for DirectoryResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.blob.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        BLOB_DEFAULT_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/blob/{}/{}/{}", account, self.container, self.path)
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        let opts = self.options.as_ref();
        let correlation_id = opts.and_then(|o| o.correlation_id).map(|id| id.to_string());
        BlobStringToSign {
            ctx,
            sr: "d",
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
        let depth = self.path.split('/').count().to_string();
        let mut url = append_path(
            account_endpoint,
            &format!("{}/{}", self.container, self.path),
        );
        let opts = self.options.as_ref();
        let srh = opts.map_or(&[][..], |o| &o.signed_request_headers[..]);
        let srq = opts.map_or(&[][..], |o| &o.signed_request_query_parameters[..]);
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version)
            .append_pair("sr", "d")
            .append_pair("sdd", &depth);
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

    fn make_resource(path: &str) -> DirectoryResource {
        DirectoryResource {
            container: CONTAINER.to_string(),
            path: path.to_string(),
            options: None,
        }
    }

    #[test]
    fn test_directory_sas_url_has_sdd() {
        let key = make_key();
        let resource = make_resource("dir1/dir2");
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "rl",
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
        let sig = crate::key::compute_hmac_signature(&key.value, &s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.blob.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "rl",
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
        assert_eq!(params["sr"], "d", "directory SAS must include sr=d");
        assert_eq!(
            params["sdd"], "2",
            "sdd must equal the number of path segments"
        );
        assert_eq!(
            params.len(),
            14,
            "directory SAS must have 14 parameters (sr + sdd)"
        );
    }
}
