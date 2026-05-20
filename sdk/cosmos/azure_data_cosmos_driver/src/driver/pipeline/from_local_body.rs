// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Synthesize a [`CosmosResponse`] from a locally-merged body plus driver-issued
//! transport headers.
//!
//! The PATCH handler issues an internal Replace to commit the merged document.
//! Cosmos PATCH responses normally include the post-image of the item in the
//! body, but callers may have disabled `content_response_on_write` for the
//! Replace, or the Replace itself may have stripped the body. Either way the
//! handler holds the authoritative post-image locally (it's what it sent), so
//! it materializes a [`CosmosResponse`] using the *driver-routed* headers from
//! the final Replace and its own locally-merged body bytes.

use crate::diagnostics::DiagnosticsContext;
use crate::models::{CosmosResponse, CosmosResponseHeaders, CosmosStatus};
use std::sync::Arc;

/// Builds a [`CosmosResponse`] from a locally-computed body and the headers
/// (plus status + diagnostics) from a driver-issued Replace response.
///
/// The driver crate exposes [`CosmosResponse::new`] as `pub(crate)`, so this
/// helper lives inside the crate's pipeline module and is the single place
/// the patch handler is allowed to construct a response with an
/// out-of-band body.
pub(crate) fn from_local_body_and_driver_headers(
    body: Vec<u8>,
    headers: CosmosResponseHeaders,
    status: CosmosStatus,
    diagnostics: Arc<DiagnosticsContext>,
) -> CosmosResponse {
    CosmosResponse::new(body, headers, status, diagnostics)
}
