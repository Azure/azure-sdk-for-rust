use azure::core::{
    errors::{check_status_extract_body, AzureError}, COMPLETE_ENCODE_SET,
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
use hyper::{self, StatusCode};
use ring::{digest::SHA256, hmac};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{rc::Rc, str::FromStr};

use chrono;
use hyper_tls::HttpsConnector;
use native_tls;
use tokio_core;
use url::percent_encoding::utf8_percent_encode;

use futures::future::*;

const AZURE_VERSION: &str = "2017-02-22";
const VERSION: &str = "1.0";
const TIME_FORMAT: &str = "%a, %d %h %Y %T GMT";

pub(crate) mod headers {
    use azure::core::incompletevector::ContinuationToken;
    use azure::cosmos::{ConsistencyLevel, document::IndexingDirective};

    header! { (XMSVersion, "x-ms-version") => Cow[str] }
    header! { (XMSDate, "x-ms-date") => [String] }
    header! { (Authorization, "Authorization") => [String] }
    header! { (OfferThroughput, "x-ms-offer-throughput") => [u64] }
    header! { (DocumentIsUpsert, "x-ms-documentdb-is-upsert") => [bool] }
    header! { (DocumentIndexingDirective, "x-ms-indexing-directive	") => [IndexingDirective] }
    header! { (MaxItemCount, "x-ms-max-item-count") => [u64] }
    header! { (ContinuationTokenHeader, "x-ms-continuation") => [ContinuationToken] }
    header! { (ConsistencyLevelHeader, "x-ms-consistency-level") => [ConsistencyLevel] }
    header! { (SessionTokenHeader, "x-ms-session-token") => [ContinuationToken] }
    header! { (AIM, "A-IM") => Cow[str] }
    header! { (IfNoneMatch, "If-None-Match") => [String] }
    header! { (IfMatch, "If-Match") => [String] }
    header! { (PartitionRangeId, "x-ms-documentdb-partitionkeyrangeid") => [String] }
    header! { (Charge, "x-ms-request-charge") => [f64] }
    header! { (Etag, "etag") => [String] }
    header! { (CosmosDBPartitionKey, "x-ms-documentdb-partitionkey") => [String] }
    header! { (DocumentDBIsQuery, "x-ms-documentdb-isquery") => [bool] }
    header! { (DocumentDBQueryEnableCrossPartition, "x-ms-documentdb-query-enablecrosspartition") => [bool] }
}
use self::headers::*;

#[derive(Clone, Copy)]
enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub struct Client {
    hyper_client: Rc<hyper::Client<HttpsConnector<hyper::client::HttpConnector>>>,
    auth_token: AuthorizationToken,
}

impl Client {
    pub fn new(
        handle: &tokio_core::reactor::Handle,
        auth_token: AuthorizationToken,
    ) -> Result<Client, native_tls::Error> {
        let client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, handle)?)
            .build(handle);

        Ok(Client {
            hyper_client: Rc::new(client),
            auth_token,
        })
    }

    pub fn set_auth_token(&mut self, at: AuthorizationToken) {
        self.auth_token = at;
    }

    fn list_databases_create_request(&self) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs",
            &self.auth_token.account()
        ))?;

        // No specific headers are required, list databases only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Databases,
        );

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
            check_status_extract_body(future_response, StatusCode::Ok).and_then(move |body| {
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
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls",
            self.auth_token.account(),
            database_name
        ))?;

        // No specific headers are required, list collections only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Collections,
        );

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
            check_status_extract_body(future_response, StatusCode::Ok).and_then(move |body| {
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
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs",
            self.auth_token.account()
        ))?;

        let req = CreateDatabaseRequest { id: database_name };
        let req = serde_json::to_string(&req)?;

        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Post,
            Some(&req),
            ResourceType::Databases,
        );

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
            check_status_extract_body(future_response, StatusCode::Created)
                .and_then(move |body| done(serde_json::from_str::<Database>(&body)).from_err())
        })
    }

    #[inline]
    fn get_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}",
            self.auth_token.account(),
            database_name
        ))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Databases,
        );

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
            check_status_extract_body(future_response, StatusCode::Ok)
                .and_then(move |body| done(serde_json::from_str::<Database>(&body)).from_err())
        })
    }

    #[inline]
    fn delete_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}",
            self.auth_token.account(),
            database_name
        ))?;

        // No specific headers are required, delete database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Delete,
            None,
            ResourceType::Databases,
        );

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
            check_status_extract_body(future_response, StatusCode::NoContent).and_then(|_| ok(()))
        })
    }

    #[inline]
    fn get_collection_create_request(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}",
            self.auth_token.account(),
            database_name,
            collection_name
        ))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Collections,
        );

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
            check_status_extract_body(future_response, StatusCode::Ok)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn create_collection_create_request(
        &self,
        database_name: &str,
        required_throughput: u64,
        collection: &Collection,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls",
            self.auth_token.account(),
            database_name
        ))?;

        // Headers added as per
        // https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let mut request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Post,
            Some(&collection_serialized),
            ResourceType::Collections,
        );
        request
            .headers_mut()
            .set(OfferThroughput(required_throughput));

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
            check_status_extract_body(future_response, StatusCode::Created)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn delete_collection_create_request(
        &self,
        database_name: &str,
        collection_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}",
            self.auth_token.account(),
            database_name,
            collection_name
        ))?;

        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Delete,
            None,
            ResourceType::Collections,
        );

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
            check_status_extract_body(future_response, StatusCode::NoContent).and_then(|_| ok(()))
        })
    }

    #[inline]
    fn replace_collection_prepare_request(
        &self,
        database_name: &str,
        collection: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls",
            self.auth_token.account(),
            database_name
        ))?;

        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Put,
            Some(&collection_serialized),
            ResourceType::Collections,
        );

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
            check_status_extract_body(future_response, StatusCode::Created)
                .and_then(move |body| done(serde_json::from_str::<Collection>(&body)).from_err())
        })
    }

    #[inline]
    fn create_document_as_str_create_request<S: AsRef<str>>(
        &self,
        database: &str,
        collection: &str,
        document_str: S,
    ) -> Result<hyper::Request, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.auth_token.account(),
            database,
            collection
        ))?;

        let request = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Post,
            Some(document_str.as_ref()),
            ResourceType::Documents,
        );

        trace!("request prepared");

        Ok(request)
    }

    pub fn create_document_as_str<T, S1, S2, S3>(
        &self,
        database: S1,
        collection: S2,
        document: S3,
    ) -> Result<CreateDocumentRequest, AzureError>
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "create_document_as_str called(database == {}, collection == {}, document = {}",
            database,
            collection,
            document.as_ref()
        );

        let req = self.create_document_as_str_create_request(database, collection, document)?;
        Ok(CreateDocumentRequest::new(self.hyper_client.clone(), req))
    }

    pub fn create_document<T, S1, S2>(
        &self,
        database: &S1,
        collection: &S2,
        document: &T,
    ) -> Result<CreateDocumentRequest, AzureError>
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let db = database.as_ref();
        let coll = collection.as_ref();
        let document_serialized = serde_json::to_string(document)?;
        trace!(
            "create_document_as called(database == {}, collection == {}, document = {}",
            db,
            coll,
            document_serialized
        );
        let req = self.create_document_as_str_create_request(db, coll, &document_serialized)?;
        Ok(CreateDocumentRequest::new(self.hyper_client.clone(), req))
    }

    pub fn delete_document<D: AsRef<str>, C: AsRef<str>, Dc: AsRef<str>>(
        &self,
        database_id: &D,
        collection_id: &C,
        document_id: &Dc,
    ) -> Result<DeleteDocumentRequest, AzureError> {
        trace!(
            "delete_document called (db_id == {}, collection_id == {}, doc_id = {}",
            database_id.as_ref(),
            collection_id.as_ref(),
            document_id.as_ref()
        );

        let uri = format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs/{}",
            self.auth_token.account(),
            database_id.as_ref(),
            collection_id.as_ref(),
            document_id.as_ref()
        ).parse()?;

        let req = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Delete,
            None,
            ResourceType::Documents,
        );
        Ok(DeleteDocumentRequest::new(self.hyper_client.clone(), req))
    }

    pub fn replace_document<D: AsRef<str>, C: AsRef<str>, T: Serialize + DeserializeOwned>(
        &self,
        database_id: D,
        collection_id: C,
        document: &Document<T>,
    ) -> Result<ReplaceDocumentRequest<T>, AzureError> {
        let document_serialized = serde_json::to_string(&document.entity)?;

        trace!(
            "replace_document called(db_id == {}, collection == {}, document == {}",
            database_id.as_ref(),
            collection_id.as_ref(),
            document_serialized,
        );

        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/{}",
            self.auth_token.account(),
            document.document_attributes._self
        ))?;

        let req = prepare_request_with_resource_link(
            &self.auth_token,
            uri,
            &hyper::Method::Put,
            Some(&document_serialized),
            ResourceType::Documents,
            &document.document_attributes.rid.to_lowercase(),
        );

        Ok(ReplaceDocumentRequest::new(self.hyper_client.clone(), req))
    }

    pub fn list_documents<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        database: &S1,
        collection: &S2,
    ) -> Result<ListDocumentsRequest, AzureError> {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "list-_documents called(database == {}, collection == {}",
            database,
            collection
        );

        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.auth_token.account(),
            database,
            collection
        ))?;

        let req = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Documents,
        );

        Ok(ListDocumentsRequest::new(self.hyper_client.clone(), req))
    }

    pub fn get_document<S1, S2, S3>(
        &self,
        database: &S1,
        collection: &S2,
        document_id: &S3,
    ) -> Result<GetDocumentRequest, AzureError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let db = database.as_ref();
        let coll = collection.as_ref();
        let doc_id = document_id.as_ref();

        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs/{}",
            self.auth_token.account(),
            db,
            coll,
            doc_id
        ))?;

        let req = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Get,
            None,
            ResourceType::Documents,
        );

        Ok(GetDocumentRequest::new(self.hyper_client.clone(), req))
    }

    pub fn query_document<'b, S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        database: &S1,
        collection: &S2,
        query: &Query<'b>,
    ) -> Result<QueryDocumentRequest, AzureError> {
        let database = database.as_ref();
        let collection = collection.as_ref();

        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.auth_token.account(),
            database,
            collection,
        ))?;

        let query_json = serde_json::to_string(query)?;

        let req = prepare_request(
            &self.auth_token,
            uri,
            &hyper::Method::Post,
            Some(&query_json),
            ResourceType::Documents,
        );
        Ok(QueryDocumentRequest::new(self.hyper_client.clone(), req))
    }
}

