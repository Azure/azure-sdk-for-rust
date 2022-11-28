use crate::prelude::BlobServiceClient;
use azure_core::headers::{account_kind_from_headers, sku_name_from_headers, Headers};
use azure_core::Method;
use azure_storage::headers::CommonStorageResponseHeaders;

operation! {
    GetAccountInformation,
    client: BlobServiceClient,
}

impl GetAccountInformationBuilder {
    pub fn into_future(mut self) -> GetAccountInformation {
        Box::pin(async move {
            let mut url = self.client.url()?;

            for (k, v) in [("restype", "account"), ("comp", "properties")].iter() {
                url.query_pairs_mut().append_pair(k, v);
            }
            let mut request =
                self.client
                    .finalize_request(url, Method::Get, Headers::new(), None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            GetAccountInformationResponse::try_from(response.headers())
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountInformationResponse {
    pub common: CommonStorageResponseHeaders,
    pub sku_name: String,
    pub account_kind: String,
}

impl GetAccountInformationResponse {
    pub(crate) fn try_from(headers: &Headers) -> azure_core::Result<GetAccountInformationResponse> {
        let common = CommonStorageResponseHeaders::try_from(headers)?;
        let sku_name = sku_name_from_headers(headers)?;
        let account_kind = account_kind_from_headers(headers)?;

        Ok(GetAccountInformationResponse {
            common,
            sku_name,
            account_kind,
        })
    }
}
