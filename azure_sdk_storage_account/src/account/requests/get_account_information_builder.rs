use crate::account::responses::GetAccountInformationResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetAccountInformationBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> GetAccountInformationBuilder<'a> {
        GetAccountInformationBuilder { client }
    }
}

impl<'a> ClientRequired<'a> for GetAccountInformationBuilder<'a> {
    fn client(&self) -> &'a Client {
        self.client
    }
}

// methods callable regardless
impl<'a> GetAccountInformationBuilder<'a> {
    #[inline]
    pub async fn finalize(self) -> Result<GetAccountInformationResponse, AzureError> {
        let uri = format!(
            "{}/?restype=account&comp=properties",
            self.client.blob_uri()
        );
        trace!("uri == {:?}", uri);

        let req = self
            .client()
            .perform_request(&uri, &Method::GET, |request| request, None);
        let (headers, _) = check_status_extract_headers_and_body(req?, StatusCode::OK).await?;
        GetAccountInformationResponse::from_headers(&headers)
    }
}
