use std::time::Duration;

use crate::{
    timeout::TimeoutExt, token_credentials::cache::TokenCache, AppServiceManagedIdentityCredential,
    AzureCliCredential, EnvironmentCredential, SpecificAzureCredential, TokenCredentialOptions,
    VirtualMachineManagedIdentityCredential,
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
};

/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    options: TokenCredentialOptions,
    include_specific_credential: bool,
    include_environment_credential: bool,
    include_app_service_managed_identity_credential: bool,
    include_virtual_machine_managed_identity_credential: bool,
    include_azure_cli_credential: bool,
}

impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            options: TokenCredentialOptions::default(),
            include_specific_credential: true,
            include_environment_credential: true,
            include_app_service_managed_identity_credential: true,
            // Unable to quickly detect if running in Azure VM, so it is disabled by default.
            include_virtual_machine_managed_identity_credential: false,
            include_azure_cli_credential: true,
        }
    }
}

impl DefaultAzureCredentialBuilder {
    /// Create a new `DefaultAzureCredentialBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(&mut self, options: impl Into<TokenCredentialOptions>) -> &mut Self {
        self.options = options.into();
        self
    }

    /// Exclude specific credential
    pub fn exclude_specific_credential(&mut self) -> &mut Self {
        self.include_specific_credential = false;
        self
    }

    /// Exclude using credentials from the environment
    pub fn exclude_environment_credential(&mut self) -> &mut Self {
        self.include_environment_credential = false;
        self
    }

    /// Exclude using any managed identity credential
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = false;
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Exclude using virtual machine managed identity credential
    pub fn exclude_virtual_machine_managed_identity_credential(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Include using virtual machine managed identity credential
    pub fn include_virtual_machine_managed_identity_credentials(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = true;
        self
    }

    /// Inlucde using app service managed identity credential
    pub fn include_app_service_managed_identity_credentials(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = true;
        self
    }

    /// Exclude using credentials from the cli
    pub fn exclude_azure_cli_credential(&mut self) -> &mut Self {
        self.include_azure_cli_credential = false;
        self
    }

    /// Get a list of the credential types to include.
    fn included(&self) -> Vec<DefaultAzureCredentialType> {
        let mut sources = Vec::new();
        if self.include_specific_credential {
            sources.push(DefaultAzureCredentialType::Specific);
        }
        if self.include_environment_credential {
            sources.push(DefaultAzureCredentialType::Environment);
        }
        if self.include_app_service_managed_identity_credential {
            sources.push(DefaultAzureCredentialType::AppService);
        }
        if self.include_virtual_machine_managed_identity_credential {
            sources.push(DefaultAzureCredentialType::VirtualMachine);
        }
        if self.include_azure_cli_credential {
            sources.push(DefaultAzureCredentialType::AzureCli);
        }
        sources
    }

    fn try_create(
        &self,
        included: &Vec<DefaultAzureCredentialType>,
    ) -> Vec<DefaultAzureCredentialEnum> {
        // If specific credential is included, try to create it.
        // Use only the specific credential if it is created successfully.
        if self.include_specific_credential {
            if let Ok(credential) = SpecificAzureCredential::create(self.options.clone()) {
                return vec![DefaultAzureCredentialEnum::Specific(credential)];
            }
        }

        let mut sources = Vec::<DefaultAzureCredentialEnum>::with_capacity(included.len());
        for source in included {
            match source {
                DefaultAzureCredentialType::Specific => {}
                DefaultAzureCredentialType::Environment => {
                    if let Ok(credential) = EnvironmentCredential::create(self.options.clone()) {
                        sources.push(DefaultAzureCredentialEnum::Environment(credential));
                    }
                }
                DefaultAzureCredentialType::AppService => {
                    if let Ok(credential) =
                        AppServiceManagedIdentityCredential::create(self.options.clone())
                    {
                        sources.push(DefaultAzureCredentialEnum::AppService(credential));
                    }
                }
                DefaultAzureCredentialType::VirtualMachine => {
                    sources.push(DefaultAzureCredentialEnum::VirtualMachine(
                        VirtualMachineManagedIdentityCredential::new(self.options.clone()),
                    ));
                }
                DefaultAzureCredentialType::AzureCli => {
                    if let Ok(credential) = AzureCliCredential::create() {
                        sources.push(DefaultAzureCredentialEnum::AzureCli(credential));
                    }
                }
            }
        }
        sources
    }

    /// Create a `DefaultAzureCredential` from this builder.
    pub fn build(&self) -> DefaultAzureCredential {
        let included = self.included();
        let sources = self.try_create(&included);
        DefaultAzureCredential::with_sources(sources)
    }
}

/// Types that may be enabled for use by `DefaultAzureCredential`.
#[derive(Debug, PartialEq)]
enum DefaultAzureCredentialType {
    Specific,
    Environment,
    AppService,
    VirtualMachine,
    AzureCli,
}

