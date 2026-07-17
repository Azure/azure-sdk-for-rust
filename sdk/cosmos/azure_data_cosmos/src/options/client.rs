// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosClientOptions`] — options for [`CosmosClient`](crate::CosmosClient) construction.

use azure_data_cosmos_driver::options::{OperationOptions, UserAgentSuffix};

/// Options used when creating a [`CosmosClient`](crate::CosmosClient).
///
/// This struct is used internally by [`CosmosClientBuilder`](crate::CosmosClientBuilder).
/// Use the builder pattern via [`CosmosClient::builder()`](crate::CosmosClient::builder())
/// to configure client options.
#[derive(Clone, Default, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    /// Default [`OperationOptions`] applied to all requests made by this client,
    /// unless overridden by per-request options.
    pub operation: OperationOptions,
    pub(crate) user_agent_suffix: Option<UserAgentSuffix>,
    /// Explicit binary-encoding options. `None` (the default) falls back to the
    /// `AZURE_COSMOS_BINARY_ENCODING_ENABLED` environment variable for
    /// enablement.
    pub(crate) binary_encoding: Option<BinaryEncodingOptions>,
}

/// Options controlling Cosmos binary JSON encoding for a
/// [`CosmosClient`](crate::CosmosClient).
///
/// Binary encoding governs item write bodies (encoded as Cosmos binary JSON)
/// and the response-format negotiation advertised to the service. Enablement is
/// resolved once at client-build time.
///
/// Binary encoding is in preview. Additional tuning knobs (for example, ULong
/// support or Base64 optimization) may be added here in future versions, so the
/// struct is `#[non_exhaustive]`.
///
/// # Examples
///
/// ```rust
/// use azure_data_cosmos::options::BinaryEncodingOptions;
///
/// // Encode write bodies as binary, but ask the service for text responses.
/// let options = BinaryEncodingOptions::new()
///     .with_enabled(true)
///     .with_request_text_response(true);
/// assert!(options.enabled);
/// assert!(options.request_text_response);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct BinaryEncodingOptions {
    /// Whether Cosmos binary JSON encoding is enabled for the client.
    ///
    /// When enabled, item write bodies are encoded as Cosmos binary JSON and
    /// the client advertises (via the
    /// `x-ms-cosmos-supported-serialization-formats` header) that it accepts
    /// binary responses.
    pub enabled: bool,

    /// Whether to request text-JSON response payloads even when binary encoding
    /// is [`enabled`](Self::enabled).
    ///
    /// When `false` (the default), the client advertises both text and binary
    /// response formats (`JsonText,CosmosBinary`) and the service typically
    /// replies with binary. When `true`, the client advertises only `JsonText`,
    /// so the service returns text-JSON response payloads while item **write**
    /// bodies are still sent as binary. This has no effect when
    /// [`enabled`](Self::enabled) is `false` (no binary is used in either
    /// direction and no negotiation header is sent).
    pub request_text_response: bool,
}

impl BinaryEncodingOptions {
    /// Creates binary-encoding options with defaults (encoding disabled).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether Cosmos binary JSON encoding is enabled.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets whether to request text-JSON response payloads even when binary
    /// encoding is enabled.
    ///
    /// See [`request_text_response`](Self::request_text_response) for the
    /// negotiation behavior.
    pub fn with_request_text_response(mut self, request_text_response: bool) -> Self {
        self.request_text_response = request_text_response;
        self
    }
}

impl CosmosClientOptions {
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.user_agent_suffix = Some(suffix);
        self
    }

    pub fn with_operation_options(mut self, operation: OperationOptions) -> Self {
        self.operation = operation;
        self
    }
}
