// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_processor_checkpoint_submit, cosmos_processor_completion_free,
    cosmos_processor_completion_take_processor, cosmos_processor_completion_take_receipt,
    cosmos_processor_free, cosmos_processor_next_submit, cosmos_processor_queue_create,
    cosmos_processor_queue_free, cosmos_processor_queue_wait, cosmos_processor_receipt_free,
    cosmos_processor_request_init, cosmos_processor_start_submit, cosmos_processor_stop_submit,
    CosmosProcessorCompletion, CosmosProcessorRequest, ProcessorHandle, ProcessorQueue,
};
use crate::{
    completion::{
        cosmos_operation_handle_free, cosmos_operation_handle_state,
        cosmos_operation_handle_status, CosmosOperationHandleState, OperationHandle,
    },
    container_ref::{cosmos_container_ref_free, ContainerRefHandle},
    cursor::fixture,
    driver::{cosmos_driver_free, DriverHandle},
    error::{CosmosErrorCode, CosmosStatusCode, COSMOS_STATUS_SUCCESS},
    runtime::{cosmos_runtime_free, RuntimeContext},
    string::view,
};
use azure_data_cosmos_driver::{
    models::{CosmosOperation, DatabaseReference},
    options::OperationOptions,
};
use std::{mem::MaybeUninit, ptr, time::Duration};

struct Fixture {
    runtime: *mut RuntimeContext,
    driver: *mut DriverHandle,
    source: *mut ContainerRefHandle,
    leases: *mut ContainerRefHandle,
    queue: *mut ProcessorQueue,
}

impl Fixture {
    fn new() -> Self {
        let (runtime, driver) = fixture::create(true).unwrap();
        let rt = RuntimeContext::inner_arc(runtime).unwrap();
        let dr = DriverHandle::inner_arc(driver).unwrap();
        let (source, leases) = rt.tokio.block_on(async {
            dr.inner.execute_singleton_operation(
                CosmosOperation::create_container(DatabaseReference::from_name(dr.inner.account().clone(), "testdb"))
                    .with_body(serde_json::to_vec(&serde_json::json!({
                        "id":"leases","partitionKey":{"paths":["/id"],"kind":"Hash","version":2},"defaultTtl":-1,
                    })).unwrap()),
                OperationOptions::default(),
            ).await.unwrap();
            let source = dr.inner.resolve_container("testdb", "testcoll", OperationOptions::default()).await.unwrap();
            let leases = dr.inner.resolve_container("testdb", "leases", OperationOptions::default()).await.unwrap();
            (source, leases)
        });
        Self {
            runtime,
            driver,
            source: ContainerRefHandle::into_raw(source),
            leases: ContainerRefHandle::into_raw(leases),
            queue: cosmos_processor_queue_create(runtime, 1),
        }
    }

    fn request(&self) -> CosmosProcessorRequest {
        let mut request = MaybeUninit::uninit();
        cosmos_processor_request_init(request.as_mut_ptr());
        // SAFETY: initializer wrote the complete request.
        let mut request = unsafe { request.assume_init() };
        request.processor_name = view(b"processor");
        request.instance_name = view(b"worker");
        request.start_from = 1;
        request.max_item_count = 1;
        request
    }

    fn submit_start(
        &self,
        request: &CosmosProcessorRequest,
        error: *mut CosmosStatusCode,
    ) -> *mut OperationHandle {
        cosmos_processor_start_submit(
            self.driver,
            self.source,
            self.driver,
            self.leases,
            request,
            self.queue,
            1,
            error,
        )
    }

    fn drive(&self, operation: *mut OperationHandle) {
        assert!(!operation.is_null());
        RuntimeContext::inner_arc(self.runtime)
            .unwrap()
            .tokio
            .block_on(async {
                tokio::time::timeout(Duration::from_secs(10), async {
                    while cosmos_operation_handle_state(operation)
                        == CosmosOperationHandleState::CosmosOperationHandleStateInFlight
                    {
                        tokio::task::yield_now().await;
                    }
                })
                .await
                .expect("operation timeout");
            });
    }

    fn receive(&self) -> *mut CosmosProcessorCompletion {
        let mut result = ptr::null_mut();
        let mut count = 0;
        assert_eq!(
            cosmos_processor_queue_wait(self.queue, &mut result, 1, 0, &mut count),
            COSMOS_STATUS_SUCCESS
        );
        assert_eq!(count, 1);
        result
    }

    fn start(&self) -> *mut ProcessorHandle {
        let operation = self.submit_start(&self.request(), ptr::null_mut());
        self.drive(operation);
        let completion = self.receive();
        // SAFETY: completion is live until free below.
        assert_eq!(
            unsafe { (*completion).common.status },
            COSMOS_STATUS_SUCCESS
        );
        let processor = cosmos_processor_completion_take_processor(completion);
        assert!(!processor.is_null());
        assert!(cosmos_processor_completion_take_processor(completion).is_null());
        cosmos_processor_completion_free(completion);
        cosmos_operation_handle_free(operation);
        processor
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        cosmos_processor_queue_free(self.queue);
        cosmos_container_ref_free(self.source);
        cosmos_container_ref_free(self.leases);
        cosmos_driver_free(self.driver);
        cosmos_runtime_free(self.runtime);
    }
}

