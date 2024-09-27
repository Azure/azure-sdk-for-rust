use std::error::Error;

use azure_data_cosmos::{
    clients::{ContainerClientMethods, DatabaseClientMethods},
    CosmosClient, CosmosClientMethods, PartitionKey,
};
use clap::Args;
use futures::StreamExt;

#[derive(Clone, Args)]
pub struct QueryCommand {
    /// The database to query.
    database: String,

    /// The container to query.
    container: String,

    /// The query to execute.
    query: String,

    /// The partition key to use when querying the container. Currently this only supports a single string partition key.
    #[clap(long, short)]
    partition_key: String,
}

impl QueryCommand {
    pub async fn run(&self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        let pk = PartitionKey::from(&self.partition_key);
        let mut items_pager =
            container_client.query_items::<serde_json::Value>(&self.query, pk, None)?;

        while let Some(page) = items_pager.next().await {
            let response = page?;
            println!("Results Page");
            println!("  Query Metrics: {:?}", response.query_metrics);
            println!("  Index Metrics: {:?}", response.index_metrics);
            println!("  Items:");
            for item in response.items {
                println!("    * {:#?}", item);
            }
        }
        Ok(())
    }
}
