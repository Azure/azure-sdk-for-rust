//! Azure Queue Storage SAS resources.
//!
//! Provides [`QueueResource`] for scoping a SAS token to a single queue.
//! Queue SAS URLs omit the `sr` parameter entirely.
//!
//! Permissions are expressed with [`QueueSasPermissions`] and encoded in spec order (`raup`).
//!
//! Requires API version [`QUEUE_MIN_VERSION`] or later.

use std::fmt;

use url::Url;

use crate::{
    error::SasError,
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// Minimum API version for Queue Storage user delegation SAS.
pub const QUEUE_MIN_VERSION: &str = "2025-07-05";

/// Permissions for a Queue Storage SAS token.
///
/// Permissions are emitted in spec order: `raup`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
#[derive(Default)]
pub struct QueueSasPermissions {
    /// Read messages and peek at messages in a queue.
    pub read: bool,
    /// Add messages to a queue.
    pub add: bool,
    /// Update a message's visibility timeout or content.
    pub update: bool,
    /// Retrieve and delete messages from a queue (dequeue).
    pub process: bool,
}

impl fmt::Display for QueueSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.read {
            write!(f, "r")?;
        }
        if self.add {
            write!(f, "a")?;
        }
        if self.update {
            write!(f, "u")?;
        }
        if self.process {
            write!(f, "p")?;
        }
        Ok(())
    }
}

pub(crate) struct QueueStringToSign<'a> {
    pub ctx: &'a SasSigningContext<'a>,
}

impl std::fmt::Display for QueueStringToSign<'_> {
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
            ]
            .join("\n")
        )
    }
}

/// A single queue. The SAS URL omits `sr`. Requires API version [`QUEUE_MIN_VERSION`] or later.
pub struct QueueResource {
    pub queue: String,
}

impl Resource for QueueResource {
    type Permissions = QueueSasPermissions;
}

impl sealed::DelegatedUserService for QueueResource {}

impl sealed::Resource for QueueResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.queue.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        QUEUE_MIN_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/queue/{}/{}", account, self.queue)
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        QueueStringToSign { ctx }.to_string()
    }
    fn sas_url(&self, account_endpoint: &Url, params: &SasUrlParams<'_>) -> Result<Url, SasError> {
        let mut url = append_path(account_endpoint, &self.queue);
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version);
        append_common_sas_params(&mut q, params);
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

    const QUEUE: &str = "testqueue";

    #[test]
    fn test_queue_string_to_sign_has_15_fields() {
        let key = make_key();
        let resource = QueueResource {
            queue: QUEUE.to_string(),
        };
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "ap",
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
        assert_eq!(parts.len(), 15, "queue string-to-sign must have 15 fields");
        assert_eq!(parts[13], "https,http", "[13] signedProtocol");
        assert_eq!(
            parts[14],
            resource.default_api_version(),
            "[14] signedVersion"
        );
    }

    #[test]
    fn test_queue_sas_url_has_no_sr() {
        let key = make_key();
        let resource = QueueResource {
            queue: QUEUE.to_string(),
        };
        let canon = resource.canonicalized_resource(ACCOUNT);
        let ctx = SasSigningContext {
            permissions: "ap",
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
        let endpoint = Url::parse(&format!("https://{}.queue.core.windows.net", ACCOUNT)).unwrap();
        let url = resource
            .sas_url(
                &endpoint,
                &SasUrlParams {
                    permissions: "ap",
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
        assert!(!params.contains_key("sr"), "queue SAS must not have sr");
        assert_eq!(params.len(), 12, "queue SAS must have 12 parameters");
        assert_eq!(url.path(), format!("/{}", QUEUE));
    }
}
