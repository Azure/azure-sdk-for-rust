// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_cursor_checkpoint_submit, cosmos_cursor_completion_free,
    cosmos_cursor_completion_take_cursor, cosmos_cursor_free, cosmos_cursor_next_submit,
    cosmos_cursor_open_submit, cosmos_cursor_queue_create, cosmos_cursor_queue_wait,
    cosmos_cursor_status, fixture, CosmosCursorCompletion, CursorHandle,
};
use crate::{
    completion::{
        cosmos_completion_queue_create, cosmos_completion_queue_free,
        cosmos_completion_queue_shutdown, cosmos_completion_queue_wait,
        cosmos_operation_handle_cancel, cosmos_operation_handle_free,
        cosmos_operation_handle_state, cosmos_operation_handle_status, CompletionQueue,
        CosmosCompletion, CosmosOperationHandleState, OperationHandle,
    },
    container_ref::{cosmos_container_ref_free, ContainerRefHandle},
    cursor_request::{build_cursor_request, cosmos_cursor_request_init, CosmosCursorRequest},
    driver::{cosmos_driver_free, DriverHandle},
    error::{CosmosErrorCode, COSMOS_STATUS_SUCCESS},
    op_request::cosmos_operation_options_default,
    partition_key::{cosmos_partition_key_free, PartitionKeyHandle},
    runtime::{cosmos_runtime_free, RuntimeContext},
    string::view,
};
use azure_data_cosmos_driver::options::OperationOptions;
use std::{mem::MaybeUninit, ptr, time::Duration};

struct Fixture {
    runtime: *mut RuntimeContext,
    driver: *mut DriverHandle,
    container: *mut ContainerRefHandle,
    queue: *mut CompletionQueue,
    partition_key: *mut PartitionKeyHandle,
}

impl Fixture {
    fn new(capacity: u32) -> Self {
        Self::from_handles(fixture::create(true).unwrap(), capacity)
    }

    fn scripted() -> Self {
        Self::from_handles(fixture::scripted(true), 2)
    }

    fn from_handles(
        (runtime, driver): (*mut RuntimeContext, *mut DriverHandle),
        capacity: u32,
    ) -> Self {
        let rt = RuntimeContext::inner_arc(runtime).unwrap();
        let dr = DriverHandle::inner_arc(driver).unwrap();
        let container = rt
            .tokio
            .block_on(
                dr.inner
                    .resolve_container("testdb", "testcoll", OperationOptions::default()),
            )
            .unwrap();
        Self {
            runtime,
            driver,
            container: ContainerRefHandle::into_raw(container),
            queue: cosmos_cursor_queue_create(runtime, capacity),
            partition_key: Box::into_raw(Box::new(PartitionKeyHandle {
                inner: azure_data_cosmos_driver::models::PartitionKey::from("pk-0"),
            })),
        }
    }
}

#[test]
fn non_streaming_order_by_retains_all_rows_after_unsupported_checkpoint() {
    for page_size in [1, 2] {
        let fixture = Fixture::scripted();
        let mut request = fixture.request();
        let body = br#"{"query":"SELECT TOP 6 * FROM c ORDER BY c.rank"}"#;
        request.operation.body = body.as_ptr();
        request.operation.body_len = body.len();
        request.operation.max_item_count = page_size;
        let mut options = cosmos_operation_options_default();
        options.query_plan_mode = 2;
        options.binary_encoding_enabled = 1;
        request.operation.options = &options;
        let cursor = fixture.open(&request);
        let mut ranks = Vec::new();
        let mut unsupported = false;
        loop {
            let page = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
            // SAFETY: page is a live completion.
            if unsafe { (*page).result_kind } == 4 {
                cosmos_cursor_completion_free(page);
                break;
            }
            ranks.extend(
                documents(page)
                    .iter()
                    .map(|doc| doc["rank"].as_i64().unwrap()),
            );
            cosmos_cursor_completion_free(page);
            let checkpoint =
                fixture.receive(cosmos_cursor_checkpoint_submit(cursor, 0, ptr::null_mut()));
            // SAFETY: checkpoint is a live completion.
            let status = unsafe { (*checkpoint).common.status };
            if status != COSMOS_STATUS_SUCCESS {
                assert_eq!(status.0 & 0xffff, 20125);
                unsupported = true;
            }
            cosmos_cursor_completion_free(checkpoint);
        }
        assert_eq!(ranks, vec![0, 1, 2, 3, 4, 5]);
        assert!(unsupported);
        cosmos_cursor_free(cursor);
    }
}

