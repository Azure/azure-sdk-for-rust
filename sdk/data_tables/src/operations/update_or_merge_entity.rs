use crate::{operations::*, prelude::*, IfMatchCondition, TransactionOperation};
use azure_core::{prelude::*, CollectedResponse, Context, Method, Request};
use bytes::Bytes;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct UpdateOrMergeEntityBuilder {
    entity_client: EntityClient,
    body: Bytes,
    if_match_condition: IfMatchCondition,
    operation: UpdateOperation,
    #[allow(unused)]
    timeout: Option<Timeout>,
    context: Context,
}

impl UpdateOrMergeEntityBuilder {
    pub(crate) fn new(
        entity_client: EntityClient,
        body: Bytes,
        if_match_condition: IfMatchCondition,
        operation: UpdateOperation,
    ) -> Self {
        Self {
            entity_client,
            body,
            if_match_condition,
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
            let url = self.entity_client.url().clone();

            let mut request = self.entity_client.prepare_request(
                url,
                match self.operation {
                    UpdateOperation::Merge => crate::MERGE.to_owned(),
                    UpdateOperation::Update => Method::PUT,
                },
                Some(self.body),
            )?;
            request.insert_header("Content-Type", "application/json");
            request.add_mandatory_header(&self.if_match_condition);

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
                UpdateOperation::Merge => crate::MERGE.to_owned(),
                UpdateOperation::Update => Method::PUT,
            },
        );
        request.insert_header("Accept", "application/json;odata=fullmetadata");
        request.insert_header("Content-Type", "application/json");
        request.add_mandatory_header(&self.if_match_condition);
        request.set_body(self.body);

        Ok(TransactionOperation::new(request))
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<OperationOnEntityResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for UpdateOrMergeEntityBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
