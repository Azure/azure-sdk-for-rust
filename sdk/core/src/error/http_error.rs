use crate::{
    content_type, from_json,
    headers::{self, Headers},
    Response, StatusCode,
};
use bytes::Bytes;
use serde::Deserialize;

/// An unsuccessful HTTP response
#[derive(Debug)]
pub struct HttpError {
    status: StatusCode,
    details: ErrorDetails,
    headers: Headers,
    body: Bytes,
}

impl HttpError {
    /// Create an error from an http response.
    ///
    /// This does not check whether the response was a success and should only be used with unsuccessful responses.
    pub async fn new(response: Response) -> Self {
        let (status, headers, body) = response.deconstruct();
        let body = body
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
        for (k, v) in self.headers.iter() {
            write!(
                f,
                "{tab}{tab}{k}:{v}{newline}",
                k = k.as_str(),
                v = v.as_str()
            )?;
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
    fn new(headers: &Headers, body: &[u8]) -> Self {
        let header_err_code = get_error_code_from_header(headers);
        let content_type = headers.get_optional_str(&headers::CONTENT_TYPE);
        let (body_err_code, body_err_message) =
            get_error_code_message_from_body(body, content_type);

        let code = header_err_code.or(body_err_code);
        Self {
            code,
            message: body_err_message,
        }
    }
}

/// Gets the error code if it's present in the headers
///
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
pub(crate) fn get_error_code_from_header(headers: &Headers) -> Option<String> {
    headers.get_optional_string(&headers::ERROR_CODE)
}

#[derive(Deserialize)]
struct NestedError {
    #[serde(alias = "Message")]
    message: Option<String>,
    #[serde(alias = "Code")]
    code: Option<String>,
}

/// Error from a response body, aliases are set because XML responses follow different case-ing
#[derive(Deserialize)]
struct ErrorBody {
    #[serde(alias = "Error")]
    error: Option<NestedError>,
    #[serde(alias = "Message")]
    message: Option<String>,
    #[serde(alias = "Code")]
    code: Option<String>,
}

impl ErrorBody {
    /// Deconstructs self into error (code, message)
    ///
    /// The nested errors fields take precedence over those in the root of the structure
    fn into_code_message(self) -> (Option<String>, Option<String>) {
        let (nested_code, nested_message) = self
            .error
            .map(|nested_error| (nested_error.code, nested_error.message))
            .unwrap_or((None, None));
        (nested_code.or(self.code), nested_message.or(self.message))
    }
}

/// Gets the error code and message from the body based on the specified content_type
/// Support for xml decoding is dependent on the 'xml' feature flag
///
/// Assumes JSON if unspecified/inconclusive to maintain old behaviour
/// [#1275](https://github.com/Azure/azure-sdk-for-rust/issues/1275)
/// For more info, see [here](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors)
pub(crate) fn get_error_code_message_from_body(
    body: &[u8],
    content_type: Option<&str>,
) -> (Option<String>, Option<String>) {
    let err_body: Option<ErrorBody> = if content_type
        .is_some_and(|ctype| ctype == content_type::APPLICATION_XML.as_str())
    {
        #[cfg(feature = "xml")]
        {
            crate::xml::read_xml(body).ok()
        }
        #[cfg(not(feature = "xml"))]
        {
            tracing::warn!("encountered XML response but the 'xml' feature flag was not specified");
            None
        }
    } else {
        // keep old default of assuming JSON
        from_json(body).ok()
    };

    err_body
        .map(ErrorBody::into_code_message)
        .unwrap_or((None, None))
}
