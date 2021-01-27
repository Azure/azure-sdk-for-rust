use http::header::ToStrError;
use hyper::{self, body, Body, StatusCode};
use std::io::Error as IOError;
use std::num;
use std::num::ParseIntError;
use std::str;
use std::str::ParseBoolError;
use std::string;
use url::ParseError as URLParseError;
use xml::BuilderError as XMLError;

#[derive(Debug)]
pub enum ParsingError {
    ElementNotFound(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    SplitNotFound,
    ParseIntError(ParseIntError),
}

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
pub struct UnexpectedValue {
    expected: Vec<String>,
    received: String,
}

impl UnexpectedValue {
    pub fn new(expected: String, received: String) -> UnexpectedValue {
        Self {
            expected: vec![expected],
            received,
        }
    }

    pub fn new_multiple(allowed: Vec<String>, received: String) -> Self {
        UnexpectedValue {
            expected: allowed,
            received,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnexpectedHTTPResult {
    expected: Vec<StatusCode>,
    received: StatusCode,
    body: String,
}

impl UnexpectedHTTPResult {
    pub fn new(expected: StatusCode, received: StatusCode, body: &str) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: vec![expected],
            received,
            body: body.to_owned(),
        }
    }

    pub fn new_multiple(
        allowed: Vec<StatusCode>,
        received: StatusCode,
        body: &str,
    ) -> UnexpectedHTTPResult {
        UnexpectedHTTPResult {
            expected: allowed,
            received,
            body: body.to_owned(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        self.received
    }
}

impl std::fmt::Display for UnexpectedHTTPResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unexpected HTTP result (expected: {:?}, received: {:?})",
            self.expected, self.received
        )
    }
}

impl std::error::Error for UnexpectedHTTPResult {
    fn description(&self) -> &str {
        "Unexpected HTTP result"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Not512ByteAlignedError {
        StartRange(u: u64) {
            display("start range not 512-byte aligned: {}", u)
        }
        EndRange(u: u64) {
            display("end range not 512-byte aligned: {}", u)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum Parse512AlignedError {
        SplitNotFound {
            display("split not found")
        }
        ParseIntError(p :ParseIntError) {
            from()
            display("parse int error: {}", p)
            cause(p)
        }
        Not512ByteAlignedError(nb: Not512ByteAlignedError)  {
            from()
            display("not 512 byte aligned error: {}", nb)
            cause(nb)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum AzureError {
        PageNot512ByteAlignedError(start: u64, end: u64) {
            display("{}-{} is not 512 byte aligned", start, end)
        }
        Not512ByteAlignedError(size: u64) {
            display("{} is not 512 byte aligned", size)
        }
        OperationNotSupported(operation: String, reason: String){
            display("Operation not supported. Operation == {}, reason == {}", operation, reason)
        }
        Base64DecodeError(err: base64::DecodeError) {
            from()
            display("base64 decode error: {}", err)
            cause(err)
        }
        DigestNot16BytesLong(len : u64) {
            display("digest length {} bytes instead of 16", len)
        }
        CRC64Not8BytesLong(len : u64) {
            display("CRC64 length {} bytes instead of 8", len)
        }
         ParseBoolError(err: ParseBoolError) {
            from()
            display("parse bool error: {}", err)
            cause(err)
        }
        ToStrError(err: ToStrError) {
            from()
            display("to str error: {}", err)
            cause(err)
        }
        JSONError(err: serde_json::Error) {
            from()
            display("json error: {}", err)
            cause(err)
        }
        HyperError(err: hyper::Error){
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
        UnexpectedXMLError(err: String) {
            display("UnexpectedXMLError: {}", err)
        }
        AzurePathParseError(err: AzurePathParseError){
            from()
            display("Azure Path parse error: {}", err)
            cause(err)
        }
        UnexpectedHTTPResult(err: UnexpectedHTTPResult){
            from()
            display("UnexpectedHTTPResult error: {}", err)
        }
        UnexpectedValue(err: UnexpectedValue){
            from()
            display("UnexpectedValue error: {:?}", err)
        }
        HeaderNotFound(msg: String) {
            display("Header not found: {}", msg)
        }
        HeadersNotFound(headers: Vec<String>) {
            display("At least one of these headers must be present: {:?}", headers)
        }
        UrlQueryParameterNotFound(expected_parameter: String, url: url::Url) {
            display("The expected query parameter {} was not found in the provided Url: {:?}", expected_parameter, url)
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
        ParseFloatError(err: std::num::ParseFloatError) {
            from()
            display("Parse float error: {}", err)
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
        HttpPrepareError(err: http::Error) {
            from()
            display("Error preparing HTTP request: {}", err) // todo: revisit usages / message here
            cause(err)
        }
        ParseUuidError(err: uuid::Error){
            from()
            display("uuid error: {}", err)
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
        SerdeXMLDeserializationError(err:serde_xml_rs::Error) {
            from()
            display("XML deserialization error: {}", err)
            cause(err)
        }
        MissingHeaderError(header: String) {
            display("A required header is missing: {}", header)
        }
        MissingValueError(value: String, expected_type: String) {
            display("An expected JSON node is missing: {} of expected type {}", value, expected_type)
        }
        FailureError(error: failure::Error) {
            display("failure::Error error {}", error)
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
        BooleanNotMatched(s: String) {
            display("Input string cannot be converted in boolean: {}", s)
        }
        UnexpectedNodeTypeError(expected: String) {
            display("Unexpected node type received: expected {}", expected)
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
   }
}

impl From<()> for AzureError {
    fn from(_: ()) -> AzureError {
        AzureError::GenericError
    }
}

impl From<failure::Error> for AzureError {
    fn from(error: failure::Error) -> AzureError {
        AzureError::FailureError(error)
    }
}

#[inline]
pub async fn extract_status_headers_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(hyper::StatusCode, hyper::HeaderMap, body::Bytes), AzureError> {
    let res = resp.await?;
    let (head, body) = res.into_parts();
    let status = head.status;
    let headers = head.headers;
    let body = body::to_bytes(body).await?;

    Ok((status, headers, body))
}

#[inline]
pub async fn extract_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(StatusCode, String), AzureError> {
    let res = resp.await?;
    let status = res.status();
    let body = body::to_bytes(res.into_body()).await?;
    Ok((status, str::from_utf8(&body)?.to_owned()))
}

#[inline]
pub async fn extract_location_status_and_body(
    resp: hyper::client::ResponseFuture,
) -> Result<(http::StatusCode, String, String), AzureError> {
    let res = resp.await?;
    let status = res.status();
    let location: String = match res.headers().get("Location") {
        Some(header_value) => header_value.to_str()?.to_owned(),
        _ => "".to_owned(),
    };
    let body = body::to_bytes(res.into_body()).await?;
    Ok((status, location, str::from_utf8(&body)?.to_owned()))
}

#[inline]
pub async fn check_status_extract_body(
    resp: hyper::client::ResponseFuture,
    expected_status_code: hyper::StatusCode,
) -> Result<String, AzureError> {
    let (status, body) = extract_status_and_body(resp).await?;
    if status == expected_status_code {
        Ok(body)
    } else {
        Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            expected_status_code,
            status,
            &body,
        )))
    }
}

pub async fn check_status_extract_body_2(
    resp: hyper::Response<Body>,
    expected_status: StatusCode,
) -> Result<String, AzureError> {
    let received_status = resp.status();

    let body = body::to_bytes(resp.into_body()).await?;
    let s = String::from_utf8(body.to_vec())?;
    debug!("body: {}", s);
    if received_status != expected_status {
        Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            expected_status,
            received_status,
            &s,
        )))
    } else {
        Ok(s)
    }
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

    fn error_generator() -> Result<(), AzureError> {
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

    // This does not compile
    //#[test]
    //fn test_not_send() {
    //    let a = std::rc::Rc::new(100);
    //    send_fn(a);
    //}

    // This does not compile
    //#[test]
    //fn test_not_sync() {
    //    let a = std::cell::Cell::new(500);
    //    sync_fn(a);
    //}
}
