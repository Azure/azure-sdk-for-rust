use azure_core::prelude::*;
use azure_storage::clients::*;
use azure_storage::data_lake::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("ADSL_STORAGE_ACCOUNT")
        .expect("Set env variable ADSL_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADSL_STORAGE_MASTER_KEY")
        .expect("Set env variable ADSL_STORAGE_MASTER_KEY first!");

    let file_system_name = std::env::args()
        .nth(1)
        .expect("please specify the file system name as first parameter");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let data_lake = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account)?;

    let file_system = data_lake.as_file_system_client(file_system_name)?;

    // let's add some metadata. We call them "properties"
    // to be consistent with the REST API definition
    // from
    // [https://docs.microsoft.com/en-us/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/en-us/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
    let mut properties = Properties::new();
    properties.insert("AddedVia", "Azure SDK for Rust");
    properties.insert("CreatedAt", chrono::Utc::now().to_string());
    let response = file_system
        .create()
        .properties(&properties)
        .execute()
        .await?;
    println!("repsonse == {:?}", response);

    let mut stream = Box::pin(
        data_lake
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );

    while let Some(response) = stream.next().await {
        println!("response == {:?}\n\n", response);
    }

    properties.insert("ModifiedBy", "Iota");
    let response = file_system
        .set_properties(Some(&properties))
        .execute()
        .await?;
    println!("response == {:?}\n\n", response);

    let response = file_system.get_properties().execute().await?;
    println!("response == {:?}\n\n", response);

    let response = file_system.delete().execute().await?;
    println!("response == {:?}\n\n", response);

    Ok(())
}
