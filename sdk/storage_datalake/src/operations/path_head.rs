use crate::{clients::PathClient, request_options::*, Properties};
use azure_core::headers::{self, etag_from_headers, last_modified_from_headers};
use azure_core::{prelude::*, Request};
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

operation! {
    HeadPath<C: PathClient + 'static>,
    client: C,
    ?action: PathGetPropertiesAction,
    ?upn: Upn,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?lease_id: LeaseId,
}

impl<C: PathClient + 'static> HeadPathBuilder<C> {
    pub fn into_future(self) -> HeadPath {
        Box::pin(async move {
            let mut url = self.client.url()?;

            self.action.append_to_url_query(&mut url);
            self.upn.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Head);

            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            request.insert_headers(&self.lease_id);

            let response = self
                .client
                .send(&mut self.context.clone(), &mut request)
                .await?;

            HeadPathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct HeadPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub content_length: Option<i64>,
    pub content_type: Option<String>,
    pub last_modified: OffsetDateTime,
    pub properties: Option<Properties>,
    pub acl: Option<String>,
}

impl HeadPathResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let headers = response.headers();

        Ok(Self {
            common_storage_response_headers: headers.try_into()?,
            etag: etag_from_headers(headers)?,
            last_modified: last_modified_from_headers(headers)?,
            content_length: headers.get_optional_as(&headers::CONTENT_LENGTH)?,
            content_type: headers.get_optional_as(&headers::CONTENT_TYPE)?,
            properties: headers.get_optional_as(&headers::PROPERTIES)?,
            acl: headers.get_optional_string(&headers::ACL),
        })
    }
}
