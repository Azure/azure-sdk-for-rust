use crate::{clients::PathClient, request_options::*, Properties};
use azure_core::headers::{self, etag_from_headers, last_modified_from_headers};
use azure_core::{prelude::*, Request};
use azure_core::{AppendToUrlQuery, Response as HttpResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct HeadPathBuilder<C>
where
    C: PathClient,
{
    client: C,
    action: Option<PathGetPropertiesAction>,
    upn: Option<Upn>,
    timeout: Option<Timeout>,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl<C: PathClient + 'static> HeadPathBuilder<C> {
    pub(crate) fn new(client: C, context: Context) -> Self {
        Self {
            client,
            action: None,
            upn: None,
            timeout: None,
            if_match_condition: None,
            if_modified_since: None,
            client_request_id: None,
            lease_id: None,
            context,
        }
    }

    setters! {
        action: PathGetPropertiesAction => Some(action),
        upn: Upn => Some(upn),
        timeout: Timeout => Some(timeout),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        if_modified_since: IfModifiedSinceCondition => Some(if_modified_since),
        client_request_id: ClientRequestId => Some(client_request_id),
        lease_id: LeaseId => Some(lease_id),
        context: Context => context,
    }

    pub fn into_future(self) -> HeadPath {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let mut url = this.client.url()?;

            self.action.append_to_url_query(&mut url);
            self.upn.append_to_url_query(&mut url);
            self.timeout.append_to_url_query(&mut url);

            let mut request = Request::new(url, azure_core::Method::Head);

            request.insert_headers(&this.client_request_id);
            request.insert_headers(&this.if_match_condition);
            request.insert_headers(&this.if_modified_since);
            request.insert_headers(&this.lease_id);

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            HeadPathResponse::try_from(response).await
        })
    }
}

azure_core::future!(HeadPath);

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
