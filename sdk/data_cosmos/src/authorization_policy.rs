use crate::{resources::permission::AuthorizationToken, resources::ResourceType};
use azure_core::{
    date,
    headers::{HeaderValue, AUTHORIZATION, MS_DATE, VERSION},
    hmac::hmac_sha256,
    Context, Policy, PolicyResult, Request, Url,
};
use std::{borrow::Cow, sync::Arc};
use time::OffsetDateTime;
use tracing::trace;
use url::form_urlencoded;

const AZURE_VERSION: &str = "2018-12-31";
const VERSION_NUMBER: &str = "1.0";

/// The `AuthorizationPolicy` takes care to authenticate your calls to Azure `CosmosDB`.
///
/// Currently it supports two type of authorization: one at service level and another at resource level (see
/// [`AuthorizationToken`] for more info). The policy must be added just before the transport policy
/// because it needs to inspect the values that are about to be sent to the transport and inject
/// the proper authorization token.
///
/// The `AuthorizationPolicy` is the only owner of the passed credentials, so if you want to
/// authenticate the same operation with different credentials, all you have to do is to swap the
/// `AuthorizationPolicy`.
///
/// This struct implements `Debug` but secrets are encrypted by `AuthorizationToken` so there is no risk of
/// leaks in debug logs (secrets are stored in cleartext in memory: dumps are still leaky).
#[derive(Debug, Clone)]
pub struct AuthorizationPolicy {
    authorization_token: AuthorizationToken,
}

impl AuthorizationPolicy {
    pub(crate) fn new(authorization_token: AuthorizationToken) -> Self {
        Self {
            authorization_token,
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

        let auth = {
            let resource_link = generate_resource_link(request);
            generate_authorization(
                &self.authorization_token,
                request.method(),
                request.url(),
                ctx.get()
                    .expect("ResourceType must be in the Context at this point"),
                &resource_link,
                time_nonce,
            )
            .await?
        };

        trace!(
            "AuthorizationPolicy calculated authorization == {:?}: {}",
            AUTHORIZATION,
            &auth
        );

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
///    starts with a leading slash so this check will not match uri compsed **only** by the
///    `ENDING_STRING`.
/// 3. Find if the uri **is** the ending string (without the leading slash). If so return an empty
///    string. This covers the exception of the rule above.
/// 4. Return the received uri unchanged.
fn generate_resource_link(request: &Request) -> String {
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
    trace!("uri used by AuthorizationPolicy == {:#?}", uri);

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

/// The `CosmosDB` authorization can either be:
/// - "primary": one of the two service-level tokens
/// - "resource": e.g. a single database
/// - "aad": Azure Active Directory token
///
/// In the "primary" case the signature must be constructed by signing the HTTP method,
/// resource type, resource link (the relative URI) and the current time.
///
/// In the "resource" case, the signature is just the resource key.
///
/// In the "aad" case, the signature is the AAD token.
async fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &azure_core::Method,
    url: &Url,
    resource_type: &ResourceType,
    resource_link: &str,
    time_nonce: OffsetDateTime,
) -> azure_core::Result<String> {
    let (authorization_type, signature) = match auth_token {
        AuthorizationToken::PrimaryKey(key) => {
            let string_to_sign =
                string_to_sign(http_method, resource_type, resource_link, time_nonce);
            ("master", Cow::Owned(hmac_sha256(&string_to_sign, key)?))
        }
        AuthorizationToken::Resource(key) => ("resource", Cow::Borrowed(key)),
        AuthorizationToken::TokenCredential(token_credential) => (
            "aad",
            Cow::Owned(
                token_credential
                    .get_token(&[&scope_from_url(url)])
                    .await?
                    .token
                    .secret()
                    .to_string(),
            ),
        ),
    };

    let str_unencoded = format!("type={authorization_type}&ver={VERSION_NUMBER}&sig={signature}");
    trace!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

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
fn string_to_sign(
    http_method: &azure_core::Method,
    rt: &ResourceType,
    resource_link: &str,
    time_nonce: OffsetDateTime,
) -> String {
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
        match *http_method {
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
        match rt {
            ResourceType::Databases => "dbs",
            ResourceType::Collections => "colls",
            ResourceType::Documents => "docs",
            ResourceType::StoredProcedures => "sprocs",
            ResourceType::Users => "users",
            ResourceType::Permissions => "permissions",
            ResourceType::Attachments => "attachments",
            ResourceType::PartitionKeyRanges => "pkranges",
            ResourceType::UserDefinedFunctions => "udfs",
            ResourceType::Triggers => "triggers",
        },
        resource_link,
        date::to_rfc1123(&time_nonce).to_lowercase()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_sign_00() {
        let time = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();

        let ret = string_to_sign(
            &azure_core::Method::Get,
            &ResourceType::Databases,
            "dbs/MyDatabase/colls/MyCollection",
            time,
        );
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
    async fn generate_authorization_00() {
        let time = date::parse_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();

        let auth_token = AuthorizationToken::primary_key(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
        )
        .unwrap();

        let url = Url::parse("https://.documents.azure.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &azure_core::Method::Get,
            &url,
            &ResourceType::Databases,
            "dbs/MyDatabase/colls/MyCollection",
            time,
        )
        .await
        .unwrap();

        assert_eq!(
            ret,
            "type%3Dmaster%26ver%3D1.0%26sig%3DQkz%2Fr%2B1N2%2BPEnNijxGbGB%2FADvLsLBQmZ7uBBMuIwf4I%3D"
        );
    }

    #[tokio::test]
    async fn generate_authorization_01() {
        let time = date::parse_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();

        let auth_token = AuthorizationToken::primary_key(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
        )
        .unwrap();

        let url = Url::parse("https://.documents.azure.com/dbs/ToDoList").unwrap();

        let ret = generate_authorization(
            &auth_token,
            &azure_core::Method::Get,
            &url,
            &ResourceType::Databases,
            "dbs/ToDoList",
            time,
        )
        .await
        .unwrap();

        // This is the result shown in the MSDN page. It's clearly wrong :)
        // below is the correct one.
        //assert_eq!(ret,
        //           "type%3dmaster%26ver%3d1.0%26sig%3dc09PEVJrgp2uQRkr934kFbTqhByc7TVr3O");

        assert_eq!(
            ret,
            "type%3Dmaster%26ver%3D1.0%26sig%3DKvBM8vONofkv3yKm%2F8zD9MEGlbu6jjHDJBp4E9c2ZZI%3D"
        );
    }

    #[test]
    fn generate_resource_link_00() {
        let request = Request::new(
            Url::parse("https://.documents.azure.com/dbs/second").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&generate_resource_link(&request), "dbs/second");
    }

    #[test]
    fn generate_resource_link_01() {
        let request = Request::new(
            Url::parse("https://.documents.azure.com/dbs").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&generate_resource_link(&request), "");
    }

    #[test]
    fn generate_resource_link_02() {
        let request = Request::new(
            Url::parse("https://.documents.azure.com/colls/second/third").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&generate_resource_link(&request), "colls/second/third");
    }

    #[test]
    fn generate_resource_link_03() {
        let request = Request::new(
            Url::parse("https://.documents.azure.com/dbs/test_db/colls").unwrap(),
            azure_core::Method::Get,
        );
        assert_eq!(&generate_resource_link(&request), "dbs/test_db");
    }

    #[test]
    fn scope_from_url_01() {
        let scope =
            scope_from_url(&Url::parse("https://.documents.azure.com/dbs/test_db/colls").unwrap());
        assert_eq!(scope, "https://.documents.azure.com/.default");
    }
}
