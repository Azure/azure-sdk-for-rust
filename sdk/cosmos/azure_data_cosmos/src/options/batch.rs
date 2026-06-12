// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-operation options for transactional batch sub-operations, plus the
//! request-level [`BatchOptions`] passed to
//! [`ContainerClient::execute_transactional_batch()`](crate::clients::ContainerClient::execute_transactional_batch()).

use azure_data_cosmos_driver::models::{Precondition, SessionToken};
use azure_data_cosmos_driver::options::OperationOptions;

/// Options for transactional batch operations.
///
/// Used by [`ContainerClient::execute_transactional_batch()`](crate::clients::ContainerClient::execute_transactional_batch()).
/// ETag-based conditional options are specified per-operation within the batch itself.
///
/// General-purpose settings such as custom headers and content response behavior
/// are configured via the [`with_operation_options`](Self::with_operation_options) setter.
/// See [`OperationOptions`] for details.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct BatchOptions {
    /// General-purpose options that apply to this request.
    /// See [`OperationOptions`] for available settings and layered resolution behavior.
    pub operation: OperationOptions,

    /// Session token for session-consistent batch operations.
    pub session_token: Option<SessionToken>,
}

impl BatchOptions {
    /// Sets the session token for this request.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the [`OperationOptions`] for this request.
    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}

/// Options for batch upsert operations.
///
/// Upsert supports both conditional options for optimistic concurrency control.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchUpsertOptions {
    /// Conditional ETag check for optimistic concurrency control.
    pub precondition: Option<Precondition>,
}

impl BatchUpsertOptions {
    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}

/// Options for batch replace operations.
///
/// Replace only supports `if_match` for optimistic concurrency control,
/// since the item must exist to be replaced.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReplaceOptions {
    /// Conditional ETag check for optimistic concurrency control.
    pub precondition: Option<Precondition>,
}

impl BatchReplaceOptions {
    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}

/// Options for batch read operations.
///
/// Read supports both conditional options, commonly used for cache validation.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReadOptions {
    /// Conditional ETag check, commonly used for cache validation.
    pub precondition: Option<Precondition>,
}

impl BatchReadOptions {
    /// Sets the precondition (useful for caching or concurrency control).
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}

/// Options for batch delete operations.
///
/// Delete only supports `if_match` for optimistic concurrency control,
/// since the item must exist to be deleted.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchDeleteOptions {
    /// Conditional ETag check for optimistic concurrency control.
    pub precondition: Option<Precondition>,
}

impl BatchDeleteOptions {
    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}
