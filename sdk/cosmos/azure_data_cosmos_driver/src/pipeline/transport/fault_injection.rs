// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Collects fault-injection evaluations for individual transport attempts.

use crate::{
    diagnostics::{DiagnosticsContextBuilder, RequestHandle},
    fault_injection::EvaluationCollector,
    pipeline::transport::cosmos_transport_client::HttpRequest,
};

/// Attaches a new collector when enabled and returns a handle for reading its evaluations.
pub(super) fn attach_collector(
    mut request: HttpRequest,
    enabled: bool,
) -> (HttpRequest, Option<EvaluationCollector>) {
    let collector = if enabled {
        let collector = EvaluationCollector::default();
        request.evaluation_collector = Some(collector.clone());
        Some(collector)
    } else {
        None
    };
    (request, collector)
}

/// Drains collected evaluations into the attempt's diagnostics, leaving empty collections out.
///
/// Call after completion so the terminal record copied into the hedge journal stays unchanged.
pub(super) fn record_evaluations(
    collector: Option<EvaluationCollector>,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) {
    if let Some(collector) = collector {
        let evaluations = collector.take();
        if !evaluations.is_empty() {
            diagnostics.set_fault_injection_evaluations(handle, evaluations);
        }
    }
}