#[test]
fn change_feed_idle_resume_envelopes_and_service_rejection() {
    for mode in [1, 2] {
        let fixture = Fixture::scripted();
        let mut request = fixture.request();
        request.operation.kind = 0;
        request.change_feed_mode = mode;
        request.start_from = 2;
        let cursor = fixture.open(&request);
        let mut saw_idle = false;
        let mut saw_items = false;
        for _ in 0..6 {
            let page = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
            // SAFETY: completion is owned and live.
            unsafe {
                assert_eq!((*page).result_kind, 2);
                assert_eq!((*page).common.status, COSMOS_STATUS_SUCCESS);
                if (*page).common.http_status_code == 304 {
                    saw_idle = true;
                } else {
                    let docs = documents(page);
                    assert_eq!(docs.len(), 2);
                    if mode == 2 {
                        assert_eq!(docs[0]["previous"]["rank"], -1);
                        assert_eq!(docs[1]["metadata"]["lsn"], 2);
                    } else {
                        assert_eq!(docs[0]["Documents"][0], "application-field");
                    }
                    saw_items = true;
                }
            }
            cosmos_cursor_completion_free(page);
        }
        assert!(saw_idle && saw_items);
        let checkpoint =
            fixture.receive(cosmos_cursor_checkpoint_submit(cursor, 0, ptr::null_mut()));
        // SAFETY: token view remains live throughout the Open call.
        unsafe {
            request.operation.continuation_token = (*checkpoint).checkpoint;
        }
        request.start_from = 0;
        let resumed = fixture.open(&request);
        cosmos_cursor_completion_free(checkpoint);
        cosmos_cursor_free(cursor);
        let page = fixture.receive(cosmos_cursor_next_submit(resumed, 0, ptr::null_mut()));
        // SAFETY: completion is live.
        unsafe {
            assert_eq!((*page).common.http_status_code, 304);
            assert_eq!((*page).result_kind, 2);
        }
        cosmos_cursor_completion_free(page);
        cosmos_cursor_free(resumed);
    }
    let fixture = Fixture::scripted();
    let mut request = fixture.request();
    request.operation.kind = 0;
    request.change_feed_mode = 2;
    request.start_from = 1;
    let cursor = fixture.open(&request);
    let page = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
    // SAFETY: completion is live. Well-formed unsupported starts reach the transport.
    unsafe {
        assert_eq!((*page).common.http_status_code, 400);
        assert_eq!((*page).common.is_from_wire, 1);
    }
    cosmos_cursor_completion_free(page);
    cosmos_cursor_free(cursor);
}

impl Fixture {
    fn request(&self) -> CosmosCursorRequest {
        let mut request = MaybeUninit::uninit();
        cosmos_cursor_request_init(request.as_mut_ptr());
        // SAFETY: initializer wrote the complete record.
        let mut request = unsafe { request.assume_init() };
        request.operation.kind = 17;
        request.operation.container = self.container;
        request.operation.max_item_count = 1;
        request.operation.partition_key = self.partition_key;
        request
    }

    fn drive(&self, op: *mut OperationHandle) {
        assert!(!op.is_null());
        let rt = RuntimeContext::inner_arc(self.runtime).unwrap();
        rt.tokio.block_on(async {
            tokio::time::timeout(Duration::from_secs(10), async {
                while cosmos_operation_handle_state(op)
                    == CosmosOperationHandleState::CosmosOperationHandleStateInFlight
                {
                    tokio::task::yield_now().await;
                }
            })
            .await
            .unwrap();
        });
    }

    fn receive(&self, op: *mut OperationHandle) -> *mut CosmosCursorCompletion {
        self.drive(op);
        let mut completion = ptr::null_mut();
        let mut count = 0;
        assert_eq!(
            cosmos_cursor_queue_wait(self.queue, &mut completion, 1, 0, &mut count),
            COSMOS_STATUS_SUCCESS
        );
        assert_eq!(count, 1);
        cosmos_operation_handle_free(op);
        completion
    }

