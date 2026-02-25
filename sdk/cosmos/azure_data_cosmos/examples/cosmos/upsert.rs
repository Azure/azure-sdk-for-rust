use std::error::Error;

use azure_data_cosmos::{CosmosClient, ItemOptions, PartitionKey};
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
        let container_client = db_client.container_client(&self.container).await;

        let pk = PartitionKey::from(&self.partition_key);
        let item: serde_json::Value = serde_json::from_str(&self.json)?;

        let options =
            ItemOptions::default().with_content_response_on_write_enabled(self.show_updated);

        let response = container_client
            .upsert_item(pk, item, Some(options))
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
