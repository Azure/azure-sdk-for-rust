// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Regions contacted by a Cosmos DB operation and the reasons for dispatch.

use crate::options::Region;

/// The reason the SDK dispatched a request to a particular region.
///
/// Carried by each [`RequestedRegion`] entry returned from
/// [`DiagnosticsContext::requested_regions`](crate::diagnostics::DiagnosticsContext::requested_regions).
///
/// The enum is `#[non_exhaustive]`; always include a wildcard arm in `match`
/// expressions.
///
/// # Examples
///
/// ```rust
/// # use azure_data_cosmos::diagnostics::{RequestedRegion, RequestedRegionReason};
/// fn describe(r: &RequestedRegion) -> &'static str {
///     match r.reason {
///         RequestedRegionReason::Initial => "initial dispatch",
///         RequestedRegionReason::OperationRetry => "SDK-level retry",
///         RequestedRegionReason::TransportRetry => "transport-level retry",
///         RequestedRegionReason::Hedging => "speculative hedge",
///         RequestedRegionReason::RegionFailover => "region-failover retry",
///         RequestedRegionReason::CircuitBreakerProbe => "circuit-breaker probe",
///         _ => "unknown",
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RequestedRegionReason {
    /// The first dispatch of the operation.
    Initial,
    /// An operation-level retry decided by the SDK's client-retry policy.
    OperationRetry,
    /// A transport-level retry inside the per-region transport stack.
    TransportRetry,
    /// A speculative cross-region hedge fan-out dispatch.
    Hedging,
    /// An endpoint-failure-driven retry to a different region.
    RegionFailover,
    /// A probe dispatch to a previously circuit-broken region.
    CircuitBreakerProbe,
}

impl From<azure_data_cosmos_driver::diagnostics::ExecutionContext> for RequestedRegionReason {
    fn from(
        driver: azure_data_cosmos_driver::diagnostics::ExecutionContext,
    ) -> RequestedRegionReason {
        match driver {
            azure_data_cosmos_driver::diagnostics::ExecutionContext::Initial => {
                RequestedRegionReason::Initial
            }
            azure_data_cosmos_driver::diagnostics::ExecutionContext::OperationRetry => {
                RequestedRegionReason::OperationRetry
            }
            azure_data_cosmos_driver::diagnostics::ExecutionContext::TransportRetry => {
                RequestedRegionReason::TransportRetry
            }
            azure_data_cosmos_driver::diagnostics::ExecutionContext::Hedging => {
                RequestedRegionReason::Hedging
            }
            azure_data_cosmos_driver::diagnostics::ExecutionContext::RegionFailover => {
                RequestedRegionReason::RegionFailover
            }
            azure_data_cosmos_driver::diagnostics::ExecutionContext::CircuitBreakerProbe => {
                RequestedRegionReason::CircuitBreakerProbe
            }
            // The driver enum is #[non_exhaustive]; map any future variants to
            // the closest known reason rather than panicking.
            _ => RequestedRegionReason::Initial,
        }
    }
}

/// A region contacted by an operation and the reason for the dispatch.
///
/// Returned by
/// [`DiagnosticsContext::requested_regions`](crate::diagnostics::DiagnosticsContext::requested_regions).
///
/// The struct is `#[non_exhaustive]`; pattern-match with `..` to allow
/// additional fields in future versions.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RequestedRegion {
    /// The region the SDK dispatched to.
    pub region: Region,
    /// The reason the SDK chose this region for this dispatch attempt.
    pub reason: RequestedRegionReason,
}

impl From<azure_data_cosmos_driver::diagnostics::RequestedRegion> for RequestedRegion {
    fn from(driver: azure_data_cosmos_driver::diagnostics::RequestedRegion) -> RequestedRegion {
        RequestedRegion {
            region: driver.region,
            reason: driver.reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::diagnostics::ExecutionContext as DriverCtx;

    #[test]
    fn reason_from_driver_all_variants() {
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::Initial),
            RequestedRegionReason::Initial
        );
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::OperationRetry),
            RequestedRegionReason::OperationRetry
        );
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::TransportRetry),
            RequestedRegionReason::TransportRetry
        );
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::Hedging),
            RequestedRegionReason::Hedging
        );
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::RegionFailover),
            RequestedRegionReason::RegionFailover
        );
        assert_eq!(
            RequestedRegionReason::from(DriverCtx::CircuitBreakerProbe),
            RequestedRegionReason::CircuitBreakerProbe
        );
    }
}
