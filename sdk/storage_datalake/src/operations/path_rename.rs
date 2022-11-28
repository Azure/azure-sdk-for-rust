use crate::{clients::PathClient, request_options::*, Properties};
use azure_core::{prelude::*, AppendToUrlQuery, Request};

operation! {
    RenamePath<C: PathClient + 'static>,
    client: C,
    ?mode: PathRenameMode,
    ?continuation: NextMarker,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?rename_source: RenameSource,
    ?properties: Properties,
}

impl<C: PathClient + 'static> RenamePathBuilder<C> {
    pub fn into_future(self) -> RenamePath {
        let mut ctx = self.context.clone();

        Box::pin(async move {
            let mut url = self.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.mode.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Put);

            request.insert_headers(&self.properties);
            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            request.insert_headers(&self.rename_source);
            request.insert_headers(&ContentLength::new(0));

            self.client.send(&mut ctx, &mut request).await?;

            Ok(())
        })
    }
}

type RenamePathResponse = ();
