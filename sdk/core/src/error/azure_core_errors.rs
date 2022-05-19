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
            crate::errors::Error::GetToken(e) => Error::new(ErrorKind::Credential, e),
            crate::errors::Error::HttpPrepare(e) => e.into(),
            crate::errors::Error::Other(e) => Error::new(ErrorKind::Other, e),
            crate::errors::Error::AuthorizationPolicy(msg) => Error::with_message(
                ErrorKind::Credential,
                format!("Failed token acquisition: {}", msg),
            ),
        }
    }
}
