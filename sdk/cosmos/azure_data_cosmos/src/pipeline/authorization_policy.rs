// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines Cosmos DB's unique Authentication Policy.
//!
//! The Cosmos DB data plane doesn't use a standard `Authorization: Bearer` header for authentication.
//! Instead, it uses a custom header format, as defined in the [official documentation](https://learn.microsoft.com/rest/api/cosmos-db/access-control-on-cosmosdb-resources).
//! We implement that policy here, because we can't use any standard Azure SDK authentication policy.

#[cfg_attr(not(feature = "key_auth"), allow(unused_imports))]
use azure_core::{
    credentials::{Secret, TokenCredential},
    fmt::SafeDebug,
    http::{
        headers::{HeaderValue, AUTHORIZATION, MS_DATE, VERSION},
        policies::{Policy, PolicyResult},
        request::Request,
        Context,
    },
    time::{self, OffsetDateTime},
};
use std::sync::Arc;
use tracing::{debug, trace};

use crate::{pipeline::signature_target::SignatureTarget, resource_context::ResourceLink};

use crate::utils::url_encode;

const AZURE_VERSION: &str = "2020-07-15";
const COSMOS_AAD_SCOPE: &str = "https://cosmos.azure.com/.default";

#[derive(SafeDebug, Clone)]
#[safe(false)]
enum Credential {
    /// The credential is an Entra ID token.
    Token(Arc<dyn TokenCredential>),

    /// The credential is a key to be used to sign the HTTP request (a shared key)
    #[cfg(feature = "key_auth")]
    PrimaryKey(Secret),
}

#[derive(SafeDebug, Clone)]
#[safe(true)]
pub struct AuthorizationPolicy {
    credential: Credential,
}

impl AuthorizationPolicy {
    pub(crate) fn from_token_credential(token: Arc<dyn TokenCredential>) -> Self {
        Self {
            credential: Credential::Token(token),
        }
    }

