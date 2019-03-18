use super::*;

pub struct ExecuteStoredProcedureRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}

impl DocumentRequestExt for ExecuteStoredProcedureRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}

impl ExecuteStoredProcedureRequest {
    pub(crate) fn new(
        hyper_client: HyperClient,
        request: RequestBuilder,
        payload: Result<String, serde_json::Error>,
    ) -> ExecuteStoredProcedureRequest {
        ExecuteStoredProcedureRequest {
            hyper_client,
            request,
            payload,
        }
    }

    request_bytes_ref!(partition_key, str, HEADER_DOCUMENTDB_PARTITIONKEY);

    pub fn execute<R: DeserializeOwned>(self) -> impl Future<Item = ExecuteStoredProcedureResponse<R>, Error = AzureError> {
        trace!("execute_stored_procedure called(request == {:?}", self.request);
        let hc = self.hyper_client;
        let mut req = self.request;
        future::result(self.payload)
            .from_err()
            .and_then(move |payload| future::result(req.body(payload.into())).from_err())
            .and_then(move |r| check_status_extract_headers_and_body(hc.request(r), StatusCode::OK))
            .and_then(move |(headers, v_body)| Self::extract_result(&headers, &v_body))
    }

    fn extract_result<R: DeserializeOwned>(headers: &HeaderMap, v_body: &[u8]) -> Result<ExecuteStoredProcedureResponse<R>, AzureError> {
        let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
        let result = serde_json::from_slice(v_body)?;
        Ok(ExecuteStoredProcedureResponse {
            result,
            additional_headers,
        })
    }
}
