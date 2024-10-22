use std::error::Error;

use azure_core::StatusCode;
use azure_data_cosmos::CosmosClient;
use clap::Args;

/// Reads a specific item.
#[derive(Clone, Args)]
pub struct ReadCommand {
    /// The database containing the item.
    database: String,

    /// The container containing the item.
    container: String,

    /// The ID of the item.
    #[clap(long, short)]
    item_id: String,

    /// The partition key of the item.
    #[clap(long, short)]
    partition_key: String,
}

impl ReadCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        let response = container_client
            .read_item(&self.partition_key, &self.item_id, None)
            .await;
        match response {
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => println!("Item not found!"),
            Ok(r) => {
                let item: serde_json::Value = r.deserialize_body().await?.unwrap();
                println!("Found item:");
                println!("{:#?}", item);
            }
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }
}
