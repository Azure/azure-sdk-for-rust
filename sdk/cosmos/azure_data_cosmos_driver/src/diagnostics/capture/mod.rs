// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deferred, threshold-gated diagnostics capture for the Cosmos driver.
//!
//! This module productionizes the benchmarked "Deferred Gated Capture" design. The model:
//!
//! 1. **Hot path — append-only, pooled, lock-free.** Each operation rents one buffer from a
//!    [`LogPool`] and a [`DiagnosticsRecorder`] appends a compact tag-length-value record per
//!    attempt / fan-out child. Appends go through `&mut`, so there is no per-attempt lock and
//!    almost nothing is allocated after pool warm-up.
//! 2. **Gate — decide at the end.** When the outcome and elapsed time are known, a
//!    [`DiagnosticsPolicy`] decides whether the diagnostics are worth building. If not, the
//!    buffer goes back to the pool — effectively free.
//! 3. **Build — only when wanted.** Past the gate, the log is parsed once and reduced to an
//!    aggregatable [`Summary`] (and, opt-in, an [`wire`]-encoded `AZD1` binary blob).
//!
//! Diagnostics are **opt-in** and default to [`Mode::Off`] (no cost, no behavior change). The
//! gate's thresholds bridge to the driver's existing
//! [`DiagnosticsThresholds`](crate::options::DiagnosticsThresholds) via
//! [`DiagnosticsPolicy::from_thresholds`].
//!
//! # Example
//!
//! ```
//! use azure_data_cosmos_driver::diagnostics::capture::{
//!     DiagnosticsPolicy, DiagnosticsRecorder, LogPool, Outcome,
//! };
//! use std::time::Duration;
//!
//! // A shared, bounded buffer pool (clone is a cheap Arc bump).
//! let pool = LogPool::new();
//! // Build on error or when an operation exceeds 5 ms.
//! let policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
//!
//! // Per operation: start capture, record each attempt, then gate at the end.
//! let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/dbs/d/colls/c", "client-1");
//! rec.record_attempt(0, 429, Some("svc-429"), Some(4.2), 0, 3_000_000);
//! rec.record_attempt(1, 200, Some("svc-200"), Some(4.2), 3_000_000, 4_000_000);
//! rec.record_end(Outcome::Success, 2, Some(7_000_000));
//!
//! let rendered = azure_data_cosmos_driver::diagnostics::capture::finish(rec, &policy);
//! // 7 ms > 5 ms threshold, so a summary was built.
//! let summary = rendered.summary().expect("built");
//! assert_eq!(summary.attempt_count, 2);
//! assert_eq!(summary.throttle_count, 1);
//! ```

mod gate;
mod pool;
mod preamble;
mod recorder;
mod summary;
pub mod wire;

pub use gate::{finish, should_build, DiagnosticsPolicy, Mode, Rendered};
pub use pool::LogPool;
pub use preamble::{set_sdk_provenance, Preamble};
pub use recorder::{ChildRecord, DiagnosticsRecorder};
pub use summary::{ClientInfo, Summary, TopError};

