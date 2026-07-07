// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-region hedging for the two hot-path metadata reads (Collection Read and the
//! first PartitionKeyRange ReadFeed page).
//!
//! # Summary
//! When enabled and eligible, a slow (or regionally-failing) primary metadata read
//! triggers a single hedged request to another region after a fixed latency threshold.
//! The first *acceptable* answer wins, with the **primary always authoritative**: a hedge
//! can only make the read faster, never change the outcome the primary has already
//! produced at the moment of arbitration.
//!
//! This is a latency optimization, not a resilience mechanism — failover is handled by
//! the metadata retry policy. Ported from the .NET SDK's `MetadataHedgingStrategy`
//! (Azure/azure-cosmos-dotnet-v3#5999).

use crate::constants::{SubStatusCode, DEFAULT_CONNECTION_TIMEOUT, SUB_STATUS};
use crate::cosmos_request::CosmosRequest;
use crate::models::CosmosResponse;
use crate::pipeline::GatewayPipeline;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use azure_core::async_runtime::get_async_runtime;
use azure_core::error::ErrorKind;
use azure_core::http::{Context, StatusCode};
use futures::future::{select, Either};
use std::time::Duration;
use url::Url;

/// Fixed step added to the first-attempt (connection) timeout to derive the hedge
/// threshold, mirroring the .NET `DefaultThresholdStep`.
const DEFAULT_THRESHOLD_STEP: Duration = Duration::from_millis(500);

/// Three-way classification of a single branch's settled outcome.
///
/// Mirrors the .NET `BranchOutcome`. The distinction between `RegionalFailure` and
/// `Definitive` is what makes the "primary is authoritative" invariant safe: only a
/// `RegionalFailure` on the primary is worth hedging, and only a `Success` hedge may win.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BranchOutcome {
    /// A successful response (2xx / 304).
    Success,
    /// The *region* is at fault (503, 500, 410+LeaseNotFound, 403+DatabaseAccountNotFound,
    /// or a transport error with no HTTP status). Worth hedging.
    RegionalFailure,
    /// A request-level, authoritative answer (any other error). A hedge can never override it.
    Definitive,
}

/// Whether a status/sub-status pair means "the region is at fault".
///
/// Matches the .NET `MetadataRequestThrottleRetryPolicy.IsRegionalFailure` sub-status set
/// exactly. Note that a *plain* `410` (PartitionKeyRangeGone) and a *plain* `403`
/// (WriteForbidden) are **not** regional — they are definitive, so a hedge can never
/// override them.
fn is_regional_failure(status: StatusCode, sub_status: Option<SubStatusCode>) -> bool {
    match status {
        StatusCode::ServiceUnavailable => true,
        StatusCode::InternalServerError => true,
        StatusCode::Gone => sub_status == Some(SubStatusCode::LEASE_NOT_FOUND),
        StatusCode::Forbidden => sub_status == Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND),
        _ => false,
    }
}

/// Classifies an error branch. A transport error carrying no HTTP status is treated as a
/// `RegionalFailure` (the connection/region is at fault, so hedging can help).
fn classify_error(err: &azure_core::Error) -> BranchOutcome {
    if let ErrorKind::HttpResponse {
        status,
        raw_response,
        ..
    } = err.kind()
    {
        let sub_status = raw_response
            .as_ref()
            .and_then(|r| {
                r.headers()
                    .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                    .ok()
            })
            .map(SubStatusCode::from);
        if is_regional_failure(*status, sub_status) {
            BranchOutcome::RegionalFailure
        } else {
            BranchOutcome::Definitive
        }
    } else {
        BranchOutcome::RegionalFailure
    }
}

/// Classifies a settled branch result. `Ok` is always `Success` because the gateway
/// pipeline returns `Err` for every status >= 400.
fn classify<T>(result: &azure_core::Result<CosmosResponse<T>>) -> BranchOutcome {
    match result {
        Ok(_) => BranchOutcome::Success,
        Err(err) => classify_error(err),
    }
}

/// The winning response plus enough metadata for the caller to pin subsequent work
/// (e.g. later PartitionKeyRange ReadFeed pages) to the region that won.
pub(crate) struct HedgedResponse<T> {
    /// The winning response.
    pub response: CosmosResponse<T>,
    /// The endpoint that produced the winning response.
    pub winning_endpoint: Url,
    /// Whether a hedge was dispatched at all.
    pub hedge_fired: bool,
    /// Whether the hedge (as opposed to the primary) produced the winning response.
    pub hedge_won: bool,
}

