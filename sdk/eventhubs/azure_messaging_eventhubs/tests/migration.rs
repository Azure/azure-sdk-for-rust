// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![allow(dead_code)]
use azure_messaging_eventhubs::{ConsumerClient, ProducerClient};
use include_file::include_markdown;

#[ignore = "only compile doc examples"]
#[tokio::test]
async fn migration() -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("MIGRATION.md", "use_statements", scope);
    include_markdown!("MIGRATION.md", "authentication", scope);
    include_markdown!("MIGRATION.md", "connection_string", scope);
    include_markdown!("MIGRATION.md", "client_construction", scope);
    include_markdown!("MIGRATION.md", "producing", scope);
    include_markdown!("MIGRATION.md", "batch", scope);
    include_markdown!("MIGRATION.md", "consuming", scope);
    include_markdown!("MIGRATION.md", "processor", scope);
    include_markdown!("MIGRATION.md", "checkpoint_store", scope);
    include_markdown!("MIGRATION.md", "async_runtime", scope);
    Ok(())
}

// The error-handling samples are written as bare statements for readability, so
// define the surrounding functions (and the client bindings they need) here
// rather than in the guide.
async fn error_handling(consumer: &ConsumerClient) -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("MIGRATION.md", "error_handling", scope);
    Ok(())
}

async fn send_rejected(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    include_markdown!("MIGRATION.md", "send_rejected", scope);
    Ok(())
}
