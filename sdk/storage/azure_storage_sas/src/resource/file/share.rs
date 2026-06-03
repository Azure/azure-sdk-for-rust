use std::fmt;

use url::Url;

use crate::{
    file::{FileResourceOptions, FileStringToSign, FILE_MIN_VERSION},
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// Permissions for an Azure Share SAS token.
///
/// Permissions are emitted in spec order: `rwdl`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#permissions-for-a-share>
#[derive(Default)]
pub struct ShareSasPermissions {
    /// Read file content, properties, and metadata.
    pub read: bool,
    /// Write file content, properties, and metadata.
    pub write: bool,
    /// Delete a file.
    pub delete: bool,
    /// List files and directories in a share.
    pub list: bool,
}

impl fmt::Display for ShareSasPermissions {
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
        if self.list {
            write!(f, "l")?;
        }
        Ok(())
    }
}

/// A file share (`sr=s`). Grants access to all files in the share.
///
/// Requires API version [`FILE_MIN_VERSION`] or later.
pub struct ShareResource {
    pub share: String,
    pub options: Option<FileResourceOptions>,
}

impl Resource for ShareResource {
    type Permissions = ShareSasPermissions;
}

impl sealed::DelegatedUserService for ShareResource {}

impl sealed::Resource for ShareResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.file.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        FILE_MIN_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/file/{}/{}", account, self.share)
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
    fn sas_url(
        &self,
        account_endpoint: &Url,
        params: &SasUrlParams<'_>,
    ) -> azure_core::Result<Url> {
        let mut url = append_path(account_endpoint, &self.share);
        let opts = self.options.as_ref();
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version).append_pair("sr", "s");
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
