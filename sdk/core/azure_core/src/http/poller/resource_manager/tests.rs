use super::{new_poller, AZURE_ASYNC_OPERATION, LOCATION, OPERATION_LOCATION};
use crate::{
    http::poller::{PollerStatus, StatusMonitor},
    http::{
        headers::{Headers, RETRY_AFTER_MS},
        AsyncRawResponse, ClientOptions, HttpClient, JsonFormat, Method, Pipeline, Poller, Request,
        StatusCode, Transport,
    },
};
use azure_core_test::http::MockHttpClient;
use futures::FutureExt as _;
use std::sync::{Arc, Mutex};
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
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-1");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/1?api-version=2024-01-01",
            "https://example.com/operations/1",
            "https://example.com/resources/1",
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
                    br#"{"properties":{"provisioningState":"Succeeded"}}"#.to_vec(),
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
            "https://example.com/resources/2".parse().unwrap(),
            Method::Put,
        ),
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-2");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/2",
            "https://example.com/resources/2",
            "https://example.com/resources/2",
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
                        br#"{"status":"Succeeded"}"#.to_vec(),
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
            "https://example.com/resources/3".parse().unwrap(),
            Method::Delete,
        ),
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-3");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/3",
            "https://example.com/operations/3",
            "https://example.com/resources/3",
        ]
    );
}

/// When the initial response has `azure-asyncoperation` but no `location` header, the final GET
/// uses the original resource URL.
#[tokio::test]
async fn new_poller_async_pattern_no_location_uses_resource_url() {
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
                    // No Location header: final GET should go to the original resource URL
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
            "https://example.com/resources/5".parse().unwrap(),
            Method::Post,
        ),
        None,
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-5");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/5",
            "https://example.com/operations/5",
            "https://example.com/resources/5",
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
            "https://example.com/resources/6".parse().unwrap(),
            Method::Put,
        ),
        None,
    );

    // The poller should complete after the initial request (synchronous completion) and then
    // fetch the final resource URL again.
    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-6");

    // Exactly two requests: the initial PUT and one final GET to the resource URL.
    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/6",
            "https://example.com/resources/6",
        ]
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
            "https://example.com/resources/7".parse().unwrap(),
            Method::Delete,
        ),
        None,
    );

    let err = poller.await.unwrap_err();
    assert_eq!(
        err.to_string(),
        "resource manager long-running operation failed"
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
            "https://example.com/resources/7".parse().unwrap(),
            Method::Delete,
        ),
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
            "https://example.com/resources/8".parse().unwrap(),
            Method::Patch,
        ),
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
            "https://example.com/resources/8".parse().unwrap(),
            Method::Patch,
        ),
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
            "https://example.com/resources/9".parse().unwrap(),
            Method::Put,
        ),
        None,
    );

    // 204 responses have no body; the poller should complete successfully.
    let _ = poller.await.unwrap();

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/9",
            "https://example.com/resources/9",
            "https://example.com/resources/9",
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
            "https://example.com/resources/10".parse().unwrap(),
            Method::Delete,
        ),
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
            "https://example.com/resources/10".parse().unwrap(),
            Method::Delete,
        ),
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
            "https://example.com/resources/11".parse().unwrap(),
            Method::Delete,
        ),
        None,
    );

    // For a DELETE with location, 204 means success; the final GET also returns 204.
    let _ = poller.await.unwrap();

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/resources/11",
            "https://example.com/operations/11",
            "https://example.com/operations/11",
        ]
    );
}
