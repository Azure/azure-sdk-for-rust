use hyper;
use hyper::status::StatusCode;
use chrono;
// use std::io;
use std::io::Read;
// use xml;

#[derive(Debug)]
pub enum AzureError {
    HyperError(hyper::error::Error),
    IOError(String),
    XMLError(String),
    UnexpectedResult((StatusCode, StatusCode, String)),
    ResponseParsingError(TraversingError),
}

#[derive(Debug)]
pub enum TraversingError {
    PathNotFound(String),
    MultipleNode(String),
    EnumerationNotMatched(String),
    DateTimeParseError(chrono::format::ParseError),
    TextNotFound,
}

impl From<hyper::error::Error> for AzureError {
    fn from(he: hyper::error::Error) -> AzureError {
        AzureError::HyperError(he)
    }
}

impl From<chrono::format::ParseError> for AzureError {
    fn from(pe: chrono::format::ParseError) -> AzureError {
        AzureError::ResponseParsingError(TraversingError::DateTimeParseError(pe))
    }
}

impl From<TraversingError> for AzureError {
    fn from(te: TraversingError) -> AzureError {
        AzureError::ResponseParsingError(te)
    }
}


pub fn new_from_xmlerror_string(s: String) -> AzureError {
    AzureError::XMLError(s)
}

pub fn new_from_ioerror_string(s: String) -> AzureError {
    AzureError::IOError(s)
}

#[inline]
pub fn check_status(resp: &mut hyper::client::response::Response,
                    s: StatusCode)
                    -> Result<(), AzureError> {
    if resp.status != s {
        let mut resp_s = String::new();
        match resp.read_to_string(&mut resp_s) {
            Ok(_) => (),
            Err(err) => return Err(new_from_ioerror_string(err.to_string())),
        };

        return Err(AzureError::UnexpectedResult((resp.status, s, resp_s)));
    }

    Ok(())
}
