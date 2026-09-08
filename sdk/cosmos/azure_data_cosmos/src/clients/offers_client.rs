// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal helpers for managing throughput offers via the driver.
//!
//! These functions are used by container and database clients to read and
//! replace throughput offers. All operations go through the Cosmos driver.

use std::sync::Arc;

use crate::clients::ClientContext;
use crate::diagnostics::{CosmosOperationContext, DiagnosticsContext};
use crate::{feed::FeedBody, models::CosmosResponse, models::ThroughputProperties, Query};
use azure_data_cosmos_driver::models::{AccountReference, ContainerReference, CosmosOperation};
use azure_data_cosmos_driver::options::OperationOptions;

struct OfferQueryResult {
    offer: Option<ThroughputProperties>,
    diagnostics: Arc<DiagnosticsContext>,
}

async fn query_offer(
    context: &ClientContext,
    account: &AccountReference,
    resource_id: &str,
    operation_options: OperationOptions,
) -> crate::Result<Option<OfferQueryResult>> {
    let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
        .with_parameter("@rid", resource_id)?;
    let body = serde_json::to_vec(&query)?;
    let operation = CosmosOperation::query_offers(account.clone()).with_body(body);

    let Some(driver_response) = context
        .driver
        .execute_operation(operation, operation_options)
        .await
        .map_err(crate::CosmosError::from)?
    else {
        return Ok(None);
    };

    tracing::debug!(
        activity_id = ?driver_response.headers().activity_id,
        request_charge = ?driver_response.headers().request_charge,
        "offer query completed"
    );
    let response = crate::driver_bridge::driver_response_to_cosmos_response(driver_response);
    let diagnostics = response.diagnostics();
    let feed: FeedBody<ThroughputProperties> = response.into_model()?;
    Ok(Some(OfferQueryResult {
        offer: feed.items.into_iter().next(),
        diagnostics,
    }))
}

fn aggregate_diagnostics(
    prior: &[Arc<DiagnosticsContext>],
    current: Arc<DiagnosticsContext>,
) -> Arc<DiagnosticsContext> {
    let mut sources = prior.to_vec();
    sources.push(Arc::clone(&current));
    DiagnosticsContext::aggregate_sub_operations(&sources)
        .map(Arc::new)
        .unwrap_or(current)
}

fn with_prior_diagnostics(
    error: crate::CosmosError,
    prior: &[Arc<DiagnosticsContext>],
) -> crate::CosmosError {
    if prior.is_empty() {
        return error;
    }

    let mut sources = prior.to_vec();
    if let Some(current) = error.diagnostics() {
        sources.push(current);
    }
    match DiagnosticsContext::aggregate_sub_operations(&sources) {
        Some(diagnostics) => error.with_diagnostics(Arc::new(diagnostics)),
        None => error,
    }
}

/// Queries the offer for a given resource ID (RID) via the driver.
///
/// Returns `None` if no offer is configured for the resource. The offer-query
/// operation is dispatched to the client's diagnostics handler chain — on both
/// success and failure — under the supplied `op_context` identity, so throughput
/// reads honor the same once-per-operation contract as singleton operations.
pub(crate) async fn find_offer(
    context: &ClientContext,
    account: &AccountReference,
    resource_id: &str,
    operation_options: OperationOptions,
    op_context: CosmosOperationContext,
) -> crate::Result<Option<ThroughputProperties>> {
    match query_offer(context, account, resource_id, operation_options).await {
        Ok(Some(result)) => {
            context.dispatch_diagnostics(&result.diagnostics, || op_context);
            Ok(result.offer)
        }
        // No offer found for this resource: no operation completed to dispatch.
        Ok(None) => Ok(None),
        Err(err) => {
            context.dispatch_error(&err, || op_context);
            Err(err)
        }
    }
}

pub(crate) async fn find_offer_for_container(
    context: &ClientContext,
    container: &ContainerReference,
    operation_options: OperationOptions,
    op_context: CosmosOperationContext,
) -> crate::Result<Option<ThroughputProperties>> {
    let first = match query_offer(
        context,
        container.account(),
        container.rid(),
        operation_options.clone(),
    )
    .await
    {
        Ok(Some(result)) => result,
        Ok(None) => return Ok(None),
        Err(error) => {
            context.dispatch_error(&error, || op_context);
            return Err(error);
        }
    };
    if first.offer.is_some() || container.is_by_rid() {
        context.dispatch_diagnostics(&first.diagnostics, || op_context);
        return Ok(first.offer);
    }

    let replacement = match context
        .driver
        .refresh_container_if_recreated(container, operation_options.clone())
        .await
    {
        Ok(replacement) => replacement,
        Err(error) => {
            let error = with_prior_diagnostics(error.into(), &[first.diagnostics]);
            context.dispatch_error(&error, || op_context);
            return Err(error);
        }
    };
    let Some(replacement) = replacement else {
        context.dispatch_diagnostics(&first.diagnostics, || op_context);
        return Ok(None);
    };

    match query_offer(
        context,
        replacement.account(),
        replacement.rid(),
        operation_options,
    )
    .await
    {
        Ok(Some(result)) => {
            let diagnostics = aggregate_diagnostics(&[first.diagnostics], result.diagnostics);
            context.dispatch_diagnostics(&diagnostics, || op_context);
            Ok(result.offer)
        }
        Ok(None) => {
            context.dispatch_diagnostics(&first.diagnostics, || op_context);
            Ok(None)
        }
        Err(error) => {
            let error = with_prior_diagnostics(error, &[first.diagnostics]);
            context.dispatch_error(&error, || op_context);
            Err(error)
        }
    }
}