    fn open(&self, request: &CosmosCursorRequest) -> *mut CursorHandle {
        let mut error = COSMOS_STATUS_SUCCESS;
        let op = cosmos_cursor_open_submit(self.driver, request, self.queue, 123, &mut error);
        assert_eq!(error, COSMOS_STATUS_SUCCESS);
        let completion = self.receive(op);
        // SAFETY: completion is owned and live.
        unsafe {
            assert_eq!((*completion).common.status, COSMOS_STATUS_SUCCESS);
            assert_eq!((*completion).result_kind, 1);
        }
        let cursor = cosmos_cursor_completion_take_cursor(completion);
        assert!(!cursor.is_null());
        assert!(cosmos_cursor_completion_take_cursor(completion).is_null());
        cosmos_cursor_completion_free(completion);
        cursor
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        cosmos_completion_queue_free(self.queue);
        cosmos_container_ref_free(self.container);
        cosmos_partition_key_free(self.partition_key);
        cosmos_driver_free(self.driver);
        cosmos_runtime_free(self.runtime);
    }
}

fn documents(completion: *const CosmosCursorCompletion) -> Vec<serde_json::Value> {
    // SAFETY: all views are borrowed from a live completion.
    unsafe {
        let page = &*completion;
        assert_eq!(page.common.status, COSMOS_STATUS_SUCCESS);
        match page.body_kind {
            0 => Vec::new(),
            1 => {
                let json: serde_json::Value = serde_json::from_slice(std::slice::from_raw_parts(
                    page.common.body,
                    page.common.body_len,
                ))
                .unwrap();
                json["Documents"].as_array().unwrap().clone()
            }
            2 => {
                if page.items_len == 0 {
                    return Vec::new();
                }
                std::slice::from_raw_parts(page.items, page.items_len)
                    .iter()
                    .map(|item| {
                        serde_json::from_slice(std::slice::from_raw_parts(item.data, item.len))
                            .unwrap()
                    })
                    .collect()
            }
            kind => panic!("unexpected body kind {kind}"),
        }
    }
}

#[test]
fn populated_queries_page_checkpoint_and_wrapper_prefetch() {
    for page_size in [1, 2] {
        for query in [
            "SELECT * FROM c",
            "SELECT * FROM c ORDER BY c.rank",
            "SELECT DISTINCT VALUE c.group FROM c",
        ] {
            let fixture = Fixture::new(2);
            let mut request = fixture.request();
            let body = serde_json::to_vec(&serde_json::json!({"query": query})).unwrap();
            request.operation.body = body.as_ptr();
            request.operation.body_len = body.len();
            request.operation.max_item_count = page_size;
            let mut options = cosmos_operation_options_default();
            options.binary_encoding_request_text_response = 2;
            request.operation.options = &options;
            let cursor = fixture.open(&request);
            let mut values = Vec::new();
            let mut retained = Vec::new();
            let mut unsupported = false;
            loop {
                let op = cosmos_cursor_next_submit(cursor, 9, ptr::null_mut());
                let page = fixture.receive(op);
                // SAFETY: page is owned and live.
                let kind = unsafe { (*page).result_kind };
                if kind == 4 {
                    cosmos_cursor_completion_free(page);
                    break;
                }
                assert_eq!(kind, 2);
                values.extend(documents(page));
                retained.push(page);
                let checkpoint =
                    fixture.receive(cosmos_cursor_checkpoint_submit(cursor, 0, ptr::null_mut()));
                // SAFETY: checkpoint is owned and live.
                unsafe {
                    if (*checkpoint).common.status != COSMOS_STATUS_SUCCESS {
                        assert_eq!((*checkpoint).common.status.0 & 0xffff, 20124);
                        unsupported = true;
                    }
                }
                cosmos_cursor_completion_free(checkpoint);
            }
            for _ in 0..2 {
                let end = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
                // SAFETY: end is owned and live.
                unsafe {
                    assert_eq!((*end).result_kind, 4);
                }
                cosmos_cursor_completion_free(end);
            }
            cosmos_cursor_free(cursor);
            let reread: Vec<_> = retained.iter().flat_map(|page| documents(*page)).collect();
            assert_eq!(reread, values);
            for page in retained {
                cosmos_cursor_completion_free(page);
            }
            if query.contains("DISTINCT") {
                values.sort_by_key(|v| v.as_i64());
                assert_eq!(values, vec![0, 1, 2]);
                assert!(unsupported);
            } else {
                let mut ranks: Vec<_> =
                    values.iter().map(|v| v["rank"].as_i64().unwrap()).collect();
                if !query.contains("ORDER BY") {
                    ranks.sort();
                }
                assert_eq!(ranks, vec![0, 1, 2, 3, 4, 5]);
            }
        }
    }
}

