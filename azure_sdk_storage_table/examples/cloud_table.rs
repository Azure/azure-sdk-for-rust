#[macro_use]
extern crate serde_derive;

use azure_sdk_storage_table::{Batch, CloudTable, Continuation, TableClient};
use std::error::Error;
use std::mem;

#[derive(Debug, Serialize, Deserialize)]
struct MyEntity {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = TableClient::new(&account, &master_key);
    let cloud_table = CloudTable::new(client, "test");
    cloud_table.create_if_not_exists().await?;

    let entity = cloud_table.get::<MyEntity>("pk", "rk", None).await?;
    println!("entity: {:?}", entity);

    let cnt = 20usize;

    let mut batch = Batch::new("big2".to_owned());
    for r in 0usize..cnt {
        batch.add_insert(
            format!("rk-{}", r),
            &MyEntity {
                data: "hello".to_owned(),
            },
        )?;
        if batch.is_full() {
            println!("batch insert {}", r);
            let batch = mem::replace(&mut batch, Batch::new("big2".to_owned()));
            cloud_table.execute_batch(batch).await?;
        }
    }
    if !batch.is_empty() {
        println!("batch insert last part");
        cloud_table.execute_batch(batch).await?;
    }

    let entity = cloud_table
        .get::<serde_json::Value>("big2", "0", None)
        .await?;
    println!("entity(value): {:?}", entity);

    let mut cont = Continuation::start();
    while let Some(entities) = cloud_table
        .execute_query::<MyEntity>(None, &mut cont)
        .await?
    {
        println!("segment: {:?}", entities.first());
    }

    let mut cont = Continuation::start();
    while let Some(entities) = cloud_table
        .execute_query::<serde_json::Value>(None, &mut cont)
        .await?
    {
        println!("segment(value): {:?}", entities.first());
    }

    let mut batch = Batch::new("big2".to_owned());
    for r in 0usize..cnt {
        if r % 2 == 0 {
            batch.add_delete(format!("rk-{}", r), None)?;
        } else {
            batch.add_update(
                format!("rk-{}", r),
                &MyEntity {
                    data: "updated".to_owned(),
                },
                None,
            )?;
        }
        if batch.is_full() {
            println!("batch delete/update {}", r);
            let batch = mem::replace(&mut batch, Batch::new("big2".to_owned()));
            cloud_table.execute_batch(batch).await?;
        }
    }
    if !batch.is_empty() {
        println!("batch delete/update last part");
        cloud_table.execute_batch(batch).await?;
    }

    Ok(())
}
