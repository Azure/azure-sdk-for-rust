use std::error::Error;

use azure_core::StatusCode;
use azure_data_cosmos::{CosmosClient, PartitionKey};
use clap::Args;

/// Creates a new item.
#[derive(Clone, Args)]
pub struct ReplaceCommand {
    /// The database in which to create the item.
    database: String,

    /// The container in which to create the item.
    container: String,

    /// The ID of the item.
    #[clap(long, short)]
    item_id: String,

    /// The partition key of the new item.
    #[clap(long, short)]
    partition_key: String,

    /// The JSON of the new item.
    #[clap(long, short)]
    json: String,
}

impl ReplaceCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        let pk = PartitionKey::from(&self.partition_key);
        let item: serde_json::Value = serde_json::from_str(&self.json)?;

        let response = container_client
            .replace_item(pk, &self.item_id, item, None)
            .await;
        match response {
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => println!("Item not found!"),
            Ok(r) => {
                let item: serde_json::Value = r.deserialize_body().await?.unwrap();
                println!("Replaced item:");
                println!("{:#?}", item);
            }
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }
}