#[test]
fn cross_partition_read_all_preserves_driver_rejection() {
    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 16;
    let completion = fixture.receive(cosmos_cursor_open_submit(
        fixture.driver,
        &request,
        fixture.queue,
        0,
        ptr::null_mut(),
    ));
    // SAFETY: completion is live and owned until freed below.
    unsafe {
        assert_eq!((*completion).common.status.0 >> 16, 400);
        assert_eq!((*completion).common.status.0 & 0xffff, 20112);
        assert!((*completion).cursor.is_null());
    }
    cosmos_cursor_completion_free(completion);
}

#[test]
fn admission_busy_cancel_shutdown_and_abandon_are_observable() {
    let mut fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 15;
    let cursor = fixture.open(&request);
    let op = cosmos_cursor_next_submit(cursor, 0, ptr::null_mut());
    let mut status = COSMOS_STATUS_SUCCESS;
    assert!(cosmos_cursor_next_submit(cursor, 0, &mut status).is_null());
    assert_eq!(
        status,
        CosmosErrorCode::CosmosErrorCodeCursorBusy.as_status_code()
    );
    assert!(
        cosmos_cursor_open_submit(fixture.driver, &request, fixture.queue, 0, &mut status)
            .is_null()
    );
    assert_eq!(
        status,
        CosmosErrorCode::CosmosErrorCodeQueueFull.as_status_code()
    );
    fixture.drive(op);
    assert!(cosmos_cursor_checkpoint_submit(cursor, 0, &mut status).is_null());
    assert_eq!(
        status,
        CosmosErrorCode::CosmosErrorCodeCursorBusy.as_status_code()
    );
    cosmos_completion_queue_free(fixture.queue);
    fixture.queue = ptr::null_mut();
    assert_eq!(
        cosmos_operation_handle_status(op),
        CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code()
    );
    assert_eq!(
        cosmos_cursor_status(cursor),
        CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code()
    );
    cosmos_operation_handle_free(op);
    cosmos_cursor_free(cursor);

    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 15;
    let cursor = fixture.open(&request);
    let op = cosmos_cursor_next_submit(cursor, 0, ptr::null_mut());
    cosmos_operation_handle_cancel(op);
    cosmos_completion_queue_shutdown(fixture.queue);
    let cancelled = fixture.receive(op);
    // SAFETY: completion is owned and live.
    unsafe {
        assert_eq!((*cancelled).common.outcome as i32, 2);
    }
    assert_eq!(
        cosmos_cursor_status(cursor),
        CosmosErrorCode::CosmosErrorCodeOperationCancelled.as_status_code()
    );
    cosmos_cursor_completion_free(cancelled);
    cosmos_cursor_free(cursor);
}

#[test]
fn late_cancel_does_not_retract_published_page_and_free_does_not_cancel() {
    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 15;
    let cursor = fixture.open(&request);
    let op = cosmos_cursor_next_submit(cursor, 7, ptr::null_mut());
    fixture.drive(op);
    cosmos_operation_handle_cancel(op);
    cosmos_cursor_free(cursor);
    let page = fixture.receive(op);
    // SAFETY: page is owned and live even after cursor free.
    unsafe {
        assert_eq!((*page).common.status, COSMOS_STATUS_SUCCESS);
        assert_eq!((*page).result_kind, 2);
        assert_eq!((*page).common.was_cancel_requested, 1);
    }
    cosmos_cursor_completion_free(page);
}

