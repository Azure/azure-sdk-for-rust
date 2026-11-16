// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request execution pipelines.

pub(crate) mod transport;

/// Whether a pipeline should retry its stage or return a terminal failure.
pub(crate) enum RecoveryAction<T> {
    /// Repeat the pipeline attempt.
    Retry,
    /// End the pipeline with the supplied failure.
    Fail(T),
}
