// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::ip_range::SasIpRange;
use crate::protocol::SasProtocol;
use crate::resource::blob::{
    blob_udk_query_parameters, blob_udk_string_to_sign, Blob, BlobPermissions, Container,
    ContainerPermissions, Directory,
};
use crate::resource::{
    queue_udk_query_parameters, queue_udk_string_to_sign, Queue, QueuePermissions,
};
use azure_core::error::{Error, ErrorKind};
use azure_storage_common::models::UserDelegationKey;
use base64::Engine;
use hmac::{Hmac, Mac};
use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};
use sha2::Sha256;
use std::collections::BTreeMap;
use time::OffsetDateTime;

/// Percent-encoding set for SAS query parameter values.
///
/// Encodes everything except the RFC 3986 unreserved characters (`A-Z a-z 0-9 - _ . ~`).
const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Typestate markers for [`SasBuilder`].
pub mod state {
    use crate::resource::blob::{
        Blob, BlobPermissions, Container, ContainerPermissions, Directory,
    };
    use crate::resource::{Queue, QueuePermissions};

    /// Initial state before a resource type has been selected.
    pub struct Untyped;

    /// State after selecting a blob resource.
    pub struct BlobState {
        pub(crate) resource: Blob,
        pub(crate) permissions: BlobPermissions,
    }

    /// State after selecting a container resource.
    pub struct ContainerState {
        pub(crate) resource: Container,
        pub(crate) permissions: ContainerPermissions,
    }

    /// State after selecting a directory resource.
    pub struct DirectoryState {
        pub(crate) resource: Directory,
        pub(crate) permissions: ContainerPermissions,
    }

    /// State after selecting a queue resource.
    pub struct QueueState {
        pub(crate) resource: Queue,
        pub(crate) permissions: QueuePermissions,
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::state::BlobState {}
    impl Sealed for super::state::ContainerState {}
    impl Sealed for super::state::DirectoryState {}
}

/// Marker trait for blob-service typestate markers.
///
/// Types implementing this trait support response header overrides and
/// other blob-service-specific SAS fields.
pub trait BlobServiceState: sealed::Sealed {}
impl BlobServiceState for state::BlobState {}
impl BlobServiceState for state::ContainerState {}
impl BlobServiceState for state::DirectoryState {}

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
    fn from_key(key: &'a UserDelegationKey) -> azure_core::Result<Self> {
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

/// Internal fields shared across all builder states.
pub(crate) struct Fields {
    pub account: String,
    pub start: Option<OffsetDateTime>,
    pub expiry: OffsetDateTime,
    pub protocol: Option<SasProtocol>,
    pub ip_range: Option<SasIpRange>,
    pub encryption_scope: Option<String>,
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub content_type: Option<String>,
    pub authorized_object_id: Option<String>,
    pub unauthorized_object_id: Option<String>,
    pub correlation_id: Option<String>,
    pub delegated_user_object_id: Option<String>,
    pub signed_request_headers: Option<BTreeMap<String, String>>,
    pub signed_request_query_parameters: Option<BTreeMap<String, String>>,
}

impl Fields {
    /// Formats an `OffsetDateTime` as an ISO 8601 UTC string for SAS.
    pub fn format_time(t: &OffsetDateTime) -> String {
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
            .map(|ip| ip.to_string())
            .unwrap_or_default()
    }

    pub fn protocol_str(&self) -> String {
        self.protocol
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default()
    }

    pub fn encryption_scope_str(&self) -> String {
        self.encryption_scope.clone().unwrap_or_default()
    }

    /// Canonicalizes signed request headers for the string-to-sign.
    ///
    /// Each entry is formatted as `key:value\n`. Returns an empty string
    /// when no headers are set.
    pub fn signed_request_headers_str(&self) -> String {
        match &self.signed_request_headers {
            Some(headers) if !headers.is_empty() => {
                let mut s = String::new();
                for (k, v) in headers {
                    s.push_str(k);
                    s.push(':');
                    s.push_str(v);
                    s.push('\n');
                }
                s
            }
            _ => String::new(),
        }
    }

    /// Canonicalizes signed request query parameters for the string-to-sign.
    ///
    /// Each entry is formatted as `\nkey:value`. Returns an empty string
    /// when no parameters are set.
    pub fn signed_request_query_parameters_str(&self) -> String {
        match &self.signed_request_query_parameters {
            Some(params) if !params.is_empty() => {
                let mut s = String::new();
                for (k, v) in params {
                    s.push('\n');
                    s.push_str(k);
                    s.push(':');
                    s.push_str(v);
                }
                s
            }
            _ => String::new(),
        }
    }
}

/// A builder for constructing Shared Access Signature (SAS) tokens.
///
/// The type parameter `S` tracks the builder state, gating which methods
/// are available at compile time. Call a resource method (e.g.,
/// [`.blob()`](SasBuilder::blob)) to transition from
/// [`Untyped`](state::Untyped) to a typed state, then call
/// [`.build()`](SasBuilder::build) to produce the signed query string.
pub struct SasBuilder<'a, S> {
    key: ValidatedKey<'a>,
    fields: Fields,
    state: S,
}

impl<'a> SasBuilder<'a, state::Untyped> {
    /// Creates a new SAS builder.
    ///
    /// # Parameters
    /// - `account`: The storage account name.
    /// - `key`: The user delegation key used to sign the SAS.
    /// - `expiry`: When the SAS expires.
    ///
    /// # Errors
    ///
    /// Returns an error if `key` is missing any required field
    /// (e.g., `signed_oid`, `signed_start`, `value`).
    pub fn new(
        account: impl Into<String>,
        key: &'a UserDelegationKey,
        expiry: OffsetDateTime,
    ) -> azure_core::Result<Self> {
        Ok(Self {
            key: ValidatedKey::from_key(key)?,
            fields: Fields {
                account: account.into(),
                start: None,
                expiry,
                protocol: None,
                ip_range: None,
                encryption_scope: None,
                cache_control: None,
                content_disposition: None,
                content_encoding: None,
                content_language: None,
                content_type: None,
                authorized_object_id: None,
                unauthorized_object_id: None,
                correlation_id: None,
                delegated_user_object_id: None,
                signed_request_headers: None,
                signed_request_query_parameters: None,
            },
            state: state::Untyped,
        })
    }

