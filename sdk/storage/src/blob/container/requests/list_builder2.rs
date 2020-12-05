use crate::clients::{ServiceType, StorageAccountClient};
use crate::container::incomplete_vector_from_container_response;
use crate::container::responses::ListContainersResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::request_id_from_headers;
use azure_core::prelude::*;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct ListBuilder2<'a> {
    storage_account_client: &'a StorageAccountClient,
}

impl<'a> ListBuilder2<'a> {
    pub(crate) fn new(storage_account_client: &'a StorageAccountClient) -> Self {
        Self {
            storage_account_client,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> ListBuilder2<'a> {
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let uri = format!(
            "{}?comp=list",
            self.storage_account_client.blob_storage_uri()
        );

        debug!("generated uri = {}", uri);

        let request = self.storage_account_client.prepare_request(
            &uri,
            &Method::GET,
            &|request| request,
            ServiceType::Blob,
            None,
        )?;

        let response = self
            .storage_account_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::OK)
            .await?;

        println!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;

        println!("body == {}", body);

        Ok(())

        //let (headers, body) = perform_request_response
        //    .check_status_extract_headers_and_body(StatusCode::OK)
        //    .await?;
        //let body = std::str::from_utf8(&body)?;
        //let incomplete_vector = incomplete_vector_from_container_response(&body)?;
        //let request_id = request_id_from_headers(&headers)?;
        //Ok(ListContainersResponse {
        //    incomplete_vector,
        //    request_id,
        //})
    }
}
