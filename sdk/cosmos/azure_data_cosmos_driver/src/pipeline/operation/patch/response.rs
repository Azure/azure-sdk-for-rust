// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Synthesize a [`CosmosResponse`] from a locally-merged body plus driver-issued
//! transport headers.
//!
//! The PATCH handler issues an internal Replace to commit the merged document.
//! The handler holds the post-image locally (it's what it sent), so it can
//! materialize a [`CosmosResponse`] using the *driver-routed* headers from the
//! final Replace and its own locally-merged body bytes. The logical PATCH
//! boundary subsequently removes this payload when the effective
//! `content_response_on_write` setting is disabled.

use std::sync::Arc;

use crate::{
    diagnostics::DiagnosticsContext,
    models::{CosmosResponse, CosmosResponseHeaders, CosmosStatus},
};

/// Combines the local PATCH post-image with the committing Replace response.
///
/// The driver-issued headers, status, and diagnostics remain authoritative even
/// when the service omits the Replace response body.
pub(crate) fn from_local_body_and_driver_headers(
    body: Vec<u8>,
    headers: CosmosResponseHeaders,
    status: CosmosStatus,
    diagnostics: Arc<DiagnosticsContext>,
) -> CosmosResponse {
    CosmosResponse::new(body, headers, status, diagnostics)
}
