use crate::clients::PathClient;
use crate::request_options::*;
use crate::Properties;
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::*;
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
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
    bytes: Option<Bytes>,
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
            bytes: None,
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
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        properties: Properties => Some(properties),
        bytes: Bytes => Some(bytes),
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

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.properties);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);

            if let Some(bytes) = this.bytes {
                request.insert_headers(&ContentLength::new(bytes.len() as i32));
                request.insert_headers(&ContentType::new("application/octet-stream"));
                request.set_body(bytes)
            } else {
                request.insert_headers(&ContentLength::new(0));
            }

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
    pub etag: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
    pub continuation: Option<NextMarker>,
}

impl PatchPathResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let etag = match etag_from_headers(&headers) {
            Ok(tag) => Some(tag),
            _ => None,
        };

        let last_modified = match last_modified_from_headers(&headers) {
            Ok(modified) => Some(modified),
            _ => None,
        };

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag,
            last_modified,
            continuation: NextMarker::from_header_optional(&headers)?,
        })
    }
}
