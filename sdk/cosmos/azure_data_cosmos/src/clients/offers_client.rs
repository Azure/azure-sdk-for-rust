// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal helpers for managing throughput offers via the driver.
//!
//! These functions are used by container and database clients to read and
//! replace throughput offers. All operations go through the Cosmos driver.

use crate::{
    feed::FeedBody,
    models::{CosmosResponse, ThroughputProperties},
    Query,
};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation};
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::CosmosDriver;
use std::sync::Arc;

/// Queries the offer for a given resource ID (RID) via the driver.
///
/// Returns `None` if no offer is configured for the resource.
pub(crate) async fn find_offer(
    driver: &CosmosDriver,
    account: &AccountReference,
    resource_id: &str,
    operation_options: OperationOptions,
) -> crate::Result<Option<ThroughputProperties>> {
    let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
        .with_parameter("@rid", resource_id)?;
    let body = serde_json::to_vec(&query)?;

    let operation = CosmosOperation::query_offers(account.clone()).with_body(body);

    let driver_response = driver
        .execute_operation(operation, operation_options)
        .await?;
    let Some(driver_response) = driver_response else {
        // No offer found for this resource
        return Ok(None);
    };
    tracing::debug!(
        activity_id = ?driver_response.headers().activity_id,
        request_charge = ?driver_response.headers().request_charge,
        "offer query completed"
    );
    let feed: FeedBody<ThroughputProperties> = driver_response.into_body().into_single()?;
    Ok(feed.items.into_iter().next())
}

/// Reads a specific offer by its RID via the driver, returning the full response.
pub(crate) async fn read_offer_by_id(
    driver: &CosmosDriver,
    account: &AccountReference,
    offer_id: &str,
) -> crate::Result<CosmosResponse> {
    let operation = CosmosOperation::read_offer(account.clone(), offer_id.to_owned());
    let driver_response = driver
        .execute_singleton_operation(operation, OperationOptions::default())
        .await?;
    Ok(crate::driver_bridge::driver_response_to_cosmos_response(
        driver_response,
    ))
}

/// Replaces the throughput for a resource and returns a [`ThroughputPoller`] to track the operation.
///
/// Reads the current offer, validates the offer RID, applies the new throughput, and
/// executes the replace via the driver. Returns a poller for async completion tracking.
pub(crate) async fn begin_replace(
    driver: Arc<CosmosDriver>,
    account: AccountReference,
    resource_id: &str,
    throughput: ThroughputProperties,
    operation_options: OperationOptions,
) -> crate::Result<crate::clients::ThroughputPoller> {
    let mut current_throughput =
        find_offer(&driver, &account, resource_id, operation_options.clone())
            .await?
            .ok_or_else(|| {
                // No offer exists for the resource — typically the caller
                // pointed at a resource that doesn't support throughput
                // (e.g. a serverless or shared-throughput container).
                crate::DriverCosmosError::builder()
                    .with_status(crate::CosmosStatus::CLIENT_NO_THROUGHPUT_OFFER_FOR_RESOURCE)
                    .with_message("no throughput offer found for this resource")
                    .build()
            })?;

    if current_throughput.offer_id.is_empty() {
        // Service contract violation: an offer was returned but it has
        // no id. Map to 500 with a dedicated sub-status so callers can
        // distinguish this from a transport-generated 503.
        return Err(crate::DriverCosmosError::builder()
            .with_status(crate::CosmosStatus::SERVICE_RETURNED_OFFER_WITHOUT_ID)
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

    let driver_response = driver
        .execute_singleton_operation(operation, replace_options)
        .await?;

    let response = crate::driver_bridge::driver_response_to_cosmos_response(driver_response);

    Ok(crate::clients::ThroughputPoller::new(
        response, driver, account, offer_id,
    ))
}
