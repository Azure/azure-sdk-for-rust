// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SAS signing primitives shared by every resource type.
//!
//! This module owns the pieces that are common to blob and queue signing:
//! the [`CommonFields`] shared across builder states, the validated
//! [`ValidatedKey`] view of a user delegation key, the HMAC [`sign`] helper,
//! and the [`SasResource`] trait that lets each typed state own its own
//! service-specific signing logic.

use crate::ip_range::SasIpRange;
use crate::protocol::SasProtocol;
use azure_core::error::{Error, ErrorKind};
use azure_storage_common::models::UserDelegationKey;
use base64::Engine;
use hmac::{Hmac, Mac};
use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};
use sha2::Sha256;
use time::OffsetDateTime;

/// Percent-encoding set for SAS query parameter values.
///
/// Encodes everything except the RFC 3986 unreserved characters (`A-Z a-z 0-9 - _ . ~`).
const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

pub(crate) mod sealed {
    /// Seals [`SasResource`](super::SasResource) and
    /// [`BlobServiceState`](crate::blob::BlobServiceState) so they
    /// can only be implemented for the typestate markers defined in this crate.
    pub trait Sealed {}
}

/// Fields shared across every builder state, regardless of service.
pub(crate) struct CommonFields {
    pub account: String,
    pub start: Option<OffsetDateTime>,
    pub expiry: OffsetDateTime,
    pub protocol: Option<SasProtocol>,
    pub ip_range: Option<SasIpRange>,
    /// Delegated user object ID (`sduoid`). Emitted by both blob and queue SAS.
    pub delegated_user_object_id: Option<String>,
}

impl CommonFields {
    /// Formats an `OffsetDateTime` as an ISO 8601 UTC string for SAS.
    pub fn format_time(t: &OffsetDateTime) -> String {
        let t = t.to_offset(time::UtcOffset::UTC);
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            t.year(),
            u8::from(t.month()),
            t.day(),
            t.hour(),
            t.minute(),
            t.second(),
        )
    }

    /// Percent-encodes a string for use in a SAS query parameter value.
    pub fn encode(value: &str) -> String {
        percent_encode(value.as_bytes(), ENCODE_SET).to_string()
    }

    pub fn start_str(&self) -> String {
        self.start
            .as_ref()
            .map(Self::format_time)
            .unwrap_or_default()
    }

    pub fn expiry_str(&self) -> String {
        Self::format_time(&self.expiry)
    }

    pub fn ip_str(&self) -> String {
        self.ip_range
            .as_ref()
            .map(|ip| ip.sip_value())
            .unwrap_or_default()
    }

    pub fn protocol_str(&self) -> String {
        self.protocol
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default()
    }
}

/// Internal validated view of a [`UserDelegationKey`] with all required fields
/// guaranteed to be present.
pub(crate) struct ValidatedKey<'a> {
    pub signed_oid: &'a str,
    pub signed_tid: &'a str,
    pub signed_start: &'a OffsetDateTime,
    pub signed_expiry: &'a OffsetDateTime,
    pub signed_service: &'a str,
    pub signed_version: &'a str,
    /// The delegated user tenant ID (`skdutid`). This is a property of the
    /// user delegation key, set by the service only when the key was
    /// requested with a delegated user tenant ID; otherwise `None`.
    pub signed_delegated_user_tid: Option<&'a str>,
    pub value: &'a [u8],
}

impl<'a> ValidatedKey<'a> {
    pub(crate) fn from_key(key: &'a UserDelegationKey) -> azure_core::Result<Self> {
        #[inline]
        fn missing(field: &'static str) -> Error {
            Error::with_message_fn(ErrorKind::DataConversion, move || {
                format!("user delegation key is missing required field: {field}")
            })
        }
        Ok(Self {
            signed_oid: key
                .signed_oid
                .as_deref()
                .ok_or_else(|| missing("signed_oid"))?,
            signed_tid: key
                .signed_tid
                .as_deref()
                .ok_or_else(|| missing("signed_tid"))?,
            signed_start: key
                .signed_start
                .as_ref()
                .ok_or_else(|| missing("signed_start"))?,
            signed_expiry: key
                .signed_expiry
                .as_ref()
                .ok_or_else(|| missing("signed_expiry"))?,
            signed_service: key
                .signed_service
                .as_deref()
                .ok_or_else(|| missing("signed_service"))?,
            signed_version: key
                .signed_version
                .as_deref()
                .ok_or_else(|| missing("signed_version"))?,
            signed_delegated_user_tid: key.signed_delegated_user_tid.as_deref(),
            value: key.value.as_deref().ok_or_else(|| missing("value"))?,
        })
    }
}

/// Computes the SAS string-to-sign and query parameters for a resource state.
///
/// Each typed state owns its service-specific signing logic, so adding a new
/// field to one service does not affect the others.
pub(crate) trait SasResource: sealed::Sealed {
    /// Builds the string-to-sign for this resource.
    fn string_to_sign(&self, common: &CommonFields, key: &ValidatedKey<'_>) -> String;

    /// Builds the signed query string for this resource.
    fn query_parameters(
        &self,
        common: &CommonFields,
        key: &ValidatedKey<'_>,
        signature: &str,
    ) -> String;
}

/// Computes an HMAC-SHA256 signature and returns it as a base64 string.
pub(crate) fn sign(key: &[u8], message: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC-SHA256 accepts any key length");
    mac.update(message.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::CommonFields;
    use azure_storage_common::models::UserDelegationKey;
    use time::macros::datetime;
    use time::OffsetDateTime;

    pub(crate) fn test_udk() -> UserDelegationKey {
        UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid-value".into()),
            signed_tid: Some("tid-value".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(vec![116, 101, 115, 116, 107, 101, 121]), // "testkey"
        }
    }

    /// Builds a `CommonFields` with only the required values set, for testing
    /// the internal string-to-sign helpers directly.
    pub(crate) fn test_common(expiry: OffsetDateTime) -> CommonFields {
        CommonFields {
            account: "acct".into(),
            start: None,
            expiry,
            protocol: None,
            ip_range: None,
            delegated_user_object_id: None,
        }
    }
}
