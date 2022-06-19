use crate::headers::{HEADER_DATE, HEADER_VERSION};
use crate::resources::permission::AuthorizationToken;
use crate::resources::ResourceType;
use crate::TimeNonce;
use azure_core::{Context, Policy, PolicyResult, Request};
use hmac::{Hmac, Mac};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use sha2::Sha256;
use std::borrow::Cow;
use std::sync::Arc;
use url::form_urlencoded;

const AZURE_VERSION: &str = "2018-12-31";
const VERSION: &str = "1.0";

/// The `AuthorizationPolicy` takes care to authenticate your calls to Azure CosmosDB.
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
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[async_trait::async_trait]
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

        let time_nonce = TimeNonce::new();

        let uri_path = request.path_and_query();
        trace!("uri_path used by AuthorizationPolicy == {:#?}", uri_path);

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            generate_authorization(
                &self.authorization_token,
                request.method(),
                ctx.get()
                    .expect("ResourceType must be in the Context at this point"),
                resource_link,
                time_nonce,
            )
        };

        trace!(
            "AuthorizationPolicy calculated authorization == {}: {}",
            AUTHORIZATION,
            &auth
        );

        request
            .headers_mut()
            .insert(HEADER_DATE, HeaderValue::from_str(&time_nonce.to_string())?);
        request
            .headers_mut()
            .insert(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION));
        request
            .headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_str(&auth)?);

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// This function strips the resource name from the passed uri. It does not alter the uri if a
/// resource name is not present. This is accomplished in three steps (with eager return):
/// 1. Find if the uri ends with a ENDING_STRING. If so, strip it and return. Every ENDING_STRING
///    starts with a leading slash so this check will not match uri compsed **only** by the
///    ENDING_STRING.
/// 2. Find if the uri **is** the ending string (without the leading slash). If so return an empty
///    string. This covers the exception of the rule above.
/// 3. Return the received uri unchanged.
fn generate_resource_link(uri: &str) -> &str {
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

    // We find the above resource names. If found, we strip it and eagerly return. Note that the
    // resource names have a leading slash so the suffix will match `test/users` but not
    // `test-users`.
    for ending in ENDING_STRINGS {
        if let Some(uri_without_ending) = uri.strip_suffix(ending) {
            return uri_without_ending;
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
        ""
    } else {
        uri
    }
}

/// The CosmosDB authorization can either be "primary" (i.e., one of the two service-level tokens) or
/// "resource" (i.e., a single database). In the first case the signature must be constructed by
/// signing the HTTP method, resource type, resource link (the relative URI) and the current time.
/// In the second case, the signature is just the resource key.
fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &http::Method,
    resource_type: &ResourceType,
    resource_link: &str,
    time_nonce: TimeNonce,
) -> String {
    let (authorization_type, signature) = match auth_token {
        AuthorizationToken::Primary(key) => {
            let string_to_sign =
                string_to_sign(http_method, resource_type, resource_link, time_nonce);
            (
                "master",
                Cow::Owned(encode_str_to_sign(&string_to_sign, key)),
            )
        }
        AuthorizationToken::Resource(key) => ("resource", Cow::Borrowed(key)),
    };

    let str_unencoded = format!(
        "type={}&ver={}&sig={}",
        authorization_type, VERSION, signature
    );
    trace!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    form_urlencoded::byte_serialize(str_unencoded.as_bytes()).collect::<String>()
}

/// This function generates a valid authorization string, according to the documentation.
/// In case of authorization problems we can compare the string_to_sign generated by Azure against
/// our own.
fn string_to_sign(
    http_method: &http::Method,
    rt: &ResourceType,
    resource_link: &str,
    time_nonce: TimeNonce,
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
            http::Method::GET => "get",
            http::Method::PUT => "put",
            http::Method::POST => "post",
            http::Method::DELETE => "delete",
            http::Method::HEAD => "head",
            http::Method::TRACE => "trace",
            http::Method::OPTIONS => "options",
            http::Method::CONNECT => "connect",
            http::Method::PATCH => "patch",
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
        time_nonce.to_string().to_lowercase()
    )
}

/// This function HMAC_SHA256 signs the passed string, given the supplied key. The passed string
/// will be encoded as per its UTF-8 representation. The resulting byte array is then base64
/// encoded and returned to the caller. Possibile optimization: profile if the HMAC struct
/// initialization is expensive and, if so, cache it somehow to avoid recreating it at every
/// request.
fn encode_str_to_sign(data: &str, key: &[u8]) -> String {
    let mut hmac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    base64::encode(&signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc).into();

        let ret = string_to_sign(
            &http::Method::GET,
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

    #[test]
    fn generate_authorization_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc).into();

        let auth_token = AuthorizationToken::primary_from_base64(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            &ResourceType::Databases,
            "dbs/MyDatabase/colls/MyCollection",
            time,
        );
        assert_eq!(
            ret,
            "type%3Dmaster%26ver%3D1.0%26sig%3DQkz%2Fr%2B1N2%2BPEnNijxGbGB%2FADvLsLBQmZ7uBBMuIwf4I%3D"
        );
    }

    #[test]
    fn generate_authorization_01() {
        let time =
            chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc).into();

        let auth_token = AuthorizationToken::primary_from_base64(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            &ResourceType::Databases,
            "dbs/ToDoList",
            time,
        );

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
        assert_eq!(generate_resource_link("dbs/second"), "dbs/second");
        assert_eq!(generate_resource_link("dbs"), "");
        assert_eq!(
            generate_resource_link("colls/second/third"),
            "colls/second/third"
        );
        assert_eq!(generate_resource_link("dbs/test_db/colls"), "dbs/test_db");
    }
}
