// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte-level protocol fuzz target for the native serde **deserializer**.
//!
//! [`from_slice`] is the zero-`Value` streaming decode path used by the SDK's
//! typed reads; it drives a different code path from [`decode`] (it streams
//! tokens into a serde visitor instead of materializing a
//! [`serde_json::Value`]). Fuzzing it independently ensures the streaming
//! deserializer honors the same no-crash contract on malformed input.
//!
//! Oracle: for any input, deserialization must terminate with `Ok`/`Err` —
//! never panic, hang, or over-allocate.

#![no_main]

use azure_data_cosmos_driver::binary_json::from_slice;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = from_slice::<serde_json::Value>(data);
});
