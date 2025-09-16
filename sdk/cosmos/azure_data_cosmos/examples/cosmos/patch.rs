use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    models::{PatchDocument, PatchOperation},
    CosmosClient, PartitionKey,
};
use clap::Args;

/// Creates a new item.
#[derive(Clone, Args)]
pub struct PatchCommand {
    /// The database in which to create the item.
    database: String,

    /// The container in which to create the item.
    container: String,

    /// The ID of the item.
    #[arg(long, short)]
    item_id: String,

    /// The partition key of the new item.
    #[arg(long, short)]
    partition_key: String,

    /// A JSON patch operation to apply to the item, can be specified multiple times. See https://learn.microsoft.com/en-us/azure/cosmos-db/partial-document-update
    #[arg(long, short)]
    operation: Vec<String>,
}

impl PatchCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        let pk = PartitionKey::from(&self.partition_key);
        let operations: Vec<PatchOperation> = self
            .operation
            .iter()
            .map(|op| serde_json::from_str(op).expect("Invalid JSON patch operation"))
            .collect();
        let patch = PatchDocument {
            condition: None,
            operations,
        };

        let response = container_client
            .patch_item(pk, &self.item_id, patch, None)
            .await;
        match response {
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => println!("Item not found!"),
            Ok(r) => {
                let item: serde_json::Value = r.into_raw_body().json().await?;
                println!("Patched item:");
                println!("{:#?}", item);
            }
            Err(e) => return Err(e.into()),
        };
        Ok(())
    }
}