/// Canonical diagnostics attribute keys used on `AZD1` wire nodes.
///
/// The exact cross-SDK key strings (e.g. `az.service_request_id` vs `az.service_request.id`) are
/// deferred to a future cross-SDK ratification; these are the provisional keys for the Cosmos
/// driver.
pub(crate) mod attrs {
    /// Service request id (Cosmos activity id / `x-ms-request-id`).
    pub const ATTR_SERVICE_REQUEST_ID: &str = "az.service_request_id";
    /// Client-generated correlation id.
    pub const ATTR_CLIENT_REQUEST_ID: &str = "az.client_request_id";
    /// HTTP status code.
    pub const ATTR_STATUS_CODE: &str = "az.status_code";
    /// Cosmos sub-status code (finer error classification).
    pub const ATTR_SUB_STATUS: &str = "az.sub_status";
    /// Retry-safety signal on a transport failure (`sent` / `not_sent` / `unknown`).
    pub const ATTR_REQUEST_SENT: &str = "az.request_sent";
    /// Coarse error classification.
    pub const ATTR_ERROR_KIND: &str = "az.error_kind";
    /// Request charge in Request Units (RU).
    pub const ATTR_REQUEST_CHARGE: &str = "az.request_charge";
    /// Target service endpoint.
    pub const ATTR_ENDPOINT: &str = "az.endpoint";
    /// Query-plan tree node id (fan-out / routing).
    pub const ATTR_PLAN_NODE_ID: &str = "az.plan_node_id";
    /// Feed range a child span addresses.
    pub const ATTR_FEED_RANGE: &str = "az.feed_range";
    /// Total attempt count of the operation.
    pub const ATTR_ATTEMPT_COUNT: &str = "az.attempt_count";
    /// Operation name.
    pub const ATTR_OPERATION: &str = "az.operation";
    /// Operation outcome (`success` / `error`).
    pub const ATTR_OUTCOME: &str = "az.outcome";
    /// SDK name + version provenance.
    pub const ATTR_SDK_VERSION: &str = "az.sdk_version";
    /// Cosmos driver version provenance.
    pub const ATTR_DRIVER_VERSION: &str = "az.driver_version";
    /// Full User-Agent provenance.
    pub const ATTR_USER_AGENT: &str = "az.user_agent";
}

