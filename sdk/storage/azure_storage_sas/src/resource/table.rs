//! Azure Table Storage SAS resources.
//!
//! Provides [`TableResource`] for scoping a SAS token to a single table.
//! Table SAS URLs use `tn` instead of `sr` and optionally restrict access to a
//! partition/row key range via `start_key` and `end_key`.
//!
//! Permissions are expressed with [`TableSasPermissions`] and encoded in spec order (`raud`).
//!
//! Requires API version [`TABLE_MIN_VERSION`] or later.
//!
//! <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-table-name-table-storage-only>

use std::fmt;

use url::Url;

use crate::{
    error::SasError,
    resource::{sealed, Resource},
    sas::{append_common_sas_params, append_path, SasSigningContext, SasUrlParams},
};

/// Minimum API version for Table Storage user delegation SAS.
pub const TABLE_MIN_VERSION: &str = "2025-07-05";

/// Permissions for a Table Storage SAS token.
///
/// Permissions are emitted in spec order: `raud`
///
/// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-permissions>
#[derive(Default)]
pub struct TableSasPermissions {
    /// Read entities in a table.
    pub read: bool,
    /// Add entities to a table.
    pub add: bool,
    /// Update entities in a table.
    pub update: bool,
    /// Delete entities from a table.
    pub delete: bool,
}

impl fmt::Display for TableSasPermissions {
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
        if self.delete {
            write!(f, "d")?;
        }
        Ok(())
    }
}

/// Optional partition/row key range restriction for [`TableResource`].
///
/// Set `options: Some(TableResourceOptions { start_key: ..., end_key: ..., ..Default::default() })`
/// to restrict the SAS to a specific key range. Either or both may be set independently.
#[derive(Default)]
pub struct TableResourceOptions {
    /// Starting `(partition_key, row_key)` (inclusive), emitted as `spk`/`srk`.
    pub start_key: Option<(String, String)>,
    /// Ending `(partition_key, row_key)` (inclusive), emitted as `epk`/`erk`.
    pub end_key: Option<(String, String)>,
}

pub(crate) struct TableStringToSign<'a> {
    pub ctx: &'a SasSigningContext<'a>,
    pub start_partition_key: &'a str,
    pub start_row_key: &'a str,
    pub end_partition_key: &'a str,
    pub end_row_key: &'a str,
}

impl std::fmt::Display for TableStringToSign<'_> {
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
                self.start_partition_key,                   // [15] startingPartitionKey
                self.start_row_key,                         // [16] startingRowKey
                self.end_partition_key,                     // [17] endingPartitionKey
                self.end_row_key,                           // [18] endingRowKey
            ]
            .join("\n")
        )
    }
}

/// A single table. Uses `tn` in the URL instead of `sr`. Requires API version [`TABLE_MIN_VERSION`] or later.
pub struct TableResource {
    pub table: String,
    pub options: Option<TableResourceOptions>,
}

impl Resource for TableResource {
    type Permissions = TableSasPermissions;
}

impl sealed::DelegatedUserService for TableResource {}

impl sealed::Resource for TableResource {
    fn default_endpoint(&self, account: &str) -> Url {
        Url::parse(&format!("https://{}.table.core.windows.net", account)).unwrap()
    }
    fn default_api_version(&self) -> &'static str {
        TABLE_MIN_VERSION
    }
    fn canonicalized_resource(&self, account: &str) -> String {
        format!("/table/{}/{}", account, self.table.to_lowercase())
    }
    fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String {
        let opts = self.options.as_ref();
        let (spk, srk) = opts
            .and_then(|o| o.start_key.as_ref())
            .map(|(pk, rk)| (pk.as_str(), rk.as_str()))
            .unwrap_or(("", ""));
        let (epk, erk) = opts
            .and_then(|o| o.end_key.as_ref())
            .map(|(pk, rk)| (pk.as_str(), rk.as_str()))
            .unwrap_or(("", ""));
        TableStringToSign {
            ctx,
            start_partition_key: spk,
            start_row_key: srk,
            end_partition_key: epk,
            end_row_key: erk,
        }
        .to_string()
    }
    fn sas_url(&self, account_endpoint: &Url, params: &SasUrlParams<'_>) -> Result<Url, SasError> {
        let mut url = append_path(account_endpoint, &self.table);
        let opts = self.options.as_ref();
        let mut q = url.query_pairs_mut();
        q.append_pair("sv", params.version)
            .append_pair("tn", &self.table);
        append_common_sas_params(&mut q, params);
        if let Some((pk, rk)) = opts.and_then(|o| o.start_key.as_ref()) {
            q.append_pair("spk", pk).append_pair("srk", rk);
        }
        if let Some((pk, rk)) = opts.and_then(|o| o.end_key.as_ref()) {
            q.append_pair("epk", pk).append_pair("erk", rk);
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

    const TABLE: &str = "testtable";

    #[test]
    fn test_table_string_to_sign_has_19_fields() {
        let key = make_key();
        let resource = TableResource {
            table: TABLE.to_string(),
            options: None,
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
        let parts: Vec<&str> = s2s.split('\n').collect();
        assert_eq!(parts.len(), 19, "table string-to-sign must have 19 fields");
        assert_eq!(parts[13], "https,http", "[13] signedProtocol");
        assert_eq!(
            parts[14],
            resource.default_api_version(),
            "[14] signedVersion"
        );
        assert_eq!(parts[15], "", "[15] startingPartitionKey — empty");
    }

    #[test]
    fn test_table_sas_url_uses_tn_not_sr() {
        let key = make_key();
        let resource = TableResource {
            table: TABLE.to_string(),
            options: None,
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
        let sig = key.compute_signature(&s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.table.core.windows.net", ACCOUNT)).unwrap();
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
        assert!(!params.contains_key("sr"), "table SAS must not contain sr");
        assert_eq!(params["tn"], TABLE, "table SAS must include tn");
        assert_eq!(
            params.len(),
            13,
            "table SAS without key ranges must have 13 parameters"
        );
    }

    #[test]
    fn test_table_sas_url_with_key_ranges() {
        let key = make_key();
        let resource = TableResource {
            table: TABLE.to_string(),
            options: Some(TableResourceOptions {
                start_key: Some(("partA".to_string(), "row1".to_string())),
                end_key: Some(("partZ".to_string(), "row9".to_string())),
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
        let sig = key.compute_signature(&s2s).unwrap();
        let endpoint = Url::parse(&format!("https://{}.table.core.windows.net", ACCOUNT)).unwrap();
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
        assert_eq!(params["spk"], "partA");
        assert_eq!(params["srk"], "row1");
        assert_eq!(params["epk"], "partZ");
        assert_eq!(params["erk"], "row9");
        assert_eq!(
            params.len(),
            17,
            "table SAS with both key ranges must have 17 parameters"
        );
    }
}
