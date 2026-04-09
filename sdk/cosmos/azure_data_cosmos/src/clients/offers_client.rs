// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Internal helpers for managing throughput offers via the driver.
//!
//! These functions are used by container and database clients to read and
//! replace throughput offers. All operations go through the Cosmos driver.

use crate::{
    constants,
    feed::FeedBody,
    models::{CosmosResponse, ThroughputProperties},
    Query,
};
use azure_core::http::headers::{HeaderValue, CONTENT_TYPE};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation};
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::CosmosDriver;
use std::collections::HashMap;
use std::sync::Arc;

/// Queries the offer for a given resource ID (RID) via the driver.
///
/// Returns `None` if no offer is configured for the resource.
pub(crate) async fn find_offer(
    driver: &CosmosDriver,
    account: &AccountReference,
    resource_id: &str,
) -> azure_core::Result<Option<ThroughputProperties>> {
    let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
        .with_parameter("@rid", resource_id)?;
    let body = serde_json::to_vec(&query)?;

    let operation = CosmosOperation::query_offers(account.clone()).with_body(body);

    let mut headers = HashMap::new();
    headers.insert(constants::QUERY, HeaderValue::from("True"));
    headers.insert(CONTENT_TYPE, HeaderValue::from("application/query+json"));
    let options = OperationOptions::default().with_custom_headers(headers);

    let driver_response = driver.execute_operation(operation, options).await?;
    tracing::debug!(
        activity_id = ?driver_response.headers().activity_id,
        request_charge = ?driver_response.headers().request_charge,
        "offer query completed"
    );
    let feed: FeedBody<ThroughputProperties> = serde_json::from_slice(driver_response.body())?;
    Ok(feed.items.into_iter().next())
}

/// Reads a specific offer by its RID via the driver, returning the full response.
pub(crate) async fn read_offer_by_id(
    driver: &CosmosDriver,
    account: &AccountReference,
    offer_id: &str,
) -> azure_core::Result<CosmosResponse<ThroughputProperties>> {
    let operation = CosmosOperation::read_offer(account.clone(), offer_id.to_owned());
    let driver_response = driver
        .execute_operation(operation, OperationOptions::default())
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
) -> azure_core::Result<crate::clients::ThroughputPoller> {
    let mut current_throughput = find_offer(&driver, &account, resource_id)
        .await?
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "no throughput offer found for this resource",
            )
        })?;

    if current_throughput.offer_id.is_empty() {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "throughput offer has an empty id",
        ));
    }

    let offer_id = current_throughput.offer_id.clone();
    current_throughput.offer = throughput.offer;

    let body = serde_json::to_vec(&current_throughput)?;
    let operation =
        CosmosOperation::replace_offer(account.clone(), offer_id.clone()).with_body(body);

    // The Offers API always requires the full response body (the service does not
    // support Prefer: return=minimal for offers), so explicitly enable content response.
    let replace_options = {
        let mut opts = OperationOptions::default();
        opts.content_response_on_write =
            Some(azure_data_cosmos_driver::options::ContentResponseOnWrite::Enabled);
        opts
    };

    let driver_response = driver.execute_operation(operation, replace_options).await?;

    let response = crate::driver_bridge::driver_response_to_cosmos_response(driver_response);

    Ok(crate::clients::ThroughputPoller::new(
        response, driver, account, offer_id,
    ))
}
