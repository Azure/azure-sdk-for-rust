use crate::{headers, Response, StatusCode};
use bytes::Bytes;
use std::collections::HashMap;

/// An unsuccessful HTTP response
#[derive(Debug)]
pub struct HttpError {
    status: StatusCode,
    details: ErrorDetails,
    headers: std::collections::HashMap<String, String>,
    body: Bytes,
}

impl HttpError {
    /// Create an error from an http response.
    ///
    /// This does not check whether the response was a success and should only be used with unsuccessful responses.
    pub async fn new(response: Response) -> Self {
        let status = response.status();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| (name.as_str().to_owned(), value.as_str().to_owned()))
            .collect();
        let body = response
            .into_body()
            .collect()
            .await
            .unwrap_or_else(|_| Bytes::from_static(b"<ERROR COLLECTING BODY>"));
        let details = ErrorDetails::new(&headers, &body);
        HttpError {
            status,
            details,
            headers,
            body,
        }
    }

    /// Get the status code for the http error
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get a reference to the http error's error code.
    pub fn error_code(&self) -> Option<&str> {
        self.details.code.as_deref()
    }

    /// Get a reference to the http error's error message.
    pub fn error_message(&self) -> Option<&str> {
        self.details.message.as_deref()
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let newline = if f.alternate() { "\n" } else { " " };
        let tab = if f.alternate() { "\t" } else { " " };
        write!(f, "HttpError {{{newline}")?;
        write!(f, "{tab}Status: {},{newline}", self.status)?;
        write!(
            f,
            "{tab}Error Code: {},{newline}",
            self.details
                .code
                .as_deref()
                .unwrap_or("<unknown error code>")
        )?;
        // TODO: sanitize body
        write!(f, "{tab}Body: \"{:?}\",{newline}", self.body)?;
        write!(f, "{tab}Headers: [{newline}")?;
        // TODO: sanitize headers
        for (k, v) in &self.headers {
            write!(f, "{tab}{tab}{}:{}{newline}", k, v)?;
        }
        write!(f, "{tab}],{newline}}}{newline}")?;

        Ok(())
    }
}

impl std::error::Error for HttpError {}

#[derive(Debug)]
struct ErrorDetails {
    code: Option<String>,
    message: Option<String>,
}

impl ErrorDetails {
    fn new(headers: &HashMap<String, String>, body: &[u8]) -> Self {
        let mut code = get_error_code_from_header(headers);
        code = code.or_else(|| get_error_code_from_body(body));
        let message = get_error_message_from_body(body);
        Self { code, message }
    }
}

/// Gets the error code if it's present in the headers
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
fn get_error_code_from_header(headers: &HashMap<String, String>) -> Option<String> {
    headers.get(headers::ERROR_CODE.as_str()).cloned()
}

/// Gets the error code if it's present in the body
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
pub(crate) fn get_error_code_from_body(body: &[u8]) -> Option<String> {
    let json = serde_json::from_slice::<serde_json::Value>(body).ok()?;
    let nested = || json.get("error")?.get("code")?.as_str();
    let top_level = || json.get("code")?.as_str();
    let code = nested().or_else(top_level);
    code.map(|c| c.to_owned())
}

/// Gets the error message if it's present in the body
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
pub(crate) fn get_error_message_from_body(body: &[u8]) -> Option<String> {
    let json = serde_json::from_slice::<serde_json::Value>(body).ok()?;
    let nested = || json.get("error")?.get("message")?.as_str();
    let top_level = || json.get("message")?.as_str();
    let code = nested().or_else(top_level);
    code.map(|c| c.to_owned())
}
