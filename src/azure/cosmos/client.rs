use azure::core::{
    errors::{check_status_extract_body, AzureError},
    COMPLETE_ENCODE_SET,
    util::RequestBuilderExt
};

use super::{
    collection::Collection, database::Database, query::Query,
    request_response::{
        CreateDatabaseRequest, Document, ListCollectionsResponse, ListDatabasesResponse,
    },
    AuthorizationToken, CreateDocumentRequest, DeleteDocumentRequest, GetDocumentRequest,
    ListDocumentsRequest, QueryDocumentRequest, ReplaceDocumentRequest, TokenType,
};

use base64;
use http::request::Builder as RequestBuilder;
use hyper::{self, StatusCode, header::{self, HeaderValue}};
use ring::{digest::SHA256, hmac};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::sync::Arc;

use chrono;
use hyper_tls::HttpsConnector;
use url::percent_encoding::utf8_percent_encode;

use futures::future::*;

const AZURE_VERSION: &str = "2017-02-22";
const VERSION: &str = "1.0";
const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";

pub(crate) mod headers {
    pub const HEADER_VERSION: &str = "x-ms-version"; // Cow[str]
    pub const HEADER_DATE: &str = "x-ms-date"; // [String]
    pub const HEADER_OFFER_THROUGHPUT: &str = "x-ms-offer-throughput"; // [u64]
    pub const HEADER_DOCUMENTDB_IS_UPSERT: &str = "x-ms-documentdb-is-upsert"; // [bool]
    pub const HEADER_INDEXING_DIRECTIVE: &str = "x-ms-indexing-directive"; // [IndexingDirective]
    pub const HEADER_MAX_ITEM_COUNT: &str = "x-ms-max-item-count"; // [u64]
    pub const HEADER_CONTINUATION: &str = "x-ms-continuation"; // [ContinuationToken]
    pub const HEADER_CONSISTENCY_LEVEL: &str = "x-ms-consistency-level"; // [ConsistencyLevel]
    pub const HEADER_SESSION_TOKEN: &str = "x-ms-session-token"; // [ContinuationToken]
    pub const HEADER_A_IM: &str = "A-IM"; // Cow[str]
    pub const HEADER_DOCUMENTDB_PARTITIONRANGEID: &str = "x-ms-documentdb-partitionkeyrangeid"; // [String]
    pub const HEADER_REQUEST_CHARGE: &str = "x-ms-request-charge"; // [f64]
    pub const HEADER_DOCUMENTDB_PARTITIONKEY: &str = "x-ms-documentdb-partitionkey"; // [String]
    pub const HEADER_DOCUMENTDB_ISQUERY: &str = "x-ms-documentdb-isquery"; // [bool]
    pub const HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION: &str = "x-ms-documentdb-query-enablecrosspartition"; // [bool]
}
use self::headers::*;

#[derive(Clone, Copy)]
enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub struct Client {
    hyper_client: Arc<hyper::Client<HttpsConnector<hyper::client::HttpConnector>>>,
    auth_token: AuthorizationToken,
}

impl Client {
    pub fn new(
        auth_token: AuthorizationToken,
    ) -> Result<Client, AzureError> {
        let client = hyper::Client::builder()
            .build(HttpsConnector::new(4)?);

        Ok(Client {
            hyper_client: Arc::new(client),
            auth_token,
        })
    }

    pub fn set_auth_token(&mut self, at: AuthorizationToken) {
        self.auth_token = at;
    }

