use crate::core::prelude::*;
use azure_core::error::Result;
use azure_core::headers::{
    account_kind_from_headers, date_from_headers, request_id_from_headers, sku_name_from_headers,
};

use azure_core::{Context, RequestId};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder {
    storage_client: StorageClient,
    context: Context,
}

impl GetAccountInformationBuilder {
    pub(crate) fn new(storage_client: StorageClient) -> Self {
        Self {
            storage_client,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(mut self) -> GetAccountInformation {
        Box::pin(async move {
            let mut request = self
                .storage_client
                .storage_account_client()
                .blob_storage_request("", http::Method::GET);

            // TODO: add the query pairs
            // request.uri_mut().query_pairs_mut().append_pair("restype", "account");
            // request.uri_mut().query_pairs_mut().append_pair("comp", "properties");

            let response = self
                .storage_client
                .storage_account_client()
                .pipeline()
                .send(&mut self.context, &mut request)
                .await?;

            GetAccountInformationResponse::try_from(response.headers())
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type GetAccountInformation =
    futures::future::BoxFuture<'static, azure_core::error::Result<GetAccountInformationResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetAccountInformationBuilder {
    type IntoFuture = GetAccountInformation;
    type Output = <GetAccountInformation as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
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
    pub(crate) fn try_from(headers: &HeaderMap) -> Result<GetAccountInformationResponse> {
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
