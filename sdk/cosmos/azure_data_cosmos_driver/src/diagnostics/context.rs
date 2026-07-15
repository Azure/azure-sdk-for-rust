// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The operation-level diagnostics context.
//!
//! A [`DiagnosticsContext`] is the canonical, always-collected record the driver produces for each
//! operation. It rolls up the per-attempt [`RequestDiagnostics`] together with operation-level
//! outcome (status, duration, request charge, contacted regions) and exposes the predicates the SDK
//! layer uses to decide whether — and how — to emit telemetry.
//!
//! The context is produced by a [`DiagnosticsContextBuilder`] during the driver pipeline and is
//! immutable once [`complete`](DiagnosticsContextBuilder::complete) is called.

use super::request::RequestDiagnostics;
use crate::models::{ActivityId, CosmosStatus, RequestCharge};
use crate::options::{DiagnosticsOptions, DiagnosticsThresholds, DiagnosticsVerbosity, Region};
use azure_core::fmt::SafeDebug;
use azure_core::http::StatusCode;
use serde::Serialize;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Canonical operation-level diagnostics for a single Cosmos DB operation.
///
/// Obtain one from
/// [`CosmosResponse::diagnostics`](crate::models::CosmosResponse::diagnostics). The accessors and
/// predicates here are what the SDK layer consumes to make emission decisions (metrics, tracing,
/// logging) — the driver itself only ever produces the context, it never emits telemetry.
#[derive(Clone, SafeDebug)]
pub struct DiagnosticsContext {
    operation_name: String,
    activity_id: ActivityId,
    is_point_operation: bool,
    completed: bool,
    status: CosmosStatus,
    duration: Duration,
    total_request_charge: RequestCharge,
    contacted_regions: Vec<Region>,
    requests: Vec<RequestDiagnostics>,
    options: Arc<DiagnosticsOptions>,
}

impl DiagnosticsContext {
    /// Returns whether the operation reached a terminal status.
    ///
    /// This is `true` whenever the driver recorded a final status for the operation (the common
    /// case). It is `false` only for a context finalized without one, for example an operation that
    /// was abandoned before completing.
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Returns whether the operation's final status indicates failure.
    pub fn is_failure(&self) -> bool {
        !self.status.is_success()
    }

    /// Returns whether the operation crossed any of the supplied [`DiagnosticsThresholds`].
    ///
    /// Point operations are compared against the point-operation latency threshold and non-point
    /// operations (queries and feed reads) against the non-point threshold; the total request
    /// charge is compared against the request-charge threshold. Payload size is not tracked at this
    /// layer and is ignored.
    pub fn is_threshold_violated(&self, thresholds: &DiagnosticsThresholds) -> bool {
        let latency_threshold = if self.is_point_operation {
            thresholds.point_operation_latency_threshold()
        } else {
            thresholds.non_point_operation_latency_threshold()
        };
        if let Some(threshold) = latency_threshold {
            if self.duration >= threshold {
                return true;
            }
        }

        if let Some(charge_threshold) = thresholds.request_charge_threshold() {
            if self.total_request_charge >= charge_threshold {
                return true;
            }
        }

        false
    }

    /// Returns the total client-observed duration of the operation.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the final status of the operation.
    pub fn status(&self) -> CosmosStatus {
        self.status
    }

    /// Returns the total request charge (RU) consumed across every attempt.
    pub fn total_request_charge(&self) -> f64 {
        self.total_request_charge.value()
    }

    /// Returns the distinct regions contacted during the operation, in the order first contacted.
    pub fn contacted_regions(&self) -> &[Region] {
        &self.contacted_regions
    }

    /// Returns the logical operation name (for example `read_item`).
    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    /// Returns the client-generated activity id correlating the operation.
    pub fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    /// Returns the per-attempt request diagnostics.
    pub fn requests(&self) -> &[RequestDiagnostics] {
        &self.requests
    }

    /// Renders the context to a JSON string at the requested verbosity.
    ///
    /// [`DiagnosticsVerbosity::Default`] resolves to the verbosity configured on the context's
    /// [`DiagnosticsOptions`]. [`DiagnosticsVerbosity::Summary`] emits the operation-level roll-up
    /// only; [`DiagnosticsVerbosity::Detailed`] additionally includes every attempt.
    pub fn to_json_string(&self, verbosity: DiagnosticsVerbosity) -> String {
        let verbosity = self.resolve_verbosity(verbosity);
        let include_requests = matches!(verbosity, DiagnosticsVerbosity::Detailed);
        let view = DiagnosticsView {
            operation_name: &self.operation_name,
            activity_id: self.activity_id.as_str(),
            completed: self.completed,
            status: self.status.to_string(),
            duration_ms: duration_ms(self.duration),
            total_request_charge: self.total_request_charge.value(),
            contacted_regions: self.contacted_regions.iter().map(Region::as_str).collect(),
            requests: include_requests.then(|| self.requests.iter().map(request_view).collect()),
        };
        serde_json::to_string(&view).unwrap_or_else(|_| "{}".to_string())
    }

