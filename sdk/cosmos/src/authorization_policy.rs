use crate::headers::{HEADER_DATE, HEADER_VERSION};
use crate::resources::permission::AuthorizationToken;
use crate::resources::ResourceType;
use azure_core::{PipelineContext, Policy, PolicyResult, Request, Response};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use ring::hmac;
use std::borrow::Cow;
use std::sync::Arc;
use url::form_urlencoded;

const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";
const AZURE_VERSION: &str = "2018-12-31";
const VERSION: &str = "1.0";

// We can implement Debug without leaking secrets because `AuthorizationToken`
// already masks the secure bits on its own.
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct CosmosContext {
    pub(crate) resource_type: ResourceType,
}

impl From<ResourceType> for CosmosContext {
    fn from(resource_type: ResourceType) -> Self {
        Self { resource_type }
    }
}

#[async_trait::async_trait]
impl Policy<CosmosContext> for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &mut PipelineContext<CosmosContext>,
        request: &mut Request,
        next: &[Arc<dyn Policy<CosmosContext>>],
    ) -> PolicyResult<Response> {
        trace!("called AuthorizationPolicy::send. self == {:#?}", self);

        if next.is_empty() {
            return Err(Box::new(azure_core::PipelineError::InvalidTailPolicy(
                "Authorization policies cannot be the last policy of a pipeline".to_owned(),
            )));
        }

        let time = chrono::Utc::now().format(TIME_FORMAT).to_string();

        let uri_path = &request.uri().path_and_query().unwrap().to_string()[1..];
        trace!("uri_path used by AuthorizationPolicy == {:#?}", uri_path);

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            debug!("resource_link == {}", resource_link);
            generate_authorization(
                &self.authorization_token,
                &request.method(),
                &ctx.get_bag().resource_type,
                resource_link,
                &time,
            )
        };

        trace!(
            "AuthorizationPolicy calculated authorization == {}: {}",
            AUTHORIZATION,
            &auth
        );

        request
            .headers_mut()
            .append(HEADER_DATE, HeaderValue::from_str(&time)?);
        request
            .headers_mut()
            .append(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION));
        request
            .headers_mut()
            .append(AUTHORIZATION, HeaderValue::from_str(&auth)?);

        trace!("\n\nrequest =={:?}", request);

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}

// TODO: will become private as soon as cosmos_client will be migrated
// to pipeline arch.
pub(crate) fn generate_resource_link(u: &str) -> &str {
    static ENDING_STRINGS: &[&str] = &[
        "dbs",
        "colls",
        "docs",
        "sprocs",
        "users",
        "permissions",
        "attachments",
        "pkranges",
        "udfs",
        "triggers",
    ];

    // store the element only if it does not end with dbs, colls or docs
    let p = u;
    let len = p.len();
    for str_to_match in ENDING_STRINGS {
        let end_len = str_to_match.len();

        if end_len <= len {
            let end_offset = len - end_len;
            let sm = &p[end_offset..];
            if sm == *str_to_match {
                if len == end_len {
                    return "";
                }

                if &p[end_offset - 1..end_offset] == "/" {
                    let ret = &p[0..len - end_len - 1];
                    return ret;
                }
            }
        }
    }
    p
}

// TODO: make it private after pipeline migration
pub(crate) fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &http::Method,
    resource_type: &ResourceType,
    resource_link: &str,
    time: &str,
) -> String {
    let string_to_sign = string_to_sign(http_method, resource_type, resource_link, time);
    debug!(
        "generate_authorization::string_to_sign == {:?}",
        string_to_sign
    );

    let str_unencoded = format!(
        "type={}&ver={}&sig={}",
        match auth_token {
            AuthorizationToken::Primary(_) => "master",
            AuthorizationToken::Resource(_) => "resource",
        },
        VERSION,
        match auth_token {
            AuthorizationToken::Primary(key) =>
                Cow::Owned(encode_str_to_sign(&string_to_sign, key)),
            AuthorizationToken::Resource(key) => Cow::Borrowed(key),
        },
    );
    debug!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    form_urlencoded::byte_serialize(&str_unencoded.as_bytes()).collect::<String>()
}

fn string_to_sign(
    http_method: &http::Method,
    rt: &ResourceType,
    resource_link: &str,
    time: &str,
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
        time.to_lowercase()
    )
}

fn encode_str_to_sign(str_to_sign: &str, key: &[u8]) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, key);
    let sig = hmac::sign(&key, str_to_sign.as_bytes());
    base64::encode(sig.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            &http::Method::GET,
            &ResourceType::Databases,
            "dbs/MyDatabase/colls/MyCollection",
            &time,
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
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let auth_token = AuthorizationToken::primary_from_base64(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            &ResourceType::Databases,
            "dbs/MyDatabase/colls/MyCollection",
            &time,
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
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let auth_token = AuthorizationToken::primary_from_base64(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            &ResourceType::Databases,
            "dbs/ToDoList",
            &time,
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
