use azure::core::{
    errors::{
        check_status_extract_body, check_status_extract_headers_and_body, extract_status_headers_and_body, AzureError, UnexpectedHTTPResult,
    },
    incompletevector::ContinuationToken, util::RequestBuilderExt,
};
use azure::cosmos::{
    client::headers::*, document::{DocumentAttributes, IndexingDirective}, partition_key::PartitionKey, request_response::*,
    ConsistencyLevel,
};
use futures::{future, prelude::*};
use http::request::Builder as RequestBuilder;
use hyper::{
    self, client::HttpConnector, header::{self, HeaderMap, HeaderValue}, Client, StatusCode,
};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use std::sync::Arc;
use std::{marker::PhantomData, str};

type HyperClient = Arc<Client<HttpsConnector<HttpConnector>>>;

macro_rules! request_bytes_option {
    ($name:ident, $ty:ty, $h:path) => {
        pub fn $name<V: Into<$ty>>(mut self, value: V) -> Self {
            self.request.header_bytes($h, value.into());
            self
        }
    };
}

macro_rules! request_option {
    ($name:ident, bool, $h:path) => {
        pub fn $name<V: Into<bool>>(mut self, value: V) -> Self {
            self.request.header($h, ::http::header::HeaderValue::from_static(
                if value.into() { "true" } else { "false" }));
            self
        }
    };
    ($name:ident, $ty:ty, $h:path) => {
        pub fn $name<V: Into<$ty>>(mut self, value: V) -> Self {
            self.request.header_formatted($h, value.into());
            self
        }
    };
}

pub struct CreateDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}

impl DocumentRequestExt for CreateDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl CreateDocumentRequest {
    pub(crate) fn new(
        hyper_client: HyperClient,
        request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> CreateDocumentRequest {
        CreateDocumentRequest {
            hyper_client,
            request,
            payload,
        }
    }

    request_option!(is_upsert, bool, HEADER_DOCUMENTDB_IS_UPSERT);
    request_option!(indexing_directive, IndexingDirective, HEADER_INDEXING_DIRECTIVE);

    pub fn execute(self) -> impl Future<Item = DocumentAttributes, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| future::result(req.body(payload.into())).from_err())
            .and_then(move |r| check_status_extract_body(hc.request(r), StatusCode::CREATED))
            .and_then(move |body| future::result(serde_json::from_str::<DocumentAttributes>(&body)).from_err())
    }
}

pub struct GetDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
}

impl DocumentRequestExt for GetDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl GetDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: RequestBuilder) -> GetDocumentRequest {
        GetDocumentRequest { hyper_client, request }
    }

    request_bytes_option!(if_none_match, String, header::IF_NONE_MATCH);

    pub fn execute<T: DeserializeOwned>(mut self) -> impl Future<Item = GetDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        future::result(self.request.body(hyper::Body::empty()))
            .from_err::<AzureError>()
            .and_then(move |r| {
                extract_status_headers_and_body(self.hyper_client.request(r))
                    .and_then(move |(status, headers, v_body)| future::result(Self::extract_result(status, &headers, &v_body)))
            })
    }

    fn extract_result<R: DeserializeOwned>(
        status: hyper::StatusCode,
        headers: &HeaderMap,
        v_body: &[u8],
    ) -> Result<GetDocumentResponse<R>, AzureError> {
        match status {
            StatusCode::OK => {
                let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
                let document = Document::from_json(v_body)?;
                Ok(GetDocumentResponse {
                    document: Some(document),
                    additional_headers,
                })
            }
            // NotFound is not an error so we return None along
            // with the additional headers.
            StatusCode::NOT_FOUND => {
                let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
                Ok(GetDocumentResponse {
                    document: None,
                    additional_headers,
                })
            }
            _ => {
                // We treat everything else as an error. We could
                // handle 304 (Not modified) in a specific way but
                // for now we do not.
                let error_text = str::from_utf8(v_body)?;
                Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                    StatusCode::OK,
                    status,
                    error_text,
                )))
            }
        }
    }
}

pub struct QueryDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}

impl DocumentRequestExt for QueryDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

const QUERY_CONTENT_TYPE: &str = "application/query+json";

