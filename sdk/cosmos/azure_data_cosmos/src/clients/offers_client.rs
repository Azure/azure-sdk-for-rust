// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal helpers for managing throughput offers via the driver.
//!
//! These functions are used by container and database clients to read and
//! replace throughput offers. All operations go through the Cosmos driver.

use crate::clients::ClientContext;
use crate::diagnostics::CosmosOperationContext;
use crate::{feed::FeedBody, models::CosmosResponse, models::ThroughputProperties, Query};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation};
use azure_data_cosmos_driver::options::OperationOptions;

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
    let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
        .with_parameter("@rid", resource_id)?;
    let body = serde_json::to_vec(&query)?;

    let operation = CosmosOperation::query_offers(account.clone()).with_body(body);

    match context
        .driver
        .execute_operation(operation, operation_options)
        .await
    {
        Ok(Some(driver_response)) => {
            tracing::debug!(
                activity_id = ?driver_response.headers().activity_id,
                request_charge = ?driver_response.headers().request_charge,
                "offer query completed"
            );
            let response = context.complete_operation(driver_response, || op_context);
            let feed: FeedBody<ThroughputProperties> = response.into_model()?;
            Ok(feed.items.into_iter().next())
        }
        // No offer found for this resource: no operation completed to dispatch.
        Ok(None) => Ok(None),
        Err(err) => {
            let err = crate::CosmosError::from(err);
            context.dispatch_error(&err, || op_context);
            Err(err)
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
    let mut current_throughput = find_offer(
        &context,
        &account,
        resource_id,
        operation_options.clone(),
        op_context.clone(),
    )
    .await?
    .ok_or_else(|| {
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
