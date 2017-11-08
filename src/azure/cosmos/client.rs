use azure::cosmos::authorization_token::{AuthorizationToken, TokenType};

use azure::cosmos::database::Database;
use azure::cosmos::collection::Collection;
use azure::cosmos::document::{DocumentAttributes, IndexingDirective};

use azure::core::errors::{check_status_extract_body, check_status_extract_headers_and_body,
                          extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};

use azure::cosmos::request_response::{CreateDatabaseRequest, Document,
                                      GetDocumentAdditionalHeaders, GetDocumentResponse,
                                      ListCollectionsResponse, ListDatabasesResponse,
                                      ListDocumentsResponse,
                                      ListDocumentsResponseAdditionalHeaders,
                                      ListDocumentsResponseAttributes,
                                      ListDocumentsResponseEntities, QueryDocumentResponse,
                                      QueryDocumentResponseAdditonalHeaders, QueryResponseMeta,
                                      QueryResult};
use azure::core::COMPLETE_ENCODE_SET;

use azure::cosmos::ConsistencyLevel;
use azure::cosmos::list_documents::ListDocumentsOptions;
use azure::cosmos::get_document::GetDocumentOptions;
use azure::cosmos::query_document::QueryDocumentOptions;
use azure::core::incompletevector::ContinuationToken;
use azure::cosmos::query::Query;


use std::str::{FromStr, from_utf8};

use serde::Serialize;
use serde_json::Value;
use serde_json::map::Map;
use serde::de::DeserializeOwned;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

use base64;
use hyper;
use serde_json;
use hyper::header::{ContentLength, ContentType, Headers};
use hyper::StatusCode;

use chrono;
use mime::Mime;

use url::percent_encoding::utf8_percent_encode;

use tokio_core;
use hyper_tls;
use native_tls;

use futures::future::*;

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
header! { (Charge, "x-ms-request-charge") => [f64] }
header! { (Etag, "etag") => [String] }
header! { (CosmosDBPartitionKey, "x-ms-documentdb-partitionkey") => [String] }
header! { (DocumentDBIsQuery, "x-ms-documentdb-isquery") => [bool] }
header! { (DocumentDBQueryEnableCrossPartition,
"x-ms-documentdb-query-enablecrosspartition") => [bool] }