    #[cfg(feature = "key_auth")]
    pub(crate) fn from_shared_key(key: Secret) -> Self {
        Self {
            credential: Credential::PrimaryKey(key),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        trace!("called AuthorizationPolicy::send. self == {:#?}", self);

        assert!(
            !next.is_empty(),
            "Authorization policies cannot be the last policy of a pipeline"
        );

        // x-ms-date and the string used in the signature must be exactly the same, so just generate it here once.
        let date_string = time::to_rfc7231(&OffsetDateTime::now_utc()).to_lowercase();

        let resource_link: &ResourceLink = ctx
            .value()
            .expect("ResourceContext should have been provided by CosmosPipeline");
        debug!(?resource_link, "generating authorization for resource");

        let auth = generate_authorization(
            &self.credential,
            SignatureTarget::new(request.method(), resource_link, &date_string),
        )
        .await?;

        request.insert_header(MS_DATE, HeaderValue::from(date_string));
        request.insert_header(VERSION, HeaderValue::from_static(AZURE_VERSION));
        request.insert_header(AUTHORIZATION, HeaderValue::from(auth));

        // next[0] will not panic, because we checked at the beginning of the function
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// Generates the 'Authorization' header value based on the provided values.
///
/// The specific result format depends on the type of the auth token provided.
///   - "primary": one of the two service-level tokens
///   - "aad": Azure Active Directory token
///
/// In the "primary" case the signature must be constructed by signing the HTTP method,
/// resource type, resource link (the relative URI) and the current time.
///
/// In the "aad" case, the signature is the AAD token.
///
/// NOTE: Resource tokens are not yet supported.
async fn generate_authorization(
    auth_token: &Credential,
    // Unused unless feature="key_auth", but I don't want to mess with excluding it since it makes call sites more complicated
    #[allow(unused_variables)] signature_target: SignatureTarget<'_>,
) -> azure_core::Result<String> {
    let token = match auth_token {
        Credential::Token(token_credential) => {
            let token = token_credential
                .get_token(&[COSMOS_AAD_SCOPE], None)
                .await?
                .token
                .secret()
                .to_string();
            format!("type=aad&ver=1.0&sig={token}")
        }

        #[cfg(feature = "key_auth")]
        Credential::PrimaryKey(key) => signature_target.into_authorization(key)?,
    };

    Ok(url_encode(token))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use azure_core::{
        credentials::{AccessToken, TokenCredential, TokenRequestOptions},
        http::Method,
        time::{Duration, OffsetDateTime},
    };

    use crate::{
        pipeline::{
            authorization_policy::{generate_authorization, Credential, COSMOS_AAD_SCOPE},
            signature_target::SignatureTarget,
        },
        resource_context::{ResourceLink, ResourceType},
        utils::url_encode,
    };

    #[derive(Debug)]
    struct TestTokenCredential(String);

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for TestTokenCredential {
        async fn get_token(
            &self,
            scopes: &[&str],
            _: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            let token = format!("{}+{}", self.0, scopes.join(","));
            Ok(AccessToken::new(
                token,
                OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
            ))
        }
    }

    #[tokio::test]
    async fn generate_authorization_for_token_credential() {
        let time_nonce =
            azure_core::time::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let date_string = azure_core::time::to_rfc7231(&time_nonce).to_lowercase();
        let cred = Arc::new(TestTokenCredential("test_token".to_string()));
        let auth_token = Credential::Token(cred);

        let ret = generate_authorization(
            &auth_token,
            SignatureTarget::new(
                Method::Get,
                &ResourceLink::root(ResourceType::Databases).item("ToDoList"),
                &date_string,
            ),
        )
        .await
        .unwrap();

        let expected: String =
            url_encode(format!("type=aad&ver=1.0&sig=test_token+{}", COSMOS_AAD_SCOPE).as_bytes());
        assert_eq!(ret, expected);
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_primary_key_0() {
        let time_nonce =
            azure_core::time::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let date_string = azure_core::time::to_rfc7231(&time_nonce).to_lowercase();

        let auth_token = Credential::PrimaryKey(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".into(),
        );

        let ret = generate_authorization(
            &auth_token,
            SignatureTarget::new(
                Method::Get,
                &ResourceLink::root(ResourceType::Databases)
                    .item("MyDatabase")
                    .feed(ResourceType::Containers)
                    .item("MyCollection"),
                &date_string,
            ),
        )
        .await
        .unwrap();

        let expected: String =
            url_encode(b"type=master&ver=1.0&sig=vrHmd02almbIg1e4htVWH+Eg/OhEHip3VTwFivZLH0A=");

        assert_eq!(ret, expected);
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_primary_key_1() {
        let time_nonce =
            azure_core::time::parse_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();
        let date_string = azure_core::time::to_rfc7231(&time_nonce).to_lowercase();

        let auth_token = Credential::PrimaryKey(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".into(),
        );

        let ret = generate_authorization(
            &auth_token,
            SignatureTarget::new(
                Method::Get,
                &ResourceLink::root(ResourceType::Databases).item("ToDoList"),
                &date_string,
            ),
        )
        .await
        .unwrap();

        let expected: String =
            url_encode(b"type=master&ver=1.0&sig=KvBM8vONofkv3yKm/8zD9MEGlbu6jjHDJBp4E9c2ZZI=");

        assert_eq!(ret, expected);
    }

    /// Tests that AAD authentication explicitly uses the constant scope value.
    #[tokio::test]
    async fn aad_token_uses_constant_scope() {
        use std::sync::Mutex;

        // Mock credential that captures the exact scopes passed to get_token
        #[derive(Debug)]
        struct ScopeCapturingCredential {
            captured_scopes: Arc<Mutex<Vec<Vec<String>>>>,
        }

        #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
        impl TokenCredential for ScopeCapturingCredential {
            async fn get_token(
                &self,
                scopes: &[&str],
                _: Option<TokenRequestOptions<'_>>,
            ) -> azure_core::Result<AccessToken> {
                self.captured_scopes
                    .lock()
                    .unwrap()
                    .push(scopes.iter().map(|s| s.to_string()).collect());

                Ok(AccessToken::new(
                    "mock_token".to_string(),
                    OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
                ))
            }
        }

        let captured_scopes = Arc::new(Mutex::new(Vec::new()));
        let cred = Arc::new(ScopeCapturingCredential {
            captured_scopes: captured_scopes.clone(),
        });
        let auth_token = Credential::Token(cred);

        let time_nonce =
            azure_core::time::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let date_string = azure_core::time::to_rfc7231(&time_nonce).to_lowercase();

        let _result = generate_authorization(
            &auth_token,
            SignatureTarget::new(
                Method::Get,
                &ResourceLink::root(ResourceType::Databases).item("TestDB"),
                &date_string,
            ),
        )
        .await
        .unwrap();

        // Verifies that get_token was called exactly once with the constant scope
        let scopes = captured_scopes.lock().unwrap();
        assert_eq!(scopes.len(), 1, "get_token should be called exactly once");
        assert_eq!(
            scopes[0].len(),
            1,
            "get_token should be called with exactly one scope"
        );
        assert_eq!(
            scopes[0][0], COSMOS_AAD_SCOPE,
            "get_token should be called with COSMOS_AAD_SCOPE constant"
        );
    }
}
