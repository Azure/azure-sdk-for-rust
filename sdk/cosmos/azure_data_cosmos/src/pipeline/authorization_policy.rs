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
    date::{self, OffsetDateTime},
    headers::{HeaderValue, AUTHORIZATION, MS_DATE, VERSION},
    Context, Policy, PolicyResult, Request, Url,
};
use std::sync::Arc;
use tracing::trace;

use crate::{pipeline::signature_target::SignatureTarget, resource_context::ResourceLink};

use crate::utils::url_encode;

const AZURE_VERSION: &str = "2020-07-15";

#[derive(Debug, Clone)]
enum Credential {
    /// The credential is an Entra ID token.
    Token(Arc<dyn TokenCredential>),

    /// The credential is a key to be used to sign the HTTP request (a shared key)
    #[cfg(feature = "key_auth")]
    PrimaryKey(Secret),
}

#[derive(Debug, Clone)]
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
        let date_string = date::to_rfc1123(&OffsetDateTime::now_utc()).to_lowercase();

        let resource_link: &ResourceLink = ctx
            .value()
            .expect("ResourceContext should have been provided by CosmosPipeline");

        let auth = generate_authorization(
            &self.credential,
            request.url(),
            SignatureTarget::new(*request.method(), resource_link, &date_string),
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
async fn generate_authorization<'a>(
    auth_token: &Credential,
    url: &Url,

    // Unused unless feature="key_auth", but I don't want to mess with excluding it since it makes call sites more complicated
    #[allow(unused_variables)] signature_target: SignatureTarget<'a>,
) -> azure_core::Result<String> {
    let token = match auth_token {
        Credential::Token(token_credential) => {
            let token = token_credential
                .get_token(&[&scope_from_url(url)])
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

/// This function generates the scope string from the passed url. The scope string is used to
/// request the AAD token.
fn scope_from_url(url: &Url) -> String {
    let scheme = url.scheme();
    let hostname = url.host_str().unwrap();
    format!("{scheme}://{hostname}/.default")
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use azure_core::{
        credentials::{AccessToken, TokenCredential},
        date,
    };
    use time::OffsetDateTime;
    use url::Url;

    use crate::{
        pipeline::{
            authorization_policy::{generate_authorization, scope_from_url, Credential},
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
        async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
            let token = format!("{}+{}", self.0, scopes.join(","));
            Ok(AccessToken::new(
                token,
                OffsetDateTime::now_utc().saturating_add(time::Duration::minutes(5)),
            ))
        }

        async fn clear_cache(&self) -> azure_core::Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn generate_authorization_for_token_credential() {
        let time_nonce = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let date_string = date::to_rfc1123(&time_nonce).to_lowercase();
        let cred = Arc::new(TestTokenCredential("test_token".to_string()));
        let auth_token = Credential::Token(cred);

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget::new(
                azure_core::Method::Get,
                &ResourceLink::root(ResourceType::Databases).item("ToDoList"),
                &date_string,
            ),
        )
        .await
        .unwrap();

        let expected: String = url_encode(
            b"type=aad&ver=1.0&sig=test_token+https://test_account.example.com/.default",
        );

        assert_eq!(ret, expected);
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_primary_key_0() {
        let time_nonce = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let date_string = date::to_rfc1123(&time_nonce).to_lowercase();

        let auth_token = Credential::PrimaryKey(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".into(),
        );

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget::new(
                azure_core::Method::Get,
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
        let time_nonce = date::parse_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();
        let date_string = date::to_rfc1123(&time_nonce).to_lowercase();

        let auth_token = Credential::PrimaryKey(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".into(),
        );

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget::new(
                azure_core::Method::Get,
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

    #[test]
    fn scope_from_url_extracts_correct_scope() {
        let scope = scope_from_url(&Url::parse("https://example.com/dbs/test_db/colls").unwrap());
        assert_eq!(scope, "https://example.com/.default");
    }
}
