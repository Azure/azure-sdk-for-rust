// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_container_ref_free, cosmos_driver_resolve_container_blocking, ContainerRefHandle,
};
use crate::{
    completion::{
        cosmos_completion_queue_create, cosmos_completion_queue_free,
        cosmos_completion_queue_free_completions, cosmos_completion_queue_shutdown,
        cosmos_completion_queue_state, cosmos_completion_queue_wait,
        cosmos_completion_take_container, cosmos_operation_handle_free,
        cosmos_operation_handle_state, CompletionQueue, CosmosCompletion, CosmosCompletionOutcome,
        CosmosCompletionQueueState, CosmosOperationHandleState,
    },
    driver::{cosmos_driver_free, DriverHandle},
    error::{CosmosErrorCode, COSMOS_STATUS_SUCCESS},
    op_request::{build_request, CosmosOperationKind, CosmosOperationRequest},
    partition_key::{
        cosmos_partition_key_create, cosmos_partition_key_free, CosmosPartitionKeyComponent,
        CosmosPartitionKeyComponentKind, CosmosPartitionKeyComponentValue,
    },
    runtime::{cosmos_runtime_free, RuntimeContext},
    string::{view, CosmosStringView},
    submit::{
        cosmos_driver_resolve_container_submit, cosmos_submit_operation,
        cosmos_submit_singleton_operation,
    },
};
use async_trait::async_trait;
use azure_core::http::headers::Headers;
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntime,
    models::{AccountReference, PartitionKey, PartitionKeyVersion},
    options::DriverOptions,
    test::{
        ConnectionPoolOptions, HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse,
        TransportClient, TransportError,
    },
};
use std::{
    mem::MaybeUninit,
    ptr,
    sync::{Arc, Mutex},
    time::Duration,
};
use url::Url;

const DATABASE: &str = "données";
const CONTAINER: &str = "容器";
const CONTAINER_PATH: &str = "/dbs/donn%C3%A9es/colls/%E5%AE%B9%E5%99%A8";

#[derive(Debug, Default)]
struct MetadataTransport {
    paths: Mutex<Vec<String>>,
}

#[async_trait]
impl TransportClient for MetadataTransport {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let path = request.url.path();
        self.paths.lock().unwrap().push(path.to_owned());
        let body = match path {
            "/" => serde_json::json!({
                "_self": "",
                "id": "counted-strings",
                "_rid": "counted-strings",
                "writableLocations": [{
                    "name": "West US 2",
                    "databaseAccountEndpoint": "https://counted-strings.invalid/"
                }],
                "readableLocations": [{
                    "name": "West US 2",
                    "databaseAccountEndpoint": "https://counted-strings.invalid/"
                }],
                "enableMultipleWriteLocations": false,
                "userConsistencyPolicy": {"defaultConsistencyLevel": "Session"}
            }),
            CONTAINER_PATH => serde_json::json!({
                "id": CONTAINER,
                "_rid": "AQIDBAUGBwg=",
                "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 1}
            }),
            _ => panic!("unexpected downstream request: {path}"),
        };
        Ok(HttpResponse {
            status: 200,
            headers: Headers::new(),
            body: serde_json::to_vec(&body).unwrap(),
        })
    }
}

#[derive(Debug)]
struct MetadataFactory(Arc<MetadataTransport>);

impl HttpClientFactory for MetadataFactory {
    fn build(
        &self,
        _: &ConnectionPoolOptions,
        _: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        Ok(self.0.clone())
    }
}

struct Fixture {
    runtime: *mut RuntimeContext,
    driver: *mut DriverHandle,
    queue: *mut CompletionQueue,
    transport: Arc<MetadataTransport>,
}

