// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{models::PatchDocument, CosmosClient, TransactionalBatch};
use clap::Args;
use serde_json::Value;
use std::error::Error;

/// Execute a transactional batch of operations.
#[derive(Clone, Args)]
pub struct BatchCommand {
    /// The database containing the container.
    database: String,

    /// The container to execute the batch in.
    container: String,

    /// The partition key for all operations in the batch.
    #[arg(long, short)]
    partition_key: String,

    /// JSON array of operations to execute. Each operation should have "type" and other fields.
    /// Example: [{"type":"create","json":"{\"id\":\"1\"}"},{"type":"read","id":"1"}]
    #[arg(long, short)]
    operations: String,
}

impl BatchCommand {
    pub async fn run(&self, client: &CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        // Parse the operations JSON
        let operations: Vec<Value> = serde_json::from_str(&self.operations)?;

        // Build the batch
        let mut batch = TransactionalBatch::new(&self.partition_key);

        for op in operations {
            let op_type = op["type"].as_str().ok_or("Missing 'type' field")?;

            match op_type {
                "create" => {
                    let json_str = op["json"].as_str().ok_or("Missing 'json' field")?;
                    let value: Value = serde_json::from_str(json_str)?;
                    batch = batch.create_item(value, None)?;
                }
                "upsert" => {
                    let json_str = op["json"].as_str().ok_or("Missing 'json' field")?;
                    let value: Value = serde_json::from_str(json_str)?;
                    batch = batch.upsert_item(value, None)?;
                }
                "replace" => {
                    let id = op["id"].as_str().ok_or("Missing 'id' field")?.to_string();
                    let json_str = op["json"].as_str().ok_or("Missing 'json' field")?;
                    let value: Value = serde_json::from_str(json_str)?;
                    batch = batch.replace_item(id, value, None)?;
                }
                "read" => {
                    let id = op["id"].as_str().ok_or("Missing 'id' field")?.to_string();
                    batch = batch.read_item(id, None);
                }
                "delete" => {
                    let id = op["id"].as_str().ok_or("Missing 'id' field")?.to_string();
                    batch = batch.delete_item(id, None);
                }
                "patch" => {
                    let id = op["id"].as_str().ok_or("Missing 'id' field")?.to_string();
                    let patch_ops = op["patch"].as_array().ok_or("Missing 'patch' field")?;

                    let mut patch_doc = PatchDocument::default();
                    for patch_op in patch_ops {
                        let patch_type = patch_op["op"].as_str().ok_or("Missing 'op' field")?;
                        let path = patch_op["path"]
                            .as_str()
                            .ok_or("Missing 'path' field")?
                            .to_string();

                        match patch_type {
                            "add" | "set" | "replace" => {
                                let value = &patch_op["value"];
                                if patch_type == "add" {
                                    patch_doc = patch_doc.with_add(path, value)?;
                                } else if patch_type == "set" {
                                    patch_doc = patch_doc.with_set(path, value)?;
                                } else {
                                    patch_doc = patch_doc.with_replace(path, value)?;
                                }
                            }
                            "remove" => {
                                patch_doc = patch_doc.with_remove(path)?;
                            }
                            _ => {
                                return Err(
                                    format!("Unknown patch operation: {}", patch_type).into()
                                )
                            }
                        }
                    }

                    batch = batch.patch_item(id, patch_doc, None);
                }
                _ => return Err(format!("Unknown operation type: {}", op_type).into()),
            }
        }

        // Execute the batch
        let response = container_client
            .execute_transactional_batch(batch, None)
            .await?;

        let batch_response = response.into_model()?;

        println!("Batch executed successfully!");
        println!("Total operations: {}", batch_response.results.len());

        for (i, result) in batch_response.results.iter().enumerate() {
            println!("\nOperation {}: Status {}", i + 1, result.status_code);
            if let Some(body) = &result.resource_body {
                println!("  Body: {}", serde_json::to_string_pretty(body)?);
            }
            if let Some(charge) = result.request_charge {
                println!("  Request charge: {}", charge);
            }
        }

        Ok(())
    }
}
