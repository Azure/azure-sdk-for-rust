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

    let file_path = "some/path/example-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    println!("creating file '{}'...", file_path);
    let create_file_response = file_client.create().into_future().await?;
    println!("create file response == {:?}\n", create_file_response);

    let string1 = "some data";
    let data1 = bytes::Bytes::from(string1);
    let data1_length = data1.len() as i64;

    let string2 = "some more data";
    let data2 = bytes::Bytes::from(string2);
    let data2_length = data2.len() as i64;

    println!("appending '{}' to file '{}'...", string1, file_path);
    let append_to_file_response = file_client.append(0, data1).into_future().await?;
    println!("append to file response == {:?}\n", append_to_file_response);

    println!("appending '{}' to file '{}'...", string2, file_path);
    let append_to_file_response = file_client
        .append(data1_length, data2)
        .into_future()
        .await?;
    println!("append to file response == {:?}\n", append_to_file_response);

    println!("flushing file '{}'...", file_path);
    let flush_file_response = file_client
        .flush(data1_length + data2_length)
        .close(true)
        .into_future()
        .await?;
    println!("flush file response == {:?}\n", flush_file_response);

    println!("reading file '{}'...", file_path);
    let read_file_response = file_client.read().into_future().await?;
    println!("read file response == {:?}\n", read_file_response);

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
