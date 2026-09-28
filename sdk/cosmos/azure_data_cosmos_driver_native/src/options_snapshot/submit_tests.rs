// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_operation_options_snapshot_create, cosmos_operation_options_snapshot_free,
    OperationOptionsSnapshot,
};
use crate::{
    account_ref::{cosmos_account_ref_free, AccountRefHandle},
    completion::{
        cosmos_completion_queue_create, cosmos_completion_queue_free,
        cosmos_completion_queue_free_completions, cosmos_completion_queue_shutdown,
        cosmos_completion_queue_state, cosmos_completion_queue_wait, cosmos_operation_handle_free,
        CompletionQueue, CosmosCompletion, CosmosCompletionOutcome, CosmosCompletionQueueState,
        OperationHandle,
    },
    driver::{cosmos_driver_free, DriverHandle},
    error::{CosmosErrorCode, CosmosStatusCode},
    op_request::{
        cosmos_operation_options_default, CosmosHeaderKv, CosmosOperationKind,
        CosmosOperationOptions, CosmosOperationRequest,
    },
    runtime::{cosmos_runtime_free, RuntimeContext},
    string::view,
    submit::{cosmos_submit_operation, cosmos_submit_singleton_operation},
};
use async_trait::async_trait;
use azure_core::http::headers::{HeaderName, Headers};
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntimeBuilder,
    error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT,
    models::AccountReference,
    options::{ConnectionPoolOptions, DriverOptions},
    test::{
        HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse, TransportClient,
        TransportError,
    },
};
use std::{
    mem::MaybeUninit,
    ptr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Notify;

#[derive(Debug, Default)]
struct TransportState {
    requests: Mutex<Vec<HttpRequest>>,
    release: Notify,
}

#[derive(Clone, Debug)]
struct SnapshotTransport(Arc<TransportState>);

impl HttpClientFactory for SnapshotTransport {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(Arc::new(self.clone()))
    }
}

#[async_trait]
impl TransportClient for SnapshotTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let body = if request.url.path() == "/" {
            serde_json::to_vec(&serde_json::json!({
                "_self": "", "id": "test", "_rid": "test.documents.azure.com",
                "media": "//media/", "addresses": "//addresses/", "_dbs": "//dbs/",
                "writableLocations": [{"name": "East US", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com/"}],
                "readableLocations": [
                    {"name": "East US", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com/"},
                    {"name": "West US", "databaseAccountEndpoint": "https://test-westus.documents.azure.com/"}
                ],
                "enableMultipleWriteLocations": false,
                "userReplicationPolicy": {"minReplicaSetSize": 3, "maxReplicasetSize": 4},
                "userConsistencyPolicy": {"defaultConsistencyLevel": "Session"},
                "systemReplicationPolicy": {"minReplicaSetSize": 3, "maxReplicasetSize": 4},
                "readPolicy": {"primaryReadCoefficient": 1, "secondaryReadCoefficient": 1},
                "queryEngineConfiguration": "{}"
            })).unwrap()
        } else if request.url.path() == "/probe" {
            Vec::new()
        } else {
            self.0.requests.lock().unwrap().push(request.clone());
            self.0.release.notified().await;
            if request.url.path() == "/dbs" {
                br#"{"Databases":[],"_count":0}"#.to_vec()
            } else {
                b"{}".to_vec()
            }
        };
        Ok(HttpResponse {
            status: 200,
            headers: Headers::new(),
            body,
        })
    }
}

struct Fixture {
    runtime: *mut RuntimeContext,
    driver: *mut DriverHandle,
    account: *mut AccountRefHandle,
    queue: *mut CompletionQueue,
    transport: Arc<TransportState>,
}

impl Fixture {
    fn new() -> Self {
        let transport = Arc::new(TransportState::default());
        let builder = CosmosDriverRuntimeBuilder::new()
            .with_mock_http_client_factory(Arc::new(SnapshotTransport(Arc::clone(&transport))))
            .with_connection_pool(
                ConnectionPoolOptions::builder()
                    .with_is_http2_allowed(false)
                    .build()
                    .unwrap(),
            );
        let runtime = RuntimeContext::new_with_builder(builder)
            .ok()
            .expect("mock runtime builds");
        let inner = RuntimeContext::from_ptr(runtime).unwrap();
        let account = AccountReference::with_master_key(
            "https://test.documents.azure.com/".parse().unwrap(),
            "dGVzdA==",
        );
        let driver = inner
            .tokio
            .block_on(
                inner
                    .driver
                    .create_driver(DriverOptions::builder(account.clone()).build()),
            )
            .unwrap();
        Self {
            runtime,
            driver: DriverHandle::from_arc_into_raw(Arc::new(DriverHandle { inner: driver })),
            account: Box::into_raw(Box::new(AccountRefHandle { inner: account })),
            queue: cosmos_completion_queue_create(runtime, ptr::null()),
            transport,
        }
    }

    fn snapshot(&self, options: &CosmosOperationOptions) -> *mut OperationOptionsSnapshot {
        let mut snapshot = ptr::null_mut();
        let mut timeout = -2;
        // SAFETY: fixture runtime, input arrays and both output slots are live.
        let status = unsafe {
            cosmos_operation_options_snapshot_create(
                self.runtime,
                ptr::null(),
                options,
                &mut snapshot,
                &mut timeout,
            )
        };
        assert_eq!(
            status,
            CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code()
        );
        snapshot
    }