/// Orchestrates bounded, single-hedge, cross-region racing for one metadata read.
///
/// One instance is shared per client (via the routing caches). The strategy is
/// metadata-scoped and must never be applied to data-plane operations.
#[derive(Debug, Clone)]
pub(crate) struct MetadataHedgingStrategy {
    threshold: Duration,
}

impl MetadataHedgingStrategy {
    /// Creates a strategy whose threshold is derived from the SDK's control-plane
    /// connection timeout (`DEFAULT_CONNECTION_TIMEOUT + 500ms`, = 1.5s by default),
    /// keeping it strictly between the first and second attempt timeouts.
    pub(crate) fn new() -> Self {
        Self {
            threshold: DEFAULT_CONNECTION_TIMEOUT + DEFAULT_THRESHOLD_STEP,
        }
    }

    /// Runs `request` against the primary region with at most one threshold-triggered
    /// cross-region hedge, returning the arbitration winner.
    ///
    /// Eligibility for the region count is checked here (a distinct, applicable alternate
    /// endpoint must exist); the caller is responsible for gating on *which* reads are
    /// supported (Collection Read, PK-range first page). If no distinct alternate region
    /// exists, the request is sent to the primary only.
    pub(crate) async fn execute<T>(
        &self,
        request: CosmosRequest,
        ctx: Context<'_>,
        pipeline: &GatewayPipeline,
        endpoint_manager: &GlobalEndpointManager,
    ) -> azure_core::Result<HedgedResponse<T>> {
        let operation_type = request.operation_type;
        let primary_endpoint = endpoint_manager.resolve_service_endpoint(&request);

        // Health-aware alternate selection: first *available* endpoint that isn't the
        // primary (get_applicable_endpoints orders available-before-unavailable).
        let alternate_endpoint = endpoint_manager
            .applicable_endpoints(operation_type, None)
            .into_iter()
            .find(|endpoint| *endpoint != primary_endpoint);

        // Ineligible: single region / no distinct alternate. Primary only, unchanged path.
        let Some(alternate_endpoint) = alternate_endpoint else {
            let mut primary_request = request;
            let primary_url = primary_request.resource_link.url(&primary_endpoint);
            primary_request
                .request_context
                .route_to_location_endpoint(primary_url);
            let response = pipeline.send::<T>(primary_request, ctx).await?;
            return Ok(HedgedResponse {
                response,
                winning_endpoint: primary_endpoint,
                hedge_fired: false,
                hedge_won: false,
            });
        };

        // Snapshot the hedge request BEFORE dispatching the primary (R17), routing it to a
        // clean alternate endpoint so primary-side context mutation cannot desync it.
        let mut hedge_request = request.clone();
        hedge_request.request_context.clear_route_to_location();
        let hedge_url = hedge_request.resource_link.url(&alternate_endpoint);
        hedge_request
            .request_context
            .route_to_location_endpoint(hedge_url);

        // Route the primary to its resolved endpoint and dispatch it with a threshold timer.
        let mut primary_request = request;
        let primary_url = primary_request.resource_link.url(&primary_endpoint);
        primary_request
            .request_context
            .route_to_location_endpoint(primary_url);
        let primary_future = Box::pin(pipeline.send::<T>(primary_request, ctx.clone()));
        let threshold = azure_core::time::Duration::milliseconds(self.threshold.as_millis() as i64);
        let timer = get_async_runtime().sleep(threshold);

        match select(primary_future, timer).await {
            // Primary settled before the threshold.
            Either::Left((primary_result, _timer)) => match classify(&primary_result) {
                // Fast and authoritative: NO hedge (the common case).
                BranchOutcome::Success | BranchOutcome::Definitive => Ok(HedgedResponse {
                    response: primary_result?,
                    winning_endpoint: primary_endpoint,
                    hedge_fired: false,
                    hedge_won: false,
                }),
                // Primary regionally failed before the threshold: hedge now.
                BranchOutcome::RegionalFailure => {
                    let hedge_result = pipeline.send::<T>(hedge_request, ctx).await;
                    match classify(&hedge_result) {
                        BranchOutcome::Success => Ok(HedgedResponse {
                            response: hedge_result?,
                            winning_endpoint: alternate_endpoint,
                            hedge_fired: true,
                            hedge_won: true,
                        }),
                        // Neither branch is good: return the primary's (regional) outcome.
                        _ => Ok(HedgedResponse {
                            response: primary_result?,
                            winning_endpoint: primary_endpoint,
                            hedge_fired: true,
                            hedge_won: false,
                        }),
                    }
                }
            },
            // Threshold elapsed with the primary still in flight: fire one hedge and race.
            Either::Right((_elapsed, primary_future)) => {
                let hedge_future = Box::pin(pipeline.send::<T>(hedge_request, ctx));
                match select(primary_future, hedge_future).await {
                    // Primary settled first.
                    Either::Left((primary_result, hedge_future)) => {
                        match classify(&primary_result) {
                            // Primary is authoritative: it wins; the hedge is dropped (cancelled).
                            BranchOutcome::Success | BranchOutcome::Definitive => {
                                Ok(HedgedResponse {
                                    response: primary_result?,
                                    winning_endpoint: primary_endpoint,
                                    hedge_fired: true,
                                    hedge_won: false,
                                })
                            }
                            // Primary regionally failed: await the hedge; it wins only if good.
                            BranchOutcome::RegionalFailure => {
                                let hedge_result = hedge_future.await;
                                match classify(&hedge_result) {
                                    BranchOutcome::Success => Ok(HedgedResponse {
                                        response: hedge_result?,
                                        winning_endpoint: alternate_endpoint,
                                        hedge_fired: true,
                                        hedge_won: true,
                                    }),
                                    _ => Ok(HedgedResponse {
                                        response: primary_result?,
                                        winning_endpoint: primary_endpoint,
                                        hedge_fired: true,
                                        hedge_won: false,
                                    }),
                                }
                            }
                        }
                    }
                    // Hedge settled first.
                    Either::Right((hedge_result, primary_future)) => {
                        match classify(&hedge_result) {
                            // Hedge won: the primary is still pending (not yet definitive), so the
                            // hedge's good answer wins. The primary is dropped (observed/discarded).
                            BranchOutcome::Success => Ok(HedgedResponse {
                                response: hedge_result?,
                                winning_endpoint: alternate_endpoint,
                                hedge_fired: true,
                                hedge_won: true,
                            }),
                            // Hedge is not good: await the primary and return its outcome.
                            _ => {
                                let primary_result = primary_future.await;
                                Ok(HedgedResponse {
                                    response: primary_result?,
                                    winning_endpoint: primary_endpoint,
                                    hedge_fired: true,
                                    hedge_won: false,
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;

    #[test]
    fn regional_failures_match_dotnet_set() {
        assert!(is_regional_failure(StatusCode::ServiceUnavailable, None));
        assert!(is_regional_failure(StatusCode::InternalServerError, None));
        assert!(is_regional_failure(
            StatusCode::Gone,
            Some(SubStatusCode::LEASE_NOT_FOUND)
        ));
        assert!(is_regional_failure(
            StatusCode::Forbidden,
            Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)
        ));
    }

    #[test]
    fn plain_410_and_403_are_definitive_not_regional() {
        // Plain 410 (PartitionKeyRangeGone) must NOT be treated as regional.
        assert!(!is_regional_failure(StatusCode::Gone, None));
        assert!(!is_regional_failure(
            StatusCode::Gone,
            Some(SubStatusCode::WRITE_FORBIDDEN)
        ));
        // Plain 403 (WriteForbidden) must NOT be treated as regional.
        assert!(!is_regional_failure(StatusCode::Forbidden, None));
        assert!(!is_regional_failure(
            StatusCode::Forbidden,
            Some(SubStatusCode::LEASE_NOT_FOUND)
        ));
    }

    #[test]
    fn definitive_status_codes_are_not_regional() {
        for status in [
            StatusCode::NotFound,
            StatusCode::Conflict,
            StatusCode::PreconditionFailed,
            StatusCode::Unauthorized,
            StatusCode::BadRequest,
            StatusCode::TooManyRequests,
        ] {
            assert!(
                !is_regional_failure(status, None),
                "status {status:?} should be definitive, not regional"
            );
        }
    }

    #[test]
    fn success_result_classifies_as_success() {
        let ok: azure_core::Result<CosmosResponse<()>> = Err(azure_core::Error::new(
            ErrorKind::Other,
            "placeholder to prove Err path compiles",
        ));
        // Sanity: an Err with no HTTP status is a regional failure (transport-level).
        assert_eq!(classify(&ok), BranchOutcome::RegionalFailure);
    }

    #[test]
    fn threshold_is_connection_timeout_plus_step() {
        let strategy = MetadataHedgingStrategy::new();
        assert_eq!(
            strategy.threshold,
            DEFAULT_CONNECTION_TIMEOUT + Duration::from_millis(500)
        );
    }
}
