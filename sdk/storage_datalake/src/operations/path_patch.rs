use crate::clients::PathClient;
use crate::request_options::*;
use crate::Properties;
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::*;
use azure_core::Request;
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    PatchPath<C: PathClient + 'static>,
    client: C,
    action: PathUpdateAction,
    ?acl: AccessControlList,
    ?close: Close,
    ?continuation: NextMarker,
    ?position: Position,
    ?retain_uncommitted_data: RetainUncommittedData,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?properties: Properties,
    ?bytes: Bytes,
}

impl<C: PathClient + 'static> PatchPathBuilder<C> {
    pub fn into_future(self) -> PatchPath {
        Box::pin(async move {
            let mut url = self.client.url()?;

            if let Some(continuation) = self.continuation {
                continuation.append_to_url_query_as_continuation(&mut url);
            };
            self.action.append_to_url_query(&mut url);
            self.close.append_to_url_query(&mut url);
            self.position.append_to_url_query(&mut url);
            self.retain_uncommitted_data.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Patch);

            request.insert_headers(&self.acl);
            request.insert_headers(&self.properties);
            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);

            if let Some(bytes) = self.bytes {
                request.insert_headers(&ContentLength::new(bytes.len() as i32));
                request.insert_headers(&ContentType::new("application/octet-stream"));
                request.set_body(bytes);
            } else {
                request.insert_headers(&ContentLength::new(0));
            }

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            PatchPathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct PatchPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: Option<String>,
    pub last_modified: Option<OffsetDateTime>,
    pub continuation: Option<NextMarker>,
}

impl PatchPathResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
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
