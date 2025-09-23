// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    Error,
};
#[cfg(not(target_arch = "wasm32"))]
use azure_identity::AzureCliCredential;
use azure_identity::{ManagedIdentityCredential, WorkloadIdentityCredential};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    let credential = SpecificAzureCredential::new()?;

    // Enumerate the Azure storage accounts in the subscription using the REST API directly.
    // This is just an example: you would normally pass in an `Arc::new(credential)` to an Azure SDK client.
    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;

    let access_token = credential
        .get_token(&["https://management.azure.com/.default"], None)
        .await?;

    let response = reqwest::Client::new()
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", access_token.token.secret()),
        )
        .send()
        .await?
        .text()
        .await?;

    println!("{response}");
    Ok(())
}

// Define the variable name and possible values you want to detect from the environment.
const AZURE_CREDENTIAL_KIND: &str = "AZURE_CREDENTIAL_KIND";
mod azure_credential_kinds {
    #[cfg(not(target_arch = "wasm32"))]
    pub const AZURE_CLI: &str = "azurecli";
    pub const MANAGED_IDENTITY: &str = "managedidentity";
    pub const WORKLOAD_IDENTITY: &str = "workloadidentity";
}

#[derive(Debug)]
enum SpecificAzureCredentialKind {
    #[cfg(not(target_arch = "wasm32"))]
    AzureCli(Arc<AzureCliCredential>),
    ManagedIdentity(Arc<ManagedIdentityCredential>),
    WorkloadIdentity(Arc<WorkloadIdentityCredential>),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredentialKind {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
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
struct SpecificAzureCredential {
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
                #[cfg(not(target_arch = "wasm32"))]
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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.source.get_token(scopes, options).await
    }
}
