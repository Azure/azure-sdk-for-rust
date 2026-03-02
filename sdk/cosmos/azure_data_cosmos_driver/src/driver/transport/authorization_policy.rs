// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Authorization policy for Cosmos DB requests.
//!
//! This policy implements Cosmos DB's custom authorization header format.
//! Unlike standard Azure services that use `Authorization: Bearer`, Cosmos DB
//! uses a custom format as defined in the [official documentation](https://learn.microsoft.com/rest/api/cosmos-db/access-control-on-cosmosdb-resources).

use crate::models::{Credential, ResourceType};
use azure_core::http::{
    headers::{HeaderName, HeaderValue, AUTHORIZATION},
    policies::{Policy, PolicyResult},
    Context, Method, Request,
};
use azure_core::time::{self, OffsetDateTime};
use std::sync::Arc;
use tracing::{debug, trace};

/// x-ms-date header name.
const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");

/// Cosmos DB AAD scope for token authentication.
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";

/// Context key for providing resource information to the authorization policy.
///
/// This must be set in the request context before sending through the pipeline.
#[derive(Debug, Clone)]
pub(crate) struct AuthorizationContext {
    /// The HTTP method of the request.
    method: Method,
    /// The resource type being accessed.
    resource_type: ResourceType,
    /// The resource link for signing (path without leading slash, unencoded).
    resource_link: String,
}

impl AuthorizationContext {
    /// Creates a new authorization context.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (GET, POST, etc.)
    /// * `resource_type` - The type of resource being accessed
    /// * `resource_link` - The resource path for signing (e.g., "dbs/mydb/colls/mycoll")
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

/// Authorization policy that adds the Cosmos DB authorization header.
///
/// This policy computes the authorization signature based on the request method,
/// resource type, resource link, and current time, then adds the appropriate
/// `Authorization` and `x-ms-date` headers.
#[derive(Clone)]
pub(crate) struct AuthorizationPolicy {
    /// The credential used for signing/authenticating requests.
    credential: Credential,
}

impl std::fmt::Debug for AuthorizationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthorizationPolicy")
            .field("credential", &self.credential)
            .finish()
    }
}

impl AuthorizationPolicy {
    /// Creates a new authorization policy from authentication options.
    pub(crate) fn new(credential: &Credential) -> Self {
        Self {
            credential: credential.clone(),
        }
    }
}

#[async_trait::async_trait]
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        trace!("AuthorizationPolicy::send");

        assert!(
            !next.is_empty(),
            "Authorization policy cannot be the last policy in a pipeline"
        );

        // Get the authorization context from the request context
        let auth_ctx: &AuthorizationContext = ctx.value().expect(
            "AuthorizationContext must be provided in the request context for authorization",
        );

        // Generate timestamp - must match exactly between header and signature
        let date_string = time::to_rfc7231(&OffsetDateTime::now_utc()).to_lowercase();

        debug!(
            method = ?auth_ctx.method,
            resource_type = ?auth_ctx.resource_type,
            resource_link = %auth_ctx.resource_link,
            "generating authorization for resource"
        );

        // Generate the authorization header value
        let auth = self.generate_authorization(auth_ctx, &date_string).await?;

        // Add headers
        request.insert_header(MS_DATE, HeaderValue::from(date_string));
        request.insert_header(AUTHORIZATION, HeaderValue::from(auth));

        next[0].send(ctx, request, &next[1..]).await
    }
}

impl AuthorizationPolicy {
    /// Generates the authorization header value.
    async fn generate_authorization(
        &self,
        auth_ctx: &AuthorizationContext,
        date_string: &str,
    ) -> azure_core::Result<String> {
        let token = match &self.credential {
            Credential::TokenCredential(cred) => {
                // AAD/Entra ID authentication
                let token = cred
                    .get_token(&[COSMOS_AAD_SCOPE], None)
                    .await?
                    .token
                    .secret()
                    .to_string();
                format!("type=aad&ver=1.0&sig={token}")
            }
            Credential::MasterKey(key) => {
                // Master key authentication - compute HMAC signature
                let string_to_sign = Self::build_string_to_sign(auth_ctx, date_string);

                trace!(signature_payload = ?string_to_sign, "generating Cosmos auth signature");

                let signature = azure_core::hmac::hmac_sha256(&string_to_sign, key)?;
                format!("type=master&ver=1.0&sig={signature}")
            }
        };

        // URL-encode the token
        Ok(url_encode(&token))
    }

