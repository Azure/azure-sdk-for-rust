// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{QueueServiceClient, QueueServiceClientOptions};

use crate::{clients::QueueClient, logging::apply_storage_logging_defaults};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl QueueServiceClient {
    /// Creates a new `QueueServiceClient` from a service URL.
    ///
    /// # Arguments
    ///
    /// * `service_url` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Service")]
    pub fn new(
        service_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if service_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{service_url} is not a valid base URL"),
            ));
        }
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !service_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{service_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: service_url,
            version: options.version,
            pipeline,
        })
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Returns a new instance of QueueClient.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue.
    pub fn queue_client(&self, queue_name: &str) -> Result<QueueClient> {
        let mut queue_url = self.url().clone();
        queue_url
            .path_segments_mut()
            .map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?
            .push(queue_name);
        Ok(QueueClient {
            endpoint: queue_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_non_base_url() {
        let url = Url::parse("data:text/plain,hello").unwrap();
        assert!(QueueServiceClient::new(url, None, None).is_err());
    }

    #[test]
    fn new_accepts_https_url() {
        let url = Url::parse("https://myaccount.queue.core.windows.net/").unwrap();
        assert!(QueueServiceClient::new(url, None, None).is_ok());
    }
}
