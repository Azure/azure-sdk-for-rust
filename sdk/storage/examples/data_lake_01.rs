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
    println!("token expires on {}", bearer_token.expires_on);
    println!();

    let data_lake_client = storage_account_client
        .as_storage_client()
        .as_data_lake_client(account, bearer_token.token.secret().to_owned())?;

    let file_system_client = data_lake_client.as_file_system_client(&file_system_name)?;

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().execute().await?;
    println!("create file system response == {:?}", create_fs_response);
    println!();

    let file_path = "some/path/example-file.txt";

    println!("creating file '{}'...", file_path);
    let create_file_response = file_system_client
        .create_file(Context::default(), file_path, FileCreateOptions::default())
        .await?;
    println!("create file response == {:?}", create_file_response);
    println!();

    println!("creating file '{}' (overwrite)...", file_path);
    let create_file_response = file_system_client
        .create_file(Context::default(), file_path, FileCreateOptions::default())
        .await?;
    println!("create file response == {:?}", create_file_response);
    println!();

    println!("creating file '{}' if not exists...", file_path);
    let create_file_if_not_exists_result = file_system_client
        .create_file_if_not_exists(Context::default(), file_path)
        .await;
    println!(
        "create file result (should fail) == {:?}",
        create_file_if_not_exists_result
    );
    println!();

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
    println!("append to file response == {:?}", append_to_file_response);
    println!();

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
    println!("flush file response == {:?}", flush_file_response);
    println!();

    let destination_file_path = "some/path/example-file-renamed.txt";
    println!(
        "renaming file '{}' to '{}'...",
        file_path, destination_file_path
    );
    let rename_file_response = file_system_client
        .rename_file(
            Context::default(),
            file_path,
            destination_file_path,
            FileRenameOptions::default(),
        )
        .await?;
    println!("rename file response == {:?}", rename_file_response);
    println!();

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().execute().await?;
    println!("delete file system response == {:?}", delete_fs_response);
    println!();

    println!("data lake example 01 done.");

    Ok(())
}
