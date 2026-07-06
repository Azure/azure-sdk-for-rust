// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::blob::{
    BlobPermissions, BlobResource, BlobSasOptions, BlobState, ContainerPermissions,
    ContainerResource, ContainerState, DirectoryResource, DirectoryState,
};
use crate::common::{sign, CommonFields, SasResource, ValidatedKey};
use crate::ip_range::SasIpRange;
use crate::protocol::SasProtocol;
use crate::queue::{QueuePermissions, QueueResource, QueueState};
use azure_storage_common::models::UserDelegationKey;
use time::OffsetDateTime;
/// Initial state before a resource type has been selected.
pub struct Untyped;

/// A builder for constructing Shared Access Signature (SAS) tokens.
///
/// The type parameter `S` tracks the builder state, gating which methods
/// are available at compile time. Call a resource method (e.g.,
/// [`.blob()`](SasBuilder::blob)) to transition from the initial untyped
/// state to a typed state, then call `.build()` to produce the signed SAS
/// token.
pub struct SasBuilder<'a, S = Untyped> {
    pub(crate) key: ValidatedKey<'a>,
    pub(crate) common: CommonFields,
    pub(crate) state: S,
}

impl<'a> SasBuilder<'a, Untyped> {
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
            common: CommonFields {
                account: account.into(),
                start: None,
                expiry,
                protocol: None,
                ip_range: None,
                delegated_user_object_id: None,
            },
            state: Untyped,
        })
    }

    /// Selects a blob resource and transitions the builder to blob state.
    ///
    /// Permissions, an optional snapshot/version target, and response header
    /// overrides are then set with the fluent setters available on the
    /// resulting [`SasBuilder<BlobState>`].
    pub fn blob(
        self,
        container: impl Into<String>,
        blob: impl Into<String>,
    ) -> SasBuilder<'a, BlobState> {
        SasBuilder {
            key: self.key,
            common: self.common,
            state: BlobState {
                resource: BlobResource::new(container, blob),
                permissions: BlobPermissions::default(),
                options: BlobSasOptions::default(),
            },
        }
    }

    /// Selects a container resource and transitions the builder to container state.
    pub fn container(self, container: impl Into<String>) -> SasBuilder<'a, ContainerState> {
        SasBuilder {
            key: self.key,
            common: self.common,
            state: ContainerState {
                resource: ContainerResource::new(container),
                permissions: ContainerPermissions::default(),
                options: BlobSasOptions::default(),
            },
        }
    }

    /// Selects a directory resource and transitions the builder to directory state.
    pub fn directory(
        self,
        container: impl Into<String>,
        directory: impl Into<String>,
    ) -> SasBuilder<'a, DirectoryState> {
        SasBuilder {
            key: self.key,
            common: self.common,
            state: DirectoryState {
                resource: DirectoryResource::new(container, directory),
                permissions: ContainerPermissions::default(),
                options: BlobSasOptions::default(),
            },
        }
    }

    /// Selects a queue resource and transitions the builder to queue state.
    pub fn queue(self, queue: impl Into<String>) -> SasBuilder<'a, QueueState> {
        SasBuilder {
            key: self.key,
            common: self.common,
            state: QueueState {
                resource: QueueResource::new(queue),
                permissions: QueuePermissions::default(),
            },
        }
    }
}

// Common setters available in any state.
impl<S> SasBuilder<'_, S> {
    /// Sets the optional start time for the SAS.
    pub fn start(mut self, start: OffsetDateTime) -> Self {
        self.common.start = Some(start);
        self
    }

    /// Sets the permitted protocol (HTTPS only, or HTTPS and HTTP).
    pub fn protocol(mut self, protocol: SasProtocol) -> Self {
        self.common.protocol = Some(protocol);
        self
    }

    /// Restricts the SAS to requests from the given IP address or range.
    pub fn ip_range(mut self, ip: SasIpRange) -> Self {
        self.common.ip_range = Some(ip);
        self
    }

    /// Sets the delegated user object ID (sduoid).
    pub fn delegated_user_object_id(mut self, value: impl Into<String>) -> Self {
        self.common.delegated_user_object_id = Some(value.into());
        self
    }
}

