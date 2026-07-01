// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queue resource for user delegation SAS.
//!
//! # Example
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, SasProtocol, UserDelegationKey};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(8))?
//!     .queue("work-items")
//!     .read()
//!     .process()
//!     .protocol(SasProtocol::Https)
//!     .build();
//! # Ok(())
//! # }
//! ```

use crate::builder::SasBuilder;
use crate::common::sealed::Sealed;
use crate::common::{CommonFields, SasResource, ValidatedKey};
use crate::SAS_VERSION;

/// A queue resource for user delegation SAS.
#[derive(Debug)]
pub(crate) struct QueueResource {
    queue: String,
}

impl QueueResource {
    /// Creates a new queue resource.
    pub(crate) fn new(queue: impl Into<String>) -> Self {
        Self {
            queue: queue.into(),
        }
    }

    pub(crate) fn canonicalized_resource(&self, account: &str) -> String {
        format!("/queue/{}/{}", account, self.queue)
    }
}

/// Permissions for a queue SAS.
///
/// Serialization order: `raup`. Flags are set through the permission setters on
/// [`SasBuilder<QueueState>`](crate::SasBuilder).
#[derive(Clone, Copy, Default)]
pub(crate) struct QueuePermissions {
    pub(crate) read: bool,
    pub(crate) add: bool,
    pub(crate) update: bool,
    pub(crate) process: bool,
}

impl QueuePermissions {
    /// Serializes the enabled permissions to the SAS token format.
    pub(crate) fn to_sas_str(&self) -> String {
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
        if self.process {
            s.push('p');
        }
        s
    }
}

/// State after selecting a queue resource.
pub struct QueueState {
    pub(crate) resource: QueueResource,
    pub(crate) permissions: QueuePermissions,
}

impl Sealed for QueueState {}

/// Permission setters for a queue SAS, gated on [`QueueState`].
impl SasBuilder<'_, QueueState> {
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

    /// Enables process permission.
    pub fn process(mut self) -> Self {
        self.state.permissions.process = true;
        self
    }
}

impl SasResource for QueueState {
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String {
        let sp = self.permissions.to_sas_str();
        let canonical = self.resource.canonicalized_resource(&common.account);
        queue_udk_string_to_sign(&sp, common, key, &canonical)
    }

    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String {
        let sp = self.permissions.to_sas_str();
        queue_udk_query_parameters(&sp, common, key, signature)
    }
}

/// Builds the queue-service user delegation SAS string-to-sign.
///
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
fn queue_udk_string_to_sign(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    canonicalized_resource: &str,
) -> String {
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = common.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = common.ip_str();
    let spr = common.protocol_str();
    let st = common.start_str();
    let se = common.expiry_str();
    let skt = CommonFields::format_time(key.signed_start);
    let ske = CommonFields::format_time(key.signed_expiry);

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
        skdutid,                // [10] signedDelegatedUserTenantId
        sduoid,                 // [11] signedDelegatedUserObjectId
        &sip,                   // [12] signedIP
        &spr,                   // [13] signedProtocol
        SAS_VERSION,            // [14] signedVersion
    ];
    parts.join("\n")
}

/// Builds the queue-service user delegation SAS query parameters.
fn queue_udk_query_parameters(
    permissions: &str,
    common: &CommonFields,
    key: &ValidatedKey<'_>,
    signature: &str,
) -> String {
    let mut parts = Vec::with_capacity(15);
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
    parts.push(format!("sig={}", CommonFields::encode(signature)));
    parts.join("&")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::{test_common, test_udk};
    use time::macros::datetime;

    #[test]
    fn queue_string_to_sign_has_15_fields_in_order() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("q-tenant".into());
        let key = ValidatedKey::from_key(&udk).unwrap();
        let mut common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        common.delegated_user_object_id = Some("duoid".into());

        let sts = queue_udk_string_to_sign("ra", &common, &key, "/queue/acct/q");
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 15, "queue STS must have exactly 15 fields");
        assert_eq!(lines[0], "ra"); // sp
        assert_eq!(lines[3], "/queue/acct/q"); // cr
        assert_eq!(lines[4], "oid-value"); // skoid
        assert_eq!(lines[9], "2025-11-05"); // skv
        assert_eq!(lines[10], "q-tenant"); // skdutid (from key)
        assert_eq!(lines[11], "duoid"); // sduoid (from builder)
        assert_eq!(lines[14], "2026-04-06"); // sv
    }

    #[test]
    fn queue_string_to_sign_skdutid_empty_when_key_omits() {
        let udk = test_udk(); // signed_delegated_user_tid: None
        let key = ValidatedKey::from_key(&udk).unwrap();
        let common = test_common(datetime!(2025-06-01 12:00:00 UTC));
        let sts = queue_udk_string_to_sign("r", &common, &key, "/queue/acct/q");
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[10], ""); // skdutid empty
    }
}
