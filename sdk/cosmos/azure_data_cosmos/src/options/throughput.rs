// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`ThroughputOptions`] — options for throughput / offer operations.

use azure_data_cosmos_driver::options::OperationOptions;

/// Options to be passed to operations related to Throughput offers.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ThroughputOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,
}

impl ThroughputOptions {
    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}
