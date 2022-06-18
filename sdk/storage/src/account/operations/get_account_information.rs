use crate::core::prelude::*;
use azure_core::headers::{
    account_kind_from_headers, date_from_headers, request_id_from_headers, sku_name_from_headers,
    Headers,
};
use azure_core::{Context, RequestId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GetAccountInformationBuilder {
    client: StorageClient,
    context: Context,
}

impl GetAccountInformationBuilder {
    pub(crate) fn new(client: StorageClient) -> Self {
        Self {
            client,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
    }

    pub fn into_future(mut self) -> GetAccountInformation {
        Box::pin(async move {
            let mut request = self
                .client
                .storage_account_client()
                .blob_storage_request(http::Method::GET);

            for (k, v) in [("restype", "account"), ("comp", "properties")].iter() {
                request.url_mut().query_pairs_mut().append_pair(k, v);
            }

            let response = self
                .client
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
    futures::future::BoxFuture<'static, azure_core::Result<GetAccountInformationResponse>>;

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
