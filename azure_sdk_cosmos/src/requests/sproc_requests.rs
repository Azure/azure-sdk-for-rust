use super::*;

pub struct ExecuteStoredProcedureRequest {
    hyper_client: HyperClient,
    request: RequestBuilder,
    payload: Result<String, serde_json::Error>,
}
/*
impl DocumentRequestExt for ExecuteStoredProcedureRequest {
    fn request(&mut self) -> &mut RequestBuilder {
        &mut self.request
    }
}*/

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

    request_bytes_ref!(partition_key, HEADER_DOCUMENTDB_PARTITIONKEY);
    request_option!(
        use_multiple_write_locations,
        bool,
        HEADER_ALLOW_MULTIPLE_WRITES
    );

    pub async fn execute<R: DeserializeOwned>(
        self,
    ) -> Result<ExecuteStoredProcedureResponse<R>, AzureError> {
        trace!(
            "execute_stored_procedure called(request == {:?}",
            self.request
        );
        let hc = self.hyper_client;
        let mut req = self.request;
        let payload = self.payload?;
        let r = req.body(payload.into())?;
        let (headers, v_body) =
            check_status_extract_headers_and_body(hc.request(r), StatusCode::OK).await?;
        Self::extract_result(&headers, &v_body)
    }

    fn extract_result<R: DeserializeOwned>(
        headers: &HeaderMap,
        v_body: &[u8],
    ) -> Result<ExecuteStoredProcedureResponse<R>, AzureError> {
        let additional_headers = DocumentAdditionalHeaders::derive_from(headers);
        let result = serde_json::from_slice(v_body)?;
        Ok(ExecuteStoredProcedureResponse {
            result,
            additional_headers,
        })
    }
}
