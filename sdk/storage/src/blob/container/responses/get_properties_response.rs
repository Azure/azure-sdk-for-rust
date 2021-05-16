use crate::{container::Container, AzureStorageError};
use azure_core::headers::REQUEST_ID;
use azure_core::RequestId;
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
    type Error = AzureStorageError;

    fn try_from((body, header_map): (&str, &HeaderMap)) -> Result<Self, Self::Error> {
        GetPropertiesResponse::from_response(body, header_map)
    }
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(
        container_name: &str,
        headers: &HeaderMap,
    ) -> Result<GetPropertiesResponse, AzureStorageError> {
        let request_id = match headers.get(REQUEST_ID) {
            Some(request_id) => Uuid::parse_str(request_id.to_str()?)?,
            None => return Err(AzureStorageError::MissingHeaderError(REQUEST_ID.to_owned())),
        };

        let date = match headers.get(header::DATE) {
            Some(date) => DateTime::parse_from_rfc2822(date.to_str()?)?,
            None => {
                static D: header::HeaderName = header::DATE;
                return Err(AzureStorageError::MissingHeaderError(D.as_str().to_owned()));
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
