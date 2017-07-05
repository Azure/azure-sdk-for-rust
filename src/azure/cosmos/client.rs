use azure::cosmos::authorization_token::{TokenType, AuthorizationToken};

use azure::cosmos::database::Database;
use azure::cosmos::collection::Collection;
use azure::cosmos::document::{IndexingDirective, DocumentAttributes};

use azure::core::errors::{AzureError, check_status_extract_body,
                          check_status_extract_headers_and_body};

use azure::cosmos::request_response::{ListDatabasesResponse, CreateDatabaseRequest,
                                      ListCollectionsResponse, ListDocumentsResponseAttributes,
                                      ListDocumentsResponseEntities, ListDocumentsResponse,
                                      ListDocumentsResponseEntry};
use azure::core::COMPLETE_ENCODE_SET;

use azure::cosmos::ConsistencyLevel;
use azure::cosmos::list_documents::{ListDocumentsOptions, ListDocumentsResponseAdditionalHeaders};
use azure::core::incompletevector::ContinuationToken;

use std::str::FromStr;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

use base64;
use hyper;
use serde_json;
use hyper::header::{ContentLength, Headers};
use hyper::StatusCode;

use chrono;

use url::percent_encoding::utf8_percent_encode;

use tokio_core;
use hyper_tls;
use native_tls;

use futures::future::{Future, ok, done};

const AZURE_VERSION: &'static str = "2017-02-22";
const VERSION: &'static str = "1.0";
const TIME_FORMAT: &'static str = "%a, %d %h %Y %T GMT";

header! { (XMSVersion, "x-ms-version") => [String] }
header! { (XMSDate, "x-ms-date") => [String] }
header! { (Authorization, "Authorization") => [String] }
header! { (OfferThroughput, "x-ms-offer-throughput") => [u64] }
header! { (DocumentIsUpsert, "x-ms-documentdb-is-upsert") => [bool] }
header! { (DocumentIndexingDirective, "x-ms-indexing-directive	") => [IndexingDirective] }
header! { (MaxItemCount, "x-ms-max-item-count") => [u64] }
header! { (ContinuationTokenHeader, "x-ms-continuation") => [ContinuationToken] }
header! { (ConsistencyLevelHeader, "x-ms-consistency-level") => [ConsistencyLevel] }
header! { (SessionTokenHeader, "x-ms-session-token") => [ContinuationToken] }
header! { (AIM, "A-IM") => [String] }
header! { (IfNoneMatch, "If-None-Match") => [String] }
header! { (PartitionRangeId, "x-ms-documentdb-partitionkeyrangeid") => [String] }
header! { (Charge, "x-ms-request-charge") => [u64] }
header! { (Etag, "etag") => [String] }

#[derive(Clone, Copy)]
pub enum ResourceType {
    Databases,
    Collections,
    Documents,
}

pub struct Client {
    hyper_client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    authorization_token: AuthorizationToken,
}

