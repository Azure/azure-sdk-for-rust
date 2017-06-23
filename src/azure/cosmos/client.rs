use azure::cosmos::authorization_token::{TokenType, AuthorizationToken};
use azure::core::HTTPMethod;

use azure::cosmos::database::Database;
use azure::cosmos::collection::Collection;
use azure::cosmos::document::{IndexingDirective, DocumentAttributes};

use azure::core::errors::{AzureError, check_status_extract_body, extract_status_and_body};

use azure::cosmos::request_response::{ListDatabasesResponse, CreateDatabaseRequest,
                                      ListCollectionsResponse};
use std::str::FromStr;

use serde::Serialize;

use std::io::{Read, Cursor};

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

use base64;
use hyper;
use serde_json;
use hyper::header::{ContentLength, Headers};
use hyper::StatusCode;

use chrono;

use url::percent_encoding;
use url::percent_encoding::utf8_percent_encode;

use tokio_core;
use hyper_tls;
use native_tls;

use futures::future::{Future, ok, err, done};

const AZURE_VERSION: &'static str = "2017-02-22";
const VERSION: &'static str = "1.0";
const TIME_FORMAT: &'static str = "%a, %d %h %Y %T GMT";

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }
header! { (OfferThroughput, "x-ms-offer-throughput") => [u64] }
header! { (DocumentIsUpsert, "x-ms-documentdb-is-upsert") => [bool] }
header! { (DocumentIndexingDirective, "x-ms-indexing-directive	") => [IndexingDirective] }

