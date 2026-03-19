// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection results including server errors.

use std::time::Duration;

use azure_core::http::{
    headers::{HeaderName, HeaderValue, Headers},
    StatusCode,
};

use super::FaultInjectionErrorType;

/// A synthetic response to return when a fault injection rule matches.
///
/// Instead of injecting an error, this returns a successful response with
/// the specified status code, headers, and body. Useful for mocking service
/// responses such as `GetDatabaseAccount` in tests.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CustomResponse {
    status_code: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}

impl CustomResponse {
    /// Returns the HTTP status code for the synthetic response.
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns the headers for the synthetic response.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Returns the body for the synthetic response.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

/// Builder for creating a [`CustomResponse`].
pub struct CustomResponseBuilder {
    status_code: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}

impl CustomResponseBuilder {
    /// Creates a new builder with the given HTTP status code.
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status_code,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }

    /// Adds a header to the response.
    pub fn with_header(
        mut self,
        name: impl Into<HeaderName>,
        value: impl Into<HeaderValue>,
    ) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// Adds a sub-status header to the response.
    pub fn with_sub_status(self, code: u32) -> Self {
        self.with_header("x-ms-substatus", code.to_string())
    }

    /// Sets the body of the response.
    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// Builds the [`CustomResponse`].
    pub fn build(self) -> CustomResponse {
        CustomResponse {
            status_code: self.status_code,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// Represents a server error to be injected.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FaultInjectionResult {
    error_type: Option<FaultInjectionErrorType>,
    custom_response: Option<CustomResponse>,
    delay: Option<Duration>,
    probability: f32,
}

impl FaultInjectionResult {
    /// Returns the type of server error to inject.
    pub fn error_type(&self) -> Option<FaultInjectionErrorType> {
        self.error_type
    }

    /// Returns the custom response to return instead of injecting an error.
    pub fn custom_response(&self) -> Option<&CustomResponse> {
        self.custom_response.as_ref()
    }

    /// Returns the delay before injecting the error.
    pub fn delay(&self) -> Option<Duration> {
        self.delay
    }

    /// Returns the probability of injecting the fault (0.0 to 1.0).
    pub fn probability(&self) -> f32 {
        self.probability
    }
}

/// Builder for creating a FaultInjectionResult.
pub struct FaultInjectionResultBuilder {
    error_type: Option<FaultInjectionErrorType>,
    custom_response: Option<CustomResponse>,
    delay: Option<Duration>,
    probability: f32,
}

impl FaultInjectionResultBuilder {
    /// Creates a new FaultInjectionResultBuilder with default values.
    pub fn new() -> Self {
        Self {
            error_type: None,
            custom_response: None,
            delay: None,
            probability: 1.0,
        }
    }

    /// Sets the error type to inject.
    pub fn with_error(mut self, error_type: FaultInjectionErrorType) -> Self {
        self.error_type = Some(error_type);
        self
    }

    /// Sets a custom response to return instead of injecting an error.
    ///
    /// When set, the fault injection rule returns this synthetic response
    /// rather than forwarding the request to the real service. This takes
    /// precedence over `error_type` if both are set.
    pub fn with_custom_response(mut self, response: CustomResponse) -> Self {
        self.custom_response = Some(response);
        self
    }

    /// Sets the delay before injecting the error.
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Sets the probability of injecting the error (0.0 to 1.0).
    pub fn with_probability(mut self, probability: f32) -> Self {
        self.probability = if probability.is_finite() {
            probability.clamp(0.0, 1.0)
        } else {
            0.0
        };
        self
    }

    /// Builds the FaultInjectionResult.
    ///
    /// **Note**: A result with no `error_type`, no `custom_response`, and no
    /// `delay` will match requests but produce no observable effect — silently
    /// consuming any configured `hit_limit`. Ensure at least one of these is set.
    pub fn build(self) -> FaultInjectionResult {
        FaultInjectionResult {
            error_type: self.error_type,
            custom_response: self.custom_response,
            delay: self.delay,
            probability: self.probability,
        }
    }
}

impl Default for FaultInjectionResultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomResponseBuilder, FaultInjectionResultBuilder};
    use crate::fault_injection::FaultInjectionErrorType;
    use azure_core::http::StatusCode;
    use std::time::Duration;

    #[test]
    fn builder_default_values() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build();

        assert_eq!(
            error.error_type().unwrap(),
            FaultInjectionErrorType::Timeout
        );
        assert!(error.delay().is_none());
        assert!((error.probability() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_probability_clamped_above() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(1.5)
            .build();

        assert!((error.probability() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_probability_clamped_below() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(-0.5)
            .build();

        assert!(error.probability().abs() < f32::EPSILON);
    }

    #[test]
    fn builder_with_custom_response() {
        let body = b"{\"test\": true}".to_vec();
        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(
                CustomResponseBuilder::new(StatusCode::Ok)
                    .with_body(body.clone())
                    .build(),
            )
            .build();

        assert!(result.error_type().is_none());
        let custom = result.custom_response().unwrap();
        assert_eq!(custom.status_code(), StatusCode::Ok);
        assert_eq!(custom.body(), body);
    }

    #[test]
    fn builder_with_delay() {
        let result = FaultInjectionResultBuilder::new()
            .with_delay(Duration::from_millis(200))
            .build();

        assert_eq!(result.delay(), Some(Duration::from_millis(200)));
    }

    #[test]
    fn builder_probability_nan_normalized_to_zero() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(f32::NAN)
            .build();
        assert!(error.probability().abs() < f32::EPSILON);
    }

    #[test]
    fn custom_response_builder_with_sub_status() {
        let response = CustomResponseBuilder::new(StatusCode::Forbidden)
            .with_sub_status(3)
            .with_body(b"forbidden")
            .build();

        assert_eq!(response.status_code(), StatusCode::Forbidden);
        assert_eq!(response.body(), b"forbidden");
    }
}
