#![cfg(feature = "mock_transport_framework")]
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
async fn file_upload() -> Result<(), Box<dyn Error + Send + Sync>> {
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
async fn file_rename() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    file_client1.create().into_future().await?;
    file_client2.create().into_future().await?;

    let rename_file_if_not_exists_result = file_client1
        .rename_if_not_exists(file_path2)
        .into_future()
        .await;
    assert!(rename_file_if_not_exists_result.is_err());

    file_client1.rename(file_path2).into_future().await?;

    file_system_client.delete().into_future().await?;

    Ok(())
}