/// Reads a specific offer by its RID via the driver, returning the full response.
///
/// The read is routed through the result-aware completion seam so the offer-read
/// operation reaches the diagnostics handler chain on both success and failure.
pub(crate) async fn read_offer_by_id(
    context: &ClientContext,
    account: &AccountReference,
    offer_id: &str,
    op_context: CosmosOperationContext,
) -> crate::Result<CosmosResponse> {
    let operation = CosmosOperation::read_offer(account.clone(), offer_id.to_owned());
    let driver_result = context
        .driver
        .execute_singleton_operation(operation, OperationOptions::default())
        .await;
    context.complete_result(driver_result, || op_context)
}

/// Replaces the throughput for a resource and returns a [`ThroughputPoller`] to track the operation.
///
/// Reads the current offer, validates the offer RID, applies the new throughput, and
/// executes the replace via the driver. Returns a poller for async completion tracking.
///
/// The offer-query, offer-replace, and every subsequent poll are dispatched to the
/// client's diagnostics handler chain under `op_context`, so an asynchronous
/// throughput replace surfaces each of its wire operations to registered handlers.
pub(crate) async fn begin_replace(
    context: ClientContext,
    account: AccountReference,
    resource_id: &str,
    throughput: ThroughputProperties,
    operation_options: OperationOptions,
    op_context: CosmosOperationContext,
) -> crate::Result<crate::clients::ThroughputPoller> {
    let current_throughput = find_offer(
        &context,
        &account,
        resource_id,
        operation_options.clone(),
        op_context.clone(),
    )
    .await?;
    begin_replace_with_offer(
        context,
        account,
        current_throughput,
        throughput,
        operation_options,
        op_context,
    )
    .await
}

pub(crate) async fn begin_replace_for_container(
    context: ClientContext,
    container: &ContainerReference,
    throughput: ThroughputProperties,
    operation_options: OperationOptions,
    op_context: CosmosOperationContext,
) -> crate::Result<crate::clients::ThroughputPoller> {
    let current_throughput = find_offer_for_container(
        &context,
        container,
        operation_options.clone(),
        op_context.clone(),
    )
    .await?;
    begin_replace_with_offer(
        context,
        container.account().clone(),
        current_throughput,
        throughput,
        operation_options,
        op_context,
    )
    .await
}

async fn begin_replace_with_offer(
    context: ClientContext,
    account: AccountReference,
    current_throughput: Option<ThroughputProperties>,
    throughput: ThroughputProperties,
    operation_options: OperationOptions,
    op_context: CosmosOperationContext,
) -> crate::Result<crate::clients::ThroughputPoller> {
    let mut current_throughput = current_throughput.ok_or_else(|| {
        // No offer exists for the resource — typically the caller
        // pointed at a resource that doesn't support throughput
        // (e.g. a serverless or shared-throughput container).
        crate::DriverCosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE)
            .with_message("no throughput offer found for this resource")
            .build()
    })?;

    if current_throughput.offer_id.is_empty() {
        // Service contract violation: an offer was returned but it has
        // no id. Map to 500 with a dedicated sub-status so callers can
        // distinguish this from a transport-generated 503.
        return Err(crate::DriverCosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERVICE_RETURNED_OFFER_WITHOUT_ID)
            .with_message("throughput offer has an empty id")
            .build()
            .into());
    }

    let offer_id = current_throughput.offer_id.clone();
    current_throughput.offer = throughput.offer;

    let body = serde_json::to_vec(&current_throughput)?;
    let operation =
        CosmosOperation::replace_offer(account.clone(), offer_id.clone()).with_body(body);

    // The Offers API always requires the full response body (the service does not
    // support Prefer: return=minimal for offers), so explicitly enable content response.
    let replace_options = {
        let mut opts = operation_options;
        opts.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);
        opts
    };

    let driver_result = context
        .driver
        .execute_singleton_operation(operation, replace_options)
        .await;
    let response = context.complete_result(driver_result, || op_context.clone())?;

    Ok(crate::clients::ThroughputPoller::new(
        response, context, account, offer_id, op_context,
    ))
}
