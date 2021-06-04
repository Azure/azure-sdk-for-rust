use azure_storage::core::prelude::*;
use azure_storage::table::prelude::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyEntity {
    #[serde(rename = "PartitionKey")]
    pub city: String,
    pub name: String,
    #[serde(rename = "RowKey")]
    pub surname: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let table_name = std::env::args()
        .nth(1)
        .expect("please specify the table name as first command line parameter");

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let table_service = storage_account_client
        .as_storage_client()
        .as_table_service_client()?;

    let table = table_service.as_table_client(table_name);
    let response = table.create().execute().await?;
    println!("response = {:?}\n", response);

    let mut entity = MyEntity {
        city: "Milan".to_owned(),
        name: "Francesco".to_owned(),
        surname: "Cogno".to_owned(),
    };

    let partition_key_client = table.as_partition_key_client(&entity.city);

    let mut transaction = Transaction::default();

    transaction.add(table.insert().to_transaction_operation(&entity)?);

    entity.surname = "Doe".to_owned();
    transaction.add(table.insert().to_transaction_operation(&entity)?);

    entity.surname = "Karl".to_owned();
    transaction.add(table.insert().to_transaction_operation(&entity)?);

    entity.surname = "Potter".to_owned();
    let entity_client = partition_key_client.as_entity_client(&entity.surname)?;
    transaction.add(
        entity_client
            .insert_or_replace()
            .to_transaction_operation(&entity)?,
    );

    let response = partition_key_client
        .submit_transaction()
        .execute(&transaction)
        .await?;
    println!("response = {:?}\n", response);

    let response = entity_client.delete().execute().await?;
    println!("response = {:?}\n", response);

    let response = table.insert().return_entity(false).execute(&entity).await?;
    println!("response = {:?}\n", response);

    // Get an entity from the table
    let response = entity_client.get().execute().await?;
    println!("response = {:?}\n", response);

    let mut entity: MyEntity = response.entity;
    entity.city = "Rome".to_owned();

    let response = table.insert().return_entity(true).execute(&entity).await?;
    println!("response = {:?}\n", response);

    let entity_client = table
        .as_partition_key_client(&entity.city)
        .as_entity_client(&entity.surname)?;
    // update the name passing the Etag received from the previous call.
    entity.name = "Ryan".to_owned();
    let response = entity_client
        .update()
        .execute(&entity, &(response.etag.into()))
        .await?;
    println!("response = {:?}\n", response);

    // now we perform an upsert
    entity.name = "Carl".to_owned();
    let response = entity_client.insert_or_replace().execute(&entity).await?;
    println!("response = {:?}\n", response);

    let mut stream = Box::pin(table_service.list().top(2).stream());
    while let Some(response) = stream.next().await {
        println!("response = {:?}\n", response);
    }

    let mut stream = Box::pin(
        table
            .query()
            .filter("Name = 'Carl'")
            .top(2)
            .stream::<MyEntity>(),
    );
    while let Some(response) = stream.next().await {
        println!("response = {:?}\n", response);
    }

    let response = table.delete().execute().await?;
    println!("response = {:?}\n", response);

    Ok(())
}
