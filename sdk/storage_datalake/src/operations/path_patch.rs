use crate::clients::PathClient;
use crate::request_options::*;
use crate::Properties;
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::{
    ClientRequestId, Context, IfMatchCondition, IfModifiedSinceCondition, NextMarker, Timeout, *,
};
use azure_core::{
    headers::{add_mandatory_header2, add_optional_header2},
    AppendToUrlQuery, Response as HttpResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use futures::future::BoxFuture;
use std::convert::TryInto;

/// A future of a patch file response
type PatchPath = BoxFuture<'static, crate::Result<PatchPathResponse>>;

#[derive(Debug, Clone)]
pub struct PatchPathBuilder<C>
where
    C: PathClient,
{
    client: C,
    action: Option<PathUpdateAction>,
    close: Option<Close>,
    continuation: Option<NextMarker>,
    position: Option<Position>,
    retain_uncommitted_data: Option<RetainUncommittedData>,
    timeout: Option<Timeout>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    properties: Option<Properties>,
    context: Context,
}

impl<C: PathClient + 'static> PatchPathBuilder<C> {
    pub(crate) fn new(client: C, context: Context) -> Self {
        Self {
            client,
            action: None,
            close: None,
            continuation: None,
            position: None,
            retain_uncommitted_data: None,
            timeout: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            properties: None,
            context,
        }
    }

    setters! {
        action: PathUpdateAction => Some(action),
        close: Close => Some(close),
        continuation: NextMarker => Some(continuation),
        position: Position => Some(position),
        retain_uncommitted_data: RetainUncommittedData => Some(retain_uncommitted_data),
        timeout: Timeout => Some(timeout),

        // mode: PathRenameMode => Some(mode),
        // resource: ResourceType => Some(resource),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        properties: Properties => Some(properties),

        context: Context => context,
    }

    pub fn into_future(self) -> PatchPath {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.action.append_to_url_query(&mut url);
            self.close.append_to_url_query(&mut url);
            self.position.append_to_url_query(&mut url);
            self.retain_uncommitted_data.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = this
                .client
                .prepare_request(url.as_str(), http::Method::PATCH);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.properties, &mut request)?;
            add_optional_header2(&this.if_match_condition, &mut request)?;
            add_optional_header2(&this.if_modified_since, &mut request)?;

            add_mandatory_header2(&ContentLength::new(0), &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            PatchPathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatchPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub continuation: Option<NextMarker>,
}

impl PatchPathResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}
