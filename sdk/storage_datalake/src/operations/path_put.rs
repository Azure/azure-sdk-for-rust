use crate::{clients::PathClient, request_options::*, Properties};
use azure_core::{
    headers::{etag_from_headers, last_modified_from_headers},
    prelude::*,
    AppendToUrlQuery, Request, Response,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    PutPath<C: PathClient + 'static>,
    client: C,
    ?mode: PathRenameMode,
    ?resource: ResourceType,
    ?continuation: NextMarker,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?properties: Properties,
}

impl<C: PathClient + 'static> PutPathBuilder<C> {
    pub fn into_future(self) -> PutPath {
        Box::pin(async move {
            let mut url = self.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.resource.append_to_url_query(&mut url);
            self.mode.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Put);

            request.insert_headers(&self.properties);
            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            request.insert_headers(&ContentLength::new(0));

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            PutPathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct PutPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub continuation: Option<NextMarker>,
}

impl PutPathResponse {
    pub async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}
