// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::blob::{
    BlobPermissions, BlobResource, BlobSasOptions, BlobState, ContainerPermissions,
    ContainerResource, ContainerState, DirectoryResource, DirectoryState,
};
use crate::common::{sign, CommonFields, SasResource, ValidatedKey};
use crate::file::{
    FilePermissions, FileResource, FileSasOptions, FileState, SharePermissions, ShareResource,
    ShareState,
};
use crate::ip_range::SasIpRange;
use crate::protocol::SasProtocol;
use crate::queue::{QueuePermissions, QueueResource, QueueState};
use crate::table::{TablePermissions, TableResource, TableState};
use azure_storage_common::models::UserDelegationKey;
use time::OffsetDateTime;

/// The signed query string portion of a SAS token (the `?`-less query parameters).
///
/// `Deref`s to `str` for direct access to string methods. Implements
/// [`Display`](std::fmt::Display) and converts to [`String`] via [`From`].
#[derive(Debug, Clone)]
pub struct SasToken(String);

impl std::ops::Deref for SasToken {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SasToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<SasToken> for String {
    fn from(t: SasToken) -> String {
        t.0
    }
}

/// A complete signed SAS URL.
///
/// `Deref`s to [`url::Url`] for direct access to URL methods. Implements
/// [`Display`](std::fmt::Display) and converts to [`url::Url`] via [`From`].
#[derive(Debug, Clone)]
pub struct SasUrl(url::Url);

impl std::ops::Deref for SasUrl {
    type Target = url::Url;
    fn deref(&self) -> &url::Url {
        &self.0
    }
}

impl std::fmt::Display for SasUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<SasUrl> for url::Url {
    fn from(u: SasUrl) -> url::Url {
        u.0
    }
}

/// Initial state before a resource type has been selected.
pub struct Untyped;

/// A builder for constructing Shared Access Signature (SAS) tokens.
///
/// The type parameter `S` tracks the builder state, gating which methods
/// are available at compile time. Call a resource method (e.g.,
/// [`.blob()`](SasTokenBuilder::blob)) to transition from the initial untyped
/// state to a typed state, then call [`.build()`](SasTokenBuilder::build) to produce the signed SAS
/// token as a [`SasToken`].
///
/// To build a complete signed URL instead, use [`SasUrlBuilder`].
pub struct SasTokenBuilder<'a, S = Untyped> {
    pub(crate) key: ValidatedKey<'a>,
    pub(crate) common: CommonFields,
    pub(crate) state: S,
}

impl<'a> SasTokenBuilder<'a, Untyped> {
    /// Creates a new SAS token builder.
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
    /// resulting [`SasTokenBuilder<BlobState>`].
    pub fn blob(
        self,
        container: impl Into<String>,
        blob: impl Into<String>,
    ) -> SasTokenBuilder<'a, BlobState> {
        SasTokenBuilder {
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
    pub fn container(self, container: impl Into<String>) -> SasTokenBuilder<'a, ContainerState> {
        SasTokenBuilder {
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
    ) -> SasTokenBuilder<'a, DirectoryState> {
        SasTokenBuilder {
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
    pub fn queue(self, queue: impl Into<String>) -> SasTokenBuilder<'a, QueueState> {
        SasTokenBuilder {
            key: self.key,
            common: self.common,
            state: QueueState {
                resource: QueueResource::new(queue),
                permissions: QueuePermissions::default(),
            },
        }
    }

    /// Selects a table resource.
    pub fn table(self, table: impl Into<String>) -> SasTokenBuilder<'a, TableState> {
        SasTokenBuilder {
            key: self.key,
            common: self.common,
            state: TableState {
                resource: TableResource::new(table),
                permissions: TablePermissions::default(),
            },
        }
    }

