// Licensed under the MIT License.

#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unnameable_test_items)]

use include_file::include_markdown;

mod azure_security_keyvault_secrets {
    use azure_core::{credentials::TokenCredential, fmt::SafeDebug, http::ClientOptions};
    use std::sync::Arc;

    #[derive(Default, SafeDebug)]
    pub struct SecretClientOptions {
        pub client_options: ClientOptions,
    }

    // This local stub keeps the README compile test on azure_core_test's published-core graph.
    // Reusing azure_core_examples::secrets::SecretClient would pull in local sdk/core/azure_core
    // and cause type mismatches for ClientOptions and TokenCredential during workspace-wide builds.
    pub struct SecretClient;

    impl SecretClient {
        pub fn new(
            _endpoint: &str,
            _credential: Arc<dyn TokenCredential>,
            _options: Option<SecretClientOptions>,
        ) -> azure_core::Result<Self> {
            Ok(Self)
        }
    }
}

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn readme() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "get-secret");

    Ok(())
}