    /// Resolves a possibly-[`Default`](DiagnosticsVerbosity::Default) verbosity to a concrete level.
    fn resolve_verbosity(&self, verbosity: DiagnosticsVerbosity) -> DiagnosticsVerbosity {
        match verbosity {
            DiagnosticsVerbosity::Default => match self.options.default_verbosity() {
                DiagnosticsVerbosity::Default => DiagnosticsVerbosity::Detailed,
                other => other,
            },
            other => other,
        }
    }
}

/// Builds a [`DiagnosticsContext`] as an operation executes.
///
/// The builder is created at the start of an operation, fed one [`RequestDiagnostics`] per attempt
/// via [`record_request`](Self::record_request), and finalized with
/// [`complete`](Self::complete).
pub(crate) struct DiagnosticsContextBuilder {
    operation_name: String,
    activity_id: ActivityId,
    is_point_operation: bool,
    options: Arc<DiagnosticsOptions>,
    started_at: Instant,
    requests: Vec<RequestDiagnostics>,
}

impl DiagnosticsContextBuilder {
    /// Starts collecting diagnostics for an operation.
    pub(crate) fn new(
        operation_name: impl Into<String>,
        activity_id: ActivityId,
        is_point_operation: bool,
        options: Arc<DiagnosticsOptions>,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            activity_id,
            is_point_operation,
            options,
            started_at: Instant::now(),
            requests: Vec::new(),
        }
    }

    /// Records the diagnostics for one request attempt.
    pub(crate) fn record_request(&mut self, request: RequestDiagnostics) {
        self.requests.push(request);
    }

    /// Finalizes the context.
    ///
    /// `final_status` is `Some` for an operation that reached a terminal status (marking the context
    /// completed) and `None` for one that was abandoned, in which case the status falls back to the
    /// last recorded attempt (or a synthetic `500` when there were none).
    pub(crate) fn complete(self, final_status: Option<CosmosStatus>) -> DiagnosticsContext {
        let duration = self.started_at.elapsed();
        let completed = final_status.is_some();
        let status = final_status
            .or_else(|| self.requests.last().map(RequestDiagnostics::status))
            .unwrap_or_else(|| CosmosStatus::from_parts(StatusCode::from(500u16), None));

        let total_request_charge = self
            .requests
            .iter()
            .map(RequestDiagnostics::request_charge)
            .sum();

        let mut contacted_regions: Vec<Region> = Vec::new();
        for request in &self.requests {
            if let Some(region) = request.region() {
                if !contacted_regions.contains(region) {
                    contacted_regions.push(region.clone());
                }
            }
        }

        DiagnosticsContext {
            operation_name: self.operation_name,
            activity_id: self.activity_id,
            is_point_operation: self.is_point_operation,
            completed,
            status,
            duration,
            total_request_charge,
            contacted_regions,
            requests: self.requests,
            options: self.options,
        }
    }
}

/// Converts a [`Duration`] to whole milliseconds, saturating at [`u64::MAX`].
fn duration_ms(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

/// Serializable operation-level view produced by [`DiagnosticsContext::to_json_string`].
#[derive(Serialize)]
struct DiagnosticsView<'a> {
    operation_name: &'a str,
    activity_id: &'a str,
    completed: bool,
    status: String,
    duration_ms: u64,
    total_request_charge: f64,
    contacted_regions: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requests: Option<Vec<RequestView<'a>>>,
}

/// Serializable per-attempt view.
#[derive(Serialize)]
struct RequestView<'a> {
    execution_context: &'a str,
    endpoint: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<&'a str>,
    status: String,
    request_charge: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity_id: Option<&'a str>,
    duration_ms: u64,
}

