use super::{Error, ErrorKind};

use http::header::{InvalidHeaderName, InvalidHeaderValue};
use http::method::InvalidMethod;
use http::status::InvalidStatusCode;
use http::uri::{InvalidUri, InvalidUriParts};

impl<O: Copy> From<http::Error> for super::Error<O> {
    fn from(err: http::Error) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidHeaderName> for super::Error<O> {
    fn from(err: InvalidHeaderName) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidHeaderValue> for super::Error<O> {
    fn from(err: InvalidHeaderValue) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidMethod> for super::Error<O> {
    fn from(err: InvalidMethod) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidStatusCode> for super::Error<O> {
    fn from(err: InvalidStatusCode) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidUri> for super::Error<O> {
    fn from(err: InvalidUri) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}

impl<O: Copy> From<InvalidUriParts> for super::Error<O> {
    fn from(err: InvalidUriParts) -> super::Error<O> {
        Error::new(ErrorKind::DataConversion, err)
    }
}
