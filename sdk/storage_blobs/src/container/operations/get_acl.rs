use crate::{
    container::{public_access_from_header, PublicAccess},
    prelude::*,
};
use azure_core::{date, headers::*, prelude::*, Method, RequestId, Response};
use azure_storage::core::StoredAccessPolicyList;
use time::OffsetDateTime;

operation! {
    GetACL,
    client: ContainerClient,
    ?lease_id: LeaseId
}

impl GetACLBuilder {
    pub fn into_future(mut self) -> GetACL {
        Box::pin(async move {
            let url = self.client.url()?;

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request = self
                .client
                .finalize_request(url, Method::Get, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;
            GetACLResponse::from_response(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetACLResponse {
    pub public_access: PublicAccess,
    pub etag: String,
    pub last_modified: OffsetDateTime,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
    pub stored_access_policy_list: StoredAccessPolicyList,
}

impl GetACLResponse {
    // this should be named into and be consuming
    pub(crate) async fn from_response(response: Response) -> azure_core::Result<GetACLResponse> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        // todo: parse SAS policies
        let public_access = public_access_from_header(&headers)?;

        let etag = headers.get_as(&ETAG)?;

        let last_modified = headers.get_str(&LAST_MODIFIED)?;
        let last_modified = date::parse_rfc1123(last_modified)?;

        let request_id = headers.get_as(&REQUEST_ID)?;

        let date = headers.get_str(&DATE)?;
        let date = date::parse_rfc1123(date)?;

        let stored_access_policy_list = StoredAccessPolicyList::from_xml(&body)?;

        Ok(GetACLResponse {
            public_access,
            etag,
            last_modified,
            request_id,
            date,
            stored_access_policy_list,
        })
    }
}