// `SasResource` is a sealed crate-internal trait; the bound is intentionally
// more private than the public `token` method it gates.
#[allow(private_bounds)]
impl<S: SasResource> SasBuilder<'_, S> {
    /// Signs the SAS and returns the token.
    pub fn build(&self) -> String {
        let sts = self.state.string_to_sign(&self.common, &self.key);
        let signature = sign(self.key.value, &sts);
        self.state
            .query_parameters(&self.common, &self.key, &signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::test_udk;
    use time::macros::datetime;

    #[test]
    fn blob_string_to_sign() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let qp = SasBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .start(datetime!(2025-01-15 00:00:00 UTC))
            .protocol(SasProtocol::HttpsAndHttp)
            .blob("mycontainer", "myblob.txt")
            .read()
            .write()
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
            .blob("mycontainer", "myblob.txt")
            .read()
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
            .container("mycontainer")
            .read()
            .list()
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
            .queue("myqueue")
            .read()
            .add()
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
            .queue("q")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .snapshot("2025-01-15T12:00:00.0000000Z")
            .read()
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
            .blob("c", "b")
            .read()
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
            .delegated_user_object_id("user/oid")
            .blob("c", "b")
            .read()
            .encryption_scope("scope name")
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
            .blob("c", "b")
            .version("2025-01-15T12:00:00.0000000Z")
            .read()
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
                .blob("c", "b")
                .version(version)
                .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
            .build();

        assert!(!qp.contains("skdutid="), "got: {qp}");
    }

    #[test]
    fn format_time_produces_iso8601() {
        let t = datetime!(2025-01-15 09:30:45 UTC);
        assert_eq!(CommonFields::format_time(&t), "2025-01-15T09:30:45Z");
    }

    #[test]
    fn format_time_normalizes_non_utc_to_utc() {
        // 2025-01-15 14:30:45 +05:00 is the same instant as 09:30:45 UTC.
        let t = datetime!(2025-01-15 14:30:45 +5);
        assert_eq!(CommonFields::format_time(&t), "2025-01-15T09:30:45Z");
    }

    #[test]
    fn encode_percent_encodes_special_chars() {
        assert_eq!(CommonFields::encode("a+b/c=d"), "a%2Bb%2Fc%3Dd");
        assert_eq!(CommonFields::encode("hello"), "hello");
    }

    #[test]
    fn build_produces_deterministic_signature() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let builder = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
            .read();

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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
            .build();

        let with_headers = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
            .read()
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
            .container("c")
            .read()
            .list()
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
            .blob("c", "b")
            .read()
            .build();

        assert!(!qp.contains("srh="), "got: {qp}");
        assert!(!qp.contains("srq="), "got: {qp}");
    }

    #[test]
    fn blob_version_query_omits_snapshot_param() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
            .version("2025-01-15T12:00:00.0000000Z")
            .read()
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
            .blob("c", "b")
            .snapshot("2025-01-15T12:00:00.0000000Z")
            .read()
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
            .directory("fs", "a/b/c")
            .read()
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
            .directory("fs", "")
            .read()
            .build();
        assert!(qp.contains("sdd=0"), "got: {qp}");
    }

    #[test]
    fn ip_range_single_address_in_query() {
        use std::net::Ipv4Addr;
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .ip_range(SasIpRange::Address(Ipv4Addr::new(1, 2, 3, 4)))
            .blob("c", "b")
            .read()
            .build();
        assert!(qp.contains("sip=1.2.3.4"), "got: {qp}");
    }

    #[test]
    fn ip_range_span_in_query() {
        use std::net::Ipv4Addr;
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .ip_range(SasIpRange::InclusiveRange {
                start: Ipv4Addr::new(10, 0, 0, 1),
                end: Ipv4Addr::new(10, 0, 0, 255),
            })
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
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
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
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
            .read()
            .build();
        assert!(qp.contains("sp=racwdxytmeopi"), "got: {qp}");
    }

    #[test]
    fn queue_query_has_no_blob_only_params() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let qp = SasBuilder::new("acct", &udk, expiry)
            .unwrap()
            .queue("q")
            .read()
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
            .blob("c", "b")
            .read()
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
            .blob("c", "b")
            .read()
            .build();

        let mut udk_some = test_udk();
        udk_some.signed_delegated_user_tid = Some("tenant".into());
        let sig_some = SasBuilder::new("acct", &udk_some, expiry)
            .unwrap()
            .blob("c", "b")
            .read()
            .build();

        let a = sig_none.split("sig=").nth(1).unwrap();
        let b = sig_some.split("sig=").nth(1).unwrap();
        assert_ne!(a, b);
    }
}
