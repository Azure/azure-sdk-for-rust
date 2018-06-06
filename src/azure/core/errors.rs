use azure::core::enumerations::ParsingError;
use azure::core::range::ParseError;
use chrono;
use futures::future::*;
use futures::Future;
use futures::Stream;
use hyper;
use hyper::StatusCode;
use native_tls;
use serde_json;
use std;
use std::io::Error as IOError;
use std::num;
use std::str;
use std::string;
use url::ParseError as URLParseError;
use xml::BuilderError as XMLError;

quick_error! {
    #[derive(Debug)]
     pub enum AzurePathParseError {
        PathSeparatorNotFoundError {
            display("Path separator not found")
        }
        MultiplePathSeparatorsFoundError {
            display("Multiple path separators found")
        }
        MissingContainerError {
            display("Missing container name")
        }
        MissingBlobError {
            display("Missing blob name")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedHTTPResult {
    expected: StatusCode,
    received: StatusCode,
    body: String,
}

impl UnexpectedHTTPResult {
    pub fn new(expected: StatusCode, received: StatusCode, body: &str) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected,
            received,
            body: body.to_owned(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.received
    }
}

impl std::fmt::Display for UnexpectedHTTPResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unexpected HTTP result (expected: {}, received: {})",
            self.expected, self.received
        )
    }
}

impl std::error::Error for UnexpectedHTTPResult {
    fn description(&self) -> &str {
        "Unexpected HTTP result"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
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
        AzurePathParseError(err: AzurePathParseError){
            from()
            display("Azure Path parse error: {}", err)
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
        GenericErrorWithText(err: String) {
            display("Generic error: {}", err)
        }
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
        FromUtf8Error(err: string::FromUtf8Error) {
            from()
            display("FromUTF8 error: {}", err)
            cause(err)
        }
        NativeTLSError(err: native_tls::Error) {
            from()
            display("Native TLS error: {}", err)
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
            display("Parsing error: {:?}", err)
        }
        BlockListParseError(err: BlockListParseError){
            from()
            display("Block list XML parsing error: {:?}", err)
        }
   }
}

impl From<()> for AzureError {
    fn from(_: ()) -> AzureError {
        AzureError::GenericError
    }
}

#[derive(Debug, Fail)]
pub enum BlockListParseError {
    #[fail(display = "invalid BlockList XML")]
    InvalidBlockListXML,
    #[fail(display = "Invalid Block type: {}", name)]
    InvalidBlockType { name: String },
    #[fail(display = "Token not found: {}", token)]
    TokemNotFound { token: String },
    #[fail(display = "Gneric parse error")]
    GenericParseError,
}

impl std::convert::From<std::option::NoneError> for BlockListParseError {
    fn from(_: std::option::NoneError) -> Self {
        BlockListParseError::GenericParseError
    }
}

#[inline]
pub fn extract_status_headers_and_body(
    resp: hyper::client::FutureResponse,
) -> impl Future<Item = (hyper::StatusCode, hyper::Headers, Vec<u8>), Error = AzureError> {
    resp.from_err().and_then(|res| {
        let status = res.status();
        let headers = res.headers().clone();
        res.body()
            .concat2()
            .from_err()
            .and_then(move |whole_body| ok((status, headers, Vec::from(&whole_body as &[u8]))))
    })
}

#[inline]
pub fn check_status_extract_headers_and_body(
    resp: hyper::client::FutureResponse,
    expected_status_code: hyper::StatusCode,
) -> impl Future<Item = (hyper::Headers, Vec<u8>), Error = AzureError> {
    extract_status_headers_and_body(resp).and_then(move |(status, headers, body)| {
        if status == expected_status_code {
            ok((headers, body))
        } else {
            match str::from_utf8(&body) {
                Ok(s_body) => err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult {
                    expected: expected_status_code,
                    received: status,
                    body: s_body.to_owned(),
                })),
                Err(error) => err(AzureError::UTF8Error(error)),
            }
        }
    })
}

#[inline]
pub fn extract_status_and_body(
    resp: hyper::client::FutureResponse,
) -> impl Future<Item = (hyper::StatusCode, String), Error = AzureError> {
    resp.from_err().and_then(|res| {
        let status = res.status();
        res.body().concat2().from_err().and_then(move |whole_body| {
            match str::from_utf8(&whole_body) {
                Ok(s_body) => ok((status, s_body.to_owned())),
                Err(error) => err(AzureError::UTF8Error(error)),
            }
        })
    })
}

#[inline]
pub fn check_status_extract_body(
    resp: hyper::client::FutureResponse,
    expected_status_code: hyper::StatusCode,
) -> impl Future<Item = String, Error = AzureError> {
    extract_status_and_body(resp).and_then(move |(status, body)| {
        if status == expected_status_code {
            ok(body)
        } else {
            err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult {
                expected: expected_status_code,
                received: status,
                body,
            }))
        }
    })
}
