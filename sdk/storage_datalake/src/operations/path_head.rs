use crate::{clients::PathClient, request_options::*, Properties};
use azure_core::headers::{etag_from_headers, last_modified_from_headers};
use azure_core::prelude::IfMatchCondition;
use azure_core::prelude::LeaseId;
use azure_core::prelude::{ClientRequestId, Context, IfModifiedSinceCondition, Timeout};
use azure_core::{headers::add_optional_header2, AppendToUrlQuery, Response as HttpResponse};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use chrono::{DateTime, Utc};
use std::convert::TryInto;

/// A future of a delete file response
type HeadPath = futures::future::BoxFuture<'static, crate::Result<HeadPathResponse>>;

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

            let mut request = this
                .client
                .prepare_request(url.as_str(), http::Method::HEAD);

            add_optional_header2(&this.client_request_id, &mut request)?;
            add_optional_header2(&this.if_match_condition, &mut request)?;
            add_optional_header2(&this.if_modified_since, &mut request)?;
            add_optional_header2(&this.lease_id, &mut request)?;

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            HeadPathResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct HeadPathResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub properties: Properties,
}

impl HeadPathResponse {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::Error> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        Ok(Self {
            common_storage_response_headers: (&headers).try_into()?,
            etag: etag_from_headers(&headers)?,
            last_modified: last_modified_from_headers(&headers)?,
            properties: (&headers).try_into()?,
        })
    }
}
