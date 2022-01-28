use super::{Error, ErrorKind};

impl From<crate::errors::Error> for Error {
    fn from(err: crate::errors::Error) -> Error {
        match err {
            crate::errors::Error::Json(e) => e.into(),
            crate::errors::Error::Header(e) => Error::new(ErrorKind::DataConversion, e),
            crate::errors::Error::Parse(e) => Error::new(ErrorKind::DataConversion, e),
            crate::errors::Error::HeadersNotFound(hs) => Error::with_message(
                ErrorKind::DataConversion,
                format!("headers not found: {}", hs.join(", ")),
            ),
            crate::errors::Error::HeaderNotFound(h) => Error::with_message(
                ErrorKind::DataConversion,
                format!("header not found: {}", h),
            ),
            crate::errors::Error::Http(e) => e.into(),
            crate::errors::Error::Stream(e) => Error::new(ErrorKind::Io, e),
            crate::errors::Error::GetToken(e) => Error::new(ErrorKind::Credential, e),
            crate::errors::Error::HttpPrepare(e) => e.into(),
            crate::errors::Error::Other(e) => Error::new(ErrorKind::Other, e),
        }
    }
}

impl From<crate::errors::HttpError> for Error {
    fn from(err: crate::errors::HttpError) -> Error {
        match err {
            crate::HttpError::StatusCode { status, ref body } => Error::new(
                ErrorKind::http_response_from_body(status.as_u16(), body),
                err,
            ),
            crate::HttpError::ExecuteRequest(e) => Error::new(ErrorKind::Io, e),
            crate::HttpError::ReadBytes(e) => Error::new(ErrorKind::Io, e),
            crate::HttpError::StreamReset(e) => Error::new(ErrorKind::Io, e),
            crate::HttpError::Utf8(e) => Error::new(ErrorKind::DataConversion, e),
            crate::HttpError::BuildResponse(e) => Error::new(ErrorKind::DataConversion, e),
            crate::HttpError::BuildClientRequest(e) => Error::new(ErrorKind::Other, e),
        }
    }
}
