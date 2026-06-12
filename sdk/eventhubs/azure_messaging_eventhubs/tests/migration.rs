// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn migration() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("MIGRATION.md", "use_statements", scope);
    include_markdown!("MIGRATION.md", "authentication", scope);
    include_markdown!("MIGRATION.md", "client_construction", scope);
    include_markdown!("MIGRATION.md", "producing", scope);
    include_markdown!("MIGRATION.md", "batch", scope);
    include_markdown!("MIGRATION.md", "consuming", scope);
    include_markdown!("MIGRATION.md", "processor", scope);
    include_markdown!("MIGRATION.md", "error_handling", scope);
    include_markdown!("MIGRATION.md", "async_runtime", scope);
    Ok(())
}
