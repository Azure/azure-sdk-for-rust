use super::new_poller;
use crate::{
    http::{
        headers::{HeaderName, Headers, RETRY_AFTER_MS},
        AsyncRawResponse, ClientOptions, Context, HttpClient, JsonFormat, Method, Pipeline, Poller,
        Request, StatusCode, Transport, Url,
    },
    http::poller::{PollerContinuation, PollerOptions, PollerState, PollerStatus, StatusMonitor},
    time::Duration,
};
use azure_core_test::http::MockHttpClient;
use futures::FutureExt as _;
use std::sync::{Arc, Mutex};

const AZURE_ASYNC_OPERATION: HeaderName = HeaderName::from_static("azure-asyncoperation");
const LOCATION: HeaderName = HeaderName::from_static("location");

#[derive(Debug, serde::Deserialize)]
struct ArmOperationStatus {
    #[serde(default)]
    status: Option<PollerStatus>,
    #[serde(default)]
    properties: Option<ArmOperationProperties>,
}

#[derive(Debug, serde::Deserialize)]
struct ArmOperationProperties {
    #[serde(default, rename = "provisioningState")]
    provisioning_state: Option<PollerStatus>,
}

#[derive(Debug, serde::Deserialize)]
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

#[tokio::test]
async fn new_poller_can_resume_from_state() {
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
                1 => AsyncRawResponse::from_bytes(
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

    let next_link: Url = "https://example.com/operations/4".parse().unwrap();
    let final_link: Url = "https://example.com/resources/4".parse().unwrap();
    let poller: Poller<ArmOperationStatus> = new_poller(
        pipeline_with(mock),
        Request::new(
            "https://example.com/resources/4".parse().unwrap(),
            Method::Put,
        ),
        Some(PollerOptions {
            state: PollerState::More(PollerContinuation::Links {
                next_link,
                final_link: Some(final_link),
            }),
            context: Context::new(),
            frequency: Duration::seconds(1),
        }),
    );

    let response = poller.await.unwrap();
    let model: ArmResource = response.into_model().unwrap();
    assert_eq!(model.id, "resource-4");

    let requests = requests.lock().unwrap().clone();
    assert_eq!(
        requests,
        vec![
            "https://example.com/operations/4",
            "https://example.com/resources/4",
        ]
    );
}
