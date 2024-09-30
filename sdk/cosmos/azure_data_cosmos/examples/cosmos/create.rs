use std::error::Error;

use azure_data_cosmos::{
    clients::{ContainerClientMethods, DatabaseClientMethods},
    CosmosClient, CosmosClientMethods, PartitionKey,
};
use clap::Args;

/// Creates a new item.
#[derive(Clone, Args)]
pub struct CreateCommand {
    /// The database in which to create the item.
    database: String,

    /// The container in which to create the item.
    container: String,

    /// The partition key of the new item.
    #[clap(long, short)]
    partition_key: String,

    /// The JSON of the new item.
    #[clap(long, short)]
    json: String,
}

impl CreateCommand {
    pub async fn run(&self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        let pk = PartitionKey::from(&self.partition_key);
        let item: serde_json::Value = serde_json::from_str(&self.json)?;

        let created = container_client
            .create_item(pk, item, None)
            .await?
            .deserialize_body()
            .await?
            .into_inner();
        println!("Created item:");
        println!("{:#?}", created);
        Ok(())
    }
}
