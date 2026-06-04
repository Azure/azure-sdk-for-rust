// Licensed under the MIT License.

#![allow(dead_code)]
#![allow(unnameable_test_items)]
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn readme() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "get-secret");

    Ok(())
}