/// Types of `TokenCredential` supported by `DefaultAzureCredential`
#[derive(Debug)]
pub enum DefaultAzureCredentialEnum {
    /// A `TokenCredential` instance specified with an `AZURE_CREDENTIAL_TYPE` environment variable.
    Specific(SpecificAzureCredential),
    /// `TokenCredential` from environment variable.
    Environment(EnvironmentCredential),
    /// `TokenCredential` from managed identity that has been assigned to an App Service.
    AppService(AppServiceManagedIdentityCredential),
    /// `TokenCredential` from managed identity that has been assigned to a virtual machine.
    VirtualMachine(VirtualMachineManagedIdentityCredential),
    /// `TokenCredential` from Azure CLI.
    AzureCli(AzureCliCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            DefaultAzureCredentialEnum::Specific(credential) => credential
                .get_token(scopes)
                .await
                .context(ErrorKind::Credential, "error getting specific credential"),
            DefaultAzureCredentialEnum::Environment(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting environment credential",
                )
            }
            DefaultAzureCredentialEnum::AppService(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting managed identity credential for App Service",
                )
            }
            DefaultAzureCredentialEnum::VirtualMachine(credential) => {
                // IMSD timeout is only limited to 1 second when used in DefaultAzureCredential
                credential
                    .get_token(scopes)
                    .timeout(Duration::from_secs(1))
                    .await
                    .context(
                        ErrorKind::Credential,
                        "getting virtual machine managed identity credential timed out",
                    )?
                    .context(
                        ErrorKind::Credential,
                        "error getting virtual machine managed identity credential",
                    )
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting token credential from Azure CLI",
                )
            }
        }
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            DefaultAzureCredentialEnum::Specific(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::Environment(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::AppService(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
/// - `EnvironmentCredential`
/// - `ManagedIdentityCredential`
/// - `AzureCliCredential`
/// Consult the documentation of these credential types for more information on how they attempt authentication.
#[derive(Debug)]
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialEnum>,
    cache: TokenCache,
}

impl DefaultAzureCredential {
    /// Creates a `DefaultAzureCredential` with specified sources.
    ///
    /// These sources will be tried in the order provided in the `TokenCredential` authentication flow.
    pub fn with_sources(sources: Vec<DefaultAzureCredentialEnum>) -> Self {
        DefaultAzureCredential {
            sources,
            cache: TokenCache::new(),
        }
    }

    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(scopes).await;

            match token_res {
                Ok(token) => return Ok(token),
                Err(error) => errors.push(error),
            }
        }
        Err(Error::with_message(ErrorKind::Credential, || {
            format!(
                "Multiple errors were encountered while attempting to authenticate:\n{}",
                format_aggregate_error(&errors)
            )
        }))
    }
}

impl Default for DefaultAzureCredential {
    fn default() -> Self {
        DefaultAzureCredentialBuilder::new().build()
    }
}

/// Returns a new `DefaultAzureCredential`.
pub fn new_credential() -> std::sync::Arc<dyn TokenCredential> {
    std::sync::Arc::new(DefaultAzureCredential::default())
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        // clear the internal cache as well as each of the underlying providers
        self.cache.clear().await?;

        for source in &self.sources {
            source.clear_cache().await?;
        }

        Ok(())
    }
}

fn format_aggregate_error(errors: &[Error]) -> String {
    errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_options, EnvironmentCredentialEnum, SpecificAzureCredentialEnum};

    #[test]
    fn test_builder_included_credential_flags() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert!(builder.include_specific_credential);
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        assert!(builder.include_specific_credential);
        assert!(!builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_environment_credential();
        assert!(builder.include_specific_credential);
        assert!(builder.include_azure_cli_credential);
        assert!(!builder.include_environment_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        assert!(builder.include_specific_credential);
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(!builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);
    }

    #[test]
    /// test default included credential types
    fn test_default_included_credential_types() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Specific,
                DefaultAzureCredentialType::Environment,
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test including virtual machine managed identity credential
    #[test]
    fn test_include_virtual_machine_managed_identity_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.include_virtual_machine_managed_identity_credentials();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Specific,
                DefaultAzureCredentialType::Environment,
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::VirtualMachine,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test excluding environment credential
    #[test]
    fn test_exclude_environment_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_environment_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Specific,
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test excluding azure cli credential
    #[test]
    fn test_exclude_azure_cli_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Specific,
                DefaultAzureCredentialType::Environment,
                DefaultAzureCredentialType::AppService,
            ]
        );
    }

    /// test exluding managed identity credentials
    #[test]
    fn test_exclude_managed_identity_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Specific,
                DefaultAzureCredentialType::Environment,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test excluding specific credential
    #[test]
    fn test_exclude_specific_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_specific_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::Environment,
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    #[test]
    fn test_specific_environment_credential() {
        let credential = DefaultAzureCredentialBuilder::new()
            .with_options(test_options(
                &[
                    ("AZURE_CREDENTIAL_TYPE", "environment"),
                    ("AZURE_TENANT_ID", "1"),
                    ("AZURE_CLIENT_ID", "2"),
                    ("AZURE_CLIENT_SECRET", "3"),
                ][..],
            ))
            .build();
        assert_eq!(credential.sources.len(), 1);
        match &credential.sources[0] {
            DefaultAzureCredentialEnum::Specific(credential) => match credential.source() {
                SpecificAzureCredentialEnum::Environment(credential) => match credential.source() {
                    EnvironmentCredentialEnum::ClientSecret(_) => {}
                    _ => panic!("expect client secret credential"),
                },
                _ => panic!("expected environment credential"),
            },
            _ => panic!("expected specific credential"),
        }
    }
}
