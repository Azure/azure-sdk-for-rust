use super::*;

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
    request_bytes_ref!(partition_key, str, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);

    pub fn execute(self) -> impl Future<Item = DocumentAttributes, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| Ok(req.body(payload.into())?))
            .and_then(move |r| check_status_extract_body(hc.request(r), StatusCode::CREATED))
            .and_then(move |body| Ok(serde_json::from_str::<DocumentAttributes>(&body)?))
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

    request_bytes_ref!(if_none_match, str, header::IF_NONE_MATCH);
    request_bytes_ref!(partition_key, str, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);

    pub fn execute<T: DeserializeOwned>(mut self) -> impl Future<Item = GetDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);

        future::result(self.request.body(hyper::Body::empty()))
            .from_err()
            .and_then(move |r| extract_status_headers_and_body(self.hyper_client.request(r)))
            .and_then(move |(status, headers, body)| Self::extract_result(status, &headers, &body))
    }

    fn extract_result<R: DeserializeOwned>(
        status: hyper::StatusCode,
        headers: &HeaderMap,
        body: &[u8],
    ) -> Result<GetDocumentResponse<R>, AzureError> {
        match status {
            StatusCode::OK => {
                let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
                let document = Document::from_json(body)?;
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
                let error_text = str::from_utf8(body)?;
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
    request_bytes_ref!(continuation_token, str, HEADER_CONTINUATION);
    request_option!(enable_cross_partition, bool, HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION);
    request_option!(
        enable_parallelize_cross_partition_query,
        bool,
        HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY
    );
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);
    request_option!(consistency_level, ConsistencyLevel, HEADER_CONSISTENCY_LEVEL);

    pub fn execute<T: DeserializeOwned>(self) -> impl Future<Item = QueryDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        self.execute_json().and_then(Self::convert_query_document_type)
    }

    pub fn execute_json(self) -> impl Future<Item = QueryDocumentResponse<serde_json::Value>, Error = AzureError> {
        trace!("query_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| Ok(req.body(payload.into())?))
            .and_then(move |r| check_status_extract_headers_and_body(hc.request(r), StatusCode::OK))
            .and_then(move |(headers, body)| Self::extract_result_json(&body, &headers))
    }

    fn extract_result_json(body: &[u8], headers: &HeaderMap) -> Result<QueryDocumentResponse<serde_json::Value>, AzureError> {
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

        let query_response_meta = serde_json::from_slice::<QueryResponseMeta>(body)?;
        debug!("query_response_meta == {:?}", &query_response_meta);

        let json = str::from_utf8(body)?;
        debug!("json == {}", json);

        let mut v: serde_json::Value = serde_json::from_slice(body)?;

        // Work on Documents section
        let mut d = v.get_mut("Documents").unwrap().take();
        debug!("\n\nd == {:?}\n\n", d);

        let docs = d.as_array_mut().unwrap().into_iter().map(|doc| {
            // We could either have a Document or a plain entry.
            // We will find out here.
            let mut doc = doc.take();

            let attrs = {
                if let Some(map) = doc.as_object_mut() {
                    DocumentAttributes::try_extract(map)
                } else {
                    None
                }
            };

            debug!("attrs == {:?}", attrs);

            QueryResult {
                document_attributes: attrs,
                result: doc,
            }
        });

        Ok(QueryDocumentResponse {
            query_response_meta,
            additional_headers,
            results: docs.collect(),
        })
    }

    #[inline]
    fn convert_query_document_type<T>(qdr: QueryDocumentResponse<serde_json::Value>) -> Result<QueryDocumentResponse<T>, AzureError>
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
                result: serde_json::from_value(res_json.result)?,
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
    request_bytes_ref!(continuation_token, str, HEADER_CONTINUATION);
    request_option!(consistency_level, ConsistencyLevel, HEADER_CONSISTENCY_LEVEL);
    request_bytes_ref!(session_token, str, HEADER_SESSION_TOKEN);
    request_bytes_ref!(if_none_match, str, header::IF_NONE_MATCH);
    request_bytes_ref!(partition_range_id, str, HEADER_DOCUMENTDB_PARTITIONRANGEID);
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);

    pub fn incremental_feed(mut self) -> Self {
        self.request.header(HEADER_A_IM, HeaderValue::from_static("Incremental feed"));
        self
    }

    pub fn execute<T: DeserializeOwned>(mut self) -> impl Future<Item = ListDocumentsResponse<T>, Error = AzureError> {
        future::result(self.request.body(hyper::Body::empty()))
            .from_err()
            .and_then(move |r| check_status_extract_headers_and_body(self.hyper_client.request(r), StatusCode::OK))
            .and_then(|(headers, whole_body)| Self::extract_result::<T>(&whole_body, &headers))
    }

    fn extract_result<T>(body: &[u8], headers: &HeaderMap) -> Result<ListDocumentsResponse<T>, AzureError>
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
        let document_attributes = serde_json::from_slice::<ListDocumentsResponseAttributes>(body)?;
        let entries = serde_json::from_slice::<ListDocumentsResponseEntities<T>>(body)?;

        let documents = document_attributes
            .documents
            .into_iter()
            .zip(entries.entities.into_iter())
            .map(|(da, e)| Document {
                document_attributes: da,
                entity: e,
            })
            .collect();

        Ok(ListDocumentsResponse {
            rid: document_attributes.rid,
            documents,
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

    request_bytes_ref!(if_match, str, header::IF_MATCH);
    request_option!(indexing_directive, IndexingDirective, HEADER_INDEXING_DIRECTIVE);
    request_bytes_ref!(partition_key, str, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);

    pub fn execute(self) -> impl Future<Item = ReplaceDocumentResponse<T>, Error = AzureError> {
        trace!("get_document called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| Ok(req.body(payload.into())?))
            .and_then(move |r| check_status_extract_headers_and_body(hc.request(r), StatusCode::OK))
            .and_then(move |(headers, body)| Self::extract_result(&headers, &body))
    }

    fn extract_result<R: DeserializeOwned>(headers: &HeaderMap, body: &[u8]) -> Result<ReplaceDocumentResponse<R>, AzureError> {
        let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
        let document = Document::from_json(body)?;
        Ok(ReplaceDocumentResponse {
            document,
            additional_headers,
        })
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

    request_bytes_ref!(if_match, str, header::IF_MATCH);
    request_bytes_ref!(partition_key, str, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(use_multiple_write_locations, bool, HEADER_ALLOW_MULTIPLE_WRITES);

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
