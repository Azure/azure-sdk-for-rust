use azure::core::{
    errors::{
        check_status_extract_body, check_status_extract_headers_and_body,
        extract_status_headers_and_body, AzureError, UnexpectedHTTPResult,
    },
    incompletevector::ContinuationToken,
};
use azure::cosmos::client::headers::AIM;
use azure::cosmos::{
    client::headers::*, document::{DocumentAttributes, IndexingDirective},
    partition_key::PartitionKey, request_response::*, ConsistencyLevel,
};
use futures::{future::ok, future::result, prelude::*};
use hyper::{self, client::HttpConnector, Client, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use std::rc::Rc;
use std::{marker::PhantomData, str};

type HyperClient = Rc<Client<HttpsConnector<HttpConnector>>>;

macro_rules! request_option {
    ($name:ident, bool, $h:ident) => {
        pub fn $name<V: Into<bool>>(mut self, value: V) -> Self {
            if value.into() {
                self.request.headers_mut().set($h(true));
            }
            else {
                self.request.headers_mut().remove::<$h>();
            }
            self
        }
    };
    ($name:ident, $ty:ty, $h:ident) => {
        pub fn $name<V: Into<Option<$ty>>>(mut self, value: V) -> Self {
            if let Some(v) = value.into() {
                self.request.headers_mut().set($h(v));
            }
            else {
                self.request.headers_mut().remove::<$h>();
            }
            self
        }
    };
}

pub struct CreateDocumentRequest {
    hyper_client: HyperClient,
    request: Request,
}

impl DocumentRequestExt for CreateDocumentRequest {
    fn request(&mut self) -> &mut Request {
        &mut self.request
    }
}

impl CreateDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: Request) -> CreateDocumentRequest {
        CreateDocumentRequest {
            hyper_client,
            request,
        }
    }

    request_option!(is_upsert, bool, DocumentIsUpsert);
    request_option!(indexing_directive, IndexingDirective, DocumentIndexingDirective);

    pub fn execute(self) -> impl Future<Item = DocumentAttributes, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        let future_response = self.hyper_client.request(self.request);
        check_status_extract_body(future_response, StatusCode::Created).and_then(move |body| {
            result(serde_json::from_str::<DocumentAttributes>(&body)).from_err()
        })
    }
}

pub struct GetDocumentRequest {
    hyper_client: HyperClient,
    request: Request,
}

impl DocumentRequestExt for GetDocumentRequest {
    fn request(&mut self) -> &mut Request {
        &mut self.request
    }
}

impl GetDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: Request) -> GetDocumentRequest {
        GetDocumentRequest {
            hyper_client,
            request,
        }
    }

    request_option!(if_none_match, String, IfNoneMatch);

    pub fn execute<T: DeserializeOwned>(
        self,
    ) -> impl Future<Item = GetDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        extract_status_headers_and_body(self.hyper_client.request(self.request)).and_then(
            move |(status, headers, v_body)| {
                result(Self::get_document_extract_result(status, &headers, &v_body))
            },
        )
    }

    fn get_document_extract_result<R: DeserializeOwned>(
        status: hyper::StatusCode,
        headers: &hyper::Headers,
        v_body: &[u8],
    ) -> Result<GetDocumentResponse<R>, AzureError> {
        match status {
            StatusCode::Ok => {
                let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
                let document = Document::from_json(v_body)?;
                Ok(GetDocumentResponse {
                    document: Some(document),
                    additional_headers,
                })
            }
            // NotFound is not an error so we return None along
            // with the additional headers.
            StatusCode::NotFound => {
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
                    StatusCode::Ok,
                    status,
                    error_text,
                )))
            }
        }
    }
}

pub struct QueryDocumentRequest {
    hyper_client: HyperClient,
    request: Request,
}

impl DocumentRequestExt for QueryDocumentRequest {
    fn request(&mut self) -> &mut Request {
        &mut self.request
    }
}

lazy_static! {
    static ref QUERY_CONTENT_TYPE: hyper::header::ContentType =
        hyper::header::ContentType("application/query+json".parse().unwrap());
}

