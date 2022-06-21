use super::{Error, ErrorKind};
use http::method::InvalidMethod;
use http::status::InvalidStatusCode;
use http::uri::{InvalidUri, InvalidUriParts};

impl From<http::Error> for super::Error {
    fn from(err: http::Error) -> super::Error {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl From<InvalidMethod> for super::Error {
    fn from(err: InvalidMethod) -> super::Error {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl From<InvalidStatusCode> for super::Error {
    fn from(err: InvalidStatusCode) -> super::Error {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl From<InvalidUri> for super::Error {
    fn from(err: InvalidUri) -> super::Error {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl From<InvalidUriParts> for super::Error {
    fn from(err: InvalidUriParts) -> super::Error {
        Error::new(ErrorKind::DataConversion, err)
    }
}