    /// Selects a file resource.
    pub fn file(
        self,
        share: impl Into<String>,
        path: impl Into<String>,
    ) -> SasTokenBuilder<'a, FileState> {
        SasTokenBuilder {
            key: self.key,
            common: self.common,
            state: FileState {
                resource: FileResource::new(share, path),
                permissions: FilePermissions::default(),
                options: FileSasOptions::default(),
            },
        }
    }

    /// Selects a share resource.
    pub fn share(self, share: impl Into<String>) -> SasTokenBuilder<'a, ShareState> {
        SasTokenBuilder {
            key: self.key,
            common: self.common,
            state: ShareState {
                resource: ShareResource::new(share),
                permissions: SharePermissions::default(),
                options: FileSasOptions::default(),
            },
        }
    }
}

// Common setters available in any state.
impl<S> SasTokenBuilder<'_, S> {
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
impl<S: SasResource> SasTokenBuilder<'_, S> {
    /// Signs the SAS and returns the token.
    pub fn build(&self) -> SasToken {
        let sts = self.state.string_to_sign(&self.common, &self.key);
        let sig = sign(self.key.value, &sts);
        SasToken(self.state.query_parameters(&self.common, &self.key, &sig))
    }
}

/// A builder for constructing complete signed Shared Access Signature URLs.
///
/// Functions identically to [`SasTokenBuilder`], but [`build`](SasUrlBuilder::build)
/// returns a [`SasUrl`] — a complete URL with the signed query string embedded —
/// rather than a bare query string.
///
/// Use [`endpoint`](SasUrlBuilder::endpoint) to override the default
/// `https://{account}.{service}.core.windows.net` base (e.g. for Azurite or
/// sovereign clouds).
pub struct SasUrlBuilder<'a, S = Untyped> {
    pub(crate) inner: SasTokenBuilder<'a, S>,
    pub(crate) endpoint: Option<url::Url>,
}

impl<'a> SasUrlBuilder<'a, Untyped> {
    /// Creates a new SAS URL builder.
    ///
    /// # Errors
    ///
    /// Returns an error if `key` is missing any required field.
    pub fn new(
        account: impl Into<String>,
        key: &'a UserDelegationKey,
        expiry: OffsetDateTime,
    ) -> azure_core::Result<Self> {
        Ok(Self {
            inner: SasTokenBuilder::new(account, key, expiry)?,
            endpoint: None,
        })
    }

    /// Selects a blob resource.
    pub fn blob(
        self,
        container: impl Into<String>,
        blob: impl Into<String>,
    ) -> SasUrlBuilder<'a, BlobState> {
        SasUrlBuilder {
            inner: self.inner.blob(container, blob),
            endpoint: self.endpoint,
        }
    }

    /// Selects a container resource.
    pub fn container(self, container: impl Into<String>) -> SasUrlBuilder<'a, ContainerState> {
        SasUrlBuilder {
            inner: self.inner.container(container),
            endpoint: self.endpoint,
        }
    }

    /// Selects a directory resource.
    pub fn directory(
        self,
        container: impl Into<String>,
        directory: impl Into<String>,
    ) -> SasUrlBuilder<'a, DirectoryState> {
        SasUrlBuilder {
            inner: self.inner.directory(container, directory),
            endpoint: self.endpoint,
        }
    }

    /// Selects a queue resource.
    pub fn queue(self, queue: impl Into<String>) -> SasUrlBuilder<'a, QueueState> {
        SasUrlBuilder {
            inner: self.inner.queue(queue),
            endpoint: self.endpoint,
        }
    }

    /// Selects a table resource.
    pub fn table(self, table: impl Into<String>) -> SasUrlBuilder<'a, TableState> {
        SasUrlBuilder {
            inner: self.inner.table(table),
            endpoint: self.endpoint,
        }
    }

    /// Selects a file resource.
    pub fn file(
        self,
        share: impl Into<String>,
        path: impl Into<String>,
    ) -> SasUrlBuilder<'a, FileState> {
        SasUrlBuilder {
            inner: self.inner.file(share, path),
            endpoint: self.endpoint,
        }
    }

    /// Selects a share resource.
    pub fn share(self, share: impl Into<String>) -> SasUrlBuilder<'a, ShareState> {
        SasUrlBuilder {
            inner: self.inner.share(share),
            endpoint: self.endpoint,
        }
    }
}

