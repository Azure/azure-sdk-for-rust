// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Byte-level protocol fuzz target for the driver-side response transcode.
//!
//! [`transcode_to_text`] is what the driver runs on a binary response body when
//! a text-only host asked for text back: it decodes the binary buffer and
//! re-serializes it as UTF-8 text JSON (or passes text/empty input through
//! unchanged). It sits on the FFI/text-host response path, so a panic here on a
//! malformed service body would take down the host.
//!
//! Oracle: for any input, transcoding must terminate with `Ok`/`Err` — never
//! panic, hang, or over-allocate.

#![no_main]

use azure_data_cosmos_driver::binary_json::transcode_to_text;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = transcode_to_text(data);
});
