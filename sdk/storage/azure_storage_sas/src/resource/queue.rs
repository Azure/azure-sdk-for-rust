// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Queue resource for user delegation SAS.
//!
//! # Example
//!
//! ```rust no_run
//! use azure_storage_sas::{SasBuilder, SasProtocol, UserDelegationKey, resource::{Queue, QueuePermissions}};
//! use time::OffsetDateTime;
//!
//! # fn example(udk: UserDelegationKey) -> azure_core::Result<()> {
//! let token = SasBuilder::new("myaccount", &udk,
//!         OffsetDateTime::now_utc() + time::Duration::hours(8))?
//!     .protocol(SasProtocol::Https)
//!     .queue(Queue::new("work-items"), QueuePermissions::new().read().process())
//!     .build();
//! # Ok(())
//! # }
//! ```

use std::fmt;

use crate::builder::{Fields, ValidatedKey};
use crate::SAS_VERSION;

/// A queue resource for user delegation SAS.
pub struct Queue {
    queue: String,
}

impl Queue {
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
}

impl fmt::Display for QueuePermissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.read {
            f.write_str("r")?;
        }
        if self.add {
            f.write_str("a")?;
        }
        if self.update {
            f.write_str("u")?;
        }
        if self.process {
            f.write_str("p")?;
        }
        Ok(())
    }
}

/// Builds the queue-service user delegation SAS string-to-sign.
///
/// See <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>.
pub(crate) fn queue_udk_string_to_sign(
    permissions: &impl std::fmt::Display,
    fields: &Fields,
    key: &ValidatedKey<'_>,
    canonicalized_resource: &str,
) -> String {
    format!(
        "{sp}\n\
         {st}\n\
         {se}\n\
         {cr}\n\
         {skoid}\n\
         {sktid}\n\
         {skt}\n\
         {ske}\n\
         {sks}\n\
         {skv}\n\
         {skdutid}\n\
         {sduoid}\n\
         {sip}\n\
         {spr}\n\
         {sv}",
        sp = permissions,
        st = fields.start_str(),
        se = fields.expiry_str(),
        cr = canonicalized_resource,
        skoid = key.signed_oid,
        sktid = key.signed_tid,
        skt = Fields::format_time(key.signed_start),
        ske = Fields::format_time(key.signed_expiry),
        sks = key.signed_service,
        skv = key.signed_version,
        skdutid = fields.delegated_tenant_id.as_deref().unwrap_or(""),
        sduoid = fields.delegated_user_object_id.as_deref().unwrap_or(""),
        sip = fields.ip_str(),
        spr = fields.protocol_str(),
        sv = SAS_VERSION,
    )
}

/// Builds the queue-service user delegation SAS query parameters.
pub(crate) fn queue_udk_query_parameters(
    permissions: &impl std::fmt::Display,
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
    if let Some(ref v) = fields.delegated_tenant_id {
        parts.push(format!("skdutid={}", Fields::encode(v)));
    }
    if let Some(ref v) = fields.delegated_user_object_id {
        parts.push(format!("sduoid={}", Fields::encode(v)));
    }
    parts.push(format!("sig={}", Fields::encode(signature)));
    parts.join("&")
}
