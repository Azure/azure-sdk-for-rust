use azure_data_cosmos::{clients::DatabaseClientMethods, CosmosClientMethods};

mod setup;

#[tokio::test]
pub async fn read_database_that_exists() {
    let client =
        setup::create_cosmos_client(azure_testing::context!("read_database_that_exists"), None)
            .unwrap();
    let db_client = client.database_client("TestDatabase");
    let response = db_client.read(None).await.unwrap();
    let properties = response.deserialize_body().await.unwrap();

    assert_eq!("TestDatabase", properties.id);
}
