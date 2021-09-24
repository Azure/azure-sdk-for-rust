use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::data_lake::prelude::*;
use azure_storage::data_lake::operations::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let file_system_name = std::env::args()
        .nth(1)
        .expect("lease specify the file system name as first parameter");

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let data_lake = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account)?;

    let file_system = data_lake.as_file_system_client(file_system_name)?;

    // let's create the file system
    let response = file_system
        .create()
        .execute()
        .await?;
    println!("response == {:?}", response);

    // let's [create a path](https://docs.microsoft.com/en-us/rest/api/storageservices/datalakestoragegen2/path/create)
    let response = file_system
        // TODO: Only supports files right now, not directories
        .create_path(Context::new(), "test_file_123.txt", CreatePathOptions::default())
        .await?;
    println!("response == {:?}", response);

    // let's list the file system
    let mut stream = Box::pin(
        data_lake
            .list()
            .max_results(NonZeroU32::new(3).unwrap())
            .stream(),
    );

    while let Some(response) = stream.next().await {
        println!("response == {:?}\n\n", response);
    }

    // let's delete the file system
    let response = file_system.delete().execute().await?;
    println!("response == {:?}\n\n", response);

    Ok(())
}
