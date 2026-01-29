// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Defines fault injection results including server errors.

use std::time::Duration;

/// Trait for fault injection results.
pub trait FaultInjectionResult: Send + Sync + std::fmt::Debug {
    /// Clones this result into a boxed trait object.
    fn clone_box(&self) -> Box<dyn FaultInjectionResult>;

    /// Returns this result as a server error if it is one.
    fn as_server_error(&self) -> Option<&FaultInjectionServerError>;
}

impl Clone for Box<dyn FaultInjectionResult> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Represents different server error types that can be injected for fault testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultInjectionServerErrorType {
    /// 500 from server.
    InternalServerError,
    /// 429 from server.
    TooManyRequests,
    /// 404-1002 from server.
    ReadSessionNotAvailable,
    /// 408 from server.
    Timeout,
    /// Response delay, when it is over request timeout, can simulate transit timeout.
    ResponseDelay,
    /// Simulate high channel acquisition, when it is over connection timeout, can simulate connectionTimeoutException.
    ConnectionDelay,
    /// Simulate service unavailable (503).
    ServiceUnavailable,
    /// 410-1002 from server.
    PartitionIsGone,
}

/// Represents a server error to be injected.
#[derive(Clone, Debug)]
pub struct FaultInjectionServerError {
    /// The type of server error to inject.
    pub error_type: FaultInjectionServerErrorType,
    /// Number of times to inject the error.
    /// Default is that it will be injected forever.
    pub times: Option<u32>,
    /// Delay before injecting the error.
    /// default is no delay.
    pub delay: Duration,
    /// Probability of injecting the error (0.0 to 1.0).
    /// Default is 1.0 (always inject).
    pub probability: f32,
}

impl FaultInjectionResult for FaultInjectionServerError {
    fn clone_box(&self) -> Box<dyn FaultInjectionResult> {
        Box::new(self.clone())
    }

    fn as_server_error(&self) -> Option<&FaultInjectionServerError> {
        Some(self)
    }
}

/// Builder for creating a FaultInjectionServerError.
pub struct FaultInjectionServerErrorBuilder {
    error_type: FaultInjectionServerErrorType,
    times: Option<u32>,
    delay: Duration,
    probability: f32,
}

impl FaultInjectionServerErrorBuilder {
    /// Creates a new FaultInjectionServerErrorBuilder with the specified error type.
    pub fn new(error_type: FaultInjectionServerErrorType) -> Self {
        Self {
            error_type,
            times: None,
            delay: Duration::ZERO,
            probability: 1.0,
        }
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

    /// Builds the FaultInjectionServerError.
    pub fn build(self) -> FaultInjectionServerError {
        FaultInjectionServerError {
            error_type: self.error_type,
            times: self.times,
            delay: self.delay,
            probability: self.probability,
        }
    }
}

