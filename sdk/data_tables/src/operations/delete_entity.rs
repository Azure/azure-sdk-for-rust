use crate::{
    prelude::{IfMatchCondition, *},
    TransactionOperation,
};
use azure_core::{error::Error, headers::Headers, prelude::*, Context, Method, Request, Response};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct DeleteEntityBuilder {
    entity_client: EntityClient,
    if_match: IfMatchCondition,
    timeout: Option<Timeout>,
    context: Context,
}

impl DeleteEntityBuilder {
    pub(crate) fn new(entity_client: EntityClient) -> Self {
        Self {
            entity_client,
            if_match: IfMatchCondition::Any,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        if_match: IfMatchCondition => if_match,
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let mut url = self.entity_client.url().clone();

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.add(self.if_match);

            let mut request =
                self.entity_client
                    .finalize_request(url, Method::Delete, headers, None)?;

            let response = self
                .entity_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }

    pub fn to_transaction_operation(&self) -> azure_core::Result<TransactionOperation> {
        let url = self.entity_client.url();

        let mut request = Request::new(url.clone(), Method::Delete);
        request.insert_header("Accept", "application/json;odata=minimalmetadata");
        request.insert_header("If-Match", "*");

        request.set_body("");

        Ok(TransactionOperation::new(request))
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<DeleteEntityResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteEntityBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteEntityResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl TryFrom<Response> for DeleteEntityResponse {
    type Error = Error;

    fn try_from(response: Response) -> azure_core::Result<Self> {
        Ok(DeleteEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
