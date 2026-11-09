// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for error outcomes.

pub(crate) const MAX_SERVICE_DETAIL_LEN: usize = 512;

use crate::{
    models::{CosmosResponseHeaders, CosmosStatus},
    pipeline::operation::recovery,
};

/// Formats the human-readable message for a Cosmos HTTP error status.
pub(crate) fn service_error_message(status: &CosmosStatus) -> String {
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };
    format!(
        "Cosmos DB returned HTTP {}{}: {}",
        u16::from(status.status_code()),
        sub_status_str,
        status.name().unwrap_or("Unknown"),
    )
}

/// Extracts the service's human-readable explanation from an error response
/// body so it can be folded into the error message.
///
/// Cosmos error payloads are shaped like
/// `{"code":"BadRequest","message":"Message: {\"Errors\":[\"...\"]}\r\nActivityId: ..., Request URI: ..."}`.
/// Only the `Errors` text carries information the caller can act on — the
/// activity ID is already exposed as typed state on the error, and the request
/// URI and SDK banner identify service-internal replicas — so those trailers
/// are stripped. Bodies that are not JSON (or not shaped as expected) fall back
/// to the raw text so nothing the service said is lost; the untouched payload
/// (trailers included) is always retrievable via
/// [`CosmosError::response`](crate::error::CosmosError::response).
///
/// The result is normalized to a single line (see [`normalize_single_line`])
/// before it is truncated, so a service-controlled body cannot inject line
/// breaks or control characters into a log record through
/// [`CosmosError`](crate::error::CosmosError)'s single-line `Display`.
///
/// Returns `None` when the body is empty or carries no usable text.
pub(crate) fn service_body_detail(body: &[u8]) -> Option<String> {
    let text = std::str::from_utf8(body).ok()?.trim();
    if text.is_empty() {
        return None;
    }

    let detail = match serde_json::from_str::<serde_json::Value>(text) {
        Ok(value) => match value.get("message").and_then(serde_json::Value::as_str) {
            Some(message) => recovery::error::condense_service_message(message),
            // Valid JSON without a `message` field: keep the payload as-is.
            None => text.to_string(),
        },
        Err(_) => text.to_string(),
    };

    // Normalize before truncating so the byte budget applies to the text that
    // is actually rendered.
    let detail = recovery::error::normalize_single_line(&detail);
    if detail.is_empty() {
        return None;
    }

    Some(recovery::error::truncate_detail(&detail))
}

/// Collapses `text` onto a single line: every control character (CR, LF, TAB,
/// other C0/C1 codes, and DEL) becomes a space, and runs of whitespace collapse
/// to one space.
///
/// The service controls this text, so this is what keeps it from forging or
/// mangling log records emitted through
/// [`CosmosError`](crate::error::CosmosError)'s single-line `Display`.
pub(crate) fn normalize_single_line(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;

    for ch in text.chars() {
        if ch.is_control() || ch.is_whitespace() {
            // Defer the separator so trailing whitespace never lands in `out`.
            pending_space = !out.is_empty();
            continue;
        }
        if pending_space {
            out.push(' ');
            pending_space = false;
        }
        out.push(ch);
    }

    out
}

/// Reduces a service `message` field to the actionable text: drops the
/// `Message: ` prefix and the `ActivityId: ...` trailer, and unwraps the
/// nested `{"Errors":[...]}` envelope when present.
pub(crate) fn condense_service_message(message: &str) -> String {
    let head = match message.find("\r\nActivityId:") {
        Some(index) => &message[..index],
        None => match message.find("\nActivityId:") {
            Some(index) => &message[..index],
            None => message,
        },
    };
    let head = head.trim().strip_prefix("Message:").unwrap_or(head).trim();

    // The inner envelope is itself JSON, e.g. `{"Errors":["..."]}`.
    if let Ok(inner) = serde_json::from_str::<serde_json::Value>(head) {
        if let Some(errors) = inner.get("Errors").and_then(serde_json::Value::as_array) {
            let joined = errors
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
                .join("; ");
            if !joined.is_empty() {
                return joined;
            }
        }
    }

    head.to_string()
}

