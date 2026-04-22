// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection results including server errors.

use std::time::Duration;

use azure_core::http::{headers::Headers, StatusCode};

use super::FaultInjectionErrorType;

/// A synthetic response to return when a fault injection rule matches.
///
/// Instead of injecting an error, this returns a successful response with
/// the specified status code, headers, and body. Useful for mocking service
/// responses such as `GetDatabaseAccount` in tests.
#[derive(Clone, Debug)]
pub struct CustomResponse {
    /// The HTTP status code for the synthetic response.
    pub status_code: StatusCode,
    /// The headers for the synthetic response.
    pub headers: Headers,
    /// The body for the synthetic response.
    pub body: Vec<u8>,
}

/// Represents a server error to be injected.
#[derive(Clone, Debug)]
pub struct FaultInjectionResult {
    /// The type of server error to inject.
    pub error_type: Option<FaultInjectionErrorType>,
    /// A custom response to return instead of injecting an error.
    pub custom_response: Option<CustomResponse>,
    /// Delay before injecting the error.
    pub delay: Duration,
    /// Probability of injecting the error (0.0 to 1.0).
    probability: f32,
}

impl FaultInjectionResult {
    /// Returns the probability of injecting the fault (0.0 to 1.0).
    pub fn probability(&self) -> f32 {
        self.probability
    }
}

/// Builder for creating a FaultInjectionResult.
pub struct FaultInjectionResultBuilder {
    error_type: Option<FaultInjectionErrorType>,
    custom_response: Option<CustomResponse>,
    delay: Duration,
    probability: f32,
}

impl FaultInjectionResultBuilder {
    /// Creates a new FaultInjectionResultBuilder with default values.
    pub fn new() -> Self {
        Self {
            error_type: None,
            custom_response: None,
            delay: Duration::ZERO,
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
        self.delay = delay;
        self
    }

    /// Sets the probability of injecting the error (0.0 to 1.0).
    pub fn with_probability(mut self, probability: f32) -> Self {
        self.probability = probability.clamp(0.0, 1.0);
        self
    }

    /// Builds the FaultInjectionResult.
    ///
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
    use super::{CustomResponse, FaultInjectionResultBuilder};
    use crate::fault_injection::FaultInjectionErrorType;
    use azure_core::http::{headers::Headers, StatusCode};
    use std::time::Duration;

    #[test]
    fn builder_default_values() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build();

        assert_eq!(error.error_type.unwrap(), FaultInjectionErrorType::Timeout);
        assert_eq!(error.delay, Duration::ZERO);
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
            .with_custom_response(CustomResponse {
                status_code: StatusCode::Ok,
                headers: Headers::new(),
                body: body.clone(),
            })
            .build();

        assert!(result.error_type.is_none());
        let custom = result.custom_response.unwrap();
        assert_eq!(custom.status_code, StatusCode::Ok);
        assert_eq!(custom.body, body);
    }
}