impl<'a> Client {
    pub fn new(
        handle: &tokio_core::reactor::Handle,
        authorization_token: AuthorizationToken,
    ) -> Result<Client, native_tls::Error> {

        let client = hyper::Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, handle)?)
            .build(handle);

        Ok(Client {
            hyper_client: client,
            authorization_token: authorization_token,
        })
    }

    pub fn set_authorization_token(&mut self, at: AuthorizationToken) {
        self.authorization_token = at;
    }

    fn list_databases_create_request(&self) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs",
            &self.authorization_token.account()
        ))?;

        // No specific headers are required, list databases only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Get,
            None,
            ResourceType::Databases,
            |_| {},
        );

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

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
            self.authorization_token.account(),
            database_name
        ))?;

        // No specific headers are required, list collections only needs standard headers
        // which will be provied by perform_request. This is handled by passing an
        // empty closure.
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Get,
            None,
            ResourceType::Collections,
            |_| {},
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
            self.authorization_token.account()
        ))?;

        let req = CreateDatabaseRequest { id: database_name };
        let req = serde_json::to_string(&req)?;

        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Post,
            Some(&req),
            ResourceType::Databases,
            |_| {},
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
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<Database>(&body)).from_err()
            })
        })
    }

    #[inline]
    fn get_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}",
            self.authorization_token.account(),
            database_name
        ))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Get,
            None,
            ResourceType::Databases,
            |_| {},
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
            check_status_extract_body(future_response, StatusCode::Ok).and_then(move |body| {
                done(serde_json::from_str::<Database>(&body)).from_err()
            })
        })
    }

    #[inline]
    fn delete_database_create_request(
        &self,
        database_name: &str,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}",
            self.authorization_token.account(),
            database_name
        ))?;

        // No specific headers are required, delete database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Delete,
            None,
            ResourceType::Databases,
            |_| {},
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
            self.authorization_token.account(),
            database_name,
            collection_name
        ))?;

        // No specific headers are required, get database only needs standard headers
        // which will be provied by perform_request
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Get,
            None,
            ResourceType::Collections,
            |_| {},
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
            check_status_extract_body(future_response, StatusCode::Ok).and_then(move |body| {
                done(serde_json::from_str::<Collection>(&body)).from_err()
            })
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
            self.authorization_token.account(),
            database_name
        ))?;

        // Headers added as per
        // https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Post,
            Some(&collection_serialized),
            ResourceType::Collections,
            |ref mut headers| {
                headers.set(OfferThroughput(required_throughput));
            }
            );

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
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<Collection>(&body)).from_err()
            })
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
            self.authorization_token.account(),
            database_name,
            collection_name
        ))?;

        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Delete,
            None,
            ResourceType::Collections,
            |_| {}
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
            self.authorization_token.account(),
            database_name
        ))?;

        // No specific headers are required.
        // Standard headers (auth and version) will be provied by perform_request
        let collection_serialized = serde_json::to_string(collection)?;
        trace!("collection_serialized == {}", collection_serialized);

        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Put,
            Some(&collection_serialized),
            ResourceType::Collections,
            |_| {},
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
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<Collection>(&body)).from_err()
            })
        })
    }

    #[inline]
    fn create_document_as_str_create_request<S>(
        &self,
        database: &str,
        collection: &str,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        document_str: S,
    ) -> Result<hyper::client::FutureResponse, AzureError>
    where
        S: AsRef<str>,
    {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.authorization_token.account(),
            database,
            collection
        ))?;

        // Standard headers (auth and version) will be provied by perform_request
        // Optional headers as per
        // https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-document
        let request = prepare_request(
                &self.authorization_token,
                uri,
                hyper::Method::Post,
                Some(document_str.as_ref()),
                ResourceType::Documents,
                |ref mut headers| {
                   headers.set(DocumentIsUpsert(is_upsert));

                    if let Some(id) = indexing_directive {
                        headers.set(DocumentIndexingDirective(id));
                    }
                });

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    #[inline]
    fn create_document_as_entity_create_request<T>(
        &self,
        database: &str,
        collection: &str,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        document: &T,
    ) -> Result<hyper::client::FutureResponse, AzureError>
    where
        T: Serialize,
    {
        let document_serialized = serde_json::to_string(document)?;
        trace!("document_serialized == {}", document_serialized);

        self.create_document_as_str_create_request(
            database,
            collection,
            is_upsert,
            indexing_directive,
            &document_serialized,
        )
    }

    pub fn create_document_as_str<T, S>(
        &self,
        database: S,
        collection: S,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        document_str: S,
    ) -> impl Future<Item = DocumentAttributes, Error = AzureError>
    where
        T: Serialize,
        S: AsRef<str>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "create_document_as_str called(database == {}, collection == {}, is_upsert == {}",
            database,
            collection,
            is_upsert
        );

        let req = self.create_document_as_str_create_request(
            database.as_ref(),
            collection.as_ref(),
            is_upsert,
            indexing_directive,
            document_str,
        );

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<DocumentAttributes>(&body)).from_err()
            })
        })
    }

    pub fn create_document_as_entity<T, S>(
        &self,
        database: S,
        collection: S,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        document: &T,
    ) -> impl Future<Item = DocumentAttributes, Error = AzureError>
    where
        T: Serialize,
        S: AsRef<str>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "create_document called(database == {}, collection == {}, is_upsert == {}",
            database,
            collection,
            is_upsert
        );

        let req = self.create_document_as_entity_create_request(
            database,
            collection,
            is_upsert,
            indexing_directive,
            document,
        );

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<DocumentAttributes>(&body)).from_err()
            })
        })
    }

    #[inline]
    fn list_documents_create_request(
        &self,
        database: &str,
        collection: &str,
        ldo: &ListDocumentsOptions,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.authorization_token.account(),
            database,
            collection
        ))?;

        let request = prepare_request(
                &self.authorization_token,
                uri,
                hyper::Method::Get,
                None,
                ResourceType::Documents,
                |ref mut headers| {
                    if let Some(val) = ldo.max_item_count {
                        headers.set(MaxItemCount(val));
                    }
                    if let Some(val) = ldo.continuation_token {
                        headers.set(ContinuationTokenHeader(val.to_owned()));
                    }
                    if let Some(val) = ldo.consistency_level_override {
                        headers.set(ConsistencyLevelHeader(val));
                    }
                    if let Some(val) = ldo.session_token {
                        headers.set(SessionTokenHeader(val.to_owned()));
                    }
                    if ldo.incremental_feed {
                        headers.set(AIM("Incremental feed".to_owned()));
                    }
                    if let Some(val) = ldo.if_none_match {
                        headers.set(IfNoneMatch(val.to_owned()));
                    }
                     if let Some(val) = ldo.partition_range_id {
                        headers.set(PartitionRangeId(val.to_owned()));
                    }
                 });

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }


    pub fn list_documents<'b, S, T>(
        &self,
        database: S,
        collection: S,
        ldo: &ListDocumentsOptions,
    ) -> impl Future<
        Item = (
            ListDocumentsResponse<T>,
            ListDocumentsResponseAdditionalHeaders,
        ),
        Error = AzureError,
    >
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "list-_documents called(database == {}, collection == {}, ldo == {:?}",
            database,
            collection,
            ldo
        );

        let req = self.list_documents_create_request(database, collection, ldo);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_headers_and_body(future_response, StatusCode::Ok)
                .and_then(move |(headers, whole_body)| {
                    debug!("headers == {:?}", headers);

                    let ado = ListDocumentsResponseAdditionalHeaders {
                        // This match just tries to extract the info and convert it
                        // into the correct type. It is complicated because headers
                        // can be missing and also because headers.get<T> will return
                        // a T reference (&T) so we need to cast it into the
                        // correct type and clone it (in this case into a &str that will
                        // become a String using to_owned())
                        continuation_token: match headers.get::<ContinuationTokenHeader>() {
                            Some(s) => Some((s as &str).to_owned()),
                            None => None,
                        },
                        // Here we assume the Charge header to always be present.
                        // If problems arise we
                        // will change the field to be Option(al).
                        charge: *(headers.get::<Charge>().unwrap() as &u64),
                        etag: match headers.get::<Etag>() {
                            Some(s) => Some((s as &str).to_owned()),
                            None => None,
                        },
                    };
                    debug!("ado == {:?}", ado);
                    done(list_documents_extract_result::<T>(&whole_body))
                        .and_then(move |body| ok((body, ado)))
                })
        })
    }
}

