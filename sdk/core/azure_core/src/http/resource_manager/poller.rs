// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Poller helpers for Azure Resource Manager (ARM) long-running operations (LRO).

use crate::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, Headers, RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS},
        poller::{PollerContinuation, PollerResult, PollerState, PollerStatus, StatusMonitor},
        Context, Method, Pipeline, Poller, RawResponse, Request, Response, StatusCode, Url,
    },
    json,
    time::Duration,
};
use async_lock::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use typespec_client_core::http::UrlExt;

pub(super) const AZURE_ASYNC_OPERATION: HeaderName =
    HeaderName::from_static("azure-asyncoperation");
const OPERATION_LOCATION: HeaderName = HeaderName::from_static("operation-location");
pub(super) const LOCATION: HeaderName = HeaderName::from_static("location");

/// Options for [`new_poller()`].
#[derive(Debug, Clone)]
pub struct PollerOptions<'a> {
    /// Allows customization of the method call.
    pub context: Context<'a>,
    /// The time to wait between polling intervals in absence of a `retry-after` header.
    ///
    /// The default is 30 seconds. The minimum time enforced by [`new_poller()`] is 1 second.
    pub frequency: Duration,
}

impl Default for PollerOptions<'_> {
    fn default() -> Self {
        let options = crate::http::poller::PollerOptions::default();
        Self {
            context: options.context,
            frequency: options.frequency,
        }
    }
}

impl<'a> PollerOptions<'a> {
    /// Converts these options into an owned form so they can outlive the current scope.
    #[must_use]
    pub fn into_owned(self) -> PollerOptions<'static> {
        PollerOptions {
            context: self.context.into_owned(),
            frequency: self.frequency,
        }
    }
}

impl<'a> From<crate::http::resource_manager::PollerOptions<'a>>
    for crate::http::poller::PollerOptions<'a>
{
    fn from(options: crate::http::resource_manager::PollerOptions<'a>) -> Self {
        Self {
            context: options.context,
            frequency: options.frequency,
        }
    }
}

/// Extracts the final result response from an operation-location monitor response.
///
/// Used by [`FinalStateVia::OperationLocation`] when no final GET is required.
pub type OperationLocationResultCallback<M> =
    fn(
        Response<M>,
    ) -> crate::Result<Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>>;

