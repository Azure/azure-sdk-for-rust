// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Type-state builder for queue user delegation SAS **URLs**.

use azure_core::http::Url;
use azure_storage_sas::{state::QueueState, SasBuilder, SasIpRange, SasProtocol};

/// Builder for a queue user delegation SAS URL.
///
/// Created by [`QueueClient::user_delegation_sas`](crate::QueueClient::user_delegation_sas).
/// Chain optional setters, then call [`url()`](QueueSasBuilder::url) to produce
/// the signed queue URL.
pub struct QueueSasBuilder<'a> {
    endpoint: Url,
    inner: SasBuilder<'a, QueueState>,
}

impl<'a> QueueSasBuilder<'a> {
    pub(crate) fn new(endpoint: Url, inner: SasBuilder<'a, QueueState>) -> Self {
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

    /// Signs the SAS and returns the full queue URL with the token appended.
    pub fn url(self) -> Url {
        let token = self.inner.build();
        append_sas_query(&self.endpoint, &token)
    }
}

/// Appends a SAS query string to a URL, preserving existing query parameters.
fn append_sas_query(endpoint: &Url, query: &str) -> Url {
    let mut url = endpoint.clone();
    match url.query() {
        Some(existing) if !existing.is_empty() => {
            url.set_query(Some(&format!("{existing}&{query}")));
        }
        _ => {
            url.set_query(Some(query));
        }
    }
    url
}
