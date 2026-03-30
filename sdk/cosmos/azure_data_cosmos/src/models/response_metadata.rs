// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostics metadata for Cosmos DB operations.

use azure_data_cosmos_driver::models::CosmosResponseHeaders;

/// Diagnostics for a Cosmos DB operation.
///
/// Provides operation-level metadata for debugging and performance analysis.
///
/// This is a lightweight diagnostics type populated from HTTP response headers.
/// It will be expanded to include full driver diagnostics (retry tracking,
/// regions contacted, pipeline events) once the SDK pipeline is ported to the
/// driver's transport pipeline.
#[derive(Debug, Clone, Default)]
pub struct CosmosDiagnostics {
    activity_id: Option<String>,
    server_duration_ms: Option<f64>,
}

impl CosmosDiagnostics {
    pub(crate) fn from_headers(headers: &CosmosResponseHeaders) -> Self {
        Self {
            activity_id: headers.activity_id.as_ref().map(|a| a.as_str().to_owned()),
            server_duration_ms: headers.server_duration_ms,
        }
    }

    /// Returns the activity ID for request correlation, if available.
    pub fn activity_id(&self) -> Option<&str> {
        self.activity_id.as_deref()
    }

    /// Returns the server-side request processing duration in milliseconds, if available.
    pub fn server_duration_ms(&self) -> Option<f64> {
        self.server_duration_ms
    }
}
