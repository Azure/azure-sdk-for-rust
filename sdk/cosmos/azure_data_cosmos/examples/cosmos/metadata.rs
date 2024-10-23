// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::error::Error;

use azure_data_cosmos::CosmosClient;
use clap::Args;

/// Retrieves basic metadata about databases and containers.
#[derive(Clone, Args)]
pub struct MetadataCommand {
    /// The database to fetch information for.
    database: String,

    /// Optionally, the container to fetch information for.
    #[clap(long, short)]
    container: Option<String>,
}

impl MetadataCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        let db_client = client.database_client(&self.database);
        if let Some(container_name) = &self.container {
            let container_client = db_client.container_client(container_name);
            let response = container_client
                .read(None)
                .await?
                .deserialize_body()
                .await?;
            println!("{:#?}", response);
            return Ok(());
        } else {
            let response = db_client.read(None).await?.deserialize_body().await?;
            println!("{:#?}", response);
        }
        Ok(())
    }
}
