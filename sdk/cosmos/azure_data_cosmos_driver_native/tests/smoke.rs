// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Smoke tests for the FFI surface, run via cargo test on the rlib.

use std::ffi::{CStr, CString};
use std::ptr;

use azurecosmosdriver::bytes::{cosmos_bytes_free, CosmosBytes};
use azurecosmosdriver::cosmos_version;
use azurecosmosdriver::error::{CosmosError, CosmosErrorCode};
use azurecosmosdriver::handles::account::{
    cosmos_account_ref, cosmos_account_ref_free, cosmos_account_ref_with_master_key,
};
use azurecosmosdriver::handles::partition_key::{
    cosmos_partition_key, cosmos_partition_key_builder_append_string,
    cosmos_partition_key_builder_build, cosmos_partition_key_builder_new,
    cosmos_partition_key_free, cosmos_partition_key_from_string,
};
use azurecosmosdriver::runtime::{cosmos_runtime_create, cosmos_runtime_free};
use azurecosmosdriver::string::cosmos_string_free;

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

#[test]
fn version_matches_cargo_pkg_version() {
    unsafe {
        let p = cosmos_version();
        assert!(!p.is_null());
        assert_eq!(
            CStr::from_ptr(p).to_str().unwrap(),
            env!("CARGO_PKG_VERSION")
        );
    }
}

#[test]
fn runtime_create_and_free() {
    let mut err = CosmosError::default();
    let rt = cosmos_runtime_create(ptr::null(), &mut err);
    assert!(!rt.is_null(), "code={:?}", err.code);
    unsafe { cosmos_runtime_free(rt) };
}

#[test]
fn account_ref_rejects_invalid_endpoint() {
    let mut err = CosmosError::default();
    let endpoint = cstr("not a url");
    let key = cstr("c2VjcmV0");
    unsafe {
        let acct: *mut cosmos_account_ref =
            cosmos_account_ref_with_master_key(endpoint.as_ptr(), key.as_ptr(), &mut err);
        assert!(acct.is_null());
        assert_eq!(err.code, CosmosErrorCode::InvalidAccountReference);
        if !err.detail.is_null() {
            cosmos_string_free(err.detail);
        }
    }
}

#[test]
fn account_ref_with_valid_endpoint_succeeds() {
    let mut err = CosmosError::default();
    let endpoint = cstr("https://example.documents.azure.com:443/");
    let key = cstr("c2VjcmV0");
    unsafe {
        let acct: *mut cosmos_account_ref =
            cosmos_account_ref_with_master_key(endpoint.as_ptr(), key.as_ptr(), &mut err);
        assert!(!acct.is_null(), "code={:?}", err.code);
        cosmos_account_ref_free(acct);
    }
}

#[test]
fn partition_key_builder_single_string_round_trip() {
    unsafe {
        let b = cosmos_partition_key_builder_new();
        let v = cstr("tenant-1");
        let c1 = cosmos_partition_key_builder_append_string(b, v.as_ptr());
        assert_eq!(c1, CosmosErrorCode::Success);
        let mut pk: *mut cosmos_partition_key = ptr::null_mut();
        let c2 = cosmos_partition_key_builder_build(b, &mut pk);
        assert_eq!(c2, CosmosErrorCode::Success);
        assert!(!pk.is_null());
        cosmos_partition_key_free(pk);
    }
}

#[test]
fn partition_key_builder_empty_build_is_rejected() {
    unsafe {
        let b = cosmos_partition_key_builder_new();
        let mut pk: *mut cosmos_partition_key = ptr::null_mut();
        let c = cosmos_partition_key_builder_build(b, &mut pk);
        assert_eq!(c, CosmosErrorCode::InvalidPartitionKey);
        assert!(pk.is_null());
    }
}

#[test]
fn partition_key_from_string_convenience_works() {
    let mut err = CosmosError::default();
    let v = cstr("hello");
    unsafe {
        let mut pk: *mut cosmos_partition_key = ptr::null_mut();
        let c = cosmos_partition_key_from_string(v.as_ptr(), &mut pk, &mut err);
        assert_eq!(c, CosmosErrorCode::Success);
        assert!(!pk.is_null());
        cosmos_partition_key_free(pk);
    }
}

#[test]
fn bytes_buffer_round_trip() {
    let bytes = CosmosBytes::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(bytes.len, 5);
    assert!(!bytes.data.is_null());
    assert!(!bytes.handle.is_null());
    unsafe {
        let s = std::slice::from_raw_parts(bytes.data, bytes.len);
        assert_eq!(s, &[1u8, 2, 3, 4, 5]);
        cosmos_bytes_free(bytes);
    }
}
