use crate::{
    container::{public_access_from_header, PublicAccess},
    prelude::*,
};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{Headers, REQUEST_ID},
    prelude::*,
    RequestId,
};
use azure_storage::core::StoredAccessPolicyList;
use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use http::{header, method::Method};
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetACLBuilder {
    container_client: ContainerClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
}

impl GetACLBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");
            url.query_pairs_mut().append_pair("comp", "acl");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::GET, None)?;
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);

            let response = self
                .container_client
                .storage_client()
                .storage_account_client()
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            // todo: parse SAS policies
            (response.body(), response.headers()).try_into()
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

impl TryFrom<(&Bytes, &Headers)> for GetACLResponse {
    type Error = crate::Error;

    fn try_from((body, header_map): (&Bytes, &Headers)) -> azure_core::Result<Self> {
        GetACLResponse::from_response(body, header_map)
    }
}

impl GetACLResponse {
    // this should be named into and be consuming
    pub(crate) fn from_response(
        body: &Bytes,
        headers: &Headers,
    ) -> azure_core::Result<GetACLResponse> {
        let public_access = public_access_from_header(headers)?;

        let etag = match headers.get(header::ETAG) {
            Some(etag) => etag.as_str(),
            None => {
                static E: header::HeaderName = header::ETAG;
                return Err(Error::message(ErrorKind::DataConversion, E.as_str()));
            }
        };

        let last_modified = match headers.get(header::LAST_MODIFIED) {
            Some(last_modified) => last_modified.as_str(),
            None => {
                static LM: header::HeaderName = header::LAST_MODIFIED;
                return Err(Error::message(ErrorKind::DataConversion, LM.as_str()));
            }
        };
        let last_modified =
            DateTime::parse_from_rfc2822(last_modified).map_kind(ErrorKind::DataConversion)?;

        let request_id = match headers.get(REQUEST_ID) {
            Some(request_id) => request_id.as_str(),
            None => return Err(Error::message(ErrorKind::DataConversion, REQUEST_ID)),
        };

        let date = match headers.get(header::DATE) {
            Some(date) => date.as_str(),
            None => {
                static D: header::HeaderName = header::DATE;
                return Err(Error::message(ErrorKind::DataConversion, D.as_str()));
            }
        };
        let date = DateTime::parse_from_rfc2822(date).map_kind(ErrorKind::DataConversion)?;

        let stored_access_policy_list =
            StoredAccessPolicyList::from_xml(body).map_kind(ErrorKind::DataConversion)?;

        Ok(GetACLResponse {
            public_access,
            etag: etag.to_owned(),
            last_modified,
            request_id: Uuid::parse_str(request_id).map_kind(ErrorKind::DataConversion)?,
            date,
            stored_access_policy_list,
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetACLResponse>>;
