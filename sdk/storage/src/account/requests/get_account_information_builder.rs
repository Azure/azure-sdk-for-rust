use crate::account::responses::GetAccountInformationResponse;
use crate::core::prelude::*;
use azure_core::error::Result;

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder<'a> {
    storage_client: &'a StorageClient,
}

impl<'a> GetAccountInformationBuilder<'a> {
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self {
        Self { storage_client }
    }
}

impl<'a> GetAccountInformationBuilder<'a> {
    pub async fn execute(self) -> Result<GetAccountInformationResponse> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();

        url.query_pairs_mut().append_pair("restype", "account");
        url.query_pairs_mut().append_pair("comp", "properties");

        trace!("url == {:?}", url);

        let (request, _url) = self.storage_client.prepare_request(
            url.as_str(),
            &http::Method::GET,
            &|request| request,
            None,
        )?;

        let response = self
            .storage_client
            .http_client()
            .execute_request_check_status(request, http::StatusCode::OK)
            .await?;

        Ok(GetAccountInformationResponse::from_headers(
            response.headers(),
        )?)
    }
}
