// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    new_poller, FinalStateVia, PollerOptions, AZURE_ASYNC_OPERATION, LOCATION, OPERATION_LOCATION,
};
use crate::{
    http::poller::{PollerStatus, StatusMonitor},
    http::{
        headers::{
            HeaderName, Headers, ACCEPT, CLIENT_REQUEST_ID, CONTENT_LENGTH, CONTENT_TYPE,
            RETRY_AFTER, RETRY_AFTER_MS, X_MS_RETRY_AFTER_MS,
        },
        request::options::ClientRequestId,
        AsyncRawResponse, ClientOptions, Context, HttpClient, JsonFormat, Method, Pipeline, Poller,
        RawResponse, Request, Response, StatusCode, Transport,
    },
    time::Duration,
    Bytes,
};
use azure_core_test::http::MockHttpClient;
use futures::{FutureExt as _, TryStreamExt as _};
use serde_json::value::RawValue;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use typespec_client_core::fmt::SafeDebug;

#[derive(SafeDebug, serde::Deserialize)]
struct ArmOperationStatus {
    #[serde(default)]
    status: Option<PollerStatus>,
    #[serde(default)]
    properties: Option<ArmOperationProperties>,
}

#[derive(SafeDebug, serde::Deserialize)]
struct ArmOperationProperties {
    #[serde(default, rename = "provisioningState")]
    provisioning_state: Option<PollerStatus>,
}

#[derive(SafeDebug, serde::Deserialize)]
struct ArmResource {
    id: String,
}

impl StatusMonitor for ArmOperationStatus {
    type Output = ArmResource;
    type Format = JsonFormat;

    fn status(&self) -> PollerStatus {
        self.status
            .clone()
            .or_else(|| {
                self.properties
                    .as_ref()
                    .and_then(|properties| properties.provisioning_state.clone())
            })
            .unwrap_or_else(|| PollerStatus::UnknownValue("unknown".to_owned()))
    }
}

fn pipeline_with(client: Arc<dyn HttpClient>) -> Pipeline {
    Pipeline::new(
        None,
        None,
        ClientOptions {
            transport: Some(Transport::new(client)),
            retry: crate::http::RetryOptions::none(),
            ..Default::default()
        },
        vec![],
        vec![],
        None,
    )
}

#[tokio::test]
async fn new_poller_supports_async_pattern() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(AZURE_ASYNC_OPERATION, "https://example.com/operations/1");
                    headers.insert(LOCATION, "https://example.com/resources/1");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                2 => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"status":"Succeeded"}"#.to_vec(),
                ),
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"id":"resource-1"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/1?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Put,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-1");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/1?api-version=2024-01-01",
            "https://example.com/operations/1?api-version=2024-01-01",
            "https://example.com/resources/1?api-version=2024-01-01",
        ]
    );
}

#[tokio::test]
async fn new_poller_supports_body_pattern() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"properties":{"provisioningState":"InProgress"}}"#.to_vec(),
                    )
                }
                2 => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"properties":{"provisioningState":"Succeeded"},"id":"resource-2"}"#
                        .to_vec(),
                ),
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"id":"resource-2"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/2?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Put,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-2");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/2?api-version=2024-01-01",
            "https://example.com/resources/2?api-version=2024-01-01",
        ]
    );
}

/// When the initial response has a relative `operation-location` header, the poll URL is resolved
/// against the request URL and the final GET uses the original resource URL when no `location`
/// header is present.
#[tokio::test]
async fn new_poller_supports_operation_location_pattern() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(OPERATION_LOCATION, "/operations/4?api-version=2024-01-01");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                2 => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"status":"Succeeded"}"#.to_vec(),
                ),
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"id":"resource-4"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/4?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Put,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-4");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/4?api-version=2024-01-01",
            "https://example.com/operations/4?api-version=2024-01-01",
            "https://example.com/resources/4?api-version=2024-01-01",
        ]
    );
}

#[tokio::test]
async fn new_poller_supports_location_pattern() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(LOCATION, "https://example.com/operations/3");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                2 => {
                    let mut headers = Headers::new();
                    headers.insert(LOCATION, "https://example.com/resources/3");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        headers,
                        br#"{"status":"Succeeded","id":"resource-3"}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"id":"resource-3"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/3?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-3");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/3?api-version=2024-01-01",
            "https://example.com/operations/3?api-version=2024-01-01",
        ]
    );
}

/// A POST with `azure-asyncoperation` but no `location` returns the terminal response.
#[tokio::test]
async fn new_poller_async_pattern_no_location_uses_terminal_response() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(AZURE_ASYNC_OPERATION, "https://example.com/operations/5");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                2 => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"status":"Succeeded","id":"resource-5"}"#.to_vec(),
                ),
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"id":"resource-5"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/5?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Post,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-5");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/5?api-version=2024-01-01",
            "https://example.com/operations/5?api-version=2024-01-01",
        ]
    );
}

/// When the initial PUT response already contains a terminal `provisioningState`, the poller
/// completes without additional polling.
#[tokio::test]
async fn new_poller_body_pattern_synchronous_completion() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let requests_for_mock = requests.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                br#"{"properties":{"provisioningState":"Succeeded"},"id":"resource-6"}"#.to_vec(),
            ))
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/6?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Put,
        ),
        None,
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-6");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec!["https://example.com/resources/6?api-version=2024-01-01",]
    );
}

/// When the operation status poll returns "Failed", the poller returns an error.
#[tokio::test]
async fn new_poller_async_pattern_failed() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(AZURE_ASYNC_OPERATION, "https://example.com/operations/7");
                    headers.insert(LOCATION, "https://example.com/resources/7");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"status":"Failed"}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/7?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    let err = poller.await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "Resource Manager long-running operation failed"
    );
}