#[test]
fn versions_kinds_and_queue_format_fail_before_execution() {
    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    for version in [0, 2] {
        request.abi_version = version;
        // SAFETY: full initialized request.
        assert!(unsafe { build_cursor_request(&request) }.is_err());
    }
    request.abi_version = 1;
    request.struct_size_bytes = 8;
    // SAFETY: full allocation with deliberately short declared prefix.
    assert!(unsafe { build_cursor_request(&request) }.is_err());
    request.struct_size_bytes = std::mem::size_of::<CosmosCursorRequest>() as u32;
    for kind in [0, 1, 18, 99, -1] {
        request.operation.kind = kind;
        // SAFETY: full initialized request.
        assert!(unsafe { build_cursor_request(&request) }.is_err());
    }
    request.operation.kind = 15;
    let legacy = cosmos_completion_queue_create(fixture.runtime, ptr::null());
    let mut status = COSMOS_STATUS_SUCCESS;
    assert!(cosmos_cursor_open_submit(fixture.driver, &request, legacy, 0, &mut status).is_null());
    assert_eq!(
        status,
        CosmosErrorCode::CosmosErrorCodeQueueFormat.as_status_code()
    );
    let mut out = ptr::null_mut();
    let mut count = 99;
    assert_eq!(
        cosmos_cursor_queue_wait(legacy, &mut out, 1, 0, &mut count),
        status
    );
    assert_eq!(count, 0);
    cosmos_completion_queue_free(legacy);
    let cursor = fixture.open(&request);
    let op = cosmos_cursor_next_submit(cursor, 0, ptr::null_mut());
    fixture.drive(op);
    let mut legacy_out = MaybeUninit::<CosmosCompletion>::uninit();
    assert_eq!(
        cosmos_completion_queue_wait(fixture.queue, legacy_out.as_mut_ptr(), 1, 0),
        0
    );
    let page = fixture.receive(op);
    cosmos_cursor_completion_free(page);
    cosmos_cursor_free(cursor);
}

#[test]
fn change_feed_start_validation_preserves_well_formed_avad_starts() {
    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 0;
    request.change_feed_mode = 2;
    // SAFETY: full initialized request; missing fresh start is invalid.
    assert!(unsafe { build_cursor_request(&request) }.is_err());
    for start in [1, 2] {
        request.start_from = start;
        // SAFETY: full initialized request.
        assert!(unsafe { build_cursor_request(&request) }.is_ok());
    }
    request.start_from = 3;
    request.start_time = view(b"2026-09-18T12:00:00Z");
    // SAFETY: full initialized request with valid UTF-8.
    assert!(unsafe { build_cursor_request(&request) }.is_ok());
    request.start_time = view(b"2026-13-99T25:00:00Z");
    // SAFETY: full initialized request; malformed date is rejected.
    assert!(unsafe { build_cursor_request(&request) }.is_err());
    request.start_from = 2;
    request.start_time = view(b"2026-09-18T12:00:00Z");
    // SAFETY: full initialized request; time with Now is rejected.
    assert!(unsafe { build_cursor_request(&request) }.is_err());
}

#[test]
fn concurrent_admission_has_one_winner_and_preserves_capacity() {
    let fixture = Fixture::new(1);
    let mut request = fixture.request();
    request.operation.kind = 15;
    let cursor = fixture.open(&request);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let workers: Vec<_> = [false, true]
        .into_iter()
        .map(|checkpoint| {
            let barrier = barrier.clone();
            let cursor = cursor as usize;
            std::thread::spawn(move || {
                barrier.wait();
                let mut status = COSMOS_STATUS_SUCCESS;
                let op = if checkpoint {
                    cosmos_cursor_checkpoint_submit(cursor as *const CursorHandle, 0, &mut status)
                } else {
                    cosmos_cursor_next_submit(cursor as *const CursorHandle, 0, &mut status)
                };
                (op as usize, status)
            })
        })
        .collect();
    barrier.wait();
    let outcomes: Vec<_> = workers
        .into_iter()
        .map(|worker| worker.join().unwrap())
        .collect();
    assert_eq!(outcomes.iter().filter(|(op, _)| *op != 0).count(), 1);
    assert_eq!(
        outcomes.iter().find(|(op, _)| *op == 0).unwrap().1,
        CosmosErrorCode::CosmosErrorCodeCursorBusy.as_status_code()
    );
    let op = outcomes.iter().find(|(op, _)| *op != 0).unwrap().0 as *mut OperationHandle;
    let completion = fixture.receive(op);
    cosmos_cursor_completion_free(completion);
    let next = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
    cosmos_cursor_completion_free(next);
    cosmos_cursor_free(cursor);
}