#[inline]
fn prepare_request(
    auth_token: &AuthorizationToken,
    uri: hyper::Uri,
    http_method: &hyper::Method,
    request_body: Option<&str>,
    resource_type: ResourceType,
) -> hyper::client::Request {
    let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

    let auth = {
        let resource_link = generate_resource_link(&uri);
        generate_authorization(auth_token, http_method, resource_type, resource_link, &time)
    };
    prepare_request_with_signature(uri, http_method, request_body, time, auth)
}

#[inline]
fn prepare_request_with_resource_link(
    auth_token: &AuthorizationToken,
    uri: hyper::Uri,
    http_method: &hyper::Method,
    request_body: Option<&str>,
    resource_type: ResourceType,
    resource_link: &str,
) -> hyper::client::Request {
    let time = format!("{}", chrono::Utc::now().format(TIME_FORMAT));

    let sig =
        { generate_authorization(auth_token, http_method, resource_type, resource_link, &time) };
    prepare_request_with_signature(uri, http_method, request_body, time, sig)
}

#[inline]
fn prepare_request_with_signature(
    uri: hyper::Uri,
    http_method: &hyper::Method,
    request_body: Option<&str>,
    time: String,
    signature: String,
) -> hyper::client::Request {
    trace!("prepare_request::auth == {:?}", signature);
    let mut request = hyper::Request::new(http_method.clone(), uri);
    {
        let headers = request.headers_mut();
        headers.set(XMSDate(time));
        headers.set(XMSVersion::new(AZURE_VERSION));
        headers.set(Authorization(signature));
    }
    trace!("prepare_request::headers == {:?}", request.headers());

    if let Some(body) = request_body {
        request
            .headers_mut()
            .set(hyper::header::ContentLength(body.len() as u64));
        request.set_body(body.to_string());
    }

    request
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

fn generate_resource_link(u: &hyper::Uri) -> &str {
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
    use hyper::Uri;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            &hyper::Method::Get,
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
            &hyper::Method::Get,
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
            &hyper::Method::Get,
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
        let u = Uri::from_str("https://mindflavor.raldld.r4eee.sss/dbs/second").unwrap();
        assert_eq!(generate_resource_link(&u), "dbs/second");
        let u = Uri::from_str("https://mindflavor.raldld.r4eee.sss/dbs").unwrap();
        assert_eq!(generate_resource_link(&u), "");
        let u = Uri::from_str("https://mindflavor.raldld.r4eee.sss/colls/second/third").unwrap();
        assert_eq!(generate_resource_link(&u), "colls/second/third");
        let u = Uri::from_str("https://mindflavor.documents.azure.com/dbs/test_db/colls").unwrap();
        assert_eq!(generate_resource_link(&u), "dbs/test_db");
    }
}
