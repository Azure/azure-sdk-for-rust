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
async fn migration() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("MIGRATION.md", "use_statements", scope);
    include_markdown!("MIGRATION.md", "authentication", scope);
    include_markdown!("MIGRATION.md", "client_construction", scope);
    include_markdown!("MIGRATION.md", "developer_tools", scope);
    include_markdown!("MIGRATION.md", "azure_cli", scope);
    include_markdown!("MIGRATION.md", "client_secret", scope);
    include_markdown!("MIGRATION.md", "managed_identity", scope);
    include_markdown!("MIGRATION.md", "workload_identity", scope);
    include_markdown!("MIGRATION.md", "azure_pipelines", scope);
    #[cfg(feature = "client_certificate")]
    include_markdown!("MIGRATION.md", "client_certificate", scope);
    include_markdown!("MIGRATION.md", "service_client", scope);
    include_markdown!("MIGRATION.md", "error_handling", scope);
    include_markdown!("MIGRATION.md", "async_runtime", scope);
    include_markdown!("MIGRATION.md", "concurrency", scope);

    Ok(())
}