/// Truncates `detail` to [`MAX_SERVICE_DETAIL_LEN`] bytes on a character
/// boundary, appending an ellipsis when anything was dropped. The ellipsis is
/// counted against the budget, so the result never exceeds the cap.
pub(crate) fn truncate_detail(detail: &str) -> String {
    const ELLIPSIS: &str = "...";

    if detail.len() <= MAX_SERVICE_DETAIL_LEN {
        return detail.to_string();
    }

    let mut end = MAX_SERVICE_DETAIL_LEN - ELLIPSIS.len();
    while end > 0 && !detail.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}{ELLIPSIS}", &detail[..end])
}

/// Builds an operation error retaining service status, headers, and body.
///
/// Includes bounded service detail in the message and recognizes legacy query
/// rejection bodies. Diagnostics are attached by response finalization, not classification.
pub(crate) fn build_service_error(
    status: &CosmosStatus,
    cosmos_headers: &CosmosResponseHeaders,
    body: &[u8],
) -> crate::error::CosmosError {
    // Some gateway versions return HTTP 400 for cross-partition queries with
    // unsupported features (ORDER BY, aggregates, GROUP BY, ...) without
    // emitting the `x-ms-substatus: 1004` header that the .NET / Java SDKs
    // rely on. Detect that case from the response body and synthesize the
    // canonical [`crate::error::status_codes::CROSS_PARTITION_QUERY_NOT_SERVABLE`] so
    // callers get a consistent typed status regardless of gateway version.
    let effective_status = recovery::error::synthesize_cross_partition_query_status(*status, body);
    let mut message = recovery::error::service_error_message(&effective_status);
    if let Some(detail) = recovery::error::service_body_detail(body) {
        message.push_str(". Details: ");
        message.push_str(&detail);
    }
    crate::error::CosmosError::builder()
        .with_status(effective_status)
        .with_message(message)
        .with_response_parts(crate::models::CosmosResponsePayload::new(
            body.to_vec(),
            cosmos_headers.clone(),
        ))
        .build()
}

/// Returns [`crate::error::status_codes::CROSS_PARTITION_QUERY_NOT_SERVABLE`] when `status`
/// is a bare HTTP 400 (no sub-status) and `body` is the gateway's
/// "unsupported query features" rejection. Otherwise returns `status`
/// unchanged.
pub(crate) fn synthesize_cross_partition_query_status(
    status: CosmosStatus,
    body: &[u8],
) -> CosmosStatus {
    use azure_core::http::StatusCode;
    if status.status_code() != StatusCode::BadRequest || status.sub_status().is_some() {
        return status;
    }
    let Ok(text) = std::str::from_utf8(body) else {
        return status;
    };

    // Match the gateway's well-known message rather than parsing JSON to
    // avoid a serde dependency on the hot error path. The fragment is
    // stable across .NET / Java / Python emulator gateways.
    if text.contains("unsupported features") && text.contains("Upgrade your SDK") {
        crate::error::status_codes::CROSS_PARTITION_QUERY_NOT_SERVABLE
    } else {
        status
    }
}

/// Preserves the transport error source in the operation error representation.
pub(crate) fn build_transport_error(
    status: &CosmosStatus,
    error: crate::error::CosmosError,
) -> crate::error::CosmosError {
    let status_code = status.status_code();
    let name = status.name().unwrap_or("Unknown");
    let sub_status_str = match status.sub_status() {
        Some(s) => format!("/{}", s.value()),
        None => String::new(),
    };

    let detail_summary = crate::driver::error_chain_summary(&error);
    let message = format!(
        "Cosmos DB transport failure HTTP {}{}: {}. Details: {}",
        u16::from(status_code),
        sub_status_str,
        name,
        detail_summary,
    );

    // Wrap into a fresh transport-kind error carrying the enriched message
    // and the original Cosmos error as source. Forward the inner error's
    // diagnostics so `outer.diagnostics()` is not silently `None` — callers
    // should not have to walk `source()` to recover the operation's
    // diagnostic context.
    let mut b = crate::error::CosmosError::builder()
        .with_status(*status)
        .with_message(message)
        .with_arc_source(std::sync::Arc::new(error.clone()));
    if let Some(diag) = error.diagnostics() {
        b = b.with_diagnostics(diag);
    }
    b.build()
}
