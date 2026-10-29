// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
#![allow(
    clippy::needless_update,
    reason = "non_exhaustive prevents struct initialization otherwise"
)]
use azure_core_examples::certificates as azure_security_keyvault_certificates;
use azure_core_examples::identity as azure_identity;
use azure_core_examples::secrets as azure_security_keyvault_secrets;
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn readme() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "new-client");
    include_markdown!("README.md", "request", scope);
    include_markdown!("README.md", "response");
    include_markdown!("README.md", "errors");
    include_markdown!("README.md", "item-pager", scope);
    include_markdown!("README.md", "page-pager", scope);
    include_markdown!("README.md", "safe-debug");

    {
        use azure_security_keyvault_certificates::CertificateClient;

        let client =
            CertificateClient::new("https://my-vault.vault.azure.net", credential.clone(), None)?;

        include_markdown!("README.md", "poller-future", scope);
        include_markdown!("README.md", "poller-stream", scope);
    }

    Ok(())
}

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn custom_policy() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "custom-policy");

    Ok(())
}

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn custom_reqwest() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "custom-reqwest");

    Ok(())
}

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn reqwest_hang() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "reqwest-hang");

    Ok(())
}

#[cfg(all(feature = "test", feature = "unstable_fake"))]
mod fakes {
    use include_file::include_markdown;

    // Keep compile-only service stubs on the same local azure_core graph.
    mod azure_security_keyvault_secrets {
        pub mod models {
            pub use azure_core_examples::secrets::models::Secret;
            pub type SecretClientGetSecretOptions<'a> = azure_core::http::ClientMethodOptions<'a>;
        }

        pub mod clients {
            pub mod fake {
                use super::super::models::SecretClientGetSecretOptions;
                use azure_core::{http::AsyncRawResponse, Result};

                #[async_trait::async_trait]
                pub trait SecretServer: Send + Sync {
                    async fn get_secret(
                        &self,
                        secret_name: &str,
                        options: Option<SecretClientGetSecretOptions<'_>>,
                    ) -> Result<AsyncRawResponse>;
                }
            }
        }

        pub struct SecretClient;

        impl SecretClient {
            pub fn with_fake<T: clients::fake::SecretServer + ?Sized + 'static>(
                _endpoint: &str,
                _server: std::sync::Arc<T>,
                _options: Option<azure_core_examples::secrets::SecretClientOptions>,
            ) -> azure_core::Result<Self> {
                unimplemented!()
            }

            pub async fn get_secret(
                &self,
                _secret_name: &str,
                _options: Option<models::SecretClientGetSecretOptions<'_>>,
            ) -> azure_core::Result<azure_core::http::Response<models::Secret>> {
                unimplemented!()
            }
        }
    }

    #[ignore = "only compile doc examples"]
    #[tokio::test]
    async fn readme() -> azure_core::Result<()> {
        include_markdown!("README.md", "fakes", scope);
        Ok(())
    }
}
