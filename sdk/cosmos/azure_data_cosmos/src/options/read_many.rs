// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    feed::ReadManyFilter,
    options::{MaxItemCountHint, OperationOptions, SessionToken},
};

/// Options for [`ContainerClient::read_many()`](crate::clients::ContainerClient::read_many).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadManyOptions {
    /// General operation options, including consistency and timeouts.
    pub operation: OperationOptions,
    /// An optional predicate evaluated by the service within the selection.
    pub filter: Option<ReadManyFilter>,
    /// Explicit session token for all requests.
    pub session_token: Option<SessionToken>,
    /// Service page-size hint, not a limit on the total results.
    pub max_item_count: Option<MaxItemCountHint>,
    /// Maximum physical partitions admitted at planning time; defaults to 100.
    pub max_fan_out: Option<u32>,
}

impl ReadManyOptions {
    /// Sets the optional predicate without broadening the selection.
    pub fn with_filter(mut self, filter: ReadManyFilter) -> Self {
        self.filter = Some(filter);
        self
    }
    /// Sets general operation options.
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation = options;
        self
    }
    /// Sets an explicit session token.
    pub fn with_session_token(mut self, token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(token.into());
        self
    }
    /// Sets the per-request page-size hint.
    pub fn with_max_item_count(mut self, hint: MaxItemCountHint) -> Self {
        self.max_item_count = Some(hint);
        self
    }
    /// Sets the physical-partition admission limit. Zero selects the default.
    pub fn with_max_fan_out(mut self, limit: u32) -> Self {
        self.max_fan_out = Some(limit);
        self
    }
}
