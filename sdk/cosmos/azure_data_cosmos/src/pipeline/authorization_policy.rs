//! Defines Cosmos DB's unique Authentication Policy.
//!
//! The Cosmos DB data plane doesn't use a standard `Authorization: Bearer` header for authentication.
//! Instead, it uses a custom header format, as defined in the [official documentation](https://docs.microsoft.com/en-us/rest/api/cosmos-db/access-control-on-cosmosdb-resources).
//! We implement that policy here, because we can't use any standard Azure SDK authentication policy.

use azure_core::credentials::TokenCredential;
use azure_core::date::OffsetDateTime;
use azure_core::{
    date,
    headers::{HeaderValue, AUTHORIZATION, MS_DATE, VERSION},
    Context, Policy, PolicyResult, Request, Url,
};
use std::sync::Arc;
use tracing::trace;
use url::form_urlencoded;

#[cfg(feature = "key_auth")]
use azure_core::{credentials::Secret, hmac::hmac_sha256};

const AZURE_VERSION: &str = "2018-12-31";
const VERSION_NUMBER: &str = "1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // For the variants. Can be removed when we have them all implemented.
pub(crate) enum ResourceType {
    Databases,
    Containers,
    Items,
    StoredProcedures,
    Users,
    Permissions,
    Attachments,
    PartitionKeyRanges,
    UserDefinedFunctions,
    Triggers,
}

#[derive(Debug, Clone)]
enum Credential {
    /// The credential is an Entra ID token.
    Token(Arc<dyn TokenCredential>),

    /// The credential is a key to be used to sign the HTTP request (a shared key)
    #[cfg(feature = "key_auth")]
    PrimaryKey(Secret),
}