    fn list_databases_create_request(&self) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required, list databases only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = self.prepare_request(
            "dbs",
            hyper::Method::GET,
            ResourceType::Databases,
        )
        .body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    /// Returns database list associated to the account
    /// specified in the
    ///     `azure_sdk_for_rust::cosmos::auth_token::AuthorizationToken`.
    pub fn list_databases(&self) -> impl Future<Item = Vec<Database>, Error = AzureError> {
        trace!("list_databases called");

        let req = self.list_databases_create_request();

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK).and_then(move |body| {
                done(serde_json::from_str::<ListDatabasesResponse>(&body))
                    .from_err()
                    .and_then(move |response| ok(response.databases))
            })
        })
    }

    #[inline]
    fn list_collections_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required, list collections only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = self.prepare_request(
            &format!("dbs/{}/colls", database_name),
            hyper::Method::GET,
            ResourceType::Collections,
        )
        .body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn list_collections(
        &self,
        database_name: &str,
    ) -> impl Future<Item = Vec<Collection>, Error = AzureError> {
        trace!("list_collections called");

        let req = self.list_collections_create_request(database_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK).and_then(move |body| {
                done(serde_json::from_str::<ListCollectionsResponse>(&body))
                    .from_err()
                    .and_then(|database_response| ok(database_response.collections))
            })
        })
    }

    #[inline]
    fn create_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        let req = CreateDatabaseRequest { id: database_name };
        let req = serde_json::to_string(&req)?;

        let request = self.prepare_request(
            "dbs",
            hyper::Method::POST,
            ResourceType::Databases,
        )
        .body(req.into())?; // todo: set content-length here and elsewhere without builders

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn create_database(
        &self,
        database_name: &str,
    ) -> impl Future<Item = Database, Error = AzureError> {
        trace!(
            "create_databases called (database_name == {})",
            database_name
        );

        let req = self.create_database_create_request(database_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::CREATED)
                .and_then(move |body| done(serde_json::from_str::<Database>(&body)).from_err())
        })
    }

    #[inline]
    fn get_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = self.prepare_request(
            &format!("dbs/{}", database_name),
            hyper::Method::GET,
            ResourceType::Databases,
        ).body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn get_database(
        &self,
        database_name: &str,
    ) -> impl Future<Item = Database, Error = AzureError> {
        trace!("get_database called (database_name == {})", database_name);

        let req = self.get_database_create_request(database_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK)
                .and_then(move |body| done(serde_json::from_str::<Database>(&body)).from_err())
        })
    }

    #[inline]
    fn delete_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required, delete database only needs standard headers
        // which will be provied by perform_request
        let request = self.prepare_request(
            &format!("dbs/{}", database_name),
            hyper::Method::DELETE,
            ResourceType::Databases,
        ).body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn delete_database(
        &self,
        database_name: &str,
    ) -> impl Future<Item = (), Error = AzureError> {
        trace!(
            "delete_database called (database_name == {})",
            database_name
        );

        let req = self.delete_database_create_request(database_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::NO_CONTENT).and_then(|_| ok(()))
        })
    }

    #[inline]
    fn get_collection_create_request(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = self.prepare_request(
            &format!("dbs/{}/colls/{}", database_name, collection_name),
            hyper::Method::GET,
            ResourceType::Collections,
        ).body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn get_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> impl Future<Item = Collection, Error = AzureError> {
        trace!(
            "get_collection called (database_name == {}, collection_name == {})",
            database_name,
            collection_name
        );

        let req = self.get_collection_create_request(database_name, collection_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn create_collection_create_request(
        &self,
        database_name: &str,
        required_throughput: u64,
        collection: &Collection,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // Headers added as per
        // https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let mut request = self.prepare_request(
            &format!("dbs/{}/colls", database_name),
            hyper::Method::POST,
            ResourceType::Collections,
        );
        request.header_formatted(HEADER_OFFER_THROUGHPUT, required_throughput);
        let request = request.body(collection_serialized.into())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn create_collection(
        &self,
        database_name: &str,
        required_throughput: u64,
        collection: &Collection,
    ) -> impl Future<Item = Collection, Error = AzureError> {
        trace!(
            "create_collection(database_name == {:?}, \
             required_throughput == {:?}, collection == {:?} called",
            database_name,
            required_throughput,
            collection
        );

        let req =
            self.create_collection_create_request(database_name, required_throughput, collection);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::CREATED)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn delete_collection_create_request(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let request = self.prepare_request(
            &format!("dbs/{}/colls/{}", database_name, collection_name),
            hyper::Method::DELETE,
            ResourceType::Collections,
        )
        .body(hyper::Body::empty())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn delete_collection(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> impl Future<Item = (), Error = AzureError> {
        trace!(
            "delete_collection called (database_name == {}, collection_name == {}",
            database_name,
            collection_name
        );

        let req = self.delete_collection_create_request(database_name, collection_name);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::NO_CONTENT).and_then(|_| ok(()))
        })
    }

    #[inline]
    fn replace_collection_prepare_request(
        &self,
        database_name: &str,
        collection: &str,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let request = self.prepare_request(
            &format!("dbs/{}/colls", database_name),
            hyper::Method::PUT,
            ResourceType::Collections,
        )
        .body(collection_serialized.into())?;

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn replace_collection(
        &self,
        database_name: &str,
        collection: &str,
    ) -> impl Future<Item = Collection, Error = AzureError> {
        trace!("replace_collection called");

        let req = self.replace_collection_prepare_request(database_name, collection);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::CREATED)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn create_document_as_str_create_request(
        &self,
        database: &str,
        collection: &str,
    ) -> RequestBuilder {
        let uri = format!(
            "dbs/{}/colls/{}/docs",
            database,
            collection
        );

        let request = self.prepare_request(
            &uri,
            hyper::Method::POST,
            ResourceType::Documents,
        );

        trace!("request prepared");

        request
    }

    pub fn create_document_as_str<T, S1, S2, S3>(
        &self,
        database: S1,
        collection: S2,
        document: S3,
    ) -> CreateDocumentRequest
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: Into<String>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();
        let document = document.into();

        trace!(
            "create_document_as_str called(database == {}, collection == {}, document = {}",
            database,
            collection,
            document,
        );

        let req = self.create_document_as_str_create_request(database, collection);
        CreateDocumentRequest::new(self.hyper_client.clone(), req, Ok(document))
    }

    pub fn create_document<T, S1, S2>(
        &self,
        database: S1,
        collection: S2,
        document: &T,
    ) -> CreateDocumentRequest
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let db = database.as_ref();
        let coll = collection.as_ref();
        let document_serialized = serde_json::to_string(document);
        trace!(
            "create_document_as called(database == {}, collection == {}, document = {:?}",
            db,
            coll,
            document_serialized
        );
        let req = self.create_document_as_str_create_request(db, coll);
        CreateDocumentRequest::new(self.hyper_client.clone(), req, document_serialized)
    }

    pub fn delete_document<D: AsRef<str>, C: AsRef<str>, Dc: AsRef<str>>(
        &self,
        database_id: D,
        collection_id: C,
        document_id: Dc,
    ) -> DeleteDocumentRequest {
        trace!(
            "delete_document called (db_id == {}, collection_id == {}, doc_id = {}",
            database_id.as_ref(),
            collection_id.as_ref(),
            document_id.as_ref()
        );

        let uri = format!(
            "dbs/{}/colls/{}/docs/{}",
            database_id.as_ref(),
            collection_id.as_ref(),
            document_id.as_ref()
        );

        let req = self.prepare_request(
            &uri,
            hyper::Method::DELETE,
            ResourceType::Documents,
        );
        DeleteDocumentRequest::new(self.hyper_client.clone(), req)
    }

    pub fn replace_document<D: AsRef<str>, C: AsRef<str>, T: Serialize + DeserializeOwned>(
        &self,
        database_id: D,
        collection_id: C,
        document: &Document<T>,
    ) -> ReplaceDocumentRequest<T> {
        let document_serialized = serde_json::to_string(&document.entity);

        trace!(
            "replace_document called(db_id == {}, collection == {}, document == {:?}",
            database_id.as_ref(),
            collection_id.as_ref(),
            document_serialized,
        );

        let req = self.prepare_request_with_resource_link(
            &document.document_attributes._self,
            hyper::Method::PUT,
            ResourceType::Documents,
            &document.document_attributes.rid.to_lowercase(),
        );

        ReplaceDocumentRequest::new(self.hyper_client.clone(), req, document_serialized)
    }

    pub fn list_documents<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        database: S1,
        collection: S2,
    ) -> ListDocumentsRequest {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "list_documents called(database == {}, collection == {}",
            database,
            collection
        );

        let req = self.prepare_request(
            &format!("dbs/{}/colls/{}/docs", database, collection),
            hyper::Method::GET,
            ResourceType::Documents,
        );

        ListDocumentsRequest::new(self.hyper_client.clone(), req)
    }

    pub fn get_document<S1, S2, S3>(
        &self,
        database: S1,
        collection: S2,
        document_id: S3,
    ) -> GetDocumentRequest
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let db = database.as_ref();
        let coll = collection.as_ref();
        let doc_id = document_id.as_ref();

        let req = self.prepare_request(
            &format!("dbs/{}/colls/{}/docs/{}", db, coll, doc_id),
            hyper::Method::GET,
            ResourceType::Documents,
        );

        GetDocumentRequest::new(self.hyper_client.clone(), req)
    }

    pub fn query_document<'b, S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        database: S1,
        collection: S2,
        query: &Query<'b>,
    ) -> QueryDocumentRequest {
        let database = database.as_ref();
        let collection = collection.as_ref();

        let req = self.prepare_request(
            &format!("dbs/{}/colls/{}/docs", database, collection),
            hyper::Method::POST,
            ResourceType::Documents,
        );
        let query_json = serde_json::to_string(query);
        QueryDocumentRequest::new(self.hyper_client.clone(), req, query_json)
    }

    #[inline]
    fn prepare_request(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        resource_type: ResourceType,
    ) -> RequestBuilder {
        let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

        let auth = {
            let resource_link = generate_resource_link(&uri_path);
            generate_authorization(&self.auth_token, &http_method, resource_type, resource_link, &time)
        };
        self.prepare_request_with_signature(uri_path, http_method, time, auth)
    }

    #[inline]
    fn prepare_request_with_resource_link(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        resource_type: ResourceType,
        resource_link: &str,
    ) -> RequestBuilder {
        let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

        let sig =
            { generate_authorization(&self.auth_token, &http_method, resource_type, resource_link, &time) };
        self.prepare_request_with_signature(uri_path, http_method, time, sig)
    }

    #[inline]
    fn prepare_request_with_signature(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        time: String,
        signature: String,
    ) -> RequestBuilder {
        trace!("prepare_request::auth == {:?}", signature);
        let uri = format!("https://{}.documents.azure.com/{}", self.auth_token.account(), uri_path);
        let mut request = hyper::Request::builder();
        request
            .method(http_method)
            .uri(uri)
            .header(HEADER_DATE, time.as_str())
            .header(HEADER_VERSION, HeaderValue::from_static(AZURE_VERSION))
            .header(header::AUTHORIZATION, signature.as_str());
        request
    }
}

