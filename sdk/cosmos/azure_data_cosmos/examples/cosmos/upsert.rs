// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::error::Error;

use azure_data_cosmos::{
    ContentResponseOnWrite, CosmosClient, ItemWriteOptions, OperationOptions, PartitionKey,
};
use clap::Args;

/// Creates a new item or replaces an existing item, if a matching item already exists.
#[derive(Clone, Args)]
pub struct UpsertCommand {
    /// The database in which to create the item.
    database: String,

    /// The container in which to create the item.
    container: String,

    /// The partition key of the new item.
    #[arg(long, short)]
    partition_key: String,

    /// The id of the new item.
    #[arg(long, short)]
    item_id: String,

    /// The JSON of the new item.
    #[arg(long, short)]
    json: String,

    /// If set, the updated item will be included in the response.
    #[arg(long)]
    show_updated: bool,
}

impl UpsertCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container).await?;

        let pk = PartitionKey::from(&self.partition_key);
        let item: serde_json::Value = serde_json::from_str(&self.json)?;

        let options = if self.show_updated {
            let mut operation = OperationOptions::default();
            operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
            Some(ItemWriteOptions::default().with_operation_options(operation))
        } else {
            None
        };

        let response = container_client
            .upsert_item(pk, &self.item_id, item, options)
            .await?;
        println!("Item updated successfully");

        if self.show_updated {
            let created: serde_json::Value = response.into_body().json()?;
            println!("Updated item:");
            println!("{:#?}", created);
        }
        Ok(())
    }
}