impl QueryDocumentRequest {
    pub(crate) fn new(
        hyper_client: HyperClient,
        mut request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> QueryDocumentRequest {
        request
            .header(HEADER_DOCUMENTDB_ISQUERY, HeaderValue::from_static("true"))
            .header(header::CONTENT_TYPE, HeaderValue::from_static(QUERY_CONTENT_TYPE));
        QueryDocumentRequest {
            hyper_client,
            request,
            payload,
        }
    }

    request_option!(max_item_count, u64, HEADER_MAX_ITEM_COUNT);
    request_bytes_option!(continuation_token, ContinuationToken, HEADER_CONTINUATION);
    request_option!(enable_cross_partition, bool, HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION);
    request_option!(consistency_level, ConsistencyLevel, HEADER_CONSISTENCY_LEVEL);

    pub fn execute<T: DeserializeOwned>(self) -> impl Future<Item = QueryDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        self.execute_json()
            .and_then(move |qdr_json| future::result(Self::convert_query_document_type(qdr_json)))
    }

    pub fn execute_json(self) -> impl Future<Item = QueryDocumentResponse<String>, Error = AzureError> {
        trace!("query_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err::<AzureError>()
            .and_then(move |payload| future::result(req.body(payload.into())).from_err())
            .and_then(move |r| {
                check_status_extract_headers_and_body(hc.request(r), StatusCode::OK)
                    .and_then(move |(headers, v_body)| future::result(Self::extract_result_json(&v_body, &headers)))
            })
    }

    const AZURE_KEYS: [&'static str; 5] = ["_attachments", "_etag", "_rid", "_self", "_ts"];

    fn extract_result_json(v_body: &[u8], headers: &HeaderMap) -> Result<QueryDocumentResponse<String>, AzureError> {
        trace!("headers == {:?}", headers);

        let additional_headers = QueryDocumentResponseAdditonalHeaders {
            // This match just tries to extract the info and convert it
            // into the correct type. It is complicated because headers
            // can be missing and also because headers.get<T> will return
            // a T reference (&T) so we need to cast it into the
            // correct type and clone it (in this case into a &str that will
            // become a String using to_owned())
            continuation_token: derive_continuation_token(headers),
            // Here we assume the Charge header to always be present.
            // If problems arise we
            // will change the field to be Option(al).
            charge: derive_request_charge(headers),
        };
        debug!("additional_headers == {:?}", additional_headers);

        let query_response_meta = serde_json::from_slice::<QueryResponseMeta>(v_body)?;
        debug!("query_response_meta == {:?}", &query_response_meta);

        let json = str::from_utf8(v_body)?;
        debug!("json == {}", json);

        let v: serde_json::Value = serde_json::from_slice(v_body)?;

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
                let mut o_new = serde_json::Value::Object(serde_json::map::Map::new());
                {
                    let m_new = o_new.as_object_mut().unwrap();

                    for (key, val) in doc.as_object().unwrap() {
                        if Self::AZURE_KEYS.binary_search(&(key as &str)).is_err() {
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
    fn convert_query_document_type<T>(qdr: QueryDocumentResponse<String>) -> Result<QueryDocumentResponse<T>, AzureError>
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
}

pub struct ListDocumentsRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
}

impl ListDocumentsRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: RequestBuilder) -> ListDocumentsRequest {
        ListDocumentsRequest { hyper_client, request }
    }

    request_option!(max_item_count, u64, HEADER_MAX_ITEM_COUNT);
    request_bytes_option!(continuation_token, ContinuationToken, HEADER_CONTINUATION);
    request_option!(consistency_level, ConsistencyLevel, HEADER_CONSISTENCY_LEVEL);
    request_bytes_option!(session_token, String, HEADER_SESSION_TOKEN);
    request_bytes_option!(if_none_match, String, header::IF_NONE_MATCH);
    request_bytes_option!(partition_range_id, String, HEADER_DOCUMENTDB_PARTITIONRANGEID);

    pub fn incremental_feed(mut self) -> Self {
        self.request.header(HEADER_A_IM, HeaderValue::from_static("Incremental feed"));
        self
    }

    pub fn execute<T: DeserializeOwned>(mut self) -> impl Future<Item = ListDocumentsResponse<T>, Error = AzureError> {
        future::result(self.request.body(hyper::Body::empty()))
            .from_err()
            .and_then(move |r| check_status_extract_headers_and_body(self.hyper_client.request(r), StatusCode::OK))
            .and_then(|(headers, whole_body)| future::result(Self::extract_result::<T>(&whole_body, &headers)))
    }

    fn extract_result<T>(v_body: &[u8], headers: &HeaderMap) -> Result<ListDocumentsResponse<T>, AzureError>
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
            continuation_token: derive_continuation_token(headers),
            // Here we assume the Charge header to always be present.
            // If problems arise we
            // will change the field to be Option(al).
            charge: derive_request_charge(headers),
            etag: headers.get(header::ETAG).and_then(|v| v.to_str().ok()).map(|s| s.to_owned()),
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
        for (da, e) in document_attributes.documents.into_iter().zip(entries.entities.into_iter()) {
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
}

pub struct ReplaceDocumentRequest<T> {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
    _t: PhantomData<T>,
}

impl<T> DocumentRequestExt for ReplaceDocumentRequest<T> {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl<T: DeserializeOwned> ReplaceDocumentRequest<T> {
    pub(crate) fn new(
        hyper_client: HyperClient,
        request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> ReplaceDocumentRequest<T> {
        ReplaceDocumentRequest {
            hyper_client,
            request,
            payload,
            _t: PhantomData,
        }
    }

    request_bytes_option!(if_match, String, header::IF_MATCH);
    request_option!(indexing_directive, IndexingDirective, HEADER_INDEXING_DIRECTIVE);

    pub fn execute(self) -> impl Future<Item = ReplaceDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| future::result(req.body(payload.into())).from_err())
            .and_then(move |r| extract_status_headers_and_body(hc.request(r)))
            .and_then(move |(status, headers, v_body)| future::result(Self::extract_result(status, &headers, &v_body)))
    }

    fn extract_result<R: DeserializeOwned>(
        status: hyper::StatusCode,
        headers: &HeaderMap,
        v_body: &[u8],
    ) -> Result<ReplaceDocumentResponse<R>, AzureError> {
        match status {
            StatusCode::OK => {
                let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
                let document = Document::from_json(v_body)?;
                Ok(ReplaceDocumentResponse {
                    document,
                    additional_headers,
                })
            }
            _ => {
                let error_text = str::from_utf8(v_body)?;
                Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                    StatusCode::OK,
                    status,
                    error_text,
                )))
            }
        }
    }
}

pub struct DeleteDocumentRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
}

