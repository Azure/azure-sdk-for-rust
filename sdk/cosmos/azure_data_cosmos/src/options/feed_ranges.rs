// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`ReadFeedRangesOptions`] — options for routing-map / feed-range reads.

use azure_data_cosmos_driver::options::OperationOptions;

/// Options for [`ContainerClient::read_feed_ranges()`](crate::clients::ContainerClient::read_feed_ranges)
/// and [`ContainerClient::feed_range_from_partition_key()`](crate::clients::ContainerClient::feed_range_from_partition_key).
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct ReadFeedRangesOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    force_refresh: bool,
}

impl ReadFeedRangesOptions {
    /// When `true`, discards any cached routing map and fetches a fresh copy from the service.
    pub fn with_force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = force_refresh;
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }

    pub(crate) fn force_refresh(&self) -> bool {
        self.force_refresh
    }
}
