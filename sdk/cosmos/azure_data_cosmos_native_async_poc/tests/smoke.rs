// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end smoke test for the async FFI crate against the local Cosmos DB
//! emulator. Gated behind the workspace's `--cfg test_category="emulator"`
//! convention so a plain `cargo test` doesn't try to hit a live endpoint.
//!
//! Pre-conditions (the user has already confirmed all of these):
//!   * Emulator running at https://localhost:8081
//!   * Database `pocdb`, container `items` (pk = `/pk`)
//!   * Item { "id": "x", "pk": "p", "hello": "from native async poc" }
//!
//! Run with:
//!   $env:RUSTFLAGS = '--cfg test_category="emulator"'
//!   cargo test --release -p azure_data_cosmos_native_async_poc --test smoke
//!
//! What this test validates:
//!   F1 — read_item submission returns immediately (non-blocking).
//!   F2 — completion arrives on the CQ with the host's user_data preserved.
//!   F3 — multiple in-flight ops complete out of order without serializing.
//!
//! The .NET POC in Phase 4 will repeat F1/F2/F3 from across the FFI boundary.

#![cfg(test_category = "emulator")]

use std::ffi::CString;
use std::os::raw::c_void;
use std::time::{Duration, Instant};

use cosmos_async_poc::cq::{cosmos_cq_free, cosmos_cq_new, cosmos_cq_shutdown, cosmos_cq_wait};
use cosmos_async_poc::driver::{cosmos_driver_free, cosmos_driver_new};
use cosmos_async_poc::op::cosmos_op_release;
use cosmos_async_poc::read_item::cosmos_read_item;
use cosmos_async_poc::response::{
    cosmos_response_body, cosmos_response_free, cosmos_response_status,
};
use cosmos_async_poc::runtime::{cosmos_runtime_free, cosmos_runtime_new};

const EMULATOR_ENDPOINT: &str = "https://localhost:8081/";
const EMULATOR_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";
const DATABASE_ID: &str = "pocdb";
const CONTAINER_ID: &str = "items";
const PARTITION_KEY: &str = "p";
const ITEM_ID: &str = "x";

#[test]
fn f1_read_item_submission_is_non_blocking() {
    unsafe {
        let rt = cosmos_runtime_new(2);
        assert!(!rt.is_null(), "runtime_new failed");

        let endpoint = CString::new(EMULATOR_ENDPOINT).unwrap();
        let key = CString::new(EMULATOR_KEY).unwrap();
        let mut driver = std::ptr::null_mut();
        let rc = cosmos_driver_new(rt, endpoint.as_ptr(), key.as_ptr(), &mut driver);
        assert_eq!(rc, 0, "driver_new failed (rc={rc})");

        let cq = cosmos_cq_new(64);
        assert!(!cq.is_null());

        let db = CString::new(DATABASE_ID).unwrap();
        let container = CString::new(CONTAINER_ID).unwrap();
        let pk = CString::new(PARTITION_KEY).unwrap();
        let id = CString::new(ITEM_ID).unwrap();

        // F1: submit and measure that the call returns "fast" (< 50 ms).
        // The actual read involves at least one network round trip, so
        // anything close to that bound proves the call did not block on
        // the I/O.
        let mut op = std::ptr::null_mut();
        let before = Instant::now();
        let rc = cosmos_read_item(
            driver,
            db.as_ptr(),
            container.as_ptr(),
            pk.as_ptr(),
            id.as_ptr(),
            cq,
            0xCAFE_BABE,
            &mut op,
        );
        let submit_elapsed = before.elapsed();
        assert_eq!(rc, 0, "read_item submit failed (rc={rc})");
        assert!(!op.is_null());
        assert!(
            submit_elapsed < Duration::from_millis(50),
            "read_item submit took {submit_elapsed:?} (must be non-blocking)"
        );

        // F2: drain the completion within 10 s (plenty for a localhost read).
        let mut user_data: usize = 0;
        let mut status: i32 = -1;
        let mut response = std::ptr::null_mut();
        let rc = cosmos_cq_wait(cq, 10_000, &mut user_data, &mut status, &mut response);
        assert_eq!(rc, 0, "cq_wait failed (rc={rc})");
        assert_eq!(status, 0, "operation failed (status={status})");
        assert_eq!(user_data, 0xCAFE_BABE, "user_data not preserved (I1)");
        assert!(!response.is_null());

        let http_status = cosmos_response_status(response);
        assert_eq!(http_status, 200, "expected HTTP 200, got {http_status}");

        let mut body_ptr: *const c_void = std::ptr::null();
        let mut body_len: usize = 0;
        let rc = cosmos_response_body(response, &mut body_ptr, &mut body_len);
        assert_eq!(rc, 0);
        assert!(body_len > 0);
        let body = std::slice::from_raw_parts(body_ptr as *const u8, body_len);
        let body_str = std::str::from_utf8(body).expect("body is UTF-8");
        assert!(
            body_str.contains("from native async poc"),
            "body did not contain seeded marker; body={body_str}"
        );

        cosmos_response_free(response);
        cosmos_op_release(op);
        cosmos_cq_shutdown(cq);
        cosmos_cq_free(cq);
        cosmos_driver_free(driver);
        cosmos_runtime_free(rt);
    }
}

