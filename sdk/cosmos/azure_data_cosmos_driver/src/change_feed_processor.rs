// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Coordination and consumption of a shared change feed workload.
//!
//! Bootstrap creates unowned range leases. Processing acquires these leases and
//! delivers schema-agnostic batches; consumers acknowledge successful processing
//! explicitly before the driver saves progress.

mod admission;
mod bootstrap;
mod processing;

pub(crate) use admission::BatchAdmission;
pub use bootstrap::bootstrap;
pub(crate) use bootstrap::ChangeFeedProcessorBootstrapCoalescer;
pub use processing::{
    BatchReceipt, ChangeFeedBatch, CheckpointPolicy, Processor, ProcessorOptions,
};
