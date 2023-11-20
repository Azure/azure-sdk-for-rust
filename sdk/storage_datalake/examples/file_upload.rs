use azure_storage::prelude::StorageCredentials;
use azure_storage_datalake::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client();

    let file_system_name = format!(
        "azurerustsdk-datalake-example01-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client.file_system_client(file_system_name.to_string());

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client.create().await?;
    println!("create file system response == {create_fs_response:?}\n");

    let file_path = "some/path/example-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    println!("creating file '{file_path}'...");
    let create_file_response = file_client.create().await?;
    println!("create file response == {create_file_response:?}\n");

    let string1 = "some data";
    let data1 = bytes::Bytes::from(string1);
    let data1_length = data1.len() as i64;

    let string2 = "some more data";
    let data2 = bytes::Bytes::from(string2);
    let data2_length = data2.len() as i64;

    println!("appending '{string1}' to file '{file_path}'...");
    let append_to_file_response = file_client.append(0, data1).await?;
    println!("append to file response == {append_to_file_response:?}\n");

    println!("appending '{string2}' to file '{file_path}'...");
    let append_to_file_response = file_client.append(data1_length, data2).await?;
    println!("append to file response == {append_to_file_response:?}\n");

    println!("flushing file '{file_path}'...");
    let flush_file_response = file_client
        .flush(data1_length + data2_length)
        .close(true)
        .await?;
    println!("flush file response == {flush_file_response:?}\n");

    println!("reading file '{file_path}'...");
    let read_file_response = file_client.read().await?;
    println!("read file response == {read_file_response:?}\n");

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().await?;
    println!("delete file system response == {delete_fs_response:?}\n");

    Ok(())
}

fn create_data_lake_client() -> DataLakeClient {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account_name.clone(), account_key);
    DataLakeClient::new(account_name, storage_credentials)
}
