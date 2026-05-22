use time::OffsetDateTime;
use url::Url;

use crate::{
    blob::{
        format_blob_time, BlobResourceOptions, BlobSasPermissions, BlobStringToSign,
        BLOB_DEFAULT_VERSION,
    },
    error::SasError,
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// A specific blob snapshot (`sr=bs`). Requires [`BLOB_DEFAULT_VERSION`] or later.
///
/// `snapshot_time` is the snapshot datetime returned by Azure. It is emitted as the `sst`
/// URL parameter to scope the SAS to this specific snapshot.
pub struct BlobSnapshotResource {
    pub container: String,
    pub blob: String,
    /// Snapshot datetime as returned by Azure Blob Storage.
    pub snapshot_time: OffsetDateTime,
    pub options: Option<BlobResourceOptions>,
}

impl Resource for BlobSnapshotResource {
    type Permissions = BlobSasPermissions;
}

impl sealed::BlobService for BlobSnapshotResource {}

impl sealed::Resource for BlobSnapshotResource {
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
        let snapshot_time_str = format_blob_time(self.snapshot_time);
        let opts = self.options.as_ref();
        let correlation_id = opts.and_then(|o| o.correlation_id).map(|id| id.to_string());
        BlobStringToSign {
            ctx,
            sr: "bs",
            snapshot_time: &snapshot_time_str,
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
        let snapshot_time_str = format_blob_time(self.snapshot_time);
        let mut url = append_path(
            account_endpoint,
            &format!("{}/{}", self.container, self.blob),
        );
        let opts = self.options.as_ref();
        let srh = opts.map_or(&[][..], |o| &o.signed_request_headers[..]);
        let srq = opts.map_or(&[][..], |o| &o.signed_request_query_parameters[..]);
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version)
            .append_pair("sr", "bs")
            .append_pair("sst", &snapshot_time_str);
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
    use time::macros::datetime;

    use super::*;
    use crate::{
        resource::sealed::Resource as _,
        sas::{SasSigningContext, SasUrlParams},
        test_utils::{make_key, url_params, ACCOUNT},
    };

    const CONTAINER: &str = "testcontainer";
    const BLOB: &str = "testblob.txt";

    fn make_resource() -> BlobSnapshotResource {
        BlobSnapshotResource {
            container: CONTAINER.to_string(),
            blob: BLOB.to_string(),
            snapshot_time: datetime!(2024-01-15 12:00:00 UTC),
            options: None,
        }
    }

    #[test]
    fn test_blob_snapshot_string_to_sign_has_snapshot_time_at_field_17() {
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
            "blob snapshot string-to-sign must have 28 fields"
        );
        assert_eq!(parts[18], "bs", "[18] signedResource");
        assert_eq!(
            parts[19], "2024-01-15T12:00:00.0000000Z",
            "[19] signedSnapshotTime"
        );
    }

    #[test]
    fn test_blob_snapshot_sas_url_has_sst() {
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
                    permissions: "r",
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
        assert_eq!(params["sr"], "bs", "snapshot SAS must include sr=bs");
        assert_eq!(
            params["sst"], "2024-01-15T12:00:00.0000000Z",
            "snapshot SAS must include sst"
        );
        assert_eq!(
            params.len(),
            14,
            "blob snapshot SAS must have 14 parameters (sr + sst)"
        );
    }
}
