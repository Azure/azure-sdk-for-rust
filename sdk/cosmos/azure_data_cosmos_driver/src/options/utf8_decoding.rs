// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Controls how invalid UTF-8 in response JSON strings is handled.
///
/// This setting applies after a response is received and before its JSON is
/// deserialized. It doesn't make malformed JSON valid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Utf8DecodingMode {
    /// Rejects response strings that contain invalid UTF-8.
    #[default]
    Strict,

    /// Replaces each invalid UTF-8 sequence with the Unicode replacement
    /// character (`U+FFFD`).
    Lossy,
}
