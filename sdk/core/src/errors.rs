use std::cmp::PartialEq;
use std::fmt::Debug;

/// A specialized `Result` type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// An error caused by an HTTP header.
#[derive(Debug, thiserror::Error)]
pub enum HttpHeaderError {
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("invalid header name")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error("to str error")]
    ToStr(#[source] http::header::ToStrError),
}

/// A general Azure error type.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse error")]
    Parse(#[from] ParseError),
    #[error("error getting token")]
    GetToken(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("header error")]
    Header(#[from] HttpHeaderError),
    #[error("header not found: {0}")]
    HeaderNotFound(String),
    #[error("at least one of these headers must be present: {0:?}")]
    HeadersNotFound(Vec<String>),
    #[error("error preparing HTTP request")]
    HttpPrepare(#[source] http::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("authorization policy error")]
    AuthorizationPolicy(String),
    #[error("Other error")]
    Other(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<super::error::Error> for Error {
    fn from(err: super::error::Error) -> Self {
        match err.into_downcast() {
            Ok(e) => e,
            Err(e) => Self::Other(Box::new(e)),
        }
    }
}

/// An error caused by a failure to parse data.
#[non_exhaustive]
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseError {
    #[error("unknown variant of {item} found: \"{variant}\"")]
    UnknownVariant { item: &'static str, variant: String },
    #[error("expected token \"{token}\" not found when parsing {item} from \"{full}\"")]
    TokenNotFound {
        item: &'static str,
        token: String,
        full: String,
    },
    #[error("error parsing int")]
    Int(#[from] std::num::ParseIntError),
    #[error("error parsing uuid")]
    Uuid(#[from] uuid::Error),
    #[error("error parsing date time")]
    DateTime(#[from] chrono::ParseError),
    #[error("error parsing a float")]
    Float(#[from] std::num::ParseFloatError),
    #[error("error parsing bool")]
    Bool(#[from] std::str::ParseBoolError),
}

/// An unexpected value.
#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedValue {
    expected: Vec<String>,
    received: String,
}

impl UnexpectedValue {
    pub fn new(expected: String, received: String) -> Self {
        Self {
            expected: vec![expected],
            received,
        }
    }

    pub fn new_multiple(allowed: Vec<String>, received: String) -> Self {
        Self {
            expected: allowed,
            received,
        }
    }
}

/// An error caused by invalid permissions.
#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("Permission token not supported in this service ({}). Received token {}, supported tokens {:?}",
        service, received_token, supported_tokens)]
    NonSupportedToken {
        service: String,
        received_token: char,
        supported_tokens: Vec<char>,
    },
}

/// An error caused by failure to traverse a data structure.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum TraversingError {
    #[error("Path not found: {0}")]
    PathNotFound(String),
    #[error("Multiple node: {0}")]
    MultipleNode(String),
    #[error("Enumeration not matched: {0}")]
    EnumerationNotMatched(String),
    #[error("input string cannot be converted in boolean: {0}")]
    BooleanNotMatched(String),
    #[error("DateTime parse error")]
    DateTimeParse(#[from] chrono::format::ParseError),
    #[error("text not found")]
    TextNotFound,
    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse error")]
    Parse(#[from] ParseError),
}

#[cfg(test)]
mod test {
    use super::*;

    fn send_fn<T>(_t: T)
    where
        T: Send,
    {
    }

    fn sync_fn<T>(_t: T)
    where
        T: Sync,
    {
    }

    fn error_generator() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_azure_error_send() {
        error_generator().map_err(send_fn).unwrap();
    }

    #[test]
    fn test_azure_error_sync() {
        error_generator().map_err(sync_fn).unwrap();
    }
}
