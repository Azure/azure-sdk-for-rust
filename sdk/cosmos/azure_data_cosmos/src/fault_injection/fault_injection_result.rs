// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection results including server errors.

use std::time::Duration;

/// Represents different server error types that can be injected for fault testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultInjectionErrorType {
    /// 500 from server.
    InternalServerError,
    /// 429 from server.
    TooManyRequests,
    /// 404-1002 from server.
    ReadSessionNotAvailable,
    /// 408 from server.
    Timeout,
    /// Simulate service unavailable (503).
    ServiceUnavailable,
    /// 410-1002 from server.
    PartitionIsGone,
    /// 403-3 Forbidden from server.
    WriteForbidden,
    /// 403-1008 Forbidden from server.
    DatabaseAccountNotFound,
    /// Simulate DNS resolution failure.
    DnsResolutionFailure,
    /// Simulate TCP connection failure.
    TcpConnectionFailure,
    /// Simulate client timed out waiting for a response.
    ResponseTimeout,
}

/// Represents a server error to be injected.
#[derive(Clone, Debug)]
pub struct FaultInjectionResult {
    /// The type of server error to inject.
    pub(crate) error_type: Option<FaultInjectionErrorType>,
    /// Number of times to inject the error.
    /// Default is that it will be injected forever.
    pub(crate) times: Option<u32>,
    /// Delay before injecting the error.
    /// default is no delay.
    pub(crate) delay: Duration,
    /// Probability of injecting the error (0.0 to 1.0).
    /// Default is 1.0 (always inject).
    pub(crate) probability: f32,
}

/// Builder for creating a FaultInjectionResult.
pub struct FaultInjectionResultBuilder {
    error_type: Option<FaultInjectionErrorType>,
    times: Option<u32>,
    delay: Duration,
    probability: f32,
}

impl FaultInjectionResultBuilder {
    /// Creates a new FaultInjectionResultBuilder with default values.
    pub fn new() -> Self {
        Self {
            error_type: None,
            times: None,
            delay: Duration::ZERO,
            probability: 1.0,
        }
    }

    /// Sets the error type to inject.
    pub fn with_error(mut self, error_type: FaultInjectionErrorType) -> Self {
        self.error_type = Some(error_type);
        self
    }

    /// Sets the number of times to inject the error.
    pub fn with_times(mut self, times: u32) -> Self {
        self.times = Some(times);
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
            times: self.times,
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
    use super::{FaultInjectionErrorType, FaultInjectionResultBuilder};
    use std::time::Duration;

    #[test]
    fn builder_default_values() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::Timeout)
            .build();

        assert_eq!(error.error_type.unwrap(), FaultInjectionErrorType::Timeout);
        assert!(error.times.is_none());
        assert_eq!(error.delay, Duration::ZERO);
        assert!((error.probability - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_probability_clamped_above() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(1.5)
            .build();

        assert!((error.probability - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_probability_clamped_below() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::ServiceUnavailable)
            .with_probability(-0.5)
            .build();

        assert!(error.probability.abs() < f32::EPSILON);
    }

    #[test]
    fn builder_chained() {
        let error = FaultInjectionResultBuilder::new()
            .with_error(FaultInjectionErrorType::PartitionIsGone)
            .with_times(3)
            .with_delay(Duration::from_millis(100))
            .with_probability(0.8)
            .build();

        assert_eq!(
            error.error_type.unwrap(),
            FaultInjectionErrorType::PartitionIsGone
        );
        assert_eq!(error.times, Some(3));
        assert_eq!(error.delay, Duration::from_millis(100));
        assert!((error.probability - 0.8).abs() < f32::EPSILON);
    }
}