define_encode_set! {
    pub COMPLETE_ENCODE_SET = [percent_encoding::USERINFO_ENCODE_SET] | {
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
    hyper_client: hyper::Client<hyper::client::HttpConnector>,
    authorization_token: &'a AuthorizationToken<'a>,
}

impl<'a> Client<'a> {
    pub fn new(
        handle: &tokio_core::reactor::Handle,
        authorization_token: &'a AuthorizationToken<'a>,
    ) -> Result<Client<'a>, native_tls::Error> {

        let client = hyper::Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, handle)?)
            .build(handle);

        let client = hyper::Client::new(handle);

        Ok(Client {
            hyper_client: client,
            authorization_token: authorization_token,
        })
    }

    pub fn set_authorization_token(&mut self, at: &'a AuthorizationToken<'a>) {
        self.authorization_token = at;
    }

    #[inline]
    fn prepare_request(
        &self,
        uri: hyper::Uri,
        http_method: hyper::Method,
        request_body: Option<&str>,
        resource_type: ResourceType,
        headers: Option<Headers>,
    ) -> hyper::client::FutureResponse {
        let dt = chrono::UTC::now();
        let time = format!("{}", dt.format(TIME_FORMAT));

        let resource_link = generate_resource_link(&uri);

        let auth = generate_authorization(
            self.authorization_token,
            http_method.clone(),
            resource_type,
            resource_link,
            &time,
        );
        trace!("prepare_request::auth == {:?}", auth);

        let mut request = hyper::Request::new(http_method, uri);

        // we need to add custom headers. If the caller has passed its collection of
        // headers we will import them.
        if let Some(hs) = headers {
            for h in hs.iter() {
                request.headers_mut().set_raw(h.name(), h.value_string());
            }
        }

        request.headers_mut().set(XMSDate(time));
        request.headers_mut().set(
            XMSVersion(AZURE_VERSION.to_owned()),
        );
        request.headers_mut().set(Authorization(auth));

        trace!("prepare_request::headers == {:?}", request.headers());

        if let Some(body) = request_body {
            request.headers_mut().set(ContentLength(body.len() as u64));
            request.set_body(body.to_string());
        }

        self.hyper_client.request(request)
    }

    pub fn list_databases(&'a self) -> impl Future<Item = Vec<Database>, Error = AzureError> {
        trace!("list_databases called");

        done(hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs",
            self.authorization_token.account()
        ))).from_err()
            .and_then(move |uri| {
                // No specific headers are required, list databases only needs standard headers
                // which will be provied by perform_request
                let future_request = self.prepare_request(
                    uri,
                    hyper::Method::Get,
                    None,
                    ResourceType::Databases,
                    None,
                );
                check_status_extract_body(future_request, StatusCode::Ok).and_then(|body| {
                    match serde_json::from_str::<ListDatabasesResponse>(&body) {
                        Ok(r) => ok(r.databases),
                        Err(error) => err(error.into()),
                    }
                })
            })
    }

    //    pub fn create_database(&self, database_name: &str) -> Result<Database, AzureError> {
    //        trace!(
    //            "create_databases called (database_name == {})",
    //            database_name
    //        );
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs",
    //            self.authorization_token.account()
    //        ))?;
    //
    //        // No specific headers are required, create databases only needs standard headers
    //        // which will be provied by perform_request
    //        // for the body, we will serialize the appropriate structure
    //
    //        let req = CreateDatabaseRequest { id: database_name };
    //        let req = serde_json::to_string(&req)?;
    //
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Post,
    //            Some(&req),
    //            ResourceType::Databases,
    //            None,
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
    //        let db: Database = serde_json::from_str(&body)?;
    //
    //        Ok(db)
    //    }
    //
    //    pub fn get_database(&self, database_name: &str) -> Result<Database, AzureError> {
    //        trace!("get_database called (database_name == {})", database_name);
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}",
    //            self.authorization_token.account(),
    //            database_name
    //        ))?;
    //
    //        // No specific headers are required, get database only needs standard headers
    //        // which will be provied by perform_request
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Get,
    //            None,
    //            ResourceType::Databases,
    //            None,
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
    //        let db: Database = serde_json::from_str(&body)?;
    //
    //        Ok(db)
    //    }
    //
    //    pub fn delete_database(&self, database_name: &str) -> Result<(), AzureError> {
    //        trace!(
    //            "delete_database called (database_name == {})",
    //            database_name
    //        );
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}",
    //            self.authorization_token.account(),
    //            database_name
    //        ))?;
    //
    //        // No specific headers are required, delete database only needs standard headers
    //        // which will be provied by perform_request
    //        let future = self.perform_request(
    //            uri,
    //            hyper::Method::Delete,
    //            None,
    //            ResourceType::Databases,
    //            None,
    //        )?;
    //
    //        check_status(future, StatusCode::NoContent)?;
    //
    //        Ok(())
    //    }
    //
    //    pub fn get_collection(
    //        &self,
    //        database_name: &str,
    //        collection_name: &str,
    //    ) -> Result<Collection, AzureError> {
    //        trace!(
    //            "get_collection called (database_name == {}, collection_name == {})",
    //            database_name,
    //            collection_name
    //        );
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls/{}",
    //            self.authorization_token.account(),
    //            database_name,
    //            collection_name
    //        ))?;
    //
    //        // No specific headers are required, get database only needs standard headers
    //        // which will be provied by perform_request
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Get,
    //            None,
    //            ResourceType::Collections,
    //            None,
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
    //        let coll: Collection = serde_json::from_str(&body)?;
    //
    //        Ok(coll)
    //    }
    //
    //    pub fn list_collections(&self, database_name: &str) -> Result<Vec<Collection>, AzureError> {
    //        trace!("list_collections called");
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls",
    //            self.authorization_token.account(),
    //            database_name
    //        ))?;
    //
    //        // No specific headers are required, list collections only needs standard headers
    //        // which will be provied by perform_request
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Get,
    //            None,
    //            ResourceType::Collections,
    //            None,
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Ok)?;
    //        let colls: ListCollectionsResponse = serde_json::from_str(&body)?;
    //
    //        Ok(colls.collections)
    //    }
    //
    //    pub fn create_collection(
    //        &self,
    //        database_name: &str,
    //        required_throughput: u64,
    //        collection: &Collection,
    //    ) -> Result<Collection, AzureError> {
    //        trace!("create_collection called");
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls",
    //            self.authorization_token.account(),
    //            database_name
    //        ))?;
    //
    //        // Headers added as per https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection
    //        // Standard headers (auth and version) will be provied by perform_request
    //        let mut headers = Headers::new();
    //        headers.set(OfferThroughput(required_throughput));
    //
    //        let collection_serialized = serde_json::to_string(collection)?;
    //
    //        trace!("collection_serialized == {}", collection_serialized);
    //
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Post,
    //            Some(&collection_serialized),
    //            ResourceType::Collections,
    //            Some(headers),
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
    //        let coll: Collection = serde_json::from_str(&body)?;
    //
    //        Ok(coll)
    //    }
    //
    //    pub fn delete_collection(
    //        &self,
    //        database_name: &str,
    //        collection_name: &str,
    //    ) -> Result<(), AzureError> {
    //        trace!(
    //            "delete_collection called (database_name == {}, collection_name == {}",
    //            database_name,
    //            collection_name
    //        );
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls/{}",
    //            self.authorization_token.account(),
    //            database_name,
    //            collection_name
    //        ))?;
    //
    //        // No specific headers are required.
    //        // Standard headers (auth and version) will be provied by perform_request
    //        let future = self.perform_request(
    //            uri,
    //            hyper::Method::Delete,
    //            None,
    //            ResourceType::Collections,
    //            None,
    //        )?;
    //
    //        check_status(future, StatusCode::NoContent)?;
    //
    //        Ok(())
    //    }
    //
    //    pub fn replace_collection(
    //        &self,
    //        database_name: &str,
    //        collection: &str,
    //    ) -> Result<Collection, AzureError> {
    //        trace!("replace_collection called");
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls",
    //            self.authorization_token.account(),
    //            database_name
    //        ))?;
    //
    //        // No specific headers are required.
    //        // Standard headers (auth and version) will be provied by perform_request
    //        let collection_serialized = serde_json::to_string(collection)?;
    //
    //        trace!("collection_serialized == {}", collection_serialized);
    //
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Put,
    //            Some(&collection_serialized),
    //            ResourceType::Collections,
    //            None,
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
    //        let coll: Collection = serde_json::from_str(&body)?;
    //
    //        Ok(coll)
    //    }
    //
    //    pub fn create_document<T>(
    //        &self,
    //        database: &str,
    //        collection: &str,
    //        is_upsert: bool,
    //        indexing_directive: Option<IndexingDirective>,
    //        document: &T,
    //    ) -> Result<DocumentAttributes, AzureError>
    //    where
    //        T: Serialize,
    //    {
    //        trace!(
    //            "create_document called(database == {}, collection == {}, is_upsert == {}",
    //            database,
    //            collection,
    //            is_upsert
    //        );
    //
    //        let uri = hyper::Uri::from_str(&format!(
    //            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
    //            self.authorization_token.account(),
    //            database,
    //            collection
    //        ))?;
    //
    //        // Standard headers (auth and version) will be provied by perform_request
    //        // Optional headers as per https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-document
    //        let mut headers = Headers::new();
    //        headers.set(DocumentIsUpsert(is_upsert));
    //        if let Some(id) = indexing_directive {
    //            headers.set(DocumentIndexingDirective(id));
    //        }
    //
    //        let document_serialized = serde_json::to_string(document)?;
    //        trace!("document_serialized == {}", document_serialized);
    //
    //        let mut resp = self.perform_request(
    //            uri,
    //            hyper::Method::Post,
    //            Some(&document_serialized),
    //            ResourceType::Documents,
    //            Some(headers),
    //        )?;
    //
    //        let body = check_status_extract_body(&mut resp, StatusCode::Created)?;
    //        let document_attributes: DocumentAttributes = serde_json::from_str(&body)?;
    //
    //        Ok(document_attributes)
    //    }
}