const AZURE_KEYS: [&'static str; 5] = ["_attachments", "_etag", "_rid", "_self", "_ts"];


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

fn serialize_partition_key(
    partition_key: &Option<Vec<&str>>,
) -> Result<Option<String>, AzureError> {
    match partition_key {
        // the partition key should be a json formatted string list
        &Some(ref val) => Ok(Some(serde_json::to_string(val)?)),
        &None => Ok(None),
    }
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

    /// Returns database list associated to the account
    /// specified in the
    ///     `azure_sdk_for_rust::azure::cosmos::authorization_token::AuthorizationToken`.
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
            },
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
            |_| {},
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
        partition_key: &Option<Vec<&str>>,
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

        let serialized_partition_key = serialize_partition_key(&partition_key)?;

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

                if let Some(ref val) = serialized_partition_key {
                    headers.set(CosmosDBPartitionKey(val.to_owned()));
                }
            },
        );

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
        partition_key: &Option<Vec<&str>>,
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
            partition_key,
            &document_serialized,
        )
    }

    pub fn create_document_as_str<T, S1, S2, S3>(
        &self,
        database: S1,
        collection: S2,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        partition_key: &Option<Vec<&str>>,
        document_str: S3,
    ) -> impl Future<Item = DocumentAttributes, Error = AzureError>
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
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
            partition_key,
            document_str,
        );

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
                done(serde_json::from_str::<DocumentAttributes>(&body)).from_err()
            })
        })
    }

    pub fn create_document_as_entity<T, S1, S2>(
        &self,
        database: S1,
        collection: S2,
        is_upsert: bool,
        indexing_directive: Option<IndexingDirective>,
        partition_key: &Option<Vec<&str>>,
        document: &T,
    ) -> impl Future<Item = DocumentAttributes, Error = AzureError>
    where
        T: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "create_document_as_entity called(database == {}, collection == {}, is_upsert == {}",
            database,
            collection,
            is_upsert
        );

        let req = self.create_document_as_entity_create_request(
            database,
            collection,
            is_upsert,
            indexing_directive,
            partition_key,
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
            },
        );

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }


    pub fn list_documents<S1, S2, T>(
        &self,
        database: S1,
        collection: S2,
        ldo: &ListDocumentsOptions,
    ) -> impl Future<Item = ListDocumentsResponse<T>, Error = AzureError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
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
            check_status_extract_headers_and_body(future_response, StatusCode::Ok).and_then(
                move |(headers, whole_body)| {
                    done(list_documents_extract_result::<T>(&whole_body, &headers))
                },
            )
        })
    }


    #[inline]
    fn get_document_create_request(
        &self,
        database: &str,
        collection: &str,
        document_id: &str,
        gdo: &GetDocumentOptions,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs/{}",
            self.authorization_token.account(),
            database,
            collection,
            document_id
        ))?;

        let serialized_partition_key = serialize_partition_key(&gdo.partition_key)?;

        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Get,
            None,
            ResourceType::Documents,
            move |ref mut headers| {
                if let Some(val) = gdo.consistency_level_override {
                    headers.set(ConsistencyLevelHeader(val));
                }
                if let Some(val) = gdo.session_token {
                    headers.set(SessionTokenHeader(val.to_owned()));
                }
                if let Some(val) = gdo.if_none_match {
                    headers.set(IfNoneMatch(val.to_owned()));
                }
                if let Some(val) = serialized_partition_key {
                    headers.set(CosmosDBPartitionKey(val));
                }
            },
        );

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }

    pub fn get_document<S1, S2, S3, T>(
        &self,
        database: S1,
        collection: S2,
        document_id: S3,
        gdo: &GetDocumentOptions,
    ) -> impl Future<Item = GetDocumentResponse<T>, Error = AzureError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
        T: DeserializeOwned,
    {
        let database = database.as_ref();

        let collection = collection.as_ref();
        let document_id = document_id.as_ref();

        trace!(
            "get_document called(database == {}, collection == {}, document_id == {} gdo == {:?}",
            database,
            collection,
            document_id,
            gdo
        );

        let req = self.get_document_create_request(database, collection, document_id, gdo);

        done(req).from_err().and_then(move |future_response| {
            extract_status_headers_and_body(future_response).and_then(
                move |(status, headers, v_body)| {
                    done(get_document_extract_result(status, &headers, &v_body))
                },
            )
        })
    }

    pub fn query_document<'b, S1, S2, T>(
        &self,
        database: S1,
        collection: S2,
        query: &Query<'b>,
        options: &QueryDocumentOptions,
    ) -> impl Future<Item = QueryDocumentResponse<T>, Error = AzureError> + 'b
    where
        T: DeserializeOwned + 'b,
        S1: AsRef<str> + 'b,
        S2: AsRef<str> + 'b,
    {
        self.query_document_json(database, collection, query, options)
            .and_then(move |qdr_json| done(convert_query_document_type(qdr_json)))
    }

    pub fn query_document_json<'b, S1, S2>(
        &self,
        database: S1,
        collection: S2,
        query: &Query<'b>,
        options: &QueryDocumentOptions,
    ) -> impl Future<Item = QueryDocumentResponse<String>, Error = AzureError>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let database = database.as_ref();
        let collection = collection.as_ref();

        trace!(
            "query_document_json called(database == {}, \
             collection == {}, query == {:?}, options = {:?}",
            database,
            collection,
            query,
            options
        );

        let req = self.query_document_create_request(database, collection, query, options);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_headers_and_body(future_response, StatusCode::Ok).and_then(
                move |(headers, v_body)| {
                    done(query_documents_extract_result_json(&v_body, &headers))
                },
            )
        })
    }

    #[inline]
    fn query_document_create_request<'b>(
        &self,
        database: &str,
        collection: &str,
        query: &Query<'b>,
        options: &QueryDocumentOptions,
    ) -> Result<hyper::client::FutureResponse, AzureError> {
        let uri = hyper::Uri::from_str(&format!(
            "https://{}.documents.azure.com/dbs/{}/colls/{}/docs",
            self.authorization_token.account(),
            database,
            collection,
        ))?;

        let query_json = serde_json::to_string(query)?;

        debug!("query_json == {}", query_json);

        let request = prepare_request(
            &self.authorization_token,
            uri,
            hyper::Method::Post,
            Some(&query_json),
            ResourceType::Documents,
            move |ref mut headers| {
                headers.set(DocumentDBIsQuery(true));
                headers.set(ContentType(get_query_content_type()));

                if let Some(val) = options.max_item_count {
                    headers.set(MaxItemCount(val));
                }
                if let Some(val) = options.continuation_token {
                    headers.set(ContinuationTokenHeader(val.to_owned()));
                }
                if let Some(val) = options.enable_cross_partition {
                    headers.set(DocumentDBQueryEnableCrossPartition(val));
                }
                if let Some(val) = options.consistency_level_override {
                    headers.set(ConsistencyLevelHeader(val));
                }
                if let Some(val) = options.session_token {
                    headers.set(SessionTokenHeader(val.to_owned()));
                }
            },
        );

        trace!("request prepared");

        Ok(self.hyper_client.request(request))
    }
}

fn get_document_extract_result<T>(
    status: hyper::StatusCode,
    headers: &hyper::Headers,
    v_body: &[u8],
) -> Result<GetDocumentResponse<T>, AzureError>
where
    T: DeserializeOwned,
{
    match status {
        StatusCode::Ok => {
            let gdah = GetDocumentAdditionalHeaders {
                charge: *(headers.get::<Charge>().unwrap() as &f64),
            };
            debug!("gdah == {:?}", gdah);

            // we will proceed in two steps:
            // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
            // 2- Deserialize the result a type T. The extra fields will be ignored.
            let document = Document {
                document_attributes: serde_json::from_slice::<DocumentAttributes>(v_body)?,
                entity: serde_json::from_slice::<T>(v_body)?,
            };

            Ok(GetDocumentResponse {
                document: Some(document),
                additional_headers: gdah,
            })
        }
        // NotFound is not an error so we return None along
        // with the additional headers.
        StatusCode::NotFound => {
            let gdah = GetDocumentAdditionalHeaders {
                charge: *(headers.get::<Charge>().unwrap() as &f64),
            };
            debug!("gdah == {:?}", gdah);

            Ok(GetDocumentResponse {
                document: None,
                additional_headers: gdah,
            })
        }
        _ => {
            // We treat everything else as an error. We could
            // handle 304 (Not modified) in a specific way but
            // for now we do not.
            let error_text = from_utf8(v_body)?;
            Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                StatusCode::Ok,
                status,
                error_text,
            )))
        }
    }
}

