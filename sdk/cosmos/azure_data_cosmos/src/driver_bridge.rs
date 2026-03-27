// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bridge between driver types and SDK types.
//!
//! This module provides conversion functions for translating between the driver's
//! operation/response types and the SDK's public-facing types. It is the shared
//! foundation for routing SDK operations through the driver.

use azure_core::{
    http::{
        headers::{HeaderName, Headers},
        response::Response,
        RawResponse, StatusCode,
    },
    Bytes,
};
use azure_data_cosmos_driver::{
    models::{
        CosmosResponse as DriverResponse, CosmosResponseHeaders, ETag, Precondition,
        SessionToken as DriverSessionToken,
    },
    options::{ExcludedRegions, OperationOptions, Region},
};

use crate::{models::CosmosResponse, options::ItemOptions};

/// Converts a driver [`DriverResponse`] into the SDK's typed [`CosmosResponse<T>`].
///
/// This reconstructs an `azure_core::Response<T>` from the driver's raw bytes,
/// status code, and headers, then wraps it in the SDK's response type.
pub(crate) fn driver_response_to_cosmos_response<T>(
    driver_response: DriverResponse,
) -> CosmosResponse<T> {
    let status_code: StatusCode = driver_response.status().status_code();
    let headers = driver_response_headers_to_headers(driver_response.headers());
    let body = driver_response.into_body();

    let raw_response = RawResponse::from_bytes(status_code, headers, Bytes::from(body));
    let typed_response: Response<T> = raw_response.into();

    CosmosResponse::new(typed_response, None)
}

/// Converts driver [`CosmosResponseHeaders`] into raw [`Headers`] for the SDK response.
///
/// Only headers that were parsed by the driver are included. Any "extra" headers
/// from the server that the driver did not capture are lost.
fn driver_response_headers_to_headers(cosmos_headers: &CosmosResponseHeaders) -> Headers {
    let mut headers = Headers::new();

    if let Some(activity_id) = &cosmos_headers.activity_id {
        headers.insert(
            HeaderName::from_static("x-ms-activity-id"),
            activity_id.as_str().to_owned(),
        );
    }
    if let Some(charge) = &cosmos_headers.request_charge {
        headers.insert(
            HeaderName::from_static("x-ms-request-charge"),
            charge.value().to_string(),
        );
    }
    if let Some(session_token) = &cosmos_headers.session_token {
        headers.insert(
            HeaderName::from_static("x-ms-session-token"),
            session_token.as_str().to_owned(),
        );
    }
    if let Some(etag) = &cosmos_headers.etag {
        headers.insert(HeaderName::from_static("etag"), etag.as_str().to_owned());
    }
    if let Some(continuation) = &cosmos_headers.continuation {
        headers.insert(
            HeaderName::from_static("x-ms-continuation"),
            continuation.clone(),
        );
    }
    if let Some(item_count) = cosmos_headers.item_count {
        headers.insert(
            HeaderName::from_static("x-ms-item-count"),
            item_count.to_string(),
        );
    }
    if let Some(substatus) = &cosmos_headers.substatus {
        headers.insert(
            HeaderName::from_static("x-ms-substatus"),
            substatus.value().to_string(),
        );
    }

    headers
}

/// Translates SDK [`ItemOptions`] into driver [`OperationOptions`].
pub(crate) fn item_options_to_operation_options(options: &ItemOptions) -> OperationOptions {
    let mut driver_options = OperationOptions::new();

    if let Some(session_token) = options.session_token() {
        driver_options =
            driver_options.with_session_token(DriverSessionToken::new(session_token.to_string()));
    }

    if let Some(etag) = options.if_match_etag() {
        driver_options =
            driver_options.with_etag_condition(Precondition::if_match(ETag::new(etag.to_string())));
    }

    if !options.custom_headers().is_empty() {
        driver_options = driver_options.with_custom_headers(options.custom_headers().clone());
    }

    if let Some(regions) = &options.excluded_regions {
        let driver_regions =
            ExcludedRegions(regions.iter().map(|r| Region::new(r.to_string())).collect());
        driver_options = driver_options.with_excluded_regions(driver_regions);
    }

    driver_options
}
