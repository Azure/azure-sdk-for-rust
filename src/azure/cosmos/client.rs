use azure::cosmos::authorization_token::{TokenType, AuthorizationToken};
use azure::core::HTTPMethod;

use azure::cosmos::database::Database;
use azure::cosmos::collection::Collection;

use azure::core::errors::{AzureError, check_status_extract_body, check_status};

use azure::cosmos::request_response::{ListDatabasesResponse, CreateDatabaseRequest,
                                      ListCollectionsResponse};

use url;

use std::io::{Read, Cursor};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

use base64;
use hyper;
use serde_json;
use hyper::header::{ContentLength, Headers};
use hyper::status::StatusCode;
use hyper_native_tls;

use chrono;

use url::percent_encoding::utf8_percent_encode;


const AZURE_VERSION: &'static str = "2017-02-22";
const VERSION: &'static str = "1.0";
const TIME_FORMAT: &'static str = "%a, %d %h %Y %T GMT";

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }
header! { (OfferThroughput, "x-ms-offer-throughput") => [u64] }

define_encode_set! {
    pub COMPLETE_ENCODE_SET = [url::percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}


#[derive(Clone, Copy)]
pub enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub struct Client<'a> {
    hyper_client: hyper::client::Client,
    authorization_token: &'a AuthorizationToken<'a>,
}