impl Fixture {
    fn new() -> Self {
        // A current-thread runtime cannot poll submitted work until explicitly driven.
        let tokio = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let transport = Arc::new(MetadataTransport::default());
        let runtime = tokio
            .block_on(
                CosmosDriverRuntime::builder()
                    .with_mock_http_client_factory(Arc::new(MetadataFactory(transport.clone())))
                    .build(),
            )
            .unwrap();
        let account = AccountReference::with_master_key(
            Url::parse("https://counted-strings.invalid/").unwrap(),
            "dGVzdA==",
        );
        let driver = tokio
            .block_on(runtime.create_driver(DriverOptions::builder(account).build()))
            .unwrap();
        let runtime = Arc::into_raw(Arc::new(RuntimeContext {
            tokio,
            driver: runtime,
        })) as *mut RuntimeContext;
        let driver = DriverHandle::from_arc_into_raw(Arc::new(DriverHandle { inner: driver }));
        let queue = cosmos_completion_queue_create(runtime, ptr::null());
        assert!(!queue.is_null());
        transport.paths.lock().unwrap().clear();
        Self {
            runtime,
            driver,
            queue,
            transport,
        }
    }

    fn completion(&self) -> CosmosCompletion {
        let runtime = RuntimeContext::inner_arc(self.runtime).unwrap();
        runtime.tokio.block_on(async {
            tokio::time::timeout(Duration::from_secs(5), async {
                loop {
                    let mut slot = MaybeUninit::<CosmosCompletion>::uninit();
                    if cosmos_completion_queue_wait(self.queue, slot.as_mut_ptr(), 1, 0) == 1 {
                        // SAFETY: the queue wrote exactly one initialized completion.
                        return unsafe { slot.assume_init() };
                    }
                    tokio::task::yield_now().await;
                }
            })
            .await
            .expect("resolution must deliver a completion")
        })
    }

    fn assert_empty_queue(&self) {
        let mut slot = MaybeUninit::<CosmosCompletion>::uninit();
        assert_eq!(
            cosmos_completion_queue_wait(self.queue, slot.as_mut_ptr(), 1, 0),
            0
        );
        cosmos_completion_queue_shutdown(self.queue);
        assert_eq!(
            cosmos_completion_queue_state(self.queue),
            CosmosCompletionQueueState::CosmosCompletionQueueStateDrained
        );
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        cosmos_completion_queue_free(self.queue);
        cosmos_driver_free(self.driver);
        cosmos_runtime_free(self.runtime);
    }
}

#[test]
fn public_resolution_rejects_malformed_counted_identifiers_without_io() {
    let fixture = Fixture::new();
    let cases = [
        (
            view(b"name\0suffix"),
            CosmosErrorCode::CosmosErrorCodeInvalidOptionValue,
        ),
        (
            view(b"name\0\xff"),
            CosmosErrorCode::CosmosErrorCodeInvalidUtf8,
        ),
        (
            CosmosStringView {
                data: ptr::null(),
                len: 1,
            },
            CosmosErrorCode::CosmosErrorCodeInvalidArgument,
        ),
    ];
    for (malformed, error) in cases {
        for (database, container) in [
            (malformed, view(CONTAINER.as_bytes())),
            (view(DATABASE.as_bytes()), malformed),
        ] {
            let mut out = ptr::null_mut();
            let mut rich_error = ptr::null_mut();
            assert_eq!(
                cosmos_driver_resolve_container_blocking(
                    fixture.runtime,
                    fixture.driver,
                    database,
                    container,
                    &mut out,
                    &mut rich_error,
                ),
                error.as_status_code()
            );
            assert!(out.is_null());
            assert!(rich_error.is_null());
            let mut pre_error = COSMOS_STATUS_SUCCESS;
            let operation = cosmos_driver_resolve_container_submit(
                fixture.driver,
                database,
                container,
                fixture.queue,
                42,
                &mut pre_error,
            );
            assert!(operation.is_null());
            assert_eq!(pre_error, error.as_status_code());
            assert!(fixture.transport.paths.lock().unwrap().is_empty());
        }
    }
    fixture.assert_empty_queue();
}

