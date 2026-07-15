// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-attempt request diagnostics.
//!
//! A [`RequestDiagnostics`] captures the outcome of a single request attempt made while executing
//! a Cosmos DB operation. A [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) rolls up
//! one or more of these into an operation-level view.

use crate::models::{ActivityId, CosmosStatus, RequestCharge};
use crate::options::Region;
use azure_core::fmt::SafeDebug;
use std::time::Duration;

/// The context in which a request attempt was executed.
///
/// This categorizes *why* an attempt was made, which helps when reasoning about retry and
/// failover behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionContext {
    /// The initial attempt (first try).
    Initial,
    /// A retry after a transient error (for example `429` or `503`).
    Retry,
    /// A transport-level retry within the same region (for example a different HTTP/2 connection).
    TransportRetry,
    /// A hedged attempt issued to reduce tail latency.
    Hedging,
    /// An attempt against a different region after failover.
    RegionFailover,
    /// A circuit-breaker recovery probe.
    CircuitBreakerProbe,
}

impl ExecutionContext {
    /// Returns the stable string representation of this execution context.
    pub fn as_str(&self) -> &'static str {
        match self {
            ExecutionContext::Initial => "initial",
            ExecutionContext::Retry => "retry",
            ExecutionContext::TransportRetry => "transport_retry",
            ExecutionContext::Hedging => "hedging",
            ExecutionContext::RegionFailover => "region_failover",
            ExecutionContext::CircuitBreakerProbe => "circuit_breaker_probe",
        }
    }
}

impl AsRef<str> for ExecutionContext {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Diagnostics for a single request attempt within an operation.
///
/// Construct one with [`RequestDiagnostics::new`] and enrich it with the builder-style
/// `with_*` methods before recording it on a
/// [`DiagnosticsContextBuilder`](crate::diagnostics::DiagnosticsContextBuilder).
#[derive(Clone, PartialEq, SafeDebug)]
pub struct RequestDiagnostics {
    execution_context: ExecutionContext,
    endpoint: String,
    region: Option<Region>,
    status: CosmosStatus,
    request_charge: RequestCharge,
    activity_id: Option<ActivityId>,
    duration: Duration,
}

impl RequestDiagnostics {
    /// Creates a new request diagnostics record for an attempt with a known terminal status.
    pub(crate) fn new(
        execution_context: ExecutionContext,
        endpoint: impl Into<String>,
        status: CosmosStatus,
    ) -> Self {
        Self {
            execution_context,
            endpoint: endpoint.into(),
            region: None,
            status,
            request_charge: RequestCharge::default(),
            activity_id: None,
            duration: Duration::ZERO,
        }
    }

    /// Sets the region the attempt targeted.
    ///
    /// The driver does not yet resolve per-attempt regions, so this is currently only used by
    /// tests; production wiring is a follow-up. The [`region`](Self::region) accessor is
    /// always available and returns `None` until then.
    #[cfg(test)]
    pub(crate) fn with_region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Sets the request charge (RU) consumed by the attempt.
    pub(crate) fn with_request_charge(mut self, request_charge: RequestCharge) -> Self {
        self.request_charge = request_charge;
        self
    }

    /// Sets the service-assigned activity id for the attempt.
    pub(crate) fn with_activity_id(mut self, activity_id: ActivityId) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// Sets the client-observed duration of the attempt.
    pub(crate) fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Returns the execution context of the attempt.
    pub fn execution_context(&self) -> ExecutionContext {
        self.execution_context
    }

    /// Returns the endpoint URL the attempt targeted.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the region the attempt targeted, if known.
    pub fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    /// Returns the terminal status of the attempt.
    pub fn status(&self) -> CosmosStatus {
        self.status
    }

    /// Returns the request charge (RU) consumed by the attempt.
    pub fn request_charge(&self) -> RequestCharge {
        self.request_charge
    }

    /// Returns the service-assigned activity id for the attempt, if known.
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }

    /// Returns the client-observed duration of the attempt.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;

    fn ok_status() -> CosmosStatus {
        CosmosStatus::new(StatusCode::Ok)
    }

    #[test]
    fn new_sets_defaults() {
        let req = RequestDiagnostics::new(ExecutionContext::Initial, "https://acct/", ok_status());
        assert_eq!(req.execution_context(), ExecutionContext::Initial);
        assert_eq!(req.endpoint(), "https://acct/");
        assert!(req.region().is_none());
        assert!(req.activity_id().is_none());
        assert_eq!(req.request_charge(), RequestCharge::default());
        assert_eq!(req.duration(), Duration::ZERO);
    }

    #[test]
    fn with_setters_enrich_the_record() {
        let req = RequestDiagnostics::new(ExecutionContext::Retry, "https://west/", ok_status())
            .with_region(Region::WEST_US_2)
            .with_request_charge(RequestCharge::new(2.5))
            .with_activity_id(ActivityId::from_static("act-1"))
            .with_duration(Duration::from_millis(12));

        assert_eq!(req.execution_context(), ExecutionContext::Retry);
        assert_eq!(req.region(), Some(&Region::WEST_US_2));
        assert_eq!(req.request_charge(), RequestCharge::new(2.5));
        assert_eq!(req.activity_id().map(|a| a.as_str()), Some("act-1"));
        assert_eq!(req.duration(), Duration::from_millis(12));
    }

    #[test]
    fn execution_context_strings() {
        assert_eq!(ExecutionContext::Initial.as_str(), "initial");
        assert_eq!(ExecutionContext::TransportRetry.as_str(), "transport_retry");
        assert_eq!(
            ExecutionContext::RegionFailover.to_string(),
            "region_failover"
        );
    }
}
