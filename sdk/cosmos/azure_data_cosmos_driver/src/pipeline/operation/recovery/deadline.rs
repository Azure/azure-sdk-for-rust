// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Recovery decisions for deadline outcomes.

use crate::{
    diagnostics::RequestSentStatus,
    models::CosmosStatus,
    pipeline::operation::{recovery::state::OperationAction, routing::LocationEffects},
};

/// Handles a deadline-exceeded transport outcome — the end-to-end operation
/// timeout fired before a response could be returned.
///
/// No retry is possible (the deadline applies to the whole operation, so
/// retrying would immediately re-trigger). The synthesized error carries
/// `RequestTimeout` + `CLIENT_OPERATION_TIMEOUT` so callers can distinguish
/// a client-side end-to-end timeout from a service 408.
pub(crate) fn recover(request_sent: RequestSentStatus) -> (OperationAction, LocationEffects) {
    let message: &'static str = if request_sent.definitely_not_sent() {
        "end-to-end operation timeout exceeded before request was sent"
    } else {
        "end-to-end operation timeout exceeded"
    };

    // Build the typed end-to-end timeout error (carries
    // `RequestTimeout` + `CLIENT_OPERATION_TIMEOUT` on `error.status()`)
    // and abort. The operation pipeline propagates
    // `crate::error::CosmosError` directly via `OperationAction::Abort.error`.
    let cosmos_err = crate::error::CosmosError::builder()
        .with_status(CosmosStatus::from_parts(
            azure_core::http::StatusCode::RequestTimeout,
            Some(crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT),
        ))
        .with_message(message)
        .build();

    (
        OperationAction::Abort { error: cosmos_err },
        LocationEffects::default(),
    )
}