    fn request(
        &self,
        snapshot: *const OperationOptionsSnapshot,
        feed: bool,
    ) -> CosmosOperationRequest {
        // SAFETY: the flat request contains only integers, raw pointers and counted views.
        let mut request: CosmosOperationRequest = unsafe { std::mem::zeroed() };
        request.account = self.account;
        request.kind = if feed {
            CosmosOperationKind::CosmosOperationKindReadAllDatabases
        } else {
            CosmosOperationKind::CosmosOperationKindReadOffer
        } as i32;
        request.resource_link = view(b"offer");
        request.max_item_count = -1;
        request.options_snapshot = snapshot;
        request
    }

    fn completion(&self, wait_ms: u32) -> CosmosCompletion {
        let mut slot = MaybeUninit::uninit();
        assert_eq!(
            cosmos_completion_queue_wait(self.queue, slot.as_mut_ptr(), 1, wait_ms),
            1
        );
        // SAFETY: queue wait wrote one initialized completion.
        unsafe { slot.assume_init() }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        cosmos_completion_queue_free(self.queue);
        cosmos_driver_free(self.driver);
        cosmos_account_ref_free(self.account);
        cosmos_runtime_free(self.runtime);
    }
}

type Submit = extern "C" fn(
    *const DriverHandle,
    *const CosmosOperationRequest,
    *mut CompletionQueue,
    isize,
    *mut CosmosStatusCode,
) -> *mut OperationHandle;

fn submitter(feed: bool) -> Submit {
    if feed {
        cosmos_submit_operation
    } else {
        cosmos_submit_singleton_operation
    }
}

#[test]
fn submit_copies_snapshot_and_owned_collections_before_host_free() {
    for feed in [false, true] {
        let fixture = Fixture::new();
        let snapshot = {
            let header_name = String::from("x-admission");
            let header_value = String::from("captured");
            let region = String::from("East US");
            let regions = [view(region.as_bytes())];
            let headers = [CosmosHeaderKv {
                name: view(header_name.as_bytes()),
                value: view(header_value.as_bytes()),
            }];
            let mut options = cosmos_operation_options_default();
            options.hedging_enabled = 1;
            options.excluded_regions = regions.as_ptr();
            options.excluded_regions_len = regions.len();
            options.custom_headers = headers.as_ptr();
            options.custom_headers_len = headers.len();
            fixture.snapshot(&options)
        };
        let request = fixture.request(snapshot, feed);
        let mut status = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let operation = submitter(feed)(fixture.driver, &request, fixture.queue, 42, &mut status);
        assert!(!operation.is_null());
        // SAFETY: submit must have cloned the snapshot before returning.
        unsafe {
            cosmos_operation_options_snapshot_free(snapshot);
        }
        fixture.transport.release.notify_one();
        let mut completion = fixture.completion(5000);
        assert_eq!(
            completion.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeOk
        );
        assert_eq!(completion.user_data, 42);
        let requests = fixture.transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].url.host_str(),
            Some("test-westus.documents.azure.com")
        );
        assert_eq!(
            requests[0]
                .headers
                .get_optional_str(&HeaderName::from_static("x-admission")),
            Some("captured")
        );
        drop(requests);
        cosmos_completion_queue_free_completions(&mut completion, 1);
        cosmos_operation_handle_free(operation);
    }
}

#[test]
fn submit_rejects_other_runtime_snapshot_before_queue_admission() {
    let fixture = Fixture::new();
    let other = Fixture::new();
    let snapshot = other.snapshot(&cosmos_operation_options_default());
    for feed in [false, true] {
        let request = fixture.request(snapshot, feed);
        let mut status = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        assert!(
            submitter(feed)(fixture.driver, &request, fixture.queue, 42, &mut status).is_null()
        );
        assert_eq!(
            status,
            CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
        );
    }
    cosmos_completion_queue_shutdown(fixture.queue);
    assert_eq!(
        cosmos_completion_queue_state(fixture.queue),
        CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
    );
    assert!(fixture.transport.requests.lock().unwrap().is_empty());
    // SAFETY: snapshot is owned by this test and neither rejected submit retained a borrow.
    unsafe {
        cosmos_operation_options_snapshot_free(snapshot);
    }
}

#[test]
fn submit_uses_remaining_admission_budget_instead_of_restarting_it() {
    for feed in [false, true] {
        let fixture = Fixture::new();
        let mut options = cosmos_operation_options_default();
        options.hedging_enabled = 1;
        options.end_to_end_timeout_ms = 1000;
        let snapshot = fixture.snapshot(&options);
        // SAFETY: test owns the live snapshot exclusively, simulating time spent initializing.
        unsafe {
            (*snapshot).started -= Duration::from_millis(990);
        }
        let request = fixture.request(snapshot, feed);
        let mut status = CosmosErrorCode::CosmosErrorCodeSuccess.as_status_code();
        let operation = submitter(feed)(fixture.driver, &request, fixture.queue, 42, &mut status);
        assert!(!operation.is_null());
        // SAFETY: submit copied the handle; the transport intentionally remains blocked.
        unsafe {
            cosmos_operation_options_snapshot_free(snapshot);
        }
        let mut completion = fixture.completion(500);
        assert_eq!(
            completion.outcome,
            CosmosCompletionOutcome::CosmosCompletionOutcomeError
        );
        assert_eq!(completion.status.0 >> 16, 408);
        assert_eq!(
            completion.status.0 & 0xffff,
            i32::from(CLIENT_OPERATION_TIMEOUT.value()),
        );
        cosmos_completion_queue_free_completions(&mut completion, 1);
        cosmos_operation_handle_free(operation);
    }
}
