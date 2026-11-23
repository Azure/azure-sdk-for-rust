// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Installs the run's [`TraceCapture`] as the global `tracing` subscriber.

use crate::trace_capture::{TraceCapture, TRIAL_TARGET};
use std::sync::OnceLock;
use tracing_subscriber::{
    filter::{filter_fn, EnvFilter, FilterExt},
    layer::SubscriberExt,
    Layer, Registry,
};

/// The filter used when `RUST_LOG` isn't set: debug output from the SDK
/// and cosspec, warnings and errors from everything else.
pub const DEFAULT_FILTER: &str =
    "warn,azure_core=debug,azure_identity=debug,azure_data_cosmos=debug,typespec=debug,cosspec=debug";

static CAPTURE: OnceLock<(TraceCapture, String)> = OnceLock::new();

/// The filter directives in effect: `rust_log` when set, else [`DEFAULT_FILTER`].
fn directives(rust_log: Option<&str>) -> String {
    match rust_log.map(str::trim) {
        Some(value) if !value.is_empty() => value.to_owned(),
        _ => DEFAULT_FILTER.to_owned(),
    }
}

/// The capture layer, filtered by `directives` plus every trial span.
fn subscriber(capture: &TraceCapture, directives: &str) -> impl tracing::Subscriber {
    let env = EnvFilter::try_new(directives).unwrap_or_else(|e| {
        eprintln!("warning: invalid RUST_LOG '{directives}': {e}; using '{DEFAULT_FILTER}'");
        EnvFilter::new(DEFAULT_FILTER)
    });
    // Trial spans are always enabled; without them, nothing is attributed.
    let filter = env.or(filter_fn(|meta| meta.target() == TRIAL_TARGET));
    Registry::default().with(capture.clone().with_filter(filter))
}

/// Installs the global subscriber, once. Returns the capture and the filter
/// directives in effect, or `None` if another global subscriber was already
/// installed.
pub(crate) fn install() -> Option<&'static (TraceCapture, String)> {
    if let Some(installed) = CAPTURE.get() {
        return Some(installed);
    }
    let directives = directives(std::env::var("RUST_LOG").ok().as_deref());
    let capture = TraceCapture::new();
    if let Err(e) = tracing::subscriber::set_global_default(subscriber(&capture, &directives)) {
        eprintln!("warning: spec logs aren't captured: {e}");
        return None;
    }
    Some(CAPTURE.get_or_init(|| (capture, directives)))
}

/// The installed capture, if any.
pub(crate) fn capture() -> Option<&'static TraceCapture> {
    CAPTURE.get().map(|(capture, _)| capture)
}

#[cfg(test)]
mod tests {
    use super::{directives, subscriber, DEFAULT_FILTER};
    use crate::trace_capture::TraceCapture;
    use std::time::Instant;
    use tracing::Level;

    /// Emits one event per case under `directives`; returns the captured
    /// `target level` pairs.
    fn captured(directives: &str) -> Vec<String> {
        let capture = TraceCapture::new();
        let _guard = tracing::subscriber::set_default(subscriber(&capture, directives));
        let (trace, span) = capture.begin("t", Instant::now());
        span.in_scope(|| {
            tracing::event!(target: "azure_data_cosmos_driver::driver", Level::DEBUG, "");
            tracing::event!(target: "azure_data_cosmos_driver::driver", Level::TRACE, "");
            tracing::event!(target: "azure_data_cosmos::clients", Level::DEBUG, "");
            tracing::event!(target: "azure_core::http", Level::DEBUG, "");
            tracing::event!(target: "typespec_client_core::http", Level::DEBUG, "");
            tracing::event!(target: "cosspec::pool", Level::DEBUG, "");
            tracing::event!(target: "hyper::proto", Level::INFO, "");
            tracing::event!(target: "hyper::proto", Level::WARN, "");
        });
        trace
            .finish()
            .logs
            .iter()
            .map(|log| format!("{} {}", log.target, log.level))
            .collect()
    }

    #[test]
    fn rust_log_replaces_the_default() {
        assert_eq!(directives(None), DEFAULT_FILTER);
        assert_eq!(directives(Some("  ")), DEFAULT_FILTER);
        assert_eq!(directives(Some("trace")), "trace");
    }

    #[test]
    fn default_is_debug_for_the_sdk_and_warn_otherwise() {
        assert_eq!(
            captured(DEFAULT_FILTER),
            [
                "azure_data_cosmos_driver::driver DEBUG",
                "azure_data_cosmos::clients DEBUG",
                "azure_core::http DEBUG",
                "typespec_client_core::http DEBUG",
                "cosspec::pool DEBUG",
                "hyper::proto WARN",
            ]
        );
    }

    #[test]
    fn trace_escalates_and_trial_spans_survive_off() {
        assert_eq!(captured("trace").len(), 8);
        assert!(captured("off").is_empty());
        let capture = TraceCapture::new();
        let _guard = tracing::subscriber::set_default(subscriber(&capture, "off"));
        let (trace, span) = capture.begin("t", Instant::now());
        assert!(!span.is_disabled());
        drop(trace);
    }
}
