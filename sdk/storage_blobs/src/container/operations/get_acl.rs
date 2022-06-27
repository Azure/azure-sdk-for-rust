use crate::{
    container::{public_access_from_header, PublicAccess},
    prelude::*,
};
use azure_core::{
    collect_pinned_stream,
    error::{ErrorKind, ResultExt},
    headers::{self, Headers, REQUEST_ID},
    prelude::*,
    RequestId,
};
use azure_storage::core::StoredAccessPolicyList;
use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use http::Method;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetACLBuilder {
    container_client: ContainerClient,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl GetACLBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            context: Context::new(),
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::GET, None)?;
            request.add_optional_header(&self.lease_id);

            let response = self
                .container_client
                .send(&mut self.context, &mut request)
                .await?;

            let (_, headers, body) = response.deconstruct();
            let body = collect_pinned_stream(body).await?;

            // todo: parse SAS policies
            (body, headers).try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetACLResponse {
    pub public_access: PublicAccess,
    pub etag: String,
    pub last_modified: DateTime<FixedOffset>,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
    pub stored_access_policy_list: StoredAccessPolicyList,
}

impl TryFrom<(Bytes, Headers)> for GetACLResponse {
    type Error = crate::Error;

    fn try_from((body, header_map): (Bytes, Headers)) -> azure_core::Result<Self> {
        GetACLResponse::from_response(body, header_map)
    }
}

impl GetACLResponse {
    // this should be named into and be consuming
    pub(crate) fn from_response(
        body: Bytes,
        headers: Headers,
    ) -> azure_core::Result<GetACLResponse> {
        let public_access = public_access_from_header(&headers)?;

        let etag = headers.get_as_str_or_err(&headers::ETAG)?;

        let last_modified = headers.get_as_str_or_err(&headers::LAST_MODIFIED)?;
        let last_modified =
            DateTime::parse_from_rfc2822(last_modified).map_kind(ErrorKind::DataConversion)?;

        let request_id = headers.get_as(&REQUEST_ID)?;

        let date = headers.get_as_str_or_err(&headers::DATE)?;
        let date = DateTime::parse_from_rfc2822(date).map_kind(ErrorKind::DataConversion)?;

        let stored_access_policy_list =
            StoredAccessPolicyList::from_xml(&body).map_kind(ErrorKind::DataConversion)?;

        Ok(GetACLResponse {
            public_access,
            etag: etag.to_owned(),
            last_modified,
            request_id,
            date,
            stored_access_policy_list,
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetACLResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetACLBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