fn query_documents_extract_result_json(
    v_body: &[u8],
    headers: &Headers,
) -> Result<QueryDocumentResponse<String>, AzureError> {
    trace!("headers == {:?}", headers);

    let additional_headers = QueryDocumentResponseAdditonalHeaders {
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
        charge: *(headers.get::<Charge>().unwrap() as &f64),
    };
    debug!("additional_headers == {:?}", additional_headers);

    let query_response_meta = serde_json::from_slice::<QueryResponseMeta>(v_body)?;
    debug!("query_response_meta == {:?}", &query_response_meta);

    let json = from_utf8(v_body)?;
    debug!("json == {}", json);

    let v: Value = serde_json::from_slice(v_body)?;

    // Work on Documents section
    let d = &v["Documents"];
    debug!("\n\nd == {:?}\n\n", d);

    let mut v_docs = Vec::new();

    for doc in d.as_array().unwrap() {
        // We could either have a Document or a plain entry.
        // We will find out here.

        let document_attributes = match serde_json::from_value::<DocumentAttributes>(doc.clone()) {
            Ok(document_attributes) => Some(document_attributes),
            Err(_) => None,
        };

        debug!("\ndocument_attributes == {:?}", document_attributes);

        // Now we are about to create a new Value::Object
        // without the extra Azure fields.
        // This involves a lot a copying (unfortunately).
        let o_new = {
            let mut o_new = Value::Object(Map::new());
            {
                let m_new = o_new.as_object_mut().unwrap();

                for (key, val) in doc.as_object().unwrap() {
                    if AZURE_KEYS.binary_search(&(key as &str)).is_err() {
                        m_new.insert(key.clone(), val.clone());
                    }
                }
            }

            o_new
        };

        v_docs.push(QueryResult {
            document_attributes: document_attributes,
            result: o_new.to_string(),
        });
    }

    Ok(QueryDocumentResponse {
        query_response_meta: query_response_meta,
        additional_headers: additional_headers,
        results: v_docs,
    })
}

#[inline]
fn convert_query_document_type<T>(
    qdr: QueryDocumentResponse<String>,
) -> Result<QueryDocumentResponse<T>, AzureError>
where
    T: DeserializeOwned,
{
    let mut qdr_converted: QueryDocumentResponse<T> = QueryDocumentResponse {
        query_response_meta: qdr.query_response_meta,
        results: Vec::new(),
        additional_headers: qdr.additional_headers,
    };

    for res_json in qdr.results {
        qdr_converted.results.push(QueryResult {
            document_attributes: res_json.document_attributes,
            result: serde_json::from_str(&res_json.result)?,
        });
    }

    Ok(qdr_converted)
}

fn list_documents_extract_result<T>(
    v_body: &[u8],
    headers: &Headers,
) -> Result<ListDocumentsResponse<T>, AzureError>
where
    T: DeserializeOwned,
{
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
        charge: *(headers.get::<Charge>().unwrap() as &f64),
        etag: match headers.get::<Etag>() {
            Some(s) => Some((s as &str).to_owned()),
            None => None,
        },
    };
    debug!("ado == {:?}", ado);

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
        v.push(Document {
            document_attributes: da,
            entity: e,
        });
    }

    Ok(ListDocumentsResponse {
        rid: document_attributes.rid,
        documents: v,
        additional_headers: ado,
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

#[inline]
fn get_query_content_type() -> Mime {
    "application/query+json".parse().unwrap()
}


#[cfg(test)]
mod tests {
    use azure::cosmos::client::*;
    use azure::cosmos::authorization_token;
    use hyper::Uri;

    #[test]
    fn string_to_sign_00() {
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
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
        let time =
            chrono::DateTime::parse_from_rfc3339("1900-01-01T01:00:00.000000000+00:00").unwrap();
        let time = time.with_timezone(&chrono::Utc);
        let time = format!("{}", time.format(TIME_FORMAT));

        let authorization_token = authorization_token::AuthorizationToken::new(
            "mindflavor".to_owned(),
            authorization_token::TokenType::Master,
            "8F8xXXOptJxkblM1DBXW7a6NMI5oE8NnwPGYBmwxLCKfejOK7B7yhcCHMGvN3PBrlMLIOeol1Hv9RCdzAZR5sg=="
                .to_owned(),
        ).unwrap();

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
        let time =
            chrono::DateTime::parse_from_rfc3339("2017-04-27T00:51:12.000000000+00:00").unwrap();
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