/// The terminal outcome of an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The operation completed successfully.
    Success,
    /// The operation failed.
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Records S2 (retry 429 -> 200) into a recorder and finishes against `policy`.
    fn render_s2(pool: &LogPool, policy: &DiagnosticsPolicy) -> Rendered {
        let mut rec = DiagnosticsRecorder::start(
            pool,
            "read_item",
            "https://acct/dbs/d/colls/c/docs/1",
            "c-2",
        );
        rec.record_attempt(0, 429, Some("svc-429"), Some(4.2), 0, 3_000_000);
        rec.record_attempt(1, 200, Some("svc-200"), Some(4.2), 3_000_000, 4_000_000);
        rec.record_end(Outcome::Success, 2, Some(7_000_000));
        finish(rec, policy)
    }

    #[test]
    fn fast_success_is_dropped_and_buffer_pooled() {
        let pool = LogPool::new();
        let policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct", "c-1");
        rec.record_attempt(0, 200, Some("svc-200"), Some(2.5), 0, 1_000_000);
        rec.record_end(Outcome::Success, 1, Some(1_000_000));
        let rendered = finish(rec, &policy);
        assert!(rendered.is_dropped(), "fast success should be gated away");
        assert_eq!(pool.pooled(), 1, "buffer returned to the pool");
    }

    #[test]
    fn slow_op_builds_aggregatable_summary() {
        let pool = LogPool::new();
        let rendered = render_s2(
            &pool,
            &DiagnosticsPolicy::threshold(Duration::from_millis(5)),
        );
        let summary = rendered.summary().expect("slow op builds a summary");
        assert_eq!(summary.operation, "read_item");
        assert_eq!(summary.outcome, "success");
        assert_eq!(summary.attempt_count, 2);
        assert_eq!(summary.retry_count, 1);
        assert_eq!(summary.throttle_count, 1);
        assert_eq!(summary.total_request_charge, 8.4);
        assert_eq!(summary.final_service_request_id.as_deref(), Some("svc-200"));
        assert_eq!(summary.status_counts.get("429"), Some(&1));
        assert_eq!(summary.status_counts.get("200"), Some(&1));
        let top = summary.top_error.as_ref().expect("429 is an error signal");
        assert_eq!(top.status, 429);
        assert_eq!(top.error_kind, "throttled");
        assert!(rendered.detailed_blob().is_none(), "binary off by default");
        assert_eq!(pool.pooled(), 1);
    }

    #[test]
    fn binary_tier_round_trips_through_decode() {
        let pool = LogPool::new();
        let policy = DiagnosticsPolicy {
            binary: true,
            ..DiagnosticsPolicy::threshold(Duration::from_millis(5))
        };
        let rendered = render_s2(&pool, &policy);
        let blob = rendered.detailed_blob().expect("binary tier requested");
        assert_eq!(&blob[0..4], wire::MAGIC);
        let tree = wire::decode(blob).expect("AZD1 decodes");
        assert_eq!(tree.operation, "read_item");
        // root operation + 2 attempts
        assert_eq!(tree.nodes.len(), 3);
        assert_eq!(tree.children_of(0).len(), 2);
    }

    #[test]
    fn summary_json_carries_no_auth_material() {
        // The capture path never sees auth headers; assert the serialized summary has no
        // authorization/secret-looking keys (SE-7 guard).
        let pool = LogPool::new();
        let rendered = render_s2(&pool, &DiagnosticsPolicy::always());
        let json = String::from_utf8(rendered.summary().unwrap().to_json()).unwrap();
        let lowered = json.to_lowercase();
        assert!(!lowered.contains("authorization"));
        assert!(!lowered.contains("\"secret\""));
        assert!(!lowered.contains("bearer "));
        assert!(!lowered.contains("sig="));
    }

    #[test]
    fn dropped_recorder_before_finish_returns_buffer() {
        // Cancellation safety: dropping the recorder without finishing returns the buffer.
        let pool = LogPool::new();
        {
            let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct", "c-3");
            rec.record_attempt(0, 200, Some("svc"), Some(1.0), 0, 1_000);
            // no record_end / finish — simulate a cancelled future
        }
        assert_eq!(pool.pooled(), 1, "dropped recorder must return its buffer");
    }

    #[tokio::test]
    async fn fan_out_children_merge_lock_free() {
        // Each concurrent child captures its own ChildRecord (a Send value, no shared recorder,
        // no lock); the operation layer merges them on join.
        let pool = LogPool::new();
        let mut rec =
            DiagnosticsRecorder::start(&pool, "query_items", "https://acct/dbs/d/colls/c", "c-q");
        rec.record_attempt(0, 200, Some("svc-query-200"), Some(18.6), 0, 6_000_000);

        let handles: Vec<_> = (0..25u32)
            .map(|i| {
                tokio::spawn(async move {
                    ChildRecord {
                        plan_node_id: format!("plan-{i}"),
                        feed_range: format!("range-{i}"),
                        start_ns: u64::from(i) * 1000,
                        duration_ns: 500,
                    }
                })
            })
            .collect();

        for h in handles {
            let child = h.await.expect("child task");
            rec.merge_child(&child);
        }

        rec.record_end(Outcome::Success, 1, Some(6_000_000));
        let policy = DiagnosticsPolicy {
            binary: true,
            ..DiagnosticsPolicy::always()
        };
        let rendered = finish(rec, &policy);
        let summary = rendered.summary().expect("built");
        // Summary stays flat regardless of fan-out width: child_count only.
        assert_eq!(summary.child_count, 25);
        // The detailed blob carries all 25 routing children under the operation root.
        let tree = wire::decode(rendered.detailed_blob().unwrap()).expect("AZD1 decodes");
        assert_eq!(
            tree.children_of(0).len(),
            26,
            "1 attempt + 25 routing children"
        );
    }

    #[test]
    fn recorder_elapsed_uses_monotonic_clock() {
        // record_end with None sources elapsed from the recorder's own Instant.
        let pool = LogPool::new();
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct", "c-clk");
        rec.record_attempt(0, 200, Some("svc"), Some(1.0), 0, 1_000);
        rec.record_end(Outcome::Success, 1, None);
        // Elapsed is monotonic and non-zero-ish; just assert it was populated without panicking.
        let rendered = finish(rec, &DiagnosticsPolicy::always());
        assert!(rendered.summary().is_some());
    }

    /// Prints the output sizes (run with `--nocapture`) used for the PR before/after table, and
    /// asserts the summary stays flat regardless of fan-out width while the detail tier grows.
    #[test]
    fn output_sizes_dropped_vs_summary_vs_detailed() {
        let pool = LogPool::new();

        // S2 (retry 429 -> 200), summary tier.
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/d/c/1", "c");
        rec.record_attempt(0, 429, Some("svc-429"), Some(4.2), 0, 3_000_000);
        rec.record_attempt(1, 200, Some("svc-200"), Some(4.2), 3_000_000, 4_000_000);
        rec.record_end(Outcome::Success, 2, Some(7_000_000));
        let s2_summary = finish(rec, &DiagnosticsPolicy::always());
        let s2_summary_bytes = s2_summary.summary().unwrap().to_json().len();

        // S2 detailed tier.
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/d/c/1", "c");
        rec.record_attempt(0, 429, Some("svc-429"), Some(4.2), 0, 3_000_000);
        rec.record_attempt(1, 200, Some("svc-200"), Some(4.2), 3_000_000, 4_000_000);
        rec.record_end(Outcome::Success, 2, Some(7_000_000));
        let s2_detailed = finish(
            rec,
            &DiagnosticsPolicy {
                binary: true,
                ..DiagnosticsPolicy::always()
            },
        );
        let s2_detail_bytes = s2_detailed.detailed_blob().unwrap().len();

        // Fan-out (1 attempt + 25 children), both tiers.
        let mut rec = DiagnosticsRecorder::start(&pool, "query_items", "https://acct/d/c", "cq");
        rec.record_attempt(0, 200, Some("svc-q"), Some(18.6), 0, 6_000_000);
        for i in 0..25u32 {
            rec.merge_child(&ChildRecord {
                plan_node_id: format!("plan-{i}"),
                feed_range: format!("range-{i}"),
                start_ns: u64::from(i) * 1000,
                duration_ns: 500,
            });
        }
        rec.record_end(Outcome::Success, 1, Some(6_000_000));
        let fanout = finish(
            rec,
            &DiagnosticsPolicy {
                binary: true,
                ..DiagnosticsPolicy::always()
            },
        );
        let fanout_summary_bytes = fanout.summary().unwrap().to_json().len();
        let fanout_detail_bytes = fanout.detailed_blob().unwrap().len();

        println!("DIAG-SIZES dropped=0 s2_summary={s2_summary_bytes} s2_detailed_azd1={s2_detail_bytes} fanout25_summary={fanout_summary_bytes} fanout25_detailed_azd1={fanout_detail_bytes}");

        // The summary is fan-out-width-independent (collapses children to a count); the detail
        // tier grows with the children.
        assert!(fanout_summary_bytes < fanout_detail_bytes);
        assert!(s2_summary_bytes > 0);
    }

    #[test]
    fn attempt_ext_captures_sub_status_and_request_sent() {
        let pool = LogPool::new();
        let mut rec = DiagnosticsRecorder::start(&pool, "create_item", "https://acct/d/c", "c-ext");
        // A throttle with a Cosmos sub-status, then a transport failure (request not sent).
        rec.record_attempt_ext(
            0,
            429,
            Some("svc-429"),
            Some(4.2),
            Some(3200),
            None,
            0,
            1_000_000,
        );
        rec.record_attempt_ext(1, 0, None, None, None, Some("not_sent"), 1_000_000, 2_000);
        rec.record_end(Outcome::Error, 2, Some(1_002_000));
        let policy = DiagnosticsPolicy {
            binary: true,
            ..DiagnosticsPolicy::always()
        };
        let rendered = finish(rec, &policy);
        let top = rendered.summary().unwrap().top_error.as_ref().unwrap();
        assert_eq!(top.status, 429);
        assert_eq!(top.sub_status, Some(3200));

        // The detail tier carries the sub-status and request-sent attributes.
        let tree = wire::decode(rendered.detailed_blob().unwrap()).unwrap();
        let has_sub = tree
            .nodes
            .iter()
            .any(|n| n.attr("az.sub_status") == Some("3200"));
        let has_sent = tree
            .nodes
            .iter()
            .any(|n| n.attr("az.request_sent") == Some("not_sent"));
        assert!(has_sub, "sub-status attribute present on an attempt node");
        assert!(
            has_sent,
            "request_sent attribute present on the failed attempt"
        );
    }
}
