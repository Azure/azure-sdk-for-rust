// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queue resource for user delegation SAS.
//!
//! # Example
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, SasProtocol, UserDelegationKey, resource::queue::{QueueResource, QueuePermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(8))?
//!     .protocol(SasProtocol::Https)
//!     .queue(QueueResource::new("work-items"), QueuePermissions::new().read().process())
//!     .build();
//! # Ok(())
//! # }
//! ```

use crate::builder::{Fields, ValidatedKey};
use crate::SAS_VERSION;

/// A queue resource for user delegation SAS.
pub struct QueueResource {
    queue: String,
}

impl QueueResource {
    /// Creates a new queue resource.
    pub fn new(queue: impl Into<String>) -> Self {
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
/// Serialization order: `raup`.
#[derive(Clone, Copy, Default)]
pub struct QueuePermissions {
    read: bool,
    add: bool,
    update: bool,
    process: bool,
}

impl QueuePermissions {
    /// Creates a new permissions set with all permissions disabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables read permission.
    pub fn read(mut self) -> Self {
        self.read = true;
        self
    }

    /// Enables add permission.
    pub fn add(mut self) -> Self {
        self.add = true;
        self
    }

    /// Enables update permission.
    pub fn update(mut self) -> Self {
        self.update = true;
        self
    }

    /// Enables process permission.
    pub fn process(mut self) -> Self {
        self.process = true;
        self
    }

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

/// Builds the queue-service user delegation SAS string-to-sign.
///
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
pub(crate) fn queue_udk_string_to_sign(
    permissions: &str,
    fields: &Fields,
    key: &ValidatedKey<'_>,
    canonicalized_resource: &str,
) -> String {
    let skdutid = key.signed_delegated_user_tid.unwrap_or("");
    let sduoid = fields.delegated_user_object_id.as_deref().unwrap_or("");
    let sip = fields.ip_str();
    let spr = fields.protocol_str();
    let st = fields.start_str();
    let se = fields.expiry_str();
    let skt = Fields::format_time(key.signed_start);
    let ske = Fields::format_time(key.signed_expiry);

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
pub(crate) fn queue_udk_query_parameters(
    permissions: &str,
    fields: &Fields,
    key: &ValidatedKey<'_>,
    signature: &str,
) -> String {
    let mut parts = Vec::with_capacity(15);
    parts.push(format!("sv={SAS_VERSION}"));
    if let Some(ref start) = fields.start {
        parts.push(format!("st={}", Fields::format_time(start)));
    }
    parts.push(format!("se={}", fields.expiry_str()));
    parts.push(format!("sp={permissions}"));
    if let Some(ref ip) = fields.ip_range {
        parts.push(format!("sip={ip}"));
    }
    if let Some(ref proto) = fields.protocol {
        parts.push(format!("spr={proto}"));
    }
    parts.push(format!("skoid={}", key.signed_oid));
    parts.push(format!("sktid={}", key.signed_tid));
    parts.push(format!("skt={}", Fields::format_time(key.signed_start)));
    parts.push(format!("ske={}", Fields::format_time(key.signed_expiry)));
    parts.push(format!("sks={}", key.signed_service));
    parts.push(format!("skv={}", key.signed_version));
    if let Some(v) = key.signed_delegated_user_tid {
        parts.push(format!("skdutid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.delegated_user_object_id {
        parts.push(format!("sduoid={}", Fields::encode(v)));
    }
    parts.push(format!("sig={}", Fields::encode(signature)));
    parts.join("&")
}
