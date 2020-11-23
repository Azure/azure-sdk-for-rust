#![cfg(all(test, feature = "test_e2e"))]
use azure_cosmos::prelude::*;
use azure_cosmos::PermissionMode;

mod setup;

#[tokio::test]
async fn permissions() {
    const DATABASE_NAME: &str = "cosmos-test-db-users";
    const COLLECTION_NAME: &str = "cosmos-test-db-users";
    const USER_NAME1: &str = "someone@cool.net";
    const USER_NAME2: &str = "pollastro@cool.net";
    const PERMISSION1: &str = "godmode";
    const PERMISSION2: &str = "spyme";

    let client = setup::initialize().unwrap();

    // create a temp database
    let _create_database_response = client
        .create_database()
        .with_database_name(&DATABASE_NAME)
        .execute()
        .await
        .unwrap();

    let database_client = client.into_database_client(DATABASE_NAME);

    // create two users
    let user1_client = database_client.into_user_client(USER_NAME1);
    let _create_user_response = user1_client.create_user().execute().await.unwrap();
    let user2_client = database_client.into_user_client(USER_NAME2);
    let _create_user_response = user2_client.create_user().execute().await.unwrap();

    // create a temp collection
    let create_collection_response = {
        let indexes = IncludedPathIndex {
            kind: KeyKind::Hash,
            data_type: DataType::String,
            precision: Some(3),
        };

        let ip = IncludedPath {
            path: "/*".to_owned(),
            indexes: Some(vec![indexes]),
        };

        let ip = IndexingPolicy {
            automatic: true,
            indexing_mode: IndexingMode::Consistent,
            included_paths: vec![ip],
            excluded_paths: vec![],
        };

        database_client
            .create_collection()
            .with_collection_name(&COLLECTION_NAME)
            .with_partition_key(&("/id".into()))
            .with_offer(Offer::Throughput(400))
            .with_indexing_policy(&ip)
            .execute()
            .await
            .unwrap()
    };

    // create two permissions
    let permission_client_user1 = user1_client.into_permission_client(PERMISSION1);
    let permission_client_user2 = user2_client.into_permission_client(PERMISSION2);

    let _create_permission_user1_response = permission_client_user1
        .create_permission()
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute_with_permission(&PermissionMode::All(&create_collection_response.collection))
        .await
        .unwrap();

    let _create_permission_user2_response = permission_client_user2
        .create_permission()
        .with_expiry_seconds(18000) // 5 hours, max!
        .execute_with_permission(&PermissionMode::Read(create_collection_response.collection))
        .await
        .unwrap();

    let list_permissions_response = user1_client.list_permissions().execute().await.unwrap();
    assert_eq!(list_permissions_response.permissions.len(), 1);

    let list_permissions_response = user2_client.list_permissions().execute().await.unwrap();
    assert_eq!(list_permissions_response.permissions.len(), 1);

    // delete the database
    database_client.delete_database().execute().await.unwrap();
}
