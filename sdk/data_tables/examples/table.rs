use azure_core::StatusCode;
use azure_data_tables::{operations::InsertEntityResponse, prelude::*};
use azure_storage::prelude::*;
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

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let table_service = TableServiceClient::new(account, storage_credentials);

    let table_client = table_service.table_client(table_name);
    table_client.create().await?;

    let base_city = "Milan".to_string();

    let entity1 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "A".to_owned(),
    };
    let entity2 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "B".to_owned(),
    };
    let entity3 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "C".to_owned(),
    };
    let entity4 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "D".to_owned(),
    };
    let entity5 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "E".to_owned(),
    };
    let entity6 = MyEntity {
        city: base_city.clone(),
        name: "Francesco".to_owned(),
        surname: "F".to_owned(),
    };

    // these are used later
    for entity in [&entity1, &entity5, &entity6] {
        let _: InsertEntityResponse<MyEntity> = table_client.insert(entity)?.await?;
    }

    let partition_key_client = table_client.partition_key_client(&entity1.city);

    let response = partition_key_client
        .transaction()
        .delete(&entity1.surname, None)?
        .insert(&entity2)?
        .update(&entity3.surname, &entity3, None)?
        .merge(&entity4.surname, &entity4, None)?
        .insert_or_replace(&entity5.surname, &entity5)?
        .insert_or_merge(&entity6.surname, &entity6)?
        .await?;

    // check all the events in the transaction completed successfully.
    assert!(response.operation_responses.iter().all(|r| {
        [StatusCode::Ok, StatusCode::NoContent, StatusCode::Created].contains(&r.status_code)
    }));

    let entity_client = partition_key_client.entity_client(&entity2.surname);

    // Get an entity from the table
    let response = entity_client.get().await?;
    let mut entity: MyEntity = response.entity;

    let entity_client = table_client
        .partition_key_client(&entity.city)
        .entity_client(&entity.surname);

    // update the name passing the Etag received from the previous call.
    entity.name = "Ryan".to_owned();
    let response = entity_client.update(&entity, response.etag.into())?.await?;
    println!("update with etag: response = {response:?}\n");

    let response = entity_client.delete().await?;
    println!("delete entity response = {response:?}\n");

    // now we perform an upsert
    entity.name = "Carl".to_owned();
    let response = entity_client.insert_or_replace(&entity)?.await?;
    println!("insert_or_replace response = {response:?}\n");

    let mut stream = table_service.list().into_stream();
    while let Some(response) = stream.next().await {
        let response = response?;
        let names = response
            .tables
            .into_iter()
            .map(|x| x.name)
            .collect::<Vec<_>>()
            .join(", ");
        println!("table names: {names}");
    }

    let mut stream = table_client
        .query()
        .filter("name eq 'Carl'")
        .top(2)
        .into_stream::<MyEntity>();

    while let Some(response) = stream.next().await {
        let response = response?;
        for entity in response.entities {
            println!("{entity:?}");
        }
    }

    table_client.delete().await?;
    Ok(())
}