    /// Selects a blob resource and transitions the builder to blob state.
    pub fn blob(
        self,
        resource: Blob,
        permissions: BlobPermissions,
    ) -> SasBuilder<'a, state::BlobState> {
        SasBuilder {
            key: self.key,
            fields: self.fields,
            state: state::BlobState {
                resource,
                permissions,
            },
        }
    }

    /// Selects a container resource and transitions the builder to container state.
    pub fn container(
        self,
        resource: Container,
        permissions: ContainerPermissions,
    ) -> SasBuilder<'a, state::ContainerState> {
        SasBuilder {
            key: self.key,
            fields: self.fields,
            state: state::ContainerState {
                resource,
                permissions,
            },
        }
    }

    /// Selects a directory resource and transitions the builder to directory state.
    pub fn directory(
        self,
        resource: Directory,
        permissions: ContainerPermissions,
    ) -> SasBuilder<'a, state::DirectoryState> {
        SasBuilder {
            key: self.key,
            fields: self.fields,
            state: state::DirectoryState {
                resource,
                permissions,
            },
        }
    }

    /// Selects a queue resource and transitions the builder to queue state.
    pub fn queue(
        self,
        resource: Queue,
        permissions: QueuePermissions,
    ) -> SasBuilder<'a, state::QueueState> {
        SasBuilder {
            key: self.key,
            fields: self.fields,
            state: state::QueueState {
                resource,
                permissions,
            },
        }
    }
}

// Common setters available in any state.
impl<S> SasBuilder<'_, S> {
    /// Sets the optional start time for the SAS.
    pub fn start(mut self, start: OffsetDateTime) -> Self {
        self.fields.start = Some(start);
        self
    }

    /// Sets the permitted protocol (HTTPS only, or HTTPS and HTTP).
    pub fn protocol(mut self, protocol: SasProtocol) -> Self {
        self.fields.protocol = Some(protocol);
        self
    }

    /// Restricts the SAS to requests from the given IP address or range.
    pub fn ip_range(mut self, ip: SasIpRange) -> Self {
        self.fields.ip_range = Some(ip);
        self
    }

    /// Sets the encryption scope for the SAS.
    pub fn encryption_scope(mut self, scope: impl Into<String>) -> Self {
        self.fields.encryption_scope = Some(scope.into());
        self
    }

    /// Sets the delegated user object ID (sduoid).
    pub fn delegated_user_object_id(mut self, value: impl Into<String>) -> Self {
        self.fields.delegated_user_object_id = Some(value.into());
        self
    }
}

