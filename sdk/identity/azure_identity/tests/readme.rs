// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use azure_core_examples::secrets as azure_security_keyvault_secrets;
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn dev() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "dev", scope);
    include_markdown!("README.md", "client-assertion", scope);

    Ok(())
}
