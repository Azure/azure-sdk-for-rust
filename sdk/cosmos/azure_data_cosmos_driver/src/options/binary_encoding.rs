// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`BinaryEncodingOptions`] — driver-level Cosmos binary JSON encoding options.

/// Options controlling Cosmos **binary JSON** on the wire for an operation.
///
/// These options are **schema-agnostic**, so they live in the driver and are
/// shared by every consumer — the Rust SDK ([`azure_data_cosmos`]) and any
/// FFI-based SDK (.NET, Java, Go, …) that drives the C ABI. They are set on
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
/// A typed consumer (the Rust SDK) may pre-encode its request body straight from
/// `T: Serialize` as an optimization; the driver's request-side transcoding then
/// sees an already-binary body and passes it through unchanged.
///
/// Binary encoding is in preview. Additional tuning knobs (for example ULong
/// support or Base64 optimization) may be added here in future versions, so the
/// struct is `#[non_exhaustive]`.
///
/// [`azure_data_cosmos`]: https://docs.rs/azure_data_cosmos
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct BinaryEncodingOptions {
    /// Whether Cosmos binary JSON is used on the wire for the operation.
    ///
    /// When `true`, the request body is sent as binary (the driver transcodes a
    /// text body to binary first, or passes an already-binary body through) and
    /// the client advertises that it accepts binary responses.
    pub enabled: bool,

    /// Whether the driver hands the caller **text** JSON even when binary
    /// encoding is [`enabled`](Self::enabled).
    ///
    /// When `false` (the default), the binary response is returned as-is. When
    /// `true`, the wire stays binary in both directions and the driver
    /// transcodes the binary response to text JSON before returning it. Has no
    /// effect when [`enabled`](Self::enabled) is `false`.
    pub request_text_response: bool,
}

impl BinaryEncodingOptions {
    /// Creates binary-encoding options with defaults (disabled).
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