// Blob-service-specific setters.
impl<S: BlobServiceState> SasBuilder<'_, S> {
    /// Sets the `Cache-Control` response header override.
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.fields.cache_control = Some(value.into());
        self
    }

    /// Sets the `Content-Disposition` response header override.
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.fields.content_disposition = Some(value.into());
        self
    }

    /// Sets the `Content-Encoding` response header override.
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.fields.content_encoding = Some(value.into());
        self
    }

    /// Sets the `Content-Language` response header override.
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.fields.content_language = Some(value.into());
        self
    }

    /// Sets the `Content-Type` response header override.
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.fields.content_type = Some(value.into());
        self
    }

    /// Sets the authorized AAD object ID (saoid).
    pub fn authorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.fields.authorized_object_id = Some(value.into());
        self
    }

    /// Sets the unauthorized AAD object ID (suoid).
    pub fn unauthorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.fields.unauthorized_object_id = Some(value.into());
        self
    }

    /// Sets the correlation ID (scid).
    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.fields.correlation_id = Some(value.into());
        self
    }

    /// Adds a signed request header constraint (srh).
    ///
    /// When set, requests using the SAS must include each specified header
    /// with the given value. Multiple headers can be added by calling this
    /// method repeatedly. Headers are sorted by key in the string-to-sign.
    pub fn signed_request_header(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.fields
            .signed_request_headers
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Adds a signed request query parameter constraint (srq).
    ///
    /// When set, requests using the SAS must include each specified query
    /// parameter with the given value. Multiple parameters can be added by
    /// calling this method repeatedly. Parameters are sorted by key in the
    /// string-to-sign.
    pub fn signed_request_query_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.fields
            .signed_request_query_parameters
            .get_or_insert_with(BTreeMap::new)
            .insert(key.into(), value.into());
        self
    }
}

impl SasBuilder<'_, state::BlobState> {
    /// Builds the signed SAS query parameter string.
    pub fn build(&self) -> String {
        let canonical = self
            .state
            .resource
            .canonicalized_resource(&self.fields.account);
        let sr = self.state.resource.signed_resource();
        let snapshot = self.state.resource.snapshot_time();
        // The string-to-sign snapshot slot carries the snapshot timestamp or,
        // for a version SAS (`sr=bv`), the version ID. The `snapshot=` query
        // parameter remains snapshot-only; the version ID is not emitted there.
        let sts = blob_udk_string_to_sign(
            &self.state.permissions,
            &self.fields,
            &self.key,
            sr,
            &canonical,
            self.state.resource.snapshot_or_version_time().unwrap_or(""),
        );
        let signature = sign(self.key.value, &sts);
        blob_udk_query_parameters(
            &self.state.permissions,
            &self.fields,
            &self.key,
            sr,
            snapshot,
            None,
            &signature,
        )
    }
}

impl SasBuilder<'_, state::ContainerState> {
    /// Builds the signed SAS query parameter string.
    pub fn build(&self) -> String {
        let canonical = self
            .state
            .resource
            .canonicalized_resource(&self.fields.account);
        let sts = blob_udk_string_to_sign(
            &self.state.permissions,
            &self.fields,
            &self.key,
            "c",
            &canonical,
            "",
        );
        let signature = sign(self.key.value, &sts);
        blob_udk_query_parameters(
            &self.state.permissions,
            &self.fields,
            &self.key,
            "c",
            None,
            None,
            &signature,
        )
    }
}

impl SasBuilder<'_, state::DirectoryState> {
    /// Builds the signed SAS query parameter string.
    pub fn build(&self) -> String {
        let depth = self.state.resource.depth();
        let canonical = self
            .state
            .resource
            .canonicalized_resource(&self.fields.account);
        let sts = blob_udk_string_to_sign(
            &self.state.permissions,
            &self.fields,
            &self.key,
            "d",
            &canonical,
            "",
        );
        let signature = sign(self.key.value, &sts);
        blob_udk_query_parameters(
            &self.state.permissions,
            &self.fields,
            &self.key,
            "d",
            None,
            Some(depth),
            &signature,
        )
    }
}