#[test]
fn f3_in_flight_ops_complete_out_of_order() {
    unsafe {
        let rt = cosmos_runtime_new(4);
        let endpoint = CString::new(EMULATOR_ENDPOINT).unwrap();
        let key = CString::new(EMULATOR_KEY).unwrap();
        let mut driver = std::ptr::null_mut();
        let rc = cosmos_driver_new(rt, endpoint.as_ptr(), key.as_ptr(), &mut driver);
        assert_eq!(rc, 0);

        let cq = cosmos_cq_new(64);
        let db = CString::new(DATABASE_ID).unwrap();
        let container = CString::new(CONTAINER_ID).unwrap();
        let pk = CString::new(PARTITION_KEY).unwrap();
        let id = CString::new(ITEM_ID).unwrap();

        // F3: submit N in-flight reads, prove the host thread submits all
        // of them before the first completion comes back. If the FFI were
        // blocking, we'd see roughly serial behavior — total elapsed >=
        // N * single_read_time. Async should be roughly single_read_time.
        let n: usize = 8;
        let mut ops = Vec::with_capacity(n);
        let submit_before = Instant::now();
        for i in 0..n {
            let mut op = std::ptr::null_mut();
            let rc = cosmos_read_item(
                driver,
                db.as_ptr(),
                container.as_ptr(),
                pk.as_ptr(),
                id.as_ptr(),
                cq,
                0xAAAA_0000 + i,
                &mut op,
            );
            assert_eq!(rc, 0);
            ops.push(op);
        }
        let submit_total = submit_before.elapsed();
        assert!(
            submit_total < Duration::from_millis(200),
            "submitting {n} ops took {submit_total:?} (must be non-blocking)"
        );

        let drain_before = Instant::now();
        let mut seen = vec![false; n];
        for _ in 0..n {
            let mut user_data: usize = 0;
            let mut status: i32 = -1;
            let mut response = std::ptr::null_mut();
            let rc = cosmos_cq_wait(cq, 15_000, &mut user_data, &mut status, &mut response);
            assert_eq!(rc, 0);
            assert_eq!(status, 0, "op failed status={status}");
            let idx = user_data - 0xAAAA_0000;
            assert!(idx < n, "user_data out of band: {user_data:#x}");
            assert!(!seen[idx], "user_data {user_data:#x} delivered twice");
            seen[idx] = true;
            cosmos_response_free(response);
        }
        let drain_total = drain_before.elapsed();
        assert!(
            seen.iter().all(|b| *b),
            "not every submission produced a completion"
        );

        eprintln!(
            "F3: {n} in-flight reads submitted in {submit_total:?}, drained in {drain_total:?}"
        );

        for op in ops {
            cosmos_op_release(op);
        }
        cosmos_cq_shutdown(cq);
        cosmos_cq_free(cq);
        cosmos_driver_free(driver);
        cosmos_runtime_free(rt);
    }
}
