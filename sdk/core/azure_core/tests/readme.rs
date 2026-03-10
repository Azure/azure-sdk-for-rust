// Licensed under the MIT License.

#![allow(dead_code)]
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
