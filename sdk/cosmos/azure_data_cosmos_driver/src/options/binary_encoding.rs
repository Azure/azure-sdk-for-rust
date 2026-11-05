// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`BinaryEncodingOptions`] — driver-level Cosmos binary JSON encoding options.

/// Options controlling Cosmos **binary JSON** on the wire for an operation.
///
/// These options are **schema-agnostic**, so they live in the driver. They are
/// set on
/// [`OperationOptions::binary_encoding`](crate::options::OperationOptions::binary_encoding)
/// and participate in the standard runtime → account → operation layered
/// resolution.
///
/// The driver performs the byte-level transcoding both ways when needed, so a
/// caller can deal purely in text JSON and still get an efficient binary wire:
///
/// - [`enabled`](Self::enabled) — put **binary on the wire**. On the request
///   path, a text-JSON body is transcoded to binary before it is sent (an
///   already-binary body is passed through). The client also advertises
///   `CosmosBinary`, so the service response comes back binary.
/// - [`request_text_response`](Self::request_text_response) — hand the caller
///   **text JSON back**. The wire still stays binary; the driver transcodes the
///   binary response to text before returning it. Has no effect unless
///   [`enabled`](Self::enabled) is `true`.
///
/// The response side applies uniformly to every operation that negotiates
/// binary — point item operations and queries alike. The request side is
/// narrower: only an item body is transcoded. A query's request body is a query
/// spec rather than a document, so it stays text either way while its *response*
/// still comes back binary.
///
/// A typed consumer may pre-encode its request body straight from
/// `T: Serialize` as an optimization; the driver's request-side transcoding then
/// sees an already-binary body and passes it through unchanged.
///
/// # Examples
///
/// ```rust
/// use azure_data_cosmos_driver::options::BinaryEncodingOptions;
///
/// // Binary on the wire, but transcode the response back to text.
/// let options = BinaryEncodingOptions::new()
///     .with_enabled(true)
///     .with_request_text_response(true);
/// assert!(options.enabled);
/// assert!(options.request_text_response);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct BinaryEncodingOptions {
    /// Whether Cosmos binary JSON is used on the wire for the operation.
    ///
    /// When `true`, an item request body is sent as binary (the driver
    /// transcodes a text body to binary first, or passes an already-binary body
    /// through) and the client advertises that it accepts binary responses. A
    /// query's request body stays text — it is a query spec, not a document —
    /// but its response is still negotiated binary.
    pub enabled: bool,

    /// Whether the driver hands the caller **text** JSON even when binary
    /// encoding is [`enabled`](Self::enabled).
    ///
    /// When `false` (the default), the binary response is returned as-is. When
    /// `true`, the wire stays binary in both directions and the driver
    /// transcodes the binary response to text JSON before returning it — for a
    /// query, each result item in the page. Has no effect when
    /// [`enabled`](Self::enabled) is `false`.
    ///
    /// The text produced this way is **re-serialized by the driver**, not the
    /// service's original bytes. Values are preserved, but object keys are
    /// emitted in sorted order and numbers use Rust's shortest round-trip
    /// rendering (`1e20` renders as `1e+20`). Callers needing byte-exact service
    /// output should leave [`enabled`](Self::enabled) `false`.
    pub request_text_response: bool,
}

impl Default for BinaryEncodingOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            request_text_response: false,
        }
    }
}

impl BinaryEncodingOptions {
    /// Creates binary-encoding options with binary encoding enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether Cosmos binary JSON is used on the wire.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets whether the driver transcodes the binary response back to text JSON.
    ///
    /// See [`request_text_response`](Self::request_text_response) for the
    /// behavior.
    pub fn with_request_text_response(mut self, request_text_response: bool) -> Self {
        self.request_text_response = request_text_response;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enabled_by_default() {
        let options = BinaryEncodingOptions::default();

        assert!(options.enabled);
        assert!(!options.request_text_response);
    }
}
