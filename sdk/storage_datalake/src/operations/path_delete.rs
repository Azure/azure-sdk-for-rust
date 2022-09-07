use crate::clients::PathClient;
use crate::request_options::*;
use azure_core::{prelude::*, Request};
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

operation! {
    DeletePath<C: PathClient + 'static>,
    client: C,
    ?recursive: Recursive,
    ?continuation: NextMarker,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
}

impl<C: PathClient + 'static> DeletePathBuilder<C> {
    pub fn into_future(self) -> DeletePath {
        Box::pin(async move {
            let mut url = self.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.recursive.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Delete);

            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            DeletePathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeletePathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub continuation: Option<NextMarker>,
}

impl DeletePathResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}