impl<'a> Client<'a> {
    pub fn new(authorization_token: &'a AuthorizationToken<'a>)
               -> Result<Client<'a>, hyper_native_tls::native_tls::Error> {
        let ssl = hyper_native_tls::NativeTlsClient::new()?;
        let connector = hyper::net::HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        Ok(Client {
               hyper_client: client,
               authorization_token: authorization_token,
           })
    }

    pub fn set_authorization_token(&mut self, at: &'a AuthorizationToken<'a>) {
        self.authorization_token = at;
    }

    fn perform_request(&self,
                       url: &url::Url,
                       http_method: HTTPMethod,
                       request_body: Option<(&mut Read, u64)>,
                       resource_type: ResourceType,
                       headers: Option<Headers>)
                       -> Result<hyper::client::Response, AzureError> {
        let dt = chrono::UTC::now();
        let time = format!("{}", dt.format(TIME_FORMAT));


        // to do: calculate resource link
        let resource_link = generate_resource_link(url);

        let auth = generate_authorization(self.authorization_token,
                                          http_method,
                                          resource_type,
                                          resource_link,
                                          &time);
        trace!("perform_request::auth == {:?}", auth);

        // we need to add custom headers. If the caller has passed its collection of
        // headers we will add to his. Otherwise we create one from scratch.
        let mut headers = if let Some(h) = headers {
            h
        } else {
            Headers::new()
        };

        if let Some((_, size)) = request_body {
            headers.set(ContentLength(size));
        }

        headers.set(XMSDate(time));
        headers.set(XMSVersion(AZURE_VERSION.to_owned()));
        headers.set(Authorization(auth));

        trace!("perform_request::headers == {:?}", headers);

        let mut builder = match http_method {
            HTTPMethod::Get => self.hyper_client.get(&url.to_string()),
            HTTPMethod::Put => self.hyper_client.put(&url.to_string()),
            HTTPMethod::Post => self.hyper_client.post(&url.to_string()),
            HTTPMethod::Delete => self.hyper_client.delete(&url.to_string()),
        };

        if let Some((mut rb, size)) = request_body {
            let b = hyper::client::Body::SizedBody(rb, size);
            builder = builder.body(b);
        }

        let res = builder.headers(headers).send()?;

        Ok(res)
    }

    pub fn list_databases(&self) -> Result<Vec<Database>, AzureError> {
        trace!("list_databases called");

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs",
                                           self.authorization_token.account()))?;

        // No specific headers are required, list databases only needs standard headers
        // which will be provied by perform_request
        let mut resp =
            self.perform_request(&url, HTTPMethod::Get, None, ResourceType::Databases, None)?;

        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
        let db: ListDatabasesResponse = serde_json::from_str(&body)?;

        Ok(db.databases)
    }

    pub fn create_database(&self, database_name: &str) -> Result<Database, AzureError> {
        trace!("create_databases called (database_name == {})",
               database_name);

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs",
                                           self.authorization_token.account()))?;

        // No specific headers are required, create databases only needs standard headers
        // which will be provied by perform_request
        // for the body, we will serialize the appropriate structure

        let req = CreateDatabaseRequest { id: database_name };
        let req = serde_json::to_string(&req)?;
        let mut curs = Cursor::new(&req);

        let mut resp = self.perform_request(&url,
                                            HTTPMethod::Post,
                                            Some((&mut curs, req.len() as u64)),
                                            ResourceType::Databases,
                                            None)?;

        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
        let db: Database = serde_json::from_str(&body)?;

        Ok(db)
    }

    pub fn get_database(&self, database_name: &str) -> Result<Database, AzureError> {
        trace!("get_database called (database_name == {})", database_name);

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}",
                                           self.authorization_token.account(),
                                           database_name))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let mut resp =
            self.perform_request(&url, HTTPMethod::Get, None, ResourceType::Databases, None)?;

        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
        let db: Database = serde_json::from_str(&body)?;

        Ok(db)
    }

    pub fn delete_database(&self, database_name: &str) -> Result<(), AzureError> {
        trace!("delete_database called (database_name == {})",
               database_name);

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}",
                                           self.authorization_token.account(),
                                           database_name))?;

        // No specific headers are required, delete database only needs standard headers
        // which will be provied by perform_request
        let mut resp = self.perform_request(&url,
                                            HTTPMethod::Delete,
                                            None,
                                            ResourceType::Databases,
                                            None)?;

        check_status(&mut resp, StatusCode::NoContent)?;

        Ok(())
    }

    pub fn get_collection(&self,
                          database_name: &str,
                          collection_name: &str)
                          -> Result<Collection, AzureError> {
        trace!("get_collection called (database_name == {}, collection_name == {})",
               database_name,
               collection_name);

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}/colls/{}",
                                           self.authorization_token.account(),
                                           database_name,
                                           collection_name))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let mut resp =
            self.perform_request(&url, HTTPMethod::Get, None, ResourceType::Collections, None)?;

        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
        let coll: Collection = serde_json::from_str(&body)?;

        Ok(coll)
    }

    pub fn list_collections(&self, database_name: &str) -> Result<Vec<Collection>, AzureError> {
        trace!("list_collections called");

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}/colls",
                                           self.authorization_token.account(),
                                           database_name))?;

        // No specific headers are required, list collections only needs standard headers
        // which will be provied by perform_request
        let mut resp =
            self.perform_request(&url, HTTPMethod::Get, None, ResourceType::Collections, None)?;

        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
        let colls: ListCollectionsResponse = serde_json::from_str(&body)?;

        Ok(colls.collections)
    }

    pub fn create_collection(&self,
                             database_name: &str,
                             required_throughput: u64,
                             collection: &Collection)
                             -> Result<Collection, AzureError> {
        trace!("create_collection called");

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}/colls",
                                           self.authorization_token.account(),
                                           database_name))?;

        // Headers added as per https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection
        // Standard headers (auth and version) will be provied by perform_request
        let mut headers = Headers::new();
        headers.set(OfferThroughput(required_throughput));

        let collection_serialized = serde_json::to_string(collection)?;

        trace!("collection_serialized == {}", collection_serialized);

        let mut curs = Cursor::new(&collection_serialized);

        let mut resp = self.perform_request(&url,
                                            HTTPMethod::Post,
                                            Some((&mut curs, collection_serialized.len() as u64)),
                                            ResourceType::Collections,
                                            Some(headers))?;

        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
        let coll: Collection = serde_json::from_str(&body)?;

        Ok(coll)
    }

    pub fn delete_collection(&self,
                             database_name: &str,
                             collection_name: &str)
                             -> Result<(), AzureError> {
        trace!("delete_collection called (database_name == {}, collection_name == {}",
               database_name,
               collection_name);

        let url = url::Url::parse(&format!("https://{}.documents.azure.com/dbs/{}/colls/{}",
                                           self.authorization_token.account(),
                                           database_name,
                                           collection_name))?;

        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let mut resp = self.perform_request(&url,
                                            HTTPMethod::Delete,
                                            None,
                                            ResourceType::Collections,
                                            None)?;

        check_status(&mut resp, StatusCode::NoContent)?;

        Ok(())
    }
}


fn generate_authorization(authorization_token: &AuthorizationToken,
                          http_method: HTTPMethod,
                          resource_type: ResourceType,
                          resource_link: &str,
                          time: &str)
                          -> String {
    let string_to_sign = string_to_sign(http_method, resource_type, resource_link, time);
    trace!("generate_authorization::string_to_sign == {:?}",
           string_to_sign);

    let str_unencoded = format!("type={}&ver={}&sig={}",
                                match authorization_token.token_type() {
                                    TokenType::Master => "master",
                                    TokenType::Resource => "resource",
                                },
                                VERSION,
                                encode_str_to_sign(&string_to_sign, authorization_token));

    trace!("generate_authorization::str_unencoded == {:?}",
           str_unencoded);

    utf8_percent_encode(&str_unencoded, COMPLETE_ENCODE_SET).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, authorization_token: &AuthorizationToken) -> String {
    let mut hmac = Hmac::new(Sha256::new(), authorization_token.binary_form());
    hmac.input(str_to_sign.as_bytes());

    base64::encode(hmac.result().code())
}



