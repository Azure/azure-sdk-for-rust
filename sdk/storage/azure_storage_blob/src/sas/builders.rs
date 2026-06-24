// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Type-state builders for blob-service user delegation SAS **URLs**.
//!
//! Unlike the lower-level [`SasBuilder`](azure_storage_sas::SasBuilder), these
//! builders are seeded from the originating client's endpoint (container, blob
//! name, snapshot/version) and their terminal [`url()`](BlobSasBuilder::url)
//! returns a ready-to-use [`Url`], not just a token.

use azure_core::http::Url;
use azure_storage_sas::{BlobState, ContainerState, SasBuilder, SasIpRange, SasProtocol};

/// Builder for a blob user delegation SAS URL.
///
/// Created by [`BlobClient::user_delegation_sas`](crate::BlobClient::user_delegation_sas).
/// Chain optional setters, then call [`url()`](BlobSasBuilder::url) to produce the
/// signed blob URL.
pub struct BlobSasBuilder<'a> {
    endpoint: Url,
    inner: SasBuilder<'a, BlobState>,
}

impl<'a> BlobSasBuilder<'a> {
    pub(crate) fn new(endpoint: Url, inner: SasBuilder<'a, BlobState>) -> Self {
        Self { endpoint, inner }
    }

    /// Sets the optional start time for the SAS.
    pub fn start(mut self, start: time::OffsetDateTime) -> Self {
        self.inner = self.inner.start(start);
        self
    }

    /// Sets the permitted protocol (HTTPS only, or HTTPS and HTTP).
    pub fn protocol(mut self, protocol: SasProtocol) -> Self {
        self.inner = self.inner.protocol(protocol);
        self
    }

    /// Restricts the SAS to requests from the given IP address or range.
    pub fn ip_range(mut self, ip: SasIpRange) -> Self {
        self.inner = self.inner.ip_range(ip);
        self
    }

    /// Sets the encryption scope for the SAS.
    pub fn encryption_scope(mut self, scope: impl Into<String>) -> Self {
        self.inner = self.inner.encryption_scope(scope);
        self
    }

    /// Sets the delegated user object ID (sduoid).
    pub fn delegated_user_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.delegated_user_object_id(value);
        self
    }

    /// Sets the `Cache-Control` response header override.
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.cache_control(value);
        self
    }

    /// Sets the `Content-Disposition` response header override.
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_disposition(value);
        self
    }

    /// Sets the `Content-Encoding` response header override.
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_encoding(value);
        self
    }

    /// Sets the `Content-Language` response header override.
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_language(value);
        self
    }

    /// Sets the `Content-Type` response header override.
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_type(value);
        self
    }

    /// Sets the authorized AAD object ID (saoid).
    pub fn authorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.authorized_object_id(value);
        self
    }

    /// Sets the unauthorized AAD object ID (suoid).
    pub fn unauthorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.unauthorized_object_id(value);
        self
    }

    /// Sets the correlation ID (scid).
    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.correlation_id(value);
        self
    }

    /// Adds a signed request header constraint (srh).
    pub fn signed_request_header(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.signed_request_header(key, value);
        self
    }

    /// Adds a signed request query parameter constraint (srq).
    pub fn signed_request_query_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.signed_request_query_parameter(key, value);
        self
    }

    /// Signs the SAS and returns the full blob URL with the token appended.
    ///
    /// The endpoint's `snapshot=` query parameter (if any) is replaced by the
    /// signed token; the `versionid=` parameter, when present, is preserved.
    pub fn url(self) -> Url {
        let token = self.inner.token();
        super::helpers::append_query_excluding(&self.endpoint, &token, &["snapshot"])
    }
}

/// Builder for a container user delegation SAS URL.
///
/// Created by
/// [`BlobContainerClient::user_delegation_sas`](crate::BlobContainerClient::user_delegation_sas).
/// Chain optional setters, then call [`url()`](BlobContainerSasBuilder::url) to
/// produce the signed container URL.
pub struct BlobContainerSasBuilder<'a> {
    endpoint: Url,
    inner: SasBuilder<'a, ContainerState>,
}

impl<'a> BlobContainerSasBuilder<'a> {
    pub(crate) fn new(endpoint: Url, inner: SasBuilder<'a, ContainerState>) -> Self {
        Self { endpoint, inner }
    }

    /// Sets the optional start time for the SAS.
    pub fn start(mut self, start: time::OffsetDateTime) -> Self {
        self.inner = self.inner.start(start);
        self
    }

    /// Sets the permitted protocol (HTTPS only, or HTTPS and HTTP).
    pub fn protocol(mut self, protocol: SasProtocol) -> Self {
        self.inner = self.inner.protocol(protocol);
        self
    }

    /// Restricts the SAS to requests from the given IP address or range.
    pub fn ip_range(mut self, ip: SasIpRange) -> Self {
        self.inner = self.inner.ip_range(ip);
        self
    }

    /// Sets the encryption scope for the SAS.
    pub fn encryption_scope(mut self, scope: impl Into<String>) -> Self {
        self.inner = self.inner.encryption_scope(scope);
        self
    }

    /// Sets the delegated user object ID (sduoid).
    pub fn delegated_user_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.delegated_user_object_id(value);
        self
    }

    /// Sets the `Cache-Control` response header override.
    pub fn cache_control(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.cache_control(value);
        self
    }

    /// Sets the `Content-Disposition` response header override.
    pub fn content_disposition(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_disposition(value);
        self
    }

    /// Sets the `Content-Encoding` response header override.
    pub fn content_encoding(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_encoding(value);
        self
    }

    /// Sets the `Content-Language` response header override.
    pub fn content_language(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_language(value);
        self
    }

    /// Sets the `Content-Type` response header override.
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.content_type(value);
        self
    }

    /// Sets the authorized AAD object ID (saoid).
    pub fn authorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.authorized_object_id(value);
        self
    }

    /// Sets the unauthorized AAD object ID (suoid).
    pub fn unauthorized_object_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.unauthorized_object_id(value);
        self
    }

    /// Sets the correlation ID (scid).
    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.inner = self.inner.correlation_id(value);
        self
    }

    /// Adds a signed request header constraint (srh).
    pub fn signed_request_header(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.signed_request_header(key, value);
        self
    }

    /// Adds a signed request query parameter constraint (srq).
    pub fn signed_request_query_parameter(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.signed_request_query_parameter(key, value);
        self
    }

    /// Signs the SAS and returns the full container URL with the token appended.
    pub fn url(self) -> Url {
        let token = self.inner.token();
        super::helpers::append_query(&self.endpoint, &token)
    }
}
