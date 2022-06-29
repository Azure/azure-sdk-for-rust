use crate::{operations::*, prelude::*, TransactionOperation};
use azure_core::{headers::*, prelude::*, CollectedResponse, Method, Request};
use bytes::Bytes;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct InsertOrReplaceOrMergeEntityBuilder {
    entity_client: EntityClient,
    body: Bytes,
    operation: InsertOperation,
    timeout: Option<Timeout>,
    context: Context,
}

impl InsertOrReplaceOrMergeEntityBuilder {
    pub(crate) fn new(
        entity_client: EntityClient,
        body: Bytes,
        operation: InsertOperation,
    ) -> Self {
        Self {
            entity_client,
            body,
            operation,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let mut url = self.entity_client.url().clone();

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(CONTENT_TYPE, "application/json");

            let mut request = self.entity_client.finalize_request(
                url,
                match self.operation {
                    InsertOperation::InsertOrMerge => Method::Merge,
                    InsertOperation::InsertOrReplace => Method::Put,
                },
                headers,
                Some(self.body),
            )?;

            let response = self
                .entity_client
                .send(&mut self.context, &mut request)
                .await?;

            let collected_response = CollectedResponse::from_response(response).await?;

            collected_response.try_into()
        })
    }

    pub fn to_transaction_operation(self) -> azure_core::Result<TransactionOperation> {
        let url = self.entity_client.url();

        let mut request = Request::new(
            url.clone(),
            match self.operation {
                InsertOperation::InsertOrMerge => Method::Merge,
                InsertOperation::InsertOrReplace => Method::Put,
            },
        );
        request.insert_header(ACCEPT, "application/json;odata=fullmetadata");
        request.insert_header(CONTENT_TYPE, "application/json");
        request.set_body(self.body);

        Ok(TransactionOperation::new(request))
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<OperationOnEntityResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for InsertOrReplaceOrMergeEntityBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
