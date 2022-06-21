use crate::Response;
use bytes::Bytes;
use std::collections::HashMap;

/// An unsuccessful HTTP response
#[derive(Debug)]
pub struct HttpError {
    status: u16,
    error_code: Option<String>,
    headers: std::collections::HashMap<String, String>,
    body: Bytes,
}

impl HttpError {
    /// Create an error from an http response.
    ///
    /// This does not check whether the response was a success and should only be used with unsuccessful responses.
    pub async fn new(response: Response) -> Self {
        let status = response.status();
        let mut error_code = get_error_code_from_header(&response);
        let mut headers = HashMap::new();

        for (name, value) in response.headers().iter() {
            headers.insert(name.as_str().to_string(), value.as_str().to_string());
        }

        let body = response.into_body().await;
        error_code = error_code.or_else(|| get_error_code_from_body(&body));
        HttpError {
            status: status.as_u16(),
            headers,
            error_code,
            body,
        }
    }

    /// Get a reference to the http error's status.
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Get a reference to the http error's error code.
    pub fn error_code(&self) -> Option<&str> {
        self.error_code.as_deref()
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HttpError")?;
        writeln!(f, "\tStatus: {}", self.status)?;
        writeln!(
            f,
            "\tError Code: {}",
            self.error_code.as_deref().unwrap_or("unknown")
        )?;
        // TODO: sanitize body
        writeln!(f, "\tBody: \"{:?}\"", self.body)?;
        writeln!(f, "\tHeaders:")?;
        // TODO: sanitize headers
        for (k, v) in &self.headers {
            writeln!(f, "\t\t{}:{}", k, v)?;
        }

        Ok(())
    }
}

impl std::error::Error for HttpError {}

/// Gets the error code if it's present in the headers
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
fn get_error_code_from_header(response: &Response) -> Option<String> {
    response.headers().get_as_string("x-ms-error-code")
}

/// Gets the error code if it's present in the body
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
pub(crate) fn get_error_code_from_body(body: &[u8]) -> Option<String> {
    Some(
        serde_json::from_slice::<serde_json::Value>(body)
            .ok()?
            .get("error")?
            .get("code")?
            .as_str()?
            .to_owned(),
    )
}
