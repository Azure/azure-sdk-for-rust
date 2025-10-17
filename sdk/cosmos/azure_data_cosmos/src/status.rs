//! Defines the [`CosmosStatus`] type, which pairs an HTTP status code with a Cosmos sub-status code.

use std::num::ParseIntError;

use azure_core::{
    error::ErrorKind,
    http::{headers::FromHeaders, StatusCode},
};

use crate::constants;

/// A Cosmos sub-status code, which provides additional information about the result of an operation.
///
/// A specific sub-status code is often only meaningful in the context of a specific HTTP status code.
/// That is, sub-status code `x` may have a different meaning when paired with HTTP status code `A` than it does when paired with HTTP status code `B`.
///
/// Constants on [`CosmosStatus`] provide the full source-of-truth meanings for specific HTTP status code and sub-status code combinations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubStatusCode(u16);

impl SubStatusCode {
    /// Creates a new `SubStatusCode` from a `u16`.
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the inner `u16` value of the `SubStatusCode`.
    pub const fn value(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for SubStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromHeaders for SubStatusCode {
    type Error = ParseIntError;

    fn header_names() -> &'static [&'static str] {
        // Right now, it's not feasible to extract the static str from HeaderName
        &["x-ms-substatus"]
    }

    fn from_headers(
        headers: &azure_core::http::headers::Headers,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(s) = headers.get_optional_str(&constants::SUB_STATUS) else {
            return Ok(None);
        };
        let value = s.parse::<u16>()?;
        Ok(Some(SubStatusCode::new(value)))
    }
}

/// Represents a Cosmos DB status, which is a combination of an HTTP status code and an optional Cosmos sub-status code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CosmosStatus(StatusCode, Option<SubStatusCode>);

impl CosmosStatus {
    pub const NAME_CACHE_IS_STALE: CosmosStatus =
        CosmosStatus::new(StatusCode::Gone, Some(SubStatusCode::new(1000)));

    pub const fn new(status: StatusCode, substatus: Option<SubStatusCode>) -> Self {
        Self(status, substatus)
    }

    pub const fn status_code(&self) -> StatusCode {
        self.0
    }

    pub const fn substatus_code(&self) -> Option<SubStatusCode> {
        self.1
    }
}

pub trait ErrorExt {
    /// Fetches the [`CosmosStatus`] associated with this error, if any.
    fn cosmos_status(&self) -> azure_core::Result<Option<CosmosStatus>>;
}

impl ErrorExt for azure_core::Error {
    fn cosmos_status(&self) -> azure_core::Result<Option<CosmosStatus>> {
        match self.kind() {
            ErrorKind::HttpResponse {
                status,
                raw_response,
                ..
            } => {
                let substatus: Option<SubStatusCode> = raw_response
                    .as_ref()
                    .and_then(|resp| resp.headers().get_optional().transpose())
                    .transpose()
                    .map_err(|_| {
                        azure_core::Error::with_message(
                            ErrorKind::DataConversion,
                            "failed to parse substatus",
                        )
                    })?;
                Ok(Some(CosmosStatus::new(*status, substatus)))
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use azure_core::{
        error::ErrorKind,
        http::{headers::Headers, RawResponse, StatusCode},
    };

    use super::*;

    #[test]
    fn cosmos_status_on_non_http_error() {
        let err =
            azure_core::Error::with_message(azure_core::error::ErrorKind::Other, "test error");
        assert!(err.cosmos_status().unwrap().is_none());
    }

    #[test]
    fn cosmos_status_on_http_error_without_substatus() {
        let headers = Headers::new();
        let response = RawResponse::from_bytes(StatusCode::Conflict, headers, Vec::new());
        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::Conflict,
                error_code: None,
                raw_response: Some(Box::new(response)),
            },
            "test error",
        );
        assert_eq!(
            error.cosmos_status().unwrap(),
            Some(CosmosStatus::new(StatusCode::Conflict, None))
        );
    }

    #[test]
    fn cosmos_status_on_http_error_with_substatus() {
        let mut headers = Headers::new();
        headers.insert(
            constants::SUB_STATUS,
            CosmosStatus::NAME_CACHE_IS_STALE
                .substatus_code()
                .unwrap()
                .to_string(),
        );
        let response = RawResponse::from_bytes(
            CosmosStatus::NAME_CACHE_IS_STALE.status_code(),
            headers,
            Vec::new(),
        );
        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: CosmosStatus::NAME_CACHE_IS_STALE.status_code(),
                error_code: None,
                raw_response: Some(Box::new(response)),
            },
            "test error",
        );
        assert_eq!(
            error.cosmos_status().unwrap(),
            Some(CosmosStatus::NAME_CACHE_IS_STALE)
        );
    }

    #[test]
    fn cosmos_status_with_invalid_substatus() {
        let mut headers = Headers::new();
        headers.insert(constants::SUB_STATUS, "invalid");
        let response = RawResponse::from_bytes(StatusCode::Conflict, headers, Vec::new());
        let error = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::Conflict,
                error_code: None,
                raw_response: Some(Box::new(response)),
            },
            "test error",
        );
        assert!(error.cosmos_status().is_err());
    }
}
