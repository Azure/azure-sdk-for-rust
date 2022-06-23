use crate::error::{Error, ErrorKind};
use std::ops::Deref;

#[derive(PartialEq, Clone, Debug)]
pub struct StatusCode(pub(crate) http_types::StatusCode);

impl Deref for StatusCode {
    type Target = http_types::StatusCode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        self.0 as u16
    }
    pub fn from_u16(value: u16) -> crate::Result<Self> {
        Ok(StatusCode(
            http_types::StatusCode::try_from(value).map_err(|_error| {
                Error::with_message(ErrorKind::DataConversion, || {
                    format!("unable to convert {value} to StatusCode")
                })
            })?,
        ))
    }

    pub const BAD_GATEWAY: StatusCode = StatusCode(http_types::StatusCode::BadGateway);
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(http_types::StatusCode::GatewayTimeout);
    pub const INTERNAL_SERVER_ERROR: StatusCode =
        StatusCode(http_types::StatusCode::InternalServerError);
    pub const NOT_MODIFIED: StatusCode = StatusCode(http_types::StatusCode::NotModified);
    pub const OK: StatusCode = StatusCode(http_types::StatusCode::Ok);
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(http_types::StatusCode::RequestTimeout);
    pub const SERVICE_UNAVAILABLE: StatusCode =
        StatusCode(http_types::StatusCode::ServiceUnavailable);
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(http_types::StatusCode::TooManyRequests);
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        StatusCode::OK
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = Error;
    fn try_from(value: u16) -> crate::Result<Self> {
        Ok(StatusCode(
            http_types::StatusCode::try_from(value).map_err(|error| {
                Error::full(
                    ErrorKind::DataConversion,
                    error,
                    "StatusCode::try_from failed",
                )
            })?,
        ))
    }
}
