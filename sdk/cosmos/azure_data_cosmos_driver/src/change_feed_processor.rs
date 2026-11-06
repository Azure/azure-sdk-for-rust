// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Coordination and consumption of a shared change feed workload.
//!
//! Bootstrap creates unowned range leases. Processing acquires these leases and
//! delivers schema-agnostic batches; consumers acknowledge successful processing
//! explicitly before the driver saves progress.

use crate::options::{OperationOptions, UserAgentSuffix};

mod bootstrap;
mod processing;

pub use bootstrap::bootstrap;
pub(crate) use bootstrap::ChangeFeedProcessorBootstrapCoalescer;
pub use processing::{
    BatchReceipt, ChangeFeedBatch, CheckpointPolicy, Processor, ProcessorOptions,
};

fn operation_options() -> OperationOptions {
    OperationOptions {
        user_agent_suffix: Some(
            UserAgentSuffix::try_from("ChangeFeedProcessor")
                .expect("static processor suffix is valid"),
        ),
        ..Default::default()
    }
}