fn generate_authorization(
    authorization_token: &AuthorizationToken,
    http_method: hyper::Method,
    resource_type: ResourceType,
    resource_link: &str,
    time: &str,
) -> String {
    let string_to_sign = string_to_sign(http_method, resource_type, resource_link, time);
    trace!(
        "generate_authorization::string_to_sign == {:?}",
        string_to_sign
    );

    let str_unencoded = format!(
        "type={}&ver={}&sig={}",
        match authorization_token.token_type() {
            TokenType::Master => "master",
            TokenType::Resource => "resource",
        },
        VERSION,
        encode_str_to_sign(&string_to_sign, authorization_token)
    );

    trace!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    utf8_percent_encode(&str_unencoded, COMPLETE_ENCODE_SET).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, authorization_token: &AuthorizationToken) -> String {
    let mut hmac = Hmac::new(Sha256::new(), authorization_token.binary_form());
    hmac.input(str_to_sign.as_bytes());

    base64::encode(hmac.result().code())
}



fn string_to_sign(
    http_method: hyper::Method,
    rt: ResourceType,
    resource_link: &str,
    time: &str,
) -> String {

    // From official docs:
    // StringToSign = Verb.toLowerCase() + "\n" + ResourceType.toLowerCase() + "\n" + ResourceLink + "\n" + Date.toLowerCase() + "\n" + "" + "\n";
    // Notice the empty string at the end so we need to add two carriage returns

    format!(
        "{}\n{}\n{}\n{}\n\n",
        match http_method {
            hyper::Method::Get => "get",
            hyper::Method::Put => "put",
            hyper::Method::Post => "post",
            hyper::Method::Delete => "delete",
            hyper::Method::Head => "head",
            hyper::Method::Trace => "trace",
            hyper::Method::Options => "options",
            hyper::Method::Connect => "connect",
            hyper::Method::Patch => "patch",
            hyper::Method::Extension(_) => "extension",
        },
        match rt { 
            ResourceType::Databases => "dbs",
            ResourceType::Collections => "colls",
            ResourceType::Documents => "docs",
        },
        resource_link,
        time.to_lowercase()
    )


}

fn generate_resource_link<'a>(u: &'a hyper::Uri) -> &'a str {
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
    use uri::Url;

    #[test]
    fn string_to_sign_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            HTTPMethod::Get,
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
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new("mindflavor", authorization_token::TokenType::Master,
                                                         "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==".to_owned()).unwrap();



        let ret = generate_authorization(
            &authorization_token,
            HTTPMethod::Get,
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
        let time = chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::UTC);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token = authorization_token::AuthorizationToken::new(
            "mindflavor",
            authorization_token::TokenType::Master,
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL"
                .to_owned(),
        ).unwrap();

        let ret = generate_authorization(
            &authorization_token,
            HTTPMethod::Get,
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
