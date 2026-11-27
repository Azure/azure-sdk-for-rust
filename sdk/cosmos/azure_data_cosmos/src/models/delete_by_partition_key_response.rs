// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    diagnostics::DiagnosticsContext,
    models::{CosmosResponse, CosmosStatus, ResponseHeaders},
};
use azure_core::fmt::SafeDebug;
use std::sync::Arc;

/// The service response to a request to start deleting items by partition key.
///
/// **Unstable.** Requires the `unstable_delete_by_partition_key` feature.
///
/// Returned by
/// [`ContainerClient::begin_delete_by_partition_key`](crate::clients::ContainerClient::begin_delete_by_partition_key).
/// This response describes initiation only, not completion of physical deletion,
/// and provides no background progress information.
#[derive(SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DeleteByPartitionKeyResponse {
    response: CosmosResponse,
}

impl DeleteByPartitionKeyResponse {
    pub(crate) fn new(response: CosmosResponse) -> Self {
        Self { response }
    }

    /// Returns the status of the initiation request.
    pub fn status(&self) -> CosmosStatus {
        self.response.status()
    }

    /// Returns the headers from the initiation request.
    ///
    /// Request charge information describes this request, not the total cost
    /// of background deletion.
    pub fn headers(&self) -> &ResponseHeaders {
        self.response.cosmos_headers()
    }

    /// Returns the diagnostics for the initiation request.
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.response.diagnostics()
    }
}