fn list_documents_extract_result<'a, T>(
    v_body: &[u8],
) -> Result<ListDocumentsResponse<T>, AzureError>
where
    T: DeserializeOwned,
{
    // we will proceed in three steps:
    // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
    // 2- Deserialize the result a type T. The extra fields will be ignored.
    // 3- Zip 1 and 2 in the resulting structure.
    // There is a lot of data movement here, let's hope the compiler is smarter than me :)
    let document_attributes = serde_json::from_slice::<ListDocumentsResponseAttributes>(v_body)?;
    let entries = serde_json::from_slice::<ListDocumentsResponseEntities<T>>(v_body)?;

    let mut v = Vec::with_capacity(document_attributes.documents.len());
    for (da, e) in document_attributes
        .documents
        .into_iter()
        .zip(entries.entities.into_iter())
    {
        v.push(ListDocumentsResponseEntry {
            document_attributes: da,
            entity: e,
        });
    }

    Ok(ListDocumentsResponse {
        rid: document_attributes.rid,
        entries: v,
    })
}

#[inline]
fn prepare_request<F>(
    authorization_token: &AuthorizationToken,
    uri: hyper::Uri,
    http_method: hyper::Method,
    request_body: Option<&str>,
    resource_type: ResourceType,
    headers_func: F,
) -> hyper::client::Request
where
    F: FnOnce(&mut Headers),
{
    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format(TIME_FORMAT));

    // we surround this two statements with a scope so the borrow
    // on uri owned by generate_resource_link is released
    // as soon as generate_authorization ends. This is needed
    // because hyper::Request::new takes ownership of uri. And
    // the borrow checked won't allow ownership move of a borrowed
    // item. This way we save a useless clone.
    let auth = {
        let resource_link = generate_resource_link(&uri);

        generate_authorization(
            authorization_token,
            http_method.clone(),
            resource_type,
            resource_link,
            &time,
        )
    };
    trace!("prepare_request::auth == {:?}", auth);
    let mut request = hyper::Request::new(http_method, uri);

    // This will give the caller the ability to add custom headers.
    // The closure is needed to because request.headers_mut().set_raw(...) requires
    // a Cow with 'static lifetime...
    headers_func(request.headers_mut());

    request.headers_mut().set(XMSDate(time));
    request
        .headers_mut()
        .set(XMSVersion(AZURE_VERSION.to_owned()));
    request.headers_mut().set(Authorization(auth));

    trace!("prepare_request::headers == {:?}", request.headers());

    if let Some(body) = request_body {
        request.headers_mut().set(ContentLength(body.len() as u64));
        request.set_body(body.to_string());
    }

    request
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
    // StringToSign =
    //      Verb.toLowerCase() + "\n" +
    //      ResourceType.toLowerCase() + "\n" +
    //      ResourceLink + "\n" +
    //      Date.toLowerCase() + "\n" +
    //      "" + "\n";
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
    use azure::cosmos::authorization_token;
    use hyper::Uri;

    #[test]
    fn string_to_sign_00() {
        let time = chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00")
            .unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let ret = string_to_sign(
            hyper::Method::Get,
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
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token =
            authorization_token::AuthorizationToken::new(
                "mindflavor".to_owned(),
                authorization_token::TokenType::Master,
                "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg=="
                    .to_owned())
            .unwrap();

        let ret = generate_authorization(
            &authorization_token,
            hyper::Method::Get,
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
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token = authorization_token::AuthorizationToken::new(
            "mindflavor".to_owned(),
            authorization_token::TokenType::Master,
            "dsZQi3KtZmCv1ljt3VNWNm7sQUF1y5rJfC6kv5JiwvW0EndXdDku/dkKBp8/ufDToSxL".to_owned(),
        ).unwrap();

        let ret = generate_authorization(
            &authorization_token,
            hyper::Method::Get,
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
