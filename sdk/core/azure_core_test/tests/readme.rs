// Licensed under the MIT License.

#![allow(dead_code)]
#![allow(unknown_lints)]
#![allow(unnameable_test_items)]

use azure_core_examples::secrets as azure_security_keyvault_secrets;
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn readme() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "get-secret");

    Ok(())
}
