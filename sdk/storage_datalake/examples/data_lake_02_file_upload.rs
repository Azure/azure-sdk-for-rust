use azure_core::prelude::*;
use azure_identity::token_credentials::DefaultAzureCredential;
use azure_identity::token_credentials::TokenCredential;
use azure_storage::core::prelude::*;
use azure_storage_datalake::prelude::*;
use chrono::Utc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = create_data_lake_client().await.unwrap();

    let file_system_name = format!("azurerustsdk-datalake-example01-{}", Utc::now().timestamp());
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().execute().await?;
    println!("create file system response == {:?}\n", create_fs_response);

    let file_path = "some/path/example-file.txt";

    println!("creating file '{}'...", file_path);
    let create_file_response = file_system_client
        .create_file(Context::default(), file_path, FileCreateOptions::default())
        .await?;
    println!("create file response == {:?}\n", create_file_response);

    let string1 = "some data";
    let data1 = bytes::Bytes::from(string1);
    let data1_length = data1.len() as i64;

    let string2 = "some more data";
    let data2 = bytes::Bytes::from(string2);
    let data2_length = data2.len() as i64;

    println!("appending '{}' to file '{}'...", string1, file_path);
    let append_to_file_response = file_system_client
        .append_to_file(
            Context::default(),
            file_path,
            data1,
            0,
            FileAppendOptions::default(),
        )
        .await?;
    println!("append to file response == {:?}\n", append_to_file_response);

    println!("appending '{}' to file '{}'...", string2, file_path);
    let append_to_file_response = file_system_client
        .append_to_file(
            Context::default(),
            file_path,
            data2,
            data1_length,
            FileAppendOptions::default(),
        )
        .await?;
    println!("append to file response == {:?}\n", append_to_file_response);

    println!("flushing file '{}'...", file_path);
    let flush_file_response = file_system_client
        .flush_file(
            Context::default(),
            file_path,
            data1_length + data2_length,
            true,
            FileFlushOptions::default(),
        )
        .await?;
    println!("flush file response == {:?}\n", flush_file_response);

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().execute().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> Result<DataLakeClient, Box<dyn Error + Send + Sync>> {
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let options = StorageAccountOptions::default();

    let storage_account_client =
        StorageAccountClient::new_access_key(&account, &master_key, options);

    let resource_id = "https://storage.azure.com/";
    println!("getting bearer token for '{}'...", resource_id);
    let bearer_token = DefaultAzureCredential::default()
        .get_token(resource_id)
        .await?;
    println!("token expires on {}\n", bearer_token.expires_on);

    let storage_client = storage_account_client.as_storage_client();

    Ok(DataLakeClient::new(
        storage_client,
        account,
        bearer_token.token.secret().to_owned(),
        None,
    ))
}
