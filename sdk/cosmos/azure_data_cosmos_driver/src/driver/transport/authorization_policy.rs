// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB authorization helpers.
//!
//! Step 1 transport pipeline signs requests directly via `sign_request`, so this
//! module provides only reusable auth primitives (no policy-chain implementation).

use crate::models::{Credential, ResourceType};
use azure_core::http::Method;
use tracing::trace;

/// Cosmos DB AAD scope for token authentication.
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";

/// Authorization context needed to build a Cosmos DB signature.
#[derive(Debug, Clone)]
pub(crate) struct AuthorizationContext {
    /// The HTTP method of the request.
    pub(crate) method: Method,
    /// The resource type being accessed.
    pub(crate) resource_type: ResourceType,
    /// The resource link for signing (path without leading slash, unencoded).
    pub(crate) resource_link: String,
}

impl AuthorizationContext {
    /// Creates a new authorization context.
    pub(crate) fn new(
        method: Method,
        resource_type: ResourceType,
        resource_link: impl Into<String>,
    ) -> Self {
        Self {
            method,
            resource_type,
            resource_link: resource_link.into(),
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
            format!("type=aad&ver=1.0&sig={token}")
        }
        Credential::MasterKey(key) => {
            let string_to_sign = build_string_to_sign(auth_ctx, date_string);
            trace!(signature_payload = ?string_to_sign, "generating Cosmos auth signature");
            let signature = azure_core::hmac::hmac_sha256(&string_to_sign, key)?;
            format!("type=master&ver=1.0&sig={signature}")
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

    format!(
        "{}\n{}\n{}\n{}\n\n",
        method_str,
        auth_ctx.resource_type.path_segment(),
        auth_ctx.resource_link,
        date_string,
    )
}

/// URL-encodes a string using form URL encoding.
fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
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