/// Selects the final response for an Azure Resource Manager long-running operation.
///
/// Client implementations supply this value from the operation's definition. It applies only to
/// polling with the `azure-asyncoperation` or `operation-location` header. The original request
/// method and available result locations also determine whether a final GET is required.
///
/// Pass `None` to [`new_poller()`] when the operation does not specify a final-state selector.
/// The polling protocol, original request method, and response data determine the final result.
#[derive(Debug)]
pub enum FinalStateVia<M: StatusMonitor> {
    /// Uses the terminal `azure-asyncoperation` response for POST operations.
    ///
    /// PUT and PATCH operations fetch the original resource instead.
    AzureAsyncOperation,
    /// Uses the `location` response header to fetch the result when available.
    ///
    /// With `azure-asyncoperation`, this applies to POST operations; PUT and PATCH operations
    /// fetch the original resource instead.
    Location,
    /// Uses the original request URL for `azure-asyncoperation` POST, PUT, and PATCH operations.
    ///
    /// With `operation-location`, PUT and PATCH operations fetch the original resource unless
    /// the terminal response contains `resourceLocation`. POST operations use `resourceLocation`,
    /// `location`, or the terminal response instead.
    OriginalUri,
    /// Extracts a result from the terminal `operation-location` response.
    ///
    /// The function receives the unmodified monitor response and returns the result response.
    /// It runs only when the result is inline: no `resourceLocation`, no original-resource GET
    /// for PUT or PATCH, and no `location` GET for POST. It does not run for other polling modes
    /// or unsuccessful operations. A 204 response has no result to extract. Errors are propagated
    /// when the poller is awaited.
    ///
    /// Preserve the response's status and headers when selecting its result body. Prefer a
    /// borrowing deserializer containing only the fields needed for extraction, rather than
    /// deserializing and serializing the complete result model.
    ///
    /// # Examples
    ///
    /// This example requires `serde` with the `derive` feature and `serde_json` with the
    /// `raw_value` feature. The borrowed JSON retains escaped strings and nested values.
    ///
    /// ```
    /// use azure_core::{
    ///     http::{
    ///         headers::Headers,
    ///         poller::{PollerStatus, StatusMonitor},
    ///         resource_manager::FinalStateVia,
    ///         JsonFormat, RawResponse, Response, StatusCode,
    ///     },
    ///     Bytes,
    /// };
    /// use serde::Deserialize;
    /// use serde_json::value::RawValue;
    ///
    /// #[derive(Deserialize)]
    /// struct Monitor {
    ///     status: PollerStatus,
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct Resource {
    ///     id: String,
    /// }
    ///
    /// impl StatusMonitor for Monitor {
    ///     type Output = Resource;
    ///     type Format = JsonFormat;
    ///
    ///     fn status(&self) -> PollerStatus {
    ///         self.status.clone()
    ///     }
    /// }
    ///
    /// fn extract_result(response: Response<Monitor>) -> azure_core::Result<Response<Resource>> {
    ///     #[derive(Deserialize)]
    ///     struct Envelope<'a> {
    ///         #[serde(borrow)]
    ///         result: &'a RawValue,
    ///     }
    ///
    ///     let (status, headers, body) = response.deconstruct();
    ///     let bytes = Bytes::from(body);
    ///     let envelope: Envelope<'_> = serde_json::from_slice(&bytes)?;
    ///     let result = bytes.slice_ref(envelope.result.get().as_bytes());
    ///     Ok(RawResponse::from_bytes(status, headers, result).into())
    /// }
    ///
    /// let final_state_via = Some(FinalStateVia::OperationLocation(extract_result));
    /// let response = RawResponse::from_bytes(
    ///     StatusCode::Ok,
    ///     Headers::new(),
    ///     br#"{"status":"Succeeded","result":{"id":"resource-1"},"unused":true}"#.as_slice(),
    /// );
    /// if let Some(FinalStateVia::OperationLocation(extract)) = final_state_via {
    ///     let resource = extract(response.into())?.into_model()?;
    ///     assert_eq!(resource.id, "resource-1");
    /// }
    /// # Ok::<(), azure_core::Error>(())
    /// ```
    OperationLocation(OperationLocationResultCallback<M>),
}

impl<M: StatusMonitor> Clone for FinalStateVia<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M: StatusMonitor> Copy for FinalStateVia<M> {}

/// Creates a poller that supports the following Azure Resource Manager polling modes,
/// applied in priority order:
///
/// 1. `azure-asyncoperation` header: Polls the operation URL from the
///    `azure-asyncoperation` response header.
/// 2. `operation-location` header: Polls the operation URL from the
///    `operation-location` response header.
/// 3. `location` header: Polls the URL from the `location` response header,
///    which doubles as the final resource URL once polling completes.
/// 4. Response body fallback: PUT and PATCH operations poll the original resource URL, reading
///    operation status from the response body's `status` or `properties.provisioningState` field.
///
/// Other operations without polling headers complete with their initial response, unless that
/// response is HTTP 202, which requires a polling URL. The initial request is sent only when the
/// poller is first polled or awaited. Synchronous completion and successful body or location
/// polling return the response already received without an additional GET.
///
/// The initial `api-version` query parameter will always be set on the request URL per
/// [Azure REST API guidelines](https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#lro-operation-location-includes-api-version).
///
/// Set `final_state_via` from the operation's definition to customize final-response selection.
/// Passing `None` follows the polling protocol's method- and response-based rules; it does not
/// select [`FinalStateVia::Location`]. For `azure-asyncoperation`, PUT and PATCH fetch the original
/// resource, POST uses `location` when available, and otherwise the terminal response supplies the
/// result. For `operation-location`, `resourceLocation` takes precedence over those fallbacks.
///
/// Explicit strategies also follow the original request method:
/// `azure-asyncoperation` PUT and PATCH operations fetch the original resource; POST operations
/// can use the terminal response, the original URL, or `location`. For `operation-location`,
/// [`FinalStateVia::Location`] takes precedence when available, followed by the terminal body's
/// `resourceLocation`, the original URL for PUT or PATCH, and `location` for POST. Otherwise,
/// the terminal response supplies the result.
///
/// The stream yields unmodified monitor responses. Awaiting the poller returns the final resource
/// response or the response selected by [`FinalStateVia::OperationLocation`].
///
/// # Panics
///
/// Panics if [`PollerOptions::frequency`] is less than 1 second.
pub fn new_poller<'a, M>(
    pipeline: Pipeline,
    request: Request,
    final_state_via: Option<FinalStateVia<M>>,
    options: Option<PollerOptions<'a>>,
) -> Poller<M>
where
    M: StatusMonitor + DeserializeOwned + Send + 'static,
    M::Output: DeserializeOwned + Send + 'static,
    M::Format: Send + 'static,
{
    const API_VERSION_HEADER_NAME: &str = "api-version";
    let options = options.map(|options| options.into_owned().into());
    let polling = Arc::new(Mutex::new(None::<PollingInfo>));

    Poller::new(
        move |poller_state, poller_options| {
            let pipeline = pipeline.clone();
            let polling = polling.clone();
            let original_method = request.method();
            let resource_url = request.url().clone();
            let api_version = resource_url.query_pairs().find_map(|(k, v)| {
                if k.eq_ignore_ascii_case(API_VERSION_HEADER_NAME) {
                    return Some(v.into_owned());
                }
                None
            });
            let context = poller_options.context.clone();
            let mut request = request.clone();

            Box::pin(async move {
                if let PollerState::More(ref continuation) = poller_state {
                    let mut next_link = match continuation {
                        PollerContinuation::Links { next_link, .. } => next_link.clone(),
                    };

                    // Make sure the original api-version is always set.
                    if let Some(api_version) = api_version.as_deref() {
                        let mut qb = next_link.query_builder();
                        qb.set_pair(API_VERSION_HEADER_NAME, api_version);
                        qb.build();
                    }
                    *request.url_mut() = next_link;
                    request.set_method(Method::Get);
                }

                let response = pipeline.send(&context, &mut request, None).await?;
                let (status_code, headers, body) = response.deconstruct();
                let retry_after = crate::http::poller::get_retry_after(
                    &headers,
                    &[RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS, RETRY_AFTER],
                    &poller_options,
                );
                let response: Response<M> =
                    RawResponse::from_bytes(status_code, headers.clone(), body).into();
                let polling = {
                    let mut info = polling.lock().await;
                    match info.as_ref() {
                        Some(info) => info.clone(),
                        None => {
                            let initial =
                                PollingInfo::new(&headers, request.url(), original_method)?;
                            *info = Some(initial.clone());
                            initial
                        }
                    }
                };
                let initial = matches!(poller_state, PollerState::Initial);
                let status = polling.status(&response, initial)?;
                let previous_next_link = match poller_state {
                    PollerState::Initial => None,
                    PollerState::More(PollerContinuation::Links { next_link, .. }) => {
                        Some(next_link)
                    }
                };

                if matches!(status, PollerStatus::Failed | PollerStatus::Canceled) {
                    let message = if status == PollerStatus::Failed {
                        "Resource Manager long-running operation failed"
                    } else {
                        "Resource Manager long-running operation was canceled"
                    };
                    return Err(crate::Error::new(ErrorKind::Other, message));
                }

                Ok(match status {
                    PollerStatus::InProgress => {
                        let next_link = polling.next_link(
                            &headers,
                            request.url(),
                            &resource_url,
                            previous_next_link.as_ref(),
                        )?;
                        PollerResult::InProgress {
                            response,
                            retry_after,
                            continuation: PollerContinuation::Links {
                                next_link,
                                final_link: None,
                            },
                        }
                    }
                    PollerStatus::Succeeded => {
                        let final_link = polling.final_link(
                            final_state_via,
                            original_method,
                            &resource_url,
                            &response,
                            request.url(),
                        )?;
                        let terminal_response = response.to_raw_response();
                        PollerResult::Succeeded {
                            response,
                            target: Box::new(move || {
                                let pipeline = pipeline.clone();
                                let context = context.clone();
                                Box::pin(async move {
                                    let Some(mut final_link) = final_link else {
                                        if polling.method == PollingMethod::OperationLocation
                                            && terminal_response.status() != StatusCode::NoContent
                                        {
                                            if let Some(FinalStateVia::OperationLocation(extract)) =
                                                final_state_via
                                            {
                                                return extract(terminal_response.into());
                                            }
                                        }
                                        return Ok(terminal_response.into());
                                    };

                                    // Make sure the original api-version is always set.
                                    if let Some(api_version) = api_version.as_deref() {
                                        let mut qb = final_link.query_builder();
                                        qb.set_pair(API_VERSION_HEADER_NAME, api_version);
                                        qb.build();
                                    }

                                    let mut request = Request::new(final_link, Method::Get);
                                    request.insert_header(
                                        crate::http::headers::ACCEPT,
                                        "application/json",
                                    );
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

#[derive(Clone, Copy, PartialEq)]
enum PollingMethod {
    AzureAsyncOperation,
    OperationLocation,
    Location,
    Body,
    Nop,
}

#[derive(Clone)]
struct PollingInfo {
    method: PollingMethod,
    location: Option<Url>,
    operation_url: Option<Url>,
}

impl PollingInfo {
    fn new(headers: &Headers, request_url: &Url, method: Method) -> crate::Result<Self> {
        let async_url = get_header_url(headers, &AZURE_ASYNC_OPERATION, request_url)?;
        let operation_url = if async_url.is_none() {
            get_header_url(headers, &OPERATION_LOCATION, request_url)?
        } else {
            None
        };
        let location = get_location(headers, request_url)?;
        let method = if async_url.is_some() {
            PollingMethod::AzureAsyncOperation
        } else if operation_url.is_some() {
            PollingMethod::OperationLocation
        } else if location.is_some() {
            PollingMethod::Location
        } else if matches!(method, Method::Put | Method::Patch) {
            PollingMethod::Body
        } else {
            PollingMethod::Nop
        };
        Ok(Self {
            method,
            location,
            operation_url: async_url.or(operation_url),
        })
    }

    fn status<M>(&self, response: &Response<M>, initial: bool) -> crate::Result<PollerStatus> {
        if self.method == PollingMethod::Nop {
            return if response.status() == StatusCode::Accepted {
                Err(crate::Error::new(
                    ErrorKind::DataConversion,
                    "Resource Manager long-running operation response is missing a polling URL",
                ))
            } else {
                Ok(PollerStatus::Succeeded)
            };
        }
        if response.status() == StatusCode::NoContent {
            return Ok(PollerStatus::Succeeded);
        }

        #[derive(serde::Deserialize)]
        struct Monitor {
            status: Option<String>,
            properties: Option<Properties>,
        }
        #[derive(serde::Deserialize)]
        struct Properties {
            #[serde(rename = "provisioningState")]
            provisioning_state: Option<String>,
        }

        let monitor: Option<Monitor> = if response.body().is_empty() {
            None
        } else {
            Some(json::from_json(response.body())?)
        };
        let status = monitor.and_then(|monitor| {
            monitor
                .status
                .or_else(|| monitor.properties.and_then(|p| p.provisioning_state))
                .filter(|status| !status.is_empty())
        });
        if let Some(status) = status {
            return Ok(match PollerStatus::from(status.as_str()) {
                PollerStatus::UnknownValue(_) if status.eq_ignore_ascii_case("completed") => {
                    PollerStatus::Succeeded
                }
                PollerStatus::UnknownValue(_) => PollerStatus::InProgress,
                status => status,
            });
        }
        if matches!(
            self.method,
            PollingMethod::AzureAsyncOperation | PollingMethod::OperationLocation
        ) {
            return if initial {
                Ok(PollerStatus::InProgress)
            } else {
                Err(crate::Error::new(
                    ErrorKind::DataConversion,
                    "Resource Manager polling response is missing an operation status",
                ))
            };
        }
        if initial && self.method == PollingMethod::Location {
            return Ok(PollerStatus::InProgress);
        }
        if self.method == PollingMethod::Body && !initial && response.body().is_empty() {
            return Err(crate::Error::new(
                ErrorKind::DataConversion,
                "Resource Manager polling response has an empty body",
            ));
        }
        Ok(match response.status() {
            StatusCode::Accepted => PollerStatus::InProgress,
            StatusCode::Created if initial => PollerStatus::InProgress,
            _ => PollerStatus::Succeeded,
        })
    }

    fn next_link(
        &self,
        headers: &Headers,
        request_url: &Url,
        resource_url: &Url,
        previous_next_link: Option<&Url>,
    ) -> crate::Result<Url> {
        if let Some(url) = &self.operation_url {
            return Ok(url.clone());
        }
        if self.method == PollingMethod::Location {
            if let Some(url) = get_location(headers, request_url)? {
                return Ok(url);
            }
            if let Some(url) = previous_next_link.or(self.location.as_ref()) {
                return Ok(url.clone());
            }
        }
        Ok(resource_url.clone())
    }

    fn final_link<M: StatusMonitor>(
        &self,
        final_state_via: Option<FinalStateVia<M>>,
        original_method: Method,
        resource_url: &Url,
        response: &Response<M>,
        response_url: &Url,
    ) -> crate::Result<Option<Url>> {
        if response.status() == StatusCode::NoContent
            || !matches!(
                self.method,
                PollingMethod::AzureAsyncOperation | PollingMethod::OperationLocation
            )
        {
            return Ok(None);
        }

        if self.method == PollingMethod::AzureAsyncOperation {
            return Ok(match original_method {
                Method::Put | Method::Patch => Some(resource_url.clone()),
                Method::Post => match final_state_via {
                    Some(FinalStateVia::AzureAsyncOperation) => None,
                    Some(FinalStateVia::OriginalUri) => Some(resource_url.clone()),
                    _ => self.location.clone(),
                },
                _ => None,
            });
        }

        if matches!(final_state_via, Some(FinalStateVia::Location)) && self.location.is_some() {
            return Ok(self.location.clone());
        }
        if !response.body().is_empty() {
            #[derive(serde::Deserialize)]
            struct ResourceLocation {
                #[serde(default, rename = "resourceLocation")]
                resource_location: String,
            }
            let result: ResourceLocation = json::from_json(response.body())?;
            if !result.resource_location.is_empty() {
                return response_url
                    .join(&result.resource_location)
                    .map(Some)
                    .map_err(|error| {
                        crate::Error::with_error(
                            ErrorKind::DataConversion,
                            error,
                            "invalid Resource Manager long-running operation resourceLocation",
                        )
                    });
            }
        }
        Ok(match original_method {
            Method::Put | Method::Patch => Some(resource_url.clone()),
            Method::Post => self.location.clone(),
            _ => None,
        })
    }
}

fn get_location(headers: &Headers, request_url: &Url) -> crate::Result<Option<Url>> {
    get_header_url(headers, &LOCATION, request_url)
}

fn get_header_url(
    headers: &Headers,
    header_name: &HeaderName,
    request_url: &Url,
) -> crate::Result<Option<Url>> {
    let Some(value) = headers
        .get_optional_str(header_name)
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };

    if let Ok(url) = Url::parse(value) {
        return Ok(Some(url));
    }

    request_url.join(value).map(Some).map_err(|error| {
        crate::Error::with_error(
            ErrorKind::DataConversion,
            error,
            format!(
                "invalid Resource Manager long-running operation URL in '{}': {value}",
                header_name.as_str(),
            ),
        )
    })
}

#[cfg(test)]
mod tests;