/// When the operation status poll returns a non-success HTTP status, the poller returns an error.
#[tokio::test]
async fn new_poller_async_pattern_poll_error() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(AZURE_ASYNC_OPERATION, "https://example.com/operations/7");
                    headers.insert(LOCATION, "https://example.com/resources/7");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::NotFound,
                    Headers::new(),
                    br#"{"error":{"code":"NotFound","message":"missing"}}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/7?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    assert!(
        poller.await.is_err(),
        "non-success polling status should return an error"
    );
}

/// When the provisioning state is "Failed", the poller returns an error.
#[tokio::test]
async fn new_poller_body_pattern_failed() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"properties":{"provisioningState":"InProgress"}}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"properties":{"provisioningState":"Failed"}}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/8?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Patch,
        ),
        None,
        None,
    );

    assert!(
        poller.await.is_err(),
        "failed provisioning state should return an error"
    );
}

/// When body-pattern polling returns a non-success HTTP status, the poller returns an error.
#[tokio::test]
async fn new_poller_body_pattern_poll_error() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"properties":{"provisioningState":"InProgress"}}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::NotFound,
                    Headers::new(),
                    br#"{"error":{"code":"NotFound","message":"missing"}}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/8?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Patch,
        ),
        None,
        None,
    );

    assert!(
        poller.await.is_err(),
        "non-success polling status should return an error"
    );
}

/// A 204 No Content response with empty body is treated as a successful terminal state.
#[tokio::test]
async fn new_poller_body_pattern_204_no_content() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"properties":{"provisioningState":"InProgress"}}"#.to_vec(),
                    )
                }
                // 204 with empty body signals completion
                _ => AsyncRawResponse::from_bytes(StatusCode::NoContent, Headers::new(), vec![]),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/9?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Put,
        ),
        None,
        None,
    );

    // 204 responses have no body; the poller should complete successfully.
    let _ = poller.await.unwrap();

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/9?api-version=2024-01-01",
            "https://example.com/resources/9?api-version=2024-01-01",
        ]
    );
}

/// When a location-pattern poll response returns a non-success HTTP status, the poller treats it
/// as a failed operation.
#[tokio::test]
async fn new_poller_location_pattern_failed() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(LOCATION, "https://example.com/operations/10");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                // Non-success status indicates operation failure
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::BadRequest,
                    Headers::new(),
                    br#"{"error":{"code":"BadRequest","message":"operation failed"}}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/10?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    assert!(
        poller.await.is_err(),
        "non-success HTTP status during location polling should return an error"
    );
}

/// When location-pattern polling reports failed provisioning state, the poller returns an error.
#[tokio::test]
async fn new_poller_location_pattern_failed_with_provisioning_state() {
    let calls = Arc::new(Mutex::new(0usize));
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |_request| {
        let calls = calls_for_mock.clone();
        async move {
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(LOCATION, "https://example.com/operations/10");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                _ => AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    br#"{"properties":{"provisioningState":"Failed"}}"#.to_vec(),
                ),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/10?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    assert!(
        poller.await.is_err(),
        "failed provisioning state during location polling should return an error"
    );
}

/// A 204 No Content at the location URL signals successful completion of a DELETE-like operation.
#[tokio::test]
async fn new_poller_location_pattern_204_no_content() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::new(Mutex::new(0usize));
    let requests_for_mock = requests.clone();
    let calls_for_mock = calls.clone();
    let mock = Arc::new(MockHttpClient::new(move |request| {
        let requests = requests_for_mock.clone();
        let calls = calls_for_mock.clone();
        let url = request.url().to_string();
        async move {
            requests.lock().unwrap().push(url);
            let mut call = calls.lock().unwrap();
            *call += 1;

            let response = match *call {
                1 => {
                    let mut headers = Headers::new();
                    headers.insert(LOCATION, "https://example.com/operations/11");
                    headers.insert(RETRY_AFTER_MS, "0");
                    AsyncRawResponse::from_bytes(
                        StatusCode::Accepted,
                        headers,
                        br#"{"status":"InProgress"}"#.to_vec(),
                    )
                }
                // 204 with empty body at the location URL signals completion
                _ => AsyncRawResponse::from_bytes(StatusCode::NoContent, Headers::new(), vec![]),
            };
            Ok(response)
        }
        .boxed()
    })) as Arc<dyn HttpClient>;

    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/11?api-version=2024-01-01"
                .parse()
                .unwrap(),
            Method::Delete,
        ),
        None,
        None,
    );

    let _ = poller.await.unwrap();

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/11?api-version=2024-01-01",
            "https://example.com/operations/11?api-version=2024-01-01",
        ]
    );
}

const RESOURCE_URL: &str = "https://example.com/resources/1?api-version=original";
const POLLING_URL: &str = "https://example.com/operations/1?api-version=original";
const LOCATION_URL: &str = "https://example.com/results/1?api-version=original";
const RESOURCE_LOCATION_URL: &str = "https://example.com/results/2?api-version=original";

