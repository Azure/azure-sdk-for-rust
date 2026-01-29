use azure_data_cosmos::{
    models::{PatchDocument, TransactionalBatch},
    CosmosClient, PartitionKey,
};
use clap::Args;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

/// Demonstrates transactional batch operations.
#[derive(Clone, Args)]
pub struct TransactionalBatchCommand {
    /// The database in which to perform the batch.
    database: String,

    /// The container in which to perform the batch.
    container: String,

    /// The partition key for the batch items.
    #[arg(long, short)]
    partition_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Product {
    id: String,
    #[serde(rename = "partitionKey")]
    partition_key: String,
    name: String,
    price: f64,
    in_stock: bool,
}

impl TransactionalBatchCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        let container_client = db_client.container_client(&self.container);

        // Generate unique IDs for our test products
        let id1 = format!("product-{}", Uuid::new_v4());
        let id2 = format!("product-{}", Uuid::new_v4());
        let id3 = format!("product-{}", Uuid::new_v4());

        // Create some test products
        let product1 = Product {
            id: id1.clone(),
            partition_key: self.partition_key.clone(),
            name: "Test Product 1".to_string(),
            price: 10.99,
            in_stock: true,
        };

        let product2 = Product {
            id: id2.clone(),
            partition_key: self.partition_key.clone(),
            name: "Test Product 2".to_string(),
            price: 20.99,
            in_stock: true,
        };

        let product3 = Product {
            id: id3.clone(),
            partition_key: self.partition_key.clone(),
            name: "Test Product 3".to_string(),
            price: 30.99,
            in_stock: false,
        };

        println!("Creating a transactional batch with multiple operations...");

        // Create a batch that:
        // 1. Creates three new products
        // 2. Reads one of them back
        // 3. Updates another with a patch operation
        let patch = PatchDocument::default()
            .with_set("/in_stock", true)?
            .with_set("/price", 25.99)?;

        let batch = TransactionalBatch::new(PartitionKey::from(&self.partition_key))
            .create_item(&product1)?
            .create_item(&product2)?
            .create_item(&product3)?
            .read_item(id1.clone())
            .patch_item(id3.clone(), patch)?;

        println!(
            "Executing batch with {} operations...",
            batch.operations().len()
        );

        // Execute the batch
        match container_client
            .execute_transactional_batch(batch, None)
            .await
        {
            Ok(response) => {
                println!("Batch executed successfully!");
                println!("Status: {:?}", response.status());
                println!("All operations were committed atomically.");
            }
            Err(e) => {
                println!("Batch failed: {}", e);
                println!("All operations were rolled back.");
                return Err(e.into());
            }
        }

        // Clean up - delete the created items
        println!("\nCleaning up created items...");
        for id in [id1, id2, id3] {
            container_client
                .delete_item(&self.partition_key, &id, None)
                .await?;
            println!("Deleted item: {}", id);
        }

        Ok(())
    }
}