fn generate_authorization(
    auth_token: &AuthorizationToken,
    http_method: &hyper::Method,
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
        match auth_token.token_type() {
            TokenType::Master => "master",
            TokenType::Resource => "resource",
        },
        VERSION,
        encode_str_to_sign(&string_to_sign, auth_token)
    );

    trace!(
        "generate_authorization::str_unencoded == {:?}",
        str_unencoded
    );

    utf8_percent_encode(&str_unencoded, COMPLETE_ENCODE_SET).collect::<String>()
}

fn encode_str_to_sign(str_to_sign: &str, auth_token: &AuthorizationToken) -> String {
    let key = hmac::SigningKey::new(&SHA256, auth_token.key());
    let sig = hmac::sign(&key, str_to_sign.as_bytes());
    base64::encode(sig.as_ref())
}

fn string_to_sign(
    http_method: &hyper::Method,
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
            hyper::Method::GET => "get",
            hyper::Method::PUT => "put",
            hyper::Method::POST => "post",
            hyper::Method::DELETE => "delete",
            hyper::Method::HEAD => "head",
            hyper::Method::TRACE => "trace",
            hyper::Method::OPTIONS => "options",
            hyper::Method::CONNECT => "connect",
            hyper::Method::PATCH => "patch",
            _ => "extension",
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

fn generate_resource_link(u: &str) -> &str {
    static ENDING_STRINGS: &'static [&str] = &["dbs", "colls", "docs"];

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
    use azure::cosmos::client::*;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            &hyper::Method::GET,
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

        let auth_token = AuthorizationToken::new(
            "mindflavor".to_owned(),
            TokenType::Master,
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg==",
        ).unwrap();

        let ret = generate_authorization(
            &auth_token,
            &hyper::Method::GET,
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

        let auth_token = AuthorizationToken::new(
            "mindflavor".to_owned(),
            TokenType::Master,
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL",
        ).unwrap();

        let ret = generate_authorization(
            &auth_token,
            &hyper::Method::GET,
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
        assert_eq!(generate_resource_link("colls/second/third"), "colls/second/third");
        assert_eq!(generate_resource_link("dbs/test_db/colls"), "dbs/test_db");
    }
}
