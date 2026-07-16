// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Rate-limited, tail-sampled diagnostics logging for Cosmos DB operations.
//!
//! Provides [`SamplingLogHandler`], a [`DiagnosticsHandler`](crate::diagnostics::DiagnosticsHandler)
//! that logs a compact diagnostics line for interesting operations (failures and
//! threshold breaches) while capping the emission rate during storms via a
//! reusable [count-per-interval limiter](rate_limiter::RateLimiter).

mod handler;
mod rate_limiter;

pub use handler::SamplingLogHandler;

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    use azure_core::http::{Context, StatusCode};
    use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, RequestDiagnostics};
    use azure_data_cosmos_driver::models::{ActivityId, RequestCharge};
    use azure_data_cosmos_driver::options::Region;
    use azure_data_cosmos_driver::{CosmosStatus, DiagnosticsThresholds};
    use tracing_subscriber::layer::{Context as LayerContext, SubscriberExt};
    use tracing_subscriber::Layer;

    use super::handler::{should_log, SamplingLogHandler};
    use super::rate_limiter::RateLimiterConfig;
    use crate::diagnostics::{CosmosOperationContext, DiagnosticsHandler};

    /// Builds a completed single-attempt context with the given duration, status,
    /// and request charge.
    fn context(duration: Duration, status: CosmosStatus, charge: f64) -> DiagnosticsContext {
        let now = Instant::now();
        let request = RequestDiagnostics::for_testing(
            "https://acct.documents.azure.com:443/",
            Some(Region::new("West US 2")),
            status,
            RequestCharge::new(charge),
            now - duration,
            now,
        );
        DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            duration,
            Some(status),
            Some("read_item"),
            vec![request],
        )
    }

    #[test]
    fn gate_skips_fast_success_and_admits_failures_and_breaches() {
        let thresholds = DiagnosticsThresholds::default();

        // Fast, cheap success: nothing to log.
        let ok = context(
            Duration::from_millis(5),
            CosmosStatus::new(StatusCode::Ok),
            2.0,
        );
        assert!(!should_log(&ok, &thresholds, None));

        // Failure: log.
        let failed = context(
            Duration::from_millis(5),
            CosmosStatus::new(StatusCode::TooManyRequests),
            2.0,
        );
        assert!(should_log(&failed, &thresholds, None));

        // Threshold breach (RU over a low threshold): log.
        let strict = DiagnosticsThresholds::default().with_request_charge(1.0);
        let expensive = context(
            Duration::from_millis(5),
            CosmosStatus::new(StatusCode::Ok),
            500.0,
        );
        assert!(should_log(&expensive, &strict, None));
    }

    #[test]
    fn non_point_threshold_uses_operation_name_from_context() {
        // A production driver context carries no operation name, so a 2s
        // operation classifies as a point op (1s) and is wrongly logged. The
        // SDK operation identity switches it to the non-point (3s) threshold.
        let thresholds = DiagnosticsThresholds::default();
        let now = Instant::now();
        let request = RequestDiagnostics::for_testing(
            "https://acct.documents.azure.com:443/",
            Some(Region::new("West US 2")),
            CosmosStatus::new(StatusCode::Ok),
            RequestCharge::new(2.0),
            now - Duration::from_millis(2000),
            now,
        );
        let slow_non_point = DiagnosticsContext::for_testing_with_requests(
            ActivityId::new_uuid(),
            Duration::from_millis(2000),
            Some(CosmosStatus::new(StatusCode::Ok)),
            None,
            vec![request],
        );

        // No operation identity → point (1s) fallback → logged.
        assert!(should_log(&slow_non_point, &thresholds, None));
        // Query op identity → non-point (3s) threshold → not logged at 2s.
        let op = CosmosOperationContext::new().with_operation_name("query_items");
        assert!(!should_log(&slow_non_point, &thresholds, Some(&op)));
    }

    /// Counts sampled lines and suppression notices by `tracing` target.
    struct CountingLayer {
        sampled: Arc<AtomicUsize>,
        suppressed: Arc<AtomicUsize>,
    }

    impl<S: tracing::Subscriber> Layer<S> for CountingLayer {
        fn on_event(&self, event: &tracing::Event<'_>, _cx: LayerContext<'_, S>) {
            match event.metadata().target() {
                "azure_data_cosmos::diagnostics::sampled" => {
                    self.sampled.fetch_add(1, Ordering::SeqCst);
                }
                "azure_data_cosmos::diagnostics::suppressed" => {
                    self.suppressed.fetch_add(1, Ordering::SeqCst);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn storm_is_capped_with_one_suppression_notice() {
        let sampled = Arc::new(AtomicUsize::new(0));
        let suppressed = Arc::new(AtomicUsize::new(0));
        let layer = CountingLayer {
            sampled: Arc::clone(&sampled),
            suppressed: Arc::clone(&suppressed),
        };
        let subscriber = tracing_subscriber::registry().with(layer);

        // Cap 5 per 200ms window, no failure reserve. Drive threshold-breaching
        // successes (RU 2 > 1) so they log without tapping the failure reserve.
        let handler = SamplingLogHandler::with_config(
            DiagnosticsThresholds::default().with_request_charge(1.0),
            RateLimiterConfig {
                max_per_window: 5,
                window: Duration::from_millis(200),
                failure_reserve: 0,
            },
        );
        let ctx = context(
            Duration::from_millis(5),
            CosmosStatus::new(StatusCode::Ok),
            2.0,
        );

        tracing::subscriber::with_default(subscriber, || {
            // 30 events in one window: only 5 admitted, 25 suppressed.
            for _ in 0..30 {
                handler.handle(&ctx, &Context::new());
            }
            // Roll into the next window and log once more, flushing the notice.
            std::thread::sleep(Duration::from_millis(250));
            handler.handle(&ctx, &Context::new());
        });

        // 5 (window 1) + 1 (window 2) sampled lines, and exactly one notice.
        assert_eq!(sampled.load(Ordering::SeqCst), 6);
        assert_eq!(suppressed.load(Ordering::SeqCst), 1);
    }
}
