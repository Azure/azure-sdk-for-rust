// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared Key authentication policy for Azure Storage.
//!
//! This module is only available when the `azurite` feature is enabled and is intended
//! for local development with the Azurite storage emulator only.

#[cfg(feature = "azurite")]
use azure_core::{
    credentials::Secret,
    hmac,
    http::{
        headers::{HeaderName, AUTHORIZATION},
        policies::{Policy, PolicyResult},
        Context, Request,
    },
};
#[cfg(feature = "azurite")]
use std::sync::Arc;

#[cfg(feature = "azurite")]
const STORAGE_ACCOUNT_NAME: &str = "devstoreaccount1";
#[cfg(feature = "azurite")]
// Removed temporarily just in case the CredScan picks this up
const STORAGE_ACCOUNT_KEY: &str = "<AZURITE_KEY>";

/// Policy for Shared Key authentication with Azurite well-known credentials.
///
/// This policy is only available when the `azurite` feature is enabled.
#[cfg(feature = "azurite")]
#[derive(Debug, Clone)]
pub(crate) struct SharedKeyPolicy {
    account_name: String,
    account_key: Secret,
}

#[cfg(feature = "azurite")]
impl SharedKeyPolicy {
    /// Creates a new SharedKeyPolicy using the well-known Azurite development credentials.
    pub fn new_azurite() -> azure_core::Result<Self> {
        Ok(Self {
            account_name: STORAGE_ACCOUNT_NAME.to_string(),
            account_key: Secret::new(STORAGE_ACCOUNT_KEY),
        })
    }

    /// Signs a request using Shared Key authentication.
    fn sign_request(&self, request: &mut Request) -> azure_core::Result<()> {
        let string_to_sign = self.string_to_sign(request)?;
        let signature = hmac::hmac_sha256(&string_to_sign, &self.account_key)?;

        let authorization = format!("SharedKey {}:{}", self.account_name, signature);
        request.insert_header(AUTHORIZATION, authorization);

        Ok(())
    }

    /// Constructs the string to sign for Shared Key authentication.
    fn string_to_sign(&self, request: &Request) -> azure_core::Result<String> {
        let verb = request.method().as_str();
        let headers = request.headers();

        // Get canonicalized headers
        let content_encoding =
            headers.get_optional_string(&HeaderName::from_static("content-encoding"));
        let content_language =
            headers.get_optional_string(&HeaderName::from_static("content-language"));
        let content_length =
            headers.get_optional_string(&HeaderName::from_static("content-length"));
        let content_md5 = headers.get_optional_string(&HeaderName::from_static("content-md5"));
        let content_type = headers.get_optional_string(&HeaderName::from_static("content-type"));
        let date = headers.get_optional_string(&HeaderName::from_static("date"));
        let if_modified_since =
            headers.get_optional_string(&HeaderName::from_static("if-modified-since"));
        let if_match = headers.get_optional_string(&HeaderName::from_static("if-match"));
        let if_none_match = headers.get_optional_string(&HeaderName::from_static("if-none-match"));
        let if_unmodified_since =
            headers.get_optional_string(&HeaderName::from_static("if-unmodified-since"));
        let range = headers.get_optional_string(&HeaderName::from_static("range"));

        // Get canonicalized resource
        let canonicalized_resource = self.canonicalized_resource(request);

        // Get canonicalized headers (x-ms-* headers)
        let canonicalized_headers = self.canonicalized_headers(request);

        // Construct string to sign
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
            verb,
            content_encoding.unwrap_or_default(),
            content_language.unwrap_or_default(),
            content_length.unwrap_or_default(),
            content_md5.unwrap_or_default(),
            content_type.unwrap_or_default(),
            date.unwrap_or_default(),
            if_modified_since.unwrap_or_default(),
            if_match.unwrap_or_default(),
            if_none_match.unwrap_or_default(),
            if_unmodified_since.unwrap_or_default(),
            range.unwrap_or_default(),
            canonicalized_headers,
            canonicalized_resource
        );

        Ok(string_to_sign)
    }

    /// Constructs the canonicalized resource string.
    fn canonicalized_resource(&self, request: &Request) -> String {
        let url = request.url();
        let path = url.path();

        // For Azurite, the canonicalized resource is /account-name/path
        let mut resource = format!("/{}{}", self.account_name, path);

        // Add query parameters in alphabetical order
        let mut params: Vec<_> = url.query_pairs().collect();
        params.sort_by(|a, b| a.0.cmp(&b.0));

        for (key, value) in params {
            resource.push_str(&format!("\n{}:{}", key.to_lowercase(), value));
        }

        resource
    }

    /// Constructs the canonicalized headers string (x-ms-* headers).
    fn canonicalized_headers(&self, request: &Request) -> String {
        let headers = request.headers();
        let mut ms_headers: Vec<(String, String)> = Vec::new();

        for (name, value) in headers.iter() {
            let name_str = name.as_str();
            if name_str.starts_with("x-ms-") {
                ms_headers.push((name_str.to_lowercase(), value.as_str().to_string()));
            }
        }

        // Sort by header name
        ms_headers.sort_by(|a, b| a.0.cmp(&b.0));

        // Build canonicalized headers string
        ms_headers
            .iter()
            .map(|(name, value)| format!("{}:{}\n", name, value))
            .collect::<String>()
    }
}

#[cfg(feature = "azurite")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for SharedKeyPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        self.sign_request(request)?;
        next[0].send(ctx, request, &next[1..]).await
    }
}
