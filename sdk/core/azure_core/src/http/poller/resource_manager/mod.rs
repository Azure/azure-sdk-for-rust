// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Poller helpers for ARM (resource manager) long-running operations.

use super::{
    PollerContinuation, PollerOptions, PollerResult, PollerState, PollerStatus, StatusMonitor,
};
use crate::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, Headers, RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS},
        Method, Pipeline, Poller, RawResponse, Request, Response, StatusCode, Url,
    },
    json,
};
use serde::de::DeserializeOwned;

pub(super) const AZURE_ASYNC_OPERATION: HeaderName =
    HeaderName::from_static("azure-asyncoperation");
const OPERATION_LOCATION: HeaderName = HeaderName::from_static("operation-location");
pub(super) const LOCATION: HeaderName = HeaderName::from_static("location");

/// Creates an ARM heuristic poller that supports the following polling modes, applied in
/// priority order:
///
/// 1. `azure-asyncoperation` header: Polls the operation URL from the
///    `azure-asyncoperation` response header, then fetches the final resource from the `location`
///    header (if present) or the original resource URL.
/// 2. `operation-location` header: Polls the operation URL from the
///    `operation-location` response header, then fetches the final resource from the `location`
///    header (if present) or the original resource URL.
/// 3. `location` header: Polls the URL from the `location` response header,
///    which doubles as the final resource URL once polling completes.
/// 4. Falls back to polling the original resource URL, reading operation status from
///    the response body's `status` or `properties.provisioningState` field.
pub fn new_poller<'a, M>(
    pipeline: Pipeline,
    initial_request: Request,
    options: Option<PollerOptions<'a>>,
) -> Poller<M>
where
    M: StatusMonitor + DeserializeOwned + Send + 'static,
    M::Output: DeserializeOwned + Send + 'static,
    M::Format: Send + 'static,
{
    let options = options.map(PollerOptions::into_owned);

    Poller::new(
        move |poller_state, poller_options| {
            let pipeline = pipeline.clone();
            let resource_url = initial_request.url().clone();
            let mut request = match &poller_state {
                PollerState::Initial => initial_request.clone(),
                PollerState::More(PollerContinuation::Links { next_link, .. }) => {
                    let mut request = Request::new(next_link.clone(), Method::Get);
                    request.insert_header("accept", "application/json");
                    request
                }
            };
            let context = poller_options.context.clone();
            Box::pin(async move {
                let response = pipeline.send(&context, &mut request, None).await?;
                let (status_code, headers, body) = response.deconstruct();
                let retry_after = crate::http::poller::get_retry_after(
                    &headers,
                    &[RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS, RETRY_AFTER],
                    &poller_options,
                );
                let monitor: Option<M> = if body.is_empty() {
                    None
                } else {
                    Some(json::from_json(&body)?)
                };
                let response: Response<M> =
                    RawResponse::from_bytes(status_code, headers.clone(), body).into();
                let mut status = monitor
                    .as_ref()
                    .map(|monitor| monitor.status())
                    .unwrap_or_else(|| PollerStatus::UnknownValue("unknown".to_owned()));
                if matches!(status, PollerStatus::UnknownValue(_)) {
                    status = match status_code {
                        StatusCode::Accepted | StatusCode::Created => PollerStatus::InProgress,
                        StatusCode::Ok | StatusCode::NoContent => PollerStatus::Succeeded,
                        _ => status,
                    };
                }

                let previous_final_link = match poller_state {
                    PollerState::Initial => None,
                    PollerState::More(PollerContinuation::Links { final_link, .. }) => final_link,
                };

                Ok(match status {
                    PollerStatus::InProgress => {
                        let (next_link, final_link) = get_poll_links(
                            &headers,
                            request.url(),
                            &resource_url,
                            previous_final_link.as_ref(),
                        )?;
                        PollerResult::InProgress {
                            response,
                            retry_after,
                            continuation: PollerContinuation::Links {
                                next_link,
                                final_link,
                            },
                        }
                    }
                    PollerStatus::Succeeded => {
                        let final_link = get_location(&headers, request.url())?
                            .or(previous_final_link)
                            .or_else(|| Some(resource_url.clone()));
                        PollerResult::Succeeded {
                            response,
                            target: Box::new(move || {
                                let pipeline = pipeline.clone();
                                let context = context.clone();
                                Box::pin(async move {
                                    let Some(final_link) = final_link else {
                                        return Err(crate::Error::new(
                                            ErrorKind::DataConversion,
                                            "missing final link for ARM LRO",
                                        ));
                                    };
                                    let mut request = Request::new(final_link, Method::Get);
                                    request.insert_header("accept", "application/json");
                                    let response =
                                        pipeline.send(&context, &mut request, None).await?;
                                    let (status, headers, body) = response.deconstruct();
                                    Ok(RawResponse::from_bytes(status, headers, body).into())
                                })
                            }),
                        }
                    }
                    _ => PollerResult::Done { response },
                })
            })
        },
        options,
    )
}

fn get_poll_links(
    headers: &Headers,
    request_url: &Url,
    resource_url: &Url,
    previous_final_link: Option<&Url>,
) -> crate::Result<(Url, Option<Url>)> {
    if let Some(next_link) = get_header_url(headers, &AZURE_ASYNC_OPERATION, request_url)? {
        let final_link = get_location(headers, request_url)?
            .or_else(|| previous_final_link.cloned())
            .or_else(|| Some(resource_url.clone()));
        return Ok((next_link, final_link));
    }

    if let Some(next_link) = get_header_url(headers, &OPERATION_LOCATION, request_url)? {
        let final_link = get_location(headers, request_url)?
            .or_else(|| previous_final_link.cloned())
            .or_else(|| Some(resource_url.clone()));
        return Ok((next_link, final_link));
    }

    if let Some(next_link) = get_location(headers, request_url)? {
        let final_link = Some(next_link.clone());
        return Ok((next_link, final_link));
    }

    Ok((resource_url.clone(), Some(resource_url.clone())))
}

fn get_location(headers: &Headers, request_url: &Url) -> crate::Result<Option<Url>> {
    get_header_url(headers, &LOCATION, request_url)
}

fn get_header_url(
    headers: &Headers,
    header_name: &HeaderName,
    request_url: &Url,
) -> crate::Result<Option<Url>> {
    let Some(value) = headers.get_optional_str(header_name) else {
        return Ok(None);
    };

    if let Ok(url) = Url::parse(value) {
        return Ok(Some(url));
    }

    request_url.join(value).map(Some).map_err(|error| {
        crate::Error::with_error(
            ErrorKind::DataConversion,
            error,
            format!("invalid ARM LRO URL in '{header_name:?}': {value}"),
        )
    })
}

#[cfg(test)]
mod tests;
