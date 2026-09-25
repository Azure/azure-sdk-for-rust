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
/// Supports [`Precondition::IfMatch`] and [`Precondition::IfNoneMatch`] for
/// conditional upserts.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchUpsertOptions {
    /// Optional `If-Match` or `If-None-Match` ETag condition.
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
/// Only [`Precondition::IfMatch`] is applied; other preconditions are ignored.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReplaceOptions {
    /// Optional `If-Match` ETag condition; other preconditions are ignored.
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
/// Supports [`Precondition::IfMatch`] and [`Precondition::IfNoneMatch`]
/// for conditional reads.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchReadOptions {
    /// Optional `If-Match` or `If-None-Match` ETag condition.
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
/// Only [`Precondition::IfMatch`] is applied; other preconditions are ignored.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BatchDeleteOptions {
    /// Optional `If-Match` ETag condition; other preconditions are ignored.
    pub precondition: Option<Precondition>,
}

impl BatchDeleteOptions {
    /// Sets the precondition for optimistic concurrency control.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}
