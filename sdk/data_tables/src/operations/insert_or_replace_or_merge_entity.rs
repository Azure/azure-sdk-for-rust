use crate::{operations::*, prelude::*};
use azure_core::{headers::*, prelude::*, CollectedResponse, Method};
use bytes::Bytes;
use std::convert::TryInto;

operation! {
    InsertOrReplaceOrMergeEntity,
    client: EntityClient,
    body: Bytes,
    operation: InsertOperation,
    ?timeout: Timeout
}

impl InsertOrReplaceOrMergeEntityBuilder {
    pub fn into_future(mut self) -> InsertOrReplaceOrMergeEntity {
        Box::pin(async move {
            let mut url = self.client.url().clone();

            self.timeout.append_to_url_query(&mut url);

            let mut headers = Headers::new();
            headers.insert(CONTENT_TYPE, "application/json");

            let mut request = self.client.finalize_request(
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
