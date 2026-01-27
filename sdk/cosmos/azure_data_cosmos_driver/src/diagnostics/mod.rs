// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Diagnostic and telemetry types for Cosmos DB operations.
//!
//! This module provides rich diagnostic information about Cosmos DB operations,
//! similar to [CosmosDiagnosticsContext](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/CosmosDiagnosticsContext.java)
//! in the Java SDK.
//!
//! Diagnostics are **operational metadata** tracked by the SDK, not service resources.

use std::time::Duration;

/// Diagnostic context for a Cosmos DB operation.
///
/// Contains detailed information about request execution including RU consumption,
/// regions contacted, retry attempts, and timing information.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticsContext {
    /// Total request charge (RU/s) consumed by the operation.
    pub request_charge: f64,

    /// Regions that were contacted during the operation (for multi-region accounts).
    pub regions_contacted: Vec<RegionContact>,

    /// Retry attempts made during the operation.
    pub retry_count: u32,

    /// Total elapsed time for the operation.
    pub total_duration: Duration,

    /// Activity ID for correlating requests with service-side logs.
    pub activity_id: Option<String>,
}

/// Information about a region contacted during an operation.
#[derive(Clone, Debug)]
pub struct RegionContact {
    /// Azure region name (e.g., "East US", "West Europe").
    pub region_name: String,

    /// Endpoint URI for the region.
    pub endpoint: String,

    /// Whether this was the preferred (primary) region.
    pub is_preferred: bool,

    /// Duration spent communicating with this region.
    pub duration: Duration,
}

impl DiagnosticsContext {
    /// Creates a new empty diagnostics context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a region contact to the diagnostics.
    pub fn add_region_contact(&mut self, contact: RegionContact) {
        self.regions_contacted.push(contact);
    }

    /// Records a retry attempt.
    pub fn record_retry(&mut self) {
        self.retry_count += 1;
    }
}
