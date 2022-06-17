use crate::container::Container;
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::REQUEST_ID,
    RequestId,
};
use chrono::{DateTime, FixedOffset};
use http::{header, HeaderMap};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub container: Container,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}

impl TryFrom<(&str, &HeaderMap)> for GetPropertiesResponse {
    type Error = crate::Error;

    fn try_from((body, header_map): (&str, &HeaderMap)) -> azure_core::Result<Self> {
        GetPropertiesResponse::from_response(body, header_map)
    }
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(
        container_name: &str,
        headers: &HeaderMap,
    ) -> azure_core::Result<GetPropertiesResponse> {
        let request_id = match headers.get(REQUEST_ID) {
            Some(request_id) => {
                Uuid::parse_str(request_id.to_str().map_kind(ErrorKind::DataConversion)?)
                    .map_kind(ErrorKind::DataConversion)?
            }
            None => return Err(Error::message(ErrorKind::DataConversion, REQUEST_ID)),
        };

        let date = match headers.get(header::DATE) {
            Some(date) => {
                DateTime::parse_from_rfc2822(date.to_str().map_kind(ErrorKind::DataConversion)?)
                    .map_kind(ErrorKind::DataConversion)?
            }
            None => {
                static D: header::HeaderName = header::DATE;
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
