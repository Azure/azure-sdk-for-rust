use crate::prelude::*;
use azure_core::{headers::Headers, prelude::*, Method};

operation! {
    Delete,
    client: ContainerClient,
    ?lease_id: LeaseId,
    ?if_modified_since: IfModifiedSinceCondition
}

impl DeleteBuilder {
    pub fn into_future(mut self) -> Delete {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");

            let mut headers = Headers::new();
            headers.add(self.lease_id);
            headers.add(self.if_modified_since);

            let mut request =
                ContainerClient::finalize_request(url, Method::Delete, headers, None)?;

            let _response = self.client.send(&mut self.context, &mut request).await?;

            // TODO: Capture and return the response headers
            Ok(())
        })
    }
}

type DeleteResponse = ();
