// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Preview Distributed Transaction builders.
//!
//! **Preview / work in progress.** These APIs are gated behind the
//! disabled-by-default `preview_dtx` feature, depend on a service-side feature
//! that is not yet generally available, and may change or be removed without
//! notice. They are **not supported for production use**.

use std::{borrow::Cow, sync::Arc};

use azure_core::fmt::SafeDebug;
use azure_core::Bytes;
use azure_data_cosmos_driver::models as driver_models;
use serde::{de::DeserializeOwned, Serialize};

use crate::clients::{ClientContext, ContainerClient};
use crate::diagnostics::DiagnosticsContext;
use crate::models::{PartitionKey, PatchInstructions, ResponseHeaders};
use crate::options::{Precondition, SessionToken};

/// Options for a single operation inside a distributed transaction.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DistributedTransactionOperationOptions {
    /// Per-operation session token.
    pub session_token: Option<SessionToken>,
    /// ETag precondition.
    pub precondition: Option<Precondition>,
}

impl DistributedTransactionOperationOptions {
    /// Sets the per-operation session token.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the ETag precondition.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }
}

/// Options for a patch operation inside a distributed transaction.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DistributedTransactionPatchOperationOptions {
    /// Per-operation session token.
    pub session_token: Option<SessionToken>,
    /// ETag precondition.
    pub precondition: Option<Precondition>,
    /// SQL predicate evaluated before applying the patch.
    pub filter_predicate: Option<Cow<'static, str>>,
}

impl DistributedTransactionPatchOperationOptions {
    /// Sets the per-operation session token.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the ETag precondition.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets a SQL predicate evaluated before applying the patch.
    pub fn with_filter_predicate(mut self, predicate: impl Into<Cow<'static, str>>) -> Self {
        self.filter_predicate = Some(predicate.into());
        self
    }
}

/// Preview distributed write transaction builder.
#[derive(Clone, SafeDebug)]
#[safe(true)]
pub struct DistributedWriteTransaction {
    operations: Vec<driver_models::DistributedTransactionOperation>,
}

impl DistributedWriteTransaction {
    /// Creates an empty distributed write transaction.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Consumes this transaction and returns its operations.
    pub(crate) fn into_operations(self) -> Vec<driver_models::DistributedTransactionOperation> {
        self.operations
    }

    /// Adds a create item operation.
    pub fn create_item<T: Serialize>(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        item: T,
        options: Option<DistributedTransactionOperationOptions>,
    ) -> crate::Result<Self> {
        let body = serde_json::to_vec(&item)?;
        self.operations.push(operation_with_options(
            driver_models::DistributedTransactionOperationKind::Create,
            container,
            partition_key,
            item_id,
            Some(Bytes::from(body)),
            options,
        ));
        Ok(self)
    }

    /// Adds a replace item operation.
    pub fn replace_item<T: Serialize>(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        item: T,
        options: Option<DistributedTransactionOperationOptions>,
    ) -> crate::Result<Self> {
        let body = serde_json::to_vec(&item)?;
        self.operations.push(operation_with_options(
            driver_models::DistributedTransactionOperationKind::Replace,
            container,
            partition_key,
            item_id,
            Some(Bytes::from(body)),
            options,
        ));
        Ok(self)
    }

    /// Adds an upsert item operation.
    pub fn upsert_item<T: Serialize>(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        item: T,
        options: Option<DistributedTransactionOperationOptions>,
    ) -> crate::Result<Self> {
        let body = serde_json::to_vec(&item)?;
        self.operations.push(operation_with_options(
            driver_models::DistributedTransactionOperationKind::Upsert,
            container,
            partition_key,
            item_id,
            Some(Bytes::from(body)),
            options,
        ));
        Ok(self)
    }

    /// Adds a delete item operation.
    pub fn delete_item(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        options: Option<DistributedTransactionOperationOptions>,
    ) -> Self {
        self.operations.push(operation_with_options(
            driver_models::DistributedTransactionOperationKind::Delete,
            container,
            partition_key,
            item_id,
            None,
            options,
        ));
        self
    }

    /// Adds a patch item operation.
    pub fn patch_item(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        patch: PatchInstructions,
        options: Option<DistributedTransactionPatchOperationOptions>,
    ) -> crate::Result<Self> {
        let body = serde_json::to_vec(&patch)?;
        self.operations.push(patch_operation_with_options(
            container,
            partition_key,
            item_id,
            Bytes::from(body),
            options,
        )?);
        Ok(self)
    }
}

impl Default for DistributedWriteTransaction {
    fn default() -> Self {
        Self::new()
    }
}

/// Preview distributed read transaction builder.
#[derive(Clone, SafeDebug)]
#[safe(true)]
pub struct DistributedReadTransaction {
    operations: Vec<driver_models::DistributedTransactionOperation>,
}

impl DistributedReadTransaction {
    /// Creates an empty distributed read transaction.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Consumes this transaction and returns its operations.
    pub(crate) fn into_operations(self) -> Vec<driver_models::DistributedTransactionOperation> {
        self.operations
    }

