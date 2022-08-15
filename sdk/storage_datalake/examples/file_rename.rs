use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_datalake::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client().await.unwrap();

    let file_system_name = format!(
        "azurerustsdk-datalake-example01-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().into_future().await?;
    println!("create file system response == {:?}\n", create_fs_response);

    let file_path1 = "some/path/example-file1.txt";
    let file_client1 = file_system_client.get_file_client(file_path1);
    let file_path2 = "some/path/example-file2.txt";
    let file_client2 = file_system_client.get_file_client(file_path2);

    println!("creating file '{}'...", file_path1);
    let create_file_response1 = file_client1.create().into_future().await?;
    println!("create file response == {:?}\n", create_file_response1);

    println!("creating file '{}'...", file_path2);
    let create_file_response2 = file_client2.create().into_future().await?;
    println!("create file response == {:?}\n", create_file_response2);

    println!(
        "renaming file '{}' to '{}' if not exists...",
        file_path1, file_path2
    );
    let rename_file_if_not_exists_result = file_client1
        .rename_if_not_exists(file_path2)
        .into_future()
        .await;
    println!(
        "rename file result (should fail) == {:?}\n",
        rename_file_if_not_exists_result
    );

    println!("renaming file '{}' to '{}'...", file_path1, file_path2);
    file_client1.rename(file_path2).into_future().await?;
    let renamed_file_properties = file_client2.get_properties().into_future().await?;
    println!("renamed file properties == {:?}\n", renamed_file_properties);

    // getting properties for the source file should fail, when the file no longer exists
    // Eventually we will implement the `exists` method, which internally employs a similar check
    let source_file_properties_result = file_client1.get_properties().into_future().await;
    assert!(source_file_properties_result.is_err());

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().into_future().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

async fn create_data_lake_client() -> azure_core::Result<DataLakeClient> {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!");

    Ok(DataLakeClient::new(
        StorageSharedKeyCredential::new(account_name, account_key),
        None,
    ))
}
