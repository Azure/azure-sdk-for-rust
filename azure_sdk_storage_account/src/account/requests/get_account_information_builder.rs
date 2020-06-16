use crate::account::responses::GetAccountInformationResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_storage_core::prelude::*;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder<'a, C>
where
    C: Client,
{
    client: &'a C,
}

impl<'a, C> GetAccountInformationBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> GetAccountInformationBuilder<'a, C> {
        GetAccountInformationBuilder { client }
    }
}

impl<'a, C> ClientRequired<'a, C> for GetAccountInformationBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> GetAccountInformationBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub async fn finalize(self) -> Result<GetAccountInformationResponse, AzureError> {
        let uri = format!(
            "{}/?restype=account&comp=properties",
            self.client.blob_uri()
        );
        trace!("uri == {:?}", uri);

        let req = self
            .client()
            .perform_request(&uri, &Method::GET, &|request| request, None);
        let (headers, _) = check_status_extract_headers_and_body(req?, StatusCode::OK).await?;
        GetAccountInformationResponse::from_headers(&headers)
    }
}
