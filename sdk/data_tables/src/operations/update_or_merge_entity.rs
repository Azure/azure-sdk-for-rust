use crate::{operations::*, prelude::*, IfMatchCondition};
use azure_core::{headers::*, prelude::*, CollectedResponse, Method};
use bytes::Bytes;
use std::convert::TryInto;

operation! {
    UpdateOrMergeEntity,
    client: EntityClient,
    body: Bytes,
    if_match_condition: IfMatchCondition,
    operation: UpdateOperation,
    ?timeout: Timeout
}

impl UpdateOrMergeEntityBuilder {
    pub fn into_future(mut self) -> UpdateOrMergeEntity {
        Box::pin(async move {
            let url = self.client.url().clone();

            let mut headers = Headers::new();
            headers.insert(CONTENT_TYPE, "application/json");
            headers.add(self.if_match_condition);

            let mut request = self.client.finalize_request(
                url,
                match self.operation {
                    UpdateOperation::Merge => Method::Merge,
                    UpdateOperation::Update => Method::Put,
                },
                headers,
                Some(self.body),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let collected_response = CollectedResponse::from_response(response).await?;
            collected_response.try_into()
        })
    }
}

type UpdateOrMergeEntityResponse = OperationOnEntityResponse;