type ExpectedRequest = (Method, &'static str, AsyncRawResponse);

fn scripted_pipeline(
    responses: Vec<ExpectedRequest>,
) -> (Pipeline, Arc<Mutex<VecDeque<ExpectedRequest>>>) {
    let pending = Arc::new(Mutex::new(VecDeque::from(responses)));
    let pending_for_mock = pending.clone();
    let mock = MockHttpClient::new(move |request| {
        let (method, url, response) = pending_for_mock
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request");
        assert_eq!(request.method(), method);
        assert_eq!(request.url().as_str(), url);
        async move { Ok(response) }.boxed()
    });
    (pipeline_with(Arc::new(mock)), pending)
}

fn operation_response(header: HeaderName, location: bool) -> AsyncRawResponse {
    let mut headers = Headers::new();
    headers.insert(header, "/operations/1?api-version=other");
    if location {
        headers.insert(LOCATION, "/results/1?api-version=other");
    }
    headers.insert(RETRY_AFTER_MS, "0");
    AsyncRawResponse::from_bytes(
        StatusCode::Accepted,
        headers,
        br#"{"status":"InProgress"}"#.as_slice(),
    )
}

fn monitor_response(body: &'static [u8]) -> AsyncRawResponse {
    let mut headers = Headers::new();
    headers.insert(RETRY_AFTER_MS, "0");
    AsyncRawResponse::from_bytes(StatusCode::Ok, headers, body)
}

fn extract_result(response: Response<ArmOperationStatus>) -> crate::Result<Response<ArmResource>> {
    #[derive(serde::Deserialize)]
    struct Envelope<'a> {
        #[serde(borrow)]
        result: &'a RawValue,
    }

    let (status, headers, body) = response.deconstruct();
    let bytes = Bytes::from(body);
    let envelope: Envelope<'_> = serde_json::from_slice(&bytes)?;
    let result = bytes.slice_ref(envelope.result.get().as_bytes());
    Ok(RawResponse::from_bytes(status, headers, result).into())
}

fn unexpected_extraction(
    _response: Response<ArmOperationStatus>,
) -> crate::Result<Response<ArmResource>> {
    panic!("result extraction must not run")
}

#[test]
fn poller_options_defaults_and_conversion() {
    let options = PollerOptions::default();
    let base = crate::http::poller::PollerOptions::default();
    assert!(options.context.is_empty());
    assert_eq!(options.frequency, base.frequency);

    let context = Context::new().with_value(42u32);
    let options = PollerOptions {
        context: context.to_borrowed(),
        frequency: Duration::seconds(7),
    };
    let converted: crate::http::poller::PollerOptions<'_> = options.clone().into();
    assert_eq!(converted.context.value::<u32>(), Some(&42));
    assert_eq!(converted.frequency, Duration::seconds(7));

    let owned = options.into_owned();
    drop(context);
    let converted: crate::http::poller::PollerOptions<'static> = owned.into();
    assert_eq!(converted.context.value::<u32>(), Some(&42));
    assert_eq!(converted.frequency, Duration::seconds(7));
}

#[test]
fn final_state_via_does_not_require_cloneable_models() {
    fn assert_copy<T: Copy>() {}
    assert_copy::<FinalStateVia<ArmOperationStatus>>();
}

#[test]
#[should_panic(expected = "minimum polling frequency is 1 second")]
fn poller_options_minimum_frequency() {
    let (pipeline, _) = scripted_pipeline(vec![]);
    let _: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
        None,
        Some(PollerOptions {
            frequency: Duration::milliseconds(999),
            ..Default::default()
        }),
    );
}