#[cfg(feature = "key_auth")]
struct SignatureTarget<'a> {
    http_method: &'a azure_core::Method,
    resource_type: &'a ResourceType,
    resource_link: &'a str,
    time_nonce: OffsetDateTime,
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

        let time_nonce = OffsetDateTime::now_utc();

        let resource_link = extract_resource_link(request);
        let resource_type: &ResourceType = ctx
            .value()
            .expect("ResourceType must be in the Context at this point");
        let auth = generate_authorization(
            &self.credential,
            request.url(),
            #[cfg(feature = "key_auth")]
            SignatureTarget {
                http_method: request.method(),
                resource_type,
                resource_link: &resource_link,
                time_nonce,
            },
        )
        .await?;

        trace!(?resource_type, resource_link, "AuthorizationPolicy applied");

        request.insert_header(MS_DATE, HeaderValue::from(date::to_rfc1123(&time_nonce)));
        request.insert_header(VERSION, HeaderValue::from_static(AZURE_VERSION));
        request.insert_header(AUTHORIZATION, HeaderValue::from(auth));

        // next[0] will not panic, because we checked at the beginning of the function
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// This function strips the leading slash and the resource name from the uri of the passed request.
/// It does not strip the resource name if the resource name is not present. This is accomplished in
/// four steps (with eager return):
/// 1. Strip leading slash from the uri of the passed request.
/// 2. Find if the uri ends with a `ENDING_STRING`. If so, strip it and return. Every `ENDING_STRING`
///    starts with a leading slash so this check will not match uri composed **only** of the
///    `ENDING_STRING`.
/// 3. Find if the uri **is** the ending string (without the leading slash). If so return an empty
///    string. This covers the exception of the rule above.
/// 4. Return the received uri unchanged.
fn extract_resource_link(request: &Request) -> String {
    static ENDING_STRINGS: &[&str] = &[
        "/dbs",
        "/colls",
        "/docs",
        "/sprocs",
        "/users",
        "/permissions",
        "/attachments",
        "/pkranges",
        "/udfs",
        "/triggers",
    ];

    // This strips the leading slash from the uri of the passed request.
    let uri_path = request.path_and_query();
    let uri = uri_path.trim_start_matches('/');

    // We find the above resource names. If found, we strip it and eagerly return. Note that the
    // resource names have a leading slash so the suffix will match `test/users` but not
    // `test-users`.
    for ending in ENDING_STRINGS {
        if let Some(uri_without_ending) = uri.strip_suffix(ending) {
            return uri_without_ending.to_string();
        }
    }

    // This check handles the uris comprised by resource names only. It will match `users` and
    // return an empty string. This is necessary because the previous check included a leading
    // slash.
    if ENDING_STRINGS
        .iter()
        .map(|ending| &ending[1..]) // this is safe since every ENDING_STRING starts with a slash
        .any(|item| uri == item)
    {
        String::new()
    } else {
        uri.to_string()
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
    #[cfg(feature = "key_auth")] signature_target: SignatureTarget<'a>,
) -> azure_core::Result<String> {
    let (authorization_type, signature) = match auth_token {
        Credential::Token(token_credential) => (
            "aad",
            token_credential
                .get_token(&[&scope_from_url(url)])
                .await?
                .token
                .secret()
                .to_string(),
        ),

        #[cfg(feature = "key_auth")]
        Credential::PrimaryKey(key) => {
            let string_to_sign = string_to_sign(signature_target);
            ("master", hmac_sha256(&string_to_sign, key)?)
        }
    };

    let str_unencoded = format!("type={authorization_type}&ver={VERSION_NUMBER}&sig={signature}");

    Ok(form_urlencoded::byte_serialize(str_unencoded.as_bytes()).collect::<String>())
}

/// This function generates the scope string from the passed url. The scope string is used to
/// request the AAD token.
fn scope_from_url(url: &Url) -> String {
    let scheme = url.scheme();
    let hostname = url.host_str().unwrap();
    format!("{scheme}://{hostname}/.default")
}

/// This function generates a valid authorization string, according to the documentation.
/// In case of authorization problems we can compare the `string_to_sign` generated by Azure against
/// our own.
#[cfg(feature = "key_auth")]
fn string_to_sign(signature_target: SignatureTarget) -> String {
    // From official docs:
    // StringToSign =
    //      Verb.toLowerCase() + "\n" +
    //      ResourceType.toLowerCase() + "\n" +
    //      ResourceLink + "\n" +
    //      Date.toLowerCase() + "\n" +
    //      "" + "\n";
    // Notice the empty string at the end so we need to add two new lines

    format!(
        "{}\n{}\n{}\n{}\n\n",
        match *signature_target.http_method {
            azure_core::Method::Get => "get",
            azure_core::Method::Put => "put",
            azure_core::Method::Post => "post",
            azure_core::Method::Delete => "delete",
            azure_core::Method::Head => "head",
            azure_core::Method::Trace => "trace",
            azure_core::Method::Options => "options",
            azure_core::Method::Connect => "connect",
            azure_core::Method::Patch => "patch",
            _ => "extension",
        },
        match signature_target.resource_type {
            ResourceType::Databases => "dbs",
            ResourceType::Containers => "colls", // The rest API uses the old term "colls" (referring to 'collections') to refer to containers
            ResourceType::Items => "docs", // The rest API uses the old term "docs" (referring to 'documents') to refer to items
            ResourceType::StoredProcedures => "sprocs",
            ResourceType::Users => "users",
            ResourceType::Permissions => "permissions",
            ResourceType::Attachments => "attachments",
            ResourceType::PartitionKeyRanges => "pkranges",
            ResourceType::UserDefinedFunctions => "udfs",
            ResourceType::Triggers => "triggers",
        },
        signature_target.resource_link,
        date::to_rfc1123(&signature_target.time_nonce).to_lowercase()
    )
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "key_auth")]
    use azure_core::credentials::AccessToken;

    use super::*;

    #[derive(Debug)]
    #[cfg(feature = "key_auth")]
    struct TestTokenCredential(String);

    #[cfg(feature = "key_auth")]
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

    #[test]
    #[cfg(feature = "key_auth")]
    fn string_to_sign_generates_expected_string_for_signing() {
        let time_nonce = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();

        let ret = string_to_sign(SignatureTarget {
            http_method: &azure_core::Method::Get,
            resource_type: &ResourceType::Databases,
            resource_link: "dbs/MyDatabase/colls/MyCollection",
            time_nonce,
        });
        assert_eq!(
            ret,
            "get
dbs
dbs/MyDatabase/colls/MyCollection
mon, 01 jan 1900 01:00:00 gmt

"
        );
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_token_credential() {
        let time_nonce = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let cred = Arc::new(TestTokenCredential("test_token".to_string()));
        let auth_token = Credential::Token(cred);

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget {
                http_method: &azure_core::Method::Get,
                resource_type: &ResourceType::Databases,
                resource_link: "dbs/MyDatabase/colls/MyCollection",
                time_nonce,
            },
        )
        .await
        .unwrap();

        let expected: String = form_urlencoded::byte_serialize(
            b"type=aad&ver=1.0&sig=test_token+https://test_account.example.com/.default",
        )
        .collect();

        assert_eq!(ret, expected);
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_primary_key_0() {
        let time_nonce = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();

        let auth_token = Credential::PrimaryKey(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".into(),
        );

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget {
                http_method: &azure_core::Method::Get,
                resource_type: &ResourceType::Databases,
                resource_link: "dbs/MyDatabase/colls/MyCollection",
                time_nonce,
            },
        )
        .await
        .unwrap();

        let expected: String = form_urlencoded::byte_serialize(
            b"type=master&ver=1.0&sig=Qkz/r+1N2+PEnNijxGbGB/ADvLsLBQmZ7uBBMuIwf4I=",
        )
        .collect();

        assert_eq!(ret, expected);
    }

    #[tokio::test]
    #[cfg(feature = "key_auth")]
    async fn generate_authorization_for_primary_key_1() {
        let time_nonce = date::parse_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();

        let auth_token = Credential::PrimaryKey(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".into(),
        );

        // Use a fake URL since the actual endpoint URL is not important for this test
        let url = Url::parse("https://test_account.example.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &url,
            SignatureTarget {
                http_method: &azure_core::Method::Get,
                resource_type: &ResourceType::Databases,
                resource_link: "dbs/ToDoList",
                time_nonce,
            },
        )
        .await
        .unwrap();

        let expected: String = form_urlencoded::byte_serialize(
            b"type=master&ver=1.0&sig=KvBM8vONofkv3yKm/8zD9MEGlbu6jjHDJBp4E9c2ZZI=",
        )
        .collect();

        assert_eq!(ret, expected);
    }

    #[test]
    fn extract_resource_link_specific_db() {
        let request = Request::new(
            Url::parse("https://example.com/dbs/second").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&extract_resource_link(&request), "dbs/second");
    }

    #[test]
    fn extract_resource_link_dbs_root() {
        let request = Request::new(
            Url::parse("https://example.com/dbs").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&extract_resource_link(&request), "");
    }

    #[test]
    fn extract_resource_link_collection_nested() {
        let request = Request::new(
            Url::parse("https://example.com/colls/second/third").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&extract_resource_link(&request), "colls/second/third");
    }

    #[test]
    fn extract_resource_link_collections_root() {
        let request = Request::new(
            Url::parse("https://.documents.azure.com/dbs/test_db/colls").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&extract_resource_link(&request), "dbs/test_db");
    }

    #[test]
    fn scope_from_url_extracts_correct_scope() {
        let scope = scope_from_url(&Url::parse("https://example.com/dbs/test_db/colls").unwrap());
        assert_eq!(scope, "https://example.com/.default");
    }
}