impl SasBuilder<'_, state::QueueState> {
    /// Builds the signed SAS query parameter string.
    pub fn build(&self) -> String {
        let canonical = self
            .state
            .resource
            .canonicalized_resource(&self.fields.account);
        let sts =
            queue_udk_string_to_sign(&self.state.permissions, &self.fields, &self.key, &canonical);
        let signature = sign(self.key.value, &sts);
        queue_udk_query_parameters(&self.state.permissions, &self.fields, &self.key, &signature)
    }
}

/// Computes an HMAC-SHA256 signature and returns it as a base64 string.
fn sign(key: &[u8], message: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC-SHA256 accepts any key length");
    mac.update(message.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::blob::{Blob, BlobPermissions, Container, ContainerPermissions};
    use crate::resource::{Queue, QueuePermissions};
    use time::macros::datetime;

    fn test_udk() -> UserDelegationKey {
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

    /// Builds a `Fields` with only the required values set, for testing the
    /// internal string-to-sign helpers directly.
    fn test_fields(expiry: OffsetDateTime) -> Fields {
        Fields {
            account: "acct".into(),
            start: None,
            expiry,
            protocol: None,
            ip_range: None,
            encryption_scope: None,
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_type: None,
            authorized_object_id: None,
            unauthorized_object_id: None,
            correlation_id: None,
            delegated_user_object_id: None,
            signed_request_headers: None,
            signed_request_query_parameters: None,
        }
    }

    #[test]
    fn blob_string_to_sign() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .start(datetime!(2025-01-15 00:00:00 UTC))
            .protocol(SasProtocol::HttpsAndHttp)
            .blob(
                Blob::new("mycontainer", "myblob.txt"),
                BlobPermissions::new().read().write(),
            )
            .build();

        assert!(qp.contains("sp=rw"));
        assert!(qp.contains("sr=b"));
        assert!(qp.contains("skoid=oid-value"));
        assert!(qp.contains("spr=https%2Chttp") || qp.contains("spr=https,http"));
        assert!(qp.contains("sig="));
    }

    #[test]
    fn blob_build_produces_signed_query() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .blob(
                Blob::new("mycontainer", "myblob.txt"),
                BlobPermissions::new().read(),
            )
            .cache_control("no-cache")
            .build();

        assert!(qp.starts_with("sv=2026-04-06&sr=b&"));
        assert!(qp.contains("sp=r"));
        assert!(qp.contains("skoid=oid-value"));
        assert!(qp.contains("rscc=no-cache"));
        assert!(qp.contains("sig="));
    }

    #[test]
    fn container_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .container(
                Container::new("mycontainer"),
                ContainerPermissions::new().read().list(),
            )
            .build();

        assert!(qp.starts_with("sv=2026-04-06&sr=c&"));
        assert!(qp.contains("sp=rl"));
        assert!(qp.contains("sig="));
    }

    #[test]
    fn queue_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .queue(Queue::new("myqueue"), QueuePermissions::new().read().add())
            .build();

        assert!(qp.starts_with("sv=2026-04-06&"));
        assert!(qp.contains("sp=ra"));
        assert!(qp.contains("skoid=oid-value"));
        assert!(qp.contains("sig="));
        // Queue should not have blob-specific params
        assert!(!qp.contains("sr="));
        assert!(!qp.contains("rscc="));
    }

    #[test]
    fn queue_delegated_setters_are_percent_encoded() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("tenant id".into());
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .delegated_user_object_id("user/oid")
            .queue(Queue::new("q"), QueuePermissions::new().read())
            .build();

        assert!(qp.contains("skdutid=tenant%20id"), "got: {qp}");
        assert!(qp.contains("sduoid=user%2Foid"), "got: {qp}");
    }

    #[test]
    fn delegated_setters_on_blob() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("dtid".into());
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .delegated_user_object_id("duoid")
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .cache_control("no-cache")
            .content_disposition("inline")
            .content_encoding("gzip")
            .content_language("en-US")
            .content_type("text/plain")
            .authorized_object_id("saoid")
            .unauthorized_object_id("suoid")
            .correlation_id("scid")
            .build();

        assert!(qp.contains("skdutid=dtid"));
        assert!(qp.contains("sduoid=duoid"));
        assert!(qp.contains("saoid=saoid"));
        assert!(qp.contains("suoid=suoid"));
        assert!(qp.contains("scid=scid"));
        assert!(qp.contains("rscc=no-cache"));
        assert!(qp.contains("rsct=text%2Fplain") || qp.contains("rsct=text/plain"));
    }

    #[test]
    fn blob_snapshot_sets_sr_bs() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(
                Blob::new("c", "b").snapshot("2025-01-15T12:00:00.0000000Z"),
                BlobPermissions::new().read(),
            )
            .build();

        assert!(qp.contains("sr=bs"));
        // The `:` characters in the snapshot timestamp are percent-encoded
        // when emitted in the SAS query string.
        assert!(qp.contains("snapshot=2025-01-15T12%3A00%3A00.0000000Z"));
    }

    #[test]
    fn response_header_overrides_are_percent_encoded() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .content_type("image/jpeg")
            .content_disposition("attachment; filename=\"my file.txt\"")
            .build();

        // `/`, `;`, spaces, and quotes in user-supplied values must be
        // percent-encoded so the resulting URL remains parseable.
        assert!(qp.contains("rsct=image%2Fjpeg"), "got: {qp}");
        assert!(
            qp.contains("rscd=attachment%3B%20filename%3D%22my%20file.txt%22"),
            "got: {qp}"
        );
    }

    #[test]
    fn blob_identity_and_scope_fields_are_percent_encoded() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("tenant id".into());
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .encryption_scope("scope name")
            .delegated_user_object_id("user/oid")
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .authorized_object_id("saoid/value")
            .unauthorized_object_id("suoid value")
            .correlation_id("scid id")
            .build();

        assert!(qp.contains("ses=scope%20name"), "got: {qp}");
        assert!(qp.contains("skdutid=tenant%20id"), "got: {qp}");
        assert!(qp.contains("sduoid=user%2Foid"), "got: {qp}");
        assert!(qp.contains("saoid=saoid%2Fvalue"), "got: {qp}");
        assert!(qp.contains("suoid=suoid%20value"), "got: {qp}");
        assert!(qp.contains("scid=scid%20id"), "got: {qp}");
    }

    #[test]
    fn blob_version_sets_sr_bv() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(
                Blob::new("c", "b").version("2025-01-15T12:00:00.0000000Z"),
                BlobPermissions::new().read(),
            )
            .build();

        assert!(qp.contains("sr=bv"));
    }

    #[test]
    fn blob_version_id_is_signed() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let sig = |version: &str| {
            let qp = SasBuilder::new("acct", &udk, expiry)
                .unwrap()
                .blob(
                    Blob::new("c", "b").version(version),
                    BlobPermissions::new().read(),
                )
                .build();
            qp.split("sig=").nth(1).unwrap().to_string()
        };

        // The version ID is placed in the snapshot slot of the string-to-sign,
        // so two different versions must yield different signatures. Regression:
        // previously the slot was always empty for `sr=bv`, so the signature did
        // not cover the version ID and would not validate against the service.
        assert_ne!(
            sig("2025-01-15T12:00:00.0000000Z"),
            sig("2025-02-20T08:30:00.0000000Z")
        );
    }

    #[test]
    fn skdutid_comes_from_key() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("delegated-tenant".into());
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        assert!(qp.contains("skdutid=delegated-tenant"), "got: {qp}");
    }

    #[test]
    fn no_skdutid_when_key_omits_it() {
        // `test_udk()` has `signed_delegated_user_tid: None`.
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        assert!(!qp.contains("skdutid="), "got: {qp}");
    }

    #[test]
    fn format_time_produces_iso8601() {
        let t = datetime!(2025-01-15 09:30:45 UTC);
        assert_eq!(Fields::format_time(&t), "2025-01-15T09:30:45Z");
    }

    #[test]
    fn encode_percent_encodes_special_chars() {
        assert_eq!(Fields::encode("a+b/c=d"), "a%2Bb%2Fc%3Dd");
        assert_eq!(Fields::encode("hello"), "hello");
    }

    #[test]
    fn build_produces_deterministic_signature() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let builder = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read());

        let first = builder.build();
        let second = builder.build();
        assert_eq!(first, second);
    }

    #[test]
    fn common_setters_before_and_after_transition() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        // Common setters work before transition
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .protocol(SasProtocol::Https)
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .start(datetime!(2025-01-15 00:00:00 UTC))
            .build();

        assert!(qp.contains("spr=https"));
        assert!(qp.contains("st=2025-01-15T00:00:00Z"));
    }

    #[test]
    fn new_errors_when_key_missing_required_field() {
        let mut udk = test_udk();
        udk.signed_oid = None;
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let err = match SasBuilder::new("acct", &udk, expiry) {
            Ok(_) => panic!("missing signed_oid should error"),
            Err(e) => e,
        };
        assert!(format!("{err}").contains("signed_oid"));
    }

    #[test]
    fn signed_request_header_in_blob_sas() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .signed_request_header("x-ms-blob-content-type", "application/json")
            .build();

        assert!(qp.contains("srh=x-ms-blob-content-type"), "got: {qp}");
        assert!(qp.contains("sig="), "got: {qp}");
    }

    #[test]
    fn signed_request_query_parameter_in_blob_sas() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .signed_request_query_parameter("comp", "list")
            .build();

        assert!(qp.contains("srq=comp"), "got: {qp}");
        assert!(qp.contains("sig="), "got: {qp}");
    }

    #[test]
    fn multiple_signed_request_headers_sorted() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .signed_request_header("x-ms-blob-type", "BlockBlob")
            .signed_request_header("x-ms-blob-content-type", "text/plain")
            .build();

        // BTreeMap sorts keys: x-ms-blob-content-type < x-ms-blob-type.
        // Commas between keys are structural separators and must NOT be
        // percent-encoded (individual keys are encoded, commas are not).
        assert!(
            qp.contains("srh=x-ms-blob-content-type,x-ms-blob-type"),
            "got: {qp}"
        );
    }

    #[test]
    fn signed_headers_change_signature() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let without = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        let with_headers = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .signed_request_header("x-ms-blob-content-type", "application/json")
            .build();

        // Different string-to-sign must produce a different signature
        let sig_without = without.split("sig=").nth(1).unwrap();
        let sig_with = with_headers.split("sig=").nth(1).unwrap();
        assert_ne!(sig_without, sig_with);
    }

    #[test]
    fn signed_request_headers_on_container_sas() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .container(
                Container::new("c"),
                ContainerPermissions::new().read().list(),
            )
            .signed_request_header("x-ms-blob-content-type", "text/plain")
            .build();

        assert!(qp.contains("srh=x-ms-blob-content-type"), "got: {qp}");
    }

    #[test]
    fn no_srh_srq_when_not_set() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        assert!(!qp.contains("srh="), "got: {qp}");
        assert!(!qp.contains("srq="), "got: {qp}");
    }

    // ---- String-to-sign layout tests ----
    //
    // These pin the exact field positions of the string-to-sign against the
    // service spec. A reordering or a wrong source for any field (e.g., the
    // `skdutid`/version-slot bugs) would change a line index and fail here.

    #[test]
    fn blob_string_to_sign_has_28_fields_in_order() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("dut".into());
        let key = ValidatedKey::from_key(&udk).unwrap();
        let mut fields = test_fields(datetime!(2025-06-01 12:00:00 UTC));
        fields.start = Some(datetime!(2025-05-01 08:00:00 UTC));
        fields.delegated_user_object_id = Some("duoid".into());
        fields.authorized_object_id = Some("saoid".into());
        fields.unauthorized_object_id = Some("suoid".into());
        fields.correlation_id = Some("scid".into());
        fields.encryption_scope = Some("ses".into());
        fields.cache_control = Some("rscc".into());
        fields.content_disposition = Some("rscd".into());
        fields.content_encoding = Some("rsce".into());
        fields.content_language = Some("rscl".into());
        fields.content_type = Some("rsct".into());

        let sts = blob_udk_string_to_sign(
            &BlobPermissions::new().read().write(),
            &fields,
            &key,
            "b",
            "/blob/acct/c/b",
            "",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines.len(), 28, "blob STS must have exactly 28 fields");
        assert_eq!(lines[0], "rw"); // sp
        assert_eq!(lines[1], "2025-05-01T08:00:00Z"); // st
        assert_eq!(lines[2], "2025-06-01T12:00:00Z"); // se
        assert_eq!(lines[3], "/blob/acct/c/b"); // cr
        assert_eq!(lines[4], "oid-value"); // skoid
        assert_eq!(lines[5], "tid-value"); // sktid
        assert_eq!(lines[6], "2025-01-15T00:00:00Z"); // skt
        assert_eq!(lines[7], "2025-01-16T00:00:00Z"); // ske
        assert_eq!(lines[8], "b"); // sks
        assert_eq!(lines[9], "2025-11-05"); // skv (from key, not SAS_VERSION)
        assert_eq!(lines[10], "saoid"); // saoid
        assert_eq!(lines[11], "suoid"); // suoid
        assert_eq!(lines[12], "scid"); // scid
        assert_eq!(lines[13], "dut"); // skdutid (from key)
        assert_eq!(lines[14], "duoid"); // sduoid (from builder)
        assert_eq!(lines[17], "2026-04-06"); // sv (SAS_VERSION)
        assert_eq!(lines[18], "b"); // sr
        assert_eq!(lines[19], ""); // snapshot
        assert_eq!(lines[20], "ses"); // ses
        assert_eq!(lines[23], "rscc"); // rscc
        assert_eq!(lines[24], "rscd"); // rscd
        assert_eq!(lines[25], "rsce"); // rsce
        assert_eq!(lines[26], "rscl"); // rscl
        assert_eq!(lines[27], "rsct"); // rsct
    }

    #[test]
    fn blob_version_string_to_sign_places_version_in_snapshot_slot() {
        // Regression for the version-SAS bug: the version id must occupy the
        // snapshot slot (index 19) of the string-to-sign for `sr=bv`.
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let fields = test_fields(datetime!(2025-06-01 12:00:00 UTC));
        let sts = blob_udk_string_to_sign(
            &BlobPermissions::new().read(),
            &fields,
            &key,
            "bv",
            "/blob/acct/c/b",
            "2025-01-15T12:00:00.0000000Z",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[13], ""); // skdutid empty when key omits it
        assert_eq!(lines[18], "bv"); // sr
        assert_eq!(lines[19], "2025-01-15T12:00:00.0000000Z"); // version id in snapshot slot
    }

    #[test]
    fn blob_snapshot_string_to_sign_places_snapshot_in_slot() {
        let udk = test_udk();
        let key = ValidatedKey::from_key(&udk).unwrap();
        let fields = test_fields(datetime!(2025-06-01 12:00:00 UTC));
        let sts = blob_udk_string_to_sign(
            &BlobPermissions::new().read(),
            &fields,
            &key,
            "bs",
            "/blob/acct/c/b",
            "2025-02-20T08:30:00.0000000Z",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[18], "bs"); // sr
        assert_eq!(lines[19], "2025-02-20T08:30:00.0000000Z"); // snapshot in slot
    }

    #[test]
    fn queue_string_to_sign_has_15_fields_in_order() {
        let mut udk = test_udk();
        udk.signed_delegated_user_tid = Some("q-tenant".into());
        let key = ValidatedKey::from_key(&udk).unwrap();
        let mut fields = test_fields(datetime!(2025-06-01 12:00:00 UTC));
        fields.delegated_user_object_id = Some("duoid".into());

        let sts = queue_udk_string_to_sign(
            &QueuePermissions::new().read().add(),
            &fields,
            &key,
            "/queue/acct/q",
        );
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
        let fields = test_fields(datetime!(2025-06-01 12:00:00 UTC));
        let sts = queue_udk_string_to_sign(
            &QueuePermissions::new().read(),
            &fields,
            &key,
            "/queue/acct/q",
        );
        let lines: Vec<&str> = sts.split('\n').collect();
        assert_eq!(lines[10], ""); // skdutid empty
    }

    // ---- Query-parameter edge cases ----

    #[test]
    fn blob_version_query_omits_snapshot_param() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(
                Blob::new("c", "b").version("2025-01-15T12:00:00.0000000Z"),
                BlobPermissions::new().read(),
            )
            .build();
        assert!(qp.contains("sr=bv"));
        assert!(
            !qp.contains("snapshot="),
            "version SAS must not emit a snapshot= query param; got: {qp}"
        );
    }

    #[test]
    fn blob_snapshot_query_includes_snapshot_param_not_version() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(
                Blob::new("c", "b").snapshot("2025-01-15T12:00:00.0000000Z"),
                BlobPermissions::new().read(),
            )
            .build();
        assert!(qp.contains("sr=bs"));
        assert!(qp.contains("snapshot=2025-01-15T12%3A00%3A00.0000000Z"));
        assert!(!qp.contains("versionid="), "got: {qp}");
    }

    #[test]
    fn directory_build_sets_sr_d_and_sdd_depth() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .directory(
                Directory::new("fs", "a/b/c"),
                ContainerPermissions::new().read(),
            )
            .build();
        assert!(qp.contains("sr=d"), "got: {qp}");
        assert!(qp.contains("sdd=3"), "got: {qp}");
    }

    #[test]
    fn directory_root_has_zero_depth() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .directory(Directory::new("fs", ""), ContainerPermissions::new().read())
            .build();
        assert!(qp.contains("sdd=0"), "got: {qp}");
    }

    #[test]
    fn ip_range_single_address_in_query() {
        use std::net::{IpAddr, Ipv4Addr};
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .ip_range(SasIpRange::Address(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))))
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();
        assert!(qp.contains("sip=1.2.3.4"), "got: {qp}");
    }

    #[test]
    fn ip_range_span_in_query() {
        use std::net::{IpAddr, Ipv4Addr};
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .ip_range(SasIpRange::Range {
                start: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
                end: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 255)),
            })
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();
        // The `-` between addresses must not be percent-encoded.
        assert!(qp.contains("sip=10.0.0.1-10.0.0.255"), "got: {qp}");
    }

    #[test]
    fn protocol_https_only_in_query() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .protocol(SasProtocol::Https)
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();
        assert!(qp.contains("spr=https"), "got: {qp}");
        assert!(!qp.contains("spr=https,"), "got: {qp}");
    }

    #[test]
    fn no_optional_params_when_unset() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();
        for absent in [
            "st=",
            "sip=",
            "spr=",
            "saoid=",
            "suoid=",
            "scid=",
            "skdutid=",
            "sduoid=",
            "ses=",
            "snapshot=",
            "rscc=",
            "rscd=",
            "rsce=",
            "rscl=",
            "rsct=",
            "sdd=",
        ] {
            assert!(!qp.contains(absent), "unexpected `{absent}` in: {qp}");
        }
    }

    #[test]
    fn blob_permissions_serialize_in_canonical_order() {
        // Pins the documented serialization order `racwdxytmeopi`. The setters
        // are intentionally applied out of order to prove the order is fixed by
        // the type, not by call order.
        let perms = BlobPermissions::new()
            .set_immutability_policy()
            .permissions()
            .ownership()
            .execute()
            .move_blob()
            .tags()
            .permanent_delete()
            .delete_version()
            .delete()
            .write()
            .create()
            .add()
            .read();
        assert_eq!(perms.to_string(), "racwdxytmeopi");
    }

    #[test]
    fn queue_query_has_no_blob_only_params() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .queue(Queue::new("q"), QueuePermissions::new().read())
            .build();
        for absent in ["sr=", "snapshot=", "sdd=", "rscc=", "ses="] {
            assert!(!qp.contains(absent), "unexpected `{absent}` in: {qp}");
        }
    }

    #[test]
    fn start_time_appears_in_query_and_sts() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .start(datetime!(2025-05-01 08:00:00 UTC))
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();
        assert!(qp.contains("st=2025-05-01T08:00:00Z"), "got: {qp}");
    }

    #[test]
    fn delegated_key_changes_signature() {
        // Two keys differing only in `signed_delegated_user_tid` must produce
        // different signatures, proving `skdutid` is covered by the signature.
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let udk_none = test_udk();
        let sig_none = SasBuilder::new("acct", &udk_none, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        let mut udk_some = test_udk();
        udk_some.signed_delegated_user_tid = Some("tenant".into());
        let sig_some = SasBuilder::new("acct", &udk_some, expiry)
            .unwrap()
            .blob(Blob::new("c", "b"), BlobPermissions::new().read())
            .build();

        let a = sig_none.split("sig=").nth(1).unwrap();
        let b = sig_some.split("sig=").nth(1).unwrap();
        assert_ne!(a, b);
    }
}
