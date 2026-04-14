// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB authorization helpers.
//!
//! Step 1 transport pipeline signs requests directly via `sign_request`, so this
//! module provides only reusable auth primitives (no policy-chain implementation).

use crate::models::{Credential, ResourceType};
use azure_core::http::Method;
use tracing::trace;

use crate::models::ResourcePaths;

/// Cosmos DB AAD scope for token authentication.
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";

/// The resource link used when signing a Cosmos DB request.
///
/// `Paths` owns a [`ResourcePaths`] so the signing link is derived as a
/// zero-copy sub-slice of the pre-computed path buffer (the hot path).
/// `Owned` holds an independently allocated `String` for call sites that
/// construct an `AuthorizationContext` outside of the normal request pipeline.
pub(crate) enum ResourceLink {
    /// Signing link is derived from the pre-computed [`ResourcePaths`] buffer.
    Paths(ResourcePaths),
    /// Signing link is an independently owned string.
    Owned(String),
}

impl ResourceLink {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Paths(p) => p.signing_link(),
            Self::Owned(s) => s.as_str(),
        }
    }
}

impl std::fmt::Debug for ResourceLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Authorization context needed to build a Cosmos DB signature.
#[derive(Debug)]
pub(crate) struct AuthorizationContext {
    /// The HTTP method of the request.
    pub(crate) method: Method,
    /// The resource type being accessed.
    pub(crate) resource_type: ResourceType,
    /// The resource link for signing (path without leading slash, unencoded).
    pub(crate) resource_link: ResourceLink,
}

impl AuthorizationContext {
    /// Creates a new authorization context with an owned resource link string.
    ///
    /// Use [`AuthorizationContext::from_paths`] on the hot path to avoid copying
    /// the signing link out of the pre-computed [`ResourcePaths`].
    pub(crate) fn new(
        method: Method,
        resource_type: ResourceType,
        resource_link: impl Into<String>,
    ) -> Self {
        Self {
            method,
            resource_type,
            resource_link: ResourceLink::Owned(resource_link.into()),
        }
    }

    /// Creates a new authorization context that derives the signing link directly
    /// from `paths`, avoiding any additional string allocation.
    pub(crate) fn from_paths(
        method: Method,
        resource_type: ResourceType,
        paths: ResourcePaths,
    ) -> Self {
        Self {
            method,
            resource_type,
            resource_link: ResourceLink::Paths(paths),
        }
    }
}

/// Generates the Cosmos DB authorization header value.
pub(crate) async fn generate_authorization(
    credential: &Credential,
    auth_ctx: &AuthorizationContext,
    date_string: &str,
) -> azure_core::Result<String> {
    let token = match credential {
        Credential::TokenCredential(cred) => {
            let token = cred
                .get_token(&[COSMOS_AAD_SCOPE], None)
                .await?
                .token
                .secret()
                .to_string();
            let mut s = String::with_capacity(20 + token.len());
            s.push_str("type=aad&ver=1.0&sig=");
            s.push_str(&token);
            s
        }
        Credential::MasterKey(key) => {
            let string_to_sign = build_string_to_sign(auth_ctx, date_string);
            trace!(signature_payload = ?string_to_sign, "generating Cosmos auth signature");
            let signature = azure_core::hmac::hmac_sha256(&string_to_sign, key)?;
            // HMAC-SHA256 base64 is always 44 bytes; fixed prefix is 24 bytes.
            let mut s = String::with_capacity(24 + signature.len());
            s.push_str("type=master&ver=1.0&sig=");
            s.push_str(&signature);
            s
        }
    };

    Ok(url_encode(&token))
}

/// Builds the string to sign for master-key authentication.
fn build_string_to_sign(auth_ctx: &AuthorizationContext, date_string: &str) -> String {
    let method_str = match auth_ctx.method {
        Method::Get => "get",
        Method::Put => "put",
        Method::Post => "post",
        Method::Delete => "delete",
        Method::Head => "head",
        Method::Patch => "patch",
        _ => "extension",
    };

    let resource_type = auth_ctx.resource_type.path_segment();
    let resource_link = auth_ctx.resource_link.as_str();

    // method (≤9) + resource_type (≤12) + resource_link + date_string (29) + 6 separator bytes
    let capacity =
        method_str.len() + resource_type.len() + resource_link.len() + date_string.len() + 6;
    let mut s = String::with_capacity(capacity);
    use std::fmt::Write as _;
    let _ = write!(
        s,
        "{method_str}\n{resource_type}\n{resource_link}\n{date_string}\n\n"
    );
    s
}

/// URL-encodes a string using form URL encoding.
fn url_encode(s: &str) -> String {
    // Pre-allocate with the input length; most auth token chars are ASCII-safe.
    let mut out = String::with_capacity(s.len());
    out.extend(url::form_urlencoded::byte_serialize(s.as_bytes()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_string_to_sign_format() {
        let auth_ctx = AuthorizationContext::new(
            Method::Get,
            ResourceType::DocumentCollection,
            "dbs/MyDatabase/colls/MyCollection",
        );

        let date_string = "mon, 01 jan 1900 01:00:00 gmt";
        let result = build_string_to_sign(&auth_ctx, date_string);

        let expected =
            "get\ncolls\ndbs/MyDatabase/colls/MyCollection\nmon, 01 jan 1900 01:00:00 gmt\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn build_string_to_sign_for_feed() {
        let auth_ctx = AuthorizationContext::new(Method::Get, ResourceType::Database, "");

        let date_string = "mon, 01 jan 1900 01:00:00 gmt";
        let result = build_string_to_sign(&auth_ctx, date_string);

        let expected = "get\ndbs\n\nmon, 01 jan 1900 01:00:00 gmt\n\n";
        assert_eq!(result, expected);
    }
}
