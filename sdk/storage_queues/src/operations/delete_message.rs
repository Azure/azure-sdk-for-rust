use crate::clients::PopReceiptClient;
use azure_core::{error::Error, prelude::*, Context, Method, Response as AzureResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug)]
pub struct DeleteMessageBuilder {
    pop_receipt_client: PopReceiptClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl DeleteMessageBuilder {
    pub(crate) fn new(pop_receipt_client: PopReceiptClient) -> Self {
        DeleteMessageBuilder {
            pop_receipt_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.pop_receipt_client.pop_receipt_url()?;

            self.timeout.append_to_url_query(&mut url);

            let mut request = self.pop_receipt_client.storage_client().prepare_request(
                url.as_str(),
                Method::DELETE,
                None,
            )?;

            let response = self
                .pop_receipt_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<DeleteMessageResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteMessageBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for DeleteMessageResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(DeleteMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
