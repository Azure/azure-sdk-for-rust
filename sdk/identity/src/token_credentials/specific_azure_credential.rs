use super::options;
#[cfg(not(target_arch = "wasm32"))]
use crate::AzureCliCredential;
#[cfg(feature = "client_certificate")]
use crate::ClientCertificateCredential;
use crate::{
    AppServiceManagedIdentityCredential, ClientSecretCredential, EnvironmentCredential,
    TokenCredentialOptions, VirtualMachineManagedIdentityCredential, WorkloadIdentityCredential,
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{ErrorKind, ResultExt},
    Error,
};
use std::sync::Arc;

pub const AZURE_CREDENTIAL_TYPE: &str = "AZURE_CREDENTIAL_TYPE";

pub mod azure_credential_types {
    pub const ENVIRONMENT: &str = "environment";
    #[cfg(not(target_arch = "wasm32"))]
    pub const AZURE_CLI: &str = "azurecli";
    pub const VIRTUAL_MACHINE: &str = "virtualmachine";
    pub const APP_SERVICE: &str = "appservice";
    pub const CLIENT_SECRET: &str = "clientsecret";
    pub const WORKLOAD_IDENTITY: &str = "workloadidentity";
    #[cfg(feature = "client_certificate")]
    pub const CLIENT_CERTIFICATE: &str = "clientcertificate";
}

/// Creates a `DefaultAzureCredential` by default with default options.
/// If `AZURE_CREDENTIAL_TYPE` environment variable is set, it creates a `SpecificAzureCredential` with default options.
pub fn create_credential() -> azure_core::Result<Arc<dyn TokenCredential>> {
    create_credential_with_options(options::TokenCredentialOptions::default())
}

fn create_credential_with_options(
    options: TokenCredentialOptions,
) -> azure_core::Result<Arc<dyn TokenCredential>> {
    let env = options.env();
    match env.var(AZURE_CREDENTIAL_TYPE) {
        Ok(_) => SpecificAzureCredential::create(options)
            .map(|cred| Arc::new(cred) as Arc<dyn TokenCredential>),
        Err(_) => crate::DefaultAzureCredentialBuilder::default()
            .with_options(options)
            .build()
            .map(|cred| Arc::new(cred) as Arc<dyn TokenCredential>),
    }
}

/// Creates a new `SpecificAzureCredential` with the default options.
pub fn create_specific_credential() -> azure_core::Result<Arc<dyn TokenCredential>> {
    Ok(Arc::new(SpecificAzureCredential::create(
        TokenCredentialOptions::default(),
    )?))
}

#[derive(Debug)]
pub enum SpecificAzureCredentialEnum {
    Environment(EnvironmentCredential),
    #[cfg(not(target_arch = "wasm32"))]
    AzureCli(AzureCliCredential),
    VirtualMachine(VirtualMachineManagedIdentityCredential),
    AppService(AppServiceManagedIdentityCredential),
    ClientSecret(ClientSecretCredential),
    WorkloadIdentity(WorkloadIdentityCredential),
    #[cfg(feature = "client_certificate")]
    ClientCertificate(ClientCertificateCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SpecificAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            SpecificAzureCredentialEnum::Environment(credential) => {
                credential.get_token(scopes).await
            }
            #[cfg(not(target_arch = "wasm32"))]
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.get_token(scopes).await,
            SpecificAzureCredentialEnum::VirtualMachine(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::AppService(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::ClientSecret(credential) => {
                credential.get_token(scopes).await
            }
            SpecificAzureCredentialEnum::WorkloadIdentity(credential) => {
                credential.get_token(scopes).await
            }
            #[cfg(feature = "client_certificate")]
            SpecificAzureCredentialEnum::ClientCertificate(credential) => {
                credential.get_token(scopes).await
            }
        }
    }

    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            SpecificAzureCredentialEnum::Environment(credential) => credential.clear_cache().await,
            #[cfg(not(target_arch = "wasm32"))]
            SpecificAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            SpecificAzureCredentialEnum::AppService(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::ClientSecret(credential) => credential.clear_cache().await,
            SpecificAzureCredentialEnum::WorkloadIdentity(credential) => {
                credential.clear_cache().await
            }
            #[cfg(feature = "client_certificate")]
            SpecificAzureCredentialEnum::ClientCertificate(credential) => {
                credential.clear_cache().await
            }
        }
    }
}

#[derive(Debug)]
pub struct SpecificAzureCredential {
    source: SpecificAzureCredentialEnum,
}

