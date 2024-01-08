use crate::clients::PathClient;
use crate::request_options::*;
use azure_core::{prelude::*, Pageable, Request};
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
    pub fn into_stream(self) -> Pageable<DeletePathResponse, azure_core::error::Error> {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();

            async move {
                let mut url = this.client.url()?;

                let continuation = continuation.or_else(|| this.continuation.clone());
                if let Some(continuation) = continuation {
                    continuation.append_to_url_query_as_continuation(&mut url);
                };
                this.recursive.append_to_url_query(&mut url);

                let mut request = Request::new(url, azure_core::Method::Delete);

                request.insert_headers(&this.if_match_condition);
                request.insert_headers(&this.if_modified_since);
                let response = this.client.send(&mut ctx, &mut request).await?;
                DeletePathResponse::try_from(response)
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct DeletePathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub continuation: Option<NextMarker>,
}

impl DeletePathResponse {
    pub fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}

impl Continuable for DeletePathResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation.clone()
    }
}
