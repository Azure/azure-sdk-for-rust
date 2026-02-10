// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn dev() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("README.md", "dev");

    Ok(())
}
