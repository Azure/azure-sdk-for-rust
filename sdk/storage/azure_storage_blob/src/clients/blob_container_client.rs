// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobContainerClient, BlobContainerClientOptions};

use crate::{
    logging::apply_storage_logging_defaults,
    models::{BlobContainerClientGetAccessPolicyOptions, SignedIdentifier, StorageErrorCode},
    pipeline::StorageHeadersPolicy,
    BlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        NoFormat, Pipeline, RequestContent, Response, StatusCode, Url, XmlFormat,
    },
    tracing,
    xml::{from_xml, to_xml},
    Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename = "SignedIdentifiers")]
struct SignedIdentifiersEnvelope<'a> {
    #[serde(rename = "SignedIdentifier")]
    items: &'a [SignedIdentifier],
}

#[derive(Deserialize)]
#[serde(rename = "SignedIdentifiers")]
struct SignedIdentifiersEnvelopeOwned {
    #[serde(rename = "SignedIdentifier", default)]
    items: Vec<SignedIdentifier>,
}

impl BlobContainerClient {
    /// Creates a new BlobContainerClient, using Entra ID authentication.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full URL of the Azure storage account, for example `https://myaccount.blob.core.windows.net/`
    /// * `container_name` - The name of the container.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    pub fn new(
        endpoint: &str,
        container_name: &str,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut url = Url::parse(endpoint)?;

        {
            let mut path_segments = url.path_segments_mut().map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "Invalid endpoint URL: Failed to parse out path segments from provided endpoint URL.",
                )
            })?;
            path_segments.extend([container_name]);
        }

        Self::from_url(url, credential, options)
    }

    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn from_url(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let storage_headers_policy = Arc::new(StorageHeadersPolicy);
        options
            .client_options
            .per_call_policies
            .push(storage_headers_policy);

        if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
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
            endpoint: container_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as container URL has already been validated on client construction.
            .expect("Invalid endpoint URL: Cannot append blob_name to the blob endpoint.")
            .extend([blob_name]);

        BlobClient {
            endpoint: blob_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the container.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Checks if the container exists.
    ///
    /// Returns `true` if the container exists, `false` if the container does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::ContainerNotFound.as_ref() => Ok(false),
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Gets access policies as a vector of signed identifiers.
    pub async fn get_access_policy_items(
        &self,
        options: Option<BlobContainerClientGetAccessPolicyOptions<'_>>,
    ) -> Result<Vec<SignedIdentifier>> {
        let response = self.get_access_policy(options).await?;
        let envelope: SignedIdentifiersEnvelopeOwned = from_xml(response.body().as_ref())?;
        Ok(envelope.items)
    }

    /// Sets access policies from signed identifiers.
    ///
    /// This method wraps the identifiers in a `SignedIdentifiers` XML root element
    /// before sending so it works with the current generated request body shape.
    pub async fn set_access_policy_items(
        &self,
        signed_identifiers: Vec<SignedIdentifier>,
        options: Option<crate::models::BlobContainerClientSetAccessPolicyOptions<'_>>,
    ) -> Result<Response<(), NoFormat>> {
        let envelope = SignedIdentifiersEnvelope {
            items: &signed_identifiers,
        };

        let content: RequestContent<Vec<SignedIdentifier>, XmlFormat> =
            RequestContent::from(to_xml(&envelope)?.to_vec());

        self.set_access_policy(content, options).await
    }
}

#[cfg(test)]
mod tests {
    use super::SignedIdentifiersEnvelope;
    use crate::models::SignedIdentifier;

    #[test]
    fn signed_identifiers_envelope_serializes_with_root_and_multiple_items() {
        let signed_identifiers = vec![
            SignedIdentifier {
                id: Some("id-1".to_string()),
                access_policy: None,
            },
            SignedIdentifier {
                id: Some("id-2".to_string()),
                access_policy: None,
            },
        ];

        let xml = azure_core::xml::to_xml(&SignedIdentifiersEnvelope {
            items: &signed_identifiers,
        })
        .expect("serialization should succeed");
        let xml = std::str::from_utf8(&xml).expect("serialized xml should be utf-8");

        assert!(xml.contains("<SignedIdentifiers>"));
        assert_eq!(xml.match_indices("<SignedIdentifier>").count(), 2);
    }

    #[test]
    fn signed_identifiers_envelope_serializes_empty_array() {
        let signed_identifiers: Vec<SignedIdentifier> = Vec::new();

        let xml = azure_core::xml::to_xml(&SignedIdentifiersEnvelope {
            items: &signed_identifiers,
        })
        .expect("serialization should succeed");
        let xml = std::str::from_utf8(&xml).expect("serialized xml should be utf-8");

        assert!(xml.contains("<SignedIdentifiers"));
    }
}