    /// Builds the string to sign for master key authentication.
    ///
    /// Format (from official docs):
    /// ```text
    /// StringToSign =
    ///     Verb.toLowerCase() + "\n" +
    ///     ResourceType.toLowerCase() + "\n" +
    ///     ResourceLink + "\n" +
    ///     Date.toLowerCase() + "\n" +
    ///     "" + "\n";
    /// ```
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
}

/// URL-encodes a string using form URL encoding.
fn url_encode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{
        credentials::{AccessToken, Secret, TokenCredential, TokenRequestOptions},
        http::{headers::Headers, response::AsyncRawResponse, StatusCode, Url},
        time::Duration,
    };

    /// Mock transport policy for testing.
    #[derive(Debug)]
    struct MockTransport;

    #[async_trait::async_trait]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                azure_core::Bytes::new(),
            ))
        }
    }

    /// Mock token credential for testing.
    #[derive(Debug)]
    struct MockTokenCredential(String);

    #[async_trait::async_trait]
    impl TokenCredential for MockTokenCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            Ok(AccessToken::new(
                self.0.clone(),
                OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
            ))
        }
    }

    #[tokio::test]
    async fn authorization_policy_adds_headers_for_master_key() {
        let key = Secret::new("8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==");
        let auth = crate::models::Credential::MasterKey(key);
        let policy = AuthorizationPolicy::new(&auth);

        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/dbs/mydb").unwrap();
        let mut request = Request::new(url, Method::Get);

        let auth_ctx = AuthorizationContext::new(Method::Get, ResourceType::Database, "dbs/mydb");

        let mut ctx = Context::default();
        ctx.insert(auth_ctx);

        let result = policy.send(&ctx, &mut request, &policies).await;
        assert!(result.is_ok());

        // Verify headers were set
        assert!(request.headers().get_optional_str(&MS_DATE).is_some());
        let auth_header = request.headers().get_optional_str(&AUTHORIZATION).unwrap();
        assert!(auth_header.contains("type%3Dmaster")); // URL-encoded "type=master"
    }

    #[tokio::test]
    async fn authorization_policy_adds_headers_for_token_credential() {
        let cred = Arc::new(MockTokenCredential("test_token".to_string()));
        let auth = crate::models::Credential::TokenCredential(cred);
        let policy = AuthorizationPolicy::new(&auth);

        let transport: Arc<dyn Policy> = Arc::new(MockTransport);
        let policies: Vec<Arc<dyn Policy>> = vec![transport];

        let url = Url::parse("https://test.documents.azure.com/dbs/mydb").unwrap();
        let mut request = Request::new(url, Method::Get);

        let auth_ctx = AuthorizationContext::new(Method::Get, ResourceType::Database, "dbs/mydb");

        let mut ctx = Context::default();
        ctx.insert(auth_ctx);

        let result = policy.send(&ctx, &mut request, &policies).await;
        assert!(result.is_ok());

        // Verify headers were set
        assert!(request.headers().get_optional_str(&MS_DATE).is_some());
        let auth_header = request.headers().get_optional_str(&AUTHORIZATION).unwrap();
        assert!(auth_header.contains("type%3Daad")); // URL-encoded "type=aad"
        assert!(auth_header.contains("test_token"));
    }

    #[test]
    fn build_string_to_sign_format() {
        let auth_ctx = AuthorizationContext::new(
            Method::Get,
            ResourceType::DocumentCollection,
            "dbs/MyDatabase/colls/MyCollection",
        );

        let date_string = "mon, 01 jan 1900 01:00:00 gmt";
        let result = AuthorizationPolicy::build_string_to_sign(&auth_ctx, date_string);

        let expected =
            "get\ncolls\ndbs/MyDatabase/colls/MyCollection\nmon, 01 jan 1900 01:00:00 gmt\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn build_string_to_sign_for_feed() {
        // When listing databases, the resource link is empty
        let auth_ctx = AuthorizationContext::new(Method::Get, ResourceType::Database, "");

        let date_string = "mon, 01 jan 1900 01:00:00 gmt";
        let result = AuthorizationPolicy::build_string_to_sign(&auth_ctx, date_string);

        let expected = "get\ndbs\n\nmon, 01 jan 1900 01:00:00 gmt\n\n";
        assert_eq!(result, expected);
    }
}
