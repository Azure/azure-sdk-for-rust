// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Differential fuzz target: decode → encode → decode idempotence.
//!
//! Unlike the plain `decode` no-crash target, this asserts a **semantic**
//! invariant on every buffer the decoder *accepts*: re-encoding the decoded
//! value and decoding it again must reproduce the exact same value. It catches
//! the class of bug the live round-trip fuzzer cannot — a buffer the decoder
//! accepts but the encoder would round-trip to a *different* value (marker or
//! number-form disagreements between the reader and writer). libFuzzer's
//! mutation reaches decoder-accepted-but-unusual frames that hand-written
//! golden vectors don't enumerate.
//!
//! Oracle: `decode(data) = Ok(v)`  ⇒  `decode(encode(v)) = Ok(v)`.

#![no_main]

use azure_data_cosmos_driver::binary_json::{decode, encode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = decode(data) {
        let reencoded = encode(&value);
        let redecoded =
            decode(&reencoded).expect("re-encoding a decoded value must itself decode");
        assert_eq!(
            value, redecoded,
            "decode∘encode∘decode is not idempotent for a decoder-accepted buffer"
        );
    }
});
