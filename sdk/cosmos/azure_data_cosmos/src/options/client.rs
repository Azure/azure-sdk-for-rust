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
/// Binary encoding governs two things together so they cannot drift apart:
/// encoding item write bodies as Cosmos binary JSON, and advertising that the
/// client accepts binary responses via the response-format negotiation header.
/// Enablement is resolved once at client-build time.
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
/// let options = BinaryEncodingOptions::new().with_enabled(true);
/// assert!(options.enabled);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct BinaryEncodingOptions {
    /// Whether Cosmos binary JSON encoding is enabled for the client.
    pub enabled: bool,
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
