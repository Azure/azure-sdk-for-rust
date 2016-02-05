use hyper;
use hyper::status::StatusCode;
use chrono;
use std::io::Error as IOError;
use xml::BuilderError as XMLError;
use std::io::Read;
use std::num;
// use xml;
use url::ParseError as URLParseError;
use azure::core::enumerations::ParsingError;
use azure::core::range::ParseError;

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedHTTPResult {
    expected: StatusCode,
    received: StatusCode,
    body: String,
}

impl UnexpectedHTTPResult {
    pub fn new(expected: StatusCode, received: StatusCode, body: &str) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: expected,
            received: received,
            body: body.to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum AzureError {
    HyperError(hyper::error::Error),
    IOError(IOError),
    XMLError(XMLError),
    UnexpectedHTTPResult(UnexpectedHTTPResult),
    HeaderNotFound(String),
    ResponseParsingError(TraversingError),
    ParseIntError(num::ParseIntError),
    ParseError(ParseError),
    GenericError,
    ParsingError(ParsingError),
    InputParametersError(String),
    URLParseError(URLParseError),
}

#[derive(Debug)]
pub enum TraversingError {
    PathNotFound(String),
    MultipleNode(String),
    EnumerationNotMatched(String),
    DateTimeParseError(chrono::format::ParseError),
    TextNotFound,
    ParseIntError(num::ParseIntError),
    GenericParseError(String),
    ParsingError(ParsingError),
}

impl From<URLParseError> for AzureError {
    fn from(upe: URLParseError) -> AzureError {
        AzureError::URLParseError(upe)
    }
}

impl From<ParseError> for AzureError {
    fn from(pe: ParseError) -> AzureError {
        AzureError::ParseError(pe)
    }
}

impl From<()> for AzureError {
    fn from(_: ()) -> AzureError {
        AzureError::GenericError
    }
}

impl From<hyper::error::Error> for AzureError {
    fn from(he: hyper::error::Error) -> AzureError {
        AzureError::HyperError(he)
    }
}

impl From<ParsingError> for AzureError {
    fn from(pie: ParsingError) -> AzureError {
        AzureError::ParsingError(pie)
    }
}

impl From<XMLError> for AzureError {
    fn from(xmle: XMLError) -> AzureError {
        AzureError::XMLError(xmle)
    }
}

impl From<IOError> for AzureError {
    fn from(ioe: IOError) -> AzureError {
        AzureError::IOError(ioe)
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

impl From<num::ParseIntError> for AzureError {
    fn from(pie: num::ParseIntError) -> AzureError {
        AzureError::ParseIntError(pie)
    }
}

impl From<chrono::format::ParseError> for TraversingError {
    fn from(pe: chrono::format::ParseError) -> TraversingError {
        TraversingError::DateTimeParseError(pe)
    }
}

impl From<num::ParseIntError> for TraversingError {
    fn from(pie: num::ParseIntError) -> TraversingError {
        TraversingError::ParseIntError(pie)
    }
}

impl From<ParsingError> for TraversingError {
    fn from(pie: ParsingError) -> TraversingError {
        TraversingError::ParsingError(pie)
    }
}

#[inline]
pub fn check_status(resp: &mut hyper::client::response::Response,
                    s: StatusCode)
                    -> Result<(), AzureError> {
    if resp.status != s {
        let mut resp_s = String::new();
        try!(resp.read_to_string(&mut resp_s));

        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(s,
                                                                              resp.status,
                                                                              &resp_s)));
    }

    Ok(())
}