impl SpecificAzureCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<SpecificAzureCredential> {
        let env = options.env();
        let credential_type = env.var(AZURE_CREDENTIAL_TYPE)?;
        let source: SpecificAzureCredentialEnum =
            // case insensitive and allow spaces
            match credential_type.replace(' ', "").to_lowercase().as_str() {
                azure_credential_types::ENVIRONMENT => EnvironmentCredential::create(options)
                    .map(SpecificAzureCredentialEnum::Environment)
                    .with_context(ErrorKind::Credential, || {
                        format!(
                            "unable to create AZURE_CREDENTIAL_TYPE of {}",
                            azure_credential_types::ENVIRONMENT
                        )
                    })?,
                azure_credential_types::APP_SERVICE => {
                    AppServiceManagedIdentityCredential::create(options)
                        .map(SpecificAzureCredentialEnum::AppService)
                        .with_context(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_TYPE of {}",
                                azure_credential_types::APP_SERVICE
                            )
                        })?
                }
                azure_credential_types::VIRTUAL_MACHINE => {
                    SpecificAzureCredentialEnum::VirtualMachine(
                        VirtualMachineManagedIdentityCredential::new(options),
                    )
                }
                #[cfg(not(target_arch = "wasm32"))]
                azure_credential_types::AZURE_CLI => AzureCliCredential::create()
                    .map(SpecificAzureCredentialEnum::AzureCli)
                    .with_context(ErrorKind::Credential, || {
                        format!(
                            "unable to create AZURE_CREDENTIAL_TYPE of {}",
                            azure_credential_types::AZURE_CLI
                        )
                    })?,
                azure_credential_types::CLIENT_SECRET => ClientSecretCredential::create(options)
                    .map(SpecificAzureCredentialEnum::ClientSecret)?,
                azure_credential_types::WORKLOAD_IDENTITY => {
                    WorkloadIdentityCredential::create(options)
                        .map(SpecificAzureCredentialEnum::WorkloadIdentity)
                        .with_context(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_TYPE of {}",
                                azure_credential_types::WORKLOAD_IDENTITY
                            )
                        })?
                }
                #[cfg(feature = "client_certificate")]
                azure_credential_types::CLIENT_CERTIFICATE => {
                    ClientCertificateCredential::create(options)
                        .map(SpecificAzureCredentialEnum::ClientCertificate)
                        .with_context(ErrorKind::Credential, || {
                            format!(
                                "unable to create AZURE_CREDENTIAL_TYPE of {}",
                                azure_credential_types::CLIENT_CERTIFICATE
                            )
                        })?
                }
                _ => {
                    return Err(Error::with_message(ErrorKind::Credential, || {
                        format!("unknown AZURE_CREDENTIAL_TYPE of {}", credential_type)
                    }))
                }
            };
        Ok(Self { source })
    }

    #[cfg(test)]
    pub(crate) fn source(&self) -> &SpecificAzureCredentialEnum {
        &self.source
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

#[cfg(test)]
pub fn test_options(env_vars: &[(&str, &str)]) -> TokenCredentialOptions {
    let env = crate::env::Env::from(env_vars);
    let http_client = azure_core::new_http_client();
    TokenCredentialOptions::new(env, http_client)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EnvironmentCredentialKind;

    /// test AZURE_CREDENTIAL_TYPE of "environment"
    #[test]
    fn test_environment() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[
                ("AZURE_CREDENTIAL_TYPE", "environment"),
                ("AZURE_TENANT_ID", "1"),
                ("AZURE_CLIENT_ID", "2"),
                ("AZURE_CLIENT_SECRET", "3"),
            ][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::Environment(credential) => match credential.source() {
                EnvironmentCredentialKind::ClientSecret(_) => {}
                _ => panic!("expect client secret credential"),
            },
            _ => panic!("expected environment credential"),
        }
        Ok(())
    }

    /// test AZURE_CREDENTIAL_TYPE of "azurecli"
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_azure_cli() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[("AZURE_CREDENTIAL_TYPE", "azurecli")][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::AzureCli(_) => {}
            _ => panic!("expected azure cli credential"),
        }
        Ok(())
    }

    /// test naming "Azure CLI"
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_azure_cli_naming() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[("AZURE_CREDENTIAL_TYPE", "Azure CLI")][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::AzureCli(_) => {}
            _ => panic!("expected azure cli credential"),
        }
        Ok(())
    }

    /// test AZURE_CREDENTIAL_TYPE of "virtualmachine"
    #[test]
    fn test_virtual_machine() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[("AZURE_CREDENTIAL_TYPE", "virtualmachine")][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::VirtualMachine(_) => {}
            _ => panic!("expected virtual machine credential"),
        }
        Ok(())
    }

    /// test AZURE_CREDENTIAL_TYPE of "appservice"
    #[test]
    fn test_app_service() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[
                ("AZURE_CREDENTIAL_TYPE", "appservice"),
                ("IDENTITY_ENDPOINT", "https://identityendpoint/token"),
            ][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::AppService(_) => {}
            _ => panic!("expected app service credential"),
        }
        Ok(())
    }

    /// test AZURE_CREDENTIAL_TYPE of "clientsecret"
    #[test]
    fn test_client_secret() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[
                ("AZURE_CREDENTIAL_TYPE", "clientsecret"),
                ("AZURE_TENANT_ID", "1"),
                ("AZURE_CLIENT_ID", "2"),
                ("AZURE_CLIENT_SECRET", "3"),
            ][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::ClientSecret(_) => {}
            _ => panic!("expected client secret credential"),
        }
        Ok(())
    }

    /// test AZURE_CREDENTIAL_TYPE of "workloadidentity"
    #[test]
    fn test_workload_identity() -> azure_core::Result<()> {
        let credential = SpecificAzureCredential::create(test_options(
            &[
                ("AZURE_CREDENTIAL_TYPE", "workloadidentity"),
                ("AZURE_TENANT_ID", "1"),
                ("AZURE_CLIENT_ID", "2"),
                ("AZURE_FEDERATED_TOKEN", "3"),
            ][..],
        ))?;
        match credential.source() {
            SpecificAzureCredentialEnum::WorkloadIdentity(_) => {}
            _ => panic!("expected workload identity credential"),
        }
        Ok(())
    }
}
