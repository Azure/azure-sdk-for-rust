use crate::clients::DatabaseStruct;
use crate::headers::*;
use crate::requests;
use crate::{
    AuthorizationToken, CosmosClient, HasHttpClient, IntoDatabaseClient, ResourceType,
    WithDatabaseClient,
};
use azure_core::{HttpClient, No, ToAssign, Yes};
use core::marker::PhantomData;
use http::request::Builder as RequestBuilder;
use http::{header, HeaderValue};
use ring::hmac;
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;
use url::form_urlencoded;

const AZURE_VERSION: &str = "2018-12-31";
const VERSION: &str = "1.0";
const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";

pub trait CosmosUriBuilder: Send + Sync {
    fn build_base_uri(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    http_client: Arc<Box<dyn HttpClient>>,
    account: Cow<'a, str>,
    auth_token: AuthorizationToken,
    cosmos_uri_builder: CUB,
}

impl<'a, CUB> CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Clone,
{
    pub fn with_auth_token(&self, auth_token: AuthorizationToken) -> Self {
        Self {
            http_client: self.http_client.clone(),
            account: self.account.clone(),
            auth_token,
            cosmos_uri_builder: self.cosmos_uri_builder.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DefaultCosmosUri {
    uri: String,
}

impl DefaultCosmosUri {
    pub(crate) fn new(account: &str) -> DefaultCosmosUri {
        DefaultCosmosUri {
            uri: format!("https://{}.documents.azure.com", account),
        }
    }
}

impl CosmosUriBuilder for DefaultCosmosUri {
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChinaCosmosUri {
    pub(crate) uri: String,
}

impl ChinaCosmosUri {
    pub(crate) fn new(account: &str) -> ChinaCosmosUri {
        ChinaCosmosUri {
            uri: format!("https://{}.documents.azure.cn", account),
        }
    }
}

impl CosmosUriBuilder for ChinaCosmosUri {
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

#[derive(Debug, Clone, Default)]
pub struct CustomCosmosUri {
    pub(crate) uri: String,
}

impl CosmosUriBuilder for CustomCosmosUri {
    #[inline]
    fn build_base_uri(&self) -> &str {
        &self.uri
    }
}

pub struct ClientBuilder<'a, CUB, HTTPClientToAssign>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) http_client: Option<Arc<Box<dyn HttpClient>>>,
    pub(crate) p_http_client_to_assign: PhantomData<HTTPClientToAssign>,
    pub(crate) account: Cow<'a, str>,
    pub(crate) auth_token: AuthorizationToken,
    pub(crate) cosmos_uri_builder: CUB,
}

#[derive(Debug, Clone, Copy)]
pub struct HTTPClientNotAssigned {}
impl ToAssign for HTTPClientNotAssigned {}

impl<'a, CUB> ClientBuilder<'a, CUB, HTTPClientNotAssigned>
where
    CUB: CosmosUriBuilder,
{
    pub fn with_http_client(
        self,
        http_client: Arc<Box<dyn HttpClient>>,
    ) -> ClientBuilder<'a, CUB, Yes> {
        ClientBuilder {
            http_client: Some(http_client),
            p_http_client_to_assign: PhantomData {},
            account: self.account,
            auth_token: self.auth_token,
            cosmos_uri_builder: self.cosmos_uri_builder,
        }
    }
}

impl<'a, CUB> ClientBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub fn build(self) -> CosmosStruct<'a, CUB> {
        CosmosStruct {
            http_client: self.http_client.unwrap(),
            account: self.account,
            auth_token: self.auth_token,
            cosmos_uri_builder: self.cosmos_uri_builder,
        }
    }
}

impl<'a, CUB> HasHttpClient for CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Debug,
{
    #[inline]
    fn http_client(&self) -> &dyn HttpClient {
        self.http_client.as_ref().as_ref()
    }
}

impl<'a, CUB> CosmosClient for CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Debug,
{
    fn create_database(&self) -> requests::CreateDatabaseBuilder<'_, No> {
        requests::CreateDatabaseBuilder::new(self)
    }

    fn list_databases(&self) -> requests::ListDatabasesBuilder<'_> {
        requests::ListDatabasesBuilder::new(self)
    }

    #[inline]
    fn prepare_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
        resource_type: ResourceType,
    ) -> RequestBuilder {
        let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            generate_authorization(
                &self.auth_token,
                &http_method,
                resource_type,
                resource_link,
                &time,
            )
        };
        self.prepare_request_with_signature(uri_path, http_method, &time, &auth)
    }
}

impl<'a, CUB> IntoDatabaseClient<'a, Self, DatabaseStruct<'a, Self>> for CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Debug + Clone,
{
    fn into_database_client<IntoCowStr>(self, database_name: IntoCowStr) -> DatabaseStruct<'a, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        DatabaseStruct::new(Cow::Owned(self), database_name.into())
    }
}

impl<'a, CUB> WithDatabaseClient<'a, Self, DatabaseStruct<'a, Self>> for CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Debug + Clone,
{
    fn with_database_client<IntoCowStr>(
        &'a self,
        database_name: IntoCowStr,
    ) -> DatabaseStruct<'a, Self>
    where
        IntoCowStr: Into<Cow<'a, str>>,
    {
        DatabaseStruct::new(Cow::Borrowed(self), database_name.into())
    }
}

impl<'a, CUB> CosmosStruct<'a, CUB>
where
    CUB: CosmosUriBuilder + Debug,
{
    #[inline]
    fn prepare_request_with_signature(
        &self,
        uri_path: &str,
        http_method: http::Method,
        time: &str,
        signature: &str,
    ) -> RequestBuilder {
        trace!("prepare_request::auth == {:?}", signature);
        let uri = format!("{}/{}", self.cosmos_uri_builder.build_base_uri(), uri_path);
        debug!(
            "cosmos::client::prepare_request_with_resource_signature::uri == {:?}",
            uri
        );

        http::request::Builder::new()
            .method(http_method)
            .uri(uri)
            .header(HEADER_DATE, time)
            .header(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION))
            .header(header::AUTHORIZATION, signature)
    }
}

fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &http::Method,
    resource_type: ResourceType,
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
            AuthorizationToken::Master(_) => "master",
            AuthorizationToken::Resource(_) => "resource",
        },
        VERSION,
        match auth_token {
            AuthorizationToken::Master(key) => Cow::Owned(encode_str_to_sign(&string_to_sign, key)),
            AuthorizationToken::Resource(key) => Cow::Borrowed(key),
        },
    );

    debug!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    form_urlencoded::byte_serialize(&str_unencoded.as_bytes()).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, key: &[u8]) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, key);
    let sig = hmac::sign(&key, str_to_sign.as_bytes());
    base64::encode(sig.as_ref())
}

fn string_to_sign(
    http_method: &http::Method,
    rt: ResourceType,
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
    // Notice the empty string at the end so we need to add two carriage returns

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

fn generate_resource_link(u: &str) -> &str {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            &http::Method::GET,
            ResourceType::Databases,
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

        let auth_token = AuthorizationToken::new_master(
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            ResourceType::Databases,
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

        let auth_token = AuthorizationToken::new_master(
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
        )
        .unwrap();

        let ret = generate_authorization(
            &auth_token,
            &http::Method::GET,
            ResourceType::Databases,
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
