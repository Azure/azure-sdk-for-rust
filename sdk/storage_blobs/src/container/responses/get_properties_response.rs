use crate::container::Container;
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{self, Headers, REQUEST_ID},
    RequestId,
};
use chrono::{DateTime, FixedOffset};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub container: Container,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}

impl TryFrom<(&str, &Headers)> for GetPropertiesResponse {
    type Error = crate::Error;

    fn try_from((body, header_map): (&str, &Headers)) -> azure_core::Result<Self> {
        GetPropertiesResponse::from_response(body, header_map)
    }
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(
        container_name: &str,
        headers: &Headers,
    ) -> azure_core::Result<GetPropertiesResponse> {
        let request_id = headers.get_as_str_or_err(&REQUEST_ID)?;
        let request_id = Uuid::parse_str(request_id).map_kind(ErrorKind::DataConversion)?;

        let date = match headers.get(&headers::DATE) {
            Some(date) => {
                DateTime::parse_from_rfc2822(date.as_str()).map_kind(ErrorKind::DataConversion)?
            }
            None => {
                static D: headers::HeaderName = headers::DATE;
                return Err(Error::message(ErrorKind::DataConversion, D.as_str()));
            }
        };

        let container = Container::from_response(container_name, headers)?;

        Ok(GetPropertiesResponse {
            container,
            request_id,
            date,
        })
    }
}