#[test]
fn public_async_resolution_owns_identifiers_before_scheduling() {
    let fixture = Fixture::new();
    let mut database = DATABASE.as_bytes().to_vec();
    let mut container = CONTAINER.as_bytes().to_vec();
    let mut error = COSMOS_STATUS_SUCCESS;
    let operation = cosmos_driver_resolve_container_submit(
        fixture.driver,
        view(&database),
        view(&container),
        fixture.queue,
        42,
        &mut error,
    );
    assert!(!operation.is_null());
    assert_eq!(error, COSMOS_STATUS_SUCCESS);
    database.fill(b'x');
    container.fill(b'y');
    drop(database);
    drop(container);
    assert!(fixture.transport.paths.lock().unwrap().is_empty());
    assert_eq!(
        cosmos_operation_handle_state(operation),
        CosmosOperationHandleState::CosmosOperationHandleStateInFlight
    );

    let mut completion = fixture.completion();
    assert_eq!(
        completion.outcome,
        CosmosCompletionOutcome::CosmosCompletionOutcomeOk
    );
    assert_eq!(completion.status, COSMOS_STATUS_SUCCESS);
    assert_eq!(completion.user_data, 42);
    assert_eq!(completion.was_cancel_requested, 0);
    assert_eq!(
        *fixture.transport.paths.lock().unwrap(),
        vec![CONTAINER_PATH.to_owned()]
    );
    assert_eq!(
        cosmos_operation_handle_state(operation),
        CosmosOperationHandleState::CosmosOperationHandleStateCompleted
    );
    let resolved = cosmos_completion_take_container(&mut completion);
    assert!(!resolved.is_null());
    assert!(cosmos_completion_take_container(&mut completion).is_null());
    cosmos_completion_queue_free_completions(&mut completion, 1);
    let reference = ContainerRefHandle::from_ptr(resolved).unwrap();
    assert_eq!(reference.inner.database_name(), Some(DATABASE));
    assert_eq!(reference.inner.name(), CONTAINER);
    cosmos_container_ref_free(resolved);
    cosmos_operation_handle_free(operation);
    fixture.assert_empty_queue();
}

#[test]
fn native_v1_request_preserves_nul_and_protocol_epk() {
    let fixture = Fixture::new();
    let mut container = ptr::null_mut();
    assert_eq!(
        cosmos_driver_resolve_container_blocking(
            fixture.runtime,
            fixture.driver,
            view(DATABASE.as_bytes()),
            view(CONTAINER.as_bytes()),
            &mut container,
            ptr::null_mut(),
        ),
        COSMOS_STATUS_SUCCESS
    );
    assert_eq!(
        ContainerRefHandle::from_ptr(container)
            .unwrap()
            .inner
            .partition_key_definition()
            .version(),
        PartitionKeyVersion::V1
    );
    let text = format!("tenant\0{}é", "a".repeat(92));
    let component = CosmosPartitionKeyComponent {
        kind: CosmosPartitionKeyComponentKind::STRING.0,
        value: CosmosPartitionKeyComponentValue {
            string_value: view(text.as_bytes()),
        },
    };
    let mut key = ptr::null_mut();
    let mut error = cosmos_partition_key_create(&component, 1, &mut key);
    assert!(!key.is_null());
    assert_eq!(error, COSMOS_STATUS_SUCCESS);
    // SAFETY: this C request contains only integers and pointers; all-zero is valid.
    let mut request: CosmosOperationRequest = unsafe { std::mem::zeroed() };
    request.kind = CosmosOperationKind::CosmosOperationKindReadItem as i32;
    request.container = container;
    request.item_id = view(b"item");
    request.partition_key = key;
    request.max_item_count = -1;
    cosmos_completion_queue_shutdown(fixture.queue);
    for inline in [false, true] {
        if inline {
            request.partition_key_components = &component;
            request.partition_key_len = 1;
        }
        // SAFETY: all handles and counted buffers remain live throughout construction.
        let built = unsafe { build_request(&request) }.unwrap();
        assert_eq!(
            built.operation.partition_key(),
            Some(&PartitionKey::from(text.clone()))
        );
        let target = built.operation.target().unwrap();
        assert_eq!(
            target.min_inclusive().to_hex(),
            "05C1C7CDE1BBB00875666F626F75016262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262626262C4AA"
        );
        assert_eq!(
            target.min_inclusive().to_hex(),
            target.max_exclusive().to_hex()
        );
        for submit in [cosmos_submit_operation, cosmos_submit_singleton_operation] {
            let operation = submit(fixture.driver, &request, fixture.queue, 0, &mut error);
            assert!(operation.is_null());
            assert_eq!(
                error,
                CosmosErrorCode::CosmosErrorCodeQueueShutdown.as_status_code()
            );
        }
    }
    cosmos_partition_key_free(key);
    cosmos_container_ref_free(container);
    fixture.assert_empty_queue();
}
