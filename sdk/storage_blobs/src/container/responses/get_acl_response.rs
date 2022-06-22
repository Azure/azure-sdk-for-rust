use crate::container::{public_access_from_header, PublicAccess};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{self, Headers, REQUEST_ID},
    RequestId,
};
use azure_storage::core::StoredAccessPolicyList;
use bytes::Bytes;
use chrono::{DateTime, FixedOffset};
use std::convert::TryFrom;
use uuid::Uuid;

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

        let etag = match headers.get(&headers::ETAG) {
            Some(etag) => etag.as_str(),
            None => {
                static E: headers::HeaderName = headers::ETAG;
                return Err(Error::message(ErrorKind::DataConversion, E.as_str()));
            }
        };

        let last_modified = match headers.get(&headers::LAST_MODIFIED) {
            Some(last_modified) => last_modified.as_str(),
            None => {
                static LM: headers::HeaderName = headers::LAST_MODIFIED;
                return Err(Error::message(ErrorKind::DataConversion, LM.as_str()));
            }
        };
        let last_modified =
            DateTime::parse_from_rfc2822(last_modified).map_kind(ErrorKind::DataConversion)?;

        let request_id = headers.get_as_str_or_err(&REQUEST_ID)?;

        let date = match headers.get(&headers::DATE) {
            Some(date) => date.as_str(),
            None => {
                static D: headers::HeaderName = headers::DATE;
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
