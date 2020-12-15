use azure_core::HttpClient;
use azure_storage::clients::*;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct SampleEntity {
    pub something: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let storage_client = storage_account_client.as_storage_client();

    let response = storage_client
        .as_container_client("azuresdkforrust")
        .list_blobs()
        .execute()
        .await?;
    println!("key response = {:#?}", response);

    // we can still use the storage_client since it's not moved
    // by as_container_client above
    // (rather reference counted)
    let response = storage_client.list_containers().execute().await?;
    println!("key response = {:#?}", response);

    // the following code should be moved to another file as
    // it involves table storage
    let table_service_client = storage_account_client.as_table_service_client();

    let response = table_service_client.query_tables().execute().await?;
    println!("key response = {:#?}", response);

    let table_client = table_service_client.as_table_client("example");

    let entity = SampleEntity {
        something: "some data here".to_owned(),
    };

    let response = table_client
        .insert_entity(&entity)
        .with_partition_key("part100")
        .with_row_key("row100")
        .with_client_request_id("sss".into())
        .with_timeout(std::time::Duration::from_secs(20).into())
        .execute()
        .await?;
    println!("key response = {:#?}", response);

    // let's test a SAS token
    // the code is identical
    // once instantiated
    let sas_token = "?sv=2019-12-12&ss=bfqt&srt=sco&sp=rwdlacupx&se=2020-12-05T20:20:58Z&st=2020-12-05T12:20:58Z&spr=https&sig=vxUuKjQW4%2FmB884f%2BdqCp4h3O%2BYuYgIJN8RVGHFVFpY%3D";
    let client = StorageAccountClient::new_sas_token(http_client.clone(), &account, sas_token)
        .as_storage_client();
    let response = client.list_containers().execute().await?;
    println!("sas response = {:#?}", response);

    Ok(())
}
