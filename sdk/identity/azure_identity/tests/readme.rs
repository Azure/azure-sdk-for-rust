// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
#![allow(
    clippy::needless_update,
    reason = "documentation examples intentionally use ..Default::default()"
)]
use include_file::include_markdown;

mod azure_security_keyvault_secrets {
    use azure_core::credentials::TokenCredential;
    use std::sync::Arc;

    pub struct SecretClient;

    impl SecretClient {
        pub fn new(
            _endpoint: &str,
            _credential: Arc<dyn TokenCredential>,
            _options: Option<()>,
        ) -> azure_core::Result<Self> {
            Ok(Self)
        }
    }
}

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn dev() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "dev", scope);
    include_markdown!("README.md", "client-assertion", scope);

    Ok(())
}