impl QueryDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, mut request: Request) -> QueryDocumentRequest {
        {
            let headers = request.headers_mut();
            headers.set(DocumentDBIsQuery(true));
            headers.set(QUERY_CONTENT_TYPE.clone());
        }
        QueryDocumentRequest {
            hyper_client,
            request,
        }
    }

    request_option!(max_item_count, u64, MaxItemCount);
    request_option!(continuation_token, ContinuationToken, ContinuationTokenHeader);
    request_option!(enable_cross_partition, bool, DocumentDBQueryEnableCrossPartition);
    request_option!(consistency_level, ConsistencyLevel, ConsistencyLevelHeader);

    pub fn execute<T: DeserializeOwned>(
        self,
    ) -> impl Future<Item = QueryDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        self.execute_json()
            .and_then(move |qdr_json| result(Self::convert_query_document_type(qdr_json)))
    }

    pub fn execute_json(
        self,
    ) -> impl Future<Item = QueryDocumentResponse<String>, Error = AzureError> {
        trace!("query_document called(request == {:?}", self.request);

        check_status_extract_headers_and_body(
            self.hyper_client.request(self.request),
            StatusCode::Ok,
        ).and_then(move |(headers, v_body)| {
            result(Self::query_documents_extract_result_json(&v_body, &headers))
        })
    }

    const AZURE_KEYS: [&'static str; 5] = ["_attachments", "_etag", "_rid", "_self", "_ts"];

    fn query_documents_extract_result_json(
        v_body: &[u8],
        headers: &hyper::Headers,
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

            let document_attributes =
                match serde_json::from_value::<DocumentAttributes>(doc.clone()) {
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
}

pub struct ListDocumentsRequest {
    hyper_client: HyperClient,
    request: Request,
}

impl ListDocumentsRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: Request) -> ListDocumentsRequest {
        ListDocumentsRequest {
            hyper_client,
            request,
        }
    }

    request_option!(max_item_count, u64, MaxItemCount);
    request_option!(continuation_token, ContinuationToken, ContinuationTokenHeader);
    request_option!(consistency_level, ConsistencyLevel, ConsistencyLevelHeader);
    request_option!(session_token, String, SessionTokenHeader);
    request_option!(if_none_match, String, IfNoneMatch);
    request_option!(partition_range_id, String, PartitionRangeId);

    pub fn incremental_feed(mut self, value: bool) -> Self {
        if value {
            self.request.headers_mut().set(AIM::new("Incremental feed"));
        } else {
            self.request.headers_mut().remove::<AIM>();
        }
        self
    }

    pub fn execute<T: DeserializeOwned>(
        self,
    ) -> impl Future<Item = ListDocumentsResponse<T>, Error = AzureError> {
        check_status_extract_headers_and_body(
            self.hyper_client.request(self.request),
            StatusCode::Ok,
        ).and_then(|(headers, whole_body)| {
            result(Self::list_documents_extract_result::<T>(
                &whole_body,
                &headers,
            ))
        })
    }

    fn list_documents_extract_result<T>(
        v_body: &[u8],
        headers: &hyper::header::Headers,
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
        let document_attributes =
            serde_json::from_slice::<ListDocumentsResponseAttributes>(v_body)?;
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
}

pub struct ReplaceDocumentRequest<T> {
    hyper_client: HyperClient,
    request: Request,
    _t: PhantomData<T>,
}

impl<T> DocumentRequestExt for ReplaceDocumentRequest<T> {
    fn request(&mut self) -> &mut Request {
        &mut self.request
    }
}

impl<T: DeserializeOwned> ReplaceDocumentRequest<T> {
    pub(crate) fn new(hyper_client: HyperClient, request: Request) -> ReplaceDocumentRequest<T> {
        ReplaceDocumentRequest {
            hyper_client,
            request,
            _t: PhantomData,
        }
    }

    request_option!(if_match, String, IfMatch);
    request_option!(indexing_directive, IndexingDirective, DocumentIndexingDirective);

    pub fn execute(self) -> impl Future<Item = ReplaceDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        extract_status_headers_and_body(self.hyper_client.request(self.request)).and_then(
            move |(status, headers, v_body)| {
                result(Self::replace_document_extract_result(
                    status, &headers, &v_body,
                ))
            },
        )
    }

    fn replace_document_extract_result<R: DeserializeOwned>(
        status: hyper::StatusCode,
        headers: &hyper::Headers,
        v_body: &[u8],
    ) -> Result<ReplaceDocumentResponse<R>, AzureError> {
        match status {
            StatusCode::Ok => {
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
                    StatusCode::Ok,
                    status,
                    error_text,
                )))
            }
        }
    }
}

pub struct DeleteDocumentRequest {
    hyper_client: HyperClient,
    request: Request,
}

impl DocumentRequestExt for DeleteDocumentRequest {
    fn request(&mut self) -> &mut Request {
        &mut self.request
    }
}

impl DeleteDocumentRequest {
    pub(crate) fn new(hyper_client: HyperClient, request: Request) -> DeleteDocumentRequest {
        DeleteDocumentRequest {
            hyper_client,
            request,
        }
    }

    request_option!(if_match, String, IfMatch);

    pub fn execute(self) -> impl Future<Item = (), Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        let future_response = self.hyper_client.request(self.request);
        check_status_extract_body(future_response, StatusCode::NoContent).and_then(|_| ok(()))
    }
}

pub trait DocumentRequestExt: Sized {
    fn request(&mut self) -> &mut Request;

    fn session_token<S: Into<String>>(mut self, token: S) -> Self {
        self.request()
            .headers_mut()
            .set(SessionTokenHeader(token.into()));
        self
    }

    fn partition_key<'a, P: Into<PartitionKey<'a>>>(mut self, key: P) -> Self {
        // todo: move unwrap into PartitionKey impl itself as we control the impl and it surely won't error out
        if let Some(ser_key) = key.into().to_json().unwrap() {
            self.request()
                .headers_mut()
                .set(CosmosDBPartitionKey(ser_key));
        } else {
            self.request()
                .headers_mut()
                .remove::<CosmosDBPartitionKey>();
        }
        self
    }
}