    /// Adds a read item operation.
    pub fn read_item(
        mut self,
        container: &ContainerClient,
        partition_key: impl Into<PartitionKey>,
        item_id: impl Into<std::borrow::Cow<'static, str>>,
        options: Option<DistributedTransactionOperationOptions>,
    ) -> Self {
        self.operations.push(operation_with_options(
            driver_models::DistributedTransactionOperationKind::Read,
            container,
            partition_key,
            item_id,
            None,
            options,
        ));
        self
    }
}

impl Default for DistributedReadTransaction {
    fn default() -> Self {
        Self::new()
    }
}

fn operation_with_options(
    kind: driver_models::DistributedTransactionOperationKind,
    container: &ContainerClient,
    partition_key: impl Into<PartitionKey>,
    item_id: impl Into<std::borrow::Cow<'static, str>>,
    body: Option<Bytes>,
    options: Option<DistributedTransactionOperationOptions>,
) -> driver_models::DistributedTransactionOperation {
    let mut operation = driver_models::DistributedTransactionOperation::new(
        kind,
        driver_models::DistributedTransactionTarget::new(
            container.container_reference().clone(),
            partition_key,
            item_id,
        ),
    );
    if let Some(body) = body {
        operation = operation.with_resource_body(body);
    }
    if let Some(options) = options {
        if let Some(session_token) = options.session_token {
            operation = operation.with_session_token(session_token);
        }
        if let Some(precondition) = options.precondition {
            operation = operation.with_precondition(precondition);
        }
    }
    operation
}

fn patch_operation_with_options(
    container: &ContainerClient,
    partition_key: impl Into<PartitionKey>,
    item_id: impl Into<std::borrow::Cow<'static, str>>,
    body: Bytes,
    options: Option<DistributedTransactionPatchOperationOptions>,
) -> crate::Result<driver_models::DistributedTransactionOperation> {
    let mut operation = driver_models::DistributedTransactionOperation::new(
        driver_models::DistributedTransactionOperationKind::Patch,
        driver_models::DistributedTransactionTarget::new(
            container.container_reference().clone(),
            partition_key,
            item_id,
        ),
    )
    .with_resource_body(body);

    if let Some(options) = options {
        if let Some(session_token) = options.session_token {
            operation = operation.with_session_token(session_token);
        }
        if let Some(precondition) = options.precondition {
            operation = operation.with_precondition(precondition);
        }
        if let Some(predicate) = options.filter_predicate {
            operation = operation.with_patch_filter_predicate(predicate);
        }
    }

    Ok(operation)
}

pub(crate) async fn commit_distributed_write(
    context: &ClientContext,
    transaction: DistributedWriteTransaction,
) -> crate::Result<DistributedTransactionResponse> {
    let operations = transaction.into_operations();
    validate_transaction_account(context, &operations)?;
    let request = driver_models::DistributedTransactionRequest::new(
        driver_models::DistributedTransactionType::Write,
        operations,
    );
    let response = context
        .driver
        .execute_distributed_transaction(request, Default::default())
        .await?;
    Ok(DistributedTransactionResponse::from_driver(response))
}

pub(crate) async fn execute_distributed_read(
    context: &ClientContext,
    transaction: DistributedReadTransaction,
) -> crate::Result<DistributedTransactionResponse> {
    let operations = transaction.into_operations();
    validate_transaction_account(context, &operations)?;
    let request = driver_models::DistributedTransactionRequest::new(
        driver_models::DistributedTransactionType::Read,
        operations,
    );
    let response = context
        .driver
        .execute_distributed_transaction(request, Default::default())
        .await?;
    Ok(DistributedTransactionResponse::from_driver(response))
}