impl<'a, S> SasUrlBuilder<'a, S> {
    /// Applies a transformation to the inner [`SasTokenBuilder`], preserving the endpoint.
    pub(crate) fn map(
        self,
        f: impl FnOnce(SasTokenBuilder<'a, S>) -> SasTokenBuilder<'a, S>,
    ) -> Self {
        Self {
            inner: f(self.inner),
            endpoint: self.endpoint,
        }
    }

    /// Sets the optional start time for the SAS.
    pub fn start(self, start: OffsetDateTime) -> Self {
        self.map(|b| b.start(start))
    }

    /// Sets the permitted protocol (HTTPS only, or HTTPS and HTTP).
    pub fn protocol(self, protocol: SasProtocol) -> Self {
        self.map(|b| b.protocol(protocol))
    }

    /// Restricts the SAS to requests from the given IP address or range.
    pub fn ip_range(self, ip: SasIpRange) -> Self {
        self.map(|b| b.ip_range(ip))
    }

    /// Sets the delegated user object ID (`sduoid`).
    pub fn delegated_user_object_id(self, value: impl Into<String>) -> Self {
        self.map(|b| b.delegated_user_object_id(value))
    }

    /// Overrides the storage account endpoint base URL.
    ///
    /// Use for local emulators, sovereign clouds, or custom domains.
    pub fn endpoint(mut self, endpoint: url::Url) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
}

#[allow(private_bounds)]
impl<S: SasResource> SasUrlBuilder<'_, S> {
    /// Signs the SAS and returns a complete URL with the signed query string embedded.
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint URL cannot be used as a base (e.g. a
    /// `data:` URL). Standard `https://` and `http://` URLs always succeed.
    pub fn build(&self) -> azure_core::Result<SasUrl> {
        let token = self.inner.build();
        let endpoint = self
            .endpoint
            .clone()
            .map_or_else(|| S::default_endpoint(&self.inner.common.account), Ok)?;
        let url = crate::url::assemble(&endpoint, &self.inner.state.url_path_segments(), &token)?;
        Ok(SasUrl(url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_support::test_udk;
    use crate::protocol::SasProtocol;
    use time::macros::datetime;

    #[test]
    fn blob_build_produces_signed_query() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .blob("mycontainer", "myblob.txt")
            .read()
            .cache_control("no-cache")
            .build();

        assert!(token.starts_with("sv=2026-04-06&sr=b&"));
        assert!(token.contains("sp=r"));
        assert!(token.contains("skoid=oid-value"));
        assert!(token.contains("rscc=no-cache"));
        assert!(token.contains("sig="));
    }

    #[test]
    fn container_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .container("mycontainer")
            .read()
            .list()
            .build();

        assert!(token.starts_with("sv=2026-04-06&sr=c&"));
        assert!(token.contains("sp=rl"));
        assert!(token.contains("sig="));
    }

    #[test]
    fn queue_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .queue("myqueue")
            .read()
            .add()
            .build();

        assert!(token.starts_with("sv=2026-04-06&"));
        assert!(token.contains("sp=ra"));
        assert!(token.contains("skoid=oid-value"));
        assert!(token.contains("sig="));
        // Queue should not have blob-specific params
        assert!(!token.contains("sr="));
    }

    #[test]
    fn table_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .table("mytable")
            .read()
            .build();

        assert!(token.contains("tn=mytable"));
        assert!(token.contains("sp=r"));
        assert!(!token.contains("sr="));
        assert!(token.contains("sig="));
    }

    #[test]
    fn file_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .file("myshare", "path/file.txt")
            .read()
            .build();

