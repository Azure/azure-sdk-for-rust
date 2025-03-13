// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{ErrorKind, ResultExt},
    Error,
};
#[cfg(not(target_arch = "wasm32"))]
use azure_identity::AzureCliCredential;
use azure_identity::{
    AppServiceManagedIdentityCredential, ImdsId, TokenCredentialOptions,
    VirtualMachineManagedIdentityCredential, WorkloadIdentityCredential,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    let credential = SpecificAzureCredential::new(TokenCredentialOptions::default())?;

    // Enumerate the Azure storage accounts in the subscription using the REST API directly.
    // This is just an example: you would normally pass in an `Arc::new(credential)` to an Azure SDK client.
    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;

    let access_token = credential
        .get_token(&["https://management.azure.com/.default"])
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
    pub const VIRTUAL_MACHINE: &str = "virtualmachine";
    pub const APP_SERVICE: &str = "appservice";
    pub const WORKLOAD_IDENTITY: &str = "workloadidentity";
}

#[derive(Debug)]
enum SpecificAzureCredentialKind {
    #[cfg(not(target_arch = "wasm32"))]
    AzureCli(Arc<AzureCliCredential>),
    VirtualMachine(Arc<VirtualMachineManagedIdentityCredential>),
    AppService(Arc<AppServiceManagedIdentityCredential>),
    WorkloadIdentity(Arc<WorkloadIdentityCredential>),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredentialKind {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            SpecificAzureCredentialKind::AzureCli(credential) => credential.get_token(scopes).await,
            SpecificAzureCredentialKind::VirtualMachine(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialKind::AppService(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialKind::WorkloadIdentity(credential) => {
                credential.get_token(scopes).await
            }
        }
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            SpecificAzureCredentialKind::AzureCli(credential) => credential.clear_cache().await,
            SpecificAzureCredentialKind::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            SpecificAzureCredentialKind::AppService(credential) => credential.clear_cache().await,
            SpecificAzureCredentialKind::WorkloadIdentity(credential) => {
                credential.clear_cache().await
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
    pub fn new(options: TokenCredentialOptions) -> azure_core::Result<SpecificAzureCredential> {
        let credential_type = std::env::var(AZURE_CREDENTIAL_KIND)
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let source: SpecificAzureCredentialKind =
            // case insensitive and allow spaces
            match credential_type.replace(' ', "").to_lowercase().as_str() {
                azure_credential_kinds::APP_SERVICE => {
                    AppServiceManagedIdentityCredential::new(options)
                        .map(SpecificAzureCredentialKind::AppService)
                        .with_context(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_KIND of {}",
                                azure_credential_kinds::APP_SERVICE
                            )
                        })?
                }
                azure_credential_kinds::VIRTUAL_MACHINE => {
                    SpecificAzureCredentialKind::VirtualMachine(
                        VirtualMachineManagedIdentityCredential::new(ImdsId::SystemAssigned, options)?,
                    )
                }
                #[cfg(not(target_arch = "wasm32"))]
                azure_credential_kinds::AZURE_CLI => AzureCliCredential::new(Some(options.into()))
                    .map(SpecificAzureCredentialKind::AzureCli)
                    .with_context(ErrorKind::Credential, || {
                        format!(
                            "unable to create AZURE_CREDENTIAL_KIND of {}",
                            azure_credential_kinds::AZURE_CLI
                        )
                    })?,
                azure_credential_kinds::WORKLOAD_IDENTITY => {
                    WorkloadIdentityCredential::from_env(Some(options.into()))
                        .map(SpecificAzureCredentialKind::WorkloadIdentity)
                        .with_context(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_KIND of {}",
                                azure_credential_kinds::WORKLOAD_IDENTITY
                            )
                        })?
                }
                _ => {
                    return Err(Error::with_message(ErrorKind::Credential, || {
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
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.source.get_token(scopes).await
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        self.source.clear_cache().await
    }
}
