// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Demonstrates how to define a custom [`TokenCredential`] that selects a
//! specific credential based on an environment variable.

use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    Error,
};
use azure_identity::AzureCliCredential;
use azure_identity::{ManagedIdentityCredential, WorkloadIdentityCredential};
use std::sync::Arc;

// Define the variable name and possible values you want to detect from the environment.
const AZURE_CREDENTIAL_KIND: &str = "AZURE_CREDENTIAL_KIND";
mod azure_credential_kinds {
    pub const AZURE_CLI: &str = "azurecli";
    pub const MANAGED_IDENTITY: &str = "managedidentity";
    pub const WORKLOAD_IDENTITY: &str = "workloadidentity";
}

#[derive(Debug)]
enum SpecificAzureCredentialKind {
    AzureCli(Arc<AzureCliCredential>),
    ManagedIdentity(Arc<ManagedIdentityCredential>),
    WorkloadIdentity(Arc<WorkloadIdentityCredential>),
}

#[async_trait::async_trait]
impl TokenCredential for SpecificAzureCredentialKind {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        match self {
            SpecificAzureCredentialKind::AzureCli(credential) => {
                credential.get_token(scopes, options).await
            }
            SpecificAzureCredentialKind::ManagedIdentity(credential) => {
                credential.get_token(scopes, options).await
            }
            SpecificAzureCredentialKind::WorkloadIdentity(credential) => {
                credential.get_token(scopes, options).await
            }
        }
    }
}

/// Define a credential that uses an environment variable named `AZURE_CREDENTIAL_KIND`
/// that creates the appropriate [`TokenCredential`].
#[derive(Debug)]
pub struct SpecificAzureCredential {
    source: SpecificAzureCredentialKind,
}

impl SpecificAzureCredential {
    pub fn new() -> azure_core::Result<SpecificAzureCredential> {
        let credential_type = std::env::var(AZURE_CREDENTIAL_KIND)
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let source: SpecificAzureCredentialKind =
            // case insensitive and allow spaces
            match credential_type.replace(' ', "").to_lowercase().as_str() {
                azure_credential_kinds::MANAGED_IDENTITY => {
                    ManagedIdentityCredential::new(None)
                        .map(SpecificAzureCredentialKind::ManagedIdentity)
                        .with_context_fn(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_KIND of {}",
                                azure_credential_kinds::MANAGED_IDENTITY
                            )
                        })?
                }
                azure_credential_kinds::AZURE_CLI => AzureCliCredential::new(None)
                    .map(SpecificAzureCredentialKind::AzureCli)
                    .with_context_fn(ErrorKind::Credential, || {
                        format!(
                            "unable to create AZURE_CREDENTIAL_KIND of {}",
                            azure_credential_kinds::AZURE_CLI
                        )
                    })?,
                azure_credential_kinds::WORKLOAD_IDENTITY => {
                    WorkloadIdentityCredential::new(None)
                        .map(SpecificAzureCredentialKind::WorkloadIdentity)
                        .with_context_fn(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_KIND of {}",
                                azure_credential_kinds::WORKLOAD_IDENTITY
                            )
                        })?
                }
                _ => {
                    return Err(Error::with_message_fn(ErrorKind::Credential, || {
                        format!("unknown AZURE_CREDENTIAL_KIND of {}", credential_type)
                    }))
                }
            };
        Ok(Self { source })
    }
}

#[async_trait::async_trait]
impl TokenCredential for SpecificAzureCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.source.get_token(scopes, options).await
    }
}
