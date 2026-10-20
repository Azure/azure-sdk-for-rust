// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte-level protocol fuzz target for the binary-JSON **decoder**.
//!
//! libFuzzer feeds arbitrary (and, once seeded, mutated-from-valid) byte
//! buffers straight into [`decode`]. This is the format fuzzer the live
//! round-trip test can't be: it explores mis-encoded frames — truncated
//! buffers, bad length prefixes, unknown/misused markers, reference and
//! depth bombs, non-UTF-8 string payloads, trailing bytes — that the encoder
//! never produces.
//!
//! Oracle: for **any** input the decoder must terminate and return either
//! `Ok(Value)` or `Err(BinaryError)` — never panic, hang, or allocate beyond
//! what the buffer can back. A crash or hang here is a decoder-hardening bug.

#![no_main]

use azure_data_cosmos_driver::binary_json::decode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = decode(data);
});
