// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Table Storage resource for user delegation SAS.
//!
//! Table SAS uses `tn` instead of `sr` and optionally restricts access to
//! a partition/row key range.
//!
//! <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas>

use crate::builder::{SasTokenBuilder, SasUrlBuilder};
use crate::common::sealed::Sealed;
use crate::common::{CommonFields, SasResource, ValidatedKey};
use crate::SAS_VERSION;

/// Minimum API version for Table Storage user delegation SAS.
pub const TABLE_MIN_VERSION: &str = "2025-07-05";

pub(crate) struct TableResource {
    table: String,
    start_partition_key: Option<String>,
    start_row_key: Option<String>,
    end_partition_key: Option<String>,
    end_row_key: Option<String>,
}

impl TableResource {
    pub(crate) fn new(table: impl Into<String>) -> Self {
        Self {
            table: table.into().to_lowercase(),
            start_partition_key: None,
            start_row_key: None,
            end_partition_key: None,
            end_row_key: None,
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/table/{}/{}", account, self.table)
    }

    pub(crate) fn url_path_segments(&self) -> [&str; 1] {
        [&self.table]
    }
}

/// Permissions for a Table Storage SAS.
///
/// Serialization order: `raud`.
#[derive(Clone, Copy, Default)]
pub(crate) struct TablePermissions {
    pub(crate) read: bool,
    pub(crate) add: bool,
    pub(crate) update: bool,
    pub(crate) delete: bool,
}

impl TablePermissions {
    pub(crate) fn to_sas_str(self) -> String {
        let mut s = String::with_capacity(4);
        if self.read {
            s.push('r');
        }
        if self.add {
            s.push('a');
        }
        if self.update {
            s.push('u');
        }
        if self.delete {
            s.push('d');
        }
        s
    }
}

/// State after selecting a table resource.
pub struct TableState {
    pub(crate) resource: TableResource,
    pub(crate) permissions: TablePermissions,
}

impl Sealed for TableState {}

impl SasTokenBuilder<'_, TableState> {
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
    /// Enables update permission.
    pub fn update(mut self) -> Self {
        self.state.permissions.update = true;
        self
    }
    /// Enables delete permission.
    pub fn delete(mut self) -> Self {
        self.state.permissions.delete = true;
        self
    }
    /// Restricts access to rows with partition/row keys >= the given values (`spk`/`srk`).
    pub fn start_key(
        mut self,
        partition_key: impl Into<String>,
        row_key: impl Into<String>,
    ) -> Self {
        self.state.resource.start_partition_key = Some(partition_key.into());
        self.state.resource.start_row_key = Some(row_key.into());
        self
    }
    /// Restricts access to rows with partition/row keys <= the given values (`epk`/`erk`).
    pub fn end_key(mut self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self {
        self.state.resource.end_partition_key = Some(partition_key.into());
        self.state.resource.end_row_key = Some(row_key.into());
        self
    }
}

impl SasUrlBuilder<'_, TableState> {
    /// Enables read permission.
    pub fn read(self) -> Self {
        self.map(|b| b.read())
    }
    /// Enables add permission.
    pub fn add(self) -> Self {
        self.map(|b| b.add())
    }
    /// Enables update permission.
    pub fn update(self) -> Self {
        self.map(|b| b.update())
    }
    /// Enables delete permission.
    pub fn delete(self) -> Self {
        self.map(|b| b.delete())
    }
    /// Restricts access to rows with partition/row keys >= the given values (`spk`/`srk`).
    pub fn start_key(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self {
        self.map(|b| b.start_key(partition_key, row_key))
    }
    /// Restricts access to rows with partition/row keys <= the given values (`epk`/`erk`).
    pub fn end_key(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self {
        self.map(|b| b.end_key(partition_key, row_key))
    }
}

impl SasResource for TableState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        table_udk_string_to_sign(&sp, common, key, &canonical, &self.resource)
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        table_udk_query_parameters(&sp, common, key, signature, &self.resource)
    }

    fn url_path_segments(&self) -> Vec<&str> {
        self.resource.url_path_segments().to_vec()
    }

    fn default_endpoint(account: &str) -> url::Url {
        crate::url::table_endpoint(account)
    }
}

fn table_udk_string_to_sign(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    canonicalized_resource: &str,
    resource: &TableResource,
) -> String {
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = common.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = common.ip_str();
    let spr = common.protocol_str();
    let st = common.start_str();
    let se = common.expiry_str();
    let skt = CommonFields::format_time(key.signed_start);
    let ske = CommonFields::format_time(key.signed_expiry);
    let spk = resource.start_partition_key.as_deref().unwrap_or("");
    let srk = resource.start_row_key.as_deref().unwrap_or("");
    let epk = resource.end_partition_key.as_deref().unwrap_or("");
    let erk = resource.end_row_key.as_deref().unwrap_or("");

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
        spk,                    // [15] startingPartitionKey
        srk,                    // [16] startingRowKey
        epk,                    // [17] endingPartitionKey
        erk,                    // [18] endingRowKey
    ];
    parts.join("\n")
}

fn table_udk_query_parameters(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    signature: &str,
    resource: &TableResource,
) -> String {
    let mut parts = Vec::with_capacity(20);
    parts.push(format!("sv={SAS_VERSION}"));
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
    parts.push(format!("tn={}", CommonFields::encode(&resource.table)));
    if let Some(ref v) = resource.start_partition_key {
        parts.push(format!("spk={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = resource.start_row_key {
        parts.push(format!("srk={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = resource.end_partition_key {
        parts.push(format!("epk={}", CommonFields::encode(v)));
    }
    if let Some(ref v) = resource.end_row_key {
        parts.push(format!("erk={}", CommonFields::encode(v)));
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
    fn table_string_to_sign_has_19_fields() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let resource = TableResource::new("mytable");
        let sts = table_udk_string_to_sign("raud", &common, &key, "/table/acct/mytable", &resource);
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 19, "table STS must have exactly 19 fields");
        assert_eq!(lines[0], "raud");
        assert_eq!(lines[3], "/table/acct/mytable");
        assert_eq!(lines[14], "2026-04-06");
        assert_eq!(lines[15], "");
        assert_eq!(lines[16], "");
        assert_eq!(lines[17], "");
        assert_eq!(lines[18], "");
    }

    #[test]
    fn table_string_to_sign_includes_key_range() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let mut resource = TableResource::new("mytable");
        resource.start_partition_key = Some("partA".into());
        resource.start_row_key = Some("row001".into());
        resource.end_partition_key = Some("partZ".into());
        resource.end_row_key = Some("row999".into());
        let sts = table_udk_string_to_sign("r", &common, &key, "/table/acct/mytable", &resource);
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[15], "partA");
        assert_eq!(lines[16], "row001");
        assert_eq!(lines[17], "partZ");
        assert_eq!(lines[18], "row999");
    }
}
