use crate::clients::ServiceType;
use crate::core::prelude::*;
use azure_core::headers::{
    account_kind_from_headers, date_from_headers, request_id_from_headers, sku_name_from_headers,
    Headers,
};
use azure_core::RequestId;
use chrono::{DateTime, Utc};

operation! {
    GetAccountInformation,
    client: StorageClient,
}

impl GetAccountInformationBuilder {
    pub fn into_future(mut self) -> GetAccountInformation {
        Box::pin(async move {
            let mut request = self.client.blob_storage_request(azure_core::Method::Get)?;

            for (k, v) in [("restype", "account"), ("comp", "properties")].iter() {
                request.url_mut().query_pairs_mut().append_pair(k, v);
            }

            let response = self
                .client
                .send(&mut self.context, &mut request, ServiceType::Blob)
                .await?;

            GetAccountInformationResponse::try_from(response.headers())
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetAccountInformationResponse {
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub sku_name: String,
    pub account_kind: String,
}

impl GetAccountInformationResponse {
    pub(crate) fn try_from(headers: &Headers) -> azure_core::Result<GetAccountInformationResponse> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let sku_name = sku_name_from_headers(headers)?;
        let account_kind = account_kind_from_headers(headers)?;

        Ok(GetAccountInformationResponse {
            request_id,
            date,
            sku_name,
            account_kind,
        })
    }
}
