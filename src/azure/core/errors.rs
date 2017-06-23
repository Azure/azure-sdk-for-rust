use hyper;
use hyper::StatusCode;
use chrono;
use std::io::Error as IOError;
use std::io::Read;
use std::num;
use xml::BuilderError as XMLError;
use url::ParseError as URLParseError;
use azure::core::enumerations::ParsingError;
use azure::core::range::ParseError;
use serde_json;
use futures::Future;
use futures::Stream;
use std::str;
use std::str::from_utf8;
use futures::future::*;

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

quick_error! {
    #[derive(Debug)]
    pub enum AzureError {
        JSONError(err: serde_json::Error) {
            from()
            display("json error: {}", err)
            cause(err)
        }
        HyperError(err: hyper::error::Error){
            from()
            display("Hyper error: {}", err)
            cause(err)
        }
        IOError(err: IOError){
            from()
            display("IO error: {}", err)
            cause(err)
        }
        XMLError(err: XMLError){
            from()
            display("XML error: {}", err)
            cause(err)
        }
        UnexpectedHTTPResult(err: UnexpectedHTTPResult){
            from()
            display("UnexpectedHTTPResult error")
        }
        HeaderNotFound(msg: String) {
            display("Header not found: {}", msg)
        }
        ResponseParsingError(err: TraversingError){
            from()
            display("Traversing error: {}", err)
            cause(err)
        }
        ParseIntError(err: num::ParseIntError){
            from()
            display("Parse int error: {}", err)
            cause(err)
        }
        ParseError(err: ParseError){
            from()
            display("Parse error")
        }
        GenericError
        ParsingError(err: ParsingError){
            from()
            display("Parsing error")
        }
        InputParametersError(msg: String) {
            display("Input parameters error: {}", msg)
        }
        URLParseError(err: URLParseError){
            from()
            display("URL parse error: {}", err)
            cause(err)
        }
        URIParseError(err: hyper::error::UriError) {
            from()
            display("URI parse error: {}", err)
            cause(err)
        }
        ChronoParserError(err: chrono::ParseError) {
            from()
            display("Chrono parser error: {}", err)
            cause(err)
        }
        UTF8Error(err: str::Utf8Error) {
            from()
            display("UTF8 conversion error: {}", err)
            cause(err)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TraversingError {
        PathNotFound(msg: String) {
            display("Path not found: {}", msg)
        }
        MultipleNode(msg: String) {
            display("Multiple node: {}", msg)
        }
        EnumerationNotMatched(msg: String) {
            display("Enumeration not matched: {}", msg)
        }
        DateTimeParseError(err: chrono::format::ParseError){
            from()
            display("DateTime parse error: {}", err)
            cause(err)
        }
        TextNotFound
        ParseIntError(err: num::ParseIntError){
            from()
            display("Parse int error: {}", err)
            cause(err)
        }
        GenericParseError(msg: String) {
            display("Generic parse error: {}", msg)
        }
        ParsingError(err: ParsingError){
            from()
            display("Parsing error")
        }
   }
}

impl From<()> for AzureError {
    fn from(_: ()) -> AzureError {
        AzureError::GenericError
    }
}

#[inline]
pub fn extract_status_and_body(
    resp: hyper::client::FutureResponse,
) -> impl Future<Item = (hyper::StatusCode, String), Error = AzureError> {
    resp.from_err().and_then(|res| {
        let status = res.status();
        res.body().concat2().from_err().and_then(
            move |whole_body| {
                match str::from_utf8(&whole_body) {
                    Ok(s_body) => ok((status, s_body.to_owned())),
                    Err(error) => err(AzureError::UTF8Error(error)),
                }
            },
        )
    })
}

#[inline]
pub fn check_status_extract_body(
    resp: hyper::client::FutureResponse,
    expected_status_code: hyper::StatusCode,
) -> impl Future<Item = String, Error = AzureError> {
    extract_status_and_body(resp).and_then(
        move |(status, body)| if status == expected_status_code {
            ok(body)
        } else {
            err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult {
                expected: expected_status_code,
                received: status,
                body: body,
            }))
        },
    )
}
