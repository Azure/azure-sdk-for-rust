use azure_core::prelude::*;
use azure_identity::token_credentials::DefaultAzureCredential;
use azure_identity::token_credentials::TokenCredential;
use azure_storage::core::prelude::*;
use azure_storage::data_lake::prelude::*;
use chrono::Utc;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let account = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let master_key = std::env::var("ADLSGEN2_STORAGE_MASTER_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_MASTER_KEY first!");

    let now = Utc::now();
    let file_system_name = format!("azurerustsdk-datalake-example01-{}", now.timestamp());

    let http_client = new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let resource_id = "https://storage.azure.com/";
    println!("getting bearer token for '{}'...", resource_id);
    let bearer_token = DefaultAzureCredential::default()
        .get_token(resource_id)
        .await?;
    println!("token expires on {}\n", bearer_token.expires_on);

    let storage_client = storage_account_client.as_storage_client();
    let data_lake_client = DataLakeClient::new(
        storage_client,
        account,
        bearer_token.token.secret().to_owned(),
        None,
    );

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

    // =============================================================================================

    println!("appending to file '{}'...", file_path);
    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    let append_to_file_response = file_system_client
        .append_to_file(
            Context::default(),
            file_path,
            bytes,
            0,
            FileAppendOptions::default(),
        )
        .await?;
    println!("append to file response == {:?}\n", append_to_file_response);

    println!("flushing file '{}'...", file_path);
    let flush_file_response = file_system_client
        .flush_file(
            Context::default(),
            file_path,
            file_length,
            true,
            FileFlushOptions::default(),
        )
        .await?;
    println!("flush file response == {:?}\n", flush_file_response);

    // =============================================================================================

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().execute().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}
