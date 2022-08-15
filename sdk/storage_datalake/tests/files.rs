use azure_storage_datalake::Properties;
use std::{assert_eq, assert_ne};

mod setup;

#[tokio::test]
async fn file_create_delete() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_create_delete")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-create-delete";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    // TODO: CoreError(PolicyError(RelativeUrlWithoutBase))

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let create_file_if_not_exists_result = file_client.create_if_not_exists().into_future().await;
    assert!(create_file_if_not_exists_result.is_err());

    file_client.create().into_future().await?;

    file_client.delete().into_future().await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn file_upload() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_read")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-read";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    file_client.append(0, bytes).into_future().await?;

    file_client
        .flush(file_length)
        .close(true)
        .into_future()
        .await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn file_read() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_upload")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-upload";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let bytes = bytes::Bytes::from("some data");
    let file_length = bytes.len() as i64;
    file_client.append(0, bytes.clone()).into_future().await?;

    file_client
        .flush(file_length)
        .close(true)
        .into_future()
        .await?;

    let read_file_response = file_client.read().into_future().await?;
    assert_eq!(bytes, read_file_response.data);

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn file_rename() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_rename")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-rename";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
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
        .into_future()
        .await?;
    file_client2.create().into_future().await?;

    // original file properties
    let original_target_file_properties = file_client2.get_properties().into_future().await?;

    let rename_file_if_not_exists_result = file_client1
        .rename_if_not_exists(file_path2)
        .into_future()
        .await;
    assert!(rename_file_if_not_exists_result.is_err());

    file_client1.rename(file_path2).into_future().await?;

    let renamed_file_properties = file_client2.get_properties().into_future().await?;

    // when renaming a file, the source file properties should be propagated
    assert_eq!(renamed_file_properties.properties, Some(file_properties));
    assert_ne!(
        renamed_file_properties.properties,
        original_target_file_properties.properties
    );

    // getting properties for the source file should fail, when the file no longer exists
    let source_file_properties_result = file_client1.get_properties().into_future().await;
    assert!(source_file_properties_result.is_err());

    file_system_client.delete().into_future().await?;

    Ok(())
}

#[tokio::test]
async fn file_get_properties() -> azure_core::Result<()> {
    let data_lake_client = setup::create_data_lake_client("datalake_file_properties")
        .await
        .unwrap();

    let file_system_name = "azurerustsdk-datalake-file-get-properties";
    let file_system_client = data_lake_client
        .clone()
        .into_file_system_client(file_system_name.to_string());

    let create_fs_response = file_system_client.create().into_future().await?;
    assert!(
        create_fs_response.namespace_enabled,
        "namespace should be enabled"
    );

    let file_path = "some/path/e2etest-file.txt";
    let file_client = file_system_client.get_file_client(file_path);

    file_client.create().into_future().await?;

    let mut file_properties = Properties::new();
    file_properties.insert("AddedVia", "Azure SDK for Rust");

    file_client
        .create()
        .properties(file_properties.clone())
        .into_future()
        .await?;

    // Get properties
    let file_properties = file_client.get_properties().into_future().await?;
    assert!(file_properties.properties.is_some());

    // Get status (ie: only system-defined properties)
    let file_properties = file_client.get_status().into_future().await?;
    assert!(!file_properties.properties.is_some());

    // Get access control list for the file
    let file_acl = file_client.get_access_control_list().into_future().await?;
    assert_eq!(
        file_acl.acl,
        Some("user::rw-,group::r--,other::---".to_string())
    );

    // Cleanup
    file_system_client.delete().into_future().await?;

    Ok(())
}
