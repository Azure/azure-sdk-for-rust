use crate::clients::PathClient;
use crate::request_options::*;
use azure_core::prelude::*;
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

/// A future of a delete file response
type PutPath = futures::future::BoxFuture<'static, crate::Result<DeletePathResponse>>;

#[derive(Debug, Clone)]
pub struct DeletePathBuilder<C>
where
    C: PathClient,
{
    client: C,
    recursive: Option<Recursive>,
    continuation: Option<NextMarker>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    context: Context,
}

impl<C: PathClient + 'static> DeletePathBuilder<C> {
    pub(crate) fn new(client: C, recursive: Option<Recursive>, context: Context) -> Self {
        Self {
            client,
            recursive,
            continuation: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            timeout: None,
            context,
        }
    }

    setters! {
        recursive: Recursive => Some(recursive),
        continuation: NextMarker => Some(continuation),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(self) -> PutPath {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.recursive.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = this
                .client
                .prepare_request(url.as_str(), http::Method::DELETE);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
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
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}
