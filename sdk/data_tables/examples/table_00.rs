use azure_data_tables::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyEntity {
    #[serde(rename = "PartitionKey")]
    pub city: String,
    pub name: String,
    #[serde(rename = "RowKey")]
    pub surname: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let table_name = std::env::args()
        .nth(1)
        .expect("please specify the table name as first command line parameter");

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key);

    let table_service = storage_account_client
        .storage_client()
        .table_service_client()?;

    let table_client = table_service.table_client(table_name);
    println!("creating table");
    let response = table_client.create().into_future().await?;
    println!("response = {:?}\n", response);

    let mut entity = MyEntity {
        city: "Milan".to_owned(),
        name: "Francesco".to_owned(),
        surname: "Cogno".to_owned(),
    };

    let partition_key_client = table_client.partition_key_client(&entity.city);

    let mut transaction = Transaction::default();

    transaction.add(table_client.insert(&entity)?.to_transaction_operation()?);

    entity.surname = "Doe".to_owned();
    transaction.add(table_client.insert(&entity)?.to_transaction_operation()?);

    entity.surname = "Karl".to_owned();
    transaction.add(table_client.insert(&entity)?.to_transaction_operation()?);

    entity.surname = "Potter".to_owned();
    let entity_client = partition_key_client.entity_client(&entity.surname)?;
    transaction.add(
        entity_client
            .insert_or_replace(&entity)?
            .to_transaction_operation()?,
    );

    let response = partition_key_client
        .submit_transaction(transaction)
        .into_future()
        .await?;
    println!("submit transaction response = {:?}\n", response);

    let response = entity_client.delete().into_future().await?;
    println!("delete entity response = {:?}\n", response);

    table_client
        .insert(&entity)?
        .return_entity(false)
        .into_future::<MyEntity>()
        .await?;

    // Get an entity from the table
    let response = entity_client.get().into_future().await?;
    println!("get entity response = {:?}\n", response);

    let mut entity: MyEntity = response.entity;
    entity.city = "Rome".to_owned();

    let response = table_client
        .insert(&entity)?
        .return_entity(true)
        .into_future::<MyEntity>()
        .await?;
    println!("insert entity response = {:?}\n", response);

    let entity_client = table_client
        .partition_key_client(&entity.city)
        .entity_client(&entity.surname)?;
    // update the name passing the Etag received from the previous call.
    entity.name = "Ryan".to_owned();
    let response = entity_client
        .update(&entity, response.etag.into())?
        .into_future()
        .await?;
    println!("update with etag: response = {:?}\n", response);

    // now we perform an upsert
    entity.name = "Carl".to_owned();
    let response = entity_client
        .insert_or_replace(&entity)?
        .into_future()
        .await?;
    println!("insert_or_replace response = {:?}\n", response);

    let mut stream = table_service.list().top(2).into_stream();
    while let Some(response) = stream.next().await {
        println!("stream response list tables = {:?}\n", response);
    }

    let mut stream = table_client
        .query()
        .filter("Name eq 'Carl'")
        .top(2)
        .into_stream::<MyEntity>();

    while let Some(response) = stream.next().await {
        println!("stream response query entries = {:?}\n", response);
    }

    let response = table_client.delete().into_future().await?;
    println!("delete table response = {:?}\n", response);

    Ok(())
}