#[tokio::test]
async fn explicit_final_response_selection() {
    let strategies = [
        None,
        Some(FinalStateVia::AzureAsyncOperation),
        Some(FinalStateVia::Location),
        Some(FinalStateVia::OriginalUri),
        Some(FinalStateVia::OperationLocation(unexpected_extraction)),
    ];
    for header in [AZURE_ASYNC_OPERATION, OPERATION_LOCATION] {
        for method in [Method::Post, Method::Put, Method::Patch, Method::Delete] {
            for location in [false, true] {
                for strategy in strategies {
                    let final_url = if header == OPERATION_LOCATION
                        && location
                        && matches!(strategy, Some(FinalStateVia::Location))
                    {
                        Some(LOCATION_URL)
                    } else if method == Method::Delete {
                        None
                    } else if matches!(method, Method::Put | Method::Patch) {
                        Some(RESOURCE_URL)
                    } else if header == AZURE_ASYNC_OPERATION {
                        match strategy {
                            Some(FinalStateVia::AzureAsyncOperation) => None,
                            Some(FinalStateVia::OriginalUri) => Some(RESOURCE_URL),
                            _ => location.then_some(LOCATION_URL),
                        }
                    } else {
                        location.then_some(LOCATION_URL)
                    };
                    let strategy = if final_url.is_none()
                        && header == OPERATION_LOCATION
                        && matches!(strategy, Some(FinalStateVia::OperationLocation(_)))
                    {
                        Some(FinalStateVia::OperationLocation(extract_result))
                    } else {
                        strategy
                    };
                    let mut responses =
                        vec![
                        (method, RESOURCE_URL, operation_response(header.clone(), location)),
                        (
                            Method::Get,
                            POLLING_URL,
                            monitor_response(
                                br#"{"status":"Succeeded","id":"inline","result":{"id":"inline"}}"#,
                            ),
                        ),
                    ];
                    if let Some(url) = final_url {
                        responses.push((
                            Method::Get,
                            url,
                            monitor_response(br#"{"id":"fetched"}"#),
                        ));
                    }
                    let (pipeline, pending) = scripted_pipeline(responses);
                    let poller: Poller<ArmOperationStatus> = new_poller(
                        pipeline,
                        Request::new(RESOURCE_URL.parse().unwrap(), method),
                        strategy,
                        None,
                    );
                    let resource = poller.await.unwrap().into_model().unwrap();
                    assert_eq!(
                        resource.id,
                        if final_url.is_some() {
                            "fetched"
                        } else {
                            "inline"
                        }
                    );
                    assert!(pending.lock().unwrap().is_empty());
                }
            }
        }
    }
}

#[tokio::test]
async fn operation_location_resource_location_precedence() {
    for method in [Method::Post, Method::Put, Method::Patch, Method::Delete] {
        for strategy in [
            None,
            Some(FinalStateVia::AzureAsyncOperation),
            Some(FinalStateVia::Location),
            Some(FinalStateVia::OriginalUri),
            Some(FinalStateVia::OperationLocation(unexpected_extraction)),
        ] {
            let final_url = if matches!(strategy, Some(FinalStateVia::Location)) {
                LOCATION_URL
            } else {
                RESOURCE_LOCATION_URL
            };
            let (pipeline, pending) = scripted_pipeline(vec![
                (method, RESOURCE_URL, operation_response(OPERATION_LOCATION, true)),
                (
                    Method::Get,
                    POLLING_URL,
                    monitor_response(
                        br#"{"status":"Succeeded","resourceLocation":"/results/2?api-version=other"}"#,
                    ),
                ),
                (Method::Get, final_url, monitor_response(br#"{"id":"fetched"}"#)),
            ]);
            let poller: Poller<ArmOperationStatus> = new_poller(
                pipeline,
                Request::new(RESOURCE_URL.parse().unwrap(), method),
                strategy,
                None,
            );
            assert_eq!(poller.await.unwrap().into_model().unwrap().id, "fetched");
            assert!(pending.lock().unwrap().is_empty());
        }
    }
}

#[tokio::test]
async fn operation_location_borrowed_result_after_streaming() {
    static EXTRACTIONS: AtomicUsize = AtomicUsize::new(0);
    const BODY: &[u8] =
        br#"{"status":"Succeeded","result":{ "id":"a\"b", "nested":{"values":[1,2]} },"unused":true}"#;
    let (pipeline, pending) = scripted_pipeline(vec![
        (
            Method::Post,
            RESOURCE_URL,
            operation_response(OPERATION_LOCATION, false),
        ),
        (
            Method::Get,
            POLLING_URL,
            monitor_response(br#"{"status":"InProgress"}"#),
        ),
        (
            Method::Get,
            POLLING_URL,
            monitor_response(br#"{"status":"InProgress"}"#),
        ),
        (Method::Get, POLLING_URL, monitor_response(BODY)),
    ]);
    let mut poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
        Some(FinalStateVia::OperationLocation(|response| {
            EXTRACTIONS.fetch_add(1, Ordering::SeqCst);
            extract_result(response)
        })),
        None,
    );
    let mut responses = Vec::new();
    while let Some(response) = poller.try_next().await.unwrap() {
        responses.push(response);
    }
    assert_eq!(responses.len(), 4);
    let terminal = responses.last().unwrap();
    assert_eq!(terminal.body().as_ref(), BODY);
    assert_eq!(EXTRACTIONS.load(Ordering::SeqCst), 0);

    let result = poller.await.unwrap();
    assert_eq!(EXTRACTIONS.load(Ordering::SeqCst), 1);
    assert_eq!(result.status(), terminal.status());
    assert_eq!(result.headers(), terminal.headers());
    assert_eq!(
        result.body().as_ref(),
        br#"{ "id":"a\"b", "nested":{"values":[1,2]} }"#
    );
    let offset = BODY
        .windows(7)
        .position(|value| value == b"{ \"id\":")
        .unwrap();
    assert_eq!(result.body().as_ptr(), terminal.body()[offset..].as_ptr());
    assert_eq!(result.into_model().unwrap().id, "a\"b");
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn operation_location_callback_error() {
    let (pipeline, pending) = scripted_pipeline(vec![
        (
            Method::Post,
            RESOURCE_URL,
            operation_response(OPERATION_LOCATION, false),
        ),
        (
            Method::Get,
            POLLING_URL,
            monitor_response(br#"{"status":"Succeeded","unused":true}"#),
        ),
    ]);
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
        Some(FinalStateVia::OperationLocation(extract_result)),
        None,
    );
    let error = poller.await.unwrap_err();
    assert!(
        error.to_string().contains("missing field `result`"),
        "{error}"
    );
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn operation_location_unsuccessful_status_skips_callback() {
    for (body, message) in [
        (br#"{"status":"Failed"}"#.as_slice(), "failed"),
        (br#"{"status":"Canceled"}"#.as_slice(), "canceled"),
    ] {
        let (pipeline, pending) = scripted_pipeline(vec![
            (
                Method::Post,
                RESOURCE_URL,
                operation_response(OPERATION_LOCATION, false),
            ),
            (Method::Get, POLLING_URL, monitor_response(body)),
        ]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
            Some(FinalStateVia::OperationLocation(unexpected_extraction)),
            None,
        );
        assert!(poller.await.unwrap_err().to_string().contains(message));
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn explicit_strategy_keeps_header_priority() {
    let mut headers = Headers::new();
    headers.insert(OPERATION_LOCATION, "/operations/1?api-version=other");
    headers.insert(AZURE_ASYNC_OPERATION, "/async?api-version=other");
    headers.insert(RETRY_AFTER_MS, "0");
    let initial = AsyncRawResponse::from_bytes(
        StatusCode::Accepted,
        headers,
        br#"{"status":"InProgress"}"#.as_slice(),
    );
    let (pipeline, pending) = scripted_pipeline(vec![
        (Method::Post, RESOURCE_URL, initial),
        (
            Method::Get,
            "https://example.com/async?api-version=original",
            monitor_response(br#"{"status":"Succeeded","id":"inline"}"#),
        ),
    ]);
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
        Some(FinalStateVia::OperationLocation(unexpected_extraction)),
        None,
    );
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "inline");
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn explicit_strategy_does_not_change_location_or_body_polling() {
    for header in [None, Some(LOCATION)] {
        let polling_url = if header.is_some() {
            POLLING_URL
        } else {
            RESOURCE_URL
        };
        let initial = match header {
            Some(header) => operation_response(header, false),
            None => monitor_response(br#"{"status":"InProgress"}"#),
        };
        let (pipeline, pending) = scripted_pipeline(vec![
            (Method::Put, RESOURCE_URL, initial),
            (
                Method::Get,
                polling_url,
                monitor_response(br#"{"status":"Succeeded","id":"fetched"}"#),
            ),
        ]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
            Some(FinalStateVia::OperationLocation(unexpected_extraction)),
            None,
        );
        assert_eq!(poller.await.unwrap().into_model().unwrap().id, "fetched");
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn explicit_strategy_no_content_skips_extraction_and_final_get() {
    for header in [AZURE_ASYNC_OPERATION, OPERATION_LOCATION] {
        let (pipeline, pending) = scripted_pipeline(vec![
            (
                Method::Delete,
                RESOURCE_URL,
                operation_response(header, false),
            ),
            (
                Method::Get,
                POLLING_URL,
                AsyncRawResponse::from_bytes(StatusCode::NoContent, Headers::new(), ""),
            ),
        ]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Delete),
            Some(FinalStateVia::OperationLocation(unexpected_extraction)),
            None,
        );
        let response = poller.await.unwrap();
        assert_eq!(response.status(), StatusCode::NoContent);
        assert!(response.body().is_empty());
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn operation_location_invalid_resource_location() {
    for body in [
        br#"{"status":"Succeeded","resourceLocation":42}"#.as_slice(),
        br#"{"status":"Succeeded","resourceLocation":null}"#.as_slice(),
        br#"{"status":"Succeeded","resourceLocation":"http://["}"#.as_slice(),
    ] {
        let (pipeline, pending) = scripted_pipeline(vec![
            (
                Method::Post,
                RESOURCE_URL,
                operation_response(OPERATION_LOCATION, false),
            ),
            (Method::Get, POLLING_URL, monitor_response(body)),
        ]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
            Some(FinalStateVia::OperationLocation(unexpected_extraction)),
            None,
        );
        let error = poller.await.unwrap_err();
        assert_eq!(error.kind(), &crate::error::ErrorKind::DataConversion);
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn default_strategy_retains_polling_url_without_repeated_headers() {
    for header in [AZURE_ASYNC_OPERATION, OPERATION_LOCATION] {
        let (pipeline, pending) = scripted_pipeline(vec![
            (Method::Put, RESOURCE_URL, operation_response(header, false)),
            (
                Method::Get,
                POLLING_URL,
                monitor_response(br#"{"status":"InProgress"}"#),
            ),
            (
                Method::Get,
                POLLING_URL,
                monitor_response(br#"{"status":"Succeeded"}"#),
            ),
            (
                Method::Get,
                RESOURCE_URL,
                monitor_response(br#"{"id":"fetched"}"#),
            ),
        ]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
            None,
            None,
        );
        assert_eq!(poller.await.unwrap().into_model().unwrap().id, "fetched");
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn body_polling_tracks_in_progress_states_and_reuses_terminal_response() {
    for method in [Method::Put, Method::Patch] {
        let mut responses = Vec::new();
        for (index, body) in [
            br#"{"properties":{"provisioningState":"Started"}}"#.as_slice(),
            br#"{"properties":{"provisioningState":"Updating"}}"#.as_slice(),
            br#"{"properties":{"provisioningState":"Creating"}}"#.as_slice(),
            br#"{"properties":{"provisioningState":"Succeeded"},"id":"finished"}"#.as_slice(),
        ]
        .into_iter()
        .enumerate()
        {
            responses.push((
                if index == 0 { method } else { Method::Get },
                RESOURCE_URL,
                monitor_response(body),
            ));
        }
        let (pipeline, pending) = scripted_pipeline(responses);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), method),
            None,
            None,
        );
        assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn omitted_selector_uses_operation_location_fallbacks() {
    for method in [Method::Post, Method::Put, Method::Patch, Method::Delete] {
        for location in [false, true] {
            for resource_location in [false, true] {
                let final_url = if resource_location {
                    Some(RESOURCE_LOCATION_URL)
                } else if matches!(method, Method::Put | Method::Patch) {
                    Some(RESOURCE_URL)
                } else if method == Method::Post && location {
                    Some(LOCATION_URL)
                } else {
                    None
                };
                let body = if resource_location {
                    br#"{"status":"Succeeded","resourceLocation":"/results/2?api-version=other"}"#
                        .as_slice()
                } else {
                    br#"{"status":"Succeeded","id":"inline"}"#.as_slice()
                };
                let mut responses = vec![
                    (
                        method,
                        RESOURCE_URL,
                        operation_response(OPERATION_LOCATION, location),
                    ),
                    (Method::Get, POLLING_URL, monitor_response(body)),
                ];
                if let Some(url) = final_url {
                    responses.push((Method::Get, url, monitor_response(br#"{"id":"fetched"}"#)));
                }
                let (pipeline, pending) = scripted_pipeline(responses);
                let poller: Poller<ArmOperationStatus> = new_poller(
                    pipeline,
                    Request::new(RESOURCE_URL.parse().unwrap(), method),
                    None,
                    None,
                );
                assert_eq!(
                    poller.await.unwrap().into_model().unwrap().id,
                    if final_url.is_some() {
                        "fetched"
                    } else {
                        "inline"
                    }
                );
                assert!(pending.lock().unwrap().is_empty());
            }
        }
    }
}

#[tokio::test]
async fn accepted_post_or_delete_requires_polling_headers() {
    for method in [Method::Post, Method::Delete] {
        let (pipeline, pending) = scripted_pipeline(vec![(
            method,
            RESOURCE_URL,
            AsyncRawResponse::from_bytes(StatusCode::Accepted, Headers::new(), ""),
        )]);
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline,
            Request::new(RESOURCE_URL.parse().unwrap(), method),
            None,
            None,
        );
        let error = poller.await.unwrap_err();
        assert!(
            error.to_string().contains("missing a polling URL"),
            "{error}"
        );
        assert!(pending.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn synchronous_completion_is_lazy_and_does_not_fetch_again() {
    for method in [Method::Post, Method::Delete, Method::Put, Method::Patch] {
        for status in [StatusCode::Ok, StatusCode::NoContent] {
            let body = if status == StatusCode::NoContent {
                ""
            } else {
                r#"{"id":"synchronous"}"#
            };
            let (pipeline, pending) = scripted_pipeline(vec![(
                method,
                RESOURCE_URL,
                AsyncRawResponse::from_bytes(status, Headers::new(), body),
            )]);
            let poller: Poller<ArmOperationStatus> = new_poller(
                pipeline,
                Request::new(RESOURCE_URL.parse().unwrap(), method),
                None,
                None,
            );
            assert_eq!(pending.lock().unwrap().len(), 1);
            let response = poller.await.unwrap();
            assert_eq!(response.status(), status);
            assert_eq!(response.body().as_ref(), body.as_bytes());
            if status != StatusCode::NoContent {
                assert_eq!(response.into_model().unwrap().id, "synchronous");
            }
            assert!(pending.lock().unwrap().is_empty());
        }
    }
}

#[tokio::test]
async fn created_body_response_without_state_keeps_polling() {
    let mut headers = Headers::new();
    headers.insert(RETRY_AFTER_MS, "0");
    let (pipeline, pending) = scripted_pipeline(vec![
        (
            Method::Put,
            RESOURCE_URL,
            AsyncRawResponse::from_bytes(StatusCode::Created, headers, ""),
        ),
        (
            Method::Get,
            RESOURCE_URL,
            monitor_response(br#"{"id":"finished"}"#),
        ),
    ]);
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
        None,
        None,
    );
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn operation_polling_rejects_missing_status_and_malformed_json() {
    for header in [AZURE_ASYNC_OPERATION, OPERATION_LOCATION] {
        for body in [b"".as_slice(), b"{}", b"not JSON"] {
            let (pipeline, pending) = scripted_pipeline(vec![
                (
                    Method::Post,
                    RESOURCE_URL,
                    operation_response(header.clone(), false),
                ),
                (Method::Get, POLLING_URL, monitor_response(body)),
            ]);
            let poller: Poller<ArmOperationStatus> = new_poller(
                pipeline,
                Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
                None,
                None,
            );
            assert_eq!(
                poller.await.unwrap_err().kind(),
                &crate::error::ErrorKind::DataConversion
            );
            assert!(pending.lock().unwrap().is_empty());
        }
    }
}

#[tokio::test]
async fn location_polling_follows_updated_url_until_terminal_response() {
    let mut headers = Headers::new();
    headers.insert(LOCATION, "/results/1?api-version=other");
    headers.insert(RETRY_AFTER_MS, "0");
    let (pipeline, pending) = scripted_pipeline(vec![
        (
            Method::Delete,
            RESOURCE_URL,
            operation_response(LOCATION, false),
        ),
        (
            Method::Get,
            POLLING_URL,
            AsyncRawResponse::from_bytes(StatusCode::Accepted, headers, ""),
        ),
        (
            Method::Get,
            LOCATION_URL,
            monitor_response(br#"{"id":"finished"}"#),
        ),
    ]);
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Delete),
        None,
        None,
    );
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn operation_polling_does_not_switch_to_location_header() {
    let mut headers = Headers::new();
    headers.insert(LOCATION, "/results/1");
    headers.insert(RETRY_AFTER_MS, "0");
    let (pipeline, pending) = scripted_pipeline(vec![
        (
            Method::Post,
            RESOURCE_URL,
            operation_response(OPERATION_LOCATION, false),
        ),
        (
            Method::Get,
            POLLING_URL,
            AsyncRawResponse::from_bytes(StatusCode::Ok, headers, r#"{"status":"Running"}"#),
        ),
        (
            Method::Get,
            POLLING_URL,
            monitor_response(br#"{"status":"Succeeded","id":"finished"}"#),
        ),
    ]);
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline,
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
        None,
        None,
    );
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
    assert!(pending.lock().unwrap().is_empty());
}

#[tokio::test]
async fn transport_errors_propagate_from_each_request_stage() {
    for failing_call in 0..3 {
        let calls = Arc::new(AtomicUsize::new(0));
        let calls_for_mock = calls.clone();
        let mock = MockHttpClient::new(move |request| {
            let call = calls_for_mock.fetch_add(1, Ordering::SeqCst);
            assert_eq!(
                request.method(),
                if call == 0 { Method::Put } else { Method::Get }
            );
            assert_eq!(
                request.url().as_str(),
                if call == 1 { POLLING_URL } else { RESOURCE_URL }
            );
            async move {
                if call == failing_call {
                    return Err(crate::Error::new(
                        crate::error::ErrorKind::Io,
                        "transport failed",
                    ));
                }
                Ok(match call {
                    0 => operation_response(OPERATION_LOCATION, false),
                    1 => monitor_response(br#"{"status":"Succeeded"}"#),
                    _ => panic!("unexpected final request after a transport failure"),
                })
            }
            .boxed()
        });
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline_with(Arc::new(mock)),
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
            None,
            None,
        );
        let error = poller.await.unwrap_err();
        assert_eq!(error.kind(), &crate::error::ErrorKind::Io);
        assert!(error.to_string().contains("transport failed"));
        assert_eq!(calls.load(Ordering::SeqCst), failing_call + 1);
    }
}

#[tokio::test]
async fn http_errors_propagate_from_initial_poll_and_final_requests() {
    for failing_call in 0..3 {
        for status in [StatusCode::BadRequest, StatusCode::TooManyRequests] {
            let mut responses = Vec::new();
            for call in 0..=failing_call {
                let response = if call == failing_call {
                    AsyncRawResponse::from_bytes(
                        status,
                        Headers::new(),
                        r#"{"error":{"code":"ServiceError","message":"request failed"}}"#,
                    )
                } else if call == 0 {
                    operation_response(OPERATION_LOCATION, false)
                } else {
                    monitor_response(br#"{"status":"Succeeded"}"#)
                };
                responses.push((
                    if call == 0 { Method::Put } else { Method::Get },
                    if call == 1 { POLLING_URL } else { RESOURCE_URL },
                    response,
                ));
            }
            let (pipeline, pending) = scripted_pipeline(responses);
            let poller: Poller<ArmOperationStatus> = new_poller(
                pipeline,
                Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
                None,
                None,
            );
            let error = poller.await.unwrap_err();
            assert!(
                matches!(
                    error.kind(),
                    crate::error::ErrorKind::HttpResponse { status: actual, .. } if *actual == status
                ),
                "{error:?}"
            );
            assert!(pending.lock().unwrap().is_empty());
        }
    }
}

#[tokio::test(start_paused = true)]
async fn polling_delays_follow_header_precedence_and_frequency() {
    for (values, seconds) in [
        (vec![(RETRY_AFTER, "3")], 3),
        (vec![(X_MS_RETRY_AFTER_MS, "2000"), (RETRY_AFTER, "9")], 2),
        (
            vec![
                (RETRY_AFTER_MS, "1000"),
                (X_MS_RETRY_AFTER_MS, "2000"),
                (RETRY_AFTER, "9"),
            ],
            1,
        ),
        (vec![], 4),
    ] {
        let mut headers = Headers::new();
        for (name, value) in values {
            headers.insert(name, value);
        }
        headers.insert(OPERATION_LOCATION, POLLING_URL);
        let responses = Arc::new(Mutex::new(VecDeque::from([
            AsyncRawResponse::from_bytes(StatusCode::Accepted, headers, ""),
            operation_response(OPERATION_LOCATION, false),
            monitor_response(br#"{"status":"Succeeded","id":"finished"}"#),
        ])));
        let responses_for_mock = responses.clone();
        let times = Arc::new(Mutex::new(Vec::new()));
        let times_for_mock = times.clone();
        let mock = MockHttpClient::new(move |_| {
            times_for_mock
                .lock()
                .unwrap()
                .push(tokio::time::Instant::now());
            let response = responses_for_mock
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected request");
            async move { Ok(response) }.boxed()
        });
        let poller: Poller<ArmOperationStatus> = new_poller(
            pipeline_with(Arc::new(mock)),
            Request::new(RESOURCE_URL.parse().unwrap(), Method::Post),
            None,
            Some(PollerOptions {
                frequency: Duration::seconds(4),
                ..Default::default()
            }),
        );
        assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
        let times = times.lock().unwrap();
        assert_eq!(times.len(), 3);
        assert_eq!(times[1] - times[0], std::time::Duration::from_secs(seconds));
        assert_eq!(times[2] - times[1], std::time::Duration::ZERO);
        assert!(responses.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn borrowed_context_flows_to_initial_poll_and_final_requests() {
    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_mock = calls.clone();
    let mock = MockHttpClient::new(move |request| {
        assert_eq!(
            request.headers().get_optional_str(&CLIENT_REQUEST_ID),
            Some("caller-id")
        );
        let call = calls_for_mock.fetch_add(1, Ordering::SeqCst);
        async move {
            Ok(match call {
                0 => operation_response(OPERATION_LOCATION, false),
                1 => monitor_response(br#"{"status":"Succeeded"}"#),
                2 => monitor_response(br#"{"id":"finished"}"#),
                _ => panic!("unexpected request"),
            })
        }
        .boxed()
    });
    let context = Context::new().with_value(ClientRequestId::new("caller-id"));
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(Arc::new(mock)),
        Request::new(RESOURCE_URL.parse().unwrap(), Method::Put),
        None,
        Some(PollerOptions {
            context: context.to_borrowed(),
            ..Default::default()
        }),
    );
    drop(context);
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn polling_and_final_requests_do_not_replay_initial_body() {
    const BODY: &str = r#"{"location":"westus","properties":{}}"#;

    for method in [Method::Put, Method::Patch] {
        for header in [
            None,
            Some(AZURE_ASYNC_OPERATION),
            Some(OPERATION_LOCATION),
            Some(LOCATION),
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            let calls_for_mock = calls.clone();
            let header_for_mock = header.clone();
            let mock = MockHttpClient::new(move |request| {
                let call = calls_for_mock.fetch_add(1, Ordering::SeqCst);
                assert_eq!(
                    request.headers().get_optional_str(&ACCEPT),
                    Some("application/json")
                );
                if call == 0 {
                    assert_eq!(request.method(), method);
                    assert_eq!(Bytes::from(request.body()), BODY);
                    assert_eq!(
                        request.headers().get_optional_str(&CONTENT_TYPE),
                        Some("application/json")
                    );
                    assert_eq!(
                        request.headers().get_optional_string(&CONTENT_LENGTH),
                        Some(BODY.len().to_string())
                    );
                } else {
                    assert_eq!(request.method(), Method::Get);
                    assert_eq!(request.body().is_empty(), Some(true));
                    assert_eq!(request.headers().get_optional_str(&CONTENT_TYPE), None);
                    assert_eq!(request.headers().get_optional_str(&CONTENT_LENGTH), None);
                }
                assert_eq!(
                    request.url().as_str(),
                    if call == 0 || call == 3 || header_for_mock.is_none() {
                        RESOURCE_URL
                    } else {
                        POLLING_URL
                    }
                );
                let response = match call {
                    0 => match &header_for_mock {
                        Some(header) => operation_response(header.clone(), false),
                        None => monitor_response(
                            br#"{"properties":{"provisioningState":"RegisteringDns"}}"#,
                        ),
                    },
                    1 => match &header_for_mock {
                        Some(header) if header == &LOCATION => {
                            operation_response(LOCATION, false)
                        }
                        Some(_) => monitor_response(br#"{"status":"InProgress"}"#),
                        None => monitor_response(
                            br#"{"properties":{"provisioningState":"RegisteringDns"}}"#,
                        ),
                    },
                    2 => monitor_response(
                        br#"{"status":"Succeeded","properties":{"provisioningState":"Succeeded"},"id":"finished"}"#,
                    ),
                    3 => monitor_response(br#"{"id":"finished"}"#),
                    _ => panic!("unexpected request"),
                };
                async move { Ok(response) }.boxed()
            });
            let mut request = Request::new(RESOURCE_URL.parse().unwrap(), method);
            request.set_body(BODY);
            request.insert_header(ACCEPT, "application/json");
            request.insert_header(CONTENT_TYPE, "application/json");
            request.insert_header(CONTENT_LENGTH, BODY.len().to_string());
            let poller: Poller<ArmOperationStatus> =
                new_poller(pipeline_with(Arc::new(mock)), request, None, None);
            assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
            let expected_calls =
                if header == Some(AZURE_ASYNC_OPERATION) || header == Some(OPERATION_LOCATION) {
                    4
                } else {
                    3
                };
            assert_eq!(calls.load(Ordering::SeqCst), expected_calls);
        }
    }
}

#[tokio::test]
async fn final_request_preserves_original_accept_header() {
    for accept in [
        None,
        Some("application/json"),
        Some("application/xml"),
        Some("application/custom+json, application/json;q=0.5"),
    ] {
        for header in [AZURE_ASYNC_OPERATION, OPERATION_LOCATION] {
            for method in [Method::Post, Method::Put] {
                let calls = Arc::new(AtomicUsize::new(0));
                let calls_for_mock = calls.clone();
                let header = header.clone();
                let mock = MockHttpClient::new(move |request| {
                    assert_eq!(request.headers().get_optional_str(&ACCEPT), accept);
                    let call = calls_for_mock.fetch_add(1, Ordering::SeqCst);
                    let response = match call {
                        0 => {
                            assert_eq!(request.method(), method);
                            assert_eq!(request.url().as_str(), RESOURCE_URL);
                            operation_response(header.clone(), true)
                        }
                        1 => {
                            assert_eq!(request.method(), Method::Get);
                            assert_eq!(request.url().as_str(), POLLING_URL);
                            monitor_response(br#"{"status":"Succeeded"}"#)
                        }
                        2 => {
                            assert_eq!(request.method(), Method::Get);
                            assert_eq!(
                                request.url().as_str(),
                                if method == Method::Put {
                                    RESOURCE_URL
                                } else {
                                    LOCATION_URL
                                }
                            );
                            monitor_response(br#"{"id":"finished"}"#)
                        }
                        _ => panic!("unexpected request"),
                    };
                    async move { Ok(response) }.boxed()
                });
                let mut request = Request::new(RESOURCE_URL.parse().unwrap(), method);
                if let Some(accept) = accept {
                    request.insert_header(ACCEPT, accept);
                }
                let poller: Poller<ArmOperationStatus> =
                    new_poller(pipeline_with(Arc::new(mock)), request, None, None);
                assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
                assert_eq!(calls.load(Ordering::SeqCst), 3);
            }
        }
    }
}

#[cfg(feature = "xml")]
#[tokio::test]
async fn final_request_accepts_xml_resource() {
    #[derive(SafeDebug, serde::Deserialize)]
    struct Monitor {
        status: PollerStatus,
    }

    impl StatusMonitor for Monitor {
        type Output = ArmResource;
        type Format = crate::http::XmlFormat;

        fn status(&self) -> PollerStatus {
            self.status.clone()
        }
    }

    let calls = Arc::new(AtomicUsize::new(0));
    let calls_for_mock = calls.clone();
    let mock = MockHttpClient::new(move |request| {
        assert_eq!(
            request.headers().get_optional_str(&ACCEPT),
            Some("application/xml")
        );
        let call = calls_for_mock.fetch_add(1, Ordering::SeqCst);
        let response = match call {
            0 => operation_response(OPERATION_LOCATION, false),
            1 => monitor_response(br#"{"status":"Succeeded"}"#),
            2 => {
                assert_eq!(request.method(), Method::Get);
                assert_eq!(request.url().as_str(), RESOURCE_URL);
                AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    "<resource><id>finished</id></resource>",
                )
            }
            _ => panic!("unexpected request"),
        };
        async move { Ok(response) }.boxed()
    });
    let mut request = Request::new(RESOURCE_URL.parse().unwrap(), Method::Put);
    request.insert_header(ACCEPT, "application/xml");
    let poller: Poller<Monitor> = new_poller(pipeline_with(Arc::new(mock)), request, None, None);
    assert_eq!(poller.await.unwrap().into_model().unwrap().id, "finished");
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}