/// Builds a [`RequestView`] borrowing from a [`RequestDiagnostics`].
fn request_view(request: &RequestDiagnostics) -> RequestView<'_> {
    RequestView {
        execution_context: request.execution_context().as_str(),
        endpoint: request.endpoint(),
        region: request.region().map(Region::as_str),
        status: request.status().to_string(),
        request_charge: request.request_charge().value(),
        activity_id: request.activity_id().map(ActivityId::as_str),
        duration_ms: duration_ms(request.duration()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::ExecutionContext;
    use azure_core::http::StatusCode;

    fn options() -> Arc<DiagnosticsOptions> {
        Arc::new(DiagnosticsOptions::default())
    }

    fn builder(is_point: bool) -> DiagnosticsContextBuilder {
        DiagnosticsContextBuilder::new(
            "read_item",
            ActivityId::from_static("op-activity"),
            is_point,
            options(),
        )
    }

    fn ok() -> CosmosStatus {
        CosmosStatus::new(StatusCode::Ok)
    }

    fn attempt(status: CosmosStatus, charge: f64, region: Region) -> RequestDiagnostics {
        RequestDiagnostics::new(ExecutionContext::Initial, "https://acct/", status)
            .with_region(region)
            .with_request_charge(RequestCharge::new(charge))
    }

    #[test]
    fn completed_success_predicates() {
        let mut b = builder(true);
        b.record_request(attempt(ok(), 2.0, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        assert!(ctx.is_completed());
        assert!(!ctx.is_failure());
        assert_eq!(ctx.status().status_code(), StatusCode::Ok);
        assert_eq!(ctx.operation_name(), "read_item");
        assert_eq!(ctx.activity_id().as_str(), "op-activity");
        assert_eq!(ctx.requests().len(), 1);
    }

    #[test]
    fn completed_failure_is_failure() {
        let mut b = builder(true);
        let err = CosmosStatus::new(StatusCode::TooManyRequests);
        b.record_request(attempt(err, 0.0, Region::EAST_US));
        let ctx = b.complete(Some(err));

        assert!(ctx.is_completed());
        assert!(ctx.is_failure());
    }

    #[test]
    fn not_completed_falls_back_to_last_attempt_status() {
        let mut b = builder(true);
        let err = CosmosStatus::new(StatusCode::ServiceUnavailable);
        b.record_request(attempt(err, 0.0, Region::EAST_US));
        let ctx = b.complete(None);

        assert!(!ctx.is_completed());
        assert!(ctx.is_failure());
        assert_eq!(ctx.status().status_code(), StatusCode::ServiceUnavailable);
    }

    #[test]
    fn not_completed_with_no_attempts_uses_synthetic_status() {
        let ctx = builder(true).complete(None);
        assert!(!ctx.is_completed());
        assert_eq!(ctx.status().status_code(), StatusCode::from(500u16));
    }

    #[test]
    fn total_request_charge_sums_attempts() {
        let mut b = builder(false);
        b.record_request(attempt(ok(), 1.5, Region::WEST_US_2));
        b.record_request(attempt(ok(), 2.5, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        assert_eq!(ctx.total_request_charge(), 4.0);
    }

    #[test]
    fn contacted_regions_are_deduplicated_in_order() {
        let mut b = builder(false);
        b.record_request(attempt(ok(), 0.0, Region::WEST_US_2));
        b.record_request(attempt(ok(), 0.0, Region::EAST_US));
        b.record_request(attempt(ok(), 0.0, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        assert_eq!(
            ctx.contacted_regions(),
            &[Region::WEST_US_2, Region::EAST_US]
        );
    }

    #[test]
    fn point_latency_threshold_violation() {
        let mut b = builder(true);
        b.record_request(attempt(ok(), 0.0, Region::WEST_US_2));
        let mut ctx = b.complete(Some(ok()));
        // Force a deterministic duration for the predicate.
        ctx.duration = Duration::from_millis(200);

        let thresholds = DiagnosticsThresholds::new()
            .with_point_operation_latency_threshold(Duration::from_millis(100));
        assert!(ctx.is_threshold_violated(&thresholds));

        // A non-point threshold must not apply to a point operation.
        let non_point = DiagnosticsThresholds::new()
            .with_non_point_operation_latency_threshold(Duration::from_millis(100));
        assert!(!ctx.is_threshold_violated(&non_point));
    }

    #[test]
    fn request_charge_threshold_violation() {
        let mut b = builder(false);
        b.record_request(attempt(ok(), 6.0, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        let thresholds =
            DiagnosticsThresholds::new().with_request_charge_threshold(RequestCharge::new(5.0));
        assert!(ctx.is_threshold_violated(&thresholds));

        let higher =
            DiagnosticsThresholds::new().with_request_charge_threshold(RequestCharge::new(10.0));
        assert!(!ctx.is_threshold_violated(&higher));
    }

    #[test]
    fn to_json_detailed_includes_requests() {
        let mut b = builder(true);
        b.record_request(
            attempt(ok(), 3.0, Region::WEST_US_2)
                .with_activity_id(ActivityId::from_static("svc-1"))
                .with_duration(Duration::from_millis(7)),
        );
        let ctx = b.complete(Some(ok()));

        let json = ctx.to_json_string(DiagnosticsVerbosity::Detailed);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["operation_name"], "read_item");
        assert_eq!(value["completed"], true);
        assert_eq!(value["total_request_charge"], 3.0);
        assert_eq!(value["requests"].as_array().unwrap().len(), 1);
        assert_eq!(value["requests"][0]["execution_context"], "initial");
        assert_eq!(value["requests"][0]["region"], "westus2");
    }

    #[test]
    fn to_json_summary_omits_requests() {
        let mut b = builder(true);
        b.record_request(attempt(ok(), 3.0, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        let json = ctx.to_json_string(DiagnosticsVerbosity::Summary);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value.get("requests").is_none());
        assert_eq!(value["contacted_regions"][0], "westus2");
    }

    #[test]
    fn to_json_default_resolves_to_options_default() {
        // The default options resolve `Default` to `Detailed`, which includes requests.
        let mut b = builder(true);
        b.record_request(attempt(ok(), 1.0, Region::WEST_US_2));
        let ctx = b.complete(Some(ok()));

        let json = ctx.to_json_string(DiagnosticsVerbosity::Default);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(value.get("requests").is_some());
    }
}
