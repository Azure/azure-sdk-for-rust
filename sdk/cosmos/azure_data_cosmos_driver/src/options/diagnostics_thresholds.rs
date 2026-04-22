// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostics threshold types.

use crate::models::RequestCharge;
use std::time::Duration;

/// Thresholds for controlling when diagnostics are captured/logged.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiagnosticsThresholds {
    point_operation_latency_threshold: Option<Duration>,
    non_point_operation_latency_threshold: Option<Duration>,
    request_charge_threshold: Option<RequestCharge>,
    payload_size_threshold: Option<usize>,
}

impl DiagnosticsThresholds {
    /// Creates new diagnostics thresholds with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the latency threshold for point operations.
    pub fn point_operation_latency_threshold(&self) -> Option<Duration> {
        self.point_operation_latency_threshold
    }

    /// Gets the latency threshold for non-point operations.
    pub fn non_point_operation_latency_threshold(&self) -> Option<Duration> {
        self.non_point_operation_latency_threshold
    }

    /// Gets the request charge threshold.
    pub fn request_charge_threshold(&self) -> Option<RequestCharge> {
        self.request_charge_threshold
    }

    /// Gets the payload size threshold.
    pub fn payload_size_threshold(&self) -> Option<usize> {
        self.payload_size_threshold
    }

    /// Sets the latency threshold for point operations.
    pub fn with_point_operation_latency_threshold(mut self, threshold: Duration) -> Self {
        self.point_operation_latency_threshold = Some(threshold);
        self
    }

    /// Sets the latency threshold for non-point operations.
    pub fn with_non_point_operation_latency_threshold(mut self, threshold: Duration) -> Self {
        self.non_point_operation_latency_threshold = Some(threshold);
        self
    }

    /// Sets the request charge threshold.
    pub fn with_request_charge_threshold(mut self, threshold: RequestCharge) -> Self {
        self.request_charge_threshold = Some(threshold);
        self
    }

    /// Sets the payload size threshold.
    pub fn with_payload_size_threshold(mut self, threshold: usize) -> Self {
        self.payload_size_threshold = Some(threshold);
        self
    }
}
