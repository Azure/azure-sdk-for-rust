use crate::clients::BlobStorageAccountClient;
use crate::container::incomplete_vector_from_container_response;
use crate::container::responses::ListContainersResponse;
use azure_core::headers::request_id_from_headers;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct ListBuilder2<'a> {
    blob_storage_account_client: &'a BlobStorageAccountClient,
    prefix: Option<&'a str>,
    next_marker: Option<&'a str>,
    include_metadata: bool,
    max_results: Option<u32>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
}

impl<'a> ListBuilder2<'a> {
    pub(crate) fn new(blob_storage_account_client: &'a BlobStorageAccountClient) -> Self {
        Self {
            blob_storage_account_client,
            prefix: None,
            next_marker: None,
            include_metadata: false,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }

    pub fn with_prefix(self, prefix: &'a str) -> Self {
        Self {
            prefix: Some(prefix),
            ..self
        }
    }

    //next_marker: Option<&'a str>,
    // include_metadata: bool,

    pub async fn execute(
        &self,
    ) -> Result<ListContainersResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut uri = format!(
            "{}?comp=list",
            self.blob_storage_account_client
                .storage_account_client()
                .blob_storage_uri()
        );

        // TODO: this will be better once PR #110 is accepted
        if let Some(prefix) = &self.prefix {
            uri = format!("{}&prefix={}", uri, prefix);
        }

        debug!("generated uri = {}", uri);

        let request = self.blob_storage_account_client.prepare_request(
            &uri,
            &Method::GET,
            &|request| request,
            None,
        )?;

        let response = self
            .blob_storage_account_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        debug!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;
        debug!("body == {}", body);

        let incomplete_vector = incomplete_vector_from_container_response(&body)?;
        let request_id = request_id_from_headers(response.headers())?;
        Ok(ListContainersResponse {
            incomplete_vector,
            request_id,
        })
    }
}
