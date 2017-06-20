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
        UTF8Error (err: str::Utf8Error) {
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

pub fn extract_body(b: &hyper::Body) -> Box<Future<Item = String, Error = AzureError>> {
    let resp_s = b.concat2().then(|body| match body {
        Ok(body) => Ok(str::from_utf8(&body)?.to_owned()),
        Err(error) => Err(AzureError::HyperError(error)),
    });
    Box::new(resp_s)
}

#[inline]
pub fn check_status(
    resp: hyper::client::FutureResponse,
    s: StatusCode,
) -> Box<Future<Item = (), Error = AzureError>> {
    use std::str::from_utf8;

    Box::new(resp.then(|res| match res {
        Ok(res) => {
            if res.status() != s {

                //let b = res.body();
                //let rb = extract_body(&b);
                //res.body().rrr();

                res.body().concat2().then(|body| match body {
                    Ok(body) => {
                        let resp_s = from_utf8(&body)?;
                        Err(AzureError::UnexpectedHTTPResult(
                            UnexpectedHTTPResult::new(s, res.status(), &resp_s),
                        ))
                    }
                    Err(error) => Err(AzureError::HyperError(error)),
                })
            } else {
                Ok(())
            }
        }
        Err(he) => Err(AzureError::HyperError(he)),
    }))

    //Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(s,
    //                                                                  res.status(),
    //                                                                  &resp_s)))
    //} else {
    // Ok(())
    //}))
    //
    // let mut resp_s = String::new();
    //resp.read_to_string(&mut resp_s)?;

    //        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(s,
    //                                                                         resp.status,
    //                                                                       &resp_s)));
    //  }

    //Ok(())
}

pub fn check_status_extract_body(
    resp: &mut hyper::client::response::Response,
    s: StatusCode,
) -> Result<String, AzureError> {

    check_status(resp, s)?;

    let mut resp_s = String::new();
    resp.read_to_string(&mut resp_s)?;

    debug!("resp_s == {}", resp_s);

    Ok(resp_s)
}