fn validate_transaction_account(
    context: &ClientContext,
    operations: &[driver_models::DistributedTransactionOperation],
) -> crate::Result<()> {
    if operations
        .iter()
        .all(|operation| operation.target.container.account() == context.driver.account())
    {
        Ok(())
    } else {
        Err(crate::DriverCosmosError::builder()
            .with_status(crate::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message(
                "distributed transaction operations must target containers from the same Cosmos account as the committing client",
            )
            .build()
            .into())
    }
}

/// Response from a distributed transaction.
#[derive(Clone, SafeDebug)]
#[safe(true)]
pub struct DistributedTransactionResponse {
    inner: driver_models::DistributedTransactionResponse,
    headers: ResponseHeaders,
}

impl DistributedTransactionResponse {
    fn from_driver(inner: driver_models::DistributedTransactionResponse) -> Self {
        let headers = inner.headers.clone().into();
        Self { inner, headers }
    }

    /// Returns the overall transaction status.
    ///
    /// Unlike a transactional batch — which surfaces a raw `207 MultiStatus` and
    /// leaves per-operation inspection to the caller — a distributed transaction
    /// **promotes** a `207` to the status of the first failing operation in
    /// request order (excluding `424 FailedDependency`). A fully successful
    /// transaction reports its success status directly (for example `200`/`201`,
    /// or `304 NotModified` for an all-unchanged read snapshot). Use
    /// [`operation_result`](Self::operation_result) to inspect each operation's
    /// individual status.
    pub fn status(&self) -> crate::CosmosStatus {
        let status = crate::CosmosStatus::new(self.inner.status_code);
        match self.inner.sub_status_code {
            Some(sub_status) => status.with_sub_status(sub_status.value()),
            None => status,
        }
    }

    /// Returns true when the overall status is successful.
    pub fn is_success_status_code(&self) -> bool {
        self.inner.is_success_status_code()
    }

    /// Returns true when the overall transaction completed successfully.
    ///
    /// For read transactions this also treats `304 NotModified` as completed,
    /// even though `304` is not an HTTP 2xx status.
    pub fn is_completed_status_code(&self) -> bool {
        self.inner.is_completed_status_code()
    }

    /// Returns the number of operation results.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true when no operation results are present.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the operation result at `index`.
    pub fn operation_result(
        &self,
        index: usize,
    ) -> Option<DistributedTransactionOperationResult<'_>> {
        self.inner
            .operation_results
            .get(index)
            .map(|inner| DistributedTransactionOperationResult { inner })
    }

    /// Returns the response headers.
    pub fn headers(&self) -> &ResponseHeaders {
        &self.headers
    }

    /// Returns the coordinator diagnostic string, when present.
    pub fn diagnostic_string(&self) -> Option<&str> {
        self.inner.diagnostic_string.as_deref()
    }

    /// Returns the idempotency token for the transaction as a string.
    pub fn idempotency_token(&self) -> String {
        self.inner.idempotency_token.to_string()
    }

    /// Returns whether the coordinator marked this response as retryable.
    pub fn is_retriable(&self) -> bool {
        self.inner.is_retriable
    }

    /// Returns a driver-synthesized error message for malformed coordinator responses.
    pub fn error_message(&self) -> Option<&str> {
        self.inner.error_message.as_deref()
    }

    /// Returns diagnostics captured while executing the transaction.
    pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>> {
        self.inner.diagnostics.clone()
    }

    /// Returns the activity ID from the response headers, when present.
    pub fn activity_id(&self) -> Option<&str> {
        self.inner.activity_id.as_ref().map(|id| id.as_str())
    }

    /// Returns the total request charge from the response headers, when present.
    pub fn request_charge(&self) -> Option<f64> {
        self.inner.request_charge.map(|charge| charge.value())
    }

    /// Returns the retry-after hint in milliseconds, when present.
    pub fn retry_after_ms(&self) -> Option<u64> {
        self.inner.retry_after_ms
    }
}

/// Result for one operation in a distributed transaction response.
#[derive(Clone, Copy, SafeDebug)]
#[safe(true)]
pub struct DistributedTransactionOperationResult<'a> {
    inner: &'a driver_models::DistributedTransactionOperationResult,
}

impl DistributedTransactionOperationResult<'_> {
    /// Returns the operation index.
    pub fn index(&self) -> usize {
        self.inner.index
    }

    /// Returns the operation status.
    pub fn status_code(&self) -> azure_core::http::StatusCode {
        self.inner.status_code
    }

    /// Returns the operation sub-status, when present.
    pub fn sub_status_code(&self) -> Option<crate::SubStatusCode> {
        self.inner.sub_status_code
    }

    /// Returns true when the operation status is successful.
    pub fn is_success_status_code(&self) -> bool {
        self.inner.is_success_status_code()
    }

    /// Returns true when the operation is a completed DTX success outcome.
    ///
    /// For read transactions this also treats `304 NotModified` as completed,
    /// even though `304` is not an HTTP 2xx status.
    pub fn is_completed_status_code(&self) -> bool {
        self.inner.is_completed_status_code()
    }

    /// Returns the ETag, when present.
    pub fn etag(&self) -> Option<&azure_core::http::Etag> {
        self.inner.etag.as_ref()
    }

    /// Returns the per-operation session token, when present.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.inner.session_token.as_ref()
    }

    /// Returns the partition key range id, when present.
    pub fn partition_key_range_id(&self) -> Option<&str> {
        self.inner.partition_key_range_id.as_deref()
    }

    /// Returns the request charge for this operation, when present.
    pub fn request_charge(&self) -> Option<f64> {
        self.inner.request_charge.map(|charge| charge.value())
    }

    /// Deserializes the operation resource body, when present.
    pub fn resource<T: DeserializeOwned>(&self) -> crate::Result<Option<T>> {
        match &self.inner.resource_body {
            driver_models::DistributedTransactionResultBody::None => Ok(None),
            driver_models::DistributedTransactionResultBody::Bytes(bytes) => {
                serde_json::from_slice(bytes).map(Some).map_err(Into::into)
            }
            _ => Ok(None),
        }
    }
}
