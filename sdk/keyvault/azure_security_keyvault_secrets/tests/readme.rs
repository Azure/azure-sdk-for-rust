// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::SecretClient;
use include_file::include_markdown;

#[ignore = "only ensures README code snippets compile"]
#[tokio::test]
async fn readme() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = SecretClient::new("https://my-vault.vault.azure.net", credential.clone(), None)?;

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    {
        include_markdown!("README.md", "create_secret");
    }

    {
        include_markdown!("README.md", "get_secret");
    }

    {
        include_markdown!("README.md", "update_secret");
    }

    {
        include_markdown!("README.md", "delete_secret");
    }

    {
        include_markdown!("README.md", "list_secrets");
    }

    {
        include_markdown!("README.md", "errors");
    }

    Ok(())
}
