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
    /// Creates a new QueueServiceClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.queue.core.windows.net/`
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Service")]
    pub fn new(
        endpoint: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueServiceClientOptions>,
    ) -> Result<Self> {
        let endpoint = Url::parse(endpoint)?;
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        if let Some(token_credential) = credential {
            if !endpoint.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{endpoint} must use https"),
                ));
            }
            let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            ));
            options.client_options.per_try_policies.push(auth_policy);
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            Vec::default(),
            None,
        );

        Ok(Self {
            endpoint,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of QueueClient.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue.
    pub fn queue_client(&self, queue_name: &str) -> Result<QueueClient> {
        let mut queue_url = self.endpoint().clone();
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
    use super::{QueueServiceClient, QueueServiceClientOptions};
    use azure_core_test::credentials::MockCredential;

    #[test]
    fn new_requires_https_with_credential() {
        let cred = MockCredential::new().unwrap();
        let err = QueueServiceClient::new(
            "http://myaccount.queue.core.windows.net/",
            Some(cred),
            None,
        )
        .err()
        .unwrap();
        assert!(
            err.to_string().contains("must use https"),
            "Expected error message to contain 'must use https', got: {err}"
        );
    }

    #[test]
    fn new_allows_http_without_credential() {
        // HTTP is allowed when no credential is provided (e.g., SAS token in URL)
        let result = QueueServiceClient::new(
            "http://myaccount.queue.core.windows.net/",
            None,
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn new_allows_https_with_credential() {
        let cred = MockCredential::new().unwrap();
        let result = QueueServiceClient::new(
            "https://myaccount.queue.core.windows.net/",
            Some(cred),
            Some(QueueServiceClientOptions::default()),
        );
        assert!(result.is_ok());
    }
}
