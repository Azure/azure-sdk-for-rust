use crate::{blob::operations::put_block::PutBlockResponse, prelude::*};
use azure_core::{headers::*, prelude::*, Body};

operation! {
    AppendBlock,
    client: BlobClient,
    body: Body,
    ?hash: Hash,
    ?condition_max_size: ConditionMaxSize,
    ?condition_append_position: ConditionAppendPosition,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
    ?lease_id: LeaseId
}

impl AppendBlockBuilder {
    pub fn into_future(mut self) -> AppendBlock {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("comp", "appendblock");

            let mut headers = Headers::new();
            headers.add(self.hash);
            headers.add(self.condition_max_size);
            headers.add(self.condition_append_position);
            headers.add(self.if_modified_since);
            headers.add(self.if_match);
            headers.add(self.if_tags);
            headers.add(self.lease_id);

            let mut request = BlobClient::finalize_request(
                url,
                azure_core::Method::Put,
                headers,
                Some(self.body),
            )?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            PutBlockResponse::from_headers(response.headers())
        })
    }
}

type AppendBlockResponse = PutBlockResponse;
