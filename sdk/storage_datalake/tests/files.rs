#![cfg(feature = "mock_transport_framework")]
use azure_storage_datalake::Properties;
use std::error::Error;

mod setup;

#[tokio::test]
async fn file_create_delete() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_create_delete")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-create-delete";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    // TODO: CoreError(PolicyError(RelativeUrlWithoutBase))

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().await?;

    let create_file_if_not_exists_result = file_client.create_if_not_exists().await;
    assert!(create_file_if_not_exists_result.is_err());

    file_client.create().await?;

    file_client.delete().await?;

    file_system_client.delete().await?;

    Ok(())
}

#[tokio::test]
async fn file_upload() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_read")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-read";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().await?;

    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    file_client.append(0, bytes).await?;

    file_client
        .flush(file_length)
        .close(true)

        .await?;

    file_system_client.delete().await?;

    Ok(())
}

#[tokio::test]
async fn file_read() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_upload")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-upload";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().await?;

    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    file_client.append(0, bytes.clone()).await?;

    file_client
        .flush(file_length)
        .close(true)

        .await?;

    let read_file_response = file_client.read().await?;
    assert_eq!(bytes, read_file_response.data);

    file_system_client.delete().await?;

    Ok(())
}

#[tokio::test]
async fn file_rename() -> Result<(), Box<dyn Error + Send + Sync>> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_rename")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-rename";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path1 = "some/path/e2etest-file1.txt";
    let file_client1 = file_system_client.get_file_client(file_path1);
    let file_path2 = "some/path/e2etest-file2.txt";
    let file_client2 = file_system_client.get_file_client(file_path2);

    let mut file_properties = Properties::new();
    file_properties.insert("AddedVia", "Azure SDK for Rust");

    file_client1
        .create()
        .properties(file_properties.clone())

        .await?;
    file_client2.create().await?;

    // original file properties
    let original_target_file_properties = file_client2.get_properties().await?;

    let rename_file_if_not_exists_result = file_client1
        .rename_if_not_exists(file_path2)

        .await;
    assert!(rename_file_if_not_exists_result.is_err());

    let file_client3 = file_client1.rename(file_path2).await?;
    let renamed_file_properties = file_client3.get_properties().await?;

    // when renaming a file, the source file properties should be propagated
    assert_eq!(renamed_file_properties.properties, file_properties);
    assert_ne!(
        renamed_file_properties.properties,
        original_target_file_properties.properties
    );

    // getting properties for the source file should fail, when the file no longer exists
    let source_file_properties_result = file_client1.get_properties().await;
    assert!(source_file_properties_result.is_err());

    file_system_client.delete().await?;

    Ok(())
}