        assert!(token.contains("sr=f"));
        assert!(token.contains("sp=r"));
        assert!(token.contains("sig="));
    }

    #[test]
    fn share_build() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .share("myshare")
            .read()
            .list()
            .build();

        assert!(token.contains("sr=s"));
        assert!(token.contains("sp=rl"));
        assert!(token.contains("sig="));
    }

    #[test]
    fn blob_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .blob("mycontainer", "myblob.txt")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        assert!(url.path().ends_with("/mycontainer/myblob.txt"));
        let qs = url.query().unwrap();
        assert!(qs.contains("sr=b"));
        assert!(qs.contains("sp=r"));
        assert!(qs.contains("sig="));
    }

    #[test]
    fn container_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .container("mycontainer")
            .read()
            .list()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        assert!(url.path().ends_with("/mycontainer"));
        assert!(url.query().unwrap().contains("sr=c"));
    }

    #[test]
    fn queue_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .queue("myqueue")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.queue.core.windows.net"));
        assert!(url.path().ends_with("/myqueue"));
    }

    #[test]
    fn table_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .table("mytable")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.table.core.windows.net"));
        assert!(url.path().ends_with("/mytable"));
        assert!(url.query().unwrap().contains("tn=mytable"));
    }

    #[test]
    fn file_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .file("myshare", "dir/file.txt")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.file.core.windows.net"));
        assert!(
            url.path().ends_with("/myshare/dir/file.txt"),
            "path: {}",
            url.path()
        );
        assert!(url.query().unwrap().contains("sr=f"));
    }

    #[test]
    fn share_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let url = SasUrlBuilder::new("myaccount", &udk, expiry)
            .unwrap()
            .share("myshare")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.file.core.windows.net"));
        assert!(url.path().ends_with("/myshare"));
        assert!(url.query().unwrap().contains("sr=s"));
    }

    #[test]
    fn custom_endpoint_used_in_build_url() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);
        let endpoint = url::Url::parse("http://127.0.0.1:10000/devstoreaccount1").unwrap();

        let url = SasUrlBuilder::new("devstoreaccount1", &udk, expiry)
            .unwrap()
            .endpoint(endpoint)
            .blob("container", "blob.txt")
            .read()
            .build()
            .unwrap();

        assert_eq!(url.host_str(), Some("127.0.0.1"));
        assert!(url
            .as_str()
            .contains("/devstoreaccount1/container/blob.txt"));
    }

    #[test]
    fn blob_snapshot_sets_sr_bs() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
            .snapshot("2025-01-15T12:00:00.0000000Z")
            .read()
            .build();

        assert!(token.contains("sr=bs"));
        // The `:` characters in the snapshot timestamp are percent-encoded
        // when emitted in the SAS query string.
        assert!(token.contains("snapshot=2025-01-15T12%3A00%3A00.0000000Z"));
    }

    #[test]
    fn blob_version_sets_sr_bv() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("acct", &udk, expiry)
            .unwrap()
            .blob("c", "b")
            .version("2025-01-15T12:00:00.0000000Z")
            .read()
            .build();

        assert!(token.contains("sr=bv"));
    }

    #[test]
    fn protocol_https_only() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("acct", &udk, expiry)
            .unwrap()
            .protocol(SasProtocol::Https)
            .blob("c", "b")
            .read()
            .build();

        assert!(token.contains("spr=https"));
        assert!(!token.contains("spr=https,"));
    }

    #[test]
    fn new_errors_when_key_missing_required_field() {
        let mut udk = test_udk();
        udk.signed_oid = None;
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let err = SasTokenBuilder::new("acct", &udk, expiry)
            .err()
            .expect("expected error for missing field");
        assert!(format!("{err}").contains("signed_oid"));
    }

    #[test]
    fn directory_build_sets_sr_d_and_depth() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("acct", &udk, expiry)
            .unwrap()
            .directory("fs", "a/b/c")
            .read()
            .build();

        assert!(token.contains("sr=d"));
        assert!(token.contains("sdd=3"));
    }

    #[test]
    fn table_with_key_range() {
        let udk = test_udk();
        let expiry = datetime!(2025-06-01 12:00:00 UTC);

        let token = SasTokenBuilder::new("acct", &udk, expiry)
            .unwrap()
            .table("t")
            .read()
            .start_key("partA", "row001")
            .end_key("partZ", "row999")
            .build();

        assert!(token.contains("spk=partA"));
        assert!(token.contains("srk=row001"));
        assert!(token.contains("epk=partZ"));
        assert!(token.contains("erk=row999"));
    }
}
