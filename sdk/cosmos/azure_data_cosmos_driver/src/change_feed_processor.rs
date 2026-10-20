// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Coordination and consumption of a shared change feed workload.
//!
//! Bootstrap creates unowned range leases. Processing acquires these leases and
//! delivers schema-agnostic batches; consumers acknowledge successful processing
//! explicitly before the driver saves progress.

mod bootstrap;
mod processing;

pub use bootstrap::bootstrap;
pub use processing::{Batch, BatchReceipt, Processor, ProcessorOptions};

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Change feed mode bound to a logical workload.
///
/// Mode is a compatibility constraint, not part of the workload's identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ChangeFeedProcessorMode {
    /// Observe the latest version of changed items.
    LatestVersion,
    /// Observe all versions and deletes.
    AllVersionsAndDeletes,
}

/// Configuration for initializing a logical change feed workload.
///
/// Initial read positions belong to the reader and are deliberately absent here.
#[derive(Clone, SafeDebug)]
pub struct BootstrapOptions {
    mode: ChangeFeedProcessorMode,
    processor_name: String,
    instance_name: String,
    timeout: Duration,
    verify_leases_on_restart: bool,
    retry_delay: Duration,
}

impl BootstrapOptions {
    /// Creates bootstrap configuration for a mode, workload, and worker.
    ///
    /// Workers sharing the source, lease storage, and `processor_name` cooperate.
    /// `instance_name` identifies the worker, not the workload. Names must not be
    /// blank; validation occurs in [`bootstrap()`].
    pub fn new(
        mode: ChangeFeedProcessorMode,
        processor_name: impl Into<String>,
        instance_name: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            processor_name: processor_name.into(),
            instance_name: instance_name.into(),
            timeout: Duration::from_secs(120),
            verify_leases_on_restart: false,
            retry_delay: Duration::from_secs(1),
        }
    }

    /// Sets the total time budget, including claim contention and retries.
    ///
    /// Defaults to two minutes. A zero or unrepresentable duration is rejected.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Enables checking and recreating missing initial leases on re-entry.
    ///
    /// Disabled by default. Existing leases are never replaced. This does not
    /// repair topology changes or reset checkpoints.
    pub fn with_lease_verification_enabled_on_restart(mut self, enabled: bool) -> Self {
        self.verify_leases_on_restart = enabled;
        self
    }
}
