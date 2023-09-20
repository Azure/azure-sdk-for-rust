use crate::{operations::*, prelude::*};
use azure_core::{headers::*, prelude::*, Body, CollectedResponse, Method};
use std::convert::TryInto;

operation! {
    InsertOrReplaceOrMergeEntity,
    client: EntityClient,
    body: Body,
    operation: InsertOperation,
}

impl InsertOrReplaceOrMergeEntityBuilder {
    pub fn into_future(mut self) -> InsertOrReplaceOrMergeEntity {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.add(ContentType::APPLICATION_JSON);

            let mut request = EntityClient::finalize_request(
                url,
                match self.operation {
                    InsertOperation::InsertOrMerge => Method::Merge,
                    InsertOperation::InsertOrReplace => Method::Put,
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

type InsertOrReplaceOrMergeEntityResponse = OperationOnEntityResponse;
