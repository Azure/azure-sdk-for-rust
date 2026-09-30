// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::AppendBlobClient;

use crate::SessionOptions;
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{ClientOptions, Url},
    tracing, Result,
};
use std::sync::Arc;

/// Options used when creating an [`AppendBlobClient`].
#[derive(Clone, SafeDebug)]
pub struct AppendBlobClientOptions {
    /// Allows customization of the client.
    pub client_options: ClientOptions,
    /// Options for session token authentication.
    pub session_options: Option<SessionOptions>,
    /// Specifies the version of the operation to use for requests.
    pub version: String,
}

impl Default for AppendBlobClientOptions {
    fn default() -> Self {
        let generated = crate::generated::clients::AppendBlobClientOptions::default();
        Self {
            client_options: generated.client_options,
            session_options: None,
            version: generated.version,
        }
    }
}

impl AppendBlobClient {
    /// Creates a new AppendBlobClient from a blob URL.
    ///
    /// # Arguments
    ///
    /// * `blob_url` - The full URL of the Append blob, for example `https://myaccount.blob.core.windows.net/mycontainer/myblob`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.AppendBlob")]
    pub fn new(
        blob_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<AppendBlobClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if blob_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{blob_url} is not a valid base URL."),
            ));
        }

        let mut options = options.unwrap_or_default();
        let pipeline = super::build_pipeline(
            &blob_url,
            credential,
            options.session_options.as_ref(),
            &mut options.client_options,
            &options.version,
        )?;

        Ok(Self {
            endpoint: blob_url,
            pipeline,
            version: options.version,
        })
    }

    /// Gets the URL of the blob.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }
}