impl DocumentRequestExt for DeleteDocumentRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl DeleteDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: RequestBuilder) -> DeleteDocumentRequest {
        DeleteDocumentRequest { hyper_client, request }
    }

    request_bytes_option!(if_match, String, header::IF_MATCH);

    pub fn execute(mut self) -> impl Future<Item = (), Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        future::result(self.request.body(hyper::Body::empty()))
            .from_err()
            .and_then(move |r| check_status_extract_body(self.hyper_client.request(r), StatusCode::NO_CONTENT))
            .and_then(|_| Ok(()))
    }
}

pub trait DocumentRequestExt: Sized {
    fn request(&mut self) -> &mut RequestBuilder;

    fn session_token<S: AsRef<str>>(mut self, token: S) -> Self {
        self.request().header_formatted(HEADER_SESSION_TOKEN, token.as_ref());
        self
    }

    fn partition_key<'a, P: Into<PartitionKey<'a>>>(mut self, key: P) -> Self {
        // todo: move unwrap into PartitionKey impl itself as we control the impl and it surely won't error out
        if let Some(ser_key) = key.into().to_json().unwrap() {
            self.request().header_formatted(HEADER_DOCUMENTDB_PARTITIONKEY, ser_key);
        }
        self
    }
}

fn derive_continuation_token(headers: &HeaderMap) -> Option<String> {
    headers.get(HEADER_CONTINUATION).and_then(|v| v.to_str().ok()).map(|v| v.to_owned())
}

fn derive_request_charge(headers: &HeaderMap) -> f64 {
    headers.get(HEADER_REQUEST_CHARGE).unwrap().to_str().unwrap().parse().unwrap()
}