fn string_to_sign(http_method: HTTPMethod,
                  rt: ResourceType,
                  resource_link: &str,
                  time: &str)
                  -> String {

    // From official docs:
    // StringToSign = Verb.toLowerCase() + "\n" + ResourceType.toLowerCase() + "\n" + ResourceLink + "\n" + Date.toLowerCase() + "\n" + "" + "\n";
    // Notice the empty string at the end so we need to add two carriage returns

    format!("{}\n{}\n{}\n{}\n\n",
            match http_method {
                HTTPMethod::Get => "get",
                HTTPMethod::Put => "put",
                HTTPMethod::Post => "post",
                HTTPMethod::Delete => "delete",
            },
            match rt { 
                ResourceType::Databases => "dbs",
                ResourceType::Collections => "colls",
                ResourceType::Documents => "docs",
            },
            resource_link,
            time.to_lowercase())


}

fn generate_resource_link(u: &url::Url) -> &str {
    static ENDING_STRINGS: &'static [&str] = &["/dbs", "/colls", "/docs"];

    // store the element only if it does not end with dbs, colls or docs
    let p = u.path();

    for str_to_match in ENDING_STRINGS {
        if str_to_match.len() <= p.len() {
            let sm = &p[p.len() - str_to_match.len()..];
            if &sm == str_to_match {
                if p.len() == str_to_match.len() {
                    return "";
                }

                let ret = &p[1..p.len() - str_to_match.len()];
                return ret;
            }
        }
    }

    &p[1..]
}


#[cfg(test)]
mod tests {
    use azure::cosmos::client::*;
    use azure::cosmos::authorization_token;
    use url::Url;

    #[test]
    fn string_to_sign_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(HTTPMethod::Get,
                                 ResourceType::Databases,
                                 "dbs/MyDatabase/colls/MyCollection",
                                 &time);
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
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new("mindflavor", authorization_token::TokenType::Master,
                                                         "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".to_owned()).unwrap();



        let ret = generate_authorization(&authorization_token,
                                         HTTPMethod::Get,
                                         ResourceType::Databases,
                                         "dbs/MyDatabase/colls/MyCollection",
                                         &time);
        assert_eq!(ret,
                   "type%3Dmaster%26ver%3D1.0%26sig%3DQkz%2Fr%2B1N2%2BPEnNijxGbGB%2FADvLsLBQmZ7uBBMuIwf4I%3D");
    }

    #[test]
    fn generate_authorization_01() {
        let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new("mindflavor", authorization_token::TokenType::Master,
                                                         "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".to_owned()).unwrap();

        let ret = generate_authorization(&authorization_token,
                                         HTTPMethod::Get,
                                         ResourceType::Databases,
                                         "dbs/ToDoList",
                                         &time);

        // This is the result shown in the MSDN page. It's clearly wrong :)
        // below is the correct one.
        //assert_eq!(ret,
        //           "type%3dmaster%26ver%3d1.0%26sig%3dc09PEVJrgp2uQRkr934kFbTqhByc7TVr3O");

        assert_eq!(ret,
                   "type%3Dmaster%26ver%3D1.0%26sig%3DKvBM8vONofkv3yKm%2F8zD9MEGlbu6jjHDJBp4E9c2ZZI%3D");
    }

    #[test]
    fn generate_resource_link_00() {
        let u = Url::parse("https://mindflavor.raldld.r4eee.sss/dbs/second").unwrap();
        assert_eq!(generate_resource_link(&u), "dbs/second");
        let u = Url::parse("https://mindflavor.raldld.r4eee.sss/dbs").unwrap();
        assert_eq!(generate_resource_link(&u), "");
        let u = Url::parse("https://mindflavor.raldld.r4eee.sss/colls/second/third").unwrap();
        assert_eq!(generate_resource_link(&u), "colls/second/third");
        let u = Url::parse("https://mindflavor.documents.azure.com/dbs/test_db/colls").unwrap();
        assert_eq!(generate_resource_link(&u), "dbs/test_db");

    }
}