#[test]
fn binary_items_survive_cursor_release() {
    let fixture = Fixture::new(2);
    let mut request = fixture.request();
    let body = br#"{"query":"SELECT * FROM c ORDER BY c.rank"}"#;
    request.operation.body = body.as_ptr();
    request.operation.body_len = body.len();
    request.operation.max_item_count = 2;
    let mut options = cosmos_operation_options_default();
    options.binary_encoding_enabled = 2;
    request.operation.options = &options;
    let cursor = fixture.open(&request);
    let mut pages = Vec::new();
    loop {
        let page = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
        // SAFETY: page is a live, owned completion.
        unsafe {
            assert_eq!((*page).common.status, COSMOS_STATUS_SUCCESS);
            if (*page).result_kind == 4 {
                cosmos_cursor_completion_free(page);
                break;
            }
            assert_eq!((*page).body_kind, 2);
            assert!((*page).items_len <= 2);
        }
        pages.push(page);
    }
    cosmos_cursor_free(cursor);
    let mut ranks = Vec::new();
    for page in pages {
        // SAFETY: each page owns its item views independently of the freed cursor.
        unsafe {
            if (*page).items_len > 0 {
                for item in std::slice::from_raw_parts((*page).items, (*page).items_len) {
                    let bytes = std::slice::from_raw_parts(item.data, item.len);
                    assert_eq!(bytes.first(), Some(&0x80));
                    let value: serde_json::Value =
                        azure_data_cosmos_driver::binary_json::from_slice(bytes).unwrap();
                    ranks.push(value["rank"].as_i64().unwrap());
                }
            }
        }
        cosmos_cursor_completion_free(page);
    }
    assert_eq!(ranks, vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn legacy_feeds_report_checkpoint_or_representation_errors() {
    for singleton in [false, true] {
        for (query, expected) in [
            ("SELECT DISTINCT VALUE c.group FROM c", 20124),
            (
                "SELECT * FROM c ORDER BY c.rank",
                CosmosErrorCode::CosmosErrorCodeRepresentationUnsupported
                    .as_status_code()
                    .0
                    & 0xffff,
            ),
        ] {
            let fixture = Fixture::new(1);
            let legacy = cosmos_completion_queue_create(fixture.runtime, ptr::null());
            let mut request = fixture.request();
            let body = serde_json::to_vec(&serde_json::json!({"query":query})).unwrap();
            request.operation.body = body.as_ptr();
            request.operation.body_len = body.len();
            let submit = if singleton {
                crate::submit::cosmos_submit_singleton_operation
            } else {
                crate::submit::cosmos_submit_operation
            };
            let op = submit(
                fixture.driver,
                &request.operation,
                legacy,
                0,
                ptr::null_mut(),
            );
            fixture.drive(op);
            let mut completion = MaybeUninit::<CosmosCompletion>::uninit();
            assert_eq!(
                cosmos_completion_queue_wait(legacy, completion.as_mut_ptr(), 1, 0),
                1
            );
            // SAFETY: exactly one completion was written above.
            let mut completion = unsafe { completion.assume_init() };
            assert_eq!(completion.status.0 & 0xffff, expected);
            assert_eq!(cosmos_operation_handle_status(op), completion.status);
            crate::completion::cosmos_completion_queue_free_completions(&mut completion, 1);
            cosmos_operation_handle_free(op);
            cosmos_completion_queue_free(legacy);
        }
    }
}

#[test]
fn change_feed_time_and_beginning_execute_and_mode_mismatch_is_rejected() {
    let fixture = Fixture::scripted();
    for start in [1, 3] {
        let mut request = fixture.request();
        request.operation.kind = 0;
        request.change_feed_mode = 1;
        request.start_from = start;
        if start == 3 {
            request.start_time = view(b"2026-09-18T12:00:00Z");
        }
        let cursor = fixture.open(&request);
        let page = fixture.receive(cosmos_cursor_next_submit(cursor, 0, ptr::null_mut()));
        assert_eq!(documents(page).len(), 2);
        cosmos_cursor_completion_free(page);
        let checkpoint =
            fixture.receive(cosmos_cursor_checkpoint_submit(cursor, 0, ptr::null_mut()));
        // SAFETY: the counted token stays valid until the new Open copies it.
        unsafe {
            assert_eq!((*checkpoint).common.status, COSMOS_STATUS_SUCCESS);
            request.operation.continuation_token = (*checkpoint).checkpoint;
        }
        request.change_feed_mode = 2;
        request.start_from = 0;
        request.start_time = crate::string::CosmosStringView::default();
        let opened = fixture.receive(cosmos_cursor_open_submit(
            fixture.driver,
            &request,
            fixture.queue,
            0,
            ptr::null_mut(),
        ));
        // SAFETY: the failed open completion is owned and live.
        unsafe {
            assert_ne!((*opened).common.status, COSMOS_STATUS_SUCCESS);
            assert!((*opened).cursor.is_null());
        }
        cosmos_cursor_completion_free(opened);
        cosmos_cursor_completion_free(checkpoint);
        cosmos_cursor_free(cursor);
    }
}

#[test]
fn cancellation_panic_and_error_during_io_are_terminal() {
    for failure in [0, 1, 2] {
        let gate = std::sync::Arc::new(fixture::Gate::default());
        let fixture = Fixture::from_handles(fixture::scripted_gated(true, Some(gate.clone())), 1);
        let mut request = fixture.request();
        request.operation.kind = 15;
        let cursor = fixture.open(&request);
        let op = cosmos_cursor_next_submit(cursor, 0, ptr::null_mut());
        let rt = RuntimeContext::inner_arc(fixture.runtime).unwrap();
        rt.tokio.block_on(async {
            tokio::time::timeout(Duration::from_secs(5), gate.entered.notified())
                .await
                .unwrap();
        });
        if failure == 0 {
            cosmos_operation_handle_cancel(op);
        } else {
            gate.failure
                .store(failure, std::sync::atomic::Ordering::Release);
            gate.release.notify_one();
        }
        let completion = fixture.receive(op);
        assert_ne!(cosmos_cursor_status(cursor), COSMOS_STATUS_SUCCESS);
        let mut status = COSMOS_STATUS_SUCCESS;
        assert!(cosmos_cursor_next_submit(cursor, 0, &mut status).is_null());
        assert_eq!(
            status,
            CosmosErrorCode::CosmosErrorCodeCursorClosed.as_status_code()
        );
        cosmos_cursor_completion_free(completion);
        cosmos_cursor_free(cursor);
    }
}

#[test]
fn abandon_during_io_and_free_during_io_retain_task_owners() {
    for abandon in [false, true] {
        let gate = std::sync::Arc::new(fixture::Gate::default());
        let mut fixture =
            Fixture::from_handles(fixture::scripted_gated(true, Some(gate.clone())), 1);
        let mut request = fixture.request();
        request.operation.kind = 15;
        let cursor = fixture.open(&request);
        let op = cosmos_cursor_next_submit(cursor, 0, ptr::null_mut());
        let rt = RuntimeContext::inner_arc(fixture.runtime).unwrap();
        rt.tokio.block_on(async {
            tokio::time::timeout(Duration::from_secs(5), gate.entered.notified())
                .await
                .unwrap();
        });
        cosmos_cursor_free(cursor);
        if abandon {
            cosmos_completion_queue_free(fixture.queue);
            fixture.queue = ptr::null_mut();
        } else {
            cosmos_completion_queue_shutdown(fixture.queue);
        }
        gate.release.notify_one();
        if abandon {
            fixture.drive(op);
            assert_eq!(
                cosmos_operation_handle_status(op),
                CosmosErrorCode::CosmosErrorCodeDeliveryLost.as_status_code()
            );
            cosmos_operation_handle_free(op);
        } else {
            let page = fixture.receive(op);
            // SAFETY: completion outlives the freed cursor and the shutdown request.
            unsafe {
                assert_eq!((*page).result_kind, 2);
                assert_eq!((*page).common.status, COSMOS_STATUS_SUCCESS);
            }
            cosmos_cursor_completion_free(page);
        }
    }
}