#[test]
fn callback_acknowledgment_bypasses_full_data_admission() {
    let fixture = Fixture::new();
    let processor = fixture.start();
    let first = cosmos_processor_next_submit(processor, 2, ptr::null_mut());
    fixture.drive(first);
    let batch = fixture.receive();
    // SAFETY: batch owns all referenced storage until free.
    let common = unsafe { &(*batch).common };
    assert_eq!(common.status, COSMOS_STATUS_SUCCESS);
    assert_eq!(unsafe { (*batch).result_kind }, 2);
    assert!(common.body_len > 0);
    // SAFETY: completion owns the readable body view.
    let original_body =
        unsafe { std::slice::from_raw_parts(common.body, common.body_len) }.to_vec();
    let receipt = cosmos_processor_completion_take_receipt(batch);
    assert!(!receipt.is_null());
    assert!(cosmos_processor_completion_take_receipt(batch).is_null());
    let next = cosmos_processor_next_submit(processor, 3, ptr::null_mut());
    let mut error = COSMOS_STATUS_SUCCESS;
    let duplicate_next = cosmos_processor_next_submit(processor, 9, &mut error);
    assert!(duplicate_next.is_null());
    assert_eq!(
        error,
        CosmosErrorCode::CosmosErrorCodeCursorBusy.as_status_code()
    );
    let checkpoint = cosmos_processor_checkpoint_submit(receipt, 4, &mut error);
    assert!(
        !checkpoint.is_null(),
        "control must not be blocked by a next reservation"
    );
    let duplicate = cosmos_processor_checkpoint_submit(receipt, 5, &mut error);
    assert!(duplicate.is_null());
    assert_eq!(
        error,
        CosmosErrorCode::CosmosErrorCodeInvalidArgument.as_status_code()
    );
    cosmos_processor_receipt_free(receipt);
    fixture.drive(checkpoint);
    fixture.drive(next);
    let results = [fixture.receive(), fixture.receive()];
    let mut kinds = results.map(|result| unsafe { (*result).result_kind });
    kinds.sort_unstable();
    assert_eq!(kinds, [2, 3]);
    // Keeping the first payload alive across advancement must preserve its bytes.
    let retained =
        unsafe { std::slice::from_raw_parts((*batch).common.body, (*batch).common.body_len) };
    assert_eq!(retained, original_body);
    for result in results {
        cosmos_processor_completion_free(result);
    }
    cosmos_processor_completion_free(batch);
    for operation in [first, next, checkpoint] {
        cosmos_operation_handle_free(operation);
    }
    let stop = cosmos_processor_stop_submit(processor, 6, ptr::null_mut());
    fixture.drive(stop);
    cosmos_processor_completion_free(fixture.receive());
    cosmos_operation_handle_free(stop);
    cosmos_processor_free(processor);
}

#[test]
fn stale_receipt_cannot_checkpoint_after_shutdown() {
    let fixture = Fixture::new();
    let processor = fixture.start();
    let operation = cosmos_processor_next_submit(processor, 2, ptr::null_mut());
    fixture.drive(operation);
    let batch = fixture.receive();
    let receipt = cosmos_processor_completion_take_receipt(batch);
    let stop = cosmos_processor_stop_submit(processor, 3, ptr::null_mut());
    fixture.drive(stop);
    cosmos_processor_completion_free(fixture.receive());
    let stop_again = cosmos_processor_stop_submit(processor, 5, ptr::null_mut());
    fixture.drive(stop_again);
    let stopped = fixture.receive();
    assert_eq!(unsafe { (*stopped).result_kind }, 4);
    cosmos_processor_completion_free(stopped);
    let ack = cosmos_processor_checkpoint_submit(receipt, 4, ptr::null_mut());
    fixture.drive(ack);
    let failed = fixture.receive();
    assert_ne!(unsafe { (*failed).common.status }, COSMOS_STATUS_SUCCESS);
    cosmos_processor_receipt_free(receipt);
    cosmos_processor_completion_free(batch);
    cosmos_processor_completion_free(failed);
    for operation in [operation, stop, stop_again, ack] {
        cosmos_operation_handle_free(operation);
    }
    cosmos_processor_free(processor);
}

#[test]
fn queue_abandonment_finishes_outstanding_next() {
    let mut fixture = Fixture::new();
    let mut request = fixture.request();
    request.start_from = 2;
    let start = fixture.submit_start(&request, ptr::null_mut());
    fixture.drive(start);
    let opened = fixture.receive();
    let processor = cosmos_processor_completion_take_processor(opened);
    cosmos_processor_completion_free(opened);
    let next = cosmos_processor_next_submit(processor, 2, ptr::null_mut());
    cosmos_processor_queue_free(fixture.queue);
    fixture.queue = ptr::null_mut();
    fixture.drive(next);
    assert_eq!(
        cosmos_operation_handle_status(next),
        CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code()
    );
    cosmos_operation_handle_free(start);
    cosmos_operation_handle_free(next);
    cosmos_processor_free(processor);
}

#[test]
fn malformed_request_does_not_reserve_delivery_capacity() {
    let fixture = Fixture::new();
    let mut request = fixture.request();
    request.abi_version = 2;
    let mut error = COSMOS_STATUS_SUCCESS;
    assert!(fixture.submit_start(&request, &mut error).is_null());
    assert_eq!(
        error,
        CosmosErrorCode::CosmosErrorCodeInvalidOptionValue.as_status_code()
    );
    let processor = fixture.start();
    cosmos_processor_free(processor);
}
